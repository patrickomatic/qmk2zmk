//! Typed keycode domain used by both conversion directions.
//!
//! QMK and ZMK generally describe the same HID usages, but they often use
//! different symbolic names. Parsers normalize source spellings into the enums
//! in this module. Renderers write ZMK output via the standard [`fmt::Display`]
//! impl (which uses ZMK spelling) and QMK output via each type's `qmk_name()`
//! or `qmk_mod_name()` method.
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

use std::fmt;

macro_rules! keycodes {
    ($(($variant:ident, $zmk:literal, $qmk:literal)),+ $(,)?) => {
        /// A modeled HID keycode in the converter's canonical domain.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum KeyCode {
            $($variant),+
        }

        impl KeyCode {
            /// Parse a QMK keycode token into a modeled keycode.
            #[must_use]
            pub fn from_qmk(qmk: &str) -> Option<Self> {
                keycode_from_qmk(qmk)
            }

            /// Parse a ZMK key name into a modeled keycode.
            #[must_use]
            pub fn from_zmk(zmk: &str) -> Option<Self> {
                Some(match zmk {
                    $($zmk => Self::$variant,)+
                    unsupported => {
                        let _ = unsupported.len();
                        return None;
                    }
                })
            }

            /// Return this keycode's canonical ZMK spelling.
            #[must_use]
            pub const fn zmk_name(self) -> &'static str {
                match self {
                    $(Self::$variant => $zmk,)+
                }
            }

            /// Return this keycode's canonical QMK spelling.
            #[must_use]
            pub const fn qmk_name(self) -> &'static str {
                match self {
                    $(Self::$variant => $qmk,)+
                }
            }
        }
    };
}

keycodes! {
    (A, "A", "KC_A"), (B, "B", "KC_B"), (C, "C", "KC_C"), (D, "D", "KC_D"),
    (E, "E", "KC_E"), (F, "F", "KC_F"), (G, "G", "KC_G"), (H, "H", "KC_H"),
    (I, "I", "KC_I"), (J, "J", "KC_J"), (K, "K", "KC_K"), (L, "L", "KC_L"),
    (M, "M", "KC_M"), (N, "N", "KC_N"), (O, "O", "KC_O"), (P, "P", "KC_P"),
    (Q, "Q", "KC_Q"), (R, "R", "KC_R"), (S, "S", "KC_S"), (T, "T", "KC_T"),
    (U, "U", "KC_U"), (V, "V", "KC_V"), (W, "W", "KC_W"), (X, "X", "KC_X"),
    (Y, "Y", "KC_Y"), (Z, "Z", "KC_Z"),
    (N0, "N0", "KC_0"), (N1, "N1", "KC_1"), (N2, "N2", "KC_2"),
    (N3, "N3", "KC_3"), (N4, "N4", "KC_4"), (N5, "N5", "KC_5"),
    (N6, "N6", "KC_6"), (N7, "N7", "KC_7"), (N8, "N8", "KC_8"),
    (N9, "N9", "KC_9"),
    (F1, "F1", "KC_F1"), (F2, "F2", "KC_F2"), (F3, "F3", "KC_F3"),
    (F4, "F4", "KC_F4"), (F5, "F5", "KC_F5"), (F6, "F6", "KC_F6"),
    (F7, "F7", "KC_F7"), (F8, "F8", "KC_F8"), (F9, "F9", "KC_F9"),
    (F10, "F10", "KC_F10"), (F11, "F11", "KC_F11"), (F12, "F12", "KC_F12"),
    (F13, "F13", "KC_F13"), (F14, "F14", "KC_F14"), (F15, "F15", "KC_F15"),
    (F16, "F16", "KC_F16"), (F17, "F17", "KC_F17"), (F18, "F18", "KC_F18"),
    (F19, "F19", "KC_F19"), (F20, "F20", "KC_F20"), (F21, "F21", "KC_F21"),
    (F22, "F22", "KC_F22"), (F23, "F23", "KC_F23"), (F24, "F24", "KC_F24"),
    (Tab, "TAB", "KC_TAB"), (Ret, "RET", "KC_ENTER"), (Esc, "ESC", "KC_ESCAPE"),
    (Bspc, "BSPC", "KC_BSPC"), (Del, "DEL", "KC_DEL"), (Ins, "INS", "KC_INS"),
    (Space, "SPACE", "KC_SPACE"), (Caps, "CAPS", "KC_CAPS"),
    (Minus, "MINUS", "KC_MINUS"), (Equal, "EQUAL", "KC_EQUAL"),
    (Lbkt, "LBKT", "KC_LBRC"), (Rbkt, "RBKT", "KC_RBRC"),
    (Bslh, "BSLH", "KC_BSLS"), (Semi, "SEMI", "KC_SCLN"),
    (Sqt, "SQT", "KC_QUOTE"), (Grave, "GRAVE", "KC_GRAVE"),
    (Comma, "COMMA", "KC_COMMA"), (Dot, "DOT", "KC_DOT"), (Fslh, "FSLH", "KC_SLASH"),
    (Excl, "EXCL", "KC_EXLM"), (At, "AT", "KC_AT"), (Hash, "HASH", "KC_HASH"),
    (Dllr, "DLLR", "KC_DLR"), (Prcnt, "PRCNT", "KC_PERC"),
    (Caret, "CARET", "KC_CIRC"), (Amps, "AMPS", "KC_AMPR"),
    (Star, "STAR", "KC_ASTR"), (Lpar, "LPAR", "KC_LPRN"),
    (Rpar, "RPAR", "KC_RPRN"), (Under, "UNDER", "KC_UNDS"),
    (Plus, "PLUS", "KC_PLUS"), (Lbrc, "LBRC", "KC_LCBR"), (Rbrc, "RBRC", "KC_RCBR"),
    (Pipe, "PIPE", "KC_PIPE"), (Tilde, "TILDE", "KC_TILD"), (Lt, "LT", "KC_LT"),
    (Gt, "GT", "KC_GT"), (Dqt, "DQT", "KC_DQUO"), (Colon, "COLON", "KC_COLN"),
    (Qmark, "QMARK", "KC_QUES"),
    (Left, "LEFT", "KC_LEFT"), (Right, "RIGHT", "KC_RIGHT"),
    (Up, "UP", "KC_UP"), (Down, "DOWN", "KC_DOWN"),
    (PgUp, "PG_UP", "KC_PGUP"), (PgDn, "PG_DN", "KC_PGDN"),
    (Home, "HOME", "KC_HOME"), (End, "END", "KC_END"),
    (LCtrl, "LCTRL", "KC_LCTL"), (RCtrl, "RCTRL", "KC_RCTL"),
    (LShft, "LSHFT", "KC_LSFT"), (RShft, "RSHFT", "KC_RSFT"),
    (LAlt, "LALT", "KC_LALT"), (RAlt, "RALT", "KC_RALT"),
    (LGui, "LGUI", "KC_LGUI"), (RGui, "RGUI", "KC_RGUI"),
    (CVolUp, "C_VOL_UP", "KC_VOLU"), (CVolDn, "C_VOL_DN", "KC_VOLD"),
    (CMute, "C_MUTE", "KC_MUTE"), (CBriUp, "C_BRI_UP", "KC_BRIU"),
    (CBriDn, "C_BRI_DN", "KC_BRID"), (CPp, "C_PP", "KC_MPLY"),
    (CNext, "C_NEXT", "KC_MNXT"), (CPrev, "C_PREV", "KC_MPRV"),
    (KpN0, "KP_N0", "KC_KP_0"), (KpN1, "KP_N1", "KC_KP_1"),
    (KpN2, "KP_N2", "KC_KP_2"), (KpN3, "KP_N3", "KC_KP_3"),
    (KpN4, "KP_N4", "KC_KP_4"), (KpN5, "KP_N5", "KC_KP_5"),
    (KpN6, "KP_N6", "KC_KP_6"), (KpN7, "KP_N7", "KC_KP_7"),
    (KpN8, "KP_N8", "KC_KP_8"), (KpN9, "KP_N9", "KC_KP_9"),
    (KpSlash, "KP_SLASH", "KC_KP_SLASH"),
    (KpMultiply, "KP_MULTIPLY", "KC_KP_ASTERISK"),
    (KpMinus, "KP_MINUS", "KC_KP_MINUS"), (KpPlus, "KP_PLUS", "KC_KP_PLUS"),
    (KpEnter, "KP_ENTER", "KC_KP_ENTER"), (KpDot, "KP_DOT", "KC_KP_DOT"),
    (KRedo, "K_REDO", "KC_AGAIN"), (KUndo, "K_UNDO", "KC_UNDO"),
    (KCut, "K_CUT", "KC_CUT"), (KCopy, "K_COPY", "KC_COPY"),
    (KPaste, "K_PASTE", "KC_PASTE"),
    (NonUsBslh, "NON_US_BSLH", "KC_NUBS"), (NonUsHash, "NON_US_HASH", "KC_NUHS"),
    (Pscrn, "PSCRN", "KC_PSCR"), (Slck, "SLCK", "KC_SCRL"),
    (PauseBreak, "PAUSE_BREAK", "KC_PAUS"), (KApp, "K_APP", "KC_APP"),
}

