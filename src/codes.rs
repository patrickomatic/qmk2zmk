//! Keycode naming tables used by both conversion directions.
//!
//! QMK and ZMK generally describe the same HID usages, but they often use
//! different symbolic names. The IR stores key names in ZMK spelling, so QMK
//! parsers call the `qmk_*_to_zmk` functions before constructing [`crate::ir::Key`]
//! values. QMK renderers call the reverse functions when writing QMK JSON or C.
//!
//! Examples of the most common spelling differences:
//!
//! - QMK number keys use `KC_1`; ZMK uses `N1`.
//! - QMK physical bracket keys use `KC_LBRC`/`KC_RBRC`; ZMK uses `LBKT`/`RBKT`.
//! - QMK shifted brace aliases use `KC_LCBR`/`KC_RCBR`; ZMK uses `LBRC`/`RBRC`.
//! - QMK `KC_ENTER`, `KC_SCLN`, and `KC_QUOTE` become ZMK `RET`, `SEMI`, and
//!   `SQT`.
//! - QMK modifier wrappers like `LGUI(KC_C)` become ZMK modifier expressions
//!   like `LG(C)`.

/// Map a QMK keycode to a ZMK key name.
///
/// The input may include the `KC_` prefix or omit it. The output is a plain ZMK
/// key name suitable for [`crate::ir::Key::Kp`] and ZMK `&kp`, not a complete ZMK
/// binding.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn qmk_key_to_zmk(qmk: &str) -> Option<&'static str> {
    let key = qmk.strip_prefix("KC_").unwrap_or(qmk);
    Some(match key {
        // Letters
        "A" => "A",
        "B" => "B",
        "C" => "C",
        "D" => "D",
        "E" => "E",
        "F" => "F",
        "G" => "G",
        "H" => "H",
        "I" => "I",
        "J" => "J",
        "K" => "K",
        "L" => "L",
        "M" => "M",
        "N" => "N",
        "O" => "O",
        "P" => "P",
        "Q" => "Q",
        "R" => "R",
        "S" => "S",
        "T" => "T",
        "U" => "U",
        "V" => "V",
        "W" => "W",
        "X" => "X",
        "Y" => "Y",
        "Z" => "Z",
        // Numbers (ZMK uses N prefix)
        "0" => "N0",
        "1" => "N1",
        "2" => "N2",
        "3" => "N3",
        "4" => "N4",
        "5" => "N5",
        "6" => "N6",
        "7" => "N7",
        "8" => "N8",
        "9" => "N9",
        // Function keys
        "F1" => "F1",
        "F2" => "F2",
        "F3" => "F3",
        "F4" => "F4",
        "F5" => "F5",
        "F6" => "F6",
        "F7" => "F7",
        "F8" => "F8",
        "F9" => "F9",
        "F10" => "F10",
        "F11" => "F11",
        "F12" => "F12",
        "F13" => "F13",
        "F14" => "F14",
        "F15" => "F15",
        "F16" => "F16",
        "F17" => "F17",
        "F18" => "F18",
        "F19" => "F19",
        "F20" => "F20",
        "F21" => "F21",
        "F22" => "F22",
        "F23" => "F23",
        "F24" => "F24",
        // Common keys
        "TAB" => "TAB",
        "ENTER" | "ENT" => "RET",
        "ESCAPE" | "ESC" => "ESC",
        "BSPC" => "BSPC",
        "DEL" | "DELETE" => "DEL",
        "INS" | "INSERT" => "INS",
        "SPACE" | "SPC" => "SPACE",
        "CAPS" | "CAPS_LOCK" | "CAPSLOCK" => "CAPS",
        // Punctuation
        "MINUS" => "MINUS",
        "EQUAL" => "EQUAL",
        "LBRC" => "LBKT",
        "RBRC" => "RBKT",
        "BSLS" => "BSLH",
        "SCLN" => "SEMI",
        "QUOTE" | "QUOT" => "SQT",
        "GRAVE" | "GRV" => "GRAVE",
        "COMMA" | "COMM" => "COMMA",
        "DOT" => "DOT",
        "SLASH" | "SLSH" => "FSLH",
        // Shifted symbols
        "EXLM" => "EXCL",
        "AT" => "AT",
        "HASH" => "HASH",
        "DLR" => "DLLR",
        "PERC" => "PRCNT",
        "CIRC" => "CARET",
        "AMPR" => "AMPS",
        "ASTR" => "STAR",
        "LPRN" => "LPAR",
        "RPRN" => "RPAR",
        "UNDS" => "UNDER",
        "PLUS" => "PLUS",
        "LCBR" => "LBRC",
        "RCBR" => "RBRC",
        "PIPE" => "PIPE",
        "TILD" => "TILDE",
        "LT" => "LT",
        "GT" => "GT",
        "DQUO" => "DQT",
        "COLN" => "COLON",
        "QUES" => "QMARK",
        // Navigation
        "LEFT" => "LEFT",
        "RIGHT" => "RIGHT",
        "UP" => "UP",
        "DOWN" => "DOWN",
        "PGUP" | "PAGE_UP" => "PG_UP",
        "PGDN" | "PAGE_DOWN" => "PG_DN",
        "HOME" => "HOME",
        "END" => "END",
        // Modifiers
        "LCTL" | "LCTRL" => "LCTRL",
        "RCTL" | "RCTRL" => "RCTRL",
        "LSFT" | "LSHIFT" => "LSHFT",
        "RSFT" | "RSHIFT" => "RSHFT",
        "LALT" => "LALT",
        "RALT" => "RALT",
        "LGUI" => "LGUI",
        "RGUI" => "RGUI",
        // Media
        "AUDIO_VOL_UP" | "VOLU" => "C_VOL_UP",
        "AUDIO_VOL_DOWN" | "VOLD" => "C_VOL_DN",
        "AUDIO_MUTE" | "MUTE" => "C_MUTE",
        "BRIGHTNESS_UP" | "BRIU" => "C_BRI_UP",
        "BRIGHTNESS_DOWN" | "BRID" => "C_BRI_DN",
        "MEDIA_PLAY_PAUSE" | "MPLY" => "C_PP",
        "MEDIA_NEXT_TRACK" | "MNXT" => "C_NEXT",
        "MEDIA_PREV_TRACK" | "MPRV" => "C_PREV",
        // Keypad
        "KP_0" => "KP_N0",
        "KP_1" => "KP_N1",
        "KP_2" => "KP_N2",
        "KP_3" => "KP_N3",
        "KP_4" => "KP_N4",
        "KP_5" => "KP_N5",
        "KP_6" => "KP_N6",
        "KP_7" => "KP_N7",
        "KP_8" => "KP_N8",
        "KP_9" => "KP_N9",
        "KP_SLASH" => "KP_SLASH",
        "KP_ASTERISK" => "KP_MULTIPLY",
        "KP_MINUS" => "KP_MINUS",
        "KP_PLUS" => "KP_PLUS",
        "KP_ENTER" => "KP_ENTER",
        "KP_DOT" => "KP_DOT",
        // Application control
        "AGAIN" | "AGIN" => "K_REDO",
        "UNDO" => "K_UNDO",
        "CUT" => "K_CUT",
        "COPY" => "K_COPY",
        "PASTE" | "PSTE" => "K_PASTE",
        // Non-US keys
        "NUBS" | "NONUS_BACKSLASH" => "NON_US_BSLH",
        "NUHS" | "NONUS_HASH" => "NON_US_HASH",
        // Misc
        "PSCR" | "PRINT_SCREEN" => "PSCRN",
        "SCRL" | "SCROLLLOCK" => "SLCK",
        "PAUS" | "PAUSE" => "PAUSE_BREAK",
        "APP" => "K_APP",
        unsupported => {
            let _ = unsupported.len();
            return None;
        }
    })
}

