//! Parser for ZMK `.keymap` DTS overlays.
//!
//! Like the QMK C parser, this is a targeted structural parser rather than a
//! complete DTS implementation. It understands the blocks this converter emits
//! and the common ZMK keymap shape: `keymap`, `behaviors` tap dances, `macros`,
//! and `conditional_layers`. Unknown or unsupported behaviors are preserved as
//! [`Key::Unknown`].

use std::collections::{HashMap, HashSet};

use crate::codes::{KeyExpr, Modifier, MouseButton, MouseMovement, MouseScroll, RgbAction};
use crate::error::ParseZmkError;
use crate::ir::{Key, Keyboard, Layer, MacroDef, MacroStep, TapDanceDef, TriLayer};

/// Parse a ZMK `.keymap` DTS overlay into the internal `Keyboard` IR.
///
/// # Errors
/// Returns [`ParseZmkError::NoKeymapBlock`] if no `keymap {}` block is found.
/// Returns [`ParseZmkError::UnclosedBlock`] if a block brace is never closed.
pub fn parse(source: &str) -> Result<Keyboard, ParseZmkError> {
    let s = strip_comments(source);
    let tri_layer = extract_tri_layer(&s);
    let macros = extract_macros(&s);
    let macro_names: HashSet<&str> = macros.iter().map(|m| m.name.as_str()).collect();
    let tap_dances = extract_behaviors_tap_dances(&s, &macro_names);
    let tap_dance_labels: HashMap<&str, usize> = tap_dances
        .iter()
        .enumerate()
        .map(|(i, td)| (td.name.as_str(), i))
        .collect();
    let keymap_body = block_content(&s, "keymap").ok_or(ParseZmkError::NoKeymapBlock)?;
    let layers = extract_layers(keymap_body, &macro_names, &tap_dance_labels)?;
    Ok(Keyboard {
        keyboard: None,
        layout: None,
        layers,
        macros,
        tap_dances,
        tri_layer,
    })
}