#[allow(clippy::too_many_lines)]
fn keycode_from_qmk(qmk: &str) -> Option<KeyCode> {
    let key = qmk.strip_prefix("KC_").unwrap_or(qmk);
    Some(match key {
        "A" => KeyCode::A,
        "B" => KeyCode::B,
        "C" => KeyCode::C,
        "D" => KeyCode::D,
        "E" => KeyCode::E,
        "F" => KeyCode::F,
        "G" => KeyCode::G,
        "H" => KeyCode::H,
        "I" => KeyCode::I,
        "J" => KeyCode::J,
        "K" => KeyCode::K,
        "L" => KeyCode::L,
        "M" => KeyCode::M,
        "N" => KeyCode::N,
        "O" => KeyCode::O,
        "P" => KeyCode::P,
        "Q" => KeyCode::Q,
        "R" => KeyCode::R,
        "S" => KeyCode::S,
        "T" => KeyCode::T,
        "U" => KeyCode::U,
        "V" => KeyCode::V,
        "W" => KeyCode::W,
        "X" => KeyCode::X,
        "Y" => KeyCode::Y,
        "Z" => KeyCode::Z,
        "0" => KeyCode::N0,
        "1" => KeyCode::N1,
        "2" => KeyCode::N2,
        "3" => KeyCode::N3,
        "4" => KeyCode::N4,
        "5" => KeyCode::N5,
        "6" => KeyCode::N6,
        "7" => KeyCode::N7,
        "8" => KeyCode::N8,
        "9" => KeyCode::N9,
        "F1" => KeyCode::F1,
        "F2" => KeyCode::F2,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "F10" => KeyCode::F10,
        "F11" => KeyCode::F11,
        "F12" => KeyCode::F12,
        "F13" => KeyCode::F13,
        "F14" => KeyCode::F14,
        "F15" => KeyCode::F15,
        "F16" => KeyCode::F16,
        "F17" => KeyCode::F17,
        "F18" => KeyCode::F18,
        "F19" => KeyCode::F19,
        "F20" => KeyCode::F20,
        "F21" => KeyCode::F21,
        "F22" => KeyCode::F22,
        "F23" => KeyCode::F23,
        "F24" => KeyCode::F24,
        "TAB" => KeyCode::Tab,
        "ENTER" | "ENT" => KeyCode::Ret,
        "ESCAPE" | "ESC" => KeyCode::Esc,
        "BSPC" => KeyCode::Bspc,
        "DEL" | "DELETE" => KeyCode::Del,
        "INS" | "INSERT" => KeyCode::Ins,
        "SPACE" | "SPC" => KeyCode::Space,
        "CAPS" | "CAPS_LOCK" | "CAPSLOCK" => KeyCode::Caps,
        "MINUS" => KeyCode::Minus,
        "EQUAL" => KeyCode::Equal,
        "LBRC" => KeyCode::Lbkt,
        "RBRC" => KeyCode::Rbkt,
        "BSLS" => KeyCode::Bslh,
        "SCLN" => KeyCode::Semi,
        "QUOTE" | "QUOT" => KeyCode::Sqt,
        "GRAVE" | "GRV" => KeyCode::Grave,
        "COMMA" | "COMM" => KeyCode::Comma,
        "DOT" => KeyCode::Dot,
        "SLASH" | "SLSH" => KeyCode::Fslh,
        "EXLM" => KeyCode::Excl,
        "AT" => KeyCode::At,
        "HASH" => KeyCode::Hash,
        "DLR" => KeyCode::Dllr,
        "PERC" => KeyCode::Prcnt,
        "CIRC" => KeyCode::Caret,
        "AMPR" => KeyCode::Amps,
        "ASTR" => KeyCode::Star,
        "LPRN" => KeyCode::Lpar,
        "RPRN" => KeyCode::Rpar,
        "UNDS" => KeyCode::Under,
        "PLUS" => KeyCode::Plus,
        "LCBR" => KeyCode::Lbrc,
        "RCBR" => KeyCode::Rbrc,
        "PIPE" => KeyCode::Pipe,
        "TILD" => KeyCode::Tilde,
        "LT" => KeyCode::Lt,
        "GT" => KeyCode::Gt,
        "DQUO" => KeyCode::Dqt,
        "COLN" => KeyCode::Colon,
        "QUES" => KeyCode::Qmark,
        "LEFT" => KeyCode::Left,
        "RIGHT" => KeyCode::Right,
        "UP" => KeyCode::Up,
        "DOWN" => KeyCode::Down,
        "PGUP" | "PAGE_UP" => KeyCode::PgUp,
        "PGDN" | "PAGE_DOWN" => KeyCode::PgDn,
        "HOME" => KeyCode::Home,
        "END" => KeyCode::End,
        "LCTL" | "LCTRL" => KeyCode::LCtrl,
        "RCTL" | "RCTRL" => KeyCode::RCtrl,
        "LSFT" | "LSHIFT" => KeyCode::LShft,
        "RSFT" | "RSHIFT" => KeyCode::RShft,
        "LALT" => KeyCode::LAlt,
        "RALT" => KeyCode::RAlt,
        "LGUI" => KeyCode::LGui,
        "RGUI" => KeyCode::RGui,
        "AUDIO_VOL_UP" | "VOLU" => KeyCode::CVolUp,
        "AUDIO_VOL_DOWN" | "VOLD" => KeyCode::CVolDn,
        "AUDIO_MUTE" | "MUTE" => KeyCode::CMute,
        "BRIGHTNESS_UP" | "BRIU" => KeyCode::CBriUp,
        "BRIGHTNESS_DOWN" | "BRID" => KeyCode::CBriDn,
        "MEDIA_PLAY_PAUSE" | "MPLY" => KeyCode::CPp,
        "MEDIA_NEXT_TRACK" | "MNXT" => KeyCode::CNext,
        "MEDIA_PREV_TRACK" | "MPRV" => KeyCode::CPrev,
        "KP_0" => KeyCode::KpN0,
        "KP_1" => KeyCode::KpN1,
        "KP_2" => KeyCode::KpN2,
        "KP_3" => KeyCode::KpN3,
        "KP_4" => KeyCode::KpN4,
        "KP_5" => KeyCode::KpN5,
        "KP_6" => KeyCode::KpN6,
        "KP_7" => KeyCode::KpN7,
        "KP_8" => KeyCode::KpN8,
        "KP_9" => KeyCode::KpN9,
        "KP_SLASH" => KeyCode::KpSlash,
        "KP_ASTERISK" => KeyCode::KpMultiply,
        "KP_MINUS" => KeyCode::KpMinus,
        "KP_PLUS" => KeyCode::KpPlus,
        "KP_ENTER" => KeyCode::KpEnter,
        "KP_DOT" => KeyCode::KpDot,
        "AGAIN" | "AGIN" => KeyCode::KRedo,
        "UNDO" => KeyCode::KUndo,
        "CUT" => KeyCode::KCut,
        "COPY" => KeyCode::KCopy,
        "PASTE" | "PSTE" => KeyCode::KPaste,
        "NUBS" | "NONUS_BACKSLASH" => KeyCode::NonUsBslh,
        "NUHS" | "NONUS_HASH" => KeyCode::NonUsHash,
        "PSCR" | "PRINT_SCREEN" => KeyCode::Pscrn,
        "SCRL" | "SCROLLLOCK" => KeyCode::Slck,
        "PAUS" | "PAUSE" => KeyCode::PauseBreak,
        "APP" => KeyCode::KApp,
        unsupported => {
            let _ = unsupported.len();
            return None;
        }
    })
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.zmk_name())
    }
}

