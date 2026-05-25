/// Map a QMK keycode (with or without KC_ prefix) to a ZMK key name.
#[must_use] pub fn qmk_key_to_zmk(qmk: &str) -> Option<&'static str> {
    let key = qmk.strip_prefix("KC_").unwrap_or(qmk);
    Some(match key {
        // Letters
        "A" => "A", "B" => "B", "C" => "C", "D" => "D", "E" => "E",
        "F" => "F", "G" => "G", "H" => "H", "I" => "I", "J" => "J",
        "K" => "K", "L" => "L", "M" => "M", "N" => "N", "O" => "O",
        "P" => "P", "Q" => "Q", "R" => "R", "S" => "S", "T" => "T",
        "U" => "U", "V" => "V", "W" => "W", "X" => "X", "Y" => "Y",
        "Z" => "Z",
        // Numbers (ZMK uses N prefix)
        "0" => "N0", "1" => "N1", "2" => "N2", "3" => "N3", "4" => "N4",
        "5" => "N5", "6" => "N6", "7" => "N7", "8" => "N8", "9" => "N9",
        // Function keys
        "F1" => "F1",   "F2" => "F2",   "F3" => "F3",   "F4" => "F4",
        "F5" => "F5",   "F6" => "F6",   "F7" => "F7",   "F8" => "F8",
        "F9" => "F9",   "F10" => "F10", "F11" => "F11", "F12" => "F12",
        "F13" => "F13", "F14" => "F14", "F15" => "F15", "F16" => "F16",
        "F17" => "F17", "F18" => "F18", "F19" => "F19", "F20" => "F20",
        "F21" => "F21", "F22" => "F22", "F23" => "F23", "F24" => "F24",
        // Common keys
        "TAB"                       => "TAB",
        "ENTER" | "ENT"             => "RET",
        "ESCAPE" | "ESC"            => "ESC",
        "BSPC"                      => "BSPC",
        "DEL" | "DELETE"            => "DEL",
        "INS" | "INSERT"            => "INS",
        "SPACE" | "SPC"             => "SPACE",
        "CAPS" | "CAPS_LOCK" | "CAPSLOCK" => "CAPS",
        // Punctuation
        "MINUS"                     => "MINUS",
        "EQUAL"                     => "EQUAL",
        "LBRC"                      => "LBKT",
        "RBRC"                      => "RBKT",
        "BSLS"                      => "BSLH",
        "SCLN"                      => "SEMI",
        "QUOTE" | "QUOT"            => "SQT",
        "GRAVE" | "GRV"             => "GRAVE",
        "COMMA" | "COMM"            => "COMMA",
        "DOT"                       => "DOT",
        "SLASH" | "SLSH"            => "FSLH",
        // Shifted symbols
        "EXLM"                      => "EXCL",
        "AT"                        => "AT",
        "HASH"                      => "HASH",
        "DLR"                       => "DLLR",
        "PERC"                      => "PRCNT",
        "CIRC"                      => "CARET",
        "AMPR"                      => "AMPS",
        "ASTR"                      => "STAR",
        "LPRN"                      => "LPAR",
        "RPRN"                      => "RPAR",
        "UNDS"                      => "UNDER",
        "PLUS"                      => "PLUS",
        "LCBR"                      => "LBRC",
        "RCBR"                      => "RBRC",
        "PIPE"                      => "PIPE",
        "TILD"                      => "TILDE",
        "LT"                        => "LT",
        "GT"                        => "GT",
        "DQUO"                      => "DQT",
        "COLN"                      => "COLON",
        "QUES"                      => "QMARK",
        // Navigation
        "LEFT"                      => "LEFT",
        "RIGHT"                     => "RIGHT",
        "UP"                        => "UP",
        "DOWN"                      => "DOWN",
        "PGUP" | "PAGE_UP"          => "PG_UP",
        "PGDN" | "PAGE_DOWN"        => "PG_DN",
        "HOME"                      => "HOME",
        "END"                       => "END",
        // Modifiers
        "LCTL" | "LCTRL"            => "LCTRL",
        "RCTL" | "RCTRL"            => "RCTRL",
        "LSFT" | "LSHIFT"           => "LSHFT",
        "RSFT" | "RSHIFT"           => "RSHFT",
        "LALT"                      => "LALT",
        "RALT"                      => "RALT",
        "LGUI"                      => "LGUI",
        "RGUI"                      => "RGUI",
        // Media
        "AUDIO_VOL_UP" | "VOLU"     => "C_VOL_UP",
        "AUDIO_VOL_DOWN" | "VOLD"   => "C_VOL_DN",
        "AUDIO_MUTE" | "MUTE"       => "C_MUTE",
        "BRIGHTNESS_UP" | "BRIU"    => "C_BRI_UP",
        "BRIGHTNESS_DOWN" | "BRID"  => "C_BRI_DN",
        "MEDIA_PLAY_PAUSE" | "MPLY" => "C_PP",
        "MEDIA_NEXT_TRACK" | "MNXT" => "C_NEXT",
        "MEDIA_PREV_TRACK" | "MPRV" => "C_PREV",
        // Keypad
        "KP_0" => "KP_N0", "KP_1" => "KP_N1", "KP_2" => "KP_N2",
        "KP_3" => "KP_N3", "KP_4" => "KP_N4", "KP_5" => "KP_N5",
        "KP_6" => "KP_N6", "KP_7" => "KP_N7", "KP_8" => "KP_N8",
        "KP_9" => "KP_N9",
        "KP_SLASH"    => "KP_SLASH",
        "KP_ASTERISK" => "KP_MULTIPLY",
        "KP_MINUS"    => "KP_MINUS",
        "KP_PLUS"     => "KP_PLUS",
        "KP_ENTER"    => "KP_ENTER",
        "KP_DOT"      => "KP_DOT",
        // Misc
        "PSCR" | "PRINT_SCREEN" => "PSCRN",
        "SCRL" | "SCROLLLOCK"   => "SLCK",
        "PAUS" | "PAUSE"        => "PAUSE_BREAK",
        "APP"                   => "K_APP",
        _ => return None,
    })
}

