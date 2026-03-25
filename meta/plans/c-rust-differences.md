# C-to-Rust Structural Differences

This document lists ALL intentional structural differences between the C libexpat
and the Rust implementation. Any difference NOT listed here is a bug to be fixed.

## Principled Differences (by design)

### 1. Memory Management
- **C**: Custom allocator API (`XML_ParserCreate_MM`), manual malloc/free
- **Rust**: Standard Rust allocator, owned `Vec`/`String`/`HashMap`
- **Reason**: Rust's ownership model provides memory safety without custom allocators

### 2. Hash Tables
- **C**: Custom hash tables (`HASH_TABLE`) with SipHash
- **Rust**: `HashMap` from standard library
- **Reason**: Rust's HashMap is safe, well-tested, and has DoS-resistant hashing

### 3. Accounting/Amplification Limits
- **C**: `g_bytesScanned`, `XML_ACCOUNT_*` macros, billion laughs attack protection API
- **Rust**: Not implemented — Rust's memory safety provides equivalent protection
- **Reason**: C's byte-counting is for DoS mitigation; Rust's allocator provides natural limits

### 4. Entity Storage
- **C**: `ENTITY` struct with `textPtr`/`textLen` (pointer into memory pool)
- **Rust**: `internal_entities` HashMap with String values, `param_entities` HashMap
- **Reason**: Rust's owned strings avoid pool lifetime management

### 5. Tag Stack
- **C**: Linked list of `TAG` structs with raw name buffers
- **Rust**: `Vec<String>` tag_stack with UTF-8 names
- **Reason**: Rust Vec is safe and efficient; UTF-8 strings avoid raw buffer management

## Unauthorized Differences (bugs to fix)

### 1. GE Expansion — FIXED
- **Status**: Fixed. `do_content` now calls `process_entity()` matching C's
  `processEntity()` at xmlparse.c:3450. Added reenter check at xmlparse.c:3784.
  Fixed `internal_entity_processor` to use captured entity index (not `last()`)
  when updating entity state, preventing corruption when nested entities push
  onto the stack.
- **Remaining issue**: 2 test regressions (test_ext_entity_good_cdata,
  test_misc_expected_event_ptr_issue_980) need investigation.

### 2. Unknown Encoding Handler: Post-XmlDecl Transcoding
- **C**: `processXmlDecl` switches the encoding object; subsequent tokenization
  automatically uses the new encoding
- **Rust**: Encoding detection sets `custom_encoding_map`, but the current parse
  buffer was already appended as raw bytes (before encoding was detected). The
  remaining buffer data needs transcoding before further processing.
- **Impact**: `test_unknown_encoding_success` and related tests fail
- **Fix needed**: After unknown encoding handler succeeds in `handle_prolog_role`,
  transcode remaining buffer data through the new encoding map

### 3. Epilog Transition Check
- **C**: xmlparse.c:3638 checks `m_tagLevel == 0` for epilog transition
- **Rust**: Currently checks `tag_level == start_tag_level`
- **Impact**: Entity text processing incorrectly triggers epilog transition
  when tag_level matches start_tag_level within entity context
- **Fix needed**: Change to `tag_level == 0` (partially done, needs testing)

### 4. processXmlDecl as Separate Function
- **C**: `processXmlDecl()` is a standalone function called from doProlog
- **Rust**: XML declaration handling is inlined in `handle_prolog_role`
- **Impact**: AST compare tool reports missing `process_xml_decl` call
- **Fix priority**: Low (functionally equivalent, just structural)
