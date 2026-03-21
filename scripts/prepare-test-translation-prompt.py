#!/usr/bin/env python3
"""Prepare a prompt for an AI agent to translate C tests to Rust.

Reads extracted test JSON and the Rust API facade, then outputs a prompt
that can be fed to a Claude subagent.

Usage:
    python3 scripts/prepare-test-translation-prompt.py misc_tests [--batch 0-9]

Output: writes prompt to stdout, suitable for piping to an agent.
"""

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
EXTRACTED_DIR = REPO_ROOT / "expat-rust" / "tests" / "extracted"
RUST_API = REPO_ROOT / "expat-rust" / "src" / "xmlparse.rs"

def main():
    if len(sys.argv) < 2:
        print("Usage: prepare-test-translation-prompt.py <test_file_stem> [--batch START-END]", file=sys.stderr)
        sys.exit(1)

    stem = sys.argv[1]
    json_path = EXTRACTED_DIR / f"{stem}.json"

    # Parse optional batch range
    batch_start, batch_end = 0, None
    if "--batch" in sys.argv:
        idx = sys.argv.index("--batch")
        range_str = sys.argv[idx + 1]
        parts = range_str.split("-")
        batch_start = int(parts[0])
        batch_end = int(parts[1]) + 1 if len(parts) > 1 else int(parts[0]) + 1

    with open(json_path) as f:
        data = json.load(f)

    with open(RUST_API) as f:
        rust_api = f.read()

    tests = data['tests'][batch_start:batch_end]
    helpers = data['helpers']

    # Build the prompt
    prompt_parts = []

    prompt_parts.append(f"""You are translating C tests from {data['source_file']} to Rust.

## Rust API (the public interface tests should use)

```rust
{rust_api}
```

## C Helper Functions (defined in this test file)
""")

    for h in helpers:
        prompt_parts.append(f"```c\n// {h['name']}\n{h['body']}\n```\n")

    prompt_parts.append(f"""
## Tests to Translate ({len(tests)} tests)

For each C test below, write an equivalent Rust `#[test]` function.

### Translation Rules:
1. `XML_ParserCreate(NULL)` → `Parser::new(None).unwrap()`
2. `XML_ParserCreate(encoding)` → `Parser::new(Some(encoding)).unwrap()`
3. `XML_ParserCreateNS(enc, sep)` → `Parser::new_ns(enc, sep).unwrap()`
4. `XML_Parse(parser, text, len, isFinal)` → `parser.parse(text.as_bytes(), is_final)`
5. `XML_GetErrorCode(parser)` → `parser.error_code()`
6. `XML_StopParser(parser, resumable)` → `parser.stop(resumable)`
7. `XML_ResumeParser(parser)` → `parser.resume()`
8. `XML_ParserFree(parser)` → (automatic via Drop, or explicit drop(parser))
9. `XML_ParserReset(parser, enc)` → `parser.reset(enc)`
10. `fail("msg")` → `panic!("msg")`
11. `assert_true(cond)` → `assert!(cond)`
12. `XML_SetCharacterDataHandler(parser, handler)` → `parser.set_character_data_handler(Some(Box::new(|data| {{ ... }})))`
13. `XML_SetElementHandler(parser, start, end)` → `parser.set_element_handlers(Some(Box::new(...)), Some(Box::new(...)))`
14. `XCS("string")` → just `"string"`
15. `_XML_Parse_SINGLE_BYTES(parser, text, len, isFinal)` → `parser.parse(text.as_bytes(), is_final)` (single-byte feeding will be a helper)
16. `XML_ExpatVersion()` → `expat_rust::xmlparse::expat_version()`
17. `XML_ExpatVersionInfo()` → `expat_rust::xmlparse::expat_version_info()`
18. `XML_GetFeatureList()` → `expat_rust::xmlparse::get_feature_list()`
19. `XML_ErrorString(code)` → `expat_rust::xmlparse::error_string(code)`
20. `XML_SetParamEntityParsing(parser, mode)` → `parser.set_param_entity_parsing(mode)`
21. `XML_ExternalEntityParserCreate(parser, ctx, enc)` → `parser.create_external_entity_parser(ctx, enc)`
22. `XML_GetCurrentLineNumber(parser)` → `parser.current_line_number()`
23. `XML_GetCurrentColumnNumber(parser)` → `parser.current_column_number()`
24. C string literals with embedded NULs → use byte strings b"..."
25. Skip tests that heavily depend on C memory management (duff_allocator, tracking_malloc)
    — mark them with `#[test] #[ignore] // Requires custom allocator`
26. Skip tests behind #ifdef XML_DTD or #if XML_GE with `#[ignore]` and a comment
27. Use `use expat_rust::xmlparse::*;` at the top

### Important:
- Do NOT use `unsafe`
- Every test function should have `#[test]` attribute
- Tests will compile but panic at runtime (API is todo!() stubs) — that's expected
- Preserve the original test name
- Add a comment with the original line numbers

Output a single Rust file with all translated tests.
""")

    for t in tests:
        reg_note = ""
        if t['registration'] == 'ifdef_xml_dtd':
            reg_note = " [REQUIRES XML_DTD]"
        elif t['registration'] == 'if_xml_ge':
            reg_note = " [REQUIRES XML_GE]"

        prompt_parts.append(f"### {t['name']}{reg_note} (lines {t['line_start']}-{t['line_end']})\n```c\n{t['body']}\n```\n")

    print("".join(prompt_parts))


if __name__ == '__main__':
    main()
