#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};

/// A human-facing diagnostic message.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticMessage(String);

impl DiagnosticMessage {
    /// Creates a diagnostic message from non-empty plain text after trimming surrounding whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, DiagnosticTextError> {
        Ok(Self(normalize_text(value.as_ref())?))
    }

    /// Returns the message text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the message and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for DiagnosticMessage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DiagnosticMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DiagnosticMessage {
    type Err = DiagnosticTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Additional plain-text context attached to a diagnostic.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticNote(String);

impl DiagnosticNote {
    /// Creates a diagnostic note from non-empty plain text after trimming surrounding whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, DiagnosticTextError> {
        Ok(Self(normalize_text(value.as_ref())?))
    }

    /// Returns the note text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the note and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for DiagnosticNote {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DiagnosticNote {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DiagnosticNote {
    type Err = DiagnosticTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing diagnostic text primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticTextError {
    /// The text was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for DiagnosticTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diagnostic text cannot be empty"),
        }
    }
}

impl std::error::Error for DiagnosticTextError {}

fn normalize_text(value: &str) -> Result<String, DiagnosticTextError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        Err(DiagnosticTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{DiagnosticMessage, DiagnosticNote, DiagnosticTextError};

    #[test]
    fn accepts_valid_message() {
        let message =
            DiagnosticMessage::new("missing required field").expect("message should be valid");

        assert_eq!(message.as_str(), "missing required field");
    }

    #[test]
    fn rejects_empty_message() {
        assert_eq!(
            DiagnosticMessage::new(" \n\t "),
            Err(DiagnosticTextError::Empty)
        );
    }

    #[test]
    fn trims_surrounding_message_whitespace() {
        let message = DiagnosticMessage::new("  invalid configuration value  ")
            .expect("message should be valid");

        assert_eq!(message.as_str(), "invalid configuration value");
    }

    #[test]
    fn display_returns_plain_message() {
        let message = DiagnosticMessage::new("malformed input").expect("message should be valid");

        assert_eq!(message.to_string(), "malformed input");
    }

    #[test]
    fn constructs_notes() {
        let note =
            DiagnosticNote::new("field names are case-sensitive").expect("note should be valid");

        assert_eq!(note.as_str(), "field names are case-sensitive");
    }
}
