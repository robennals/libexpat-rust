---
model: haiku
description: Port C data tables and lookup arrays to Rust const arrays. Can also write Python scripts for mechanical generation.
---

# Port Data Tables

You are porting C data tables (lookup arrays, character classification, bitmaps) to Rust.

## Process

1. Read the C header files containing the tables
2. Choose approach:
   - Small/simple tables → write Rust directly
   - Large/repetitive tables → write a Python script to generate Rust
3. Output Rust const arrays with proper types
4. Verify compilation

## Requirements

- Use `pub const` arrays, not lazy_static
- Define enum types with `#[repr(u8)]` and derives
- Exact 1:1 mapping of all values
- If writing a Python script, put it in `scripts/`
- Verify: `cargo check`
