# Agent: Port Data Tables (C → Rust)

**Model tier: Haiku** (confirmed working)

## When to Use
- Porting C lookup tables, character classification arrays, bitmap tables
- Pure data with no logic — just arrays of constants
- Can also write a Python script to do the conversion if tables are large/repetitive

## Prompt Template

```
You are porting C data tables to Rust.

1. Read the C source files at: {C_SOURCE_PATHS}
2. These files contain static arrays/tables of values using {CONSTANT_PREFIX}_* constants.

3. You have two approaches:
   a. If tables are small and simple: write Rust directly
   b. If tables are large/repetitive: write a Python script at {SCRIPT_PATH} that
      parses the C files and generates the Rust code, then run it

4. Output Rust file at: {RUST_OUTPUT_PATH}

Requirements:
- Define a Rust enum for the constant types with #[repr(u8)]
- Use const arrays (not lazy_static or similar)
- Exact 1:1 mapping of every value from C to Rust
- Add #[derive(Debug, Clone, Copy, PartialEq, Eq)] on the enum

5. Verify: source "$HOME/.cargo/env" && cd {RUST_PROJECT_PATH} && cargo check
```

## Key Insight
Haiku naturally chose to write a Python script for the character tables (good instinct — reproducible, debuggable). For smaller tables like nametab, it directly wrote Rust. Let it choose.
