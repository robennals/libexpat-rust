# C-to-Rust Function Mapping

## Current State

- 255 of 290 C tests pass (88%)
- 48 of 168 C functions have matched Rust equivalents
- 24 C functions are truly missing from the Rust parser
- 69 C functions are handled in the FFI layer (expat-ffi/src/lib.rs)
- 27 C functions are C-specific infrastructure (not applicable to Rust)

## Missing Functions That Need Porting

These 24 C functions have NO Rust equivalent and their functionality is either missing or inadequately inlined.

### Group 1: Attribute Processing (called from doContent/storeAtts)
| C Function | Lines | Purpose | Current Rust Handling |
|---|---|---|---|
| `storeAtts` | 467 | Full attribute processing: validation, defaults, ID tracking, namespace bindings | `extract_attrs` — simplified, missing defaults/ID/validation |
| `getAttributeId` | 60 | Look up/create attribute ID, check for xmlns, track ID attributes | Inlined partially in extract_attrs |
| `defineAttribute` | 60 | Store attribute definition from ATTLIST, handle defaults | Missing — ATTLIST data stored but not applied |
| `getElementType` | 22 | Look up/create element type in DTD hash table | Missing — we use HashMap directly |
| `setElementTypePrefix` | 28 | Parse namespace prefix from element name | Inlined in process_namespaces |
| `addBinding` | 162 | Create namespace binding from xmlns attribute, push binding stack | Inlined in do_content start tag handling |
| `storeAttributeValue` | 76 | Process attribute value with entity expansion for ATTLIST defaults | Missing — needed for ATTLIST default application |
| `appendAttributeValue` | 192 | Recursive attribute value processing with entity/char ref expansion | Missing — entity refs in attribute values not fully expanded |
| `normalizeLines` | 20 | Normalize line endings in attribute values (\\r\\n → \\n) | Missing |
| `normalizePublicId` | 20 | Normalize whitespace in PUBLIC identifiers | Missing |
| `storeRawNames` | 48 | Store raw (un-decoded) element/attribute names for later use | Missing — we decode names eagerly |

### Group 2: Entity Value Processing (called from doProlog)
| C Function | Lines | Purpose | Current Rust Handling |
|---|---|---|---|
| `callStoreEntityValue` | 70 | Wrapper: calls storeSelfEntityValue with proper encoding setup | `store_entity_value` — partially merged |
| `storeSelfEntityValue` | 21 | Store entity value with char ref / PE expansion | `store_entity_value` — missing PE expansion in values |

### Group 3: XML Declaration & Encoding
| C Function | Lines | Purpose | Current Rust Handling |
|---|---|---|---|
| `processXmlDecl` | 91 | Parse XML/text declaration, set encoding, check version/standalone | Inlined in handle_prolog_role XmlDecl case |
| `initializeEncoding` | 29 | Set up initial encoding from user-specified or auto-detected | `detect_and_transcode` — different architecture |
| `handleUnknownEncoding` | 33 | Call user's unknown encoding handler, create custom encoding | Missing — handler is called but result not used for tokenizing |

### Group 4: External Entity / DTD Processors
| C Function | Lines | Purpose | Current Rust Handling |
|---|---|---|---|
| `setContext` | 71 | Parse context string, set up entity/prefix open lists for child parser | Partially in `create_external_entity_parser` |
| `copyEntityTable` | 62 | Copy entity hash table from parent to child parser | Partially — we clone DtdState for content child |
| `externalEntityInitProcessor2` | 44 | Second phase of ext entity init: skip BOM, detect encoding | Missing — `external_entity_init_processor` doesn't handle BOM |
| `externalParEntInitProcessor` | 19 | Init processor for external parameter entity parsers | `external_par_ent_processor` — partially |
| `externalEntityContentProcessor` | 13 | Content processor for external entity after init | Missing — init jumps directly to content |
| `entityValueInitProcessor` | 83 | Init processor for entity value parsing (PE in entity values) | Missing |
| `entityValueProcessor` | 44 | Continues entity value parsing after init | Missing |
| `ignoreSectionProcessor` | 14 | Resume processing of IGNORE sections after suspend | Missing |

### Group 5: Content Model
| C Function | Lines | Purpose | Current Rust Handling |
|---|---|---|---|
| `build_model` | 129 | Build XML_Content tree from parsed element declarations | Inlined in handle_prolog_role element decl cases |

## C-Specific Functions (Not Applicable to Rust)

These C functions have no Rust equivalent by design:

