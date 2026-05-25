# qmk2zmk

Convert between QMK and ZMK keymap formats.

Two CLI tools ship in this package:

- **`qmk2zmk`** — reads a QMK `keymap.c` or `keymap.json` and emits a ZMK `.keymap` DTS overlay
- **`zmk2qmk`** — reads a ZMK `.keymap` and emits QMK Configurator JSON or a `keymap.c`

## Installation

**macOS / Linux**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/patrickomatic/qmk2zmk/releases/latest/download/qmk2zmk-installer.sh | sh
```

**Windows (PowerShell)**

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/patrickomatic/qmk2zmk/releases/latest/download/qmk2zmk-installer.ps1 | iex"
```

Both binaries are installed by the same script.

**Via cargo**

```sh
cargo install qmk2zmk
```

## Usage

### qmk2zmk

```
qmk2zmk [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file (keymap.c or keymap.json)

Options:
  -f, --format <FORMAT>  Input format: c or json (auto-detected from extension)
  -o, --output <OUTPUT>  Output file (defaults to stdout)
  -h, --help             Print help
```

```sh
# Print to stdout
qmk2zmk keymap.c

# Write to a file
qmk2zmk keymap.c -o my_keymap.keymap

# Explicit format flag
qmk2zmk -f json keymap.json -o my_keymap.keymap
```

### zmk2qmk

```
zmk2qmk [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file (.keymap DTS overlay)

Options:
  -f, --format <FORMAT>  Output format: json or c [default: json]
      --layout <LAYOUT>  QMK LAYOUT macro name used in C output [default: LAYOUT]
  -o, --output <OUTPUT>  Output file (defaults to stdout)
  -h, --help             Print help
```

```sh
# Convert to QMK Configurator JSON
zmk2qmk my_keymap.keymap

# Convert to keymap.c
zmk2qmk my_keymap.keymap -f c -o keymap.c

# Specify the layout macro for C output
zmk2qmk my_keymap.keymap -f c --layout LAYOUT_planck_grid -o keymap.c
```

## What gets converted

### QMK → ZMK

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

### ZMK → QMK

| ZMK | QMK |
|-----|-----|
| `&kp A` … `&kp Z` | `KC_A` … `KC_Z` |
| `&kp N1` … `&kp N0` | `KC_1` … `KC_0` |
| `&trans` | `KC_TRNS` |
| `&none` | `KC_NO` |
| `&mt LALT Z` | `MT(MOD_LALT,KC_Z)` |
| `&lt 1 SPACE` | `LT(1,KC_SPACE)` |
| `&mo 1` | `MO(1)` |
| `&tog 1` | `TG(1)` |
| `&kp LG(LS(LBKT))` | `LGUI(LSFT(KC_LBRC))` |
| `&caps_word` | `CW_TOGG` |
| `&bootloader` | `QK_BOOT` |
| `&sys_reset` | `QK_RBT` |
| `&rgb_ug RGB_TOG`, … | `RGB_TOG`, … |
| `conditional_layers` block | `update_tri_layer_state` (C only) |

Punctuation and special keys are remapped where the names differ between firmwares (e.g. `SEMI` ↔ `KC_SCLN`, `LBKT` ↔ `KC_LBRC`, `BSLH` ↔ `KC_BSLS`).

## Known gaps

- **Custom macro bodies** — `process_record_user` is not parsed by `qmk2zmk`. Macros get a stub in the `macros {}` block for you to fill in. `zmk2qmk` generates a `process_record_user` stub in C output.
- **Dynamic tapping term keys** (`QK_DYNAMIC_TAPPING_TERM_*`) — no ZMK equivalent; emitted as `/* TODO */` comments.
- **ZSA-specific features** — RGB lighting config, `rawhid_state`, and LED maps are not translated.
- **Target board/shield** — output is board-agnostic. You still need to wire it into your ZMK or QMK config with the correct board files.

## Example

The `examples/zsa-qmk/` directory contains a real ZSA Planck EZ Glow export (Colemak-DH with home-row mods) used as the reference test case.
