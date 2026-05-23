#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_diagnostic_code::DiagnosticCode;
use use_diagnostic_label::DiagnosticLabel;
use use_diagnostic_level::DiagnosticLevel;
use use_diagnostic_message::{DiagnosticMessage, DiagnosticNote};

/// A structured diagnostic data value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    code: Option<DiagnosticCode>,
    level: DiagnosticLevel,
    message: DiagnosticMessage,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<DiagnosticNote>,
}

impl Diagnostic {
    /// Creates a diagnostic with a level and message.
    #[must_use]
    pub const fn new(level: DiagnosticLevel, message: DiagnosticMessage) -> Self {
        Self {
            code: None,
            level,
            message,
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Returns a diagnostic with the provided code attached.
    #[must_use]
    pub fn with_code(mut self, code: DiagnosticCode) -> Self {
        self.code = Some(code);
        self
    }

    /// Returns a diagnostic with the provided label appended.
    #[must_use]
    pub fn with_label(mut self, label: DiagnosticLabel) -> Self {
        self.labels.push(label);
        self
    }

    /// Returns a diagnostic with the provided note appended.
    #[must_use]
    pub fn with_note(mut self, note: DiagnosticNote) -> Self {
        self.notes.push(note);
        self
    }

    /// Appends a label.
    pub fn add_label(&mut self, label: DiagnosticLabel) {
        self.labels.push(label);
    }

    /// Appends a note.
    pub fn add_note(&mut self, note: DiagnosticNote) {
        self.notes.push(note);
    }

    /// Returns the optional diagnostic code.
    #[must_use]
    pub const fn code(&self) -> Option<&DiagnosticCode> {
        self.code.as_ref()
    }

    /// Returns the diagnostic level.
    #[must_use]
    pub const fn level(&self) -> DiagnosticLevel {
        self.level
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub const fn message(&self) -> &DiagnosticMessage {
        &self.message
    }

    /// Returns the diagnostic labels.
    #[must_use]
    pub fn labels(&self) -> &[DiagnosticLabel] {
        &self.labels
    }

    /// Returns the diagnostic notes.
    #[must_use]
    pub fn notes(&self) -> &[DiagnosticNote] {
        &self.notes
    }
}

/// An insertion-order collection of diagnostics.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiagnosticReport {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticReport {
    /// Creates an empty diagnostic report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Adds a diagnostic to the end of the report.
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns the number of diagnostics in the report.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns `true` when the report contains no diagnostics.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns the diagnostics as a slice in insertion order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Iterates diagnostics in insertion order.
    pub fn iter(&self) -> core::slice::Iter<'_, Diagnostic> {
        self.diagnostics.iter()
    }

    /// Counts diagnostics with the exact provided level.
    #[must_use]
    pub fn count_by_level(&self, level: DiagnosticLevel) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.level() == level)
            .count()
    }

    /// Returns `true` when the report contains an error or fatal diagnostic.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.level().is_error())
    }

    /// Returns `true` when the report contains a fatal diagnostic.
    #[must_use]
    pub fn has_fatal(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.level().is_fatal())
    }

    /// Returns the highest diagnostic severity in the report.
    #[must_use]
    pub fn highest_severity(&self) -> Option<DiagnosticLevel> {
        self.diagnostics.iter().map(Diagnostic::level).max()
    }

    /// Extends this report with diagnostics from another report, preserving insertion order.
    pub fn extend_report(&mut self, other: Self) {
        self.diagnostics.extend(other.diagnostics);
    }
}

impl FromIterator<Diagnostic> for DiagnosticReport {
    fn from_iter<T: IntoIterator<Item = Diagnostic>>(diagnostics: T) -> Self {
        Self {
            diagnostics: diagnostics.into_iter().collect(),
        }
    }
}

