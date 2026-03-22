# C Test Coverage Assessment

## Test Count by File

| Test File | Tests | Complexity | Translation Priority |
|-----------|-------|------------|---------------------|
| misc_tests.c | 22 | Low-Medium | 1st (simplest) |
| ns_tests.c | 33 | Medium | 2nd |
| basic_tests.c | 244 | Very High | 3rd (phased) |
| alloc_tests.c | 61 | Medium-High | 4th |
| nsalloc_tests.c | 27 | Medium-High | 5th |
| acc_tests.c | 4 | Low (conditional) | 6th (lowest priority) |
| **TOTAL** | **391** | | |

## API Coverage

67 XML_* API functions are tested, including:
- All parser lifecycle (Create, Free, Reset, Parse, ParseBuffer)
- All 22+ handler registration functions
- All configuration functions
- All query/state functions
- Memory management functions

## Coverage Gaps

1. **Reparse deferral** — basic tests only
2. **Buffer management edge cases** — limited GetBuffer coverage
3. **Parsing status transitions** — not exhaustive
4. **Advanced GE features** — only 4 tests
5. **Performance/scaling** — minimal
6. **Concurrent parsing** — none (not relevant for Rust port)

## Translation Strategy

1. Start with `misc_tests.c` — no callbacks needed, tests error strings and version info
2. Then `ns_tests.c` — uses basic callbacks, good for establishing patterns
3. Then `basic_tests.c` in phases — largest file, build up from simple to complex
4. Alloc tests require custom memory handling infrastructure
5. `acc_tests.c` last — requires XML_GE=1 feature flag

## Key Test Infrastructure Files

- `common.c` — `_XML_Parse_SINGLE_BYTES()`, `expect_failure()`, `run_*_check()`
- `chardata.c` — Character data accumulation + verification helpers
- `structdata.c` — Element structure recording helpers
- `handlers.c` — ~40 callback implementations
- `minicheck.c` — Unit test framework (START_TEST/END_TEST macros)
