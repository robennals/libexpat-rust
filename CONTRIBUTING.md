# Contributing to expat-rust

Thanks for your interest in contributing!

## Getting Started

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/robennals/libexpat-rust.git
cd libexpat-rust

# Build
cargo build -p expat-rust

# Run tests
RUST_TEST_THREADS=1 cargo test -p expat-rust
```

The `expat/` directory is a git submodule pointing to upstream libexpat. If you cloned without `--recurse-submodules`, run:

```bash
git submodule update --init
```

## Running Tests

```bash
# Pure Rust tests (no C compiler needed)
RUST_TEST_THREADS=1 cargo test -p expat-rust

# FFI comparison tests (requires C compiler)
RUST_TEST_THREADS=1 cargo test -p expat-rust --test c_comparison_tests
RUST_TEST_THREADS=1 cargo test -p expat-rust --test comprehensive_comparison
RUST_TEST_THREADS=1 cargo test -p expat-rust --test generated_comparison_tests

# All tests
RUST_TEST_THREADS=1 cargo test --workspace
```

Always use `RUST_TEST_THREADS=1` — some tests share state and will fail with parallel execution.

## Code Quality

Before submitting a PR, ensure:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo doc -p expat-rust --no-deps
```

## Key Principles

1. **Zero `unsafe`**: No `unsafe` blocks. If you think you need one, there's a safe alternative.
2. **Behavioral compatibility**: The parser must produce identical output to libexpat for all inputs. When in doubt, write a comparison test.
3. **Use Rust idioms**: `String`/`Vec`/`HashMap` instead of C-style data structures. Enums instead of function pointers. Pattern matching instead of switch/goto.

## Comparison Tests

The comparison test suites are the primary correctness guarantee. They run identical inputs through both the C library (via `expat-sys`) and the Rust port, comparing every output. If you fix a bug or add a feature, add a comparison test.

## Porting Tools

The `meta/` directory contains the tooling used during the original port. See [meta/README.md](meta/README.md) for details. Key tools:

- `meta/scripts/ast-compare.py` — Structural comparison between C and Rust functions
- `meta/scripts/port-function.py` — Call tree analysis and function extraction
