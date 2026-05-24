use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use qmk2zmk::error::Error;
use qmk2zmk::parser;
use qmk2zmk::zmk;

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
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        let mut src = std::error::Error::source(&e);
        while let Some(cause) = src {
            eprintln!("  caused by: {cause}");
            src = cause.source();
        }
        std::process::exit(1);
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

    let source = std::fs::read_to_string(&cli.input).map_err(|source| Error::ReadFile {
        path: cli.input.clone(),
        source,
    })?;

    let keymap = match format {
        InputFormat::C    => parser::qmk_c::parse(&source).map_err(Error::ParseC)?,
        InputFormat::Json => parser::qmk_json::parse(&source).map_err(Error::ParseJson)?,
    };

    let output = zmk::render(&keymap);

    match cli.output {
        Some(ref path) => std::fs::write(path, &output).map_err(|source| Error::WriteFile {
            path: path.clone(),
            source,
        })?,
        None => print!("{output}"),
    }

    Ok(())
}
