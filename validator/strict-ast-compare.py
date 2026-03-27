#!/usr/bin/env python3
"""Strict AST-based structural comparison of C and Rust implementations.

Uses tree-sitter to parse both C and Rust into ASTs, converts them to
a common semantic skeleton IR, applies rewrite rules to account for
known language differences, then structurally compares the results.

Every difference must be either:
1. Handled by a rewrite rule (known language difference)
2. Listed in deliberate-divergences.json (accepted structural divergence)
3. Reported as a mismatch (potential bug)

Usage:
    python3 strict-ast-compare.py <c_func> <rust_func>
    python3 strict-ast-compare.py --all
    python3 strict-ast-compare.py --ci
    python3 strict-ast-compare.py --dump <c_func> <rust_func>
    python3 strict-ast-compare.py --dump-c <c_func>
    python3 strict-ast-compare.py --dump-rust <rust_func>
"""

import sys
import os
import json

# Add parent of validator/ to path so strict_compare package is importable
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from strict_compare.nodes import SkeletonNode, Mismatch
from strict_compare.extract_c import parse_c, find_function as find_c_function, extract_skeleton as extract_c_skeleton
from strict_compare.extract_rust import parse_rust, find_function as find_rust_function, extract_skeleton as extract_rust_skeleton
from strict_compare.rewrite_rules import apply_all_rules, get_per_function_suppressions
from strict_compare.matcher import compare_skeletons
from strict_compare.reporter import format_text, format_json, print_skeleton

# Paths
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
C_FILE = os.path.join(ROOT, "expat", "expat", "lib", "xmlparse.c")
if not os.path.exists(C_FILE):
    C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")
DIVERGENCES_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "deliberate-divergences.json")


def load_function_pairs():
    """Load function pairs from deliberate-divergences.json."""
    with open(DIVERGENCES_FILE) as f:
        config = json.load(f)
    return config.get("function_pairs", [])


def load_suppressions():
    """Load per-function structural suppressions."""
    with open(DIVERGENCES_FILE) as f:
        config = json.load(f)
    return config.get("per_function_suppressions", {})


def compare_pair(c_func_name: str, r_func_name: str,
                 c_src: bytes = None, r_src: bytes = None,
                 dump: bool = False) -> list[Mismatch]:
    """Compare a single function pair."""
    if c_src is None:
        if not os.path.exists(C_FILE):
            print(f"Error: C source not found: {C_FILE}", file=sys.stderr)
            print(f"  Hint: git submodule update --init", file=sys.stderr)
            return []
        c_src = open(C_FILE, 'rb').read()
    if r_src is None:
        r_src = open(RUST_FILE, 'rb').read()

    c_tree = parse_c(c_src)
    r_tree = parse_rust(r_src)

    c_node = find_c_function(c_tree, c_func_name)
    r_node = find_rust_function(r_tree, r_func_name)

    if not c_node:
        print(f"  Warning: C function '{c_func_name}' not found", file=sys.stderr)
        return []
    if not r_node:
        print(f"  Warning: Rust function '{r_func_name}' not found", file=sys.stderr)
        return []

    # Extract skeletons
    c_skel = extract_c_skeleton(c_node, source_file="C")
    r_skel = extract_rust_skeleton(r_node, source_file="Rust")

    if dump:
        print_skeleton(c_skel, f"C: {c_func_name} (raw)")

    # Apply rewrite rules to C skeleton
    c_rewritten = apply_all_rules(c_skel)
    if c_rewritten is None:
        c_rewritten = SkeletonNode("sequence", source_file="C")

    if dump:
        print_skeleton(c_rewritten, f"C: {c_func_name} (after rewrites)")
        print_skeleton(r_skel, f"Rust: {r_func_name}")

    # Compare
    mismatches = compare_skeletons(c_rewritten, r_skel, context=f"{c_func_name}<->{r_func_name}")

    # Apply per-function suppressions from both sources
    # 1. From deliberate-divergences.json (legacy)
    suppressions = load_suppressions()
    func_sup = suppressions.get(r_func_name, {})
    suppressed_calls = set(func_sup.get("suppressed_calls", []))
    suppressed_errors = set(func_sup.get("suppressed_errors", []))
    suppressed_labels = set(func_sup.get("suppressed_labels", []))
    # 2. From structural-rewrites.json (new)
    rewrite_sup = get_per_function_suppressions(r_func_name)
    suppressed_calls |= set(rewrite_sup.get("suppressed_calls", []))
    suppressed_errors |= set(rewrite_sup.get("suppressed_errors", []))
    suppressed_labels |= set(rewrite_sup.get("suppressed_labels", []))

    filtered = []
    for m in mismatches:
        # Check if this mismatch is suppressed
        if _is_suppressed(m, suppressed_calls, suppressed_errors, suppressed_labels):
            continue
        filtered.append(m)

    return filtered