/// Map a QMK MOD_* constant or modifier name to a ZMK modifier name.
#[must_use] pub fn qmk_mod_to_zmk(qmk_mod: &str) -> &'static str {
    match qmk_mod.trim() {
        "MOD_LALT" | "LALT" => "LALT",
        "MOD_RALT" | "RALT" => "RALT",
        "MOD_LCTL" | "LCTL" | "LCTRL" => "LCTRL",
        "MOD_RCTL" | "RCTL" | "RCTRL" => "RCTRL",
        "MOD_LSFT" | "LSFT" | "LSHIFT" => "LSHFT",
        "MOD_RSFT" | "RSFT" | "RSHIFT" => "RSHFT",
        "MOD_LGUI" | "LGUI" => "LGUI",
        "MOD_RGUI" | "RGUI" => "RGUI",
        _ => "UNKNOWN_MOD",
    }
}

/// Map a QMK modifier-wrapping function name (LGUI, LSFT, etc.) to its ZMK prefix.
#[must_use] pub fn qmk_mod_fn_to_zmk(name: &str) -> Option<&'static str> {
    Some(match name {
        "LGUI"           => "LG",
        "RGUI"           => "RG",
        "LSFT" | "LSHIFT" => "LS",
        "RSFT" | "RSHIFT" => "RS",
        "LCTL" | "LCTRL" => "LC",
        "RCTL" | "RCTRL" => "RC",
        "LALT"           => "LA",
        "RALT"           => "RA",
        _ => return None,
    })
}

