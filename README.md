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

## Repository Structure

```
.
├── expat-rust/       The main Rust crate
├── expat-sys/        FFI bindings to C libexpat (for comparison testing only)
├── expat/            Git submodule — upstream libexpat at R_2_7_5
├── meta/             Porting process artifacts (tooling, plans, analysis)
├── docs/             Detailed documentation
└── Cargo.toml        Workspace root
```

## License

MIT — the same license as libexpat.

See [LICENSE](LICENSE) for the full text.
