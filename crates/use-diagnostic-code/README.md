# use-diagnostic-code

Stable diagnostic identifier primitives for RustUse.

## Foundation

`use-diagnostic-code` provides `DiagnosticCode`, a small string-backed identifier for stable diagnostic names such as `CONFIG001`, `VALIDATE_MISSING_FIELD`, and `DATA.INVALID_SHAPE`.

Codes are trimmed at the edges, reject empty values, and preserve internal characters.

## Example

```rust
use use_diagnostic_code::DiagnosticCode;

let code = DiagnosticCode::new(" VALIDATE_MISSING_FIELD ").unwrap();

assert_eq!(code.as_str(), "VALIDATE_MISSING_FIELD");
assert_eq!(code.to_string(), "VALIDATE_MISSING_FIELD");
```

## Scope

- Codes are stable string identifiers.
- The crate does not reserve prefixes or parse domain-specific code systems.
- Rendering, logging, and error handling are out of scope.

## Status

`use-diagnostic-code` is a pre-1.0 crate with a deliberately small API.
