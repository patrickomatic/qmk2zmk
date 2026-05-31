//! Shared filesystem I/O helpers for both binaries.
//!
//! The command-line entry points do format detection and conversion orchestration,
//! but file access is kept here so both directions produce the same structured
//! [`crate::error::Error`] variants for read and write failures. Passing `None`
//! to [`write_output`] is the only stdout path.

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn read_input_reads_existing_file() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let content = read_input(&path).unwrap();
        assert!(content.contains("[package]"));
    }

    #[test]
    fn read_input_returns_read_file_error_for_missing_file() {
        let path = PathBuf::from("/nonexistent/path/qmk2zmk_test.txt");
        let err = read_input(&path).unwrap_err();
        assert!(matches!(err, Error::ReadFile { .. }));
        assert!(err.to_string().contains("nonexistent"));
    }

    #[test]
    fn write_output_none_path_succeeds() {
        assert!(write_output("hello", None).is_ok());
    }

    #[test]
    fn write_output_to_file_round_trips_content() {
        let path = std::env::temp_dir()
            .join(format!("qmk2zmk_io_test_{}.txt", std::process::id()));
        write_output("test content", Some(&path)).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "test content");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn write_output_to_bad_path_returns_write_file_error() {
        let path = PathBuf::from("/nonexistent/dir/file.txt");
        let err = write_output("content", Some(&path)).unwrap_err();
        assert!(matches!(err, Error::WriteFile { .. }));
        assert!(err.to_string().contains("nonexistent"));
    }
}