fn strip_comments(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' {
            if chars.peek() == Some(&'/') {
                for c2 in chars.by_ref() {
                    if c2 == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            } else if chars.peek() == Some(&'*') {
                chars.next();
                while let Some(c2) = chars.next() {
                    if c2 == '*' && chars.peek() == Some(&'/') {
                        chars.next();
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Return the content between braces for the first `name { ... }` block found.
///
/// Matching is identifier-aware, so looking for `keymap` will not accidentally
/// match part of a longer node name. The returned slice excludes the surrounding
/// braces.
fn block_content<'a>(s: &'a str, name: &str) -> Option<&'a str> {
    let nlen = name.len();
    let mut pos = 0;
    while let Some(rel) = s[pos..].find(name) {
        let found = pos + rel;
        let after_name = found + nlen;
        pos = after_name;

        let prev_ok = found == 0
            || s[..found]
                .chars()
                .last()
                .is_none_or(|c| !c.is_alphanumeric() && c != '_');
        let next_ok = s[after_name..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_');
        if !prev_ok || !next_ok {
            continue;
        }

        let after = &s[after_name..];
        let trimmed = after.trim_start();
        if !trimmed.starts_with('{') {
            continue;
        }

        let ws = after.len() - trimmed.len();
        let brace_start = after_name + ws;
        if let Some(close) = find_matching(&s[brace_start..], '{', '}') {
            return Some(&s[brace_start + 1..brace_start + close]);
        }
    }
    None
}

fn find_matching(s: &str, open: char, close: char) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        if c == open {
            depth += 1;
        } else if c == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

/// Extract the first ZMK conditional-layer relationship, if present.
///
/// The IR currently models the common tri-layer form: two `if-layers` and one
/// `then-layer`.
fn extract_tri_layer(s: &str) -> Option<TriLayer> {
    let content = block_content(s, "conditional_layers")?;

    let if_pos = content.find("if-layers")?;
    let after_if = &content[if_pos..];
    let open = after_if.find('<')?;
    let close = after_if[open..].find('>')?;
    let nums: Vec<usize> = after_if[open + 1..open + close]
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    if nums.len() < 2 {
        return None;
    }

    let then_pos = content.find("then-layer")?;
    let after_then = &content[then_pos..];
    let open2 = after_then.find('<')?;
    let close2 = after_then[open2..].find('>')?;
    let tri: usize = after_then[open2 + 1..open2 + close2].trim().parse().ok()?;

    Some(TriLayer {
        lower: nums[0],
        upper: nums[1],
        tri,
    })
}

fn extract_macros(s: &str) -> Vec<MacroDef> {
    let Some(content) = block_content(s, "macros") else {
        return vec![];
    };
    let mut macros = Vec::new();
    let mut pos = 0;
    while let Some(brace_rel) = content[pos..].find('{') {
        let brace_pos = pos + brace_rel;
        let name = identifier_before(content, brace_pos);
        let Some(close) = find_matching(&content[brace_pos..], '{', '}') else {
            break;
        };
        let block = &content[brace_pos + 1..brace_pos + close];
        pos = brace_pos + close + 1;
        if name.is_empty() {
            continue;
        }
        let steps = parse_macro_steps(bindings_str(block).unwrap_or(""));
        macros.push(MacroDef {
            name: name.to_string(),
            steps,
        });
    }
    macros
}

/// Parse the supported subset of ZMK macro binding steps.
///
/// `&kp KEY` becomes [`MacroStep::Tap`], and `&macro_wait_time N` becomes
/// [`MacroStep::Wait`]. Other macro commands are skipped because the IR cannot
/// represent them yet.
fn parse_macro_steps(s: &str) -> Vec<MacroStep> {
    let mut steps = Vec::new();
    for chunk in s.split('&').skip(1) {
        let mut tokens = chunk.split_whitespace();
        match tokens.next() {
            Some("kp") => {
                if let Some(k) = tokens.next() {
                    steps.push(MacroStep::Tap(KeyExpr::parse_zmk(k)));
                }
            }
            Some("macro_wait_time") => {
                if let Some(ms) = tokens.next().and_then(|s| s.parse().ok()) {
                    steps.push(MacroStep::Wait(ms));
                }
            }
            Some(other) => {
                let _ = other.len();
            }
            None => {}
        }
    }
    steps
}

fn extract_layers(
    s: &str,
    macro_names: &HashSet<&str>,
    tap_dance_labels: &HashMap<&str, usize>,
) -> Result<Vec<Layer>, ParseZmkError> {
    let mut layers = Vec::new();
    let mut pos = 0;
    while let Some(brace_rel) = s[pos..].find('{') {
        let brace_pos = pos + brace_rel;
        let name = identifier_before(s, brace_pos);
        let Some(close) = find_matching(&s[brace_pos..], '{', '}') else {
            return Err(ParseZmkError::UnclosedBlock {
                context: if name.is_empty() {
                    "keymap".into()
                } else {
                    name.to_string()
                },
            });
        };
        let block = &s[brace_pos + 1..brace_pos + close];
        pos = brace_pos + close + 1;
        if !block.contains("bindings") {
            continue;
        }
        let keys = parse_binding_list(
            bindings_str(block).unwrap_or(""),
            macro_names,
            tap_dance_labels,
        );
        let idx = layers.len();
        layers.push(Layer {
            name: if name.is_empty() {
                format!("layer{idx}")
            } else {
                name.to_string()
            },
            index: idx,
            keys,
        });
    }
    Ok(layers)
}

/// Return the first DTS `bindings = <...>` list inside a block.
///
/// This helper is intentionally shallow. Tap-dance parsing has a separate path
/// because tap dances commonly use multiple `<...>` groups separated by commas.
fn bindings_str(block: &str) -> Option<&str> {
    let start = block.find("bindings")?;
    let after = &block[start + "bindings".len()..];
    let open = after.find('<')?;
    let inner = &after[open + 1..];
    let close = inner.find('>')?;
    Some(&inner[..close])
}

fn parse_binding_list(
    s: &str,
    macro_names: &HashSet<&str>,
    tap_dance_labels: &HashMap<&str, usize>,
) -> Vec<Key> {
    s.split('&')
        .skip(1)
        .filter_map(|chunk| {
            let tokens: Vec<&str> = chunk.split_whitespace().collect();
            if tokens.is_empty() {
                None
            } else {
                Some(binding_to_key(&tokens, macro_names, tap_dance_labels))
            }
        })
        .collect()
}

/// Convert one tokenized ZMK binding into an IR key.
///
/// The first token is the behavior name without `&`; subsequent tokens are
/// behavior parameters. Unsupported behaviors retain their source text in
/// [`Key::Unknown`].
fn binding_to_key(
    tokens: &[&str],
    macro_names: &HashSet<&str>,
    tap_dance_labels: &HashMap<&str, usize>,
) -> Key {
    match tokens[0] {
        "kp" => tokens
            .get(1)
            .map_or(Key::Unknown("kp".into()), |k| Key::Kp(KeyExpr::parse_zmk(k))),
        "mo" => tokens
            .get(1)
            .and_then(|n| n.parse().ok())
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::Mo),
        "lt" => {
            if let (Some(&n_str), Some(&k)) = (tokens.get(1), tokens.get(2)) {
                n_str.parse::<usize>().map_or_else(
                    |_| Key::Unknown(tokens.join(" ")),
                    |n| Key::Lt(n, KeyExpr::parse_zmk(k)),
                )
            } else {
                Key::Unknown(tokens.join(" "))
            }
        }
        "mt" => {
            if let (Some(&m), Some(&k)) = (tokens.get(1), tokens.get(2)) {
                match Modifier::from_zmk(m) {
                    Some(modifier) => Key::Mt(modifier, KeyExpr::parse_zmk(k)),
                    None => Key::Unknown(tokens.join(" ")),
                }
            } else {
                Key::Unknown(tokens.join(" "))
            }
        }
        "tog" => tokens
            .get(1)
            .and_then(|n| n.parse().ok())
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::Tog),
        "sk" => tokens
            .get(1)
            .and_then(|m| Modifier::from_zmk(m))
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::Sk),
        "sl" => tokens
            .get(1)
            .and_then(|n| n.parse().ok())
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::Sl),
        "to" => tokens
            .get(1)
            .and_then(|n| n.parse().ok())
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::Df),
        "mmv" => tokens
            .get(1)
            .and_then(|d| MouseMovement::from_zmk(d))
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::Mmv),
        "mkp" => tokens
            .get(1)
            .and_then(|b| MouseButton::from_zmk(b))
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::Mkp),
        "msc" => tokens
            .get(1)
            .and_then(|d| MouseScroll::from_zmk(d))
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::Msc),
        "bt" | "out" => Key::Unknown(format!("&{}", tokens.join(" "))),
        "trans" => Key::Trans,
        "none" => Key::None,
        "caps_word" => Key::CapsWord,
        "bootloader" => Key::Bootloader,
        "sys_reset" => Key::SysReset,
        "rgb_ug" => tokens
            .get(1)
            .and_then(|a| RgbAction::from_zmk(a))
            .map_or_else(|| Key::Unknown(tokens.join(" ")), Key::RgbUg),
        name if tap_dance_labels.contains_key(name) => Key::TapDance(tap_dance_labels[name]),
        name if macro_names.contains(name) => Key::Macro(name.to_string()),
        unsupported => {
            let _ = unsupported.len();
            Key::Unknown(format!("&{}", tokens.join(" ")))
        }
    }
}

