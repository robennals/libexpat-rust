# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in `expat-rust`, please report it privately via [GitHub Security Advisories](https://github.com/robennals/libexpat-rust/security/advisories/new).

Please provide:
- A description of the vulnerability and its impact
- Steps to reproduce the issue
- The XML input that triggers the vulnerability (if applicable)

We will respond within 7 days and aim to release a fix within 30 days of confirmation.

## Safety Guarantees

The core parser (`expat-rust`) contains **zero `unsafe` blocks** — enforced by `#![forbid(unsafe_code)]` at the crate root. This means:

- **No buffer overflows**: All buffer access is bounds-checked by the Rust compiler
- **No use-after-free**: Rust's ownership system prevents accessing freed memory
- **No double-free**: Resources are freed exactly once via RAII
- **No null pointer dereferences**: Rust's `Option` type eliminates null pointers
- **No data races**: Not applicable (single-threaded parser), but Rust would prevent them

## DoS Protection

The parser includes the same denial-of-service protections as libexpat:

- **Billion laughs attack protection**: Tracks the amplification ratio (bytes produced by entity expansion vs. bytes of direct input). When the ratio exceeds a configurable maximum (default 100x) and total output exceeds an activation threshold (default 8 MiB), parsing is aborted with `AmplificationLimitBreach`. This protection extends across external entity sub-parsers — parent and child parsers share the same accounting.
- **Recursive entity detection**: Prevents infinite loops from self-referencing entities (e.g., `&a;` defined as `&a;`)
- **Configurable limits**: `set_billion_laughs_attack_protection_maximum_amplification()` and `set_billion_laughs_attack_protection_activation_threshold()`

Both the billion laughs API tests and the amplification enforcement tests from the original C test suite pass against the Rust implementation.

The FFI layer (`expat-ffi`) necessarily uses `unsafe` for the C ABI boundary, but all unsafety is confined there — the core parser is entirely safe Rust.

## Scope

This security policy covers the `expat-rust` crate only. For vulnerabilities in the upstream C libexpat library, please report to the [libexpat project](https://github.com/libexpat/libexpat/security/policy).