/// Map a QMK RGB function name to the corresponding ZMK `rgb_ug` action string.
#[must_use] pub fn qmk_rgb_to_zmk(name: &str) -> Option<&'static str> {
    Some(match name {
        "RGB_TOG"                    => "RGB_TOG",
        "RGB_HUI"                    => "RGB_HUI",
        "RGB_HUD"                    => "RGB_HUD",
        "RGB_SAI"                    => "RGB_SAI",
        "RGB_SAD"                    => "RGB_SAD",
        "RGB_VAI"                    => "RGB_VAI",
        "RGB_VAD"                    => "RGB_VAD",
        "RGB_MODE_FORWARD" | "RGB_MOD"  => "RGB_EFF",
        "RGB_MODE_REVERSE" | "RGB_RMOD" => "RGB_EFR",
        _ => return None,
    })
}

/// Map a ZMK key name back to a QMK keycode string (with `KC_` prefix).
#[must_use]
pub fn zmk_key_to_qmk(zmk: &str) -> Option<&'static str> {
    Some(match zmk {
        // Letters
        "A" => "KC_A", "B" => "KC_B", "C" => "KC_C", "D" => "KC_D", "E" => "KC_E",
        "F" => "KC_F", "G" => "KC_G", "H" => "KC_H", "I" => "KC_I", "J" => "KC_J",
        "K" => "KC_K", "L" => "KC_L", "M" => "KC_M", "N" => "KC_N", "O" => "KC_O",
        "P" => "KC_P", "Q" => "KC_Q", "R" => "KC_R", "S" => "KC_S", "T" => "KC_T",
        "U" => "KC_U", "V" => "KC_V", "W" => "KC_W", "X" => "KC_X", "Y" => "KC_Y",
        "Z" => "KC_Z",
        // Numbers
        "N0" => "KC_0", "N1" => "KC_1", "N2" => "KC_2", "N3" => "KC_3", "N4" => "KC_4",
        "N5" => "KC_5", "N6" => "KC_6", "N7" => "KC_7", "N8" => "KC_8", "N9" => "KC_9",
        // Function keys
        "F1"  => "KC_F1",  "F2"  => "KC_F2",  "F3"  => "KC_F3",  "F4"  => "KC_F4",
        "F5"  => "KC_F5",  "F6"  => "KC_F6",  "F7"  => "KC_F7",  "F8"  => "KC_F8",
        "F9"  => "KC_F9",  "F10" => "KC_F10", "F11" => "KC_F11", "F12" => "KC_F12",
        "F13" => "KC_F13", "F14" => "KC_F14", "F15" => "KC_F15", "F16" => "KC_F16",
        "F17" => "KC_F17", "F18" => "KC_F18", "F19" => "KC_F19", "F20" => "KC_F20",
        "F21" => "KC_F21", "F22" => "KC_F22", "F23" => "KC_F23", "F24" => "KC_F24",
        // Common
        "TAB"   => "KC_TAB",
        "RET"   => "KC_ENTER",
        "ESC"   => "KC_ESCAPE",
        "BSPC"  => "KC_BSPC",
        "DEL"   => "KC_DEL",
        "INS"   => "KC_INS",
        "SPACE" => "KC_SPACE",
        "CAPS"  => "KC_CAPS",
        // Punctuation
        "MINUS" => "KC_MINUS",
        "EQUAL" => "KC_EQUAL",
        "LBKT"  => "KC_LBRC",
        "RBKT"  => "KC_RBRC",
        "BSLH"  => "KC_BSLS",
        "SEMI"  => "KC_SCLN",
        "SQT"   => "KC_QUOTE",
        "GRAVE" => "KC_GRAVE",
        "COMMA" => "KC_COMMA",
        "DOT"   => "KC_DOT",
        "FSLH"  => "KC_SLASH",
        // Shifted symbols
        "EXCL"  => "KC_EXLM",
        "AT"    => "KC_AT",
        "HASH"  => "KC_HASH",
        "DLLR"  => "KC_DLR",
        "PRCNT" => "KC_PERC",
        "CARET" => "KC_CIRC",
        "AMPS"  => "KC_AMPR",
        "STAR"  => "KC_ASTR",
        "LPAR"  => "KC_LPRN",
        "RPAR"  => "KC_RPRN",
        "UNDER" => "KC_UNDS",
        "PLUS"  => "KC_PLUS",
        "LBRC"  => "KC_LCBR",
        "RBRC"  => "KC_RCBR",
        "PIPE"  => "KC_PIPE",
        "TILDE" => "KC_TILD",
        "LT"    => "KC_LT",
        "GT"    => "KC_GT",
        "DQT"   => "KC_DQUO",
        "COLON" => "KC_COLN",
        "QMARK" => "KC_QUES",
        // Navigation
        "LEFT"  => "KC_LEFT",
        "RIGHT" => "KC_RIGHT",
        "UP"    => "KC_UP",
        "DOWN"  => "KC_DOWN",
        "PG_UP" => "KC_PGUP",
        "PG_DN" => "KC_PGDN",
        "HOME"  => "KC_HOME",
        "END"   => "KC_END",
        // Modifiers
        "LCTRL" => "KC_LCTL",
        "RCTRL" => "KC_RCTL",
        "LSHFT" => "KC_LSFT",
        "RSHFT" => "KC_RSFT",
        "LALT"  => "KC_LALT",
        "RALT"  => "KC_RALT",
        "LGUI"  => "KC_LGUI",
        "RGUI"  => "KC_RGUI",
        // Media
        "C_VOL_UP" => "KC_VOLU",
        "C_VOL_DN" => "KC_VOLD",
        "C_MUTE"   => "KC_MUTE",
        "C_BRI_UP" => "KC_BRIU",
        "C_BRI_DN" => "KC_BRID",
        "C_PP"     => "KC_MPLY",
        "C_NEXT"   => "KC_MNXT",
        "C_PREV"   => "KC_MPRV",
        // Keypad
        "KP_N0" => "KC_KP_0", "KP_N1" => "KC_KP_1", "KP_N2" => "KC_KP_2",
        "KP_N3" => "KC_KP_3", "KP_N4" => "KC_KP_4", "KP_N5" => "KC_KP_5",
        "KP_N6" => "KC_KP_6", "KP_N7" => "KC_KP_7", "KP_N8" => "KC_KP_8",
        "KP_N9" => "KC_KP_9",
        "KP_SLASH"    => "KC_KP_SLASH",
        "KP_MULTIPLY" => "KC_KP_ASTERISK",
        "KP_MINUS"    => "KC_KP_MINUS",
        "KP_PLUS"     => "KC_KP_PLUS",
        "KP_ENTER"    => "KC_KP_ENTER",
        "KP_DOT"      => "KC_KP_DOT",
        // Misc
        "PSCRN"       => "KC_PSCR",
        "SLCK"        => "KC_SCRL",
        "PAUSE_BREAK" => "KC_PAUS",
        "K_APP"       => "KC_APP",
        _ => return None,
    })
}

