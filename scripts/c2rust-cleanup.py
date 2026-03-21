#!/usr/bin/env python3
"""
Phase 1: Mechanical cleanup of C2Rust output.

This script performs safe, mechanical transformations on C2Rust-generated Rust code.
These are syntactic transforms that don't change semantics:

1. Type alias cleanup (::core::ffi::c_int -> i32, etc.)
2. Verbose literal cleanup (0 as ::core::ffi::c_int -> 0i32, etc.)
3. Remove unnecessary casts
4. Clean up null pointer patterns
5. Remove macOS-specific platform types
6. Simplify boolean expressions
"""

import re
import sys
import argparse


def cleanup_type_aliases(content: str) -> str:
    """Replace verbose C FFI type paths with Rust primitives."""
    replacements = [
        # Core FFI types -> Rust primitives
        (r'::core::ffi::c_int', 'i32'),
        (r'::core::ffi::c_uint', 'u32'),
        (r'::core::ffi::c_long', 'i64'),  # On 64-bit
        (r'::core::ffi::c_ulong', 'u64'),  # On 64-bit
        (r'::core::ffi::c_short', 'i16'),
        (r'::core::ffi::c_ushort', 'u16'),
        (r'::core::ffi::c_char', 'i8'),
        (r'::core::ffi::c_uchar', 'u8'),
        (r'::core::ffi::c_float', 'f32'),
        (r'::core::ffi::c_double', 'f64'),
        (r'::core::ffi::c_void', 'libc::c_void'),
    ]
    for old, new in replacements:
        content = content.replace(old, new)
    return content


def cleanup_darwin_types(content: str) -> str:
    """Remove macOS-specific type aliases that are just indirections."""
    # Remove __darwin type aliases
    lines = content.split('\n')
    filtered = []
    skip_types = {
        '__darwin_ptrdiff_t', '__darwin_size_t', '__darwin_off_t',
        '__int64_t', 'ptrdiff_t'  # Already isize in Rust
    }
    for line in lines:
        # Remove type alias lines for platform types
        m = re.match(r'pub type (\w+) = .*;', line)
        if m and m.group(1) in skip_types:
            continue
        # Replace references to these types
        filtered.append(line)

    content = '\n'.join(filtered)

    # Replace remaining references
    content = content.replace('__darwin_ptrdiff_t', 'isize')
    content = content.replace('__darwin_size_t', 'usize')
    content = content.replace('__darwin_off_t', 'i64')
    content = content.replace('__int64_t', 'i64')

    # size_t -> usize (if aliased to __darwin_size_t)
    # Remove: pub type size_t = usize; (redundant)
    content = re.sub(r'pub type size_t = usize;\n', '', content)
    content = re.sub(r'\bsize_t\b', 'usize', content)

    return content


def cleanup_literal_casts(content: str) -> str:
    """Simplify verbose literal casts."""
    # 0 as i32 -> 0i32, etc.
    content = re.sub(r'(\d+)\s+as\s+i32\b', r'\1i32', content)
    content = re.sub(r'(\d+)\s+as\s+u32\b', r'\1u32', content)
    content = re.sub(r'(\d+)\s+as\s+i64\b', r'\1i64', content)
    content = re.sub(r'(\d+)\s+as\s+u64\b', r'\1u64', content)
    content = re.sub(r'(\d+)\s+as\s+usize\b', r'\1usize', content)
    content = re.sub(r'(\d+)\s+as\s+isize\b', r'\1isize', content)
    content = re.sub(r'(\d+)\s+as\s+u8\b', r'\1u8', content)
    content = re.sub(r'(\d+)\s+as\s+i8\b', r'\1i8', content)

    # Remove double casts like `0i32 as i32`
    content = re.sub(r'(\d+)(i32|u32|i64|u64|usize|isize|u8|i8)\s+as\s+\2\b', r'\1\2', content)

    # 0i32 -> 0 in most contexts (when type is obvious)
    # Be conservative - only do 0i32 and 1i32 as they're most common
    # Actually, let's keep typed literals for safety

    return content


