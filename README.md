# qmk2zmk

Convert QMK keymap files to ZMK format.

Reads a `keymap.c` (QMK C source) or `keymap.json` (QMK Configurator JSON) and emits a ZMK `.keymap` file ready to drop into a ZMK config repo.

## Installation

```sh
cargo install --path .
```

Or run directly without installing:

```sh
cargo run -- <input>
```

## Usage

```
qmk2zmk [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file (keymap.c or keymap.json)

Options:
  -f, --format <FORMAT>  Input format: c or json (auto-detected from extension)
  -o, --output <OUTPUT>  Output file (defaults to stdout)
  -h, --help             Print help
```

### Examples

```sh
# Print to stdout
qmk2zmk keymap.c

# Write to a file
qmk2zmk keymap.c -o my_keymap.keymap

# Explicit format flag
qmk2zmk -f json keymap.json -o my_keymap.keymap
```

## What gets converted

| QMK | ZMK |
|-----|-----|
| `KC_A` … `KC_Z` | `&kp A` … `&kp Z` |
| `KC_1` … `KC_0` | `&kp N1` … `&kp N0` |
| `KC_TRANSPARENT` / `_______` | `&trans` |
| `KC_NO` / `XXXXXXX` | `&none` |
| `MT(MOD_LALT, KC_Z)` | `&mt LALT Z` |
| `LT(1, KC_SPACE)` | `&lt 1 SPACE` |
| `MO(_LOWER)` | `&mo 1` |
| `TG(_LOWER)` | `&tog 1` |
| `LGUI(LSFT(KC_LBRC))` | `&kp LG(LS(LBKT))` |
| `CW_TOGG` | `&caps_word` |
| `QK_BOOT` | `&bootloader` |
| `RGB_TOG`, `RGB_HUI`, … | `&rgb_ug RGB_TOG`, … |
| `#define LOWER MO(_LOWER)` | resolved automatically |
| `update_tri_layer_state` | `conditional_layers` block |
| Custom macros (`ST_MACRO_0`, …) | stub with `// TODO` |

Punctuation and special keys are remapped to ZMK names where they differ (e.g. `KC_SCLN` → `SEMI`, `KC_LBRC` → `LBKT`, `KC_BSLS` → `BSLH`).

## Known gaps

- **Custom macro bodies** — `process_record_user` is not parsed. Macros referenced in the keymap get a stub entry in the `macros {}` block that you fill in manually.
- **Dynamic tapping term keys** (`QK_DYNAMIC_TAPPING_TERM_*`) — no ZMK equivalent; emitted as `/* TODO */` comments.
- **ZSA-specific features** — RGB lighting config, `rawhid_state`, and LED maps are not translated (ZMK handles RGB differently and may need per-board config).
- **Target board/shield** — the output is board-agnostic. You still need to wire it into your ZMK config with the correct `boards/` and `config/` files for your keyboard.

## Output format

The generated file is a DTS overlay:

```dts
#include <behaviors.dtsi>
#include <dt-bindings/zmk/keys.h>

/ {
    conditional_layers { … };   // if tri-layer was detected
    macros { … };               // if custom macros are referenced
    keymap {
        compatible = "zmk,keymap";
        base_layer  { bindings = < … >; };
        lower_layer { bindings = < … >; };
        …
    };
};
```

## Example

The `examples/zsa-qmk/` directory contains a real ZSA Planck EZ Glow export (Colemak-DH with home-row mods) used as the reference test case:

```sh
cargo run -- examples/zsa-qmk/zsa_planck_ez_glow_planck_source/keymap.c
```
