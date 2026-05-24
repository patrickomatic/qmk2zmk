use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::ir::*;

#[derive(Deserialize)]
struct QmkJson {
    keyboard: Option<String>,
    layout: Option<String>,
    layers: Vec<Vec<String>>,
}

pub fn parse(source: &str) -> Result<Keymap, serde_json::Error> {
    let qmk: QmkJson = serde_json::from_str(source)?;

    let layers = qmk
        .layers
        .iter()
        .enumerate()
        .map(|(i, keys)| {
            let layer_map = HashMap::new();
            let defines = HashMap::new();
            let custom_keycodes = HashSet::new();
            Layer {
                name: format!("LAYER_{}", i),
                index: i,
                keys: keys
                    .iter()
                    .map(|k| super::qmk_c::parse_key_expr_str(k, &layer_map, &defines, &custom_keycodes))
                    .collect(),
            }
        })
        .collect();

    Ok(Keymap {
        keyboard: qmk.keyboard,
        layout: qmk.layout,
        layers,
        macros: vec![],
        tri_layer: None,
    })
}
