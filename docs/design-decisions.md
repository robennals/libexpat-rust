# Design Decisions

Key choices made during the C-to-Rust port of libexpat, and why.

## 1. Reimplementation, Not Transpilation

We produced an idiomatic Rust reimplementation rather than using C2Rust transpiler output directly.

**Why**: C2Rust output is mechanically correct but saturated with `unsafe`, raw pointers, and C idioms. It would have been a Rust crate in name only — still vulnerable to the same memory safety issues as C. An idiomatic reimplementation with zero `unsafe` provides genuine safety guarantees.

C2Rust was still valuable as a correctness reference (see [porting-process.md](porting-process.md)).

## 2. Replace C Data Structures With Rust Standard Library

| C Pattern | Rust Replacement |
|-----------|-----------------|
| `STRING_POOL` (custom arena allocator) | `String`, `Vec<u8>` |
| `HASH_TABLE` (open-addressed hash table) | `HashMap<String, T>` |
| Linked lists (`BINDING`, `OPEN_INTERNAL_ENTITY`) | `Vec<T>` |
| Manual `malloc`/`realloc`/`free` | Rust ownership + RAII |

**Why**: C's custom allocators exist because C has no standard collections. Rust's `String`, `Vec`, and `HashMap` are well-tested, optimized, and memory-safe. Reimplementing C's arena allocator in Rust would have been complex, error-prone, and pointless — the performance characteristics are comparable, and the safety properties are strictly better.

**Trade-off**: This means the Rust parser's allocation patterns differ from C's. We can't guarantee identical allocation counts or timing. But behavioral output (handler calls, error codes, byte positions) is identical, which is what matters. Performance-wise, the Rust parser is within 1.2-1.6x of C on element-heavy documents, and faster than C on small documents and deep nesting. Buffer reuse across elements mitigates much of the allocation overhead.

## 3. Enums Instead of Function Pointers

C's `m_processor` field is a function pointer that changes as the parser moves through states (prolog → content → epilog → error). The Rust port uses a `Processor` enum:

```rust
pub enum Processor {
    PrologInit,
    Prolog,
    Content,
    CdataSection,
    Epilog,
    Error,
    // ...
}
```

**Why**: Function pointers in Rust require either `unsafe` or boxing closures. An enum with a match statement is idiomatic, zero-cost, and makes the state machine explicit and exhaustive — the compiler verifies every state is handled.

## 4. Preserve C's Error Handling Pattern

The Rust port uses the same `XmlError` error codes and `XmlStatus` return values as C, rather than using Rust's `Result<T, E>`.

**Why**: Behavioral compatibility is the primary goal. The comparison tests verify that both parsers return identical error codes for identical inputs. Using `Result` would have required mapping between error representations, introducing a source of divergence.

For the public API, this means callers check `parser.error_code()` after a failed `parse()` call, just like in C. A future version could add a `Result`-based API wrapper without changing the core logic.

## 5. Transcode to UTF-8, Then Tokenize

C libexpat tokenizes XML in its **native encoding** — it has separate encoding structs with encoding-specific byte-type tables for UTF-8, Latin-1, ASCII, and UTF-16. The encoding struct is selected based on the BOM and XML declaration.

The Rust parser takes a different approach: **transcode all non-UTF-8 input to UTF-8 first**, then tokenize. The tokenizer always operates on UTF-8 data using a single `Utf8Encoding` implementation.

**Why this is better for Rust:**
- **Type safety**: Rust's `String`/`&str` types guarantee valid UTF-8. Transcoding upfront means all internal string handling uses Rust's native types.
- **Simplicity**: One tokenizer code path instead of four. The C technique of `#include`-ing `xmltok_impl.c` three times with different macros is eliminated.
- **Correctness**: The XML spec defines the same abstract character model regardless of encoding. Lossless transcoding produces identical tokens, confirmed by 463 comparison tests across UTF-8, UTF-16 LE/BE, Latin-1, and ASCII inputs.

**Byte offset handling**: `XML_GetCurrentByteIndex` returns byte offsets in the **original** input encoding, matching C behavior. For non-UTF-8 input, this requires a re-scan of the current parse chunk to convert the internal UTF-8 event position back to the original encoding's byte offset. This is O(chunk_size) per call but only happens when `XML_GetCurrentByteIndex` is actually called and only for non-UTF-8 input. No per-byte overhead during normal parsing, and no O(N) data structures in stream size — just a running `u64` counter of original bytes consumed by previous `parse()` calls.

## 6. No `unsafe` Code

The entire crate contains zero `unsafe` blocks. This was a hard requirement, not an aspiration.

**Why**: The entire point of a Rust reimplementation is memory safety. Any `unsafe` block reintroduces the same classes of bugs we're trying to eliminate. Every place where C uses raw pointers, pointer arithmetic, or manual memory management has a safe Rust equivalent:
- Buffer access → slice indexing with bounds checks
- String manipulation → `String`/`&str` operations
- Hash tables → `HashMap`
- Linked list traversal → `Vec` iteration

**Performance impact**: Bounds checking adds overhead, but Rust's optimizer eliminates most checks when it can prove they're redundant. With LTO enabled, benchmarks show Rust within 1.2-1.6x of C for element-heavy documents, and 2-2.7x faster than C for small documents and deep nesting.

## 7. 1:1 Function Correspondence With C

Each significant C function has a corresponding Rust function with a matching name (converted to snake_case). For example:

| C Function | Rust Function |
|-----------|--------------|
| `doContent` | `do_content` |
| `doProlog` | `do_prolog` |
| `storeAtts` | `store_atts` |
| `contentProcessor` | `content_processor` |

**Why**: This enables the AST structural comparison tool to verify function-level equivalence. It also makes it easy to cross-reference the C and Rust implementations when investigating behavioral differences.

**Exception**: Some C helper functions were inlined into their callers when they were trivial, and some C patterns were restructured (e.g., `goto` to loops). These exceptions are documented in [`meta/plans/call-tree-overrides.md`](../meta/plans/call-tree-overrides.md).

## 8. DTD State as `HashMap`s

C stores DTD entities, elements, and attributes in custom hash tables with interned strings. The Rust port uses `HashMap<String, Entity>`, `HashMap<String, ElementType>`, etc.

**Why**: The C hash tables are tightly coupled to the string pool allocator. Decoupling them lets us use standard Rust types. The string keys are cloned rather than interned, which uses more memory for pathological DTDs with thousands of declarations, but this is negligible for real-world XML documents.

## 9. Keep the SAX Callback Model

The Rust API uses the same callback/handler model as C's libexpat, with closures instead of function pointers:

```rust
parser.set_start_element_handler(Some(|name, attrs| { ... }));
parser.set_character_data_handler(Some(|text| { ... }));
```

**Why**: This is what libexpat users expect. A pull-based or iterator-based API would be a different parser, not a compatible replacement. The callback model also enables streaming — the parser can process arbitrarily large documents without buffering them in memory.

A future version could add a pull-based wrapper on top of the callback API, but the core remains SAX.
