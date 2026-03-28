#!/usr/bin/env python3
"""Canonical-form comparison (v4).

Rewrites C and Rust function bodies using pattern rules, then compares
the canonical forms. The rewrite rules normalize language-specific
patterns to a common form.

Usage:
    python3 canonical-compare.py <c_func> <rust_func>
    python3 canonical-compare.py --dump <c_func> <rust_func>
    python3 canonical-compare.py --all
"""

import sys
import os
import json

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from strict_compare.source_rewriter import load_rules, rewrite

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
C_FILE = os.path.join(ROOT, "expat", "expat", "lib", "xmlparse.c")
if not os.path.exists(C_FILE):
    C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")
DIVERGENCES_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                 "deliberate-divergences.json")
C_RULES_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                              "c-rewrites.yaml")
RUST_RULES_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                "rust-rewrites.yaml")


def extract_function_body(source: str, func_name: str, lang: str) -> str | None:
    """Extract the body of a function from source text.

    Returns the text between the opening { and closing } of the function.
    """
    if lang == "c":
        # Find the function DEFINITION (not declaration).
        # A definition has a { body after the closing ) of parameters.
        search_start = 0
        idx = -1
        while True:
            pos = source.find(func_name + "(", search_start)
            if pos < 0:
                break
            # Check: is this a definition? Find the closing ) then check for {
            paren_depth = 0
            scan = pos + len(func_name)
            while scan < len(source):
                if source[scan] == "(":
                    paren_depth += 1
                elif source[scan] == ")":
                    paren_depth -= 1
                    if paren_depth == 0:
                        break
                scan += 1
            # After closing ), skip whitespace and check for {
            scan += 1
            while scan < len(source) and source[scan] in " \t\n\r":
                scan += 1
            if scan < len(source) and source[scan] == "{":
                idx = pos
                break
            search_start = pos + 1
        if idx < 0:
            return None
    else:
        # Rust: find "fn func_name("
        idx = source.find(f"fn {func_name}(")
        if idx < 0:
            idx = source.find(f"fn {func_name}<")
        if idx < 0:
            return None

    # Find the opening brace
    brace_start = source.find("{", idx)
    if brace_start < 0:
        return None

    # Find matching closing brace
    depth = 1
    pos = brace_start + 1
    while pos < len(source) and depth > 0:
        if source[pos] == "{":
            depth += 1
        elif source[pos] == "}":
            depth -= 1
        pos += 1

    if depth != 0:
        return None

    return source[brace_start + 1:pos - 1]


def compare_pair(c_func: str, r_func: str, dump: bool = False):
    """Compare a single function pair using canonical form."""
    c_src = open(C_FILE).read()
    r_src = open(RUST_FILE).read()

    c_body = extract_function_body(c_src, c_func, "c")
    r_body = extract_function_body(r_src, r_func, "rust")

    if c_body is None:
        print(f"  Warning: C function '{c_func}' not found", file=sys.stderr)
        return None
    if r_body is None:
        print(f"  Warning: Rust function '{r_func}' not found", file=sys.stderr)
        return None

    # Load and apply rewrite rules
    c_rules = load_rules(C_RULES_FILE)
    r_rules = load_rules(RUST_RULES_FILE)

    c_canonical = rewrite(c_body, c_rules)
    r_canonical = rewrite(r_body, r_rules)

    if dump:
        print(f"\n=== C: {c_func} (canonical) ===")
        print(c_canonical[:500])
        if len(c_canonical) > 500:
            print(f"... ({len(c_canonical)} chars total)")
        print(f"\n=== Rust: {r_func} (canonical) ===")
        print(r_canonical[:500])
        if len(r_canonical) > 500:
            print(f"... ({len(r_canonical)} chars total)")

    return c_canonical, r_canonical


def main():
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        sys.exit(0)

    dump = False
    if args[0] == "--dump":
        dump = True
        args = args[1:]

    if args[0] == "--all":
        with open(DIVERGENCES_FILE) as f:
            config = json.load(f)
        for pair in config["function_pairs"]:
            if pair.get("skip_structural"):
                print(f"  {pair['c_function']} <-> {pair['rust_function']}: SKIPPED")
                continue
            result = compare_pair(pair["c_function"], pair["rust_function"], dump=dump)
            if result is None:
                continue
            c_can, r_can = result
            # For now just show lengths — real comparison TBD
            print(f"  {pair['c_function']} <-> {pair['rust_function']}: "
                  f"C={len(c_can)} chars, R={len(r_can)} chars")
    elif len(args) >= 2:
        result = compare_pair(args[0], args[1], dump=dump)
        if result:
            c_can, r_can = result
            print(f"  C canonical: {len(c_can)} chars")
            print(f"  R canonical: {len(r_can)} chars")
    else:
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
