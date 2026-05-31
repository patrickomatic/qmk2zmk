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
qmk2zmk [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input file (keymap.c or keymap.json)

Options:
  -f, --format <FORMAT>      Input format: c or json (auto-detected from extension)
  -o, --output <OUTPUT>      Output file (defaults to stdout)
      --keyboard <KEYBOARD>  Known keyboard name (sets column count; see --list-keyboards)
      --cols <COLS>          Override columns per row in ZMK output
      --list-keyboards       List known keyboards and their column counts, then exit
      --no-warn              Suppress warnings for unmapped keycodes
  -p, --print-layout         Parse the keymap and print a layout table, then exit
  -h, --help                 Print help
```

```sh
# Print to stdout (warnings for unmapped keys go to stderr automatically)
qmk2zmk keymap.c

# Write to a file
qmk2zmk keymap.c -o my_keymap.keymap

# Suppress unmapped key warnings
qmk2zmk keymap.c --no-warn

# Specify keyboard to set column count in output
qmk2zmk keymap.c --keyboard planck

# Override column count directly
qmk2zmk keymap.c --cols 10

# List known keyboards
qmk2zmk --list-keyboards

# Print a human-readable layout table without converting
qmk2zmk keymap.c --print-layout

# Explicit format flag
qmk2zmk -f json keymap.json -o my_keymap.keymap
```

### zmk2qmk

```
zmk2qmk [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input file (.keymap DTS overlay)

Options:
  -f, --format <FORMAT>      Output format: json or c [default: json]
      --layout <LAYOUT>      QMK LAYOUT macro name used in C output [default: LAYOUT]
  -o, --output <OUTPUT>      Output file (defaults to stdout)
      --keyboard <KEYBOARD>  Known keyboard name (sets column count; see --list-keyboards)
      --cols <COLS>          Override columns per row in QMK C output
      --list-keyboards       List known keyboards and their column counts, then exit
      --no-warn              Suppress warnings for unmapped keycodes
  -p, --print-layout         Parse the keymap and print a layout table, then exit
  -h, --help                 Print help
```

```sh
# Convert to QMK Configurator JSON
zmk2qmk my_keymap.keymap

# Convert to keymap.c
zmk2qmk my_keymap.keymap -f c -o keymap.c

# Specify the layout macro for C output
zmk2qmk my_keymap.keymap -f c --layout LAYOUT_planck_grid -o keymap.c

# Specify keyboard to set column count in output
zmk2qmk my_keymap.keymap -f c --keyboard corne

# List known keyboards
zmk2qmk --list-keyboards

# Print a human-readable layout table without converting
zmk2qmk my_keymap.keymap --print-layout
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
| `TO(_BASE)` | `&to 0` |
| `DF(_BASE)` | `&to 0` |
| `OSM(MOD_LSFT)` | `&sk LSHFT` |
| `OSL(1)` | `&sl 1` |
| `LGUI(LSFT(KC_LBRC))` | `&kp LG(LS(LBKT))` |
| `HYPR(KC_A)` | `&kp LG(LA(LS(LC(A))))` |
| `MEH(KC_A)` | `&kp LA(LS(LC(A)))` |
| `KC_MS_U/D/L/R` | `&mmv MOVE_UP/DOWN/LEFT/RIGHT` |
| `KC_BTN1/2/3` | `&mkp LCLK/RCLK/MCLK` |
| `KC_WH_U/D/L/R` | `&msc SCRL_UP/DOWN/LEFT/RIGHT` |
| `KC_UNDO`, `KC_COPY`, `KC_PASTE`, … | `&kp K_UNDO`, `&kp K_COPY`, … |
| `KC_NUBS`, `KC_NUHS` | `&kp NON_US_BSLH`, `&kp NON_US_HASH` |
| `CW_TOGG` | `&caps_word` |
| `QK_BOOT` | `&bootloader` |
| `RGB_TOG`, `RGB_HUI`, `RGB_SPI`, … | `&rgb_ug RGB_TOG`, … |
| `#define LOWER MO(_LOWER)` | resolved automatically |
| `update_tri_layer_state` | `conditional_layers` block |
| Custom macros (`ST_MACRO_0`, …) | stub with `// TODO` |
| `TD(DANCE_0)` with `ACTION_TAP_DANCE_DOUBLE` | `zmk,behavior-tap-dance` in `behaviors {}` |
| `LM(...)` | `/* TODO */` comment |

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
| `&to 0` | `DF(0)` |
| `&sk LSHFT` | `OSM(MOD_LSFT)` |
| `&sl 1` | `OSL(1)` |
| `&kp LG(LS(LBKT))` | `LGUI(LSFT(KC_LBRC))` |
| `&mmv MOVE_UP/DOWN/LEFT/RIGHT` | `KC_MS_U/D/L/R` |
| `&mkp LCLK/RCLK/MCLK` | `KC_BTN1/2/3` |
| `&msc SCRL_UP/DOWN/LEFT/RIGHT` | `KC_WH_U/D/L/R` |
| `&caps_word` | `CW_TOGG` |
| `&bootloader` | `QK_BOOT` |
| `&sys_reset` | `QK_RBT` |
| `&rgb_ug RGB_TOG`, … | `RGB_TOG`, … |
| `zmk,behavior-tap-dance` | `TD(DANCE_N)` with `tap_dance_actions[]` |
| `&bt BT_SEL …`, `&out OUT_USB` | `/* TODO */` comment |
| `conditional_layers` block | `update_tri_layer_state` (C only) |

Punctuation and special keys are remapped where the names differ between firmwares (e.g. `SEMI` ↔ `KC_SCLN`, `LBKT` ↔ `KC_LBRC`, `BSLH` ↔ `KC_BSLS`).

## Known gaps

- **Custom macro bodies** — `process_record_user` is not parsed by `qmk2zmk`. Macros get a stub in the `macros {}` block for you to fill in. `zmk2qmk` generates a `process_record_user` stub in C output.
- **Tap dance with more than 2 bindings** — `ACTION_TAP_DANCE_DOUBLE` is fully converted; other action types (`ACTION_TAP_DANCE_FN_ADVANCED`, etc.) produce a stub for you to fill in.
- **Layer-mod** (`LM(...)`) — no ZMK equivalent; emitted as a `/* TODO */` comment.
- **Bluetooth / output keys** (`&bt`, `&out`) — no QMK equivalent; preserved as `/* TODO */` in QMK output.
- **Dynamic tapping term keys** (`QK_DYNAMIC_TAPPING_TERM_*`) — no ZMK equivalent; emitted as `/* TODO */` comments.
- **ZSA-specific features** — RGB lighting config, `rawhid_state`, and LED maps are not translated.
- **Target board/shield** — output is board-agnostic. You still need to wire it into your ZMK or QMK config with the correct board files.

## Example

The `examples/zsa-qmk/` directory contains a real ZSA Planck EZ Glow export (Colemak-DH with home-row mods) used as the reference test case.