/// Map a QMK `MOD_*` constant or bare modifier name to a ZMK modifier name.
///
/// Used for hold modifiers in mod-tap and sticky-key bindings. Unknown modifiers
/// are preserved as `UNKNOWN_MOD` so callers can surface an explicit unsupported
/// binding instead of silently choosing a modifier.
#[must_use]
pub fn qmk_mod_to_zmk(qmk_mod: &str) -> &'static str {
    match qmk_mod.trim() {
        "MOD_LALT" | "LALT" => "LALT",
        "MOD_RALT" | "RALT" => "RALT",
        "MOD_LCTL" | "LCTL" | "LCTRL" => "LCTRL",
        "MOD_RCTL" | "RCTL" | "RCTRL" => "RCTRL",
        "MOD_LSFT" | "LSFT" | "LSHIFT" => "LSHFT",
        "MOD_RSFT" | "RSFT" | "RSHIFT" => "RSHFT",
        "MOD_LGUI" | "LGUI" => "LGUI",
        "MOD_RGUI" | "RGUI" => "RGUI",
        unsupported => {
            let _ = unsupported.len();
            "UNKNOWN_MOD"
        }
    }
}

/// Map a QMK modifier-wrapping function name to a ZMK key-expression prefix.
///
/// For example, QMK `LGUI(KC_C)` is represented in the IR as `Kp("LG(C)")`, so
/// this maps `LGUI` to `LG`.
#[must_use]
pub fn qmk_mod_fn_to_zmk(name: &str) -> Option<&'static str> {
    Some(match name {
        "LGUI" => "LG",
        "RGUI" => "RG",
        "LSFT" | "LSHIFT" => "LS",
        "RSFT" | "RSHIFT" => "RS",
        "LCTL" | "LCTRL" => "LC",
        "RCTL" | "RCTRL" => "RC",
        "LALT" => "LA",
        "RALT" => "RA",
        unsupported => {
            let _ = unsupported.len();
            return None;
        }
    })
}

