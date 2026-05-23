# use-diagnostic-label

Renderer-neutral diagnostic label primitives for `RustUse`.

## Foundation

`use-diagnostic-label` provides `DiagnosticLabelKind` and `DiagnosticLabel` for attaching plain-text context to diagnostics. Labels can point at spans, but they do not render source text, choose colors, or define terminal symbols.

## Example

```rust
use use_diagnostic_label::DiagnosticLabel;
use use_diagnostic_message::DiagnosticMessage;
use use_diagnostic_span::{DiagnosticPosition, DiagnosticSpan};

let start = DiagnosticPosition::new(2, 5).unwrap();
let end = DiagnosticPosition::new(2, 11).unwrap();
let span = DiagnosticSpan::without_source(start, end).unwrap();
let label = DiagnosticLabel::primary(
    DiagnosticMessage::new("value is missing").unwrap(),
    span,
);

assert!(label.span().is_some());
assert_eq!(label.to_string(), "value is missing");
```

## Scope

- Labels are context values attached to diagnostics.
- A label may have a span, but does not require one.
- Source rendering, colors, terminal symbols, and snippet formatting are out of scope.

## Status

`use-diagnostic-label` is a pre-1.0 crate with a deliberately small API.
