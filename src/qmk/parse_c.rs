//! Parser for QMK `keymap.c` source files.
//!
//! This is a targeted parser for the QMK patterns the converter understands,
//! not a full C parser. It strips comments, extracts layer metadata and simple
//! aliases, then parses the expressions inside the `keymaps` array into the
//! shared IR. Unsupported QMK constructs become [`Key::Unknown`] so renderers can
//! leave an explicit TODO in generated output.

use std::collections::{HashMap, HashSet};

use crate::codes;
use crate::error::ParseCError;
use crate::ir::{Key, Keymap, Layer, TapDanceDef, TriLayer};

/// Parse a QMK C `keymap.c` source file into the shared IR.
///
/// The parser recognizes common QMK layer enums, `#define` key aliases, custom
/// keycode enums used as macro placeholders, tap-dance declarations, and
/// `update_tri_layer_state(...)`. It does not compile or preprocess C; inputs
/// that rely on complex macros may produce [`Key::Unknown`] bindings.
///
/// # Errors
/// Returns [`ParseCError`] if the keymaps array is missing, has unmatched
/// delimiters, or a layer entry is structurally malformed.
pub fn parse(source: &str) -> Result<Keymap, ParseCError> {
    let cleaned = strip_comments(source);

    let layer_names = extract_layer_names(&cleaned);
    let layer_map: HashMap<String, usize> = layer_names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect();

    let defines = extract_defines(&cleaned);
    let custom_keycodes = extract_custom_keycodes(&cleaned);
    let tri_layer = extract_tri_layer(&cleaned, &layer_map);

    let tap_dances = extract_tap_dances(&cleaned, &layer_map, &defines, &custom_keycodes);
    let tap_dance_map: HashMap<String, usize> = tap_dances
        .iter()
        .enumerate()
        .map(|(i, td)| (td.name.clone(), i))
        .collect();

    let raw_layers = extract_raw_layers(&cleaned)?;

    let mut layers: Vec<Layer> = raw_layers
        .into_iter()
        .map(|(name, raw_keys)| {
            let index = *layer_map.get(&name).unwrap_or(&0);
            let keys = raw_keys
                .iter()
                .map(|k| {
                    parse_key_expr_str(
                        k.trim(),
                        &layer_map,
                        &defines,
                        &custom_keycodes,
                        &tap_dance_map,
                    )
                })
                .collect();
            Layer { name, index, keys }
        })
        .collect();

    layers.sort_by_key(|l| l.index);

    Ok(Keymap {
        keyboard: None,
        layout: None,
        layers,
        macros: vec![],
        tap_dances,
        tri_layer,
    })
}

/// Parse a single raw QMK key expression string into a [`Key`].
///
/// This is public so the JSON parser can reuse the same QMK expression support
/// as the C parser. The lookup maps provide context that raw expressions may
/// need: symbolic layer names, `#define` aliases, custom keycodes, and known tap
/// dances.
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn parse_key_expr_str(
    s: &str,
    layer_map: &HashMap<String, usize>,
    defines: &HashMap<String, String>,
    custom_keycodes: &HashSet<String>,
    tap_dance_map: &HashMap<String, usize>,
) -> Key {
    let expr = parse_expr(s.trim());
    expr_to_key(&expr, layer_map, defines, custom_keycodes, tap_dance_map)
}

// ── Comment stripping ─────────────────────────────────────────────────────────

fn strip_comments(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if i + 1 < b.len() && b[i] == b'/' && b[i + 1] == b'*' {
            i += 2;
            while i + 1 < b.len() {
                if b[i] == b'*' && b[i + 1] == b'/' {
                    i += 2;
                    break;
                }
                if b[i] == b'\n' {
                    out.push('\n');
                }
                i += 1;
            }
        } else if i + 1 < b.len() && b[i] == b'/' && b[i + 1] == b'/' {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
        } else {
            out.push(b[i] as char);
            i += 1;
        }
    }
    out
}

// ── Layer name extraction ─────────────────────────────────────────────────────

/// Extract layer enum entries from the first enum that looks layer-shaped.
///
/// QMK keymaps commonly use `enum planck_layers { _BASE, ... }`, but the enum
/// name is not standardized. The heuristic accepts enums whose name contains
/// `layer` or whose first entry looks like a conventional layer symbol.
fn extract_layer_names(s: &str) -> Vec<String> {
    // Look for an enum whose name or contents suggest it's a layers enum.
    let mut search = s;
    while let Some(pos) = search.find("enum ") {
        let after = &search[pos + 5..];
        if let Some(brace) = after.find('{') {
            let body_start = brace + 1;
            if let Some(close) = find_matching(&after[brace..], '{', '}') {
                let body = &after[body_start..brace + close];
                let enum_name = after[..brace].trim().to_lowercase();
                let entries: Vec<String> = body
                    .split(',')
                    .map(|e| e.split('=').next().unwrap_or("").trim().to_string())
                    .filter(|e| !e.is_empty())
                    .collect();

                let looks_like_layers = enum_name.contains("layer")
                    || entries
                        .first()
                        .is_some_and(|e| e.starts_with('_') || e == "BASE" || e == "QWERTY");

                if looks_like_layers && !entries.is_empty() {
                    return entries;
                }
                search = &after[brace + close + 1..];
                continue;
            }
        }
        search = &search[pos + 5..];
    }
    vec![]
}