/// Map a QMK RGB function name to the corresponding ZMK `rgb_ug` action string.
#[must_use]
pub fn qmk_rgb_to_zmk(name: &str) -> Option<&'static str> {
    Some(match name {
        "RGB_TOG" => "RGB_TOG",
        "RGB_HUI" => "RGB_HUI",
        "RGB_HUD" => "RGB_HUD",
        "RGB_SAI" => "RGB_SAI",
        "RGB_SAD" => "RGB_SAD",
        "RGB_VAI" => "RGB_VAI",
        "RGB_VAD" => "RGB_VAD",
        "RGB_MODE_FORWARD" | "RGB_MOD" => "RGB_EFF",
        "RGB_MODE_REVERSE" | "RGB_RMOD" => "RGB_EFR",
        "RGB_SPI" => "RGB_SPI",
        "RGB_SPD" => "RGB_SPD",
        unsupported => {
            let _ = unsupported.len();
            return None;
        }
    })
}

/// Map a ZMK key name back to a QMK keycode string.
///
/// The input is a plain ZMK key name like `N1` or `RET`. The output includes
/// QMK's `KC_` prefix.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn zmk_key_to_qmk(zmk: &str) -> Option<&'static str> {
    Some(match zmk {
        // Letters
        "A" => "KC_A",
        "B" => "KC_B",
        "C" => "KC_C",
        "D" => "KC_D",
        "E" => "KC_E",
        "F" => "KC_F",
        "G" => "KC_G",
        "H" => "KC_H",
        "I" => "KC_I",
        "J" => "KC_J",
        "K" => "KC_K",
        "L" => "KC_L",
        "M" => "KC_M",
        "N" => "KC_N",
        "O" => "KC_O",
        "P" => "KC_P",
        "Q" => "KC_Q",
        "R" => "KC_R",
        "S" => "KC_S",
        "T" => "KC_T",
        "U" => "KC_U",
        "V" => "KC_V",
        "W" => "KC_W",
        "X" => "KC_X",
        "Y" => "KC_Y",
        "Z" => "KC_Z",
        // Numbers
        "N0" => "KC_0",
        "N1" => "KC_1",
        "N2" => "KC_2",
        "N3" => "KC_3",
        "N4" => "KC_4",
        "N5" => "KC_5",
        "N6" => "KC_6",
        "N7" => "KC_7",
        "N8" => "KC_8",
        "N9" => "KC_9",
        // Function keys
        "F1" => "KC_F1",
        "F2" => "KC_F2",
        "F3" => "KC_F3",
        "F4" => "KC_F4",
        "F5" => "KC_F5",
        "F6" => "KC_F6",
        "F7" => "KC_F7",
        "F8" => "KC_F8",
        "F9" => "KC_F9",
        "F10" => "KC_F10",
        "F11" => "KC_F11",
        "F12" => "KC_F12",
        "F13" => "KC_F13",
        "F14" => "KC_F14",
        "F15" => "KC_F15",
        "F16" => "KC_F16",
        "F17" => "KC_F17",
        "F18" => "KC_F18",
        "F19" => "KC_F19",
        "F20" => "KC_F20",
        "F21" => "KC_F21",
        "F22" => "KC_F22",
        "F23" => "KC_F23",
        "F24" => "KC_F24",
        // Common
        "TAB" => "KC_TAB",
        "RET" => "KC_ENTER",
        "ESC" => "KC_ESCAPE",
        "BSPC" => "KC_BSPC",
        "DEL" => "KC_DEL",
        "INS" => "KC_INS",
        "SPACE" => "KC_SPACE",
        "CAPS" => "KC_CAPS",
        // Punctuation
        "MINUS" => "KC_MINUS",
        "EQUAL" => "KC_EQUAL",
        "LBKT" => "KC_LBRC",
        "RBKT" => "KC_RBRC",
        "BSLH" => "KC_BSLS",
        "SEMI" => "KC_SCLN",
        "SQT" => "KC_QUOTE",
        "GRAVE" => "KC_GRAVE",
        "COMMA" => "KC_COMMA",
        "DOT" => "KC_DOT",
        "FSLH" => "KC_SLASH",
        // Shifted symbols
        "EXCL" => "KC_EXLM",
        "AT" => "KC_AT",
        "HASH" => "KC_HASH",
        "DLLR" => "KC_DLR",
        "PRCNT" => "KC_PERC",
        "CARET" => "KC_CIRC",
        "AMPS" => "KC_AMPR",
        "STAR" => "KC_ASTR",
        "LPAR" => "KC_LPRN",
        "RPAR" => "KC_RPRN",
        "UNDER" => "KC_UNDS",
        "PLUS" => "KC_PLUS",
        "LBRC" => "KC_LCBR",
        "RBRC" => "KC_RCBR",
        "PIPE" => "KC_PIPE",
        "TILDE" => "KC_TILD",
        "LT" => "KC_LT",
        "GT" => "KC_GT",
        "DQT" => "KC_DQUO",
        "COLON" => "KC_COLN",
        "QMARK" => "KC_QUES",
        // Navigation
        "LEFT" => "KC_LEFT",
        "RIGHT" => "KC_RIGHT",
        "UP" => "KC_UP",
        "DOWN" => "KC_DOWN",
        "PG_UP" => "KC_PGUP",
        "PG_DN" => "KC_PGDN",
        "HOME" => "KC_HOME",
        "END" => "KC_END",
        // Modifiers
        "LCTRL" => "KC_LCTL",
        "RCTRL" => "KC_RCTL",
        "LSHFT" => "KC_LSFT",
        "RSHFT" => "KC_RSFT",
        "LALT" => "KC_LALT",
        "RALT" => "KC_RALT",
        "LGUI" => "KC_LGUI",
        "RGUI" => "KC_RGUI",
        // Media
        "C_VOL_UP" => "KC_VOLU",
        "C_VOL_DN" => "KC_VOLD",
        "C_MUTE" => "KC_MUTE",
        "C_BRI_UP" => "KC_BRIU",
        "C_BRI_DN" => "KC_BRID",
        "C_PP" => "KC_MPLY",
        "C_NEXT" => "KC_MNXT",
        "C_PREV" => "KC_MPRV",
        // Keypad
        "KP_N0" => "KC_KP_0",
        "KP_N1" => "KC_KP_1",
        "KP_N2" => "KC_KP_2",
        "KP_N3" => "KC_KP_3",
        "KP_N4" => "KC_KP_4",
        "KP_N5" => "KC_KP_5",
        "KP_N6" => "KC_KP_6",
        "KP_N7" => "KC_KP_7",
        "KP_N8" => "KC_KP_8",
        "KP_N9" => "KC_KP_9",
        "KP_SLASH" => "KC_KP_SLASH",
        "KP_MULTIPLY" => "KC_KP_ASTERISK",
        "KP_MINUS" => "KC_KP_MINUS",
        "KP_PLUS" => "KC_KP_PLUS",
        "KP_ENTER" => "KC_KP_ENTER",
        "KP_DOT" => "KC_KP_DOT",
        // Application control
        "K_REDO" => "KC_AGAIN",
        "K_UNDO" => "KC_UNDO",
        "K_CUT" => "KC_CUT",
        "K_COPY" => "KC_COPY",
        "K_PASTE" => "KC_PASTE",
        // Non-US keys
        "NON_US_BSLH" => "KC_NUBS",
        "NON_US_HASH" => "KC_NUHS",
        // Misc
        "PSCRN" => "KC_PSCR",
        "SLCK" => "KC_SCRL",
        "PAUSE_BREAK" => "KC_PAUS",
        "K_APP" => "KC_APP",
        unsupported => {
            let _ = unsupported.len();
            return None;
        }
    })
}

