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

pub fn report_and_exit(e: &dyn std::error::Error) -> ! {
    eprintln!("error: {e}");
    let mut src = e.source();
    while let Some(cause) = src {
        eprintln!("  caused by: {cause}");
        src = cause.source();
    }
    std::process::exit(1);
}
