# libexpat C-to-Rust Port

This project ports libexpat's XML parser from C to idiomatic Rust.
The C source is in `expat/lib/xmlparse.c`, the Rust port is in `expat-rust/src/xmlparse.rs`.

## Critical Rules

1. **Match C behavior exactly** — When in doubt, the compiled C library's actual behavior (tested via `expat-sys` FFI) is ground truth. The C source can mislead due to preprocessor macros, implicit conversions, and platform-specific behavior. Run comparison tests to settle ambiguities.
2. **Never edit xmlparse.rs with parallel agents** — They clobber each other's changes
3. **Always run tests with `RUST_TEST_THREADS=1`** to avoid resource contention
4. **Run tests from `expat-rust/` directory** — `cd expat-rust && RUST_TEST_THREADS=1 cargo test`
5. **Don't port C data structures** (pools, hash tables) — use Rust's `String`/`Vec`/`HashMap`
6. **Bottom-up porting** — only port a function after all functions it calls are solid
7. **AST comparison drives the work** — run `python3 scripts/ast-compare.py --all` to find divergences, don't guess

## Architecture

```
expat-rust/src/
  xmlparse.rs    — Main parser (porting from xmlparse.c)
  xmltok.rs      — Token types, encoding, XML declaration parsing
  xmltok_impl.rs — Tokenizer (content_tok, prolog_tok)
  xmlrole.rs     — Prolog role state machine
  siphash.rs     — Hash function
  char_tables.rs — Character classification
  nametab.rs     — Name character tables
```

Parse flow: `parse()` → `run_processor()` → processor (prolog/content/epilog) → tokenizer → handlers

## Key Tools

### AST Structural Comparison (run from project root)
```bash
python3 scripts/ast-compare.py --all                    # All divergences
python3 scripts/ast-compare.py doContent do_content      # One function pair
python3 scripts/ast-compare.py --list-cases doContent c  # List C switch cases
```

### Porting Status & Call Tree
```bash
python3 scripts/port-function.py ready      # Functions ready to port
python3 scripts/port-function.py analyze    # Call tree status
python3 scripts/port-function.py extract X  # Extract C function
python3 scripts/port-function.py prompt X   # Agent prompt for porting
```

### Tests
```bash
cd expat-rust && RUST_TEST_THREADS=1 cargo test                          # All tests
RUST_TEST_THREADS=1 cargo test --test c_comparison_tests                  # C comparison
RUST_TEST_THREADS=1 cargo test --test basic_tests_0 test_name -- --exact  # Single test
```

## Porting Workflow

1. Run `python3 scripts/ast-compare.py --all` to see divergences
2. Pick highest-impact divergence
3. Read the C source: `python3 scripts/port-function.py extract <c_func>`
4. Fix the Rust code to match C structurally
5. Verify: `python3 scripts/ast-compare.py <c_func> <rust_func>`
6. Run tests, confirm count went up
7. Commit with test count in message

## Sub-Agent Guidelines

When delegating to sub-agents:
- Give them the C function source and the current Rust function source
- Tell them which specific divergences to fix (from ast-compare output)
- They should NOT run ast-compare themselves — give them the output
- They should verify with `cargo check` and a targeted test
- Only one agent should edit xmlparse.rs at a time
- Each agent should fix one divergence category, not multiple

## Current Status

See `plans/porting-status.md` for function-level status.
Run `python3 scripts/ast-compare.py --all` for current divergences.
