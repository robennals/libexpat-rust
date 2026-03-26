# expat-rust

A memory-safe, idiomatic Rust reimplementation of [libexpat](https://github.com/libexpat/libexpat) — the most widely deployed XML parser in the world.

[![CI](https://github.com/robennals/libexpat-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/robennals/libexpat-rust/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust: 1.70+](https://img.shields.io/badge/rust-1.70%2B-orange.svg)]()

## What is this?

`expat-rust` is a complete rewrite of libexpat in safe Rust. It is **not** a wrapper or binding — it is a from-scratch reimplementation that reproduces libexpat's behavior exactly. It passes **all 290 of libexpat's own C tests** (except 5 that test C-specific allocation APIs), and is additionally verified by **463 comparison tests** that run identical XML inputs through both the C and Rust parsers and confirm identical SAX callback sequences, error codes, and parse results.

Like libexpat, it is a **streaming SAX-style XML parser**: you register callback handlers, feed the parser chunks of XML data, and your handlers are called as the parser encounters elements, text, processing instructions, and other XML structures.

## Why?

**libexpat** is embedded in Python, Apache, Firefox, Git, and countless other projects. It has an excellent track record, but as a C library, it has historically been subject to memory safety vulnerabilities ([CVE-2022-25235](https://nvd.nist.gov/vuln/detail/CVE-2022-25235), [CVE-2022-25236](https://nvd.nist.gov/vuln/detail/CVE-2022-25236), [CVE-2022-40674](https://nvd.nist.gov/vuln/detail/CVE-2022-40674), among others).

`expat-rust` provides:

- **Memory safety**: Zero `unsafe` blocks. Buffer overflows, use-after-free, and double-free bugs are structurally impossible.
- **Behavioral compatibility**: Passes all of libexpat 2.7.5's own test suite (285/290 tests; 5 skipped test C-specific allocator APIs). Additionally verified by [463 comparison tests](docs/verification.md) that confirm identical SAX event traces across normal parsing, error handling, encodings, DTDs, namespaces, and security limits.
- **Familiar API**: The same SAX callback model that libexpat users know, expressed in idiomatic Rust.
- **C drop-in replacement**: The `expat-ffi` crate produces a `libexpat.so`/`.dylib`/`.dll` with the same C ABI — swap it into existing C/C++ applications without changing a line of code.
- **Zero dependencies**: The core parser has no production dependencies.
- **DoS protection**: Built-in billion laughs attack protection, matching libexpat's security features.

## Status

**Verified against libexpat 2.7.5.** The full SAX callback API is implemented: element handlers, character data, processing instructions, comments, CDATA sections, DTD declarations, namespace processing, entity handling, and XML declaration handling.

| Metric | Value |
|--------|-------|
| Original C test suite | 285/290 pass (5 skipped: C-specific allocator APIs) |
| Additional comparison tests | 463 (identical XML through both C and Rust parsers) |
| Line coverage (reachable code) | 76% |
| Lines of Rust (core parser) | ~11,800 |
| `unsafe` blocks | 0 |
| Production dependencies | 0 |
| Minimum Rust version | 1.70 |

The original C test suite (`basic_tests.c`, `ns_tests.c`, `misc_tests.c`, `acc_tests.c` — 290 tests) is compiled and linked against the Rust parser's C-compatible FFI layer, verifying not just parse status but handler callback data, error positions, encoding handling, external entities, and more. The 5 skipped tests exercise C-specific custom allocator hooks (`XML_ParserCreate_MM`) which don't apply to Rust.

On top of that, 463 comparison tests run the same XML inputs through both the C library and the Rust port, comparing full SAX event traces (handler calls, error codes, attribute values, byte positions) for exact equivalence. See [docs/verification.md](docs/verification.md) for the full testing methodology.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
expat-rust = "0.1"
```

Basic parsing example:

```rust
use expat_rust::xmlparse::{Parser, XmlStatus};

let mut parser = Parser::new(None).expect("failed to create parser");

// Set up handlers
parser.set_start_element_handler(Some(|name, attrs| {
    println!("Start: {}", name);
    for attr in attrs.chunks(2) {
        println!("  {} = {}", attr[0], attr[1]);
    }
}));

parser.set_end_element_handler(Some(|name| {
    println!("End: {}", name);
}));

parser.set_character_data_handler(Some(|text| {
    println!("Text: {}", text);
}));

// Parse XML
let xml = b"<root><child attr=\"value\">Hello, world!</child></root>";
let status = parser.parse(xml, true);
assert_eq!(status, XmlStatus::Ok);
```

## Safety

`expat-rust` contains **zero `unsafe` blocks**. All memory management uses Rust's ownership system:

- C string pools are replaced with `String` and `Vec<u8>`
- C hash tables are replaced with `HashMap`
- C manual memory management is replaced with Rust's RAII
- C function pointers for state machines are replaced with Rust enums

The parser cannot produce memory safety violations regardless of input — malformed, malicious, or otherwise.

The FFI layer (`expat-ffi`) necessarily uses `unsafe` for the C ABI boundary, but all unsafety is confined there — the core parser is entirely safe Rust.

## How It Was Built

This port was created using an AI-assisted methodology with rigorous automated verification:

1. **Architecture analysis**: The C codebase (~9,200 lines) was analyzed to map module boundaries, data flows, and state machine transitions.

2. **Bottom-up porting**: Starting from leaf modules (ASCII tables, character classification, SipHash) and working up through the tokenizer, role state machine, and finally the main parser — ensuring each layer was solid before building on it.

3. **Structural verification**: A custom AST comparison tool verified that each Rust function structurally matched its C counterpart — same switch/match arms, same error paths, same handler calls.

4. **Original test suite**: The 290 tests from libexpat's own C test suite (`basic_tests.c`, `ns_tests.c`, `misc_tests.c`, `acc_tests.c`) are compiled and linked against the Rust parser's C FFI layer — 285 pass, with 5 skipped for testing C-specific allocator APIs.

5. **Behavioral verification**: 463 additional FFI comparison tests run identical XML inputs through both the C library and the Rust port, comparing full SAX event traces (handler calls, error codes, attribute values) for exact equivalence. See [docs/verification.md](docs/verification.md) for the detailed methodology.

6. **C2Rust reference**: The C2Rust transpiler produced a mechanically-correct (but unsafe) Rust translation, used as a reference when C behavior was ambiguous due to preprocessor macros or implicit conversions.

For the full story, see [docs/porting-process.md](docs/porting-process.md). The complete porting tooling is preserved in [`meta/`](meta/).

## Architecture

```
expat-rust/src/
  xmlparse.rs      Main parser — public API, SAX callbacks, state machine
  xmltok.rs        Token types, encoding detection, XML declaration parsing
  xmltok_impl.rs   Tokenizer — lexes XML into tokens (content_tok, prolog_tok)
  xmlrole.rs       Prolog role state machine — classifies DTD/prolog tokens
  siphash.rs       SipHash-2-4 — hash randomization for DoS protection
  char_tables.rs   Character classification tables
  nametab.rs       XML name character lookup tables
  ascii.rs         ASCII character constants
```

Parse flow: `parse()` → `run_processor()` → processor (prolog/content/epilog) → tokenizer → handlers

For detailed architecture documentation, see [docs/architecture.md](docs/architecture.md).

## Comparison with Alternatives

| Feature | expat-rust | quick-xml | xml-rs | roxmltree |
|---------|-----------|-----------|--------|-----------|
| API model | SAX (callbacks) | Pull / SAX | Pull | DOM (tree) |
| Streaming | Yes | Yes | Yes | No (loads entire doc) |
| DTD support | Yes | No | No | Partial |
| Namespace support | Yes | Yes | Yes | Yes |
| Entity expansion | Yes | Partial | No | Partial |
| libexpat compatible | Yes | No | No | No |
| `unsafe`-free | Yes | No | Yes | No |
| Zero dependencies | Yes | No | No | No |

`expat-rust` is the right choice when you need **libexpat behavioral compatibility**, **full DTD support**, or are replacing libexpat in an existing system.

## C Drop-in Replacement

Already have a C/C++ app that uses libexpat? You can swap in `expat-rust` without changing any code:

```bash
# 1. Clone and build
git clone --recurse-submodules https://github.com/robennals/libexpat-rust.git
cd libexpat-rust
cargo build --release -p expat-ffi

# 2. Run your app against the new library (Linux)
LD_LIBRARY_PATH=target/release ./your_app

# 2. Or on macOS
DYLD_LIBRARY_PATH=target/release ./your_app
```

That's it. The `expat-ffi` crate produces a `libexpat.so` / `libexpat.dylib` / `expat.dll` with the same function signatures as the real libexpat — `XML_ParserCreate`, `XML_Parse`, `XML_SetElementHandler`, and the rest. Your code doesn't need to change.

For a complete example of C code using the library, see [`expat-ffi/examples/basic_parse.c`](expat-ffi/examples/basic_parse.c). For detailed migration instructions, see [`expat-ffi/README.md`](expat-ffi/README.md).

## Performance

Benchmarks comparing `expat-rust` against C libexpat 2.7.5 (Apple M-series, `cargo bench` with LTO). Criterion.rs runs 100 samples per benchmark, each containing thousands of auto-calibrated iterations — sub-microsecond timings are the median of hundreds of thousands of runs.

| Scenario | expat-rust | libexpat (C) | Ratio |
|----------|-----------|-------------|-------|
| Small document (44 B) | 498 ns | 1.04 us | **0.48x (Rust 2x faster)** |
| Medium document (~10 KB) | 134 us | 81 us | 1.65x |
| 100 KB document | 1.38 ms | 904 us | 1.53x |
| 100 MB document | 438 ms | 278 ms | 1.58x |
| 100 MB streamed (8 KB chunks) | 428 ms | 345 ms | 1.24x |
| Deep nesting (100 levels) | 7.9 us | 21.0 us | **0.38x (Rust 2.7x faster)** |
| Many attributes (25/elem) | 23.1 us | 17.9 us | 1.29x |
| Error detection | 445 ns | 1.01 us | **0.44x (Rust 2.3x faster)** |

**Summary**: Rust is significantly faster for small documents, deep nesting, and error detection. C is 1.2-1.6x faster on larger element-heavy documents. The gap comes from Rust's use of standard `String`/`Vec`/`HashMap` (with per-element allocation) versus C's pooled arena allocator. This is a deliberate trade-off: we chose memory safety and idiomatic Rust data structures over matching C's allocation strategy. In streaming mode (how expat is designed to be used), Rust is within 24% of C.

**Memory**: Both parsers stream with O(1) memory. In chunked mode (8 KB chunks), both use ~33 KB regardless of total document size (memory does scale with nesting depth, but not with document length). Rust is within 7% of C in streaming memory usage. See [docs/benchmarks.md](docs/benchmarks.md) for full memory analysis.

Run benchmarks yourself:

```bash
cargo bench -p expat-rust
```

## Repository Structure

```
.
├── expat-rust/       The main Rust crate (zero dependencies, zero unsafe)
├── expat-ffi/        C-compatible FFI wrapper (produces libexpat.so/.dylib/.dll)
├── expat-sys/        FFI bindings to C libexpat (for comparison testing only)
├── c-tests-runner/   Runs the original C test suite against the Rust parser
├── expat/            Git submodule — upstream libexpat at R_2_7_5
├── meta/             Porting process artifacts (tooling, plans, analysis)
├── docs/             Detailed documentation
└── Cargo.toml        Workspace root
```

## License

MIT — the same license as libexpat.

See [LICENSE](LICENSE) for the full text.
