# Porting Plan and Methodology

See `plans/process-log.md` for transferable lessons about C-to-Rust porting methodology.
This document focuses on the practical workflow and remaining work for this specific port.

## The Correctness Verification Method

### The core tool: expat-sys + comparison tests

The most important infrastructure is the `expat-sys` crate, which builds the real C
library from source and provides a safe Rust wrapper (`CParser`). Comparison tests in
`c_comparison_tests.rs` run the same XML through both our Rust port and the C library,
comparing status codes, error codes, and handler callback output.

**This is what actually finds bugs.** It found 3 behavioral divergences immediately that
had survived agent-written code and manual review. See process-log.md for the detailed
bug-finding walkthrough.

### All test results are from the clean idiomatic Rust port
The 108 passing tests and 56/59 comparison tests all run against `expat-rust/` — the
safe, idiomatic Rust code. The C2Rust output is never tested; it exists only as an
optional reference.

### C2Rust output: available but not essential
The C2Rust transpilation output exists in `c2rust-output/src/` and compiles on nightly.
It's occasionally useful for grepping function names or estimating porting effort, but
reading the C source directly works just as well. The real value came from building the
FFI test harness, not from the transpilation.

## The Effective Workflow

### For finding bugs in existing code:
1. **Write a comparison test** for the behavior you want to verify
2. **Run it** — if it passes, the Rust port matches C; if it fails, you have a concrete bug
3. **Look at C2Rust output** — `./scripts/c2rust-pipeline.sh extract functionName` shows the mechanically correct logic
4. **Fix the Rust port** to match C behavior
5. **Re-run** to confirm

### For porting new functions:
1. **Extract from C2Rust** — `./scripts/c2rust-pipeline.sh extract doProlog --prompt` generates a transformation-ready prompt with the function, C original, and existing port conventions
2. **Transform to idiomatic Rust** — use a haiku agent with `agents/c2rust-transform.md` or manually rewrite following `scripts/c2rust-to-idiomatic.md` rules
3. **Integrate** into `expat-rust/src/xmlparse.rs`
4. **Add comparison test** to verify
5. **Run all tests** — `cargo test` must not regress

### Key transformation patterns (from C2Rust → idiomatic Rust):
| C2Rust Pattern | Count in xmlparse.rs | Idiomatic Rust |
|----------------|---------------------|----------------|
| `(*parser).m_field` | 2,729 | `self.field_name` (method on Parser) |
| `as ::core::ffi::c_int` | 1,508 | Native Rust types (i32, u32, etc.) |
| Function pointer calls (.expect("non-null")) | 120 | Trait dispatch or enum match |
| `current_block` goto patterns | 119 | Loop/match/early return |
| `malloc`/`free` | 66 | Vec, Box, String |
| C string literals | varies | `&str` or `&[u8]` |
| `XML_Bool` (unsigned char) | varies | `bool` |

### Important: C hash tables → Rust HashMap
The C code implements custom hash tables (HASH_TABLE) with open addressing. In the Rust port, use `std::collections::HashMap<String, T>` instead. Same for STRING_POOL → use `String`/`Vec<u8>`.

## Tool Reference

```bash
# Run comparison tests (idiomatic Rust port vs C library)
./scripts/c2rust-pipeline.sh compare

# Extract a function from C2Rust output with transformation prompt
./scripts/c2rust-pipeline.sh extract doProlog --prompt

# Compare function lists between C2Rust and existing port
./scripts/c2rust-pipeline.sh functions

# Analyze patterns in C2Rust output
./scripts/c2rust-pipeline.sh analyze

# Run mechanical type cleanup on C2Rust output
./scripts/c2rust-pipeline.sh cleanup
```

## Current Status

### Test results (all from the idiomatic Rust port):
- **108 passing** unit/integration tests
- **56/59 comparison tests** passing (3 DOCTYPE failures — not yet implemented)
- **1 known failure**: `test_dtd_elements` (DOCTYPE not implemented)
- **~160 ignored tests** awaiting features below

### What's implemented:
- Full XML element parsing (start/end tags, attributes, self-closing)
- Character data and CDATA sections
- Processing instructions and comments
- Predefined entity references (&lt; &gt; &amp; &quot; &apos;)
- Numeric/hex character references (&#65; &#x41;)
- UTF-8 and UTF-16 (BE/LE) encoding detection and transcoding
- Position tracking (line/column)
- All handler callbacks (21 types)
- Incremental parsing
- Error reporting

## Porting Priorities

### Priority 1: DOCTYPE/DTD Support (~98 tests blocked)

Key functions needed (sizes from C2Rust analysis):
1. **doProlog** (1,612 lines) — Giant state machine for DOCTYPE, ELEMENT, ENTITY, ATTLIST, NOTATION
2. **storeAtts** (580 lines) — Attribute storage with namespace resolution
3. **storeEntityValue** (234 lines) — Entity value storage
4. **appendAttributeValue** (239 lines) — Attribute value with entity expansion
5. **addBinding** (235 lines) — Namespace binding
6. **dtdCreate/dtdDestroy/dtdReset** — DTD lifecycle
7. **getAttributeId** (129 lines) — Attribute name lookup
8. **defineAttribute** (68 lines) — ATTLIST handler

Supporting data structures needed:
- DTD struct (use HashMap instead of C's HASH_TABLE)
- ENTITY struct
- ELEMENT_TYPE struct
- ATTRIBUTE_ID struct

**Strategy**: Port DTD infrastructure (structs + HashMap-based lookups) first, then port doProlog incrementally — it's a giant match statement with ~70 cases, can be done a few cases at a time with comparison tests verifying each batch.

### Priority 2: External Entity Support (~30 tests)
- processEntity (76 lines)
- externalEntityInitProcessor/2/3 (22+61+66 lines)
- XML_ExternalEntityParserCreate (173 lines)
- Depends on Priority 1

### Priority 3: Stop/Resume (~10 tests)
- XML_StopParser (48 lines), XML_ResumeParser (57 lines)
- callProcessor reenter logic
- Relatively independent, could be done in parallel with Priority 1

### Priority 4: Unknown Encoding (~20 tests)
- handleUnknownEncoding (80 lines)
- XmlInitUnknownEncoding (43 lines)

### Priority 5: Custom Allocator (~30 tests)
- expat_malloc/realloc/free wrappers
- XML_ParserCreate_MM
- Accounting functions (billion laughs protection)

## Conventions
- Each function must maintain 1:1 correspondence with C call tree
- Use `self.field_name` (snake_case, no `m_` prefix) for parser fields
- C `XML_FunctionName` → Rust `parser.function_name()` method
- Use Rust idioms (HashMap, Vec, String, Option, Result) instead of C patterns (hash tables, malloc, null pointers, error codes)
- Run `cargo test --test c_comparison_tests` after every change
