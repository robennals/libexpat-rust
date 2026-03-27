#!/usr/bin/env python3
"""Strict lossless AST comparison (v2).

Compares C and Rust functions by:
1. Parsing both with tree-sitter into lossless common ASTs
2. Applying simplification rules (normalize naming, strip C/Rust syntax noise)
3. Comparing top-down with bidirectional match rules

Every difference must be covered by a match rule or it's an error.
Extra nodes on either side are errors.

Usage:
    python3 strict-compare-v2.py --dump <c_func> <rust_func>
    python3 strict-compare-v2.py <c_func> <rust_func>
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from strict_compare.common_ast import Node
from strict_compare.extract_common_c import parse_c, find_function as find_c_function, extract as extract_c
from strict_compare.extract_common_rust import parse_rust, find_function as find_rust_function, extract as extract_rust
from strict_compare.simplify import apply_simplifications, load_rules as load_simplification_rules
from strict_compare.strict_match import compare, load_match_rules

# Paths
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
C_FILE = os.path.join(ROOT, "expat", "expat", "lib", "xmlparse.c")
if not os.path.exists(C_FILE):
    C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")


def compare_pair(c_func_name: str, r_func_name: str, dump: bool = False):
    """Compare a single function pair using the v2 engine."""
    c_src = open(C_FILE, 'rb').read()
    r_src = open(RUST_FILE, 'rb').read()

    c_tree = parse_c(c_src)
    r_tree = parse_rust(r_src)

    c_node = find_c_function(c_tree, c_func_name)
    r_node = find_rust_function(r_tree, r_func_name)

    if not c_node:
        print(f"Error: C function '{c_func_name}' not found", file=sys.stderr)
        return []
    if not r_node:
        print(f"Error: Rust function '{r_func_name}' not found", file=sys.stderr)
        return []

    # Extract function bodies to common AST
    c_body = c_node.child_by_field_name("body")
    r_body = r_node.child_by_field_name("body")

    c_ast = extract_c(c_body, source_file="C") if c_body else Node("block")
    r_ast = extract_rust(r_body, source_file="Rust") if r_body else Node("block")

    if dump:
        print(f"\n=== C: {c_func_name} (raw common AST) ===")
        print(c_ast.dump())

    # Apply simplification rules
    load_simplification_rules()
    c_simplified = apply_simplifications(c_ast, "c")
    r_simplified = apply_simplifications(r_ast, "rust")

    if dump:
        print(f"\n=== C: {c_func_name} (after simplification) ===")
        print(c_simplified.dump())
        print(f"\n=== Rust: {r_func_name} (after simplification) ===")
        print(r_simplified.dump())

    # Compare with match rules
    load_match_rules()
    mismatches = compare(c_simplified, r_simplified,
                         path=f"{c_func_name}<->{r_func_name}")

    return mismatches


def generate_prompt(c_func_name: str, r_func_name: str) -> str:
    """Generate a fix prompt for mismatches between a function pair."""
    c_src = open(C_FILE, 'rb').read()
    r_src = open(RUST_FILE, 'rb').read()
    c_lines = c_src.decode().split('\n')
    r_lines = r_src.decode().split('\n')

    mismatches = compare_pair(c_func_name, r_func_name)
    if not mismatches:
        return f"# {c_func_name} <-> {r_func_name}: already matched, no fixes needed."

    prompt_parts = []
    prompt_parts.append(f"# Fix Rust function `{r_func_name}` to match C function `{c_func_name}`")
    prompt_parts.append(f"# Rust file: expat-rust/src/xmlparse.rs")
    prompt_parts.append(f"# C file: expat/expat/lib/xmlparse.c")
    prompt_parts.append(f"# {len(mismatches)} structural difference(s) found:")
    prompt_parts.append("")

    for i, m in enumerate(mismatches):
        prompt_parts.append(f"## Difference {i+1}")
        prompt_parts.append("")

        if m.c_node and not m.r_node:
            # Extra C node — Rust is missing something
            line = m.c_node.line
            ctx_start = max(0, line - 3)
            ctx_end = min(len(c_lines), line + 4)
            prompt_parts.append(f"C has code at line {line} that Rust is missing:")
            prompt_parts.append("```c")
            for l in range(ctx_start, ctx_end):
                marker = ">>> " if l == line - 1 else "    "
                prompt_parts.append(f"{marker}{l+1}: {c_lines[l]}")
            prompt_parts.append("```")
            prompt_parts.append("")
            prompt_parts.append(f"**Action**: Add equivalent Rust code to `{r_func_name}` that does what this C code does.")
            prompt_parts.append(f"The C node is: `{m.c_node.kind}({m.c_node.value})`")

        elif m.r_node and not m.c_node:
            # Extra Rust node — Rust has something C doesn't
            line = m.r_node.line
            ctx_start = max(0, line - 3)
            ctx_end = min(len(r_lines), line + 4)
            prompt_parts.append(f"Rust has code at line {line} that C doesn't have:")
            prompt_parts.append("```rust")
            for l in range(ctx_start, ctx_end):
                marker = ">>> " if l == line - 1 else "    "
                prompt_parts.append(f"{marker}{l+1}: {r_lines[l]}")
            prompt_parts.append("```")
            prompt_parts.append("")
            prompt_parts.append(f"**Action**: Investigate whether this Rust code should exist. "
                                f"If it's Rust-specific boilerplate (RAII, let binding), it may be fine. "
                                f"If it's extra logic not in C, it may be a divergence that needs to be removed or the C logic restructured to match.")
            prompt_parts.append(f"The Rust node is: `{m.r_node.kind}({m.r_node.value})`")

        else:
            # Unmatched pair
            prompt_parts.append(f"C and Rust have different code at the same structural position:")
            if m.c_node:
                line = m.c_node.line
                prompt_parts.append(f"C (line {line}): `{m.c_node.kind}({m.c_node.value})`")
            if m.r_node:
                line = m.r_node.line
                prompt_parts.append(f"Rust (line {line}): `{m.r_node.kind}({m.r_node.value})`")
            prompt_parts.append(f"**Action**: Make the Rust code structurally match the C code.")

        prompt_parts.append("")

    return "\n".join(prompt_parts)


def main():
    args = sys.argv[1:]

    if not args:
        print(__doc__)
        sys.exit(0)

    dump = False
    prompt_mode = False
    if args[0] == "--dump":
        dump = True
        args = args[1:]
    elif args[0] == "--prompt":
        prompt_mode = True
        args = args[1:]
    elif args[0] == "--prompt-all":
        # Generate prompts for all failing pairs
        import json as _json
        with open(os.path.join(os.path.dirname(os.path.abspath(__file__)),
                               "deliberate-divergences.json")) as f:
            config = _json.load(f)
        load_simplification_rules()
        load_match_rules()
        for pair in config["function_pairs"]:
            if pair.get("skip_structural"):
                continue
            c_func = pair["c_function"]
            r_func = pair["rust_function"]
            mismatches = compare_pair(c_func, r_func)
            if mismatches:
                print(generate_prompt(c_func, r_func))
                print("\n" + "=" * 70 + "\n")
        sys.exit(0)

    if len(args) >= 2:
        c_func = args[0]
        r_func = args[1]

        if prompt_mode:
            print(generate_prompt(c_func, r_func))
            sys.exit(0)

        mismatches = compare_pair(c_func, r_func, dump=dump)

        if not mismatches:
            print(f"  {c_func} <-> {r_func}: MATCH (structurally identical)")
        else:
            print(f"  {c_func} <-> {r_func}: {len(mismatches)} mismatch(es)")
            for m in mismatches[:20]:
                print(f"    {m}")
            if len(mismatches) > 20:
                print(f"    ... and {len(mismatches) - 20} more")

        sys.exit(1 if mismatches else 0)
    else:
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
