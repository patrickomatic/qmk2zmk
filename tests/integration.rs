use qmk2zmk::ir::Key;
use qmk2zmk::qmk::parse_c as qmk_c;
use qmk2zmk::zmk;

const KEYMAP_C: &str =
    include_str!("../examples/zsa-qmk/zsa_planck_ez_glow_planck_source/keymap.c");

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
        assert_eq!(layer.keys.len(), 48, "layer {} has wrong key count", layer.name);
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
    assert!(matches!(&base.keys[40], Key::Mo(1)), "LOWER should be &mo 1");
    assert!(matches!(&base.keys[43], Key::Mo(2)), "RAISE should be &mo 2");
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
    let out = zmk::render(&km);
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
