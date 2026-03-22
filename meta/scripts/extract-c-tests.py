#!/usr/bin/env python3
"""Extract C test functions from expat test files into structured JSON.

Parses START_TEST(name) ... END_TEST blocks plus any static helper functions
that appear between tests. Outputs a JSON file per test source file containing
test descriptors with:
  - test name
  - test body (C source)
  - helper functions defined in the same file
  - which registration function was used (tcase_add_test, __ifdef_xml_dtd, __if_xml_ge)
  - external dependencies (handlers, common functions referenced)

Usage:
    python3 scripts/extract-c-tests.py [test_file.c ...]

If no files specified, processes all *_tests.c files in expat/tests/.
Output goes to expat-rust/tests/extracted/<basename>.json
"""

import json
import os
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
TESTS_DIR = REPO_ROOT / "expat" / "tests"
OUTPUT_DIR = REPO_ROOT / "expat-rust" / "tests" / "extracted"

# Patterns
START_TEST_RE = re.compile(r'^START_TEST\((\w+)\)\s*\{', re.MULTILINE)
END_TEST_RE = re.compile(r'^\}\s*\nEND_TEST', re.MULTILINE)

# Registration patterns
REG_NORMAL = re.compile(r'tcase_add_test\(\w+,\s*(\w+)\)')
REG_DTD = re.compile(r'tcase_add_test__ifdef_xml_dtd\(\s*\w+,\s*(\w+)\)')
REG_GE = re.compile(r'tcase_add_test__if_xml_ge\(\s*\w+,\s*(\w+)\)')

# Known external handler/helper patterns (from handlers.h, common.h, chardata.h, structdata.h)
KNOWN_EXTERNALS = {
    # From handlers.c/h
    'accumulate_characters', 'accumulate_start_element', 'accumulate_attribute',
    'start_element_event_handler', 'end_element_event_handler',
    'start_element_issue_240', 'end_element_issue_240',
    'external_entity_failer__if_not_xml_ge', 'external_entity_optioner',
    'external_entity_null_loader', 'external_entity_oneshot_loader',
    'suspend_after_element_declaration',
    # From common.c/h
    '_XML_Parse_SINGLE_BYTES', '_expect_failure', '_xml_failure',
    '_run_character_check', '_run_attribute_check', '_run_ext_character_check',
    'basic_teardown', 'duff_allocator', 'duff_reallocator', 'portable_strndup',
    # From chardata.c/h
    'CharData_Init', 'CharData_AppendXMLChars', 'CharData_CheckXMLChars',
    'CharData_CheckString',
    # From structdata.c/h
    'StructData_Init', 'StructData_AddItem', 'StructData_CheckItems',
    'StructData_Dispose',
    # From memcheck.c/h
    'tracking_malloc', 'tracking_realloc', 'tracking_free', 'tracking_report',
}


def extract_tests(filepath):
    """Extract all test functions and static helpers from a C test file."""
    with open(filepath) as f:
        content = f.read()

    tests = []
    helpers = []

    # Find all START_TEST ... END_TEST blocks
    test_starts = list(START_TEST_RE.finditer(content))

    for i, match in enumerate(test_starts):
        test_name = match.group(1)
        start_pos = match.start()

        # Find the matching END_TEST
        end_match = END_TEST_RE.search(content, match.end())
        if not end_match:
            print(f"  WARNING: No END_TEST found for {test_name}", file=sys.stderr)
            continue

        end_pos = end_match.end()
        test_body = content[start_pos:end_pos]

        # Find referenced external functions
        refs = set()
        for ext in KNOWN_EXTERNALS:
            if ext in test_body:
                refs.add(ext)

        tests.append({
            'name': test_name,
            'body': test_body,
            'line_start': content[:start_pos].count('\n') + 1,
            'line_end': content[:end_pos].count('\n') + 1,
            'external_refs': sorted(refs),
        })

    # Extract static helper functions (between tests or before first test)
    # Pattern: static return_type\nfunction_name(...) { ... }
    static_fn_re = re.compile(
        r'^(static\s+(?:void|int|char\s*\*|const\s+\w+|enum\s+\w+|unsigned)\s+(?:XMLCALL\s+)?'
        r'(\w+)\s*\([^)]*\)\s*\{)',
        re.MULTILINE
    )
    for match in static_fn_re.finditer(content):
        fn_name = match.group(2)
        fn_start = match.start()

        # Find the end of the function (matching braces)
        brace_count = 0
        pos = match.end() - 1  # start at the opening brace
        while pos < len(content):
            if content[pos] == '{':
                brace_count += 1
            elif content[pos] == '}':
                brace_count -= 1
                if brace_count == 0:
                    break
            pos += 1

        fn_body = content[fn_start:pos + 1]
        helpers.append({
            'name': fn_name,
            'body': fn_body,
            'line': content[:fn_start].count('\n') + 1,
        })

    # Also extract non-static XMLCALL functions defined in the file
    xmlcall_fn_re = re.compile(
        r'^(void\s+XMLCALL\s+(\w+)\s*\([^)]*\)\s*\{)',
        re.MULTILINE
    )
    for match in xmlcall_fn_re.finditer(content):
        fn_name = match.group(2)
        fn_start = match.start()

        brace_count = 0
        pos = match.end() - 1
        while pos < len(content):
            if content[pos] == '{':
                brace_count += 1
            elif content[pos] == '}':
                brace_count -= 1
                if brace_count == 0:
                    break
            pos += 1

        fn_body = content[fn_start:pos + 1]
        # Avoid duplicates
        if not any(h['name'] == fn_name for h in helpers):
            helpers.append({
                'name': fn_name,
                'body': fn_body,
                'line': content[:fn_start].count('\n') + 1,
            })

    # Determine registration info
    registration = {}
    for match in REG_NORMAL.finditer(content):
        registration[match.group(1)] = 'normal'
    for match in REG_DTD.finditer(content):
        registration[match.group(1)] = 'ifdef_xml_dtd'
    for match in REG_GE.finditer(content):
        registration[match.group(1)] = 'if_xml_ge'

    # Add registration info to tests
    for test in tests:
        test['registration'] = registration.get(test['name'], 'unknown')

    return {
        'source_file': str(filepath.relative_to(REPO_ROOT)),
        'test_count': len(tests),
        'helper_count': len(helpers),
        'tests': tests,
        'helpers': helpers,
    }


def main():
    if len(sys.argv) > 1:
        files = [Path(f) for f in sys.argv[1:]]
    else:
        files = sorted(TESTS_DIR.glob("*_tests.c"))

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    total_tests = 0
    for filepath in files:
        print(f"Extracting from {filepath.name}...", file=sys.stderr)
        result = extract_tests(filepath)
        total_tests += result['test_count']

        out_path = OUTPUT_DIR / f"{filepath.stem}.json"
        with open(out_path, 'w') as f:
            json.dump(result, f, indent=2)

        print(f"  {result['test_count']} tests, {result['helper_count']} helpers -> {out_path.name}",
              file=sys.stderr)

    print(f"\nTotal: {total_tests} tests extracted from {len(files)} files", file=sys.stderr)


if __name__ == '__main__':
    main()
