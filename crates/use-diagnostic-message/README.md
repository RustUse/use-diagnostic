# use-diagnostic-message

Plain-text diagnostic message primitives for `RustUse`.

## Foundation

`use-diagnostic-message` provides `DiagnosticMessage` and `DiagnosticNote`, small string-backed values for human-facing diagnostic text.

Messages and notes are trimmed at the edges, reject empty values, and remain renderer-neutral. They do not carry Markdown, ANSI, color, terminal, or layout assumptions.

## Example

```rust
use use_diagnostic_message::{DiagnosticMessage, DiagnosticNote};

let message = DiagnosticMessage::new(" missing required field ").unwrap();
let note = DiagnosticNote::new("field names are case-sensitive").unwrap();

assert_eq!(message.as_str(), "missing required field");
assert_eq!(note.to_string(), "field names are case-sensitive");
```

## Scope

- Messages and notes are plain text.
- The crate validates presence, not grammar or style.
- Formatting, colors, markup, and terminal rendering are out of scope.

## Status

`use-diagnostic-message` is a pre-1.0 crate with a deliberately small API.
