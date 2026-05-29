//! Library support for the `qmk2zmk` and `zmk2qmk` binaries.
//!
//! The crate is organized around a small internal representation in [`ir`].
//! Parsers in [`qmk`] and [`zmk`] produce that representation, and renderers in
//! the opposite module turn it back into the requested output format.

pub mod codes;
pub mod error;
pub mod io;
pub mod ir;
pub mod qmk;
pub mod zmk;

/// Print a stderr warning for every `Key::Unknown` in the parsed keymap.
pub fn warn_unknowns(keymap: &ir::Keymap) {
    for layer in &keymap.layers {
        for key in &layer.keys {
            if let ir::Key::Unknown(s) = key {
                eprintln!("warning: unmapped key in layer '{}': {s}", layer.name);
            }
        }
    }
}

/// Print an error chain to stderr and terminate the process with exit code 1.
///
/// Binaries call this at the outer edge of the program. Library code should
/// return structured errors instead of printing or exiting directly.
pub fn report_and_exit(e: &dyn std::error::Error) -> ! {
    eprintln!("error: {e}");
    let mut src = e.source();
    while let Some(cause) = src {
        eprintln!("  caused by: {cause}");
        src = cause.source();
    }
    std::process::exit(1);
}
