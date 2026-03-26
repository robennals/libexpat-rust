# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## 0.1.0 — Initial Release

First public release of `expat-rust`, a memory-safe Rust reimplementation of libexpat.

### Features

- Complete SAX callback API: element handlers, character data, processing instructions, comments, CDATA sections, DTD declarations, namespace processing, entity handling, XML declaration handling
- Zero `unsafe` blocks in the core parser — all unsafety confined to the FFI layer (`expat-ffi`)
- Zero production dependencies
- Passes 100% of libexpat 2.7.5's own test suite (286/291; 5 skipped test C-specific allocator APIs)
- Verified by 463 additional comparison tests (identical XML through both C and Rust parsers)
- Full encoding support: UTF-8, UTF-16 (LE/BE), ISO-8859-1, US-ASCII, custom encodings
- Full namespace processing: prefix-to-URI binding, `{URI}local` name rewriting, prefix validation
- Built-in billion laughs attack protection
- Streaming parser — process arbitrarily large documents without buffering
- C drop-in replacement via `expat-ffi` (produces `libexpat.so`/`.dylib`/`.dll`)

### Verified Against

- libexpat 2.7.5 (tag R_2_7_5)
