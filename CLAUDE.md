# expat-rust

A memory-safe, idiomatic Rust reimplementation of libexpat.

## Project Structure

```
expat-rust/       Main Rust crate (the parser)
expat-sys/        FFI bindings to C libexpat (for comparison testing)
expat/            Git submodule — upstream libexpat at R_2_7_5
meta/             Porting process artifacts (scripts, plans, agent prompts)
docs/             Documentation (architecture, design decisions, porting process)
```

## Building and Testing

```bash
# Build
cargo build -p expat-rust

# Run pure Rust tests (no C compiler needed)
RUST_TEST_THREADS=1 cargo test -p expat-rust

# Run FFI comparison tests (requires C compiler for expat-sys)
RUST_TEST_THREADS=1 cargo test -p expat-rust --test c_comparison_tests
RUST_TEST_THREADS=1 cargo test -p expat-rust --test comprehensive_comparison
RUST_TEST_THREADS=1 cargo test -p expat-rust --test generated_comparison_tests

# Run a single test
RUST_TEST_THREADS=1 cargo test -p expat-rust --test basic_tests_0 test_name -- --exact
```

Always use `RUST_TEST_THREADS=1` to avoid resource contention.

## Architecture

```
expat-rust/src/
  xmlparse.rs      Main parser — public API, SAX callbacks, state machine
  xmltok.rs        Token types, encoding detection, XML declaration parsing
  xmltok_impl.rs   Tokenizer — content_tok, prolog_tok
  xmlrole.rs       Prolog role state machine
  siphash.rs       SipHash-2-4 hash function
  char_tables.rs   Character classification tables
  nametab.rs       Name character lookup tables
  ascii.rs         ASCII character constants
```

Parse flow: `parse()` → `run_processor()` → processor (prolog/content/epilog) → tokenizer → handlers

See [docs/architecture.md](docs/architecture.md) for details.

## Key Rules

1. **Match C behavior exactly** — The C library's actual behavior (via FFI comparison tests) is ground truth
2. **Never edit xmlparse.rs with parallel agents** — They clobber each other's changes
3. **Zero `unsafe`** — No unsafe blocks anywhere in expat-rust
4. **Use Rust standard library types** — `String`/`Vec`/`HashMap`, not C-style pools or hash tables

## Porting Tools (in meta/)

The `meta/` directory contains the tooling used during the C-to-Rust porting process.
See [meta/README.md](meta/README.md) for a complete guide to each tool.

Key tools:
```bash
# AST structural comparison (run from project root)
python3 meta/scripts/ast-compare.py --all
python3 meta/scripts/ast-compare.py doContent do_content

# Porting status and call tree
python3 meta/scripts/port-function.py ready
python3 meta/scripts/port-function.py analyze
```
