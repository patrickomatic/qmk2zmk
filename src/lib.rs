pub mod codes;
pub mod error;
pub mod io;
pub mod ir;
pub mod parser;
pub mod qmk;
pub mod zmk;

pub fn report_and_exit(e: &dyn std::error::Error) -> ! {
    eprintln!("error: {e}");
    let mut src = e.source();
    while let Some(cause) = src {
        eprintln!("  caused by: {cause}");
        src = cause.source();
    }
    std::process::exit(1);
}
