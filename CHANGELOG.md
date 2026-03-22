# Changelog

## 0.1.0 — Initial Release

First public release of `expat-rust`, a memory-safe Rust reimplementation of libexpat.

### Features

- Complete SAX callback API: element handlers, character data, processing instructions, comments, CDATA sections, DTD declarations, namespace processing, entity handling, XML declaration handling
- Zero `unsafe` blocks
- Verified against libexpat 2.7.5 with 350+ behavioral comparison tests
- Built-in billion laughs attack protection
- UTF-8, UTF-16 (LE/BE), and ISO-8859-1 encoding support
- Streaming parser — process arbitrarily large documents without buffering

### Verified Against

- libexpat 2.7.5 (tag R_2_7_5)
