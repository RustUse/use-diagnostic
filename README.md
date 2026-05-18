# use-diagnostic

Composable sets of primitive Rust utility crates for fellow crustaceans.

`use-diagnostic` is a primitive diagnostic vocabulary set for RustUse. It provides small, reusable concepts for describing structured problems, warnings, validation issues, parse issues, configuration issues, and related notes.

This set is not an error framework, logging framework, renderer, CLI output system, compiler diagnostic system, or compiler frontend. It defines data primitives that other crates can compose without adopting a larger reporting model.

## Workspace crates

- `use-diagnostic`: facade crate for the full diagnostic vocabulary set
- `use-diagnostic-code`: stable diagnostic identifier primitives
- `use-diagnostic-level`: severity primitives
- `use-diagnostic-message`: human-facing plain-text message and note primitives
- `use-diagnostic-span`: generic source, position, and span primitives
- `use-diagnostic-label`: renderer-neutral labels attached to diagnostics
- `use-diagnostic-report`: simple insertion-order diagnostic collections and queries

## Example

```rust
use use_diagnostic::prelude::{
    Diagnostic, DiagnosticCode, DiagnosticLabel, DiagnosticLevel, DiagnosticMessage,
    DiagnosticPosition, DiagnosticReport, DiagnosticSpan,
};

let code = DiagnosticCode::new("VALIDATE_MISSING_FIELD").unwrap();
let message = DiagnosticMessage::new("missing required field").unwrap();
let start = DiagnosticPosition::new(3, 5).unwrap();
let end = DiagnosticPosition::new(3, 12).unwrap();
let span = DiagnosticSpan::without_source(start, end).unwrap();
let label = DiagnosticLabel::primary(
    DiagnosticMessage::new("field is required here").unwrap(),
    span,
);

let diagnostic = Diagnostic::new(DiagnosticLevel::Error, message)
    .with_code(code)
    .with_label(label);

let mut report = DiagnosticReport::new();
report.add(diagnostic);

assert!(report.has_errors());
```

## Related sets

Diagnostics are useful vocabulary for sibling and future RustUse sets, including:

- `use-validate`
- `use-config`
- `use-data`
- `use-encoding`
- `use-rust`
- `use-cli`

## Scope

- Codes are stable string identifiers.
- Levels describe severity only.
- Messages and notes are plain text.
- Spans are generic source ranges, not filesystem or snippet renderers.
- Labels attach context without colors, terminal symbols, or source rendering.
- Reports are simple collections and query helpers.

Out of scope: logging, terminal rendering, panic hooks, global reporters, async processing, compiler-specific diagnostics, and replacement behavior for `std::error::Error`.

## Status

This workspace is a pre-1.0 RustUse primitive set. The first version keeps the API intentionally small so the concepts can stay durable as related sets adopt them.
