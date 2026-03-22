# Agent: Port Leaf Module (C → Rust)

**Model tier: Haiku** (confirmed working for all Layer 0 modules)

## When to Use
- Porting C modules with no internal dependencies (leaf nodes)
- Pure data tables, simple algorithms, standalone utilities
- Modules under ~500 lines of C

## Prompt Template

```
You are porting a C module to safe, idiomatic Rust.

1. Read the C source file at: {C_SOURCE_PATH}
2. Also read any header files it depends on: {HEADER_PATHS}

3. Write the Rust equivalent at: {RUST_OUTPUT_PATH}

Requirements:
- Zero `unsafe` code
- Idiomatic Rust (use enums, pattern matching, slices instead of pointers)
- Preserve exact semantics — every value, every edge case must match
- Add #[derive(Debug, Clone, Copy, PartialEq, Eq)] where appropriate
- Use const arrays/slices for static data

4. Add tests in a `#[cfg(test)] mod tests` block that verify correctness.
   If the C code has known test vectors or deterministic outputs, test against those.

5. Verify compilation:
   source "$HOME/.cargo/env" && cd {RUST_PROJECT_PATH} && cargo check

6. Run tests:
   source "$HOME/.cargo/env" && cd {RUST_PROJECT_PATH} && cargo test {MODULE_NAME}
```

## Variables
- `{C_SOURCE_PATH}` — absolute path to the C file being ported
- `{HEADER_PATHS}` — comma-separated list of header dependencies
- `{RUST_OUTPUT_PATH}` — absolute path for the output Rust file
- `{RUST_PROJECT_PATH}` — path to the Cargo project root
- `{MODULE_NAME}` — Rust module name (for test filtering)

## Example Invocation
See Phase 1.2 (SipHash) in process-log.md — Haiku produced correct, tested code first try.
