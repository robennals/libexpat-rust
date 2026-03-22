#!/usr/bin/env python3
"""Verify that every C test has a corresponding Rust test with the same name.

Compares extracted C test names (from JSON) against Rust test function names.
Reports missing, extra, and misnamed tests.

Usage:
    python3 scripts/verify-test-coverage.py

Exit code 0 if all C tests have Rust equivalents, 1 otherwise.
"""

import glob
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
EXTRACTED_DIR = REPO_ROOT / "expat-rust" / "tests" / "extracted"
RUST_TESTS_DIR = REPO_ROOT / "expat-rust" / "tests"


def get_c_test_names():
    """Get all test names from extracted JSON files."""
    names = {}
    for json_path in sorted(EXTRACTED_DIR.glob("*.json")):
        with open(json_path) as f:
            data = json.load(f)
        for test in data["tests"]:
            names[test["name"]] = json_path.stem
    return names


def get_rust_test_names():
    """Get all test function names from Rust test files."""
    names = {}
    for rs_path in sorted(RUST_TESTS_DIR.glob("*.rs")):
        with open(rs_path) as f:
            content = f.read()
        for m in re.finditer(r'#\[test\]\s*(?:#\[ignore\][^\n]*\n\s*)?fn\s+(\w+)\s*\(', content):
            names[m.group(1)] = rs_path.stem
    return names


def main():
    c_tests = get_c_test_names()
    rust_tests = get_rust_test_names()

    c_names = set(c_tests.keys())
    rust_names = set(rust_tests.keys())

    missing = c_names - rust_names
    extra = rust_names - c_names
    common = c_names & rust_names

    print(f"C tests:    {len(c_names)}")
    print(f"Rust tests: {len(rust_names)}")
    print(f"Matched:    {len(common)}")
    print()

    ok = True

    if missing:
        ok = False
        print(f"MISSING from Rust ({len(missing)}):")
        by_file = {}
        for name in sorted(missing):
            src = c_tests[name]
            by_file.setdefault(src, []).append(name)
        for src, names in sorted(by_file.items()):
            print(f"  {src} ({len(names)} missing):")
            for name in names:
                print(f"    {name}")
        print()

    if extra:
        print(f"EXTRA in Rust ({len(extra)}) — may be renamed:")
        by_file = {}
        for name in sorted(extra):
            src = rust_tests[name]
            by_file.setdefault(src, []).append(name)
        for src, names in sorted(by_file.items()):
            print(f"  {src} ({len(names)} extra):")
            for name in names:
                print(f"    {name}")
        print()

    if ok:
        print("✅ All C tests have Rust equivalents")
    else:
        print(f"❌ {len(missing)} C tests missing Rust equivalents")

    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
