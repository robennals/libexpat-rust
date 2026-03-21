#!/usr/bin/env python3
"""
Analyze C2Rust output to identify transformation patterns and create
a comprehensive report for guiding idiomatic Rust conversion.

Patterns analyzed:
1. Raw pointer usage patterns (what could be &, &mut, &[], Box, Vec)
2. Global/static mutable state
3. Function pointer patterns (-> trait objects or enums)
4. String handling (C strings -> Rust strings)
5. Error handling (return codes -> Result)
6. Memory management (malloc/free -> Vec, Box)
7. Integer as boolean patterns
8. Struct access through raw pointers (-> &self methods)
"""

import re
import sys
import json
from collections import Counter


def analyze_file(filepath: str) -> dict:
    with open(filepath) as f:
        content = f.read()
        lines = content.split('\n')

    analysis = {
        'file': filepath,
        'total_lines': len(lines),
        'patterns': {},
    }

    # Pattern 1: Raw pointer dereferences (*ptr).field
    ptr_deref = re.findall(r'\(\*(\w+)\)\.(\w+)', content)
    field_access = Counter()
    ptr_vars = Counter()
    for var, field in ptr_deref:
        field_access[field] += 1
        ptr_vars[var] += 1
    analysis['patterns']['ptr_deref'] = {
        'count': len(ptr_deref),
        'top_fields': field_access.most_common(20),
        'top_vars': ptr_vars.most_common(20),
    }

    # Pattern 2: malloc/free calls
    malloc_calls = re.findall(r'malloc\(([^)]+)\)', content)
    free_calls = re.findall(r'free\(([^)]+)\)', content)
    analysis['patterns']['memory'] = {
        'malloc_count': len(malloc_calls),
        'free_count': len(free_calls),
        'malloc_sizes': malloc_calls[:10],
    }

    # Pattern 3: C string patterns
    c_strings = re.findall(r'b"[^"]*\\0"\s*as\s*\*const\s*u8\s*as\s*\*const\s*i8', content)
    analysis['patterns']['c_strings'] = {
        'count': len(c_strings),
    }

    # Pattern 4: Function pointer calls via .expect("non-null")
    fn_ptr_calls = re.findall(r'\.expect\("non-null function pointer"\)\(', content)
    analysis['patterns']['fn_ptr_calls'] = {
        'count': len(fn_ptr_calls),
    }

    # Pattern 5: Integer-as-boolean patterns
    int_bool = re.findall(r'!= 0i32\b|== 0i32\b|!= 0\b.*as.*bool|as XML_Bool', content)
    analysis['patterns']['int_as_bool'] = {
        'count': len(int_bool),
    }

    # Pattern 6: unsafe blocks/functions
    unsafe_fns = re.findall(r'unsafe\s+(?:extern\s+"C"\s+)?fn\s+(\w+)', content)
    analysis['patterns']['unsafe'] = {
        'unsafe_fn_count': len(unsafe_fns),
        'functions': unsafe_fns,
    }

    # Pattern 7: current_block control flow patterns
    current_block = re.findall(r'current_block(?:_\d+)?', content)
    analysis['patterns']['current_block_gotos'] = {
        'count': len(current_block),
        'description': 'C goto/label patterns converted to current_block state variables',
    }

    # Pattern 8: Type casts
    type_casts = Counter()
    for m in re.finditer(r'as\s+([\w:]+)', content):
        type_casts[m.group(1)] += 1
    analysis['patterns']['type_casts'] = {
        'top_casts': type_casts.most_common(20),
    }

    # Pattern 9: Struct definitions
    structs = re.findall(r'pub struct (\w+)', content)
    analysis['patterns']['structs'] = structs

    # Pattern 10: Static/const globals
    statics = re.findall(r'(?:pub\s+)?static\s+(?:mut\s+)?(\w+)', content)
    consts = re.findall(r'(?:pub\s+)?const\s+(\w+)', content)
    analysis['patterns']['globals'] = {
        'statics': statics,
        'consts': consts[:20],
    }

    return analysis


def print_report(analysis: dict):
    print(f"=== C2Rust Pattern Analysis: {analysis['file']} ===")
    print(f"Total lines: {analysis['total_lines']}")
    print()

    p = analysis['patterns']

    print("--- Raw Pointer Dereferences ---")
    print(f"Total: {p['ptr_deref']['count']}")
    print("Top accessed fields:")
    for field, count in p['ptr_deref']['top_fields']:
        print(f"  {field}: {count}")
    print("Top dereferenced variables:")
    for var, count in p['ptr_deref']['top_vars']:
        print(f"  {var}: {count}")
    print()

    print("--- Memory Management ---")
    print(f"malloc calls: {p['memory']['malloc_count']}")
    print(f"free calls: {p['memory']['free_count']}")
    print()

    print("--- C String Literals ---")
    print(f"Count: {p['c_strings']['count']}")
    print()

    print("--- Function Pointer Calls ---")
    print(f"Count: {p['fn_ptr_calls']['count']}")
    print()

    print("--- Integer-as-Boolean Patterns ---")
    print(f"Count: {p['int_as_bool']['count']}")
    print()

    print("--- Control Flow (goto→current_block) ---")
    print(f"Count: {p['current_block_gotos']['count']}")
    print()

    print("--- Type Casts ---")
    for cast_type, count in p['type_casts']['top_casts']:
        print(f"  as {cast_type}: {count}")
    print()

    print("--- Structs ---")
    for s in p['structs']:
        print(f"  {s}")
    print()

    print("--- Static Mutable Globals ---")
    for s in p['globals']['statics']:
        print(f"  {s}")


def main():
    parser = __import__('argparse').ArgumentParser()
    parser.add_argument('input', help='C2Rust output .rs file')
    parser.add_argument('--json', action='store_true')
    args = parser.parse_args()

    analysis = analyze_file(args.input)

    if args.json:
        # Convert Counter objects for JSON
        def convert(obj):
            if isinstance(obj, Counter):
                return dict(obj)
            raise TypeError(f"Not serializable: {type(obj)}")
        print(json.dumps(analysis, indent=2, default=convert))
    else:
        print_report(analysis)


if __name__ == '__main__':
    main()
