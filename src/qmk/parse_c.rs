use std::collections::{HashMap, HashSet};

use crate::codes;
use crate::error::ParseCError;
use crate::ir::{Key, Keymap, Layer, TriLayer};

// ── Public entry point ────────────────────────────────────────────────────────

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

    let raw_layers = extract_raw_layers(&cleaned)?;

    let mut layers: Vec<Layer> = raw_layers
        .into_iter()
        .map(|(name, raw_keys)| {
            let index = *layer_map.get(&name).unwrap_or(&0);
            let keys = raw_keys
                .iter()
                .map(|k| parse_key_expr_str(k.trim(), &layer_map, &defines, &custom_keycodes))
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
        tri_layer,
    })
}

/// Parse a single raw key expression string into a [`Key`].  Public so the
/// JSON parser can reuse the same logic.
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn parse_key_expr_str(
    s: &str,
    layer_map: &HashMap<String, usize>,
    defines: &HashMap<String, String>,
    custom_keycodes: &HashSet<String>,
) -> Key {
    let expr = parse_expr(s.trim());
    expr_to_key(&expr, layer_map, defines, custom_keycodes)
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
    layer_map
        .get(name)
        .copied()
        .or_else(|| name.parse().ok())
}

// ── Keymaps array extraction ──────────────────────────────────────────────────

fn extract_raw_layers(s: &str) -> Result<Vec<(String, Vec<String>)>, ParseCError> {
    let keymaps_pos = s.find("keymaps").ok_or(ParseCError::NoKeymapsArray)?;
    let after = &s[keymaps_pos..];
    let brace = after.find('{').ok_or(ParseCError::NoKeymapsBrace)?;
    let close = find_matching(&after[brace..], '{', '}')
        .ok_or(ParseCError::UnmatchedKeymapsBrace)?;
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
            .ok_or_else(|| ParseCError::MissingEquals { layer: layer_name.clone() })?;
        let after_eq = after_close[eq + 1..].trim_start();

        let paren = after_eq
            .find('(')
            .ok_or_else(|| ParseCError::MissingLayoutParen { layer: layer_name.clone() })?;
        let layout_rest = &after_eq[paren..];
        let close_paren = find_matching(layout_rest, '(', ')')
            .ok_or_else(|| ParseCError::UnmatchedLayoutParen { layer: layer_name.clone() })?;

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
    Atom(String),
    Call { name: String, args: Vec<Expr> },
}

impl Expr {
    fn as_atom(&self) -> Option<&str> {
        if let Expr::Atom(s) = self { Some(s) } else { None }
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
                return Expr::Call { name: name.to_string(), args };
            }
        }
    }
    Expr::Atom(s.to_string())
}

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
) -> Key {
    match expr {
        Expr::Atom(name) => atom_to_key(name, layer_map, defines, custom_keycodes),
        Expr::Call { name, args } => func_to_key(name, args, layer_map, defines, custom_keycodes),
    }
}

fn atom_to_key(
    name: &str,
    layer_map: &HashMap<String, usize>,
    defines: &HashMap<String, String>,
    custom_keycodes: &HashSet<String>,
) -> Key {
    // Expand #define aliases first
    if let Some(expansion) = defines.get(name) {
        let expr = parse_expr(expansion);
        return expr_to_key(&expr, layer_map, defines, custom_keycodes);
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

fn func_to_key(
    name: &str,
    args: &[Expr],
    layer_map: &HashMap<String, usize>,
    _defines: &HashMap<String, String>,
    _custom_keycodes: &HashSet<String>,
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
        "TD" => Key::Unknown(format!(
            "TD({}) /* tap dance: no ZMK equivalent */",
            args.iter().filter_map(|a| a.as_atom()).collect::<Vec<_>>().join(", ")
        )),
        "LM" => Key::Unknown(format!(
            "LM({}) /* layer-mod: no ZMK equivalent */",
            args.iter().filter_map(|a| a.as_atom()).collect::<Vec<_>>().join(", ")
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
        "MS_U" | "MS_UP"    => Key::Mmv("MOVE_UP".into()),
        "MS_D" | "MS_DOWN"  => Key::Mmv("MOVE_DOWN".into()),
        "MS_L" | "MS_LEFT"  => Key::Mmv("MOVE_LEFT".into()),
        "MS_R" | "MS_RIGHT" => Key::Mmv("MOVE_RIGHT".into()),
        "BTN1"              => Key::Mkp("LCLK".into()),
        "BTN2"              => Key::Mkp("RCLK".into()),
        "BTN3"              => Key::Mkp("MCLK".into()),
        "BTN4"              => Key::Mkp("BTN4".into()),
        "BTN5"              => Key::Mkp("BTN5".into()),
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
        pairs.iter().map(|(k, v)| ((*k).to_string(), (*v).to_string())).collect()
    }

    fn custom(names: &[&str]) -> HashSet<String> {
        names.iter().map(|s| (*s).to_string()).collect()
    }

    fn key(s: &str) -> Key {
        parse_key_expr_str(s, &HashMap::new(), &HashMap::new(), &HashSet::new())
    }

    fn key_with(
        s: &str,
        lm: &HashMap<String, usize>,
        defs: &HashMap<String, String>,
        cc: &HashSet<String>,
    ) -> Key {
        parse_key_expr_str(s, lm, defs, cc)
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
        assert!(matches!(key("KC_TRNS"),        Key::Trans));
        assert!(matches!(key("_______"),         Key::Trans));
    }

    #[test]
    fn none_variants() {
        assert!(matches!(key("KC_NO"),   Key::None));
        assert!(matches!(key("XXXXXXX"), Key::None));
    }

    #[test]
    fn special_behaviors() {
        assert!(matches!(key("CW_TOGG"),  Key::CapsWord));
        assert!(matches!(key("QK_BOOT"),  Key::Bootloader));
        assert!(matches!(key("QK_RBT"),   Key::SysReset));
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
        let key = key_with("LT(_LOWER, KC_SPACE)", &lm, &HashMap::new(), &HashSet::new());
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
    fn tap_dance_is_unknown() {
        let k = key("TD(DANCE_0)");
        assert!(matches!(k, Key::Unknown(s) if s.contains("TD")));
    }

    #[test]
    fn lm_is_unknown() {
        let lm = layer_map(&[("_LOWER", 1)]);
        let k = key_with("LM(_LOWER, MOD_LSFT)", &lm, &HashMap::new(), &HashSet::new());
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
