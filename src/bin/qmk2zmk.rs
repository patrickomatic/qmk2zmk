use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use qmk2zmk::error::Error;
use qmk2zmk::{io, qmk, zmk};

#[derive(Clone, Debug, ValueEnum)]
enum InputFormat {
    C,
    Json,
}

#[derive(Parser)]
#[command(name = "qmk2zmk", about = "Convert QMK keymap files to ZMK format")]
struct Cli {
    /// Input file (keymap.c or keymap.json)
    input: PathBuf,

    /// Input format — auto-detected from file extension if not given
    #[arg(short, long, value_enum)]
    format: Option<InputFormat>,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Print warnings for unmapped keycodes to stderr
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    if let Err(ref e) = run() {
        qmk2zmk::report_and_exit(e);
    }
}

fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    let format = cli.format.unwrap_or_else(|| {
        match cli.input.extension().and_then(|e| e.to_str()) {
            Some("c")    => InputFormat::C,
            Some("json") => InputFormat::Json,
            _ => {
                eprintln!("warning: cannot detect format from extension, assuming C");
                InputFormat::C
            }
        }
    });

    let source = io::read_input(&cli.input)?;

    let keymap = match format {
        InputFormat::C    => qmk::parse_c::parse(&source).map_err(Error::ParseC)?,
        InputFormat::Json => qmk::parse_json::parse(&source).map_err(Error::ParseJson)?,
    };

    if cli.verbose {
        qmk2zmk::warn_unknowns(&keymap);
    }
    let output = zmk::render(&keymap);
    io::write_output(&output, cli.output.as_deref())
}
