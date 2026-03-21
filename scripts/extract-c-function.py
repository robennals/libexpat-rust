#!/usr/bin/env python3
"""Extract a C function from xmlparse.c with its context.

Usage:
    python3 scripts/extract-c-function.py doProlog
    python3 scripts/extract-c-function.py storeAtts --with-structs
    python3 scripts/extract-c-function.py doProlog --prompt  # Generate agent prompt
    python3 scripts/extract-c-function.py --list  # List all functions with line counts

Extracts the function body and optionally related struct definitions.
"""

import re
import sys
import os

C_FILE = os.path.join(os.path.dirname(__file__), "..", "expat", "lib", "xmlparse.c")

def find_functions(source_lines):
    """Find all function definitions with their line ranges."""
    functions = {}
    i = 0
    while i < len(source_lines):
        line = source_lines[i]
        # Match function definition patterns (not declarations)
        # C functions start at column 0 with return type, then name on same or next line
        if (i + 1 < len(source_lines) and
            re.match(r'^(static\s+)?(\w+\s+)+\w+\s*$', line.rstrip()) and
            re.match(r'^\w+\(', source_lines[i + 1].rstrip())):
            # Function name is on next line
            m = re.match(r'^(\w+)\(', source_lines[i + 1])
            if m:
                func_name = m.group(1)
                start = i
                # Find the end of the function (matching braces)
                brace_depth = 0
                j = i
                found_open = False
                while j < len(source_lines):
                    for ch in source_lines[j]:
                        if ch == '{':
                            brace_depth += 1
                            found_open = True
                        elif ch == '}':
                            brace_depth -= 1
                            if found_open and brace_depth == 0:
                                functions[func_name] = (start, j + 1)
                                j = len(source_lines)  # break outer
                                break
                    j += 1
        # Also match single-line function signatures
        m = re.match(r'^(?:static\s+)?(?:enum\s+\w+|int|void|unsigned|XML_\w+|const\s+\w+)\s+(?:PTRCALL\s+)?(\w+)\s*\(', line)
        if m and '{' not in line:
            func_name = m.group(1)
            start = i
            brace_depth = 0
            j = i
            found_open = False
            while j < len(source_lines):
                for ch in source_lines[j]:
                    if ch == '{':
                        brace_depth += 1
                        found_open = True
                    elif ch == '}':
                        brace_depth -= 1
                        if found_open and brace_depth == 0:
                            functions[func_name] = (start, j + 1)
                            j = len(source_lines)
                            break
                j += 1
        i += 1
    return functions

def extract_structs(source_lines):
    """Extract all struct/typedef definitions."""
    structs = []
    i = 0
    while i < len(source_lines):
        line = source_lines[i]
        if re.match(r'^typedef\s+struct', line) or re.match(r'^typedef\s+\{', line):
            start = i
            brace_depth = 0
            j = i
            while j < len(source_lines):
                for ch in source_lines[j]:
                    if ch == '{':
                        brace_depth += 1
                    elif ch == '}':
                        brace_depth -= 1
                if brace_depth == 0 and j > i:
                    structs.append((start, j + 1))
                    break
                j += 1
        i += 1
    return structs

def main():
    with open(C_FILE) as f:
        lines = f.readlines()

    if '--list' in sys.argv:
        functions = find_functions(lines)
        sorted_funcs = sorted(functions.items(), key=lambda x: x[1][1] - x[1][0], reverse=True)
        print(f"{'Function':<40} {'Lines':>6} {'Start':>6}")
        print("-" * 55)
        for name, (start, end) in sorted_funcs:
            print(f"{name:<40} {end-start:>6} {start+1:>6}")
        return

    if len(sys.argv) < 2:
        print("Usage: python3 scripts/extract-c-function.py FUNCTION_NAME [--with-structs] [--prompt]")
        sys.exit(1)

    func_name = sys.argv[1]
    with_structs = '--with-structs' in sys.argv
    as_prompt = '--prompt' in sys.argv

    functions = find_functions(lines)

    if func_name not in functions:
        print(f"Function '{func_name}' not found. Available functions:")
        for name in sorted(functions.keys()):
            print(f"  {name}")
        sys.exit(1)

    start, end = functions[func_name]
    func_text = ''.join(lines[start:end])
    line_count = end - start

    if as_prompt:
        # Generate a full agent prompt
        print(f"# Port C function: {func_name}")
        print(f"# Source: expat/lib/xmlparse.c lines {start+1}-{end}")
        print(f"# Size: {line_count} lines")
        print()
        print("## C Source")
        print("```c")
        print(func_text)
        print("```")
        print()
        if with_structs:
            print("## Related Struct Definitions")
            print("```c")
            for s_start, s_end in extract_structs(lines):
                print(''.join(lines[s_start:s_end]))
                print()
            print("```")
    else:
        print(f"// {func_name} — {line_count} lines (xmlparse.c:{start+1}-{end})")
        print(func_text)

if __name__ == '__main__':
    main()
