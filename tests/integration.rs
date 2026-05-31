use qmk2zmk::ir::Key;
use qmk2zmk::qmk::parse_c as qmk_c;
use qmk2zmk::zmk;

const KEYMAP_C: &str =
    include_str!("../examples/zsa-qmk/zsa_planck_ez_glow_planck_source/keymap.c");

const ZMK_KEYMAP: &str = include_str!("../examples/zmk/keymap.zmk");

#[test]
fn parses_four_layers() {
    let km = qmk2zmk::qmk::parse_c::parse(KEYMAP_C).unwrap();
    assert_eq!(km.layers.len(), 4);
}

#[test]
fn layers_sorted_by_index() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    for (i, layer) in km.layers.iter().enumerate() {
        assert_eq!(layer.index, i, "layer {i} out of order");
    }
}

#[test]
fn planck_has_48_keys_per_layer() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    for layer in &km.layers {
        assert_eq!(
            layer.keys.len(),
            48,
            "layer {} has wrong key count",
            layer.name
        );
    }
}

#[test]
fn base_layer_first_key_is_tab() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let base = km.layers.iter().find(|l| l.name == "_BASE").unwrap();
    assert!(matches!(&base.keys[0], Key::Kp(k) if k == "TAB"));
}

#[test]
fn base_layer_has_home_row_mods() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let base = km.layers.iter().find(|l| l.name == "_BASE").unwrap();
    // Row 2, col 1: MT(MOD_LALT, KC_Z)
    assert!(matches!(&base.keys[25], Key::Mt(m, k) if m == "LALT" && k == "Z"));
    // Row 2, col 2: MT(MOD_LCTL, KC_X)
    assert!(matches!(&base.keys[26], Key::Mt(m, k) if m == "LCTRL" && k == "X"));
}

#[test]
fn base_layer_has_caps_word() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let base = km.layers.iter().find(|l| l.name == "_BASE").unwrap();
    // Row 1, col 0: CW_TOGG
    assert!(matches!(&base.keys[12], Key::CapsWord));
}

#[test]
fn base_layer_lower_raise_resolve_to_mo() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let base = km.layers.iter().find(|l| l.name == "_BASE").unwrap();
    // Bottom row: LOWER at col 4, RAISE at col 7
    assert!(
        matches!(&base.keys[40], Key::Mo(1)),
        "LOWER should be &mo 1"
    );
    assert!(
        matches!(&base.keys[43], Key::Mo(2)),
        "RAISE should be &mo 2"
    );
}

#[test]
fn base_layer_macro_detected() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let base = km.layers.iter().find(|l| l.name == "_BASE").unwrap();
    // Row 2, col 0: ST_MACRO_0
    assert!(matches!(&base.keys[24], Key::Macro(n) if n == "ST_MACRO_0"));
}

#[test]
fn lower_layer_has_symbols() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let lower = km.layers.iter().find(|l| l.name == "_LOWER").unwrap();
    assert!(matches!(&lower.keys[1], Key::Kp(k) if k == "EXCL"));
    assert!(matches!(&lower.keys[2], Key::Kp(k) if k == "AT"));
    assert!(matches!(&lower.keys[3], Key::Kp(k) if k == "HASH"));
}

#[test]
fn lower_layer_has_gui_shifted_brackets() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let lower = km.layers.iter().find(|l| l.name == "_LOWER").unwrap();
    // LGUI(LSFT(KC_LBRC)) and RGUI(RSFT(KC_RBRC))
    assert!(matches!(&lower.keys[44], Key::Kp(k) if k == "LG(LS(LBKT))"));
    assert!(matches!(&lower.keys[47], Key::Kp(k) if k == "RG(RS(RBKT))"));
}

#[test]
fn raise_layer_has_numbers() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let raise = km.layers.iter().find(|l| l.name == "_RAISE").unwrap();
    assert!(matches!(&raise.keys[1], Key::Kp(k) if k == "N1"));
    assert!(matches!(&raise.keys[9], Key::Kp(k) if k == "N9"));
    assert!(matches!(&raise.keys[10], Key::Kp(k) if k == "N0"));
}

