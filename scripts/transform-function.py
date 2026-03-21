#!/usr/bin/env python3
"""
Extract a function from C2Rust output and prepare it for transformation
by a Claude haiku agent, along with context about the existing port's patterns.
"""

import re
import sys
import argparse


def extract_function(content: str, func_name: str) -> str:
    """Extract a single function body from Rust source."""
    lines = content.split('\n')
    i = 0
    while i < len(lines):
        # Match the function definition
        if re.search(rf'\bfn\s+{re.escape(func_name)}\s*[\(<]', lines[i]):
            start = i
            brace_depth = 0
            found_body = False
            j = i
            while j < len(lines):
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if '{' in lines[j]:
                    found_body = True
                if found_body and brace_depth == 0:
                    return '\n'.join(lines[start:j+1])
                j += 1
        i += 1
    return None


def extract_types_used(func_body: str, full_content: str) -> str:
    """Extract type definitions referenced by the function."""
    # Find type names used in the function
    type_refs = set()

    # Match type references (capitalized identifiers that aren't keywords)
    for m in re.finditer(r'\b([A-Z][A-Z_a-z0-9]+)\b', func_body):
        name = m.group(1)
        if name not in {'Some', 'None', 'Ok', 'Err', 'Box', 'Vec', 'String',
                        'Option', 'Result', 'Copy', 'Clone', 'Debug', 'Send',
                        'NULL', 'SEEK', 'FILE'}:
            type_refs.add(name)

    # Extract definitions of those types from the full content
    types = []
    lines = full_content.split('\n')
    for type_name in sorted(type_refs):
        for i, line in enumerate(lines):
            if (re.match(rf'\s*pub\s+(?:type|struct|enum|const)\s+{re.escape(type_name)}\b', line) or
                re.match(rf'\s*(?:type|struct|enum|const)\s+{re.escape(type_name)}\b', line)):
                # Grab the full definition
                if 'struct' in line or 'enum' in line:
                    brace_depth = 0
                    j = i
                    while j < len(lines):
                        brace_depth += lines[j].count('{') - lines[j].count('}')
                        if brace_depth <= 0 and '{' in lines[j]:
                            break
                        if brace_depth <= 0 and j > i:
                            break
                        j += 1
                    types.append('\n'.join(lines[i:j+1]))
                else:
                    types.append(lines[i])
                break

    return '\n\n'.join(types)


def get_c_original(c_file: str, func_name: str) -> str:
    """Try to extract the original C function for reference."""
    try:
        with open(c_file) as f:
            content = f.read()
    except FileNotFoundError:
        return None

    # Map Rust names back to C names
    c_name = func_name

    lines = content.split('\n')
    for i, line in enumerate(lines):
        if re.search(rf'\b{re.escape(c_name)}\s*\(', line) and ('{' in line or (i+1 < len(lines) and '{' in lines[i+1])):
            # Check if this is a definition (not declaration)
            # Look back for 'static' or return type
            start = i
            # Go back to find the start of the function signature
            while start > 0 and not re.match(r'^\s*$', lines[start-1]):
                # Check if previous line is end of another function
                if lines[start-1].strip() == '}':
                    break
                start -= 1

            brace_depth = 0
            found_body = False
            j = i
            while j < len(lines):
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if '{' in lines[j]:
                    found_body = True
                if found_body and brace_depth == 0:
                    return '\n'.join(lines[start:j+1])
                j += 1
    return None


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('c2rust_file', help='C2Rust output .rs file')
    parser.add_argument('function', help='Function name to extract')
    parser.add_argument('--c-source', help='Original C source file for reference')
    parser.add_argument('--existing-port', help='Existing Rust port file for convention reference')
    parser.add_argument('--include-types', action='store_true',
                        help='Include type definitions used by the function')
    parser.add_argument('--prompt', action='store_true',
                        help='Output as a transformation prompt for Claude')
    args = parser.parse_args()

    with open(args.c2rust_file) as f:
        c2rust_content = f.read()

    func_body = extract_function(c2rust_content, args.function)
    if not func_body:
        print(f"Function '{args.function}' not found in {args.c2rust_file}", file=sys.stderr)
        sys.exit(1)

    if args.prompt:
        print("# Task: Transform C2Rust function to idiomatic Rust")
        print()
        print("Transform the following C2Rust-generated function into safe, idiomatic Rust.")
        print("Follow the transformation rules in the guide below.")
        print()

        # Include transformation guide summary
        print("## Key Rules:")
        print("- `(*parser).m_field` → `self.field_name` (snake_case, no m_ prefix)")
        print("- Raw pointers → references and slices")
        print("- `XML_Bool` → `bool`")
        print("- malloc/free → Vec/Box")
        print("- Keep 1:1 function correspondence with C")
        print("- Preserve exact control flow and error paths")
        print("- Function becomes a method on Parser with snake_case name")
        print()

    if args.include_types:
        types = extract_types_used(func_body, c2rust_content)
        if types:
            if args.prompt:
                print("## Type Definitions (for reference)")
                print("```rust")
            print(types)
            if args.prompt:
                print("```")
            print()

    if args.prompt:
        print("## C2Rust Output (source of truth for logic)")
        print("```rust")
    print(func_body)
    if args.prompt:
        print("```")
        print()

    if args.c_source:
        c_func = get_c_original(args.c_source, args.function)
        if c_func:
            if args.prompt:
                print("## Original C (for reference)")
                print("```c")
            print(c_func)
            if args.prompt:
                print("```")
                print()

    if args.existing_port:
        with open(args.existing_port) as f:
            existing = f.read()
        if args.prompt:
            print("## Existing Port Conventions (first 300 lines for style reference)")
            print("```rust")
            print('\n'.join(existing.split('\n')[:300]))
            print("```")


if __name__ == '__main__':
    main()