// ── #define extraction ────────────────────────────────────────────────────────

/// Extract simple one-line `#define NAME value` aliases.
///
/// The converter uses these primarily for key aliases such as `LOWER
/// MO(_LOWER)`. Function-like macros and empty defines are intentionally left
/// alone because expanding them without a preprocessor would be misleading.
fn extract_defines(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("#define ") {
            let mut parts = rest.splitn(2, |c: char| c.is_whitespace());
            if let (Some(name), Some(value)) = (parts.next(), parts.next()) {
                let name = name.trim().to_string();
                let value = value.trim().to_string();
                if !value.is_empty() {
                    map.insert(name, value);
                }
            }
        }
    }
    map
}

// ── Custom keycode extraction ─────────────────────────────────────────────────

/// Extract names from `enum custom_keycodes`.
///
/// Custom keycodes are treated as macro references in the IR. Their runtime QMK
/// behavior lives in user code outside the keymap matrix, so the converter emits
/// ZMK macro stubs rather than trying to infer the implementation.
fn extract_custom_keycodes(s: &str) -> HashSet<String> {
    let mut codes = HashSet::new();
    if let Some(pos) = s.find("enum custom_keycodes") {
        let after = &s[pos..];
        let parsed = after.find('{').and_then(|brace| {
            let close = find_matching(&after[brace..], '{', '}')?;
            Some((brace, close))
        });
        if let Some((brace, close)) = parsed {
            let body = &after[brace + 1..brace + close];
            for entry in body.split(',') {
                let name = entry.split('=').next().unwrap_or("").trim().to_string();
                if !name.is_empty() {
                    codes.insert(name);
                }
            }
        }
    }
    codes
}

// ── Tri-layer detection ───────────────────────────────────────────────────────

/// Detect a QMK `update_tri_layer_state(state, lower, upper, tri)` relationship.
///
/// Only the first occurrence is currently preserved. Layer arguments may be
/// symbolic names from the layer enum or numeric indices.
fn extract_tri_layer(s: &str, layer_map: &HashMap<String, usize>) -> Option<TriLayer> {
    // Look for update_tri_layer_state(state, LOWER_LAYER, UPPER_LAYER, TRI_LAYER)
    let pos = s.find("update_tri_layer_state")?;
    let after = &s[pos..];
    let paren = after.find('(')?;
    let close = find_matching(&after[paren..], '(', ')')?;
    let args_str = &after[paren + 1..paren + close];
    let args: Vec<&str> = split_args(args_str).into_iter().map(str::trim).collect();
    // args[0] is "state", args[1..3] are the three layer identifiers
    if args.len() >= 4 {
        let lower = resolve_layer(args[1], layer_map)?;
        let upper = resolve_layer(args[2], layer_map)?;
        let tri = resolve_layer(args[3], layer_map)?;
        return Some(TriLayer { lower, upper, tri });
    }
    None
}

fn resolve_layer(name: &str, layer_map: &HashMap<String, usize>) -> Option<usize> {
    layer_map.get(name).copied().or_else(|| name.parse().ok())
}

// ── Keymaps array extraction ──────────────────────────────────────────────────

/// Extract raw layer names and raw key argument strings from the `keymaps` array.
///
/// This function validates only the delimiter structure needed to find layer
/// initializers and split layout arguments. It returns raw key strings so later
/// stages can interpret QMK key expressions with the metadata collected above.
fn extract_raw_layers(s: &str) -> Result<Vec<(String, Vec<String>)>, ParseCError> {
    let keymaps_pos = s.find("keymaps").ok_or(ParseCError::NoKeymapsArray)?;
    let after = &s[keymaps_pos..];
    let brace = after.find('{').ok_or(ParseCError::NoKeymapsBrace)?;
    let close =
        find_matching(&after[brace..], '{', '}').ok_or(ParseCError::UnmatchedKeymapsBrace)?;
    let body = &after[brace + 1..brace + close];

    let mut layers = Vec::new();
    let mut remaining = body;

    while let Some(bracket) = remaining.find('[') {
        let after_bracket = &remaining[bracket + 1..];
        let close_bracket = after_bracket
            .find(']')
            .ok_or(ParseCError::UnclosedLayerBracket)?;
        let layer_name = after_bracket[..close_bracket].trim().to_string();

        let after_close = &after_bracket[close_bracket + 1..];
        let eq = after_close
            .find('=')
            .ok_or_else(|| ParseCError::MissingEquals {
                layer: layer_name.clone(),
            })?;
        let after_eq = after_close[eq + 1..].trim_start();

        let paren = after_eq
            .find('(')
            .ok_or_else(|| ParseCError::MissingLayoutParen {
                layer: layer_name.clone(),
            })?;
        let layout_rest = &after_eq[paren..];
        let close_paren = find_matching(layout_rest, '(', ')').ok_or_else(|| {
            ParseCError::UnmatchedLayoutParen {
                layer: layer_name.clone(),
            }
        })?;

        let keys_str = &layout_rest[1..close_paren];
        let keys: Vec<String> = split_args(keys_str)
            .into_iter()
            .map(|k| k.trim().to_string())
            .filter(|k| !k.is_empty())
            .collect();

        layers.push((layer_name, keys));
        remaining = &layout_rest[close_paren + 1..];
    }

    Ok(layers)
}