#[test]
fn adjust_layer_has_bootloader() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let adjust = km.layers.iter().find(|l| l.name == "_ADJUST").unwrap();
    assert!(matches!(&adjust.keys[0], Key::Bootloader));
}

#[test]
fn adjust_layer_has_rgb() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let adjust = km.layers.iter().find(|l| l.name == "_ADJUST").unwrap();
    assert!(matches!(&adjust.keys[44], Key::RgbUg(a) if a == "RGB_TOG"));
    assert!(matches!(&adjust.keys[47], Key::RgbUg(a) if a == "RGB_EFF"));
}

#[test]
fn tri_layer_detected() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let tri = km.tri_layer.expect("tri-layer should be detected");
    assert_eq!(tri.lower, 1);
    assert_eq!(tri.upper, 2);
    assert_eq!(tri.tri, 3);
}

#[test]
fn output_is_valid_zmk_skeleton() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let out = zmk::render(&km, None);
    assert!(out.contains("#include <behaviors.dtsi>"));
    assert!(out.contains("#include <dt-bindings/zmk/keys.h>"));
    assert!(out.contains("#include <dt-bindings/zmk/rgb.h>"));
    assert!(out.contains("zmk,keymap"));
    assert!(out.contains("conditional_layers"));
    assert!(out.contains("if-layers = <1 2>"));
    assert!(out.contains("then-layer = <3>"));
    assert!(out.contains("base_layer"));
    assert!(out.contains("lower_layer"));
    assert!(out.contains("raise_layer"));
    assert!(out.contains("adjust_layer"));
    // Macro stub present
    assert!(out.contains("ST_MACRO_0: ST_MACRO_0"));
}

// ── ZMK example: parsing ──────────────────────────────────────────────────────

#[test]
fn zmk_parses_four_layers() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    assert_eq!(km.layers.len(), 4);
}

#[test]
fn zmk_layers_named_correctly() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    assert_eq!(km.layers[0].name, "base_layer");
    assert_eq!(km.layers[1].name, "lower_layer");
    assert_eq!(km.layers[2].name, "raise_layer");
    assert_eq!(km.layers[3].name, "adjust_layer");
}

#[test]
fn zmk_has_48_keys_per_layer() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    for layer in &km.layers {
        assert_eq!(
            layer.keys.len(),
            48,
            "layer {} has wrong key count",
            layer.name
        );
    }
}

#[test]
fn zmk_base_layer_first_key_is_tab() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let base = &km.layers[0];
    assert!(matches!(&base.keys[0], Key::Kp(k) if k == "TAB"));
}

#[test]
fn zmk_base_layer_has_caps_word() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let base = &km.layers[0];
    // Row 1, col 0
    assert!(matches!(&base.keys[12], Key::CapsWord));
}

#[test]
fn zmk_base_layer_has_home_row_mods() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let base = &km.layers[0];
    // Row 2: mt LALT Z, mt LCTRL X
    assert!(matches!(&base.keys[25], Key::Mt(m, k) if m == "LALT" && k == "Z"));
    assert!(matches!(&base.keys[26], Key::Mt(m, k) if m == "LCTRL" && k == "X"));
}

#[test]
fn zmk_base_layer_has_mo_keys() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let base = &km.layers[0];
    assert!(
        matches!(&base.keys[40], Key::Mo(1)),
        "col 4 of bottom row should be mo 1"
    );
    assert!(
        matches!(&base.keys[43], Key::Mo(2)),
        "col 7 of bottom row should be mo 2"
    );
}

#[test]
fn zmk_lower_layer_has_symbols() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let lower = &km.layers[1];
    assert!(matches!(&lower.keys[0], Key::Kp(k) if k == "TILDE"));
    assert!(matches!(&lower.keys[1], Key::Kp(k) if k == "EXCL"));
    assert!(matches!(&lower.keys[2], Key::Kp(k) if k == "AT"));
    assert!(matches!(&lower.keys[3], Key::Kp(k) if k == "HASH"));
}

