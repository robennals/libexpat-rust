# C2Rust Port Plan

## Approach
Use C2Rust output (c2rust-output/src/) as the source of truth for correct logic.
Transform functions into idiomatic Rust matching the existing port's conventions.
Verify each change with comparison tests (tests/c_comparison_tests.rs).

## Priority 1: DOCTYPE/DTD Support (~98 tests)

### Key functions needed (from C2Rust xmlparse.rs):
1. **doProlog** (1612 lines) — The biggest function. Core state machine for:
   - DOCTYPE declaration
   - Internal subset (ELEMENT, ENTITY, ATTLIST, NOTATION declarations)
   - Parameter entity references
   - Conditional sections
2. **defineAttribute** (68 lines) — ATTLIST handler
3. **getAttributeId** (129 lines) — Attribute name/id lookup
4. **storeEntityValue** (234 lines) — Entity value storage
5. **storeAttributeValue** (111 lines) — Attribute value storage
6. **dtdCreate/dtdDestroy/dtdReset** — DTD lifecycle
7. **hashTable*** — Hash table for name lookups
8. **poolInit/poolGrow/poolAppend/poolStoreString** — String pool management

### Supporting data structures:
- DTD struct (with hash tables for elements, entities, attributes)
- HASH_TABLE / HASH_TABLE_ITER
- STRING_POOL
- ENTITY struct
- ELEMENT_TYPE struct
- ATTRIBUTE_ID struct

### Strategy:
Port the DTD infrastructure first (structs + hash table + string pool), then
port doProlog incrementally (it's a giant match statement with ~70 cases).

## Priority 2: External Entity Support (~30 tests)

### Key functions:
- processEntity (76 lines)
- externalEntityContentProcessor (24 lines)
- externalEntityInitProcessor/2/3 (22+61+66 lines)
- XML_ExternalEntityParserCreate (173 lines)

### Depends on: Priority 1 (DTD struct)

## Priority 3: Stop/Resume (~10 tests)

### Key functions:
- XML_StopParser (48 lines)
- XML_ResumeParser (57 lines)
- callProcessor reenter logic

### Relatively independent, could be done in parallel

## Priority 4: Unknown Encoding (~20 tests)

### Key functions:
- handleUnknownEncoding (80 lines)
- XmlInitUnknownEncoding (43 lines)

## Priority 5: Custom Allocator (~30 tests)

### Key functions:
- expat_malloc/expat_realloc/expat_free
- XML_ParserCreate_MM
- Accounting functions (accountingDiffTolerated, etc.)

## Notes
- Each function should maintain 1:1 correspondence with C
- Use `./scripts/c2rust-pipeline.sh extract FUNC --prompt` to get transformation-ready prompts
- Run `cargo test --test c_comparison_tests` after each change
- The C2Rust output in c2rust-output/ compiles on nightly and is the reference for correct logic
