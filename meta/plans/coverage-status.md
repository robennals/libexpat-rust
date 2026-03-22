# Coverage Status and Plan

## Current Metrics

### Line coverage (measured by tooling)

| File | C line coverage (gcov, native C tests) | Rust line coverage (tarpaulin, all Rust tests) |
|------|----------------------------------------|------------------------------------------------|
| xmlparse | **92.3%** of 4789 lines | **82.5%** of 1684 lines |
| xmltok_impl | **86.2%** of 950 lines | **74.3%** of 1532 lines |
| xmlrole | **84.6%** of 664 lines | **86.1%** of 526 lines |
| **Overall** | **~90%** | **80.3%** (3249/4046 lines) |

C line coverage is the baseline — it's what C's own comprehensive test suite (4692 test cases) achieves of the C source. Our Rust line coverage is lower both because we added new code without tests, and because our Rust is only 64% of C's size (missing features aren't lines to cover, but their absence means the code we DO have handles fewer cases).

### Test pass rates

| Test suite | Pass rate | What it measures |
|------------|-----------|------------------|
| **C test suite against Rust** | **196/290 (67.6%)** | Feature parity: does Rust match C behavior for complex XML? |
| **Rust comparison tests** | 1020/1212 (84%) | SAX event equivalence for simple XML inputs. 192 tests ignored (features not yet implemented). |

### Code size

| | C | Rust | Ratio |
|--|---|------|-------|
| xmlparse | 9267 lines | 3645 lines | 39% |
| xmltok_impl | 1819 lines | 3185 lines | 175% (Rust expanded from C macros) |
| xmlrole | 1255 lines | 1073 lines | 85% |
| **Total** | **12341 lines** | **7903 lines** | **64%** |

The Rust code is 64% of C's size. The missing 36% corresponds to features like namespace processing, parameter entity expansion, content model building, unknown encoding handler, and various sub-parser modes.

## How to Measure Coverage

```bash
# Rust line coverage
cargo install cargo-tarpaulin
cargo tarpaulin -p expat-rust --skip-clean --timeout 300

# C line coverage (native C tests against C libexpat)
cd expat/expat && mkdir -p build-cov
cd build-cov && cmake .. -DCMAKE_C_FLAGS="--coverage" -DCMAKE_BUILD_TYPE=Debug
make -j4 && ./tests/runtests -q
cd CMakeFiles/runtests.dir/lib && gcov xmlparse.c.gcda xmltok.c.gcda xmlrole.c.gcda

# C tests against Rust (pass/fail count)
cargo build -p c-tests-runner
./target/debug/c-tests-runner 2>/dev/null | grep -c "^PASS:"
```

## Why These Numbers Don't Tell the Whole Story

### 90% Rust coverage ≠ 90% C feature coverage

Prior PRs achieved ~90% coverage of the **Rust code that exists**. But our Rust is only 64% of C's size — we're missing ~4400 lines of C functionality. So 90% Rust coverage means covering 90% of 64% = **~58% of C's behavior**.

The C test suite is the true measure of feature parity. It exercises the full C API surface including features our Rust doesn't implement yet.

### What the C test suite tests that Rust comparison tests don't

The Rust comparison tests parse simple XML and compare SAX events. The C test suite additionally tests:

1. **Namespace URI resolution** — prefix binding, name rewriting, validation
2. **External entity sub-parsers** — `XML_ExternalEntityParserCreate` with complex handler chains
3. **Parameter entity expansion** — `%pe;` refs in DTD with recursive detection
4. **DTD content model building** — `<!ELEMENT>` → `XML_Content` tree
5. **Foreign DTD loading** — `XML_UseForeignDTD` triggering handler
6. **Unknown encoding handler** — custom encoding table integration
7. **Suspend/resume within entity expansion** — save/restore entity state
8. **`XML_DefaultCurrent`** — forwarding current event from within handlers
9. **IGNORE sections** — `<![IGNORE[...]]>` in DTD conditional sections
10. **Complex error scenarios** — async entities, infinite loops, pool integrity

## Remaining Work: By Subsystem

### Subsystem 1: Namespace Processing (22 C tests)
**Effort**: Large (estimated ~500 lines)
**Impact**: 22 tests + several encoding/NS combined tests