def _is_suppressed(m: Mismatch, suppressed_calls: set, suppressed_errors: set,
                   suppressed_labels: set = None) -> bool:
    """Check if a mismatch is covered by per-function suppressions."""
    if suppressed_labels is None:
        suppressed_labels = set()
    if m.c_node:
        # Direct call suppression
        if m.c_node.kind == "call" and m.c_node.label in suppressed_calls:
            return True
        # Error return suppression
        if m.c_node.kind == "return" and m.c_node.label:
            error = m.c_node.label.replace("XmlError::", "")
            if error in suppressed_errors:
                return True
        # Branch whose condition mentions a suppressed call or label
        if m.c_node.kind == "branch":
            label = m.c_node.label
            for call in suppressed_calls:
                if call in label:
                    return True
            for sup_label in suppressed_labels:
                if sup_label in label:
                    return True
        # Check node label against suppressed labels
        if m.c_node.label:
            for sup_label in suppressed_labels:
                if sup_label in m.c_node.label:
                    return True
    # Check if the reason text mentions a suppressed call or error
    reason = m.reason
    for call in suppressed_calls:
        if call in reason:
            return True
    for error in suppressed_errors:
        if error in reason:
            return True
    return False


def cmd_compare(c_func: str, r_func: str, dump=False):
    """Compare a single pair and print results."""
    mismatches = compare_pair(c_func, r_func, dump=dump)
    print(format_text(mismatches, c_func, r_func))
    return len([m for m in mismatches if m.severity == "ERROR"])


def cmd_dump_c(func_name: str):
    """Dump C skeleton for debugging."""
    c_src = open(C_FILE, 'rb').read()
    c_tree = parse_c(c_src)
    c_node = find_c_function(c_tree, func_name)
    if not c_node:
        print(f"Function '{func_name}' not found in C source")
        return
    skel = extract_c_skeleton(c_node, source_file="C")
    print_skeleton(skel, f"C: {func_name} (raw)")
    rewritten = apply_all_rules(skel)
    if rewritten:
        print_skeleton(rewritten, f"C: {func_name} (after rewrites)")


def cmd_dump_rust(func_name: str):
    """Dump Rust skeleton for debugging."""
    r_src = open(RUST_FILE, 'rb').read()
    r_tree = parse_rust(r_src)
    r_node = find_rust_function(r_tree, func_name)
    if not r_node:
        print(f"Function '{func_name}' not found in Rust source")
        return
    skel = extract_rust_skeleton(r_node, source_file="Rust")
    print_skeleton(skel, f"Rust: {func_name}")


def cmd_all(dump=False):
    """Compare all known function pairs."""
    pairs = load_function_pairs()
    total_errors = 0
    for pair in pairs:
        if pair.get("skip_structural"):
            print(f"  {pair['c_function']} <-> {pair['rust_function']}: SKIPPED (completely restructured)")
            continue
        c_func = pair["c_function"]
        r_func = pair["rust_function"]
        errors = cmd_compare(c_func, r_func, dump=dump)
        total_errors += errors
    return total_errors


def cmd_ci():
    """CI mode: compare all pairs, exit 1 if errors."""
    print("=== Strict AST Structural Comparison ===")
    errors = cmd_all()
    print(f"\nTotal errors: {errors}")
    if errors > 0:
        print("FAIL: Structural mismatches found")
        sys.exit(1)
    else:
        print("PASS: All function pairs structurally equivalent")


def cmd_json():
    """JSON output for all pairs."""
    pairs = load_function_pairs()
    c_src = open(C_FILE, 'rb').read()
    r_src = open(RUST_FILE, 'rb').read()
    results = []
    for pair in pairs:
        if pair.get("skip_structural"):
            continue
        c_func = pair["c_function"]
        r_func = pair["rust_function"]
        mismatches = compare_pair(c_func, r_func, c_src=c_src, r_src=r_src)
        results.append(format_json(mismatches, c_func, r_func))
    print(json.dumps(results, indent=2))


def main():
    args = sys.argv[1:]

    if not args:
        print(__doc__)
        sys.exit(0)

    if args[0] == "--ci":
        cmd_ci()
    elif args[0] == "--all":
        cmd_all()
    elif args[0] == "--json":
        cmd_json()
    elif args[0] == "--dump" and len(args) >= 3:
        cmd_compare(args[1], args[2], dump=True)
    elif args[0] == "--dump-c" and len(args) >= 2:
        cmd_dump_c(args[1])
    elif args[0] == "--dump-rust" and len(args) >= 2:
        cmd_dump_rust(args[1])
    elif len(args) >= 2:
        errors = cmd_compare(args[0], args[1])
        sys.exit(1 if errors > 0 else 0)
    else:
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
