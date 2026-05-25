use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use qmk2zmk::error::Error;
use qmk2zmk::{io, parser, qmk};

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    C,
}

#[derive(Parser)]
#[command(name = "zmk2qmk", about = "Convert ZMK keymap files to QMK format")]
struct Cli {
    /// Input file (.keymap DTS overlay)
    input: PathBuf,

    /// Output format [default: json]
    #[arg(short, long, value_enum, default_value = "json")]
    format: OutputFormat,

    /// QMK LAYOUT macro name (used in C output)
    #[arg(long, default_value = "LAYOUT")]
    layout: String,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    if let Err(ref e) = run() {
        qmk2zmk::report_and_exit(e);
    }
}

fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    let source = io::read_input(&cli.input)?;

    let mut keymap = parser::zmk::parse(&source).map_err(Error::ParseZmk)?;
    if keymap.layout.is_none() {
        keymap.layout = Some(cli.layout);
    }

    let output = match cli.format {
        OutputFormat::Json => qmk::render_json(&keymap),
        OutputFormat::C    => qmk::render_c(&keymap),
    };

    io::write_output(&output, cli.output.as_deref())
}
