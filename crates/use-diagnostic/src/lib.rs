#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_diagnostic_code as code;
pub use use_diagnostic_label as label;
pub use use_diagnostic_level as level;
pub use use_diagnostic_message as message;
pub use use_diagnostic_report as report;
pub use use_diagnostic_span as span;

pub use use_diagnostic_code::{DiagnosticCode, DiagnosticCodeError};
pub use use_diagnostic_label::{DiagnosticLabel, DiagnosticLabelKind};
pub use use_diagnostic_level::{DiagnosticLevel, DiagnosticLevelParseError};
pub use use_diagnostic_message::{DiagnosticMessage, DiagnosticNote, DiagnosticTextError};
pub use use_diagnostic_report::{Diagnostic, DiagnosticReport};
pub use use_diagnostic_span::{
    DiagnosticPosition, DiagnosticPositionError, DiagnosticSourceId, DiagnosticSourceIdError,
    DiagnosticSpan, DiagnosticSpanError,
};

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::{
        Diagnostic, DiagnosticCode, DiagnosticLabel, DiagnosticLevel, DiagnosticMessage,
        DiagnosticPosition, DiagnosticReport, DiagnosticSpan,
    };

    #[test]
    fn facade_prelude_composes_diagnostic_primitives() {
        let code = DiagnosticCode::new("VALIDATE_MISSING_FIELD").expect("code should be valid");
        let message =
            DiagnosticMessage::new("missing required field").expect("message should be valid");
        let start = DiagnosticPosition::new(3, 5).expect("position should be valid");
        let end = DiagnosticPosition::new(3, 12).expect("position should be valid");
        let span = DiagnosticSpan::without_source(start, end).expect("span should be valid");
        let label = DiagnosticLabel::primary(
            DiagnosticMessage::new("field is required here").expect("message should be valid"),
            span,
        );
        let diagnostic = Diagnostic::new(DiagnosticLevel::Error, message)
            .with_code(code)
            .with_label(label);
        let mut report = DiagnosticReport::new();

        report.add(diagnostic);

        assert_eq!(report.len(), 1);
        assert!(report.has_errors());
        assert_eq!(report.highest_severity(), Some(DiagnosticLevel::Error));
    }
}
