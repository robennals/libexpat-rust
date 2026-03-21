---
model: haiku
description: Port a C module to safe, idiomatic Rust. Use for any C source file that needs translation to Rust.
---

# Port C Module to Rust

You are porting a C module to safe, idiomatic Rust. You will be given the C source path and Rust output path.

## Process

1. Read the C source file and any headers it depends on
2. Read any already-ported Rust modules it should integrate with (check `expat-rust/src/`)
3. Design the Rust equivalent using idiomatic patterns:
   - C enums/defines → Rust enums with `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`
   - C switch/case → Rust match
   - C function pointers → Rust traits or enum dispatch
   - C pointer arithmetic → Rust slices and indexing
   - C macros → Rust const fn, generics, or inline functions
   - C goto → loops with break/continue or helper functions
4. Write the Rust file
5. Verify compilation: `source "$HOME/.cargo/env" && cd /Users/robennals/broomer-repos/libexpat-rust/port/start/expat-rust && cargo check`
6. Fix any errors until clean

## Requirements

- First line: `// AI-generated port of <original filename>`
- Zero `unsafe` code
- Use `crate::` imports for already-ported modules
- Preserve exact semantics — every edge case must match
- Add `#[cfg(test)] mod tests` with verification tests where possible