// ── Behaviors block (tap dance) ───────────────────────────────────────────────

/// Extract ZMK tap-dance behaviors from the top-level `behaviors` block.
///
/// Labels are recorded so layer bindings such as `&td0` can become
/// [`Key::TapDance`] references into the returned definition list.
fn extract_behaviors_tap_dances(s: &str, macro_names: &HashSet<&str>) -> Vec<TapDanceDef> {
    let Some(content) = block_content(s, "behaviors") else {
        return vec![];
    };
    let mut tap_dances = Vec::new();
    let mut pos = 0;
    while let Some(brace_rel) = content[pos..].find('{') {
        let brace_pos = pos + brace_rel;
        let Some(close) = find_matching(&content[brace_pos..], '{', '}') else {
            break;
        };
        let block_body = &content[brace_pos + 1..brace_pos + close];
        pos = brace_pos + close + 1;
        if !block_body.contains("zmk,behavior-tap-dance") {
            continue;
        }
        let label = label_before_brace(content, brace_pos);
        let bindings = tap_dance_binding_list(block_body, macro_names);
        tap_dances.push(TapDanceDef {
            name: label.to_string(),
            bindings,
        });
    }
    tap_dances
}

/// Return the DTS label (before the colon) for the node whose opening brace is
/// at `brace_pos`.  Falls back to the node name if there is no label.
fn label_before_brace(s: &str, brace_pos: usize) -> &str {
    let before = s[..brace_pos].trim_end();
    if let Some(colon) = before.rfind(':') {
        let label_region = before[..colon].trim_end();
        let start = label_region
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map_or(0, |i| i + 1);
        &label_region[start..]
    } else {
        identifier_before(s, brace_pos)
    }
}

