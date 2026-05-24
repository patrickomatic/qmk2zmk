use std::fmt;
use std::path::PathBuf;

/// Top-level application error, caught in `main`.
#[derive(Debug)]
pub enum Error {
    ReadFile { path: PathBuf, source: std::io::Error },
    WriteFile { path: PathBuf, source: std::io::Error },
    ParseC(ParseCError),
    ParseJson(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReadFile { path, .. } => write!(f, "cannot read '{}'", path.display()),
            Error::WriteFile { path, .. } => write!(f, "cannot write '{}'", path.display()),
            Error::ParseC(e) => write!(f, "QMK C keymap parse failed: {}", e),
            Error::ParseJson(e) => write!(f, "QMK JSON keymap parse failed: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ReadFile { source, .. } => Some(source),
            Error::WriteFile { source, .. } => Some(source),
            Error::ParseC(e) => Some(e),
            Error::ParseJson(e) => Some(e),
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

/// Structured errors from the C keymap parser.
#[derive(Debug, PartialEq)]
pub enum ParseCError {
    NoKeymapsArray,
    NoKeymapsBrace,
    UnmatchedKeymapsBrace,
    UnclosedLayerBracket,
    MissingEquals { layer: String },
    MissingLayoutParen { layer: String },
    UnmatchedLayoutParen { layer: String },
}

impl fmt::Display for ParseCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseCError::NoKeymapsArray =>
                write!(f, "no keymaps array found — expected 'const uint16_t PROGMEM keymaps'"),
            ParseCError::NoKeymapsBrace =>
                write!(f, "keymaps array is missing its opening brace"),
            ParseCError::UnmatchedKeymapsBrace =>
                write!(f, "keymaps array brace is never closed"),
            ParseCError::UnclosedLayerBracket =>
                write!(f, "unclosed '[' while scanning layer entries"),
            ParseCError::MissingEquals { layer } =>
                write!(f, "layer '{layer}': expected '=' after name"),
            ParseCError::MissingLayoutParen { layer } =>
                write!(f, "layer '{layer}': LAYOUT macro is missing its opening '('"),
            ParseCError::UnmatchedLayoutParen { layer } =>
                write!(f, "layer '{layer}': LAYOUT macro parenthesis is never closed"),
        }
    }
}

impl std::error::Error for ParseCError {}
