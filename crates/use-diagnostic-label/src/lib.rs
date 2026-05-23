#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

use use_diagnostic_message::DiagnosticMessage;
use use_diagnostic_span::DiagnosticSpan;

/// The role of a diagnostic label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticLabelKind {
    /// The primary location or reason for a diagnostic.
    Primary,
    /// Additional related context.
    Secondary,
    /// Help-oriented context.
    Help,
    /// Note-oriented context.
    Note,
}

impl DiagnosticLabelKind {
    /// Returns the canonical lowercase label kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Help => "help",
            Self::Note => "note",
        }
    }
}

impl fmt::Display for DiagnosticLabelKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Renderer-neutral context attached to a diagnostic.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticLabel {
    kind: DiagnosticLabelKind,
    message: DiagnosticMessage,
    span: Option<DiagnosticSpan>,
}

impl DiagnosticLabel {
    /// Creates a diagnostic label.
    #[must_use]
    pub const fn new(
        kind: DiagnosticLabelKind,
        message: DiagnosticMessage,
        span: Option<DiagnosticSpan>,
    ) -> Self {
        Self {
            kind,
            message,
            span,
        }
    }

    /// Creates a primary label with a span.
    #[must_use]
    pub const fn primary(message: DiagnosticMessage, span: DiagnosticSpan) -> Self {
        Self::new(DiagnosticLabelKind::Primary, message, Some(span))
    }

    /// Creates a secondary label with a span.
    #[must_use]
    pub const fn secondary(message: DiagnosticMessage, span: DiagnosticSpan) -> Self {
        Self::new(DiagnosticLabelKind::Secondary, message, Some(span))
    }

    /// Creates a help label without a span.
    #[must_use]
    pub const fn help(message: DiagnosticMessage) -> Self {
        Self::new(DiagnosticLabelKind::Help, message, None)
    }

    /// Creates a note label without a span.
    #[must_use]
    pub const fn note(message: DiagnosticMessage) -> Self {
        Self::new(DiagnosticLabelKind::Note, message, None)
    }

    /// Returns the label kind.
    #[must_use]
    pub const fn kind(&self) -> DiagnosticLabelKind {
        self.kind
    }

    /// Returns the label message.
    #[must_use]
    pub const fn message(&self) -> &DiagnosticMessage {
        &self.message
    }

    /// Returns the optional label span.
    #[must_use]
    pub const fn span(&self) -> Option<&DiagnosticSpan> {
        self.span.as_ref()
    }
}

impl fmt::Display for DiagnosticLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{DiagnosticLabel, DiagnosticLabelKind};
    use use_diagnostic_message::DiagnosticMessage;
    use use_diagnostic_span::{DiagnosticPosition, DiagnosticSpan};

    fn test_span() -> DiagnosticSpan {
        let start = DiagnosticPosition::new(2, 5).expect("position should be valid");
        let end = DiagnosticPosition::new(2, 9).expect("position should be valid");
        DiagnosticSpan::without_source(start, end).expect("span should be valid")
    }

    #[test]
    fn creates_primary_label_with_span() {
        let label = DiagnosticLabel::primary(
            DiagnosticMessage::new("invalid value").expect("message should be valid"),
            test_span(),
        );

        assert_eq!(label.kind(), DiagnosticLabelKind::Primary);
        assert!(label.span().is_some());
    }

    #[test]
    fn creates_secondary_label_with_span() {
        let label = DiagnosticLabel::secondary(
            DiagnosticMessage::new("defined here").expect("message should be valid"),
            test_span(),
        );

        assert_eq!(label.kind(), DiagnosticLabelKind::Secondary);
        assert!(label.span().is_some());
    }

    #[test]
    fn creates_help_label_without_span() {
        let label = DiagnosticLabel::help(
            DiagnosticMessage::new("provide the missing field").expect("message should be valid"),
        );

        assert_eq!(label.kind(), DiagnosticLabelKind::Help);
        assert!(label.span().is_none());
    }

    #[test]
    fn display_and_debug_do_not_assume_renderer_output() {
        let label = DiagnosticLabel::note(
            DiagnosticMessage::new("plain context").expect("message should be valid"),
        );

        assert_eq!(label.to_string(), "plain context");
        assert!(format!("{label:?}").contains("Note"));
    }
}