/// Parse the tap-dance `bindings = <&foo X>, <&bar Y>;` line in a behavior
/// block.  Each `<…>` group is a single binding.
fn tap_dance_binding_list(block: &str, macro_names: &HashSet<&str>) -> Vec<Key> {
    let Some(start) = block.find("bindings") else {
        return vec![];
    };
    let after_kw = &block[start + "bindings".len()..];
    let semi_end = after_kw.find(';').unwrap_or(after_kw.len());
    let region = &after_kw[..semi_end];

    let empty_td_labels = HashMap::new();
    let mut keys = Vec::new();
    let mut pos = 0;
    while let Some(open_rel) = region[pos..].find('<') {
        let open = pos + open_rel + 1;
        let Some(close_rel) = region[open..].find('>') else {
            break;
        };
        let inner = &region[open..open + close_rel];
        if let Some(after_amp) = inner.split('&').nth(1) {
            let tokens: Vec<&str> = after_amp.split_whitespace().collect();
            if !tokens.is_empty() {
                keys.push(binding_to_key(&tokens, macro_names, &empty_td_labels));
            }
        }
        pos = open + close_rel + 1;
    }
    keys
}

/// Extract the identifier immediately before position `brace_pos` in `s`.
fn identifier_before(s: &str, brace_pos: usize) -> &str {
    let before = s[..brace_pos].trim_end();
    if before.is_empty() {
        return "";
    }
    let end = before.len();
    let start = before
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map_or(0, |i| i + 1);
    &before[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_KEYMAP: &str = r#"
/ {
    keymap {
        compatible = "zmk,keymap";
        base_layer {
            bindings = <
                &kp Q &kp W &trans &none &mo 1
            >;
        };
        lower_layer {
            bindings = <&kp A &lt 1 SPACE &mt LSHFT Z>;
        };
    };
};"#;

    #[test]
    fn parses_layer_count() {
        let km = parse(SIMPLE_KEYMAP).unwrap();
        assert_eq!(km.layers.len(), 2);
    }

    #[test]
    fn parses_layer_names() {
        let km = parse(SIMPLE_KEYMAP).unwrap();
        assert_eq!(km.layers[0].name, "base_layer");
        assert_eq!(km.layers[1].name, "lower_layer");
    }

    #[test]
    fn parses_basic_keys() {
        let km = parse(SIMPLE_KEYMAP).unwrap();
        let keys = &km.layers[0].keys;
        assert!(matches!(&keys[0], Key::Kp(k) if k == "Q"));
        assert!(matches!(&keys[2], Key::Trans));
        assert!(matches!(&keys[3], Key::None));
        assert!(matches!(&keys[4], Key::Mo(1)));
    }

    #[test]
    fn parses_behaviors() {
        let km = parse(SIMPLE_KEYMAP).unwrap();
        let keys = &km.layers[1].keys;
        assert!(matches!(&keys[1], Key::Lt(1, k) if k == "SPACE"));
        assert!(matches!(&keys[2], Key::Mt(m, k) if m == "LSHFT" && k == "Z"));
    }

    #[test]
    fn missing_keymap_block_is_error() {
        let err = parse("/ { };").unwrap_err();
        assert_eq!(err, ParseZmkError::NoKeymapBlock);
    }

    #[test]
    fn strips_line_comments() {
        let src = r#"
/ {
    keymap {
        compatible = "zmk,keymap";
        // this is a comment
        base_layer {
            bindings = <&kp A>; // trailing comment
        };
    };
};"#;
        let km = parse(src).unwrap();
        assert_eq!(km.layers.len(), 1);
        assert!(matches!(&km.layers[0].keys[0], Key::Kp(k) if k == "A"));
    }

    #[test]
    fn parses_conditional_layers() {
        let src = r#"
/ {
    conditional_layers {
        compatible = "zmk,conditional-layers";
        tri_layer {
            if-layers = <1 2>;
            then-layer = <3>;
        };
    };
    keymap {
        compatible = "zmk,keymap";
        base_layer { bindings = <&trans>; };
    };
};"#;
        let km = parse(src).unwrap();
        let tri = km.tri_layer.unwrap();
        assert_eq!(tri.lower, 1);
        assert_eq!(tri.upper, 2);
        assert_eq!(tri.tri, 3);
    }

    #[test]
    fn parses_macro_stubs() {
        let src = r#"
/ {
    macros {
        MY_MACRO: MY_MACRO {
            compatible = "zmk,behavior-macro";
            #binding-cells = <0>;
            bindings = <&none>;
        };
    };
    keymap {
        compatible = "zmk,keymap";
        base_layer { bindings = <&MY_MACRO>; };
    };
};"#;
        let km = parse(src).unwrap();
        assert_eq!(km.macros.len(), 1);
        assert_eq!(km.macros[0].name, "MY_MACRO");
        assert!(matches!(&km.layers[0].keys[0], Key::Macro(n) if n == "MY_MACRO"));
    }

    #[test]
    fn parses_one_shot_and_to() {
        let src = r#"
/ {
    keymap {
        compatible = "zmk,keymap";
        base_layer {
            bindings = <&sk LSHFT &sl 1 &to 0>;
        };
    };
};"#;
        let km = parse(src).unwrap();
        let keys = &km.layers[0].keys;
        assert!(matches!(&keys[0], Key::Sk(m) if m == "LSHFT"));
        assert!(matches!(&keys[1], Key::Sl(1)));
        assert!(matches!(&keys[2], Key::Df(0)));
    }

    #[test]
    fn parses_mouse_keys() {
        let src = r#"
/ {
    keymap {
        compatible = "zmk,keymap";
        base_layer {
            bindings = <&mmv MOVE_UP &mkp LCLK>;
        };
    };
};"#;
        let km = parse(src).unwrap();
        let keys = &km.layers[0].keys;
        assert!(matches!(&keys[0], Key::Mmv(d) if d == "MOVE_UP"));
        assert!(matches!(&keys[1], Key::Mkp(b) if b == "LCLK"));
    }

    #[test]
    fn bt_and_out_preserved_as_unknown() {
        let src = r#"
/ {
    keymap {
        compatible = "zmk,keymap";
        base_layer {
            bindings = <&bt BT_SEL 0 &out OUT_USB>;
        };
    };
};"#;
        let km = parse(src).unwrap();
        let keys = &km.layers[0].keys;
        assert!(matches!(&keys[0], Key::Unknown(s) if s.contains("bt") && s.contains("BT_SEL")));
        assert!(matches!(&keys[1], Key::Unknown(s) if s.contains("out") && s.contains("OUT_USB")));
    }

    #[test]
    fn parses_tap_dance_behavior() {
        let src = r#"
/ {
    behaviors {
        td0: tap_dance_0 {
            compatible = "zmk,behavior-tap-dance";
            #binding-cells = <0>;
            tapping-term-ms = <200>;
            bindings = <&kp LSHFT>, <&kp CAPS>;
        };
    };
    keymap {
        compatible = "zmk,keymap";
        base_layer {
            bindings = <&td0 &kp A>;
        };
    };
};"#;
        let km = parse(src).unwrap();
        assert_eq!(km.tap_dances.len(), 1);
        assert_eq!(km.tap_dances[0].name, "td0");
        assert_eq!(km.tap_dances[0].bindings.len(), 2);
        assert!(matches!(&km.tap_dances[0].bindings[0], Key::Kp(k) if k == "LSHFT"));
        assert!(matches!(&km.tap_dances[0].bindings[1], Key::Kp(k) if k == "CAPS"));
        let keys = &km.layers[0].keys;
        assert!(matches!(&keys[0], Key::TapDance(0)));
        assert!(matches!(&keys[1], Key::Kp(k) if k == "A"));
    }

    #[test]
    fn parses_multiple_tap_dances() {
        let src = r#"
/ {
    behaviors {
        td0: tap_dance_0 {
            compatible = "zmk,behavior-tap-dance";
            #binding-cells = <0>;
            tapping-term-ms = <200>;
            bindings = <&kp A>, <&kp B>;
        };
        td1: tap_dance_1 {
            compatible = "zmk,behavior-tap-dance";
            #binding-cells = <0>;
            tapping-term-ms = <200>;
            bindings = <&kp C>, <&kp D>, <&kp E>;
        };
    };
    keymap {
        compatible = "zmk,keymap";
        base_layer { bindings = <&td0 &td1>; };
    };
};"#;
        let km = parse(src).unwrap();
        assert_eq!(km.tap_dances.len(), 2);
        assert!(matches!(&km.layers[0].keys[0], Key::TapDance(0)));
        assert!(matches!(&km.layers[0].keys[1], Key::TapDance(1)));
        assert_eq!(km.tap_dances[1].bindings.len(), 3);
    }
}
