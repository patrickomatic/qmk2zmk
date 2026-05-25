use std::path::Path;

use crate::error::Error;

/// Read the entire contents of `path` into a string.
///
/// # Errors
/// Returns [`Error::ReadFile`] if the file cannot be read.
pub fn read_input(path: &Path) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(|source| Error::ReadFile {
        path: path.to_path_buf(),
        source,
    })
}

/// Write `content` to `path`, or print it to stdout if `path` is `None`.
///
/// # Errors
/// Returns [`Error::WriteFile`] if the file cannot be written.
pub fn write_output(content: &str, path: Option<&Path>) -> Result<(), Error> {
    if let Some(p) = path {
        std::fs::write(p, content).map_err(|source| Error::WriteFile {
            path: p.to_path_buf(),
            source,
        })
    } else {
        print!("{content}");
        Ok(())
    }
}
