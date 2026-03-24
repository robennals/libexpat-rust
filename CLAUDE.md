# expat-rust

A memory-safe, idiomatic Rust reimplementation of libexpat.

## Project Structure

```
expat-rust/       Main Rust crate (the parser)
expat-ffi/        C-compatible FFI layer — drop-in libexpat replacement
expat-sys/        FFI bindings to C libexpat (for comparison testing)
c-tests-runner/   Runs the original C test suite against our Rust parser
expat/            Git submodule — upstream libexpat at R_2_7_5
meta/             Porting process artifacts (scripts, plans, agent prompts)
docs/             Documentation (architecture, design decisions, porting process)
```

## Building and Testing

```bash
# Build everything
cargo build

# Build just the parser
cargo build -p expat-rust
```

Always use `RUST_TEST_THREADS=1` to avoid resource contention.

### Test Suites

There are four distinct test suites, each testing different aspects:

#### 1. Pure Rust Unit Tests (`expat-rust`)
Hand-written Rust tests exercising the parser's public API directly.
```bash
RUST_TEST_THREADS=1 cargo test -p expat-rust
```

#### 2. FFI Comparison Tests (`expat-rust` integration tests)
Parse the same XML with both the Rust parser and C libexpat, compare
(status, error_code). Catches behavioral divergences for ~399 inputs.
Requires a C compiler (builds C libexpat via `expat-sys`).
```bash
RUST_TEST_THREADS=1 cargo test -p expat-rust --test c_comparison_tests
RUST_TEST_THREADS=1 cargo test -p expat-rust --test comprehensive_comparison
RUST_TEST_THREADS=1 cargo test -p expat-rust --test generated_comparison_tests
```

