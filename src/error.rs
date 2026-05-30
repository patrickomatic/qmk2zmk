//! Structured error types for parser and I/O failures.
//!
//! The binaries print errors only at the outer `main` boundary through
//! [`crate::report_and_exit`]. Everything below that boundary returns one of
//! these types, which keeps parse failures testable and prevents library code
//! from deciding how an application should report or exit.

use std::fmt;
use std::path::PathBuf;

/// Top-level application error returned by binary `run()` functions.
///
/// This enum groups I/O errors and format-specific parser errors so the
/// command-line binaries can use a single `Result<(), Error>` signature while
/// still preserving source errors for diagnostic chains.
#[derive(Debug)]
pub enum Error {
    /// Failed to read the requested input file.
    ReadFile {
        /// Path the user asked the binary to read.
        path: PathBuf,
        /// Original filesystem error.
        source: std::io::Error,
    },
    /// Failed to write the requested output file.
    WriteFile {
        /// Path the user asked the binary to write.
        path: PathBuf,
        /// Original filesystem error.
        source: std::io::Error,
    },
    /// Failed to parse a QMK C `keymap.c` source file.
    ParseC(ParseCError),
    /// Failed to parse QMK Configurator JSON.
    ParseJson(serde_json::Error),
    /// Failed to parse a ZMK `.keymap` DTS overlay.
    ParseZmk(ParseZmkError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReadFile { path, .. } => write!(f, "cannot read '{}'", path.display()),
            Error::WriteFile { path, .. } => write!(f, "cannot write '{}'", path.display()),
            Error::ParseC(e) => write!(f, "QMK C keymap parse failed: {e}"),
            Error::ParseJson(e) => write!(f, "QMK JSON keymap parse failed: {e}"),
            Error::ParseZmk(e) => write!(f, "ZMK keymap parse failed: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ReadFile { source, .. } | Error::WriteFile { source, .. } => Some(source),
            Error::ParseC(e) => Some(e),
            Error::ParseJson(e) => Some(e),
            Error::ParseZmk(e) => Some(e),
        }
    }
}

impl From<ParseCError> for Error {
    fn from(e: ParseCError) -> Self {
        Error::ParseC(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ParseJson(e)
    }
}

impl From<ParseZmkError> for Error {
    fn from(e: ParseZmkError) -> Self {
        Error::ParseZmk(e)
    }
}

/// Structured errors from the QMK C keymap parser.
///
/// The C parser intentionally distinguishes missing top-level input from
/// malformed layer entries. That gives tests and callers precise failure modes
/// without relying on substring matching in generic error messages.
#[derive(Debug, PartialEq)]
pub enum ParseCError {
    /// The source did not contain a `keymaps` array.
    NoKeymapsArray,
    /// The `keymaps` array did not have an opening `{`.
    NoKeymapsBrace,
    /// The `keymaps` array opened but did not close.
    UnmatchedKeymapsBrace,
    /// A layer entry started with `[` but did not close with `]`.
    UnclosedLayerBracket,
    /// A layer entry did not have `=` after the layer name.
    MissingEquals {
        /// Layer designator being parsed when the `=` was expected.
        layer: String,
    },
    /// A layer's layout macro was missing its opening `(`.
    MissingLayoutParen {
        /// Layer designator whose `LAYOUT...` initializer is malformed.
        layer: String,
    },
    /// A layer's layout macro opened but did not close.
    UnmatchedLayoutParen {
        /// Layer designator whose layout macro call never closed.
        layer: String,
    },
}

impl fmt::Display for ParseCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseCError::NoKeymapsArray => write!(
                f,
                "no keymaps array found — expected 'const uint16_t PROGMEM keymaps'"
            ),
            ParseCError::NoKeymapsBrace => write!(f, "keymaps array is missing its opening brace"),
            ParseCError::UnmatchedKeymapsBrace => write!(f, "keymaps array brace is never closed"),
            ParseCError::UnclosedLayerBracket => {
                write!(f, "unclosed '[' while scanning layer entries")
            }
            ParseCError::MissingEquals { layer } => {
                write!(f, "layer '{layer}': expected '=' after name")
            }
            ParseCError::MissingLayoutParen { layer } => write!(
                f,
                "layer '{layer}': LAYOUT macro is missing its opening '('"
            ),
            ParseCError::UnmatchedLayoutParen { layer } => write!(
                f,
                "layer '{layer}': LAYOUT macro parenthesis is never closed"
            ),
        }
    }
}

impl std::error::Error for ParseCError {}

/// Structured errors from the ZMK keymap parser.
///
/// ZMK input is DTS overlay syntax. The parser is intentionally lightweight, so
/// these variants focus on the structural issues it must understand: finding the
/// `keymap` node and matching braces for nested blocks.
#[derive(Debug, PartialEq)]
pub enum ParseZmkError {
    /// The source did not contain a DTS `keymap { ... }` block.
    NoKeymapBlock,
    /// A named DTS block opened but did not close.
    UnclosedBlock {
        /// Best-effort name of the block that was open when parsing failed.
        context: String,
    },
}

impl fmt::Display for ParseZmkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseZmkError::NoKeymapBlock => write!(
                f,
                "no keymap {{ }} block found — is this a ZMK .keymap file?"
            ),
            ParseZmkError::UnclosedBlock { context } => write!(f, "unclosed block in '{context}'"),
        }
    }
}

impl std::error::Error for ParseZmkError {}
