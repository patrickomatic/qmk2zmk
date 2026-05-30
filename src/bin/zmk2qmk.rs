//! Command-line entry point for converting ZMK overlays to QMK output.
//!
//! This binary selects QMK JSON or C rendering, supplies layout metadata that
//! ZMK overlays normally do not contain, and delegates parsing/rendering to the
//! library modules.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use qmk2zmk::error::Error;
use qmk2zmk::{codes, io, qmk, zmk};

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    /// QMK Configurator-compatible JSON.
    Json,
    /// QMK `keymap.c` source.
    C,
}

#[derive(Parser)]
#[command(name = "zmk2qmk", about = "Convert ZMK keymap files to QMK format")]
struct Cli {
    /// Input file (.keymap DTS overlay)
    #[arg(required_unless_present = "list_keyboards")]
    input: Option<PathBuf>,

    /// Output format [default: json]
    #[arg(short, long, value_enum, default_value = "json")]
    format: OutputFormat,

    /// QMK LAYOUT macro name (used in C output)
    #[arg(long, default_value = "LAYOUT")]
    layout: String,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Known keyboard name to set column count (see --list-keyboards)
    #[arg(long)]
    keyboard: Option<String>,

    /// Override columns per row in QMK C output
    #[arg(long)]
    cols: Option<usize>,

    /// List known keyboards and their column counts, then exit
    #[arg(long)]
    list_keyboards: bool,

    /// Suppress warnings for unmapped keycodes
    #[arg(long)]
    no_warn: bool,
}

/// Process exit boundary for the binary.
///
/// All fallible work happens in [`run`] so errors can be returned and printed by
/// the shared reporting function.
fn main() {
    if let Err(ref e) = run() {
        qmk2zmk::report_and_exit(e);
    }
}

/// Parse CLI arguments, perform the conversion, and write output.
fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    if cli.list_keyboards {
        print_keyboard_list();
        return Ok(());
    }

    let input = cli.input.expect("required_unless_present = list_keyboards");

    let source = io::read_input(&input)?;

    let mut keymap = zmk::parse::parse(&source)?;
    if keymap.layout.is_none() {
        keymap.layout = Some(cli.layout);
    }

    if !cli.no_warn {
        qmk2zmk::warn_unknowns(&keymap);
    }

    let cols = cli
        .cols
        .or_else(|| cli.keyboard.as_deref().and_then(codes::keyboard_cols));
    let output = match cli.format {
        OutputFormat::Json => qmk::render_json(&keymap),
        OutputFormat::C => qmk::render_c(&keymap, cols),
    };

    io::write_output(&output, cli.output.as_deref())
}

/// Print the built-in keyboard column heuristics used by `--keyboard`.
fn print_keyboard_list() {
    println!("{:<14} Columns", "Keyboard");
    for (name, cols) in codes::KNOWN_KEYBOARDS {
        println!("{name:<14} {cols}");
    }
}