def cleanup_null_patterns(content: str) -> str:
    """Simplify null pointer patterns."""
    # ::core::ptr::null::<TYPE>() -> std::ptr::null::<TYPE>()
    content = content.replace('::core::ptr::null::', 'std::ptr::null::')
    content = content.replace('::core::ptr::null_mut::', 'std::ptr::null_mut::')

    # Simplify overly-typed null:
    # std::ptr::null::<i8>() -> std::ptr::null()  (when type is inferrable)
    # (Keep this for now - removal requires type inference context)

    return content


def cleanup_bool_patterns(content: str) -> str:
    """Simplify boolean patterns."""
    # `!= 0` on booleans is redundant (but be careful - C uses int as bool)
    # `foo as i32 != 0` -> `foo` (when foo is bool)
    # This is hard to do safely without type info, so skip for now

    # `.is_null() as i32 as i64 != 0` -> `.is_null()`
    content = re.sub(
        r'\.is_null\(\)\s+as\s+i32\s+as\s+i64\s*!=\s*0',
        '.is_null()',
        content
    )

    # `(.foo as i32 != 0)` -> `(.foo != 0)` when foo is XML_Bool (unsigned char)
    # Be conservative

    return content


def cleanup_feature_gates(content: str) -> str:
    """Remove feature gates that aren't needed for stable Rust transforms."""
    # We'll keep these for now since the C2Rust output genuinely needs them
    return content


def remove_stdio_types(content: str) -> str:
    """Remove macOS stdio struct definitions that aren't needed."""
    # Remove __sbuf, __sFILE, FILE type definitions
    # These are platform-specific C types
    lines = content.split('\n')
    result = []
    skip_until_brace_close = False
    brace_depth = 0

    i = 0
    while i < len(lines):
        line = lines[i]
        # Detect start of structs we want to remove
        if (re.match(r'pub struct __sbuf\b', line.strip()) or
            re.match(r'pub struct __sFILE\b', line.strip()) or
            re.match(r'#\[derive.*\]\s*$', line.strip()) and
            i + 2 < len(lines) and
            re.match(r'pub struct __s(buf|FILE)\b', lines[i+2].strip())):
            # Skip this struct definition
            skip_until_brace_close = True
            brace_depth = 0

        if skip_until_brace_close:
            brace_depth += line.count('{') - line.count('}')
            if brace_depth <= 0 and '{' in line or '}' in line:
                # Check if struct is fully closed
                if brace_depth <= 0:
                    skip_until_brace_close = False
            i += 1
            continue

        result.append(line)
        i += 1

    return '\n'.join(result)


def cleanup_mut_params(content: str) -> str:
    """Remove unnecessary 'mut' on function parameters (C2Rust adds mut to all params)."""
    # In Rust, function parameters are already owned, so mut is only needed
    # if you actually mutate the value. But for correctness, keep mut on pointers.
    # Just remove mut on scalar/primitive params.
    # This is hard to do safely without full analysis, so be conservative.
    return content


def run_all_transforms(content: str) -> str:
    """Apply all mechanical transforms in order."""
    content = cleanup_type_aliases(content)
    content = cleanup_darwin_types(content)
    content = cleanup_literal_casts(content)
    content = cleanup_null_patterns(content)
    content = cleanup_bool_patterns(content)
    return content


def main():
    parser = argparse.ArgumentParser(description='Clean up C2Rust output')
    parser.add_argument('input', help='Input .rs file')
    parser.add_argument('-o', '--output', help='Output file (default: stdout)')
    parser.add_argument('--in-place', '-i', action='store_true',
                        help='Edit file in place')
    args = parser.parse_args()

    with open(args.input) as f:
        content = f.read()

    result = run_all_transforms(content)

    if args.in_place:
        with open(args.input, 'w') as f:
            f.write(result)
        print(f"Cleaned up {args.input} in place", file=sys.stderr)
    elif args.output:
        with open(args.output, 'w') as f:
            f.write(result)
        print(f"Wrote cleaned output to {args.output}", file=sys.stderr)
    else:
        print(result)


if __name__ == '__main__':
    main()
