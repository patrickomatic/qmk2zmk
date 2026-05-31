//! Parser for QMK Configurator JSON keymaps.
//!
//! QMK JSON is already flattened into layer arrays, so this module mostly
//! deserializes metadata and delegates individual key strings to the C key
//! expression parser. Reusing that key parser keeps JSON and C inputs aligned
//! for aliases like `MO(1)`, `KC_TRNS`, and modifier-tap expressions.

use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::ir::{Keyboard, Layer};

#[derive(Deserialize)]
struct QmkJson {
    /// Optional QMK keyboard identifier from Configurator exports.
    keyboard: Option<String>,
    /// Optional QMK layout macro name from Configurator exports.
    layout: Option<String>,
    /// Flattened layer rows, where each string is a QMK key expression.
    layers: Vec<Vec<String>>,
}

/// Parse a QMK Configurator JSON keymap into the shared IR.
///
/// Layer names are synthesized as `LAYER_0`, `LAYER_1`, and so on because the
/// Configurator format stores layers by position rather than by symbolic name.
///
/// # Errors
/// Returns a [`serde_json::Error`] if the source is not valid JSON or does
/// not match the QMK Configurator keymap format.
pub fn parse(source: &str) -> Result<Keyboard, serde_json::Error> {
    let qmk: QmkJson = serde_json::from_str(source)?;
    Ok(qmk.into())
}

impl From<QmkJson> for Keyboard {
    fn from(qmk: QmkJson) -> Self {
        let layers = qmk
            .layers
            .into_iter()
            .enumerate()
            .map(|(index, keys)| QmkJsonLayer { index, keys }.into())
            .collect();

        Keyboard {
            keyboard: qmk.keyboard,
            layout: qmk.layout,
            layers,
            macros: vec![],
            tap_dances: vec![],
            tri_layer: None,
        }
    }
}

struct QmkJsonLayer {
    /// Zero-based layer position from the JSON `layers` array.
    index: usize,
    /// Raw QMK key strings stored in physical layout order.
    keys: Vec<String>,
}

impl From<QmkJsonLayer> for Layer {
    fn from(layer: QmkJsonLayer) -> Self {
        let tap_dance_map = HashMap::new();
        let layer_map = HashMap::new();
        let defines = HashMap::new();
        let custom_keycodes = HashSet::new();

        Layer {
            name: format!("LAYER_{}", layer.index),
            index: layer.index,
            keys: layer
                .keys
                .iter()
                .map(|k| {
                    super::parse_c::parse_key_expr_str(
                        k,
                        &layer_map,
                        &defines,
                        &custom_keycodes,
                        &tap_dance_map,
                    )
                })
                .collect(),
            sensor_bindings: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Key;

    #[test]
    fn converts_qmk_json_to_keymap() {
        let source = r#"{
            "keyboard": "planck/ez",
            "layout": "LAYOUT_planck_grid",
            "layers": [["KC_A", "MO(1)"], ["KC_TRNS", "KC_NO"]]
        }"#;

        let keyboard = parse(source).unwrap();

        assert_eq!(keyboard.keyboard.as_deref(), Some("planck/ez"));
        assert_eq!(keyboard.layout.as_deref(), Some("LAYOUT_planck_grid"));
        assert_eq!(keyboard.layers[0].name, "LAYER_0");
        assert_eq!(keyboard.layers[1].index, 1);
        assert!(matches!(&keyboard.layers[0].keys[0], Key::Kp(k) if k == "A"));
        assert!(matches!(&keyboard.layers[0].keys[1], Key::Mo(1)));
        assert!(matches!(&keyboard.layers[1].keys[0], Key::Trans));
        assert!(matches!(&keyboard.layers[1].keys[1], Key::None));
    }

    #[test]
    fn invalid_json_returns_error() {
        assert!(parse("not json at all").is_err());
    }

    #[test]
    fn missing_layers_field_returns_error() {
        assert!(parse(r#"{"keyboard": "planck"}"#).is_err());
    }

    #[test]
    fn empty_layers_array_produces_empty_keyboard() {
        let source = r#"{"layers": []}"#;
        let keyboard = parse(source).unwrap();
        assert!(keyboard.layers.is_empty());
        assert!(keyboard.keyboard.is_none());
        assert!(keyboard.layout.is_none());
    }

    #[test]
    fn layer_names_synthesized_by_index() {
        let source = r#"{"layers": [["KC_A"], ["KC_B"], ["KC_C"]]}"#;
        let keyboard = parse(source).unwrap();
        assert_eq!(keyboard.layers[0].name, "LAYER_0");
        assert_eq!(keyboard.layers[1].name, "LAYER_1");
        assert_eq!(keyboard.layers[2].name, "LAYER_2");
    }
}
