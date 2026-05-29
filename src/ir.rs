//! Internal representation shared by all parsers and renderers.
//!
//! The project converts between QMK's keymap formats and ZMK's DTS overlay
//! format. This module is the neutral model in the middle: QMK parsers translate
//! QMK keycodes into these types, then the ZMK renderer writes ZMK bindings from
//! them. The reverse path uses the same IR in the opposite direction.
//!
//! Most strings stored in [`Key`] variants use ZMK names rather than QMK names.
//! For example, QMK `KC_1` is stored as `Key::Kp("N1")`, because ZMK renders it
//! as `&kp N1`. This keeps renderers simple and makes the IR line up with ZMK's
//! behavior vocabulary.

/// A complete keyboard keymap in the converter's neutral format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keymap {
    /// Optional QMK keyboard identifier, such as `planck/ez/glow`.
    ///
    /// ZMK overlays generally do not carry this value, so ZMK parsing leaves it
    /// empty. QMK JSON output uses it when present.
    pub keyboard: Option<String>,
    /// Optional QMK layout macro name, such as `LAYOUT_planck_grid`.
    ///
    /// QMK C rendering needs a layout macro. When this is absent, renderers fall
    /// back to a generic `LAYOUT` name.
    pub layout: Option<String>,
    /// Ordered layer definitions. Layer indices in keys refer to positions in
    /// this vector.
    pub layers: Vec<Layer>,
    /// Macro definitions referenced by [`Key::Macro`].
    pub macros: Vec<MacroDef>,
    /// Tap-dance definitions referenced by [`Key::TapDance`].
    pub tap_dances: Vec<TapDanceDef>,
    /// Optional tri-layer behavior, usually QMK's `update_tri_layer(...)`.
    pub tri_layer: Option<TriLayer>,
}

/// One keymap layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    /// Human-readable layer name from the source, such as `_BASE` or `lower`.
    pub name: String,
    /// Numeric layer index used by layer-switching keys.
    pub index: usize,
    /// Keys in physical layout order, already flattened out of source syntax.
    pub keys: Vec<Key>,
}

