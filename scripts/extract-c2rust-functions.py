#!/usr/bin/env python3
"""
Extract function signatures and bodies from C2Rust output.

This helps us:
1. Identify which C functions exist in the C2Rust output
2. Compare against the existing Rust port to find gaps
3. Extract individual functions for targeted transformation
"""

import re
import sys
import json
import argparse


def extract_functions(content: str) -> list:
    """Extract all function definitions from Rust source."""
    functions = []
    lines = content.split('\n')
    i = 0

    while i < len(lines):
        line = lines[i]

        # Match function definitions
        m = re.match(
            r'\s*(pub\s+)?(unsafe\s+)?(extern\s+"C"\s+)?fn\s+(\w+)\s*\(',
            line
        )
        if m:
            func_name = m.group(4)
            start_line = i + 1  # 1-indexed

            # Find the opening brace
            brace_depth = 0
            sig_lines = []
            j = i
            found_body_start = False

            while j < len(lines):
                sig_lines.append(lines[j])
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if '{' in lines[j] and not found_body_start:
                    found_body_start = True
                if found_body_start and brace_depth == 0:
                    end_line = j + 1  # 1-indexed
                    body = '\n'.join(sig_lines)

                    # Extract signature (everything before first {)
                    full_text = '\n'.join(sig_lines)
                    sig_end = full_text.index('{')
                    signature = full_text[:sig_end].strip()

                    functions.append({
                        'name': func_name,
                        'start_line': start_line,
                        'end_line': end_line,
                        'line_count': end_line - start_line + 1,
                        'signature': signature,
                        'is_pub': bool(m.group(1)),
                        'is_unsafe': bool(m.group(2)),
                        'is_extern_c': bool(m.group(3)),
                    })
                    break
                j += 1
        i += 1

    return functions


def compare_with_existing(c2rust_funcs: list, existing_file: str) -> dict:
    """Compare C2Rust functions with existing Rust port."""
    with open(existing_file) as f:
        existing_content = f.read()

    existing_funcs = extract_functions(existing_content)
    existing_names = {f['name'] for f in existing_funcs}
    c2rust_names = {f['name'] for f in c2rust_funcs}

    return {
        'only_in_c2rust': sorted(c2rust_names - existing_names),
        'only_in_existing': sorted(existing_names - c2rust_names),
        'in_both': sorted(c2rust_names & existing_names),
        'c2rust_count': len(c2rust_names),
        'existing_count': len(existing_names),
    }


def main():
    parser = argparse.ArgumentParser(description='Extract functions from C2Rust output')
    parser.add_argument('input', help='Input .rs file (C2Rust output)')
    parser.add_argument('--compare', help='Existing Rust port file to compare against')
    parser.add_argument('--json', action='store_true', help='Output as JSON')
    parser.add_argument('--names-only', action='store_true', help='Only output function names')
    parser.add_argument('--extract', help='Extract a specific function by name')
    args = parser.parse_args()

    with open(args.input) as f:
        content = f.read()

    functions = extract_functions(content)

    if args.extract:
        for func in functions:
            if func['name'] == args.extract:
                lines = content.split('\n')
                print('\n'.join(lines[func['start_line']-1:func['end_line']]))
                return
        print(f"Function '{args.extract}' not found", file=sys.stderr)
        sys.exit(1)

    if args.compare:
        comparison = compare_with_existing(functions, args.compare)
        if args.json:
            print(json.dumps(comparison, indent=2))
        else:
            print(f"C2Rust functions: {comparison['c2rust_count']}")
            print(f"Existing port functions: {comparison['existing_count']}")
            print(f"\nIn C2Rust but NOT in existing port ({len(comparison['only_in_c2rust'])}):")
            for name in comparison['only_in_c2rust']:
                # Find size
                for f in functions:
                    if f['name'] == name:
                        print(f"  {name} ({f['line_count']} lines)")
                        break
            print(f"\nIn existing port but NOT in C2Rust ({len(comparison['only_in_existing'])}):")
            for name in comparison['only_in_existing']:
                print(f"  {name}")
            print(f"\nIn both ({len(comparison['in_both'])}):")
            for name in comparison['in_both']:
                print(f"  {name}")
    elif args.names_only:
        for func in functions:
            pub = "pub " if func['is_pub'] else ""
            unsafe = "unsafe " if func['is_unsafe'] else ""
            print(f"{pub}{unsafe}{func['name']} (lines {func['start_line']}-{func['end_line']}, {func['line_count']} lines)")
    elif args.json:
        print(json.dumps(functions, indent=2))
    else:
        print(f"Found {len(functions)} functions:")
        total_lines = sum(f['line_count'] for f in functions)
        print(f"Total lines in functions: {total_lines}")
        print(f"\nTop 20 largest functions:")
        for func in sorted(functions, key=lambda f: -f['line_count'])[:20]:
            pub = "pub " if func['is_pub'] else ""
            print(f"  {pub}{func['name']}: {func['line_count']} lines")


if __name__ == '__main__':
    main()