#### 3. C Test Suite via FFI (`c-tests-runner`)
Compiles the **original C libexpat test suite** (basic_tests.c, ns_tests.c,
misc_tests.c, acc_tests.c — 303 tests excluding alloc tests) and links
them against `expat-ffi` (our Rust parser's C API). This is the most
comprehensive test — it verifies not just parse status but handler callback
data, error positions, encoding handling, external entities, and more.
```bash
# Build the test runner
cargo build -p c-tests-runner

# Run (uses lldb to handle C assert() aborts gracefully):
lldb -b -o "run" -o "quit" ./target/debug/c-tests-runner

# Count results:
lldb -b -o "run" -o "quit" ./target/debug/c-tests-runner 2>&1 | grep -c "^PASS:"
lldb -b -o "run" -o "quit" ./target/debug/c-tests-runner 2>&1 | grep -c "^FAIL "
```

Current status: **173 pass, 117 fail** of 290 tests reached (60% pass rate).
No longer needs `lldb` — custom `assert.h` converts C asserts to test failures.
```bash
cargo build -p c-tests-runner
./target/debug/c-tests-runner 2>/dev/null | grep -c "^PASS:"
./target/debug/c-tests-runner 2>/dev/null | grep -c "^FAIL "
```

See [docs/c-test-status.md](docs/c-test-status.md) for detailed failure analysis.

#### 4. expat-ffi C Integration Tests
Small standalone C test file verifying the FFI layer works from C.
```bash
cd expat-ffi/tests && make test
```

### Running a Single Test
```bash
RUST_TEST_THREADS=1 cargo test -p expat-rust --test basic_tests_0 test_name -- --exact
```

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

expat-ffi/src/
  lib.rs           C ABI shim — wraps Parser with extern "C" functions
                   matching the libexpat API (74 functions)
```

Parse flow: `parse()` → `run_processor()` → processor (prolog/content/epilog) → tokenizer → handlers

See [docs/architecture.md](docs/architecture.md) for details.

## Key Rules

1. **Match C behavior exactly** — The C library's actual behavior (via FFI comparison tests and C test suite) is ground truth
2. **Never edit xmlparse.rs with parallel agents** — They clobber each other's changes. Run Haiku agents **sequentially in the working branch**, not in parallel worktrees. Each agent should build on the previous one's committed work. Worktrees cause merge conflicts and agents working off stale code.
3. **Zero `unsafe`** — No unsafe blocks anywhere in expat-rust
4. **Use Rust standard library types** — `String`/`Vec`/`HashMap`, not C-style pools or hash tables
5. **Opus coordinates, Haiku implements** — Opus should set up tooling, generate prompts (via `ast-compare.py --prompt`), and review. Haiku agents do the actual code writing, one subsystem at a time.
6. **Match C architecture unless there's a specific Rust reason to diverge** — Function signatures, processor patterns, data flow should mirror C. Diverge only for safety (no unsafe) or Rust types (HashMap vs hash table). Don't "Rustify" the architecture in ways that break behavioral equivalence. E.g., processors take `(data, start, end) -> (error, next_pos)` matching C's `processor(parser, start, end, &endPtr)`.
7. **Never do workarounds — always port the C function** — When a test fails because a C function is missing, port that function 1:1. Don't try custom approaches. Use `--missing-functions` to find what's needed, extract the C source, port it line-by-line, verify with `--prompt`.

## Principled Test Skips

These C tests don't apply to the Rust port and are intentionally skipped:

| Test | Reason |
|------|--------|
| `test_accounting_precision` | Tests C-specific byte counting for DoS protection (`XML_ACCOUNT_*` macros). Rust uses different memory model. |
| `test_amplification_isolated_external_parser` | Tests C amplification limit thresholds. Not applicable to Rust's allocation model. |
| `test_billion_laughs_attack_protection_api` | Tests `XML_SetBillionLaughsAttackProtection*` API which uses C-specific byte accounting. |
| `test_misc_alloc_create_parser` | Tests custom `malloc`/`realloc`/`free` hooks via `XML_ParserCreate_MM`. Rust uses its own allocator. |
| `test_misc_alloc_create_parser_with_encoding` | Same as above but with encoding parameter. |

## expat-ffi Notes

The FFI layer (`expat-ffi`) has two critical design requirements:

1. **`user_data` must be the first field** of `ParserHandle` (`#[repr(C)]`) because the C macro `XML_GetUserData(parser)` reads `*(void**)(parser)` directly.

2. **Handler closures must read `user_data` at call time** (via `(*parser_ptr).user_data`), not capture it at registration time. C code commonly calls `XML_SetUserData()` after `XML_SetElementHandler()`.

## Porting Tools (in meta/)

The `meta/` directory contains the tooling used during the C-to-Rust porting process.
See [meta/README.md](meta/README.md) for a complete guide to each tool.

Key tools:
```bash
# AST structural comparison (run from project root)
python3 meta/scripts/ast-compare.py --all
python3 meta/scripts/ast-compare.py doContent do_content

# Generate Haiku-ready porting prompts with C source for missing cases
python3 meta/scripts/ast-compare.py --prompt doContent do_content
python3 meta/scripts/ast-compare.py --prompt-all

# Porting status and call tree
python3 meta/scripts/port-function.py ready
python3 meta/scripts/port-function.py analyze

# Extract C function source for reference
python3 meta/scripts/port-function.py extract storeAtts
```

## Skills (in .claude/skills/)

- **port-c-function.md** — Step-by-step process for porting a C function to Rust
- **fix-failing-test.md** — How to diagnose and fix a failing C test
- **verify-port-quality.md** — Checklist for verifying port quality after changes
- **c-to-rust-idioms.md** — All approved design decisions for C→Rust translation

## Function Mapping (in meta/plans/function-mapping.md)

Complete mapping of all 168 C functions to their Rust equivalents, documenting:
- Which functions exist in Rust (48 matched)
- Which are in the FFI layer (69)
- Which are C-specific and not needed (27)
- Which are truly missing and need porting (24)

## Agent Workflow

The proven workflow for implementing missing features:

1. **Identify divergences**: `python3 meta/scripts/ast-compare.py --all`
2. **Generate prompt**: `python3 meta/scripts/ast-compare.py --prompt <c_func> <rust_func>`
3. **Dispatch Haiku agent** sequentially (not in parallel) in the working branch
4. **Verify**: `cargo build && ./target/debug/c-tests-runner 2>/dev/null | grep -c "^PASS:"`
5. **Commit**: Include test count in commit message
6. **Repeat**: Pick next divergence