Needs a namespace binding engine in `do_content`:
- Track prefix→URI bindings in a scoped stack
- Rewrite element/attribute names to `{URI}<sep>local`
- Call `startNamespaceDeclHandler`/`endNamespaceDeclHandler`
- Validate: unbound prefixes, reserved prefixes, double colons

Can be developed independently — doesn't affect other subsystems.

### Subsystem 2: External Entity Sub-parser Mode (8 C tests)
**Effort**: Medium (~200 lines)
**Impact**: 8 tests + unlocks foreign DTD tests

`create_external_entity_parser` with NULL context needs to create a parser in DTD mode. Also needs:
- Trailing CR/] handling in sub-parser
- BOM consumption in sub-parser
- Not-standalone propagation

### Subsystem 3: Parameter Entity Expansion (6 C tests)
**Effort**: Large (~400 lines)
**Impact**: 6 tests + several related tests

When `XML_PARAM_ENTITY_PARSING_ALWAYS` is set:
- `%pe;` in DTD should trigger external entity handler
- Expand PE content through DTD processor
- Recursive PE detection
- Skipped entity handler for undefined PEs

### Subsystem 4: Foreign DTD (3 C tests)
**Effort**: Small (~50 lines, depends on subsystem 2)
**Impact**: 3 tests

Blocked on subsystem 2 (DTD-mode sub-parser). Once that works, the foreign DTD trigger at InstanceStart should work.

### Subsystem 5: Content Model Building (3 C tests)
**Effort**: Medium (~300 lines)
**Impact**: 3 tests

Build `XML_Content` tree for `<!ELEMENT>` declarations:
- Track group nesting, sequence/choice operators
- Build tree with type, quant, name, children
- Pass to `elementDeclHandler`

### Subsystem 6: Unknown Encoding Handler (8 C tests)
**Effort**: Medium (~200 lines)
**Impact**: 8 tests

Integrate `XML_SetUnknownEncodingHandler` with tokenizer:
- Call handler when encoding name is unrecognized
- Build encoding table from handler's response
- Validate table entries

### Subsystem 7: Default Handler / DefaultCurrent (2 C tests)
**Effort**: Medium (~100 lines)
**Impact**: 2 tests

`XML_DefaultCurrent` needs to work during entity expansion (save original `&entity;` text, not expanded content). DTD default handling needs more complete forwarding.

### Subsystem 8: Suspend/Resume in Entities (2 C tests)
**Effort**: Hard (~200 lines)
**Impact**: 2 tests

Save entity expansion state on suspend, restore on resume. Requires tracking which entity we're in and position within it.

### Subsystem 9: Entity Edge Cases (4 C tests)
**Effort**: Medium (~150 lines)
**Impact**: 4 tests

- Event pointer tracking during entity expansion
- Async entity detection (entity spans element boundaries)
- Reentrancy detection
- Infinite loop prevention

### Subsystem 10: Encoding Edge Cases (29 C tests)
**Effort**: Medium (~200 lines, mostly in FFI)
**Impact**: 29 tests

- UTF-16 byte offset mapping edge cases
- Latin-1 in external entities
- UTF-8 auto-alignment

### Subsystem 11: Miscellaneous (7 C tests)
**Effort**: Small-Medium (~150 lines total)
**Impact**: 7 tests

Individual fixes: buffer growth, bad DOCTYPE, IGNORE sections, user parameters, attribute leak, pool integrity, indirect PE recursion.

## Priority Order

1. **Namespace Processing** (22 tests) — biggest single impact
2. **External Entity Sub-parser + Foreign DTD** (11 tests) — unlocks several areas
3. **Parameter Entity Expansion** (6 tests) — core DTD feature
4. **Encoding Edge Cases** (29 tests) — many small fixes
5. **Content Model Building** (3 tests) — isolated feature
6. **Unknown Encoding Handler** (8 tests) — isolated feature
7. **Everything else** (15 tests) — individual fixes

## Tests Not Applicable (5)

| Test | Reason |
|------|--------|
| `test_misc_alloc_create_parser` | Custom C allocator — Rust uses own allocator |
| `test_misc_alloc_create_parser_with_encoding` | Same |
| `test_accounting_precision` | Internal C testing counter (`g_bytesScanned`) |
| `test_billion_laughs_attack_protection_api` | Tests internal C state on non-root parsers |
| `test_amplification_isolated_external_parser` | Tests amplification tracking internals |