/// Map a ZMK modifier name to a QMK `MOD_*` constant.
#[must_use]
pub fn zmk_mod_to_qmk(zmk_mod: &str) -> &'static str {
    match zmk_mod.trim() {
        "LALT"  => "MOD_LALT",
        "RALT"  => "MOD_RALT",
        "RCTRL" => "MOD_RCTL",
        "LSHFT" => "MOD_LSFT",
        "RSHFT" => "MOD_RSFT",
        "LGUI"  => "MOD_LGUI",
        "RGUI"  => "MOD_RGUI",
        _       => "MOD_LCTL",
    }
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
        _ => return None,
    })
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
        assert_eq!(qmk_key_to_zmk("KC_SCLN"),  Some("SEMI"));
        assert_eq!(qmk_key_to_zmk("KC_QUOTE"), Some("SQT"));
        assert_eq!(qmk_key_to_zmk("KC_LBRC"),  Some("LBKT"));
        assert_eq!(qmk_key_to_zmk("KC_RBRC"),  Some("RBKT"));
        assert_eq!(qmk_key_to_zmk("KC_BSLS"),  Some("BSLH"));
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
        assert_eq!(qmk_key_to_zmk("KC_PGUP"),      Some("PG_UP"));
        assert_eq!(qmk_key_to_zmk("KC_PAGE_UP"),   Some("PG_UP"));
        assert_eq!(qmk_key_to_zmk("KC_PGDN"),      Some("PG_DN"));
        assert_eq!(qmk_key_to_zmk("KC_PAGE_DOWN"), Some("PG_DN"));
    }

    #[test]
    fn media_remapped() {
        assert_eq!(qmk_key_to_zmk("KC_AUDIO_VOL_UP"),    Some("C_VOL_UP"));
        assert_eq!(qmk_key_to_zmk("KC_AUDIO_VOL_DOWN"),  Some("C_VOL_DN"));
        assert_eq!(qmk_key_to_zmk("KC_BRIGHTNESS_UP"),   Some("C_BRI_UP"));
        assert_eq!(qmk_key_to_zmk("KC_BRIGHTNESS_DOWN"), Some("C_BRI_DN"));
    }

    #[test]
    fn common_keys() {
        assert_eq!(qmk_key_to_zmk("KC_TAB"),    Some("TAB"));
        assert_eq!(qmk_key_to_zmk("KC_ENTER"),  Some("RET"));
        assert_eq!(qmk_key_to_zmk("KC_ESCAPE"), Some("ESC"));
        assert_eq!(qmk_key_to_zmk("KC_BSPC"),   Some("BSPC"));
        assert_eq!(qmk_key_to_zmk("KC_SPACE"),  Some("SPACE"));
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
        assert_eq!(qmk_rgb_to_zmk("RGB_TOG"),          Some("RGB_TOG"));
        assert_eq!(qmk_rgb_to_zmk("RGB_HUI"),          Some("RGB_HUI"));
        assert_eq!(qmk_rgb_to_zmk("RGB_MODE_FORWARD"), Some("RGB_EFF"));
        assert_eq!(qmk_rgb_to_zmk("RGB_MOD"),          Some("RGB_EFF"));
        assert_eq!(qmk_rgb_to_zmk("RGB_RMOD"),         Some("RGB_EFR"));
        assert_eq!(qmk_rgb_to_zmk("NOT_RGB"),          None);
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
        assert_eq!(zmk_key_to_qmk("SEMI"),  Some("KC_SCLN"));
        assert_eq!(zmk_key_to_qmk("SQT"),   Some("KC_QUOTE"));
        assert_eq!(zmk_key_to_qmk("LBKT"),  Some("KC_LBRC"));
        assert_eq!(zmk_key_to_qmk("RBKT"),  Some("KC_RBRC"));
        assert_eq!(zmk_key_to_qmk("BSLH"),  Some("KC_BSLS"));
        assert_eq!(zmk_key_to_qmk("RET"),   Some("KC_ENTER"));
        assert_eq!(zmk_key_to_qmk("FSLH"),  Some("KC_SLASH"));
    }

    #[test]
    fn reverse_media() {
        assert_eq!(zmk_key_to_qmk("C_VOL_UP"), Some("KC_VOLU"));
        assert_eq!(zmk_key_to_qmk("C_VOL_DN"), Some("KC_VOLD"));
        assert_eq!(zmk_key_to_qmk("C_MUTE"),   Some("KC_MUTE"));
    }

    #[test]
    fn reverse_mod_constants() {
        assert_eq!(zmk_mod_to_qmk("LALT"),  "MOD_LALT");
        assert_eq!(zmk_mod_to_qmk("LCTRL"), "MOD_LCTL");
        assert_eq!(zmk_mod_to_qmk("LSHFT"), "MOD_LSFT");
        assert_eq!(zmk_mod_to_qmk("LGUI"),  "MOD_LGUI");
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
        assert_eq!(zmk_key_to_qmk("LC(C)"),     None);
    }
}
