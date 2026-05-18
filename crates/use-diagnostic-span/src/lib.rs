#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};

/// A 1-based line and column position.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticPosition {
    line: usize,
    column: usize,
}

impl DiagnosticPosition {
    /// Creates a 1-based position.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticPositionError::ZeroLine`] when `line` is zero and
    /// [`DiagnosticPositionError::ZeroColumn`] when `column` is zero.
    pub const fn new(line: usize, column: usize) -> Result<Self, DiagnosticPositionError> {
        if line == 0 {
            return Err(DiagnosticPositionError::ZeroLine);
        }

        if column == 0 {
            return Err(DiagnosticPositionError::ZeroColumn);
        }

        Ok(Self { line, column })
    }

    /// Returns the 1-based line number.
    #[must_use]
    pub const fn line(self) -> usize {
        self.line
    }

    /// Returns the 1-based column number.
    #[must_use]
    pub const fn column(self) -> usize {
        self.column
    }
}

/// Errors returned while constructing a [`DiagnosticPosition`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticPositionError {
    /// The line number was zero.
    ZeroLine,
    /// The column number was zero.
    ZeroColumn,
}

impl fmt::Display for DiagnosticPositionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroLine => formatter.write_str("diagnostic position line must be at least 1"),
            Self::ZeroColumn => {
                formatter.write_str("diagnostic position column must be at least 1")
            }
        }
    }
}

impl std::error::Error for DiagnosticPositionError {}

/// A stable identifier for a source, file, buffer, document, or virtual source.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticSourceId(String);

impl DiagnosticSourceId {
    /// Creates a source identifier from non-empty text after trimming surrounding whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticSourceIdError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, DiagnosticSourceIdError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            return Err(DiagnosticSourceIdError::Empty);
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns the source identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the source identifier and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for DiagnosticSourceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DiagnosticSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DiagnosticSourceId {
    type Err = DiagnosticSourceIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing a [`DiagnosticSourceId`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticSourceIdError {
    /// The source ID was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for DiagnosticSourceIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diagnostic source ID cannot be empty"),
        }
    }
}

impl std::error::Error for DiagnosticSourceIdError {}

/// A generic source span with optional source identity.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticSpan {
    source: Option<DiagnosticSourceId>,
    start: DiagnosticPosition,
    end: DiagnosticPosition,
}

impl DiagnosticSpan {
    /// Creates a diagnostic span and validates that the end is not before the start.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticSpanError::Reversed`] when `end` is before `start`.
    pub fn new(
        source: Option<DiagnosticSourceId>,
        start: DiagnosticPosition,
        end: DiagnosticPosition,
    ) -> Result<Self, DiagnosticSpanError> {
        if end < start {
            return Err(DiagnosticSpanError::Reversed);
        }

        Ok(Self { source, start, end })
    }

    /// Creates a span with a source identifier.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticSpanError::Reversed`] when `end` is before `start`.
    pub fn with_source(
        source: DiagnosticSourceId,
        start: DiagnosticPosition,
        end: DiagnosticPosition,
    ) -> Result<Self, DiagnosticSpanError> {
        Self::new(Some(source), start, end)
    }

    /// Creates a span without a source identifier.
    ///
    /// # Errors
    ///
    /// Returns [`DiagnosticSpanError::Reversed`] when `end` is before `start`.
    pub fn without_source(
        start: DiagnosticPosition,
        end: DiagnosticPosition,
    ) -> Result<Self, DiagnosticSpanError> {
        Self::new(None, start, end)
    }

    /// Returns the optional source identifier.
    #[must_use]
    pub fn source(&self) -> Option<&DiagnosticSourceId> {
        self.source.as_ref()
    }

    /// Returns the start position.
    #[must_use]
    pub const fn start(&self) -> DiagnosticPosition {
        self.start
    }

    /// Returns the end position.
    #[must_use]
    pub const fn end(&self) -> DiagnosticPosition {
        self.end
    }
}

/// Errors returned while constructing a [`DiagnosticSpan`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticSpanError {
    /// The span end position was before the start position.
    Reversed,
}

impl fmt::Display for DiagnosticSpanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reversed => formatter.write_str("diagnostic span end cannot be before start"),
        }
    }
}

impl std::error::Error for DiagnosticSpanError {}

#[cfg(test)]
mod tests {
    use super::{
        DiagnosticPosition, DiagnosticPositionError, DiagnosticSourceId, DiagnosticSpan,
        DiagnosticSpanError,
    };

    #[test]
    fn accepts_valid_position() {
        let position = DiagnosticPosition::new(1, 1).expect("position should be valid");

        assert_eq!(position.line(), 1);
        assert_eq!(position.column(), 1);
    }

    #[test]
    fn rejects_zero_line_or_column() {
        assert_eq!(
            DiagnosticPosition::new(0, 1),
            Err(DiagnosticPositionError::ZeroLine)
        );
        assert_eq!(
            DiagnosticPosition::new(1, 0),
            Err(DiagnosticPositionError::ZeroColumn)
        );
    }

    #[test]
    fn accepts_valid_span() {
        let start = DiagnosticPosition::new(2, 4).expect("position should be valid");
        let end = DiagnosticPosition::new(2, 9).expect("position should be valid");
        let span = DiagnosticSpan::without_source(start, end).expect("span should be valid");

        assert_eq!(span.start(), start);
        assert_eq!(span.end(), end);
    }

    #[test]
    fn rejects_reversed_span() {
        let start = DiagnosticPosition::new(3, 10).expect("position should be valid");
        let end = DiagnosticPosition::new(3, 5).expect("position should be valid");

        assert_eq!(
            DiagnosticSpan::without_source(start, end),
            Err(DiagnosticSpanError::Reversed)
        );
    }

    #[test]
    fn creates_span_with_source_id() {
        let source = DiagnosticSourceId::new(" config.toml ").expect("source should be valid");
        let start = DiagnosticPosition::new(4, 1).expect("position should be valid");
        let end = DiagnosticPosition::new(4, 3).expect("position should be valid");
        let span = DiagnosticSpan::with_source(source, start, end).expect("span should be valid");

        assert_eq!(
            span.source().map(DiagnosticSourceId::as_str),
            Some("config.toml")
        );
    }

    #[test]
    fn creates_span_without_source_id() {
        let start = DiagnosticPosition::new(1, 1).expect("position should be valid");
        let end = DiagnosticPosition::new(1, 1).expect("position should be valid");
        let span = DiagnosticSpan::without_source(start, end).expect("span should be valid");

        assert!(span.source().is_none());
    }
}
