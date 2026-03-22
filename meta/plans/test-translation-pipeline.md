# Test Translation Pipeline: C Tests → Rust Tests

## Overview

This document describes the proven, script-driven pipeline for translating C test suites to Rust. The pipeline was developed and validated on libexpat's 391 C tests, achieving 365 translated Rust tests (93% coverage) — all compiling.

## Architecture

```
C test files (.c)
    │
    ▼
scripts/extract-c-tests.py          [deterministic script]
    │  Parses START_TEST...END_TEST blocks
    │  Extracts helpers, registration info, external refs
    │  Outputs JSON descriptors
    ▼
expat-rust/tests/extracted/*.json    [structured data]
    │
    ▼
scripts/prepare-test-translation-prompt.py   [deterministic script]
    │  Reads JSON + Rust API facade
    │  Generates complete translation prompt
    │  Includes translation rules, API mappings, examples
    ▼
Translation prompt (text)
    │
    ▼
Haiku subagent (via Claude Code Agent tool)  [AI judgment]
    │  Translates C → Rust following rules
    │  Handles callbacks, closures, error patterns
    │  Marks untranslatable tests as #[ignore]
    ▼
expat-rust/tests/*.rs                [Rust test files]
    │
    ▼
cargo test --test <name> --no-run    [deterministic validation]
    │  Compilation check — catches type errors, missing imports
    │  Tests compile against API facade (todo!() stubs)
    ▼
cargo test --test <name>             [runtime validation]
    │  Tests run once API has real implementations
    │  Panics from todo!() expected until implementation complete
```

## Step-by-Step Usage

### 1. Extract C tests to JSON

```bash
python3 scripts/extract-c-tests.py [test_file.c ...]
```

If no files specified, processes all `*_tests.c` in `expat/tests/`.

**Output**: `expat-rust/tests/extracted/<stem>.json` per file, containing:
```json
{
  "source_file": "expat/tests/misc_tests.c",
  "test_count": 22,
  "helper_count": 6,
  "tests": [
    {
      "name": "test_misc_null_parser",
      "body": "START_TEST(test_misc_null_parser) { ... } END_TEST",
      "line_start": 109,
      "line_end": 112,
      "registration": "normal",
      "external_refs": []
    }
  ],
  "helpers": [
    {
      "name": "parse_version",
      "body": "static int parse_version(...) { ... }",
      "line": 151
    }
  ]
}
```

**Registration types**: `normal`, `ifdef_xml_dtd`, `if_xml_ge`, `unknown`

### 2. Generate translation prompt

```bash
python3 scripts/prepare-test-translation-prompt.py <stem> [--batch START-END]
```

Generates a prompt to stdout containing:
- The Rust API facade (full source of xmlparse.rs)
- All helper functions from the C file
- The C test bodies to translate
- 25 translation rules (C pattern → Rust pattern)
- Requirements (no unsafe, preserve names, mark ignored tests)

**Batching**: For large files like basic_tests.c (244 tests), use `--batch 0-49` etc.

### 3. Feed prompt to Haiku subagent

In Claude Code, use the Agent tool:

```
Agent(
  description="Translate misc_tests to Rust",
  model="haiku",
  prompt=<contents of generated prompt + write instructions>
)
```

The agent:
- Reads the C test file and Rust API
- Translates each test following the rules
- Writes the output .rs file
- Runs `cargo test --test <name> --no-run` to verify compilation
- Fixes any errors

### 4. Validate compilation

```bash
cargo test --test <name> --no-run
```

Must compile cleanly. Tests will panic at runtime (todo!() stubs) until the parser is implemented.

## Translation Rules Reference

| C Pattern | Rust Equivalent |
|-----------|----------------|
| `XML_ParserCreate(NULL)` | `Parser::new(None).unwrap()` |
| `XML_ParserCreateNS(enc, sep)` | `Parser::new_ns(enc, sep).unwrap()` |
| `XML_Parse(p, text, len, final)` | `p.parse(text.as_bytes(), is_final)` |
| `XML_GetErrorCode(p)` | `p.error_code()` |
| `XML_StopParser(p, r)` | `p.stop(r)` |
| `XML_ResumeParser(p)` | `p.resume()` |
| `XML_ParserFree(p)` | `drop(p)` |
| `XML_ParserReset(p, enc)` | `p.reset(enc)` |
| `fail("msg")` | `panic!("msg")` |
| `assert_true(cond)` | `assert!(cond)` |
| `XCS("string")` | `"string"` |
| `_XML_Parse_SINGLE_BYTES(...)` | `p.parse(text.as_bytes(), is_final)` |
| `XML_SetCharacterDataHandler(p, h)` | `p.set_character_data_handler(Some(Box::new(\|data\| { ... })))` |
| `XML_SetElementHandler(p, s, e)` | `p.set_element_handlers(Some(Box::new(s)), Some(Box::new(e)))` |
| `CharData_Init/Append/Check` | Use `String` or `Vec<u8>` |
| Tests with `duff_allocator` | `#[test] #[ignore] // Requires custom allocator` |
| Tests with `#ifdef XML_DTD` | `#[test] #[ignore] // Requires XML_DTD feature` |
| Tests with `#if XML_GE` | `#[test] #[ignore] // Requires XML_GE feature` |

## Batching Strategy

For large test files, split into batches of ~50 tests and run haiku agents in parallel:

```
basic_tests.c (244 tests) → 5 batches × 50 tests → 5 parallel haiku agents
```

Each batch writes to a separate file (basic_tests_0.rs through basic_tests_4.rs).

**Why batch**: Haiku context window limits mean ~50 tests is the sweet spot. Smaller batches waste overhead; larger batches risk truncation.

## Results on libexpat

| Test File | C Tests | Rust Tests | Compilation | Notes |
|-----------|---------|------------|-------------|-------|
| misc_tests.c | 22 | 22 | ✅ | Simplest, good first target |
| ns_tests.c | 33 | 33 | ✅ | Namespace tests |
| basic_tests.c | 244 | 218 | ✅ | 5 batches, 26 tests simplified |
| alloc_tests.c | 61 | 61 | ✅ | All #[ignore] (need allocator) |
| nsalloc_tests.c | 27 | 27 | ✅ | All #[ignore] |
| acc_tests.c | 4 | 4 | ✅ | All #[ignore] (need XML_GE) |
| **Total** | **391** | **365** | **✅** | **93% coverage** |

## Model Tier Finding

**Haiku handles all test translation**, including:
- Simple tests (error strings, version info)
- Complex callback tests (element handlers, character data)
- Multi-case loop tests (entity validation)
- Namespace tests (triplets, prefix binding)

No test file required escalation to Sonnet. Haiku's main limitation is context window, solved by batching.

## Adapting for Other C Libraries

To use this pipeline on a different C library:

1. **Adjust `extract-c-tests.py`**: Change `START_TEST`/`END_TEST` patterns to match the target test framework (e.g., `TEST_F` for Google Test, `void test_*` for custom frameworks)
2. **Adjust `prepare-test-translation-prompt.py`**: Update the translation rules table for the target library's API
3. **Create Rust API facade**: Write todo!() stubs matching the C API
4. **Run the pipeline**: Extract → Generate prompts → Feed to Haiku → Validate compilation