/// A single binding in a layer, macro, or tap-dance definition.
///
/// Variant names mostly mirror ZMK behavior names:
///
/// - `Kp` renders to ZMK `&kp` and QMK `KC_*`.
/// - `Mo`, `Lt`, `Mt`, `Tog`, `Sk`, `Sl`, `To`, and `Df` are layer or modifier
///   behaviors.
/// - `Mmv`, `Mkp`, and `Msc` are ZMK mouse movement, button, and scroll
///   behaviors.
///
/// Keycode and modifier strings should already be normalized to ZMK spelling.
/// See `src/codes.rs` for the QMK-to-ZMK name tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    /// Normal key press using a ZMK key name, such as `A`, `N1`, `RET`, or
    /// `LG(C)`.
    ///
    /// Examples: QMK `KC_A` and ZMK `&kp A` both become `Kp("A")`; QMK
    /// `LGUI(KC_C)` becomes `Kp("LG(C)")`.
    Kp(String),
    /// Momentarily switch to a layer while held.
    ///
    /// QMK `MO(_LOWER)` and ZMK `&mo 1` become `Mo(1)` when `_LOWER` resolves to
    /// layer index 1.
    Mo(usize),
    /// Layer-tap: hold for a layer, tap for a key.
    ///
    /// QMK `LT(_LOWER, KC_SPACE)` and ZMK `&lt 1 SPACE` become
    /// `Lt(1, "SPACE")`.
    Lt(usize, String),
    /// Mod-tap: hold for a modifier, tap for a key.
    ///
    /// QMK `MT(MOD_LSFT, KC_Z)` and ZMK `&mt LSHFT Z` become
    /// `Mt("LSHFT", "Z")`.
    Mt(String, String),
    /// Toggle a layer on or off.
    ///
    /// QMK `TG(_RAISE)` and ZMK `&tog 2` become `Tog(2)`.
    Tog(usize),
    /// Sticky key / one-shot modifier.
    ///
    /// QMK `OSM(MOD_LSFT)` and ZMK `&sk LSHFT` become `Sk("LSHFT")`.
    Sk(String),
    /// Sticky layer / one-shot layer.
    ///
    /// QMK `OSL(_FN)` and ZMK `&sl 1` become `Sl(1)`.
    Sl(usize),
    /// Move directly to a layer until another layer-selection behavior changes
    /// it.
    ///
    /// QMK `TO(_BASE)` and ZMK `&to 0` become `To(0)`.
    To(usize),
    /// Set the default layer.
    ///
    /// QMK `DF(_QWERTY)` becomes `Df(1)`. ZMK rendering currently emits this as
    /// `&to 1` because that is the closest behavior this converter preserves.
    Df(usize),
    /// Mouse movement direction, such as `MOVE_UP`.
    ///
    /// QMK `KC_MS_U` and ZMK `&mmv MOVE_UP` become `Mmv("MOVE_UP")`.
    Mmv(String),
    /// Mouse button press, such as `LCLK`.
    ///
    /// QMK `KC_BTN1` and ZMK `&mkp LCLK` become `Mkp("LCLK")`.
    Mkp(String),
    /// Mouse scroll direction, such as `SCRL_UP`.
    ///
    /// QMK `KC_WH_U` and ZMK `&msc SCRL_UP` become `Msc("SCRL_UP")`.
    Msc(String),
    /// Transparent binding that falls through to a lower active layer.
    ///
    /// QMK `KC_TRNS`, QMK `_______`, and ZMK `&trans` become `Trans`.
    Trans,
    /// Empty binding that intentionally does nothing.
    ///
    /// QMK `KC_NO`, QMK `XXXXXXX`, and ZMK `&none` become `None`.
    None,
    /// Caps Word behavior.
    ///
    /// QMK `CW_TOGG` and ZMK `&caps_word` become `CapsWord`.
    CapsWord,
    /// Enter the bootloader for flashing firmware.
    ///
    /// QMK `QK_BOOT` and ZMK `&bootloader` become `Bootloader`.
    Bootloader,
    /// Reset the controller.
    ///
    /// QMK `QK_RBT`/`QK_RESET` and ZMK `&sys_reset` become `SysReset`.
    SysReset,
    /// Underglow RGB action name, such as `RGB_TOG` or `RGB_EFF`.
    ///
    /// QMK `RGB_MODE_FORWARD` and ZMK `&rgb_ug RGB_EFF` become
    /// `RgbUg("RGB_EFF")`.
    RgbUg(String),
    /// Reference to a named macro in [`Keymap::macros`].
    Macro(String),
    /// Reference to a tap-dance definition by index in [`Keymap::tap_dances`].
    TapDance(usize),
    /// Source binding that the converter could not model precisely.
    ///
    /// Renderers preserve it as a TODO comment instead of silently dropping it.
    Unknown(String),
}

/// A ZMK-style tap-dance definition.
///
/// QMK named tap dances are also normalized into this shape. The layer binding
/// stores only the index with [`Key::TapDance`], so renderers can choose naming
/// appropriate to the target format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TapDanceDef {
    /// Stable definition name from the source or synthesized during parsing.
    pub name: String,
    /// Ordered bindings invoked by successive taps.
    pub bindings: Vec<Key>,
}

/// A macro definition made of simple tap and wait steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroDef {
    /// Macro behavior/keycode name referenced by [`Key::Macro`].
    pub name: String,
    /// Ordered macro body.
    pub steps: Vec<MacroStep>,
}

/// One step inside a macro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroStep {
    /// Tap a normalized ZMK key name, such as `A` or `RET`.
    Tap(String),
    /// Wait for the given number of milliseconds.
    Wait(u32),
}

/// Layer relationship used to enable a third layer when two other layers are
/// active.
///
/// This usually comes from QMK `update_tri_layer(lower, upper, tri)` and renders
/// to ZMK conditional-layer syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriLayer {
    /// Layer index that must be active.
    pub lower: usize,
    /// Other layer index that must be active.
    pub upper: usize,
    /// Layer index enabled when both [`Self::lower`] and [`Self::upper`] are
    /// active.
    pub tri: usize,
}
