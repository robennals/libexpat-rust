# C Test Suite Status

Running the original C libexpat test suite against our Rust parser via `expat-ffi`.

**Current results: 173 pass, 117 fail** of 290 tests reached (60% pass rate).

## How to run

```bash
cargo build -p c-tests-runner
./target/debug/c-tests-runner 2>/dev/null | grep -c "^PASS:"   # count passes
./target/debug/c-tests-runner 2>/dev/null | grep -c "^FAIL "   # count fails
./target/debug/c-tests-runner 2>/dev/null | grep "^FAIL "      # list failures
```

No longer needs `lldb` — custom `assert.h` converts C asserts to test failures.

## Test Suites Included

The runner compiles: basic_tests.c (244 tests), ns_tests.c (33 tests),
misc_tests.c (22 tests), acc_tests.c (4 tests). It does NOT include
alloc_tests.c or nsalloc_tests.c (these test custom allocators which
don't apply to Rust).

## Remaining Failure Categories

### Encoding (38 tests) — Deferred

UTF-16, Latin-1, and unknown encoding tests. The Rust parser focuses on
UTF-8. These include UTF-16 BE/LE detection, BOM handling, Latin-1
transcoding, and the `XML_SetUnknownEncodingHandler` API.

### Namespace Processing (22 tests) — Not Yet Implemented

The Rust parser accepts `XML_ParserCreateNS` but doesn't resolve namespace
URIs or rewrite element names to `{URI}local` format. Needs:
- Namespace binding tracking (prefix → URI)
- Element/attribute name rewriting
- Prefix validation (unbound, reserved, double-colon)
- Namespace declaration handler integration

### Parser Behavior (52 tests) — Partially Fixed

Genuine behavioral differences. Key areas:

**Entity expansion** (~8 tests): General entity expansion in content and
attributes produces wrong callback data or doesn't detect recursion properly.

**Default handler** (~5 tests): DTD declarations should be forwarded to the
default handler when no specific handler is set. Partially fixed (prolog
forwarding done), but DTD internal subset forwarding incomplete.

**Foreign DTD** (~4 tests): `XML_UseForeignDTD` needs to trigger external
entity ref handler before parsing starts.

**Byte info** (~3 tests): `XML_GetCurrentByteCount` returns 0 — needs token
length tracking during handler callbacks.

**Resume edge cases** (~3 tests): Suspend/resume within internal entities
and parameter entities.

**Attribute metadata** (~2 tests): `XML_GetSpecifiedAttributeCount` and
`XML_GetIdAttributeIndex` need DTD-aware attribute tracking.

**GetBuffer edge cases** (~3 tests): Overflow detection for INT_MAX-sized
buffer requests.

### Stubs (5 tests) — By Design

- `test_accounting_precision` — `g_bytesScanned` counter not tracked
- `test_billion_laughs_attack_protection_api` — API accepts but doesn't enforce
- `test_amplification_isolated_external_parser` — Amplification not tracked
- `test_misc_alloc_create_parser{,_with_encoding}` — Custom allocators N/A for Rust

## Fixes Applied

| Fix | Tests Fixed |
|-----|------------|
| user_data first field in ParserHandle | Prevented segfaults |
| Read user_data at call time | All handler callbacks |
| 26 missing API functions | API completeness |
| Suspend/resume with data re-processing | 3 resume tests |
| Stop semantics (suspend-of-suspended) | 2 stop tests |
| Custom assert.h | Unlocked ~160 more tests |
| External entity user_data inheritance | Child parser callbacks |
| Default handler in content processor | test_cdata_default |
| Default handler in prolog processor | test_pi/comment_handled_in_default |
| GetBuffer/ParseBuffer implementation | 2 buffer tests |
| Empty final parse → NO_ELEMENTS | test_empty_parse |
| Negative len → INVALID_ARGUMENT | test_negative_len_parse{,_buffer} |
| Param entity parsing policy lock | test_user_parameters (partial) |