#[test]
fn zmk_lower_layer_has_trans_and_nav() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let lower = &km.layers[1];
    assert!(matches!(&lower.keys[12], Key::Trans));
    assert!(matches!(&lower.keys[44], Key::Kp(k) if k == "HOME"));
    assert!(matches!(&lower.keys[47], Key::Kp(k) if k == "END"));
}

#[test]
fn zmk_raise_layer_has_numbers() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let raise = &km.layers[2];
    assert!(matches!(&raise.keys[1], Key::Kp(k) if k == "N1"));
    assert!(matches!(&raise.keys[9], Key::Kp(k) if k == "N9"));
    assert!(matches!(&raise.keys[10], Key::Kp(k) if k == "N0"));
}

#[test]
fn zmk_adjust_layer_has_bootloader_and_reset() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let adjust = &km.layers[3];
    assert!(matches!(&adjust.keys[0], Key::Bootloader));
    assert!(matches!(&adjust.keys[11], Key::SysReset));
}

#[test]
fn zmk_adjust_layer_has_rgb() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let adjust = &km.layers[3];
    assert!(matches!(&adjust.keys[44], Key::RgbUg(a) if a == "RGB_TOG"));
    assert!(matches!(&adjust.keys[47], Key::RgbUg(a) if a == "RGB_EFF"));
}

#[test]
fn zmk_tri_layer_detected() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let tri = km.tri_layer.expect("tri-layer should be detected");
    assert_eq!(tri.lower, 1);
    assert_eq!(tri.upper, 2);
    assert_eq!(tri.tri, 3);
}

// ── ZMK example: rendering to QMK ────────────────────────────────────────────

#[test]
fn zmk_renders_to_valid_qmk_c() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let out = qmk2zmk::qmk::render_c(&km, None);
    assert!(out.contains("#include QMK_KEYBOARD_H"));
    assert!(out.contains("keymaps[][MATRIX_ROWS][MATRIX_COLS]"));
    assert!(out.contains("_BASE"));
    assert!(out.contains("_LOWER"));
    assert!(out.contains("_RAISE"));
    assert!(out.contains("_ADJUST"));
    assert!(out.contains("KC_TAB"));
    assert!(out.contains("CW_TOGG"));
    assert!(out.contains("MO(1)"));
    assert!(out.contains("MO(2)"));
}

#[test]
fn zmk_home_row_mods_survive_to_qmk() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let out = qmk2zmk::qmk::render_c(&km, None);
    assert!(out.contains("MT(MOD_LALT,KC_Z)"));
    assert!(out.contains("MT(MOD_LCTL,KC_X)"));
}

#[test]
fn zmk_rgb_survives_to_qmk() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let out = qmk2zmk::qmk::render_c(&km, None);
    assert!(out.contains("RGB_TOG"));
}

// ── print_layout ─────────────────────────────────────────────────────────────

#[test]
fn print_layout_contains_layer_headers_and_keys() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let mut buf = Vec::new();
    qmk2zmk::print_layout_to(&km, Some(12), &mut buf);
    let out = String::from_utf8(buf).unwrap();

    assert!(out.contains("Layer 0: _BASE"));
    assert!(out.contains("Layer 1: _LOWER"));
    assert!(out.contains("Layer 2: _RAISE"));
    assert!(out.contains("Layer 3: _ADJUST"));

    assert!(out.contains("TAB"));
    assert!(out.contains("MO(1)"));
    assert!(out.contains("MO(2)"));
    assert!(out.contains("_____"));
    assert!(out.contains("XXXXX"));
}

