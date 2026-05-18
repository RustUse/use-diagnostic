#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};

/// A stable string identifier for a diagnostic.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
    /// Creates a diagnostic code from a non-empty string after trimming surrounding whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticCodeError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, DiagnosticCodeError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            return Err(DiagnosticCodeError::Empty);
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns the diagnostic code text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the code and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for DiagnosticCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DiagnosticCode {
    type Err = DiagnosticCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing a [`DiagnosticCode`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticCodeError {
    /// The code was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for DiagnosticCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diagnostic code cannot be empty"),
        }
    }
}

impl std::error::Error for DiagnosticCodeError {}

#[cfg(test)]
mod tests {
    use super::{DiagnosticCode, DiagnosticCodeError};

    #[test]
    fn accepts_valid_code() {
        let code = DiagnosticCode::new("CONFIG001").expect("code should be valid");

        assert_eq!(code.as_str(), "CONFIG001");
    }

    #[test]
    fn rejects_empty_code() {
        assert_eq!(DiagnosticCode::new("   "), Err(DiagnosticCodeError::Empty));
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let code = DiagnosticCode::new("  DATA.INVALID_SHAPE  ").expect("code should be valid");

        assert_eq!(code.as_str(), "DATA.INVALID_SHAPE");
    }

    #[test]
    fn display_round_trips_through_parse() {
        let code = DiagnosticCode::new("VALIDATE_MISSING_FIELD").expect("code should be valid");
        let parsed: DiagnosticCode = code
            .to_string()
            .parse()
            .expect("displayed code should parse");

        assert_eq!(parsed, code);
    }

    #[test]
    fn ordering_is_deterministic() {
        let mut codes = [
            DiagnosticCode::new("DATA.INVALID_SHAPE").expect("code should be valid"),
            DiagnosticCode::new("CONFIG001").expect("code should be valid"),
            DiagnosticCode::new("VALIDATE_MISSING_FIELD").expect("code should be valid"),
        ];

        codes.sort();

        assert_eq!(codes[0].as_str(), "CONFIG001");
        assert_eq!(codes[1].as_str(), "DATA.INVALID_SHAPE");
        assert_eq!(codes[2].as_str(), "VALIDATE_MISSING_FIELD");
    }
}
