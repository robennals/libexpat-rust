# expat-rust

A memory-safe, idiomatic Rust reimplementation of [libexpat](https://github.com/libexpat/libexpat) — the most widely deployed XML parser in the world.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust: 1.70+](https://img.shields.io/badge/rust-1.70%2B-orange.svg)]()

## What is this?

`expat-rust` is a complete rewrite of libexpat in safe Rust. It is **not** a wrapper or binding — it is a from-scratch reimplementation that reproduces libexpat's behavior, verified by 350+ automated comparison tests that run identical inputs through both parsers and compare outputs byte-for-byte.

Like libexpat, it is a **streaming SAX-style XML parser**: you register callback handlers, feed the parser chunks of XML data, and your handlers are called as the parser encounters elements, text, processing instructions, and other XML structures.

## Why?

**libexpat** is embedded in Python, Apache, Firefox, Git, and countless other projects. It has an excellent track record, but as a C library, it has historically been subject to memory safety vulnerabilities ([CVE-2022-25235](https://nvd.nist.gov/vuln/detail/CVE-2022-25235), [CVE-2022-25236](https://nvd.nist.gov/vuln/detail/CVE-2022-25236), [CVE-2022-40674](https://nvd.nist.gov/vuln/detail/CVE-2022-40674), among others).

`expat-rust` provides:

- **Memory safety**: Zero `unsafe` blocks. Buffer overflows, use-after-free, and double-free bugs are structurally impossible.
- **Behavioral compatibility**: Verified against libexpat 2.7.5 with 350+ comparison tests covering normal parsing, error handling, encodings, DTDs, namespaces, and security limits.
- **Familiar API**: The same SAX callback model that libexpat users know, expressed in idiomatic Rust.
- **DoS protection**: Built-in billion laughs attack protection, matching libexpat's security features.

## Status

**Verified against libexpat 2.7.5.** The full SAX callback API is implemented: element handlers, character data, processing instructions, comments, CDATA sections, DTD declarations, namespace processing, entity handling, and XML declaration handling.

| Metric | Value |
|--------|-------|
| Comparison tests passing | 350+ |
| Unit tests | 55 |
| Lines of Rust | ~8,500 |
| `unsafe` blocks | 0 |
| Minimum Rust version | 1.70 |

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

## How It Was Built

This port was created using an AI-assisted methodology with rigorous automated verification:

1. **Architecture analysis**: The C codebase (~9,200 lines) was analyzed to map module boundaries, data flows, and state machine transitions.

2. **Bottom-up porting**: Starting from leaf modules (ASCII tables, character classification, SipHash) and working up through the tokenizer, role state machine, and finally the main parser — ensuring each layer was solid before building on it.

3. **Structural verification**: A custom AST comparison tool verified that each Rust function structurally matched its C counterpart — same switch/match arms, same error paths, same handler calls.

4. **Behavioral verification**: 350+ FFI comparison tests run identical XML inputs through both the C library and the Rust port, comparing every output (handler calls, error codes, byte positions) for exact equivalence.

5. **C2Rust reference**: The C2Rust transpiler produced a mechanically-correct (but unsafe) Rust translation, used as a reference when C behavior was ambiguous due to preprocessor macros or implicit conversions.

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

Benchmarks comparing `expat-rust` against C libexpat 2.7.5 (Apple M-series, `cargo bench`):

| Scenario | expat-rust | libexpat (C) | Ratio |
|----------|-----------|-------------|-------|
| Small document (44 B) | 1.25 us | 2.48 us | **0.50x (Rust 2x faster)** |
| Medium document (~10 KB) | 171 us | 79 us | 2.2x |
| Large document (~100 KB) | 1.65 ms | 898 us | 1.8x |
| Deep nesting (100 levels) | 7.1 us | 20.8 us | **0.34x (Rust 2.9x faster)** |
| Many attributes (25/elem) | 35 us | 17.9 us | 2.0x |
| Error detection | 416 ns | 986 ns | **0.42x (Rust 2.4x faster)** |

**Summary**: Rust is 2-3x faster on small documents, deeply nested structures, and error detection. C is ~2x faster on larger documents with many elements and attributes. The gap on larger documents is due to Rust's use of standard `String`/`Vec`/`HashMap` (with per-element allocation) versus C's pooled arena allocator. This is a deliberate trade-off: we chose memory safety and idiomatic Rust data structures over matching C's allocation performance.

For most real-world use cases, the performance difference is negligible — both parsers process typical XML documents in microseconds.

Run benchmarks yourself:

```bash
cargo bench -p expat-rust
```

## Repository Structure

```
.
├── expat-rust/       The main Rust crate
├── expat-ffi/        C-compatible FFI wrapper (produces libexpat.so/.dylib/.dll)
├── expat-sys/        FFI bindings to C libexpat (for comparison testing only)
├── expat/            Git submodule — upstream libexpat at R_2_7_5
├── meta/             Porting process artifacts (tooling, plans, analysis)
├── docs/             Detailed documentation
└── Cargo.toml        Workspace root
```

## License

MIT — the same license as libexpat.

See [LICENSE](LICENSE) for the full text.