// ── Expression parser ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Expr {
    /// A bare token such as `KC_A`, `_LOWER`, or `MY_MACRO`.
    Atom(String),
    /// A simple function-style expression such as `MO(_LOWER)` or
    /// `LGUI(LSFT(KC_C))`.
    Call { name: String, args: Vec<Expr> },
}

impl Expr {
    fn as_atom(&self) -> Option<&str> {
        if let Expr::Atom(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

fn parse_expr(s: &str) -> Expr {
    let s = s.trim();
    if let Some(paren) = s.find('(') {
        let name = s[..paren].trim();
        if !name.is_empty() {
            let rest = &s[paren..];
            if let Some(close) = find_matching(rest, '(', ')') {
                let args_str = &rest[1..close];
                let args = split_args(args_str)
                    .into_iter()
                    .map(|a| parse_expr(a.trim()))
                    .collect();
                return Expr::Call {
                    name: name.to_string(),
                    args,
                };
            }
        }
    }
    Expr::Atom(s.to_string())
}

/// Split a comma-separated argument list while respecting nested parentheses.
fn split_args(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    let tail = &s[start..];
    if !tail.trim().is_empty() {
        parts.push(tail);
    }
    parts
}

// ── Expr → Key translation ────────────────────────────────────────────────────

fn expr_to_key(
    expr: &Expr,
    layer_map: &HashMap<String, usize>,
    defines: &HashMap<String, String>,
    custom_keycodes: &HashSet<String>,
    tap_dance_map: &HashMap<String, usize>,
) -> Key {
    match expr {
        Expr::Atom(name) => atom_to_key(name, layer_map, defines, custom_keycodes, tap_dance_map),
        Expr::Call { name, args } => func_to_key(
            name,
            args,
            layer_map,
            defines,
            custom_keycodes,
            tap_dance_map,
        ),
    }
}

/// Convert a bare QMK token into an IR key.
///
/// Alias expansion happens first, then exact QMK sentinels and special
/// behaviors are handled before falling through to mapping tables.
fn atom_to_key(
    name: &str,
    layer_map: &HashMap<String, usize>,
    defines: &HashMap<String, String>,
    custom_keycodes: &HashSet<String>,
    tap_dance_map: &HashMap<String, usize>,
) -> Key {
    // Expand #define aliases first
    if let Some(expansion) = defines.get(name) {
        let expr = parse_expr(expansion);
        return expr_to_key(&expr, layer_map, defines, custom_keycodes, tap_dance_map);
    }

    match name {
        "KC_TRANSPARENT" | "KC_TRNS" | "_______" => return Key::Trans,
        "KC_NO" | "XXXXXXX" => return Key::None,
        "CW_TOGG" => return Key::CapsWord,
        "QK_BOOT" => return Key::Bootloader,
        "QK_RBT" | "QK_RESET" => return Key::SysReset,
        // No ZMK equivalents yet
        s if s.starts_with("QK_DYNAMIC_TAPPING_TERM") => return Key::Unknown(name.to_string()),
        _ => {}
    }

    if custom_keycodes.contains(name) {
        return Key::Macro(name.to_string());
    }

    if let Some(action) = codes::qmk_rgb_to_zmk(name) {
        return Key::RgbUg(action.to_string());
    }

    if let Some(k) = qmk_mouse_to_zmk_key(name) {
        return k;
    }

    if let Some(zmk) = codes::qmk_key_to_zmk(name) {
        return Key::Kp(zmk.to_string());
    }

    Key::Unknown(name.to_string())
}

/// Convert a QMK function-style expression into an IR key.
///
/// Layer behaviors resolve symbolic layer names against `layer_map`; modifier
/// wrappers are folded into ZMK-style key expressions such as `LG(C)`.
fn func_to_key(
    name: &str,
    args: &[Expr],
    layer_map: &HashMap<String, usize>,
    _defines: &HashMap<String, String>,
    _custom_keycodes: &HashSet<String>,
    tap_dance_map: &HashMap<String, usize>,
) -> Key {
    match name {
        "MT" if args.len() == 2 => {
            let mod_str = args[0].as_atom().unwrap_or("").trim();
            let key_str = args[1].as_atom().unwrap_or("").trim();
            let zmk_mod = codes::qmk_mod_to_zmk(mod_str).to_string();
            let zmk_key = codes::qmk_key_to_zmk(key_str)
                .unwrap_or(key_str)
                .to_string();
            Key::Mt(zmk_mod, zmk_key)
        }
        "MO" if args.len() == 1 => {
            let layer = args[0].as_atom().unwrap_or("").trim();
            match resolve_layer(layer, layer_map) {
                Some(idx) => Key::Mo(idx),
                None => Key::Unknown(format!("MO({layer})")),
            }
        }
        "TG" if args.len() == 1 => {
            let layer = args[0].as_atom().unwrap_or("").trim();
            match resolve_layer(layer, layer_map) {
                Some(idx) => Key::Tog(idx),
                None => Key::Unknown(format!("TG({layer})")),
            }
        }
        "LT" if args.len() == 2 => {
            let layer = args[0].as_atom().unwrap_or("").trim();
            let key_str = args[1].as_atom().unwrap_or("").trim();
            let zmk_key = codes::qmk_key_to_zmk(key_str)
                .unwrap_or(key_str)
                .to_string();
            match resolve_layer(layer, layer_map) {
                Some(idx) => Key::Lt(idx, zmk_key),
                None => Key::Unknown(format!("LT({layer}, {key_str})")),
            }
        }
        "OSM" if args.len() == 1 => {
            let mod_str = args[0].as_atom().unwrap_or("").trim();
            Key::Sk(codes::qmk_mod_to_zmk(mod_str).to_string())
        }
        "OSL" if args.len() == 1 => {
            let layer = args[0].as_atom().unwrap_or("").trim();
            match resolve_layer(layer, layer_map) {
                Some(idx) => Key::Sl(idx),
                None => Key::Unknown(format!("OSL({layer})")),
            }
        }
        "TO" if args.len() == 1 => {
            let layer = args[0].as_atom().unwrap_or("").trim();
            match resolve_layer(layer, layer_map) {
                Some(idx) => Key::To(idx),
                None => Key::Unknown(format!("TO({layer})")),
            }
        }
        "DF" if args.len() == 1 => {
            let layer = args[0].as_atom().unwrap_or("").trim();
            match resolve_layer(layer, layer_map) {
                Some(idx) => Key::Df(idx),
                None => Key::Unknown(format!("DF({layer})")),
            }
        }
        "TD" if args.len() == 1 => {
            let dance_name = args[0].as_atom().unwrap_or("").trim();
            tap_dance_map
                .get(dance_name)
                .copied()
                .map_or_else(|| Key::Unknown(format!("TD({dance_name})")), Key::TapDance)
        }
        "LM" => Key::Unknown(format!(
            "LM({}) /* layer-mod: no ZMK equivalent */",
            args.iter()
                .filter_map(|a| a.as_atom())
                .collect::<Vec<_>>()
                .join(", ")
        )),
        "HYPR" if args.len() == 1 => {
            let inner = build_zmk_key_expr(&args[0]);
            Key::Kp(format!("LG(LA(LS(LC({inner}))))"))
        }
        "MEH" if args.len() == 1 => {
            let inner = build_zmk_key_expr(&args[0]);
            Key::Kp(format!("LA(LS(LC({inner})))"))
        }
        // Modifier-wrapping functions: LGUI(x), LSFT(x), etc.
        mod_fn if codes::qmk_mod_fn_to_zmk(mod_fn).is_some() && args.len() == 1 => {
            let prefix = codes::qmk_mod_fn_to_zmk(mod_fn).unwrap();
            let inner = build_zmk_key_expr(&args[0]);
            Key::Kp(format!("{prefix}({inner})"))
        }
        _ => Key::Unknown(format!(
            "{}({})",
            name,
            args.iter()
                .map(|a| format!("{a:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn qmk_mouse_to_zmk_key(name: &str) -> Option<Key> {
    let key = name.strip_prefix("KC_").unwrap_or(name);
    Some(match key {
        "MS_U" | "MS_UP" => Key::Mmv("MOVE_UP".into()),
        "MS_D" | "MS_DOWN" => Key::Mmv("MOVE_DOWN".into()),
        "MS_L" | "MS_LEFT" => Key::Mmv("MOVE_LEFT".into()),
        "MS_R" | "MS_RIGHT" => Key::Mmv("MOVE_RIGHT".into()),
        "BTN1" => Key::Mkp("LCLK".into()),
        "BTN2" => Key::Mkp("RCLK".into()),
        "BTN3" => Key::Mkp("MCLK".into()),
        "BTN4" => Key::Mkp("BTN4".into()),
        "BTN5" => Key::Mkp("BTN5".into()),
        "WH_U" | "MS_WH_UP" => Key::Msc("SCRL_UP".into()),
        "WH_D" | "MS_WH_DOWN" => Key::Msc("SCRL_DOWN".into()),
        "WH_L" | "MS_WH_LEFT" => Key::Msc("SCRL_LEFT".into()),
        "WH_R" | "MS_WH_RIGHT" => Key::Msc("SCRL_RIGHT".into()),
        _ => return None,
    })
}

/// Recursively build a ZMK key expression string for nested mod wrappers.
/// E.g. `LGUI(LSFT(KC_LBRC))` → "LG(LS(LBKT))"
fn build_zmk_key_expr(expr: &Expr) -> String {
    match expr {
        Expr::Atom(name) => codes::qmk_key_to_zmk(name.trim())
            .unwrap_or(name.trim())
            .to_string(),
        Expr::Call { name, args } if args.len() == 1 => {
            if let Some(prefix) = codes::qmk_mod_fn_to_zmk(name) {
                return format!("{}({})", prefix, build_zmk_key_expr(&args[0]));
            }
            format!("{name}_UNKNOWN")
        }
        Expr::Call { name, .. } => format!("{name}_UNKNOWN"),
    }
}

// ── Tap dance extraction ──────────────────────────────────────────────────────

fn extract_tap_dances(
    s: &str,
    layer_map: &HashMap<String, usize>,
    defines: &HashMap<String, String>,
    custom_keycodes: &HashSet<String>,
) -> Vec<TapDanceDef> {
    let Some(pos) = s.find("tap_dance_actions") else {
        return vec![];
    };
    let after = &s[pos + "tap_dance_actions".len()..];
    let Some(brace_rel) = after.find('{') else {
        return vec![];
    };
    let Some(close) = find_matching(&after[brace_rel..], '{', '}') else {
        return vec![];
    };
    let body = &after[brace_rel + 1..brace_rel + close];

    let mut tap_dances = Vec::new();
    let mut search = body;

    while !search.is_empty() {
        let Some(br) = search.find('[') else { break };
        let after_open = &search[br + 1..];
        let Some(cbr) = after_open.find(']') else {
            break;
        };
        let name = after_open[..cbr].trim().to_string();

        let after_close = &after_open[cbr + 1..];
        let Some(eq) = after_close.find('=') else {
            break;
        };
        let action_src = after_close[eq + 1..].trim_start();

        let bindings = parse_tap_dance_action(action_src, layer_map, defines, custom_keycodes);
        tap_dances.push(TapDanceDef { name, bindings });

        // Advance past the closing paren of this action call
        if let Some(p) = action_src.find('(') {
            if let Some(cp) = find_matching(&action_src[p..], '(', ')') {
                search = &action_src[p + cp + 1..];
            } else {
                break;
            }
        } else {
            break;
        }
    }

    tap_dances
}

/// Parse a QMK `ACTION_TAP_DANCE_*` initializer into tap-dance bindings.
///
/// Simple double-tap actions are represented directly. Function-backed tap
/// dances require user code, so they become empty definitions that render as
/// stubs in the target format.
fn parse_tap_dance_action(
    s: &str,
    layer_map: &HashMap<String, usize>,
    defines: &HashMap<String, String>,
    custom_keycodes: &HashSet<String>,
) -> Vec<Key> {
    let s = s.trim();
    let Some(paren) = s.find('(') else {
        return vec![];
    };
    let action_type = s[..paren].trim();
    let rest = &s[paren..];
    let Some(close) = find_matching(rest, '(', ')') else {
        return vec![];
    };
    let args_str = &rest[1..close];

    if action_type == "ACTION_TAP_DANCE_DOUBLE" {
        let args = split_args(args_str);
        if args.len() == 2 {
            let empty_td = HashMap::new();
            return vec![
                parse_key_expr_str(
                    args[0].trim(),
                    layer_map,
                    defines,
                    custom_keycodes,
                    &empty_td,
                ),
                parse_key_expr_str(
                    args[1].trim(),
                    layer_map,
                    defines,
                    custom_keycodes,
                    &empty_td,
                ),
            ];
        }
    }
    // ACTION_TAP_DANCE_FN, ACTION_TAP_DANCE_FN_ADVANCED, etc. → stub
    vec![]
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// Find the index of the closing delimiter matching the opening at position 0.
fn find_matching(s: &str, open: char, close: char) -> Option<usize> {
    let mut depth = 0usize;
    for (i, ch) in s.char_indices() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn layer_map(pairs: &[(&str, usize)]) -> HashMap<String, usize> {
        pairs.iter().map(|(k, v)| ((*k).to_string(), *v)).collect()
    }

    fn defines(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    fn custom(names: &[&str]) -> HashSet<String> {
        names.iter().map(|s| (*s).to_string()).collect()
    }

    fn key(s: &str) -> Key {
        parse_key_expr_str(
            s,
            &HashMap::new(),
            &HashMap::new(),
            &HashSet::new(),
            &HashMap::new(),
        )
    }

    fn key_with(
        s: &str,
        lm: &HashMap<String, usize>,
        defs: &HashMap<String, String>,
        cc: &HashSet<String>,
    ) -> Key {
        parse_key_expr_str(s, lm, defs, cc, &HashMap::new())
    }

    // ── strip_comments ────────────────────────────────────────────────────────

    #[test]
    fn strip_line_comment() {
        assert_eq!(strip_comments("a // comment\nb"), "a \nb");
    }

    #[test]
    fn strip_block_comment() {
        assert_eq!(strip_comments("a /* block */ b"), "a  b");
    }

    #[test]
    fn strip_block_comment_preserves_newlines() {
        let out = strip_comments("a /* line1\nline2 */ b");
        assert!(out.contains('\n'));
        assert!(!out.contains("line1"));
    }

    #[test]
    fn strip_no_comments() {
        let src = "KC_TAB, KC_Q,";
        assert_eq!(strip_comments(src), src);
    }

    // ── extract_layer_names ───────────────────────────────────────────────────

    #[test]
    fn extracts_planck_layers() {
        let src = "enum planck_layers { _BASE, _LOWER, _RAISE, _ADJUST, };";
        let names = extract_layer_names(src);
        assert_eq!(names, vec!["_BASE", "_LOWER", "_RAISE", "_ADJUST"]);
    }

    #[test]
    fn extracts_layers_by_heuristic() {
        // Name doesn't contain "layer" but first entry starts with _
        let src = "enum my_kb { _BASE, _NUM, };";
        let names = extract_layer_names(src);
        assert_eq!(names, vec!["_BASE", "_NUM"]);
    }

    #[test]
    fn skips_non_layer_enums() {
        // custom_keycodes enum shouldn't be detected as layers
        let src = "enum custom_keycodes { RGB_SLD = SAFE_RANGE, ST_MACRO_0, }; \
                   enum planck_layers { _BASE, _LOWER, };";
        let names = extract_layer_names(src);
        assert_eq!(names, vec!["_BASE", "_LOWER"]);
    }

    // ── extract_defines ───────────────────────────────────────────────────────

    #[test]
    fn extracts_layer_defines() {
        let src = "#define LOWER MO(_LOWER)\n#define RAISE MO(_RAISE)\n";
        let defs = extract_defines(src);
        assert_eq!(defs.get("LOWER").map(String::as_str), Some("MO(_LOWER)"));
        assert_eq!(defs.get("RAISE").map(String::as_str), Some("MO(_RAISE)"));
    }

    #[test]
    fn ignores_empty_defines() {
        let src = "#define EMPTY\n#define KEY VALUE\n";
        let defs = extract_defines(src);
        assert!(!defs.contains_key("EMPTY"));
        assert!(defs.contains_key("KEY"));
    }

    // ── extract_custom_keycodes ───────────────────────────────────────────────

    #[test]
    fn extracts_custom_keycodes() {
        let src = "enum custom_keycodes { RGB_SLD = ZSA_SAFE_RANGE, ST_MACRO_0, };";
        let cc = extract_custom_keycodes(src);
        assert!(cc.contains("RGB_SLD"));
        assert!(cc.contains("ST_MACRO_0"));
    }

    // ── parse_key_expr_str: basic keys ───────────────────────────────────────

    #[test]
    fn basic_key() {
        assert!(matches!(key("KC_TAB"), Key::Kp(k) if k == "TAB"));
        assert!(matches!(key("KC_Q"),   Key::Kp(k) if k == "Q"));
        assert!(matches!(key("KC_BSPC"),Key::Kp(k) if k == "BSPC"));
    }

    #[test]
    fn transparent_variants() {
        assert!(matches!(key("KC_TRANSPARENT"), Key::Trans));
        assert!(matches!(key("KC_TRNS"), Key::Trans));
        assert!(matches!(key("_______"), Key::Trans));
    }

    #[test]
    fn none_variants() {
        assert!(matches!(key("KC_NO"), Key::None));
        assert!(matches!(key("XXXXXXX"), Key::None));
    }

    #[test]
    fn special_behaviors() {
        assert!(matches!(key("CW_TOGG"), Key::CapsWord));
        assert!(matches!(key("QK_BOOT"), Key::Bootloader));
        assert!(matches!(key("QK_RBT"), Key::SysReset));
    }

    #[test]
    fn dynamic_tapping_term_is_unknown() {
        assert!(matches!(
            key("QK_DYNAMIC_TAPPING_TERM_DOWN"),
            Key::Unknown(_)
        ));
    }

    // ── parse_key_expr_str: function calls ───────────────────────────────────

    #[test]
    fn mod_tap() {
        let lm = layer_map(&[("_LOWER", 1), ("_RAISE", 2)]);
        let key = key_with("MT(MOD_LALT, KC_Z)", &lm, &HashMap::new(), &HashSet::new());
        assert!(matches!(key, Key::Mt(m, k) if m == "LALT" && k == "Z"));
    }

    #[test]
    fn mod_tap_right_side() {
        let key = key("MT(MOD_RGUI, KC_H)");
        assert!(matches!(key, Key::Mt(m, k) if m == "RGUI" && k == "H"));
    }

    #[test]
    fn momentary_layer() {
        let lm = layer_map(&[("_BASE", 0), ("_LOWER", 1), ("_RAISE", 2)]);
        let key = key_with("MO(_LOWER)", &lm, &HashMap::new(), &HashSet::new());
        assert!(matches!(key, Key::Mo(1)));
    }

    #[test]
    fn toggle_layer() {
        let lm = layer_map(&[("_BASE", 0), ("_LOWER", 1)]);
        let key = key_with("TG(_LOWER)", &lm, &HashMap::new(), &HashSet::new());
        assert!(matches!(key, Key::Tog(1)));
    }

    #[test]
    fn layer_tap() {
        let lm = layer_map(&[("_LOWER", 1)]);
        let key = key_with(
            "LT(_LOWER, KC_SPACE)",
            &lm,
            &HashMap::new(),
            &HashSet::new(),
        );
        assert!(matches!(key, Key::Lt(1, k) if k == "SPACE"));
    }

    #[test]
    fn modifier_wrapping_simple() {
        let key = key("LGUI(KC_C)");
        assert!(matches!(key, Key::Kp(k) if k == "LG(C)"));
    }

    #[test]
    fn modifier_wrapping_nested() {
        let key = key("LGUI(LSFT(KC_LBRC))");
        assert!(matches!(key, Key::Kp(k) if k == "LG(LS(LBKT))"));
    }

    #[test]
    fn rgui_rsft_nested() {
        let key = key("RGUI(RSFT(KC_RBRC))");
        assert!(matches!(key, Key::Kp(k) if k == "RG(RS(RBKT))"));
    }

    #[test]
    fn rgb_functions() {
        assert!(matches!(key("RGB_TOG"),          Key::RgbUg(a) if a == "RGB_TOG"));
        assert!(matches!(key("RGB_HUI"),          Key::RgbUg(a) if a == "RGB_HUI"));
        assert!(matches!(key("RGB_MODE_FORWARD"), Key::RgbUg(a) if a == "RGB_EFF"));
    }

    // ── parse_key_expr_str: defines & custom keycodes ────────────────────────

    #[test]
    fn define_expansion() {
        let lm = layer_map(&[("_LOWER", 1), ("_RAISE", 2)]);
        let defs = defines(&[("LOWER", "MO(_LOWER)"), ("RAISE", "MO(_RAISE)")]);
        let key = key_with("LOWER", &lm, &defs, &HashSet::new());
        assert!(matches!(key, Key::Mo(1)));
    }

    #[test]
    fn custom_keycode_becomes_macro() {
        let cc = custom(&["ST_MACRO_0", "RGB_SLD"]);
        let key = key_with("ST_MACRO_0", &HashMap::new(), &HashMap::new(), &cc);
        assert!(matches!(key, Key::Macro(n) if n == "ST_MACRO_0"));
    }

    #[test]
    fn unknown_keycode() {
        let key = key("SOME_UNRECOGNISED_CODE");
        assert!(matches!(key, Key::Unknown(_)));
    }

    #[test]
    fn one_shot_mod() {
        let k = key("OSM(MOD_LSFT)");
        assert!(matches!(k, Key::Sk(m) if m == "LSHFT"));
    }

    #[test]
    fn one_shot_layer() {
        let lm = layer_map(&[("_FN", 1)]);
        let k = key_with("OSL(_FN)", &lm, &HashMap::new(), &HashSet::new());
        assert!(matches!(k, Key::Sl(1)));
    }

    #[test]
    fn to_layer() {
        let lm = layer_map(&[("_BASE", 0)]);
        let k = key_with("TO(_BASE)", &lm, &HashMap::new(), &HashSet::new());
        assert!(matches!(k, Key::To(0)));
    }

    #[test]
    fn default_layer() {
        let lm = layer_map(&[("_BASE", 0), ("_QWERTY", 1)]);
        let k = key_with("DF(_QWERTY)", &lm, &HashMap::new(), &HashSet::new());
        assert!(matches!(k, Key::Df(1)));
    }

    #[test]
    fn mouse_scroll_keys() {
        assert!(matches!(key("KC_WH_U"), Key::Msc(d) if d == "SCRL_UP"));
        assert!(matches!(key("KC_WH_D"), Key::Msc(d) if d == "SCRL_DOWN"));
        assert!(matches!(key("KC_WH_L"), Key::Msc(d) if d == "SCRL_LEFT"));
        assert!(matches!(key("KC_WH_R"), Key::Msc(d) if d == "SCRL_RIGHT"));
    }

    #[test]
    fn tap_dance_is_unknown_when_not_in_map() {
        // TD with no tap-dance map → Unknown
        let k = key("TD(DANCE_0)");
        assert!(matches!(k, Key::Unknown(s) if s.contains("TD") && s.contains("DANCE_0")));
    }

    #[test]
    fn tap_dance_resolved_when_in_map() {
        let td_map: HashMap<String, usize> =
            [("DANCE_0".to_string(), 0), ("DANCE_1".to_string(), 1)].into();
        let k = parse_key_expr_str(
            "TD(DANCE_0)",
            &HashMap::new(),
            &HashMap::new(),
            &HashSet::new(),
            &td_map,
        );
        assert!(matches!(k, Key::TapDance(0)));
        let k2 = parse_key_expr_str(
            "TD(DANCE_1)",
            &HashMap::new(),
            &HashMap::new(),
            &HashSet::new(),
            &td_map,
        );
        assert!(matches!(k2, Key::TapDance(1)));
    }

    #[test]
    fn extract_tap_dances_double() {
        let src = r"
enum layers { _BASE };
tap_dance_action_t tap_dance_actions[] = {
    [DANCE_0] = ACTION_TAP_DANCE_DOUBLE(KC_LSFT, KC_CAPS),
    [DANCE_1] = ACTION_TAP_DANCE_DOUBLE(KC_A, KC_B),
};
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
    [_BASE] = LAYOUT(TD(DANCE_0), TD(DANCE_1), KC_C),
};
";
        let km = parse(src).unwrap();
        assert_eq!(km.tap_dances.len(), 2);
        assert_eq!(km.tap_dances[0].name, "DANCE_0");
        assert_eq!(km.tap_dances[0].bindings.len(), 2);
        assert!(matches!(&km.tap_dances[0].bindings[0], Key::Kp(k) if k == "LSHFT"));
        assert!(matches!(&km.tap_dances[0].bindings[1], Key::Kp(k) if k == "CAPS"));
        assert_eq!(km.tap_dances[1].name, "DANCE_1");
        // Keymap cells resolved to TapDance keys
        assert!(matches!(&km.layers[0].keys[0], Key::TapDance(0)));
        assert!(matches!(&km.layers[0].keys[1], Key::TapDance(1)));
        assert!(matches!(&km.layers[0].keys[2], Key::Kp(k) if k == "C"));
    }

    #[test]
    fn extract_tap_dances_fn_advanced_is_stub() {
        let src = r"
enum layers { _BASE };
tap_dance_action_t tap_dance_actions[] = {
    [DANCE_0] = ACTION_TAP_DANCE_FN_ADVANCED(NULL, dance_0_finished, dance_0_reset),
};
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
    [_BASE] = LAYOUT(TD(DANCE_0)),
};
";
        let km = parse(src).unwrap();
        assert_eq!(km.tap_dances.len(), 1);
        assert_eq!(km.tap_dances[0].name, "DANCE_0");
        assert!(
            km.tap_dances[0].bindings.is_empty(),
            "fn_advanced should produce empty bindings"
        );
        assert!(matches!(&km.layers[0].keys[0], Key::TapDance(0)));
    }

    #[test]
    fn lm_is_unknown() {
        let lm = layer_map(&[("_LOWER", 1)]);
        let k = key_with(
            "LM(_LOWER, MOD_LSFT)",
            &lm,
            &HashMap::new(),
            &HashSet::new(),
        );
        assert!(matches!(k, Key::Unknown(s) if s.contains("LM")));
    }

    #[test]
    fn hypr_wraps_all_mods() {
        let k = key("HYPR(KC_A)");
        assert!(matches!(k, Key::Kp(s) if s == "LG(LA(LS(LC(A))))"));
    }

    #[test]
    fn meh_wraps_three_mods() {
        let k = key("MEH(KC_A)");
        assert!(matches!(k, Key::Kp(s) if s == "LA(LS(LC(A)))"));
    }

    #[test]
    fn mouse_move_keys() {
        assert!(matches!(key("KC_MS_U"), Key::Mmv(d) if d == "MOVE_UP"));
        assert!(matches!(key("KC_MS_D"), Key::Mmv(d) if d == "MOVE_DOWN"));
        assert!(matches!(key("KC_MS_L"), Key::Mmv(d) if d == "MOVE_LEFT"));
        assert!(matches!(key("KC_MS_R"), Key::Mmv(d) if d == "MOVE_RIGHT"));
    }

    #[test]
    fn mouse_button_keys() {
        assert!(matches!(key("KC_BTN1"), Key::Mkp(b) if b == "LCLK"));
        assert!(matches!(key("KC_BTN2"), Key::Mkp(b) if b == "RCLK"));
        assert!(matches!(key("KC_BTN3"), Key::Mkp(b) if b == "MCLK"));
    }

    // ── extract_raw_layers ────────────────────────────────────────────────────

    #[test]
    fn extracts_two_layers() {
        let src = r"
const uint16_t PROGMEM keymaps[][4][4] = {
  [_BASE] = LAYOUT_grid(
    KC_A, KC_B,
    KC_C, KC_D
  ),
  [_FN] = LAYOUT_grid(
    KC_TRANSPARENT, KC_NO,
    KC_X, KC_Y
  ),
};";
        let layers = extract_raw_layers(src).unwrap();
        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].0, "_BASE");
        assert_eq!(layers[0].1.len(), 4);
        assert_eq!(layers[1].0, "_FN");
    }

    // ── Full parse round-trip ─────────────────────────────────────────────────

    #[test]
    fn full_parse_minimal() {
        let src = r"
enum test_layers { _BASE, _FN, };
#define FN MO(_FN)
const uint16_t PROGMEM keymaps[][1][4] = {
  [_BASE] = LAYOUT(KC_A, KC_B, FN, KC_TRANSPARENT),
  [_FN]   = LAYOUT(KC_X, KC_NO, KC_TRANSPARENT, KC_TRANSPARENT),
};
";
        let km = super::parse(src).unwrap();
        assert_eq!(km.layers.len(), 2);

        let base = &km.layers[0];
        assert_eq!(base.index, 0);
        assert!(matches!(&base.keys[0], Key::Kp(k) if k == "A"));
        assert!(matches!(&base.keys[2], Key::Mo(1)));
        assert!(matches!(&base.keys[3], Key::Trans));

        let fn_layer = &km.layers[1];
        assert_eq!(fn_layer.index, 1);
        assert!(matches!(&fn_layer.keys[1], Key::None));
    }

    #[test]
    fn full_parse_with_tri_layer() {
        let src = r"
enum kb_layers { _BASE, _LOWER, _RAISE, _ADJUST, };
#define LOWER MO(_LOWER)
#define RAISE MO(_RAISE)
const uint16_t PROGMEM keymaps[][1][2] = {
  [_BASE]   = LAYOUT(LOWER, RAISE),
  [_LOWER]  = LAYOUT(KC_TRANSPARENT, KC_TRANSPARENT),
  [_RAISE]  = LAYOUT(KC_TRANSPARENT, KC_TRANSPARENT),
  [_ADJUST] = LAYOUT(KC_TRANSPARENT, KC_TRANSPARENT),
};
uint8_t layer_state_set_user(uint8_t state) {
    return update_tri_layer_state(state, _LOWER, _RAISE, _ADJUST);
}
";
        let km = super::parse(src).unwrap();
        let tri = km.tri_layer.expect("tri-layer should be detected");
        assert_eq!(tri.lower, 1);
        assert_eq!(tri.upper, 2);
        assert_eq!(tri.tri, 3);
    }
}