impl From<&str> for KeyExpr {
    fn from(value: &str) -> Self {
        Self::parse_zmk(value)
    }
}

impl From<String> for KeyExpr {
    fn from(value: String) -> Self {
        Self::parse_zmk(&value)
    }
}

impl PartialEq<&str> for KeyExpr {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Self::Key(code) => code.zmk_name() == *other,
            Self::Raw(raw) => raw.as_str() == *other,
            // Modified requires building the formatted string; allocation is unavoidable.
            #[allow(clippy::cmp_owned)]
            Self::Modified(_, _) => self.to_string() == *other,
        }
    }
}

impl PartialEq<str> for KeyExpr {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Key(code) => code.zmk_name() == other,
            Self::Raw(raw) => raw.as_str() == other,
            #[allow(clippy::cmp_owned)]
            Self::Modified(_, _) => self.to_string() == other,
        }
    }
}

/// A keyboard modifier in the converter's canonical domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modifier {
    LAlt,
    RAlt,
    LCtrl,
    RCtrl,
    LShft,
    RShft,
    LGui,
    RGui,
    Unknown(String),
}

impl Modifier {
    #[must_use]
    pub fn from_qmk(qmk_mod: &str) -> Option<Self> {
        Some(match qmk_mod.trim() {
            "MOD_LALT" | "LALT" => Self::LAlt,
            "MOD_RALT" | "RALT" => Self::RAlt,
            "MOD_LCTL" | "LCTL" | "LCTRL" => Self::LCtrl,
            "MOD_RCTL" | "RCTL" | "RCTRL" => Self::RCtrl,
            "MOD_LSFT" | "LSFT" | "LSHIFT" => Self::LShft,
            "MOD_RSFT" | "RSFT" | "RSHIFT" => Self::RShft,
            "MOD_LGUI" | "LGUI" => Self::LGui,
            "MOD_RGUI" | "RGUI" => Self::RGui,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn from_zmk(zmk_mod: &str) -> Option<Self> {
        Some(match zmk_mod.trim() {
            "LALT" => Self::LAlt,
            "RALT" => Self::RAlt,
            "LCTRL" => Self::LCtrl,
            "RCTRL" => Self::RCtrl,
            "LSHFT" => Self::LShft,
            "RSHFT" => Self::RShft,
            "LGUI" => Self::LGui,
            "RGUI" => Self::RGui,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn zmk_name(&self) -> &str {
        match self {
            Self::LAlt => "LALT",
            Self::RAlt => "RALT",
            Self::LCtrl => "LCTRL",
            Self::RCtrl => "RCTRL",
            Self::LShft => "LSHFT",
            Self::RShft => "RSHFT",
            Self::LGui => "LGUI",
            Self::RGui => "RGUI",
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    pub fn qmk_mod_name(&self) -> &str {
        match self {
            Self::LAlt => "MOD_LALT",
            Self::RAlt => "MOD_RALT",
            Self::LCtrl => "MOD_LCTL",
            Self::RCtrl => "MOD_RCTL",
            Self::LShft => "MOD_LSFT",
            Self::RShft => "MOD_RSFT",
            Self::LGui => "MOD_LGUI",
            Self::RGui => "MOD_RGUI",
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    pub fn qmk_fn_name(&self) -> &str {
        match self {
            Self::LAlt => "LALT",
            Self::RAlt => "RALT",
            Self::LCtrl => "LCTL",
            Self::RCtrl => "RCTL",
            Self::LShft => "LSFT",
            Self::RShft => "RSFT",
            Self::LGui => "LGUI",
            Self::RGui => "RGUI",
            Self::Unknown(raw) => raw,
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.zmk_name())
    }
}

impl From<&str> for Modifier {
    fn from(value: &str) -> Self {
        Self::from_zmk(value)
            .or_else(|| Self::from_qmk(value))
            .unwrap_or_else(|| Self::Unknown(value.to_string()))
    }
}

impl From<String> for Modifier {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl PartialEq<&str> for Modifier {
    fn eq(&self, other: &&str) -> bool {
        self.zmk_name() == *other
    }
}

impl PartialEq<str> for Modifier {
    fn eq(&self, other: &str) -> bool {
        self.zmk_name() == other
    }
}

/// A ZMK modifier-wrapper prefix used in key expressions like `LG(C)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModPrefix {
    LG,
    RG,
    LS,
    RS,
    LC,
    RC,
    LA,
    RA,
}

impl ModPrefix {
    #[must_use]
    pub fn from_qmk_fn(name: &str) -> Option<Self> {
        Some(match name {
            "LGUI" => Self::LG,
            "RGUI" => Self::RG,
            "LSFT" | "LSHIFT" => Self::LS,
            "RSFT" | "RSHIFT" => Self::RS,
            "LCTL" | "LCTRL" => Self::LC,
            "RCTL" | "RCTRL" => Self::RC,
            "LALT" => Self::LA,
            "RALT" => Self::RA,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn from_zmk(prefix: &str) -> Option<Self> {
        Some(match prefix {
            "LG" => Self::LG,
            "RG" => Self::RG,
            "LS" => Self::LS,
            "RS" => Self::RS,
            "LC" => Self::LC,
            "RC" => Self::RC,
            "LA" => Self::LA,
            "RA" => Self::RA,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub const fn zmk_name(self) -> &'static str {
        match self {
            Self::LG => "LG",
            Self::RG => "RG",
            Self::LS => "LS",
            Self::RS => "RS",
            Self::LC => "LC",
            Self::RC => "RC",
            Self::LA => "LA",
            Self::RA => "RA",
        }
    }

    #[must_use]
    pub const fn qmk_fn_name(self) -> &'static str {
        match self {
            Self::LG => "LGUI",
            Self::RG => "RGUI",
            Self::LS => "LSFT",
            Self::RS => "RSFT",
            Self::LC => "LCTL",
            Self::RC => "RCTL",
            Self::LA => "LALT",
            Self::RA => "RALT",
        }
    }
}

impl fmt::Display for ModPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.zmk_name())
    }
}

/// A typed key expression. Most expressions are a keycode, optionally wrapped in
/// one or more modifier prefixes. `Raw` is reserved for source constructs that
/// are not modeled yet but still need to round-trip visibly.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyExpr {
    Key(KeyCode),
    Modified(ModPrefix, Box<KeyExpr>),
    Raw(String),
}

impl KeyExpr {
    #[must_use]
    pub fn from_qmk_key(qmk: &str) -> Option<Self> {
        KeyCode::from_qmk(qmk).map(Self::Key)
    }

    #[must_use]
    pub fn from_zmk_key(zmk: &str) -> Option<Self> {
        KeyCode::from_zmk(zmk).map(Self::Key)
    }

    #[must_use]
    pub fn parse_zmk(s: &str) -> Self {
        let s = s.trim();
        if let Some(paren) = s.find('(')
            && let Some(prefix) = ModPrefix::from_zmk(&s[..paren])
            && let Some(inner) = extract_paren_inner(s, paren)
        {
            return Self::Modified(prefix, Box::new(Self::parse_zmk(inner)));
        }
        Self::from_zmk_key(s).unwrap_or_else(|| Self::Raw(s.to_string()))
    }

    #[must_use]
    pub fn to_qmk(&self) -> String {
        match self {
            Self::Key(code) => code.qmk_name().to_string(),
            Self::Modified(prefix, inner) => {
                format!("{}({})", prefix.qmk_fn_name(), inner.to_qmk())
            }
            Self::Raw(raw) => raw.clone(),
        }
    }
}

impl fmt::Display for KeyExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(code) => f.write_str(code.zmk_name()),
            Self::Modified(prefix, inner) => write!(f, "{prefix}({inner})"),
            Self::Raw(raw) => f.write_str(raw),
        }
    }
}

/// ZMK RGB underglow actions modeled by the converter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RgbAction {
    Toggle,
    HueInc,
    HueDec,
    SatInc,
    SatDec,
    ValInc,
    ValDec,
    EffectNext,
    EffectPrev,
    SpeedInc,
    SpeedDec,
    Unknown(String),
}

impl RgbAction {
    #[must_use]
    pub fn from_qmk(name: &str) -> Option<Self> {
        Some(match name {
            "RGB_TOG" => Self::Toggle,
            "RGB_HUI" => Self::HueInc,
            "RGB_HUD" => Self::HueDec,
            "RGB_SAI" => Self::SatInc,
            "RGB_SAD" => Self::SatDec,
            "RGB_VAI" => Self::ValInc,
            "RGB_VAD" => Self::ValDec,
            "RGB_MODE_FORWARD" | "RGB_MOD" => Self::EffectNext,
            "RGB_MODE_REVERSE" | "RGB_RMOD" => Self::EffectPrev,
            "RGB_SPI" => Self::SpeedInc,
            "RGB_SPD" => Self::SpeedDec,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn from_zmk(name: &str) -> Option<Self> {
        Some(match name {
            "RGB_TOG" => Self::Toggle,
            "RGB_HUI" => Self::HueInc,
            "RGB_HUD" => Self::HueDec,
            "RGB_SAI" => Self::SatInc,
            "RGB_SAD" => Self::SatDec,
            "RGB_VAI" => Self::ValInc,
            "RGB_VAD" => Self::ValDec,
            "RGB_EFF" => Self::EffectNext,
            "RGB_EFR" => Self::EffectPrev,
            "RGB_SPI" => Self::SpeedInc,
            "RGB_SPD" => Self::SpeedDec,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn zmk_name(&self) -> &str {
        match self {
            Self::Toggle => "RGB_TOG",
            Self::HueInc => "RGB_HUI",
            Self::HueDec => "RGB_HUD",
            Self::SatInc => "RGB_SAI",
            Self::SatDec => "RGB_SAD",
            Self::ValInc => "RGB_VAI",
            Self::ValDec => "RGB_VAD",
            Self::EffectNext => "RGB_EFF",
            Self::EffectPrev => "RGB_EFR",
            Self::SpeedInc => "RGB_SPI",
            Self::SpeedDec => "RGB_SPD",
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    pub fn qmk_name(&self) -> &str {
        match self {
            Self::Toggle => "RGB_TOG",
            Self::HueInc => "RGB_HUI",
            Self::HueDec => "RGB_HUD",
            Self::SatInc => "RGB_SAI",
            Self::SatDec => "RGB_SAD",
            Self::ValInc => "RGB_VAI",
            Self::ValDec => "RGB_VAD",
            Self::EffectNext => "RGB_MODE_FORWARD",
            Self::EffectPrev => "RGB_MODE_REVERSE",
            Self::SpeedInc => "RGB_SPI",
            Self::SpeedDec => "RGB_SPD",
            Self::Unknown(raw) => raw,
        }
    }
}

impl fmt::Display for RgbAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.zmk_name())
    }
}

impl From<&str> for RgbAction {
    fn from(value: &str) -> Self {
        Self::from_zmk(value)
            .or_else(|| Self::from_qmk(value))
            .unwrap_or_else(|| Self::Unknown(value.to_string()))
    }
}

impl From<String> for RgbAction {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl PartialEq<&str> for RgbAction {
    fn eq(&self, other: &&str) -> bool {
        self.zmk_name() == *other
    }
}

impl PartialEq<str> for RgbAction {
    fn eq(&self, other: &str) -> bool {
        self.zmk_name() == other
    }
}

/// ZMK mouse movement directions modeled by the converter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MouseMovement {
    Up,
    Down,
    Left,
    Right,
    Unknown(String),
}

impl MouseMovement {
    #[must_use]
    pub fn from_zmk(name: &str) -> Option<Self> {
        Some(match name {
            "MOVE_UP" => Self::Up,
            "MOVE_DOWN" => Self::Down,
            "MOVE_LEFT" => Self::Left,
            "MOVE_RIGHT" => Self::Right,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn from_qmk(name: &str) -> Option<Self> {
        let key = name.strip_prefix("KC_").unwrap_or(name);
        Some(match key {
            "MS_U" | "MS_UP" => Self::Up,
            "MS_D" | "MS_DOWN" => Self::Down,
            "MS_L" | "MS_LEFT" => Self::Left,
            "MS_R" | "MS_RIGHT" => Self::Right,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn zmk_name(&self) -> &str {
        match self {
            Self::Up => "MOVE_UP",
            Self::Down => "MOVE_DOWN",
            Self::Left => "MOVE_LEFT",
            Self::Right => "MOVE_RIGHT",
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    pub fn qmk_name(&self) -> &str {
        match self {
            Self::Up => "KC_MS_U",
            Self::Down => "KC_MS_D",
            Self::Left => "KC_MS_L",
            Self::Right => "KC_MS_R",
            Self::Unknown(raw) => raw,
        }
    }
}

/// ZMK mouse button names modeled by the converter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Unknown(String),
}

impl MouseButton {
    #[must_use]
    pub fn from_zmk(name: &str) -> Option<Self> {
        Some(match name {
            "LCLK" => Self::Left,
            "RCLK" => Self::Right,
            "MCLK" => Self::Middle,
            "BTN4" => Self::Button4,
            "BTN5" => Self::Button5,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn from_qmk(name: &str) -> Option<Self> {
        let key = name.strip_prefix("KC_").unwrap_or(name);
        Some(match key {
            "BTN1" => Self::Left,
            "BTN2" => Self::Right,
            "BTN3" => Self::Middle,
            "BTN4" => Self::Button4,
            "BTN5" => Self::Button5,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn zmk_name(&self) -> &str {
        match self {
            Self::Left => "LCLK",
            Self::Right => "RCLK",
            Self::Middle => "MCLK",
            Self::Button4 => "BTN4",
            Self::Button5 => "BTN5",
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    pub fn qmk_name(&self) -> &str {
        match self {
            Self::Left => "KC_BTN1",
            Self::Right => "KC_BTN2",
            Self::Middle => "KC_BTN3",
            Self::Button4 => "KC_BTN4",
            Self::Button5 => "KC_BTN5",
            Self::Unknown(raw) => raw,
        }
    }
}

/// ZMK mouse scroll directions modeled by the converter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MouseScroll {
    Up,
    Down,
    Left,
    Right,
    Unknown(String),
}

impl MouseScroll {
    #[must_use]
    pub fn from_zmk(name: &str) -> Option<Self> {
        Some(match name {
            "SCRL_UP" => Self::Up,
            "SCRL_DOWN" => Self::Down,
            "SCRL_LEFT" => Self::Left,
            "SCRL_RIGHT" => Self::Right,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn from_qmk(name: &str) -> Option<Self> {
        let key = name.strip_prefix("KC_").unwrap_or(name);
        Some(match key {
            "WH_U" | "MS_WH_UP" => Self::Up,
            "WH_D" | "MS_WH_DOWN" => Self::Down,
            "WH_L" | "MS_WH_LEFT" => Self::Left,
            "WH_R" | "MS_WH_RIGHT" => Self::Right,
            unsupported => {
                let _ = unsupported.len();
                return None;
            }
        })
    }

    #[must_use]
    pub fn zmk_name(&self) -> &str {
        match self {
            Self::Up => "SCRL_UP",
            Self::Down => "SCRL_DOWN",
            Self::Left => "SCRL_LEFT",
            Self::Right => "SCRL_RIGHT",
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    pub fn qmk_name(&self) -> &str {
        match self {
            Self::Up => "KC_WH_U",
            Self::Down => "KC_WH_D",
            Self::Left => "KC_WH_L",
            Self::Right => "KC_WH_R",
            Self::Unknown(raw) => raw,
        }
    }
}

impl fmt::Display for MouseMovement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.zmk_name())
    }
}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.zmk_name())
    }
}

impl fmt::Display for MouseScroll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.zmk_name())
    }
}

impl From<&str> for MouseMovement {
    fn from(value: &str) -> Self {
        Self::from_zmk(value)
            .or_else(|| Self::from_qmk(value))
            .unwrap_or_else(|| Self::Unknown(value.to_string()))
    }
}

impl From<String> for MouseMovement {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for MouseButton {
    fn from(value: &str) -> Self {
        Self::from_zmk(value)
            .or_else(|| Self::from_qmk(value))
            .unwrap_or_else(|| Self::Unknown(value.to_string()))
    }
}

impl From<String> for MouseButton {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for MouseScroll {
    fn from(value: &str) -> Self {
        Self::from_zmk(value)
            .or_else(|| Self::from_qmk(value))
            .unwrap_or_else(|| Self::Unknown(value.to_string()))
    }
}

impl From<String> for MouseScroll {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl PartialEq<&str> for MouseMovement {
    fn eq(&self, other: &&str) -> bool {
        self.zmk_name() == *other
    }
}

impl PartialEq<str> for MouseMovement {
    fn eq(&self, other: &str) -> bool {
        self.zmk_name() == other
    }
}

impl PartialEq<&str> for MouseButton {
    fn eq(&self, other: &&str) -> bool {
        self.zmk_name() == *other
    }
}

impl PartialEq<str> for MouseButton {
    fn eq(&self, other: &str) -> bool {
        self.zmk_name() == other
    }
}

impl PartialEq<&str> for MouseScroll {
    fn eq(&self, other: &&str) -> bool {
        self.zmk_name() == *other
    }
}

impl PartialEq<str> for MouseScroll {
    fn eq(&self, other: &str) -> bool {
        self.zmk_name() == other
    }
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
    fn qmk_key_aliases_parse_to_canonical_domain() {
        assert_eq!(KeyCode::from_qmk("KC_A"), Some(KeyCode::A));
        assert_eq!(KeyCode::from_qmk("KC_0"), Some(KeyCode::N0));
        assert_eq!(KeyCode::from_qmk("KC_ENTER"), Some(KeyCode::Ret));
        assert_eq!(KeyCode::from_qmk("KC_LBRC"), Some(KeyCode::Lbkt));
        assert_eq!(KeyCode::from_qmk("KC_LCBR"), Some(KeyCode::Lbrc));
        assert_eq!(KeyCode::from_qmk("KC_AUDIO_VOL_UP"), Some(KeyCode::CVolUp));
        assert_eq!(KeyCode::from_qmk("KC_DOESNOTEXIST"), None);
    }

    #[test]
    fn keycodes_render_to_both_domains() {
        assert_eq!(KeyCode::Lbkt.to_string(), "LBKT");
        assert_eq!(KeyCode::Lbkt.qmk_name(), "KC_LBRC");
        assert_eq!(KeyCode::Ret.to_string(), "RET");
        assert_eq!(KeyCode::Ret.qmk_name(), "KC_ENTER");
    }

    #[test]
    fn zmk_key_names_parse_to_canonical_domain() {
        assert_eq!(KeyCode::from_zmk("N9"), Some(KeyCode::N9));
        assert_eq!(KeyCode::from_zmk("SQT"), Some(KeyCode::Sqt));
        assert_eq!(KeyCode::from_zmk("NON_US_BSLH"), Some(KeyCode::NonUsBslh));
        assert_eq!(KeyCode::from_zmk("NOT_A_KEY"), None);
    }

    #[test]
    fn modifiers_parse_and_render() {
        assert_eq!(Modifier::from_qmk("MOD_LSFT"), Some(Modifier::LShft));
        assert_eq!(Modifier::from_zmk("LCTRL"), Some(Modifier::LCtrl));
        assert_eq!(Modifier::LGui.to_string(), "LGUI");
        assert_eq!(Modifier::LGui.qmk_mod_name(), "MOD_LGUI");
    }

    #[test]
    fn mod_prefixes_parse_and_render() {
        assert_eq!(ModPrefix::from_qmk_fn("LGUI"), Some(ModPrefix::LG));
        assert_eq!(ModPrefix::from_zmk("RS"), Some(ModPrefix::RS));
        assert_eq!(ModPrefix::LG.zmk_name(), "LG");
        assert_eq!(ModPrefix::LG.qmk_fn_name(), "LGUI");
        assert_eq!(ModPrefix::from_qmk_fn("KC_A"), None);
    }

    #[test]
    fn rgb_actions_parse_and_render() {
        assert_eq!(RgbAction::from_qmk("RGB_MOD"), Some(RgbAction::EffectNext));
        assert_eq!(RgbAction::from_qmk("RGB_RMOD"), Some(RgbAction::EffectPrev));
        assert_eq!(RgbAction::from_zmk("RGB_SPI"), Some(RgbAction::SpeedInc));
        assert_eq!(RgbAction::EffectNext.to_string(), "RGB_EFF");
        assert_eq!(RgbAction::EffectNext.qmk_name(), "RGB_MODE_FORWARD");
        assert_eq!(RgbAction::from_qmk("NOT_RGB"), None);
    }

    #[test]
    fn key_expr_simple_and_nested_modifiers_render_to_qmk() {
        assert_eq!(KeyExpr::parse_zmk("Q").to_qmk(), "KC_Q");
        assert_eq!(KeyExpr::parse_zmk("LG(SPACE)").to_qmk(), "LGUI(KC_SPACE)");
        assert_eq!(KeyExpr::parse_zmk("LG(LS(LBKT))").to_qmk(), "LGUI(LSFT(KC_LBRC))");
    }

    #[test]
    fn key_expr_unknown_falls_back_to_raw() {
        assert_eq!(KeyExpr::parse_zmk("WEIRD").to_qmk(), "WEIRD");
    }

    #[test]
    fn mouse_movement_parses_zmk_and_qmk() {
        assert_eq!(MouseMovement::from_zmk("MOVE_UP"), Some(MouseMovement::Up));
        assert_eq!(MouseMovement::from_zmk("MOVE_DOWN"), Some(MouseMovement::Down));
        assert_eq!(MouseMovement::from_zmk("MOVE_LEFT"), Some(MouseMovement::Left));
        assert_eq!(MouseMovement::from_zmk("MOVE_RIGHT"), Some(MouseMovement::Right));
        assert_eq!(MouseMovement::from_zmk("BOGUS"), None);

        assert_eq!(MouseMovement::from_qmk("KC_MS_U"), Some(MouseMovement::Up));
        assert_eq!(MouseMovement::from_qmk("MS_DOWN"), Some(MouseMovement::Down));
        assert_eq!(MouseMovement::from_qmk("KC_MS_L"), Some(MouseMovement::Left));
        assert_eq!(MouseMovement::from_qmk("MS_RIGHT"), Some(MouseMovement::Right));
        assert_eq!(MouseMovement::from_qmk("KC_A"), None);
    }

    #[test]
    fn mouse_movement_renders_to_both_domains() {
        assert_eq!(MouseMovement::Up.to_string(), "MOVE_UP");
        assert_eq!(MouseMovement::Down.qmk_name(), "KC_MS_D");
        assert_eq!(MouseMovement::Left.zmk_name(), "MOVE_LEFT");
        assert_eq!(MouseMovement::Right.qmk_name(), "KC_MS_R");
    }

    #[test]
    fn mouse_movement_from_str_tries_zmk_then_qmk_then_unknown() {
        assert_eq!(MouseMovement::from("MOVE_UP"), MouseMovement::Up);
        assert_eq!(MouseMovement::from("KC_MS_D"), MouseMovement::Down);
        assert!(matches!(MouseMovement::from("BOGUS"), MouseMovement::Unknown(_)));
        assert_eq!(MouseMovement::from(String::from("MOVE_LEFT")), MouseMovement::Left);
    }

    #[test]
    fn mouse_movement_partial_eq_str() {
        assert!(MouseMovement::Up == "MOVE_UP");
        assert!(MouseMovement::Down == *"MOVE_DOWN");
    }

    #[test]
    fn mouse_button_parses_zmk_and_qmk() {
        assert_eq!(MouseButton::from_zmk("LCLK"), Some(MouseButton::Left));
        assert_eq!(MouseButton::from_zmk("RCLK"), Some(MouseButton::Right));
        assert_eq!(MouseButton::from_zmk("MCLK"), Some(MouseButton::Middle));
        assert_eq!(MouseButton::from_zmk("BTN4"), Some(MouseButton::Button4));
        assert_eq!(MouseButton::from_zmk("BTN5"), Some(MouseButton::Button5));
        assert_eq!(MouseButton::from_zmk("BOGUS"), None);

        assert_eq!(MouseButton::from_qmk("KC_BTN1"), Some(MouseButton::Left));
        assert_eq!(MouseButton::from_qmk("BTN2"), Some(MouseButton::Right));
        assert_eq!(MouseButton::from_qmk("KC_BTN3"), Some(MouseButton::Middle));
        assert_eq!(MouseButton::from_qmk("KC_A"), None);
    }

    #[test]
    fn mouse_button_renders_to_both_domains() {
        assert_eq!(MouseButton::Left.to_string(), "LCLK");
        assert_eq!(MouseButton::Right.qmk_name(), "KC_BTN2");
        assert_eq!(MouseButton::Middle.zmk_name(), "MCLK");
        assert_eq!(MouseButton::Button4.qmk_name(), "KC_BTN4");
        assert_eq!(MouseButton::Button5.zmk_name(), "BTN5");
    }

    #[test]
    fn mouse_button_from_str_tries_zmk_then_qmk_then_unknown() {
        assert_eq!(MouseButton::from("LCLK"), MouseButton::Left);
        assert_eq!(MouseButton::from("KC_BTN2"), MouseButton::Right);
        assert!(matches!(MouseButton::from("BOGUS"), MouseButton::Unknown(_)));
        assert_eq!(MouseButton::from(String::from("MCLK")), MouseButton::Middle);
    }

    #[test]
    fn mouse_button_partial_eq_str() {
        assert!(MouseButton::Left == "LCLK");
        assert!(MouseButton::Right == *"RCLK");
    }

    #[test]
    fn mouse_scroll_parses_zmk_and_qmk() {
        assert_eq!(MouseScroll::from_zmk("SCRL_UP"), Some(MouseScroll::Up));
        assert_eq!(MouseScroll::from_zmk("SCRL_DOWN"), Some(MouseScroll::Down));
        assert_eq!(MouseScroll::from_zmk("SCRL_LEFT"), Some(MouseScroll::Left));
        assert_eq!(MouseScroll::from_zmk("SCRL_RIGHT"), Some(MouseScroll::Right));
        assert_eq!(MouseScroll::from_zmk("BOGUS"), None);

        assert_eq!(MouseScroll::from_qmk("KC_WH_U"), Some(MouseScroll::Up));
        assert_eq!(MouseScroll::from_qmk("MS_WH_DOWN"), Some(MouseScroll::Down));
        assert_eq!(MouseScroll::from_qmk("WH_L"), Some(MouseScroll::Left));
        assert_eq!(MouseScroll::from_qmk("KC_MS_WH_RIGHT"), Some(MouseScroll::Right));
        assert_eq!(MouseScroll::from_qmk("KC_A"), None);
    }

    #[test]
    fn mouse_scroll_renders_to_both_domains() {
        assert_eq!(MouseScroll::Up.to_string(), "SCRL_UP");
        assert_eq!(MouseScroll::Down.qmk_name(), "KC_WH_D");
        assert_eq!(MouseScroll::Left.zmk_name(), "SCRL_LEFT");
        assert_eq!(MouseScroll::Right.qmk_name(), "KC_WH_R");
    }

    #[test]
    fn mouse_scroll_from_str_tries_zmk_then_qmk_then_unknown() {
        assert_eq!(MouseScroll::from("SCRL_UP"), MouseScroll::Up);
        assert_eq!(MouseScroll::from("KC_WH_D"), MouseScroll::Down);
        assert!(matches!(MouseScroll::from("BOGUS"), MouseScroll::Unknown(_)));
        assert_eq!(MouseScroll::from(String::from("SCRL_LEFT")), MouseScroll::Left);
    }

    #[test]
    fn mouse_scroll_partial_eq_str() {
        assert!(MouseScroll::Up == "SCRL_UP");
        assert!(MouseScroll::Down == *"SCRL_DOWN");
    }

    #[test]
    fn modifier_from_str_tries_zmk_then_qmk_then_unknown() {
        assert_eq!(Modifier::from("LCTRL"), Modifier::LCtrl);
        assert_eq!(Modifier::from("MOD_LSFT"), Modifier::LShft);
        assert!(matches!(Modifier::from("BOGUS"), Modifier::Unknown(_)));
        assert_eq!(Modifier::from(String::from("LGUI")), Modifier::LGui);
    }

    #[test]
    fn modifier_partial_eq_str() {
        assert!(Modifier::LAlt == "LALT");
        assert!(Modifier::LCtrl == *"LCTRL");
    }

    #[test]
    fn rgb_action_from_str_tries_zmk_then_qmk_then_unknown() {
        assert_eq!(RgbAction::from("RGB_TOG"), RgbAction::Toggle);
        assert_eq!(RgbAction::from("RGB_MOD"), RgbAction::EffectNext);
        assert!(matches!(RgbAction::from("BOGUS"), RgbAction::Unknown(_)));
        assert_eq!(RgbAction::from(String::from("RGB_EFR")), RgbAction::EffectPrev);
    }

    #[test]
    fn rgb_action_partial_eq_str() {
        assert!(RgbAction::Toggle == "RGB_TOG");
        assert!(RgbAction::EffectNext == *"RGB_EFF");
    }

    #[test]
    fn keyboard_cols_matches_known_boards() {
        assert_eq!(keyboard_cols("planck/ez/glow"), Some(12));
        assert_eq!(keyboard_cols("LAYOUT_crkbd_base"), Some(6));
        assert_eq!(keyboard_cols("kyria_rev3"), Some(7));
        assert_eq!(keyboard_cols("unknown_board"), None);
        assert_eq!(keyboard_cols("PLANCK"), Some(12));
    }

    #[test]
    fn key_expr_partial_eq_str() {
        let key = KeyExpr::parse_zmk("A");
        assert!(key == "A");
        assert!(key == *"A");

        let raw = KeyExpr::Raw("CUSTOM".into());
        assert!(raw == "CUSTOM");

        let modified = KeyExpr::parse_zmk("LG(A)");
        assert!(modified == "LG(A)");
    }
}
