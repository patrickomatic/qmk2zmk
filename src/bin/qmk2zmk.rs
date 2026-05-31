//! Command-line entry point for converting QMK keymaps to ZMK overlays.
//!
//! This binary owns CLI parsing, format detection, warnings, and output routing.
//! Parsing and rendering stay in the library so they are testable without
//! spawning a process.

use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

use qmk2zmk::error::Error;
use qmk2zmk::{codes, io, qmk, zmk};

#[derive(Clone, Debug, ValueEnum)]
enum InputFormat {
    /// QMK `keymap.c` source.
    C,
    /// QMK Configurator JSON export.
    Json,
}

impl TryFrom<&Path> for InputFormat {
    type Error = ();

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        match path.extension().and_then(|e| e.to_str()) {
            Some("c") => Ok(InputFormat::C),
            Some("json") => Ok(InputFormat::Json),
            Some(other) => {
                let _ = other.len();
                Err(())
            }
            None => Err(()),
        }
    }
}

#[derive(Parser)]
#[command(name = "qmk2zmk", about = "Convert QMK keymap files to ZMK format")]
struct Cli {
    /// Input file (keymap.c or keymap.json)
    #[arg(required_unless_present = "list_keyboards")]
    input: Option<PathBuf>,

    /// Input format — auto-detected from file extension if not given
    #[arg(short, long, value_enum)]
    format: Option<InputFormat>,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Known keyboard name to set column count (see --list-keyboards)
    #[arg(long)]
    keyboard: Option<String>,

    /// Override columns per row in ZMK output
    #[arg(long)]
    cols: Option<usize>,

    /// List known keyboards and their column counts, then exit
    #[arg(long)]
    list_keyboards: bool,

    /// Suppress warnings for unmapped keycodes
    #[arg(long)]
    no_warn: bool,

    /// Parse the keymap and print a layout table, then exit without converting
    #[arg(short = 'p', long)]
    print_layout: bool,
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

    let format = cli.format.unwrap_or_else(|| {
        InputFormat::try_from(input.as_path()).unwrap_or_else(|()| {
            eprintln!("warning: cannot detect format from extension, assuming C");
            InputFormat::C
        })
    });

    let source = io::read_input(&input)?;

    let keyboard = match format {
        InputFormat::C => qmk::parse_c::parse(&source)?,
        InputFormat::Json => qmk::parse_json::parse(&source)?,
    };

    if !cli.no_warn {
        qmk2zmk::warn_unknowns(&keyboard);
    }

    let cols = cli
        .cols
        .or_else(|| cli.keyboard.as_deref().and_then(codes::keyboard_cols));

    if cli.print_layout {
        qmk2zmk::print_layout(&keyboard, cols);
        return Ok(());
    }

    let output = zmk::render(&keyboard, cols);
    io::write_output(&output, cli.output.as_deref())
}

/// Print the built-in keyboard column heuristics used by `--keyboard`.
fn print_keyboard_list() {
    println!("{:<14} Columns", "Keyboard");
    for (name, cols) in codes::KNOWN_KEYBOARDS {
        println!("{name:<14} {cols}");
    }
}
