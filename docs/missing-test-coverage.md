# Missing Test Coverage: libexpat C vs expat-rust

This document catalogs every form of test/verification support in the upstream
libexpat C project that we do **not** currently run in expat-rust.

## Summary

| Category | C libexpat | expat-rust | Gap |
|----------|-----------|------------|-----|
| basic_tests.c (244 tests) | Yes | Yes (via c-tests-runner FFI) | **None** |
| ns_tests.c (33 tests) | Yes | Yes (via c-tests-runner FFI) | **None** |
| misc_tests.c (22 tests) | Yes | Yes (via c-tests-runner FFI) | **None** |
| acc_tests.c (4 tests) | Yes | 3 of 4 (via c-tests-runner FFI) | 1 skipped (see below) |
| alloc_tests.c (61 tests) | Yes | **Not run** | **61 tests** |
| nsalloc_tests.c (27 tests) | Yes | **Not run** | **27 tests** |
| C++ compilation (runtests_cxx) | Yes | **Not run** | Entire C++ suite |
| xmlwf tool | Yes | **Not built** | Tool + W3C test suite |
| W3C XML Test Suite (xmltest.sh) | Yes | **Not run** | ~1800+ conformance cases |
| Fuzz targets (3 fuzzers) | Yes (OSS-Fuzz) | **Not run** | 3 fuzz targets |
| Benchmark (tests/benchmark/) | Yes | Partial (Criterion) | Different methodology |
| Large file test data | Yes (testdata/largefiles/) | **Not used** | Stress testing |
| Examples (3 programs) | Yes | **Not built/tested** | 3 example programs |

## Detailed Gaps

### 1. alloc_tests.c — 61 tests NOT RUN (Not applicable to Rust)

**What it tests**: Every test uses `duff_allocator` — a fake `malloc` that returns
NULL after exactly N calls. The test loops from N=0 upward: at each step it checks
that the parser returns `XML_STATUS_ERROR` (not crash/segfault) when the Nth malloc
fails. This verifies that every single `malloc`/`realloc` call site in C checks for
NULL before dereferencing.

**Why it doesn't apply to Rust**: The bug class being tested — "forgot to check
malloc's return value" — cannot exist in Rust. Rust's `Vec::push`,
`String::push_str`, `HashMap::insert` etc. don't return NULL; they either succeed or
abort the process. There is no "graceful degradation" code path to test. You would
need to rewrite the entire parser with `try_reserve()` everywhere to make fallible
allocation possible, which is a fundamentally different design.

The `test_mem_api_*` tests exercise `XML_MemMalloc`/`XML_MemRealloc`/`XML_MemFree` —
C API functions that expose the parser's allocator. These don't exist in the Rust API.
The `test_alloc_tracker_*` tests exercise internal allocation-size tracking for the
billion laughs detector, which Rust implements differently.

**Verdict**: Correctly excluded. Not a gap.

### 2. nsalloc_tests.c — 27 tests NOT RUN (Not applicable to Rust)

**What it tests**: Identical pattern to alloc_tests.c (duff_allocator, fail-after-N
loop), targeting namespace processing code paths — long prefixes, long URIs, context
string reallocation.

**Why it doesn't apply to Rust**: Same reason — tests C-specific NULL-check
discipline. The bug class doesn't exist in Rust.

**Verdict**: Correctly excluded. Not a gap.

**27 test functions**:
- `test_nsalloc_xmlns` through `test_nsalloc_long_attr_prefix` (6) — basic NS allocation
- `test_nsalloc_realloc_attributes` through `test_nsalloc_realloc_long_prefix` (3) — realloc
- `test_nsalloc_realloc_longer_prefix` through `test_nsalloc_long_context` (3) — long strings
- `test_nsalloc_realloc_long_context` through `test_nsalloc_realloc_long_context_7` (7) — context realloc
- `test_nsalloc_realloc_long_ge_name` through `test_nsalloc_setContext_zombie` (4) — edge cases

### 3. test_accounting_precision — 1 test SKIPPED

**What it tests**: Verifies that `g_bytesScanned` (a global counter in C `xmlparse.c`)
accurately tracks bytes processed.

**Why skipped**: `g_bytesScanned` is a C-internal testing variable, not part of the
public API.

**What we're missing**: Low concern — this is a testing-only internal counter. Our
comparison tests verify byte position reporting through the public API
(`XML_GetCurrentByteIndex` etc.), which covers the same ground.

### 4. C++ Compilation (runtests_cxx) — ENTIRE SUITE NOT RUN

**What it tests**: The exact same 391 test cases, but compiled as C++ (`.cpp` files).
This verifies that libexpat's headers and API are C++-compatible.

**Why not run**: The Rust FFI layer (`expat-ffi`) only needs C compatibility, not C++.

**What we're missing**: If anyone uses `expat-ffi` from C++ code, we have no
verification that the headers compile cleanly as C++. This matters because C++ has
stricter type rules (e.g., implicit `void*` casts are errors in C++).

**Recommendation**: Low priority unless we explicitly claim C++ compatibility for
expat-ffi.