| C Function | Reason |
|---|---|
| `expat_malloc/realloc/free` | Rust uses its own allocator |
| `expat_heap_*` | C heap tracking — Rust memory model differs |
| `dtdCreate/dtdDestroy` | Rust uses `DtdState::default()` / `Drop` |
| `copyString` | Rust `String::clone()` |
| `destroyBindings/moveToFreeBindingList` | C free-list memory management |
| `getRootParserOf` | C parent pointer traversal — Rust doesn't need |
| `triggerReenter` | C reenter mechanism — Rust uses processor enum switch |
| `errorProcessor` | Trivial: `return parser->m_errorCode` |
| `callProcessor` | C function pointer dispatch — Rust uses `match self.processor` |
| `startParsing` | C one-time init — Rust handles in `parse()` |
| `FASTCALL/XMLCALL` | C calling conventions |
| `unsignedCharToPrintable` | C debug helper |
| `ENTROPY_DEBUG/getDebugLevel` | C debug infrastructure |
| `writeRandomBytes_*` | C PRNG seed generation |
| `accounting*` | C byte-level DoS accounting |
| `testingAccounting*` | C test hooks for accounting |
| `copy_salt_to_sipkey/get_hash_secret_salt/keyeq/keylen` | C hash table internals — Rust uses `HashMap` |

## FFI Layer Functions (in expat-ffi/src/lib.rs)

All 69 `XML_*` API functions are implemented in the FFI layer as `extern "C"` wrappers that call into the Rust `Parser` struct. These are NOT missing — they just live in a different file than the AST tool expects.

## Design Decisions: C-to-Rust Idiom Mapping

### 1. Hash Tables → HashMap
C uses custom hash tables (`HASH_TABLE`) with `lookup()`. Rust uses `HashMap<String, T>`. Functions like `getAttributeId`, `getElementType`, `defineAttribute` that wrap hash table operations are replaced by direct `HashMap` calls. **However**, these C functions do MORE than just hash lookup — they also validate, track state, and set up data. We need to either:
- Port the functions 1:1 as Rust methods (preferred), OR
- Ensure all side effects are reproduced at call sites

### 2. String Pools → String/Vec
C uses `STRING_POOL` for memory management of strings during parsing. Rust uses `String` and `Vec<u8>`. Functions like `storeRawNames`, `storeAttributeValue`, `appendAttributeValue` that manipulate pools need to be ported to use `String`/`Vec` equivalents.

### 3. Processor Function Pointers → Processor Enum
C uses `parser->m_processor = functionPtr`. Rust uses `self.processor = Processor::Variant`. The `callProcessor` function is replaced by `match self.processor {}`. The `triggerReenter` mechanism (C sets a flag that causes the current doProlog/doContent to return) is replaced by checking `self.processor == Processor::InternalEntity` after `handle_prolog_role` returns.

### 4. ENTITY struct → Split Storage
C has a single `ENTITY` struct used for general entities, parameter entities, and notations. Rust splits into:
- `internal_entities: HashMap<String, String>` (name → value)
- `external_entities: HashMap<String, (Option<String>, Option<String>)>` (name → (sys_id, pub_id))
- `param_entities: HashMap<String, ParamEntity>` (name → full PE struct)
This works but means `copyEntityTable` and `setContext` need to handle all three.

### 5. Encoding Architecture
C has `ENCODING` struct with function pointers for tokenizing in different encodings (UTF-8, UTF-16LE, UTF-16BE, custom). Rust currently transcodes everything to UTF-8 upfront in `parse()`. This means:
- `initializeEncoding` → replaced by `detect_and_transcode`
- `processXmlDecl` → inlined but doesn't switch tokenizer encoding
- `handleUnknownEncoding` → handler called but custom encoding not wired to tokenizer
- UTF-16 byte-at-a-time feeding doesn't work (need buffering + transcode)
- BOM detection in child parsers (`externalEntityInitProcessor2`) is incomplete

**This is the biggest architectural divergence** and the reason ~10 UTF-16/encoding tests fail.

## Porting Priority

### Priority 1: Fix regressions, get back to 255+ (immediate)
The current session introduced some regressions from PE work. Stabilize first.

### Priority 2: Port missing prolog/content functions (fixes ~7 PE tests)
- `processXmlDecl` as standalone function (currently inlined)
- `callStoreEntityValue`/`storeSelfEntityValue` for PE expansion in entity values
- Wire `process_entity` for internal PEs in prolog (already started)

### Priority 3: Port attribute processing chain (fixes ~3 tests)
- `storeAttributeValue`/`appendAttributeValue` for ATTLIST default application
- `defineAttribute` for proper ATTLIST handling
- `normalizePublicId` for PUBLIC id validation

### Priority 4: Port external entity processors (fixes ~5 tests)
- `externalEntityInitProcessor2` for BOM handling
- `externalEntityContentProcessor` for proper ext entity flow
- `setContext`/`copyEntityTable` for child parser setup
- `entityValueInitProcessor`/`entityValueProcessor` for PE in entity values

### Priority 5: Fix encoding architecture (fixes ~10 UTF-16 tests)
- Port `initializeEncoding` to replace `detect_and_transcode`
- Port `handleUnknownEncoding` to create custom tokenizer
- Handle UTF-16 in tokenizer layer (not parse layer)
- This is the most complex change and may need an architecture review

### Priority 6: Remaining small functions
- `normalizeLines`, `storeRawNames`, `ignoreSectionProcessor`
- `build_model` improvements, `setElementTypePrefix`
