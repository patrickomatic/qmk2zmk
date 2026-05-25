use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use qmk2zmk::error::Error;
use qmk2zmk::{codes, io, qmk, zmk};

#[derive(Clone, Debug, ValueEnum)]
enum InputFormat {
    C,
    Json,
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
}

fn main() {
    if let Err(ref e) = run() {
        qmk2zmk::report_and_exit(e);
    }
}

fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    if cli.list_keyboards {
        print_keyboard_list();
        return Ok(());
    }

    let input = cli.input.expect("required_unless_present = list_keyboards");

    let format = cli.format.unwrap_or_else(|| {
        match input.extension().and_then(|e| e.to_str()) {
            Some("c")    => InputFormat::C,
            Some("json") => InputFormat::Json,
            _ => {
                eprintln!("warning: cannot detect format from extension, assuming C");
                InputFormat::C
            }
        }
    });

    let source = io::read_input(&input)?;

    let keymap = match format {
        InputFormat::C    => qmk::parse_c::parse(&source).map_err(Error::ParseC)?,
        InputFormat::Json => qmk::parse_json::parse(&source).map_err(Error::ParseJson)?,
    };

    if !cli.no_warn {
        qmk2zmk::warn_unknowns(&keymap);
    }

    let cols = cli.cols.or_else(|| cli.keyboard.as_deref().and_then(codes::keyboard_cols));
    let output = zmk::render(&keymap, cols);
    io::write_output(&output, cli.output.as_deref())
}

fn print_keyboard_list() {
    println!("{:<14} Columns", "Keyboard");
    for (name, cols) in codes::KNOWN_KEYBOARDS {
        println!("{name:<14} {cols}");
    }
}