/// Map a ZMK modifier name to a QMK `MOD_*` constant.
///
/// Unknown modifiers fall back to `MOD_LCTL`, matching the converter's current
/// best-effort behavior for QMK constructs that require a valid modifier token.
#[must_use]
pub fn zmk_mod_to_qmk(zmk_mod: &str) -> &'static str {
    match zmk_mod.trim() {
        "LALT" => "MOD_LALT",
        "RALT" => "MOD_RALT",
        "RCTRL" => "MOD_RCTL",
        "LSHFT" => "MOD_LSFT",
        "RSHFT" => "MOD_RSFT",
        "LGUI" => "MOD_LGUI",
        "RGUI" => "MOD_RGUI",
        unsupported => {
            let _ = unsupported.len();
            "MOD_LCTL"
        }
    }
}

/// Convert a ZMK key expression (simple or modifier-wrapped) to a QMK keycode string.
///
/// Handles nested modifier prefixes like `LG(LS(LBKT))` → `LGUI(LSFT(KC_LBRC))`.
/// Falls back to the raw ZMK name for unknown keys.
#[must_use]
pub fn zmk_key_expr_to_qmk(zmk: &str) -> String {
    if let Some(paren) = zmk.find('(') {
        let prefix = &zmk[..paren];
        let converted = zmk_mod_prefix_to_qmk_fn(prefix).and_then(|qmk_fn| {
            extract_paren_inner(zmk, paren)
                .map(|inner| format!("{qmk_fn}({})", zmk_key_expr_to_qmk(inner)))
        });
        if let Some(result) = converted {
            return result;
        }
    }
    zmk_key_to_qmk(zmk).unwrap_or(zmk).to_string()
}

