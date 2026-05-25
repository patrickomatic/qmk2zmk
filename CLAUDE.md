# qmk2zmk

A Rust CLI tool that converts QMK keymap files (C source or JSON) to ZMK `.keymap` format.

## Error handling

**No `.unwrap()`, `.expect()`, or `panic!()` in library code.** All fallible operations must return a `Result` and propagate errors to the caller. The only place errors are caught and printed is `main()` → `run()`.

Use the structured error types in `src/error.rs`:

- `Error` — top-level application errors (I/O, parse dispatch). Caught in `main`.
- `ParseCError` — specific failure modes from the C keymap parser. Each variant carries the context needed to understand the failure without reading source.

When adding a new failure mode, add a variant to the appropriate enum rather than using a generic string error. This keeps error handling exhaustive and testable.

**No third-party error-handling crates.** Implement `std::fmt::Display` and `std::error::Error` manually.

## Project layout

```
src/
  lib.rs          — public module exports; report_and_exit shared by both binaries
  io.rs           — shared read_input / write_output helpers
  error.rs        — Error, ParseCError, ParseZmkError types
  ir.rs           — Internal representation (Keymap, Layer, Key, …)
  codes.rs        — QMK ↔ ZMK key/modifier/RGB mapping tables (both directions)
  qmk/
    mod.rs        — Render QMK JSON and C output
    parse_c.rs    — Parse keymap.c
    parse_json.rs — Parse QMK Configurator JSON
  zmk/
    mod.rs        — Render ZMK .keymap output
    parse.rs      — Parse ZMK .keymap DTS overlay
  bin/
    qmk2zmk.rs   — CLI entry point for qmk2zmk
    zmk2qmk.rs   — CLI entry point for zmk2qmk
tests/
  integration.rs  — End-to-end tests against the example keymap
examples/
  zsa-qmk/        — ZSA Planck EZ source used as the reference example
```

## Linting

All code must pass `cargo clippy --all-targets -- -W clippy::pedantic -D warnings` with no errors. This is enforced by CI. Run it locally before committing.

The one deliberate exception is `#[allow(clippy::implicit_hasher)]` on `parse_key_expr_str` — threading separate hasher type parameters through all private helpers adds noise without value for a CLI tool.

## Testing

Run all tests with `cargo test`. There are three test suites:

- Unit tests inline in each module (`#[cfg(test)]`)
- Integration tests in `tests/integration.rs` that parse the real example `keymap.c`

Add a test for every new parser feature or key mapping. Integration tests should assert on specific keys at known positions in the example keymap.

## Releasing

1. Bump `version` in `Cargo.toml`.
2. Verify everything is green: `cargo test && cargo clippy --all-targets -- -W clippy::pedantic -D warnings`
3. Commit the version bump and push to `main`.
4. Run `make release` — this creates a `vX.Y.Z` git tag and pushes it to origin, which triggers the CI release workflow (`cargo-dist`) to build platform binaries and create a GitHub Release.
5. Run `cargo publish` — publishes the crate to crates.io. This step is not automated by CI and must be run manually.

## Key mapping conventions

QMK and ZMK use different names for the same physical keys. The canonical mapping lives in `src/codes.rs`. Notable differences:

- Numbers: `KC_0`–`KC_9` → `N0`–`N9`
- Bracket keys: `KC_LBRC`/`KC_RBRC` (physical `[`/`]`) → `LBKT`/`RBKT`
- Curly braces: `KC_LCBR`/`KC_RCBR` (shifted `{`/`}`) → `LBRC`/`RBRC`
- Semicolon: `KC_SCLN` → `SEMI`
- Quote: `KC_QUOTE` → `SQT`
- Backslash: `KC_BSLS` → `BSLH`
- Enter: `KC_ENTER` → `RET`
- Media keys: `KC_AUDIO_VOL_UP` → `C_VOL_UP`, etc.
