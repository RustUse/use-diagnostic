# use-diagnostic-span

Generic diagnostic source and span primitives for RustUse.

## Foundation

`use-diagnostic-span` provides `DiagnosticPosition`, `DiagnosticSourceId`, and `DiagnosticSpan` for identifying source locations without assuming files, snippets, renderers, or terminal output.

Positions are 1-based. A span may include a source ID for a file, buffer, document, or virtual source, but source IDs are plain stable strings rather than filesystem-only paths.

## Example

```rust
use use_diagnostic_span::{DiagnosticPosition, DiagnosticSourceId, DiagnosticSpan};

let source = DiagnosticSourceId::new("config.toml").unwrap();
let start = DiagnosticPosition::new(4, 9).unwrap();
let end = DiagnosticPosition::new(4, 16).unwrap();
let span = DiagnosticSpan::new(Some(source), start, end).unwrap();

assert_eq!(span.start().line(), 4);
assert!(span.source().is_some());
```

## Scope

- Positions use 1-based line and column numbers.
- Spans validate that the end is not before the start.
- Source IDs are generic stable strings.
- File reading, snippet extraction, and source rendering are out of scope.

## Status

`use-diagnostic-span` is a pre-1.0 crate with a deliberately small API.