fn zmk_mod_prefix_to_qmk_fn(prefix: &str) -> Option<&'static str> {
    Some(match prefix {
        "LG" => "LGUI",
        "RG" => "RGUI",
        "LS" => "LSFT",
        "RS" => "RSFT",
        "LC" => "LCTL",
        "RC" => "RCTL",
        "LA" => "LALT",
        "RA" => "RALT",
        unsupported => {
            let _ = unsupported.len();
            return None;
        }
    })
}

/// Return the text inside the outer parenthesized expression starting at `open`.
///
/// Used for nested modifier expressions such as `LG(LS(LBKT))`, where a simple
/// search for the next `)` would stop too early.
fn extract_paren_inner(s: &str, open: usize) -> Option<&str> {
    let mut depth = 0usize;
    for (i, c) in s[open..].char_indices() {
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                return Some(&s[open + 1..open + i]);
            }
        }
    }
    None
}

/// Map a ZMK `rgb_ug` action string back to a QMK RGB keycode.
#[must_use]
pub fn zmk_rgb_to_qmk(zmk: &str) -> Option<&'static str> {
    Some(match zmk {
        "RGB_TOG" => "RGB_TOG",
        "RGB_HUI" => "RGB_HUI",
        "RGB_HUD" => "RGB_HUD",
        "RGB_SAI" => "RGB_SAI",
        "RGB_SAD" => "RGB_SAD",
        "RGB_VAI" => "RGB_VAI",
        "RGB_VAD" => "RGB_VAD",
        "RGB_EFF" => "RGB_MODE_FORWARD",
        "RGB_EFR" => "RGB_MODE_REVERSE",
        "RGB_SPI" => "RGB_SPI",
        "RGB_SPD" => "RGB_SPD",
        unsupported => {
            let _ = unsupported.len();
            return None;
        }
    })
}

