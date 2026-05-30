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
  ir.rs           — Neutral semantic model (Keyboard, Layer, Key, …)
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

The preferred release path is `make release`. It runs tests and clippy, commits
and pushes the version bump, creates and pushes a `vX.Y.Z` git tag, triggers the
GitHub Release workflow, and publishes the crate to crates.io.

When asked to bump and release manually, treat the release as incomplete until
all of these are done and verified:

1. Bump `version` in `Cargo.toml` and update `Cargo.lock`.
2. Run `cargo test`.
3. Run `cargo clippy --all-targets -- -W clippy::pedantic -D warnings`.
4. Commit the version bump and push `main`.
5. Create and push the matching `vX.Y.Z` git tag.
6. Watch the GitHub Actions `Release` workflow for that tag and confirm it
   completes successfully.
7. Run `cargo publish` for the same version.
8. Verify crates.io reports the new version, for example with
   `cargo search qmk2zmk --limit 5` or `cargo info qmk2zmk`.
9. Report the pushed commit, pushed tag, GitHub Release workflow status, and
   crates.io version.

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
