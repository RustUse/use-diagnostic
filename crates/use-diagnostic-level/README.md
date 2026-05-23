# use-diagnostic-level

Diagnostic severity primitives for `RustUse`.

## Foundation

`use-diagnostic-level` provides `DiagnosticLevel`, a small ordered severity enum with `Info`, `Warning`, `Error`, and `Fatal`.

More severe levels compare greater than less severe levels. The crate does not attach colors, terminal behavior, logging behavior, or exit-code behavior.

## Example

```rust
use use_diagnostic_level::DiagnosticLevel;

let level: DiagnosticLevel = "warn".parse().unwrap();

assert_eq!(level, DiagnosticLevel::Warning);
assert!(DiagnosticLevel::Fatal > DiagnosticLevel::Error);
```

## Scope

- Levels are severity values only.
- Parsing accepts common lowercase aliases.
- Rendering, logging, process exits, and terminal color are out of scope.

## Status

`use-diagnostic-level` is a pre-1.0 crate with a deliberately small API.