/// Known keyboards and their default output column counts.
///
/// These are formatting hints for generated keymap source, not semantic layout
/// definitions. The CLI exposes them through `--list-keyboards` and accepts
/// substring matches through [`keyboard_cols`].
pub const KNOWN_KEYBOARDS: &[(&str, usize)] = &[
    ("planck", 12),
    ("preonic", 12),
    ("contra", 12),
    ("levinson", 12),
    ("nyquist", 12),
    ("corne", 6),
    ("crkbd", 6),
    ("kyria", 7),
    ("lily58", 7),
];

/// Return the column count for a keyboard matched by substring, case-insensitive.
///
/// This accepts values such as `planck/ez/glow` or `LAYOUT_crkbd_base` because
/// both contain one of the names in [`KNOWN_KEYBOARDS`].
#[must_use]
pub fn keyboard_cols(name: &str) -> Option<usize> {
    let lower = name.to_lowercase();
    KNOWN_KEYBOARDS
        .iter()
        .find(|(k, _)| lower.contains(*k))
        .map(|(_, cols)| *cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_pass_through() {
        assert_eq!(qmk_key_to_zmk("KC_A"), Some("A"));
        assert_eq!(qmk_key_to_zmk("KC_Z"), Some("Z"));
    }

    #[test]
    fn numbers_get_n_prefix() {
        assert_eq!(qmk_key_to_zmk("KC_0"), Some("N0"));
        assert_eq!(qmk_key_to_zmk("KC_9"), Some("N9"));
    }

    #[test]
    fn punctuation_remapped() {
        assert_eq!(qmk_key_to_zmk("KC_SCLN"), Some("SEMI"));
        assert_eq!(qmk_key_to_zmk("KC_QUOTE"), Some("SQT"));
        assert_eq!(qmk_key_to_zmk("KC_LBRC"), Some("LBKT"));
        assert_eq!(qmk_key_to_zmk("KC_RBRC"), Some("RBKT"));
        assert_eq!(qmk_key_to_zmk("KC_BSLS"), Some("BSLH"));
        assert_eq!(qmk_key_to_zmk("KC_GRAVE"), Some("GRAVE"));
    }

    #[test]
    fn shifted_symbols_remapped() {
        assert_eq!(qmk_key_to_zmk("KC_EXLM"), Some("EXCL"));
        assert_eq!(qmk_key_to_zmk("KC_LCBR"), Some("LBRC"));
        assert_eq!(qmk_key_to_zmk("KC_RCBR"), Some("RBRC"));
        assert_eq!(qmk_key_to_zmk("KC_PIPE"), Some("PIPE"));
        assert_eq!(qmk_key_to_zmk("KC_TILD"), Some("TILDE"));
        assert_eq!(qmk_key_to_zmk("KC_AMPR"), Some("AMPS"));
        assert_eq!(qmk_key_to_zmk("KC_ASTR"), Some("STAR"));
        assert_eq!(qmk_key_to_zmk("KC_LPRN"), Some("LPAR"));
        assert_eq!(qmk_key_to_zmk("KC_UNDS"), Some("UNDER"));
    }

    #[test]
    fn navigation_remapped() {
        assert_eq!(qmk_key_to_zmk("KC_PGUP"), Some("PG_UP"));
        assert_eq!(qmk_key_to_zmk("KC_PAGE_UP"), Some("PG_UP"));
        assert_eq!(qmk_key_to_zmk("KC_PGDN"), Some("PG_DN"));
        assert_eq!(qmk_key_to_zmk("KC_PAGE_DOWN"), Some("PG_DN"));
    }

    #[test]
    fn media_remapped() {
        assert_eq!(qmk_key_to_zmk("KC_AUDIO_VOL_UP"), Some("C_VOL_UP"));
        assert_eq!(qmk_key_to_zmk("KC_AUDIO_VOL_DOWN"), Some("C_VOL_DN"));
        assert_eq!(qmk_key_to_zmk("KC_BRIGHTNESS_UP"), Some("C_BRI_UP"));
        assert_eq!(qmk_key_to_zmk("KC_BRIGHTNESS_DOWN"), Some("C_BRI_DN"));
    }

    #[test]
    fn common_keys() {
        assert_eq!(qmk_key_to_zmk("KC_TAB"), Some("TAB"));
        assert_eq!(qmk_key_to_zmk("KC_ENTER"), Some("RET"));
        assert_eq!(qmk_key_to_zmk("KC_ESCAPE"), Some("ESC"));
        assert_eq!(qmk_key_to_zmk("KC_BSPC"), Some("BSPC"));
        assert_eq!(qmk_key_to_zmk("KC_SPACE"), Some("SPACE"));
    }

    #[test]
    fn unknown_returns_none() {
        assert_eq!(qmk_key_to_zmk("KC_DOESNOTEXIST"), None);
        assert_eq!(qmk_key_to_zmk("TOTALLY_MADE_UP"), None);
    }

    #[test]
    fn mod_constants_remapped() {
        assert_eq!(qmk_mod_to_zmk("MOD_LALT"), "LALT");
        assert_eq!(qmk_mod_to_zmk("MOD_LGUI"), "LGUI");
        assert_eq!(qmk_mod_to_zmk("MOD_LCTL"), "LCTRL");
        assert_eq!(qmk_mod_to_zmk("MOD_LSFT"), "LSHFT");
        assert_eq!(qmk_mod_to_zmk("MOD_RALT"), "RALT");
        assert_eq!(qmk_mod_to_zmk("MOD_RGUI"), "RGUI");
    }

    #[test]
    fn mod_fn_prefixes() {
        assert_eq!(qmk_mod_fn_to_zmk("LGUI"), Some("LG"));
        assert_eq!(qmk_mod_fn_to_zmk("LSFT"), Some("LS"));
        assert_eq!(qmk_mod_fn_to_zmk("LCTL"), Some("LC"));
        assert_eq!(qmk_mod_fn_to_zmk("LALT"), Some("LA"));
        assert_eq!(qmk_mod_fn_to_zmk("RGUI"), Some("RG"));
        assert_eq!(qmk_mod_fn_to_zmk("RSFT"), Some("RS"));
        assert_eq!(qmk_mod_fn_to_zmk("KC_A"), None);
    }

    #[test]
    fn rgb_actions() {
        assert_eq!(qmk_rgb_to_zmk("RGB_TOG"), Some("RGB_TOG"));
        assert_eq!(qmk_rgb_to_zmk("RGB_HUI"), Some("RGB_HUI"));
        assert_eq!(qmk_rgb_to_zmk("RGB_MODE_FORWARD"), Some("RGB_EFF"));
        assert_eq!(qmk_rgb_to_zmk("RGB_MOD"), Some("RGB_EFF"));
        assert_eq!(qmk_rgb_to_zmk("RGB_RMOD"), Some("RGB_EFR"));
        assert_eq!(qmk_rgb_to_zmk("NOT_RGB"), None);
    }

    #[test]
    fn reverse_letters() {
        assert_eq!(zmk_key_to_qmk("A"), Some("KC_A"));
        assert_eq!(zmk_key_to_qmk("Z"), Some("KC_Z"));
    }

    #[test]
    fn reverse_numbers_strip_n_prefix() {
        assert_eq!(zmk_key_to_qmk("N0"), Some("KC_0"));
        assert_eq!(zmk_key_to_qmk("N9"), Some("KC_9"));
    }

    #[test]
    fn reverse_punctuation() {
        assert_eq!(zmk_key_to_qmk("SEMI"), Some("KC_SCLN"));
        assert_eq!(zmk_key_to_qmk("SQT"), Some("KC_QUOTE"));
        assert_eq!(zmk_key_to_qmk("LBKT"), Some("KC_LBRC"));
        assert_eq!(zmk_key_to_qmk("RBKT"), Some("KC_RBRC"));
        assert_eq!(zmk_key_to_qmk("BSLH"), Some("KC_BSLS"));
        assert_eq!(zmk_key_to_qmk("RET"), Some("KC_ENTER"));
        assert_eq!(zmk_key_to_qmk("FSLH"), Some("KC_SLASH"));
    }

    #[test]
    fn reverse_media() {
        assert_eq!(zmk_key_to_qmk("C_VOL_UP"), Some("KC_VOLU"));
        assert_eq!(zmk_key_to_qmk("C_VOL_DN"), Some("KC_VOLD"));
        assert_eq!(zmk_key_to_qmk("C_MUTE"), Some("KC_MUTE"));
    }

    #[test]
    fn reverse_mod_constants() {
        assert_eq!(zmk_mod_to_qmk("LALT"), "MOD_LALT");
        assert_eq!(zmk_mod_to_qmk("LCTRL"), "MOD_LCTL");
        assert_eq!(zmk_mod_to_qmk("LSHFT"), "MOD_LSFT");
        assert_eq!(zmk_mod_to_qmk("LGUI"), "MOD_LGUI");
    }

    #[test]
    fn reverse_rgb() {
        assert_eq!(zmk_rgb_to_qmk("RGB_TOG"), Some("RGB_TOG"));
        assert_eq!(zmk_rgb_to_qmk("RGB_EFF"), Some("RGB_MODE_FORWARD"));
        assert_eq!(zmk_rgb_to_qmk("RGB_EFR"), Some("RGB_MODE_REVERSE"));
        assert_eq!(zmk_rgb_to_qmk("UNKNOWN"), None);
    }

    #[test]
    fn reverse_unknown_returns_none() {
        assert_eq!(zmk_key_to_qmk("NOT_A_KEY"), None);
        assert_eq!(zmk_key_to_qmk("LC(C)"), None);
    }

    #[test]
    fn application_keys() {
        assert_eq!(qmk_key_to_zmk("KC_UNDO"), Some("K_UNDO"));
        assert_eq!(qmk_key_to_zmk("KC_AGAIN"), Some("K_REDO"));
        assert_eq!(qmk_key_to_zmk("KC_CUT"), Some("K_CUT"));
        assert_eq!(qmk_key_to_zmk("KC_COPY"), Some("K_COPY"));
        assert_eq!(qmk_key_to_zmk("KC_PASTE"), Some("K_PASTE"));
    }

    #[test]
    fn non_us_keys() {
        assert_eq!(qmk_key_to_zmk("KC_NUBS"), Some("NON_US_BSLH"));
        assert_eq!(qmk_key_to_zmk("KC_NUHS"), Some("NON_US_HASH"));
        assert_eq!(zmk_key_to_qmk("NON_US_BSLH"), Some("KC_NUBS"));
        assert_eq!(zmk_key_to_qmk("NON_US_HASH"), Some("KC_NUHS"));
    }

    #[test]
    fn rgb_speed_keys() {
        assert_eq!(qmk_rgb_to_zmk("RGB_SPI"), Some("RGB_SPI"));
        assert_eq!(qmk_rgb_to_zmk("RGB_SPD"), Some("RGB_SPD"));
        assert_eq!(zmk_rgb_to_qmk("RGB_SPI"), Some("RGB_SPI"));
        assert_eq!(zmk_rgb_to_qmk("RGB_SPD"), Some("RGB_SPD"));
    }

    #[test]
    fn key_expr_simple() {
        assert_eq!(zmk_key_expr_to_qmk("Q"), "KC_Q");
        assert_eq!(zmk_key_expr_to_qmk("SPACE"), "KC_SPACE");
        assert_eq!(zmk_key_expr_to_qmk("LBKT"), "KC_LBRC");
    }

    #[test]
    fn key_expr_single_modifier() {
        assert_eq!(zmk_key_expr_to_qmk("LC(C)"), "LCTL(KC_C)");
        assert_eq!(zmk_key_expr_to_qmk("LG(SPACE)"), "LGUI(KC_SPACE)");
        assert_eq!(zmk_key_expr_to_qmk("LS(TAB)"), "LSFT(KC_TAB)");
    }

    #[test]
    fn key_expr_nested_modifiers() {
        assert_eq!(zmk_key_expr_to_qmk("LG(LS(LBKT))"), "LGUI(LSFT(KC_LBRC))");
        assert_eq!(zmk_key_expr_to_qmk("RG(RS(RBKT))"), "RGUI(RSFT(KC_RBRC))");
    }

    #[test]
    fn key_expr_unknown_falls_back_to_raw() {
        assert_eq!(zmk_key_expr_to_qmk("WEIRD"), "WEIRD");
    }
}
