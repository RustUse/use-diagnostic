# use-diagnostic

Facade crate for `RustUse` diagnostic primitives.

`use-diagnostic` reexports the focused crates in the diagnostic set. It is a primitive vocabulary for codes, levels, messages, spans, labels, diagnostics, and reports.

It is not an error framework, logging framework, terminal renderer, CLI output system, or compiler diagnostic system.

## Reexports

- `use_diagnostic_code`
- `use_diagnostic_level`
- `use_diagnostic_message`
- `use_diagnostic_span`
- `use_diagnostic_label`
- `use_diagnostic_report`

## Example

```rust
use use_diagnostic::prelude::{
    Diagnostic, DiagnosticCode, DiagnosticLabel, DiagnosticLevel, DiagnosticMessage,
    DiagnosticPosition, DiagnosticReport, DiagnosticSpan,
};

let code = DiagnosticCode::new("DATA.INVALID_SHAPE").unwrap();
let message = DiagnosticMessage::new("invalid data shape").unwrap();
let start = DiagnosticPosition::new(8, 3).unwrap();
let end = DiagnosticPosition::new(8, 11).unwrap();
let span = DiagnosticSpan::without_source(start, end).unwrap();
let label = DiagnosticLabel::primary(
    DiagnosticMessage::new("expected an object here").unwrap(),
    span,
);

let diagnostic = Diagnostic::new(DiagnosticLevel::Error, message)
    .with_code(code)
    .with_label(label);

let mut report = DiagnosticReport::new();
report.add(diagnostic);

assert!(report.has_errors());
```

## Scope

The facade keeps implementation minimal. It provides reexports and a shared prelude only.

## Status

`use-diagnostic` is a pre-1.0 facade for the focused diagnostic primitive crates.
