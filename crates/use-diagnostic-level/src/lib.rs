#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};

/// The severity level of a diagnostic.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticLevel {
    /// Informational diagnostic.
    Info,
    /// Warning diagnostic.
    Warning,
    /// Error diagnostic.
    Error,
    /// Fatal diagnostic.
    Fatal,
}

impl DiagnosticLevel {
    /// Returns the canonical lowercase level name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }

    /// Returns `true` when the level is an error or more severe.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }

    /// Returns `true` when the level is fatal.
    #[must_use]
    pub const fn is_fatal(self) -> bool {
        matches!(self, Self::Fatal)
    }
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DiagnosticLevel {
    type Err = DiagnosticLevelParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "info" => Ok(Self::Info),
            "warn" | "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            "fatal" => Ok(Self::Fatal),
            _ => Err(DiagnosticLevelParseError),
        }
    }
}

/// Error returned when parsing a diagnostic level fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticLevelParseError;

impl fmt::Display for DiagnosticLevelParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("unknown diagnostic level")
    }
}

impl std::error::Error for DiagnosticLevelParseError {}

#[cfg(test)]
mod tests {
    use super::{DiagnosticLevel, DiagnosticLevelParseError};

    #[test]
    fn parses_known_levels() -> Result<(), DiagnosticLevelParseError> {
        assert_eq!("info".parse::<DiagnosticLevel>()?, DiagnosticLevel::Info);
        assert_eq!(
            "warning".parse::<DiagnosticLevel>()?,
            DiagnosticLevel::Warning
        );
        assert_eq!("error".parse::<DiagnosticLevel>()?, DiagnosticLevel::Error);
        assert_eq!("fatal".parse::<DiagnosticLevel>()?, DiagnosticLevel::Fatal);
        Ok(())
    }

    #[test]
    fn parses_aliases() -> Result<(), DiagnosticLevelParseError> {
        assert_eq!("warn".parse::<DiagnosticLevel>()?, DiagnosticLevel::Warning);
        assert_eq!(
            " WARNING ".parse::<DiagnosticLevel>()?,
            DiagnosticLevel::Warning
        );
        Ok(())
    }

    #[test]
    fn displays_known_levels() {
        assert_eq!(DiagnosticLevel::Info.to_string(), "info");
        assert_eq!(DiagnosticLevel::Warning.to_string(), "warning");
        assert_eq!(DiagnosticLevel::Error.to_string(), "error");
        assert_eq!(DiagnosticLevel::Fatal.to_string(), "fatal");
    }

    #[test]
    fn severity_ordering_places_more_severe_levels_greater() {
        assert!(DiagnosticLevel::Warning > DiagnosticLevel::Info);
        assert!(DiagnosticLevel::Error > DiagnosticLevel::Warning);
        assert!(DiagnosticLevel::Fatal > DiagnosticLevel::Error);
    }
}