### 5. xmlwf Tool — NOT BUILT

**What it is**: A command-line XML well-formedness checker that ships with libexpat.
It's a real application (~8 source files) that exercises the parser in a file-oriented
mode (as opposed to the unit tests which use in-memory buffers).

**What we're missing**:
- An integration test that exercises the parser through a realistic application workflow
  (open file, read chunks, parse, report errors)
- The basis for running the W3C XML Test Suite (see next item)

### 6. W3C XML Test Suite (xmltest.sh) — NOT RUN

**What it tests**: The [W3C XML Test Suite](https://www.w3.org/XML/Test/xmlts20020606.zip)
contains ~1800+ test cases covering XML conformance. The `xmltest.sh` script runs
`xmlwf` against each test case and compares the result (pass/fail/output) against
expected outcomes.

**Why not run**: Requires building `xmlwf` (or a Rust equivalent), downloading the
W3C test suite, and adapting the shell script.

**What we're missing**: This is the industry-standard XML conformance test suite.
Our unit tests cover functionality, but the W3C suite tests *conformance to the spec*
across thousands of edge cases contributed by multiple XML implementors. Running this
would significantly strengthen our claim of "behavioral compatibility."

**Expected output baseline**: `expat/expat/tests/xmltest.log.expected` contains the
expected output from running the test suite against C libexpat.

### 7. Fuzz Targets — NOT RUN

**What they test**: Three fuzz harnesses exist in `expat/expat/fuzz/`:

| Fuzzer | What it does |
|--------|-------------|
| `xml_parse_fuzzer.c` | Feeds random bytes to `XML_Parse()` |
| `xml_parsebuffer_fuzzer.c` | Feeds random bytes to `XML_ParseBuffer()` |
| `xml_lpm_fuzzer.cpp` + `.proto` | Structure-aware fuzzing using libprotobuf-mutator |

**Why not run**: No Rust fuzz targets have been created. The C fuzzers are used with
OSS-Fuzz for the C library.

**What we're missing**: Fuzzing is one of the most effective ways to find bugs in
parsers. Since we're a new implementation, we're *more* likely to have bugs than the
battle-tested C version. We should be fuzzing more, not less.

**Recommendation**: Create Rust fuzz targets using `cargo-fuzz` (libFuzzer) or
`afl.rs`. At minimum:
1. A basic `fuzz_target!` that calls `Parser::new()` + `parser.parse(data, true)`
2. A comparison fuzzer that feeds the same input to both C and Rust and asserts
   identical SAX event traces (similar to our comparison tests but with random input)

The `test/compare-fuzz/` directory suggests some work was started on comparison
fuzzing but it's not integrated into CI.

### 8. Large File Test Data — NOT USED

**What it is**: `expat/testdata/largefiles/` contains large XML files for stress
testing and benchmarking.

**Why not used**: Our benchmarks use Criterion with synthetic documents, not these
files.

**What we're missing**: Real-world large XML files may exercise different code paths
than synthetic benchmarks.

### 9. Example Programs — NOT BUILT/TESTED

**What they are**: Three C example programs in `expat/expat/examples/`:
- `elements.c` — counts elements in an XML file
- `outline.c` — prints element tree structure
- `element_declarations.c` — prints DTD element declarations

**Why not built**: These are documentation/demo programs, not tests.

**What we're missing**: These serve as integration tests of the public API. Building
them against `expat-ffi` would verify the C drop-in replacement works for real programs.

### 10. Multi-Configuration Testing — PARTIAL

**What C does**: The C test suite runs each test with multiple configurations:
- 6 chunk sizes (0-5 bytes) — tests incremental/chunked parsing
- 2 reparse deferral settings (enabled/disabled)
- Total: 12 configurations per test = 4,692 test executions

**What we do**: The c-tests-runner runs tests once. Our Rust comparison tests also
generally use single-pass parsing.

**What we're missing**: We don't systematically test chunked parsing at every possible
chunk boundary. The C suite's multi-chunk approach catches bugs where the parser
incorrectly handles data split across `XML_Parse()` calls — a classic source of bugs
in streaming parsers.

## Priority Ranking

| Priority | Gap | Effort | Impact |
|----------|-----|--------|--------|
| **HIGH** | Fuzz targets | Medium | Finds bugs no other testing can |
| **HIGH** | W3C XML Test Suite | Medium | Industry-standard conformance |
| **HIGH** | Multi-chunk testing | Low-Medium | Catches streaming parser bugs |
| **N/A** | alloc_tests (OOM handling) | N/A | C-specific — tests NULL-check after malloc; Rust collections abort on OOM, no equivalent bug class |
| **N/A** | nsalloc_tests (OOM + NS) | N/A | Same — C malloc NULL-check testing, not applicable to Rust |
| **LOW** | xmlwf tool | Medium | Prerequisite for W3C suite |
| **LOW** | Large file test data | Low | Minor coverage improvement |
| **LOW** | C++ compilation | Low | Only if claiming C++ compat |
| **LOW** | Example programs | Low | Minor integration verification |
| **LOW** | test_accounting_precision | None | Covered by public API tests |
