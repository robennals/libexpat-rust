#!/usr/bin/env python3
"""Scoping-based structural comparison (v3).

Compares C and Rust functions by:
1. Extracting lossless common ASTs (from v2)
2. Building scoping trees (control flow structure) with content sets
3. Matching scoping trees and comparing content within each scope

Usage:
    python3 scope-compare.py <c_func> <rust_func>
    python3 scope-compare.py --dump <c_func> <rust_func>
    python3 scope-compare.py --all
    python3 scope-compare.py --ci
"""

import sys
import os
import json

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from strict_compare.common_ast import Node
from strict_compare.extract_common_c import parse_c, find_function as find_c_function, extract as extract_c
from strict_compare.extract_common_rust import parse_rust, find_function as find_rust_function, extract as extract_rust
from strict_compare.simplify import apply_simplifications, load_rules as load_simplification_rules
from strict_compare.scoping import extract_scoping_tree
from strict_compare.scope_match import compare_scopes, load_rules as load_scope_rules

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
C_FILE = os.path.join(ROOT, "expat", "expat", "lib", "xmlparse.c")
if not os.path.exists(C_FILE):
    C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")
DIVERGENCES_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                 "deliberate-divergences.json")


def compare_pair(c_func_name: str, r_func_name: str,
                 c_src=None, r_src=None, c_tree=None, r_tree=None,
                 dump=False):
    """Compare a single function pair."""
    if c_src is None:
        c_src = open(C_FILE, 'rb').read()
    if r_src is None:
        r_src = open(RUST_FILE, 'rb').read()
    if c_tree is None:
        c_tree = parse_c(c_src)
    if r_tree is None:
        r_tree = parse_rust(r_src)

    c_node = find_c_function(c_tree, c_func_name)
    r_node = find_rust_function(r_tree, r_func_name)

    if not c_node:
        print(f"  Warning: C function '{c_func_name}' not found", file=sys.stderr)
        return []
    if not r_node:
        print(f"  Warning: Rust function '{r_func_name}' not found", file=sys.stderr)
        return []

    c_body = c_node.child_by_field_name("body")
    r_body = r_node.child_by_field_name("body")
    c_ast = extract_c(c_body, "C") if c_body else Node("block")
    r_ast = extract_rust(r_body, "Rust") if r_body else Node("block")

    load_simplification_rules()
    c_simp = apply_simplifications(c_ast, "c")
    r_simp = apply_simplifications(r_ast, "rust")

    c_scope = extract_scoping_tree(c_simp)
    r_scope = extract_scoping_tree(r_simp)

    if dump:
        print(f"\n=== C: {c_func_name} (scoping tree) ===")
        print(c_scope.dump())
        print(f"\n=== Rust: {r_func_name} (scoping tree) ===")
        print(r_scope.dump())

    load_scope_rules()
    mismatches = compare_scopes(c_scope, r_scope,
                                 path=f"{c_func_name}<->{r_func_name}")
    return mismatches


def cmd_all():
    with open(DIVERGENCES_FILE) as f:
        config = json.load(f)

    c_src = open(C_FILE, 'rb').read()
    r_src = open(RUST_FILE, 'rb').read()
    c_tree = parse_c(c_src)
    r_tree = parse_rust(r_src)
    load_simplification_rules()
    load_scope_rules()

    total = 0
    passing = 0
    for pair in config["function_pairs"]:
        if pair.get("skip_structural"):
            print(f"  {pair['c_function']} <-> {pair['rust_function']}: SKIPPED")
            continue
        c_func = pair["c_function"]
        r_func = pair["rust_function"]
        mismatches = compare_pair(c_func, r_func,
                                   c_src=c_src, r_src=r_src,
                                   c_tree=c_tree, r_tree=r_tree)
        total += len(mismatches)
        if not mismatches:
            passing += 1
            print(f"  {c_func} <-> {r_func}: PASS")
        else:
            print(f"  {c_func} <-> {r_func}: {len(mismatches)} mismatch(es)")
            for m in mismatches[:5]:
                print(f"    {m}")
            if len(mismatches) > 5:
                print(f"    ... +{len(mismatches) - 5} more")

    n_compared = sum(1 for p in config["function_pairs"] if not p.get("skip_structural"))
    print(f"\n{passing}/{n_compared} pass, {total} total mismatches")
    return total


def main():
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        sys.exit(0)

    if args[0] == "--all":
        total = cmd_all()
        sys.exit(0)
    elif args[0] == "--ci":
        total = cmd_all()
        sys.exit(1 if total > 0 else 0)
    elif args[0] == "--dump" and len(args) >= 3:
        mismatches = compare_pair(args[1], args[2], dump=True)
        if not mismatches:
            print(f"\n  {args[1]} <-> {args[2]}: PASS")
        else:
            print(f"\n  {args[1]} <-> {args[2]}: {len(mismatches)} mismatch(es)")
            for m in mismatches:
                print(f"    {m}")
        sys.exit(1 if mismatches else 0)
    elif len(args) >= 2:
        mismatches = compare_pair(args[0], args[1])
        if not mismatches:
            print(f"  {args[0]} <-> {args[1]}: PASS")
        else:
            print(f"  {args[0]} <-> {args[1]}: {len(mismatches)} mismatch(es)")
            for m in mismatches:
                print(f"    {m}")
        sys.exit(1 if mismatches else 0)
    else:
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