impl<'a> IntoIterator for &'a DiagnosticReport {
    type Item = &'a Diagnostic;
    type IntoIter = core::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, DiagnosticReport};
    use use_diagnostic_code::DiagnosticCode;
    use use_diagnostic_label::DiagnosticLabel;
    use use_diagnostic_level::DiagnosticLevel;
    use use_diagnostic_message::{DiagnosticMessage, DiagnosticNote};

    fn message(text: &str) -> DiagnosticMessage {
        DiagnosticMessage::new(text).expect("message should be valid")
    }

    #[test]
    fn creates_diagnostic() {
        let diagnostic = Diagnostic::new(DiagnosticLevel::Warning, message("check the value"));

        assert_eq!(diagnostic.level(), DiagnosticLevel::Warning);
        assert_eq!(diagnostic.message().as_str(), "check the value");
        assert!(diagnostic.code().is_none());
    }

    #[test]
    fn creates_diagnostic_with_code() {
        let diagnostic = Diagnostic::new(DiagnosticLevel::Error, message("missing field"))
            .with_code(
                DiagnosticCode::new("VALIDATE_MISSING_FIELD").expect("code should be valid"),
            );

        assert_eq!(
            diagnostic.code().map(DiagnosticCode::as_str),
            Some("VALIDATE_MISSING_FIELD")
        );
    }

    #[test]
    fn creates_diagnostic_with_labels() {
        let label = DiagnosticLabel::help(message("add the missing field"));
        let diagnostic =
            Diagnostic::new(DiagnosticLevel::Error, message("missing field")).with_label(label);

        assert_eq!(diagnostic.labels().len(), 1);
    }

    #[test]
    fn adds_and_iterates_report_diagnostics() {
        let first = Diagnostic::new(DiagnosticLevel::Info, message("first"));
        let second = Diagnostic::new(DiagnosticLevel::Warning, message("second"));
        let mut report = DiagnosticReport::new();

        report.add(first);
        report.add(second);

        let messages: Vec<&str> = report
            .iter()
            .map(|diagnostic| diagnostic.message().as_str())
            .collect();

        assert_eq!(messages, vec!["first", "second"]);
    }

    #[test]
    fn counts_diagnostics_by_level() {
        let report: DiagnosticReport = [
            Diagnostic::new(DiagnosticLevel::Info, message("informational")),
            Diagnostic::new(DiagnosticLevel::Error, message("error one")),
            Diagnostic::new(DiagnosticLevel::Error, message("error two")),
        ]
        .into_iter()
        .collect();

        assert_eq!(report.count_by_level(DiagnosticLevel::Info), 1);
        assert_eq!(report.count_by_level(DiagnosticLevel::Error), 2);
    }

    #[test]
    fn detects_errors() {
        let report: DiagnosticReport = [
            Diagnostic::new(DiagnosticLevel::Warning, message("warning")),
            Diagnostic::new(DiagnosticLevel::Error, message("error")),
        ]
        .into_iter()
        .collect();

        assert!(report.has_errors());
    }

    #[test]
    fn detects_fatal_diagnostics() {
        let report: DiagnosticReport = [
            Diagnostic::new(DiagnosticLevel::Error, message("error")),
            Diagnostic::new(DiagnosticLevel::Fatal, message("fatal")),
        ]
        .into_iter()
        .collect();

        assert!(report.has_fatal());
    }

    #[test]
    fn returns_highest_severity() {
        let report: DiagnosticReport = [
            Diagnostic::new(DiagnosticLevel::Info, message("info")),
            Diagnostic::new(DiagnosticLevel::Warning, message("warning")),
            Diagnostic::new(DiagnosticLevel::Error, message("error")),
        ]
        .into_iter()
        .collect();

        assert_eq!(report.highest_severity(), Some(DiagnosticLevel::Error));
    }

    #[test]
    fn extends_report_in_order() {
        let mut report = DiagnosticReport::new();
        report.add(Diagnostic::new(DiagnosticLevel::Info, message("first")));

        let mut other = DiagnosticReport::new();
        other.add(
            Diagnostic::new(DiagnosticLevel::Warning, message("second"))
                .with_note(DiagnosticNote::new("extra context").expect("note should be valid")),
        );

        report.extend_report(other);

        let messages: Vec<&str> = report
            .iter()
            .map(|diagnostic| diagnostic.message().as_str())
            .collect();

        assert_eq!(messages, vec!["first", "second"]);
    }
}