#[test]
fn print_layout_respects_col_count() {
    let km = qmk_c::parse(KEYMAP_C).unwrap();
    let mut buf = Vec::new();
    qmk2zmk::print_layout_to(&km, Some(6), &mut buf);
    let out = String::from_utf8(buf).unwrap();

    // 48 keys at 6 cols = 8 rows per layer; each row ends with a newline before
    // the next row starts, so we can verify there are more rows than at 12-wide.
    let mut buf12 = Vec::new();
    qmk2zmk::print_layout_to(&km, Some(12), &mut buf12);
    let out12 = String::from_utf8(buf12).unwrap();
    assert!(
        out.lines().count() > out12.lines().count(),
        "6-col layout should have more lines than 12-col"
    );
}

// ── Round-trip tests ──────────────────────────────────────────────────────────

/// ZMK → render ZMK → parse → render ZMK again: the two renders must be equal.
/// This verifies our ZMK renderer is stable (idempotent after the first parse).
#[test]
fn zmk_to_zmk_round_trip() {
    let km = zmk::parse::parse(ZMK_KEYMAP).unwrap();
    let pass1 = zmk::render(&km, None);
    let km2 = zmk::parse::parse(&pass1).unwrap();
    let pass2 = zmk::render(&km2, None);
    assert_eq!(pass1, pass2, "ZMK render must be stable after re-parse");
}

/// QMK C (no unknown keys) → render ZMK → re-parse → render again: the two ZMK
/// renders must be equal.  Uses an inline fixture so there are no Unknown keys
/// that would become ZMK block comments and disappear on re-parse.
#[test]
fn qmk_c_to_zmk_to_zmk_round_trip() {
    const SIMPLE_QMK_C: &str = r"
enum layers { _BASE, _FN };
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
    [_BASE] = LAYOUT(
        KC_TAB,  KC_Q,    KC_W,    KC_E,    KC_R,    KC_T,
        CW_TOGG, KC_A,    KC_S,    KC_D,    KC_F,    KC_G,
        KC_LSFT, MT(MOD_LALT,KC_Z), MT(MOD_LCTL,KC_X), KC_C, KC_V, KC_B,
        KC_LCTL, KC_LGUI, KC_LALT, KC_LCTL, MO(1),   KC_SPC
    ),
    [_FN] = LAYOUT(
        KC_TRNS, KC_1,    KC_2,    KC_3,    KC_4,    KC_5,
        KC_TRNS, KC_NO,   KC_NO,   KC_NO,   KC_NO,   KC_NO,
        KC_TRNS, KC_NO,   KC_NO,   KC_NO,   KC_NO,   KC_NO,
        KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS, KC_TRNS
    ),
};
";
    let km = qmk_c::parse(SIMPLE_QMK_C).unwrap();
    let pass1 = zmk::render(&km, None);
    let km2 = zmk::parse::parse(&pass1).unwrap();
    let pass2 = zmk::render(&km2, None);
    assert_eq!(
        pass1, pass2,
        "ZMK render from QMK source must stabilize after one re-parse"
    );
}

/// QMK C → ZMK → parse: key bindings must be preserved for every layer that
/// contains no Unknown keys.  Unknown keys render as ZMK block comments which
/// the re-parser strips, so layers with unknowns are explicitly skipped here.
#[test]
fn qmk_c_to_zmk_preserves_all_keys() {
    let km1 = qmk_c::parse(KEYMAP_C).unwrap();
    let zmk_str = zmk::render(&km1, None);
    let km2 = zmk::parse::parse(&zmk_str).unwrap();

    assert_eq!(
        km1.layers.len(),
        km2.layers.len(),
        "layer count must survive QMK→ZMK"
    );
    for (l1, l2) in km1.layers.iter().zip(km2.layers.iter()) {
        if l1.keys.iter().any(|k| matches!(k, Key::Unknown(_))) {
            continue; // unknown keys become block comments that vanish on re-parse
        }
        assert_eq!(
            l1.keys, l2.keys,
            "layer {} keys must survive QMK→ZMK round-trip",
            l1.name
        );
    }
}
