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

/// Write all layers of `keyboard` as a formatted table, `cols` keys per row.
///
/// If `cols` is `None`, defaults to 10 (a common 40% keyboard row width).
/// Output is written to `out`; I/O errors are silently ignored (same as
/// `println!`).
pub fn print_layout_to(keyboard: &ir::Keyboard, cols: Option<usize>, out: &mut impl std::io::Write) {
    for layer in &keyboard.layers {
        let _ = writeln!(out, "Layer {}: {}", layer.index, layer.name);
        let labels: Vec<String> = layer.keys.iter().map(key_label).collect();
        if labels.is_empty() {
            let _ = writeln!(out);
            continue;
        }
        let col_count = cols.unwrap_or(10).min(labels.len());
        let width = labels.iter().map(String::len).max().unwrap_or(1);
        for row in labels.chunks(col_count) {
            let parts: Vec<String> = row.iter().map(|l| format!("{l:<width$}")).collect();
            let _ = writeln!(out, "  {}", parts.join("  "));
        }
        let _ = writeln!(out);
    }
}

/// Print all layers of `keyboard` as a formatted table to stdout.
pub fn print_layout(keyboard: &ir::Keyboard, cols: Option<usize>) {
    print_layout_to(keyboard, cols, &mut std::io::stdout());
}

fn key_label(key: &ir::Key) -> String {
    match key {
        ir::Key::Kp(e) => e.to_string(),
        ir::Key::Mo(n) => format!("MO({n})"),
        ir::Key::Lt(n, e) => format!("LT({n},{e})"),
        ir::Key::Mt(m, e) => format!("MT({m},{e})"),
        ir::Key::Tog(n) => format!("TG({n})"),
        ir::Key::Sk(m) => format!("SK({m})"),
        ir::Key::Sl(n) => format!("SL({n})"),
        ir::Key::To(n) => format!("TO({n})"),
        ir::Key::Df(n) => format!("DF({n})"),
        ir::Key::Mmv(m) => m.to_string(),
        ir::Key::Mkp(b) => b.to_string(),
        ir::Key::Msc(s) => s.to_string(),
        ir::Key::Trans => "_____".to_string(),
        ir::Key::None => "XXXXX".to_string(),
        ir::Key::CapsWord => "CAPS_WORD".to_string(),
        ir::Key::Bootloader => "BOOT".to_string(),
        ir::Key::SysReset => "RESET".to_string(),
        ir::Key::RgbUg(a) => a.to_string(),
        ir::Key::Macro(name) => format!("M({name})"),
        ir::Key::TapDance(n) => format!("TD({n})"),
        ir::Key::Unknown(s) => format!("?({s})"),
    }
}

/// Print a stderr warning for every `Key::Unknown` in the parsed keyboard.
pub fn warn_unknowns(keyboard: &ir::Keyboard) {
    for layer in &keyboard.layers {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Keyboard, Layer};

    fn kb_with_keys(keys: Vec<ir::Key>) -> Keyboard {
        Keyboard {
            keyboard: None,
            layout: None,
            layers: vec![Layer {
                name: "base".into(),
                index: 0,
                keys,
                sensor_bindings: vec![],
            }],
            macros: vec![],
            tap_dances: vec![],
            tri_layer: None,
        }
    }

    fn layout_str(keys: Vec<ir::Key>) -> String {
        let km = kb_with_keys(keys);
        let mut buf = Vec::new();
        print_layout_to(&km, None, &mut buf);
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn key_label_all_variants() {
        let out = layout_str(vec![
            ir::Key::Lt(1, "SPACE".into()),
            ir::Key::Mt("LSHFT".into(), "Z".into()),
            ir::Key::Tog(2),
            ir::Key::Sk("LCTRL".into()),
            ir::Key::Sl(3),
            ir::Key::To(0),
            ir::Key::Df(1),
            ir::Key::Mmv("MOVE_UP".into()),
            ir::Key::Mkp("LCLK".into()),
            ir::Key::Msc("SCRL_UP".into()),
            ir::Key::CapsWord,
            ir::Key::Bootloader,
            ir::Key::SysReset,
            ir::Key::RgbUg("RGB_TOG".into()),
            ir::Key::Macro("MY_MACRO".into()),
            ir::Key::TapDance(0),
            ir::Key::Unknown("WEIRD".into()),
        ]);
        assert!(out.contains("LT(1,SPACE)"));
        assert!(out.contains("MT(LSHFT,Z)"));
        assert!(out.contains("TG(2)"));
        assert!(out.contains("SK(LCTRL)"));
        assert!(out.contains("SL(3)"));
        assert!(out.contains("TO(0)"));
        assert!(out.contains("DF(1)"));
        assert!(out.contains("MOVE_UP"));
        assert!(out.contains("LCLK"));
        assert!(out.contains("SCRL_UP"));
        assert!(out.contains("CAPS_WORD"));
        assert!(out.contains("BOOT"));
        assert!(out.contains("RESET"));
        assert!(out.contains("RGB_TOG"));
        assert!(out.contains("M(MY_MACRO)"));
        assert!(out.contains("TD(0)"));
        assert!(out.contains("?(WEIRD)"));
    }

    #[test]
    fn print_layout_to_defaults_to_10_cols_when_none() {
        let keys = vec![ir::Key::Trans; 10];
        let km = kb_with_keys(keys);
        let mut buf = Vec::new();
        print_layout_to(&km, None, &mut buf);
        let out = String::from_utf8(buf).unwrap();
        // 10 keys at 10 cols = 1 data row
        assert_eq!(out.lines().filter(|l| l.starts_with("  ")).count(), 1);
    }

    #[test]
    fn print_layout_to_empty_layer_emits_blank_line() {
        let km = kb_with_keys(vec![]);
        let mut buf = Vec::new();
        print_layout_to(&km, None, &mut buf);
        let out = String::from_utf8(buf).unwrap();
        assert!(out.contains("Layer 0: base"));
    }

    #[test]
    fn warn_unknowns_does_not_panic_with_no_unknowns() {
        let km = kb_with_keys(vec![ir::Key::Kp("A".into()), ir::Key::Trans]);
        warn_unknowns(&km);
    }

    #[test]
    fn warn_unknowns_does_not_panic_with_unknowns() {
        let km = kb_with_keys(vec![ir::Key::Unknown("WEIRD".into())]);
        warn_unknowns(&km);
    }
}
