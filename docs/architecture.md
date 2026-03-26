# Architecture

How the `expat-rust` parser is structured internally.

## Module Dependency Graph

```
                    xmlparse (public API, main parser)
                        │
              ┌─────────┼─────────┐
              │         │         │
           xmltok    xmlrole   siphash
          (tokens)   (prolog)   (hash)
              │
        xmltok_impl
        (tokenizer)
              │
     ┌────────┼────────┐
     │        │        │
   ascii  char_tables  nametab
```

- **Layer 0 (leaves)**: `ascii`, `char_tables`, `nametab`, `siphash` — pure data and utility, no dependencies
- **Layer 1**: `xmltok_impl` (tokenizer implementation), `xmlrole` (prolog state machine)
- **Layer 2**: `xmltok` (token types, encoding detection, tokenizer interface)
- **Layer 3**: `xmlparse` (public API, state machine, handler dispatch)

## Parse Flow

```
User calls parser.parse(data, is_final)
    │
    ▼
run_processor()  ─── dispatches to current processor
    │
    ├── prolog_processor()      ─── XML declaration, DTD
    │       │
    │       ├── calls prolog_tok() to get tokens
    │       ├── calls xmlrole state machine to classify token
    │       └── handles DTD declarations, entity definitions
    │
    ├── content_processor()     ─── document body
    │       │
    │       ├── calls content_tok() to get tokens
    │       └── dispatches to handlers:
    │           start_element, end_element, character_data,
    │           processing_instruction, comment, cdata, etc.
    │
    └── epilog_processor()      ─── after root element closes
            │
            ├── calls content_tok() for remaining tokens
            └── only PIs, comments, and whitespace are valid
```

## Key Modules

### `xmlparse.rs` — Main Parser

The public API and parser state machine. Contains:

- **`Parser` struct**: The main parser state, holding:
  - Parse buffer and position tracking
  - 20+ callback handler slots (element, text, DTD, namespace, etc.)
  - DTD state (entities, element/attribute declarations, notations)
  - Namespace processing state
  - Security limits (entity expansion caps, billion laughs protection)

- **Processors**: State machine nodes that handle different parsing phases:
  - `prolog_processor` / `prolog_init` — XML declaration, DOCTYPE, DTD
  - `content_processor` / `do_content` — document body
  - `cdata_section_processor` — CDATA sections
  - `epilog_processor` — post-root-element content

- **Key difference from C**: The C parser uses function pointers for processor dispatch (`m_processor`). The Rust port uses the `Processor` enum with a match statement.

### `xmltok.rs` — Token Interface

Defines the token types and provides the interface to the tokenizer:

- **`XmlTok` enum**: All possible XML tokens (start tags, end tags, text, entity references, etc.)
- **`Utf8Encoding`**: The only encoding implementation used at runtime (see design decision below)
- **XML declaration parsing**: Extracts version, encoding, and standalone from `<?xml ... ?>`

### `xmltok_impl.rs` — Tokenizer

The lexer that converts raw bytes into tokens. Two main entry points:

- **`content_tok()`**: Tokenizes document content (elements, text, CDATA, etc.)
- **`prolog_tok()`**: Tokenizes the prolog/DTD (keywords, names, literals, etc.)

The tokenizer is encoding-aware — it handles multi-byte sequences, BOM detection, and encoding-specific character classification.

**Key difference from C**: The C tokenizer uses `#include` to compile `xmltok_impl.c` three times with different macro definitions (for UTF-8, UTF-16 LE, UTF-16 BE). The Rust port uses explicit encoding parameters instead.

### `xmlrole.rs` — Prolog Role State Machine

Classifies tokens in the XML prolog and DTD into semantic roles:

- Tracks state through DOCTYPE declarations, element declarations, attribute declarations, entity declarations, and notation declarations
- Maps token sequences to role identifiers (e.g., "this name token is an element type name in an ATTLIST declaration")
- Drives the prolog processor's handling of DTD content

### `siphash.rs` — Hash Function

SipHash-2-4 implementation for hash table randomization. Provides DoS protection by preventing hash-flooding attacks against the parser's internal hash tables. Zero `unsafe` blocks.

## Data Structure Mapping (C → Rust)

| C Structure | Rust Replacement | Why |
|-------------|-----------------|-----|
| `STRING_POOL` (manual arena) | `String`, `Vec<u8>` | RAII, no manual free |
| `HASH_TABLE` (open addressing) | `HashMap<String, T>` | Standard library, well-tested |
| `XML_ParserStruct` (70+ fields) | `Parser` struct | Same fields, Rust ownership |
| `DTD` struct | `Dtd` struct with `HashMap` fields | Same structure, safe collections |
| Function pointer (`m_processor`) | `Processor` enum + match | Type-safe dispatch |
| `ENTITY` / `ELEMENT_TYPE` / `ATTRIBUTE_ID` | Rust structs with `String` fields | No manual memory management |
| `BINDING` linked list | `Vec<NamespaceBinding>` | Simpler, no pointer chasing |

## Design Decisions

### Encoding: Transcode-to-UTF-8 (differs from C)

The C libexpat tokenizes XML in its **native encoding** — it has separate encoding structs with encoding-specific byte-type tables for UTF-8, Latin-1, ASCII, and UTF-16. The encoding struct is selected based on the XML declaration's `encoding=` attribute and BOM detection.

The Rust port takes a different approach: it **transcodes all non-UTF-8 input to UTF-8** in `Parser::parse()` before tokenizing. The tokenizer then always operates on UTF-8 data using a single `Utf8Encoding` implementation.

**Why this is better for Rust:**

- **Type safety**: Rust's `String`/`&str` types guarantee valid UTF-8. By transcoding upfront, all internal string handling uses Rust's native types without unsafe conversions.
- **Simplicity**: One tokenizer code path instead of four (UTF-8, Latin-1, ASCII, UTF-16). In C, `xmltok_impl.c` is `#include`d three times with different macros — this complexity is eliminated.
- **Correctness**: The XML spec defines the same abstract character model regardless of encoding. Lossless transcoding produces identical tokens for all XML-legal inputs.

**Verified by**: 463 comparison tests confirm identical SAX event sequences, status codes, and error codes between C and Rust parsers for UTF-8, UTF-16 (LE/BE), Latin-1, and US-ASCII inputs, including byte-by-byte incremental parsing.

**Byte offset correctness**: `XML_GetCurrentByteIndex` returns byte offsets in the **original input encoding**, not the internal UTF-8 stream. For non-UTF-8 input (UTF-16, Latin-1), the FFI layer lazily re-scans the original buffer to map transcoded UTF-8 positions back to original-encoding byte offsets. This produces identical results to C libexpat for all encodings.

## Error Handling

The C library uses error codes (`XML_Error` enum) stored in `m_errorCode`, checked after each operation. The Rust port preserves this pattern for behavioral compatibility — the `XmlError` enum maps 1:1 to C's error codes.

Internal functions use `XmlStatus` (Ok, Error, Suspended) as return values, matching the C pattern. This was a deliberate choice to maintain structural correspondence with the C code, making the behavioral comparison tests straightforward.
