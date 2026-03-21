---
model: haiku
description: Translate C test functions to Rust #[test] functions. Use when translating expat C tests to Rust.
---

# Translate C Tests to Rust

You are translating C test functions from libexpat's test suite to Rust.

## Process

1. Read the C test file specified
2. Read the Rust API facade at `expat-rust/src/xmlparse.rs`
3. Read an example translated test file (e.g., `expat-rust/tests/misc_tests.rs`) for style
4. Translate each `START_TEST(name) { ... } END_TEST` block to a Rust `#[test] fn name()`

## Translation Rules

- `XML_ParserCreate(NULL)` → `Parser::new(None).unwrap()`
- `XML_ParserCreateNS(enc, sep)` → `Parser::new_ns(enc, sep).unwrap()`
- `XML_Parse(parser, text, len, isFinal)` → `parser.parse(text.as_bytes(), is_final)`
- `XML_GetErrorCode(parser)` → `parser.error_code()`
- `XML_StopParser(parser, r)` → `parser.stop(r)`
- `XML_ResumeParser(parser)` → `parser.resume()`
- `XML_ParserFree(parser)` → `drop(parser)`
- `fail("msg")` → `panic!("msg")`
- `assert_true(cond)` → `assert!(cond)`
- `XCS("string")` → `"string"`
- Callbacks → `Box::new(|args| { ... })` closures
- CharData patterns → `String` or `Vec<u8>` collection

## Requirements

- First line: `// AI-generated test translation from <filename>`
- `use expat_rust::xmlparse::*;`
- Tests needing custom allocators → `#[test] #[ignore] // Requires custom allocator`
- Tests needing XML_DTD → `#[test] #[ignore] // Requires XML_DTD feature`
- Tests needing XML_GE → `#[test] #[ignore] // Requires XML_GE feature`
- No `unsafe`, preserve original test names
- Verify: `cargo test --test <name> --no-run`
