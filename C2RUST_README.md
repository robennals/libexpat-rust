# C2Rust Pipeline for libexpat Rust Port

## Overview

This directory contains a C2Rust transpilation pipeline that provides:
1. **Mechanically correct Rust** generated from the C source (reference for correctness)
2. **Comparison testing** against the real C library (catches behavioral divergences)
3. **Transformation scripts** for converting C2Rust output to idiomatic Rust
4. **Agent prompts** for haiku-based function transformation

## Directory Structure

```
port/c2rust/
├── expat/              # Original C libexpat source
├── expat-rust/         # Idiomatic Rust port (the main port)
│   └── tests/
│       └── c_comparison_tests.rs  # Tests comparing Rust vs C behavior
├── expat-sys/          # FFI bindings to C library (for reference testing)
├── c2rust-output/      # C2Rust-generated Rust (mechanically correct, unsafe)
├── scripts/
│   ├── c2rust-pipeline.sh          # Main orchestrator script
│   ├── c2rust-cleanup.py           # Phase 1: Mechanical type/syntax cleanup
│   ├── c2rust-analyze-patterns.py  # Analyze patterns in C2Rust output
│   ├── extract-c2rust-functions.py # Extract/compare functions
│   ├── transform-function.py       # Prepare functions for transformation
│   └── c2rust-to-idiomatic.md      # Transformation rules guide
└── agents/
    └── c2rust-transform.md         # Haiku agent prompt for function transformation
```

## Quick Start

```bash
# Run comparison tests
./scripts/c2rust-pipeline.sh compare

# Analyze C2Rust patterns
./scripts/c2rust-pipeline.sh analyze

# Compare functions between C2Rust output and existing port
./scripts/c2rust-pipeline.sh functions

# Extract a function for manual transformation
./scripts/c2rust-pipeline.sh extract doContent --prompt
```

## Current Status

- **Comparison tests**: 56/59 passing (3 failures all DOCTYPE-related)
- **Existing tests**: 108 passing, 1 failing (DOCTYPE), ~160 ignored
- **C2Rust output**: All 3 files compile on nightly (xmlparse.rs, xmlrole.rs, xmltok.rs)

## Key Patterns in C2Rust Output (xmlparse.rs)

| Pattern | Count | Transformation |
|---------|-------|---------------|
| Raw pointer deref `(*parser).field` | 2,729 | → `self.field` methods |
| Type casts `as c_int` | 1,508 | → native Rust types |
| Function pointer calls | 120 | → trait/enum dispatch |
| Goto patterns (current_block) | 119 | → loop/match/early return |
| malloc/free | 24+42 | → Vec/Box |

## Transformation Workflow

1. **Find mismatch**: Run comparison tests, identify failing case
2. **Reference C2Rust**: Look at the mechanically correct version in c2rust-output/
3. **Fix Rust port**: Update expat-rust/ to match C behavior
4. **Verify**: Run comparison tests to confirm fix

For new functions:
1. **Extract**: `./scripts/c2rust-pipeline.sh extract functionName --prompt`
2. **Transform**: Use haiku agent with agents/c2rust-transform.md prompt
3. **Integrate**: Add to expat-rust/src/xmlparse.rs
4. **Test**: Run comparison tests

## Requirements

- C2Rust: `LLVM_LIB_DIR=/opt/homebrew/opt/llvm@17/lib cargo install c2rust`
- LLVM 17: `brew install llvm@17`
- CMake: `brew install cmake`
- Rust nightly (for c2rust-output only): `rustup toolchain install nightly`
