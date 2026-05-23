# use-diagnostic-report

Simple diagnostic and diagnostic report primitives for `RustUse`.

## Foundation

`use-diagnostic-report` provides `Diagnostic` and `DiagnosticReport`. A diagnostic combines an optional code, severity level, message, labels, and notes. A report stores diagnostics in insertion order and exposes small query helpers.

Reports do not render output, log messages, install global reporters, or replace `std::error::Error`.

## Example

```rust
use use_diagnostic_label::DiagnosticLabel;
use use_diagnostic_level::DiagnosticLevel;
use use_diagnostic_message::DiagnosticMessage;
use use_diagnostic_report::{Diagnostic, DiagnosticReport};

let diagnostic = Diagnostic::new(
    DiagnosticLevel::Error,
    DiagnosticMessage::new("missing required field").unwrap(),
)
.with_label(DiagnosticLabel::help(
    DiagnosticMessage::new("add the field before retrying").unwrap(),
));

let mut report = DiagnosticReport::new();
report.add(diagnostic);

assert_eq!(report.count_by_level(DiagnosticLevel::Error), 1);
assert!(report.has_errors());
```

## Scope

- Diagnostics are data values.
- Reports store diagnostics in insertion order.
- Query helpers count levels and inspect severity.
- Rendering, terminal output, logging, panic hooks, and global reporters are out of scope.

## Status

`use-diagnostic-report` is a pre-1.0 crate with a deliberately small API.
