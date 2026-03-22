#!/usr/bin/env python3
"""
Script to port C character classification tables to Rust.
Reads C header files with BT_* constants and generates a Rust module.
"""

import re
import sys
from pathlib import Path


def parse_c_table(file_path):
    """Parse a C table file and extract BT_* values in order."""
    with open(file_path, 'r') as f:
        content = f.read()

    # Remove the copyright/license header (everything before the first /* 0x... */ comment)
    # Find the first line that starts with /* 0x
    lines = content.split('\n')
    table_start = 0
    for i, line in enumerate(lines):
        if '/* 0x' in line:
            table_start = i
            break

    # Reconstruct just the table data
    table_content = '\n'.join(lines[table_start:])

    # Extract all BT_* identifiers from the table content
    bt_pattern = r'BT_\w+'
    matches = re.findall(bt_pattern, table_content)

    if not matches:
        raise ValueError(f"No BT_* constants found in {file_path}")

    return matches


def extract_enum_variants(table_data):
    """Extract all unique enum variants from all table data."""
    all_variants = set()
    for table_values in table_data.values():
        all_variants.update(table_values)

    # Convert BT_* names to Rust enum variant names
    # Keep order as specified in the requirements
    required_order = [
        'NONXML', 'S', 'LF', 'CR', 'EXCL', 'QUOT', 'NUM', 'OTHER', 'PERCNT',
        'AMP', 'APOS', 'LPAR', 'RPAR', 'AST', 'PLUS', 'COMMA', 'MINUS', 'NAME',
        'SOL', 'DIGIT', 'COLON', 'SEMI', 'LT', 'EQUALS', 'GT', 'QUEST', 'HEX',
        'NMSTRT', 'LSQB', 'RSQB', 'VERBAR', 'TRAIL', 'LEAD2', 'LEAD3', 'LEAD4',
        'MALFORM'
    ]

    return required_order


def generate_rust_file(table_data, output_path):
    """Generate the Rust char_tables.rs file."""
    # Extract unique enum variants
    variants = extract_enum_variants(table_data)

    # Build the enum definition
    enum_variants = ',\n    '.join(variants)

    # Convert BT_* values to Rust enum variants
    def bt_to_variant(bt_name):
        # bt_name is like "BT_NONXML", we want "NONXML"
        return bt_name.replace('BT_', '')

    # Build array initializers
    ascii_array = ', '.join(
        f'ByteType::{bt_to_variant(val)}'
        for val in table_data['ascii']
    )

    iascii_array = ', '.join(
        f'ByteType::{bt_to_variant(val)}'
        for val in table_data['iascii']
    )

    utf8_array = ', '.join(
        f'ByteType::{bt_to_variant(val)}'
        for val in table_data['utf8']
    )

    latin1_array = ', '.join(
        f'ByteType::{bt_to_variant(val)}'
        for val in table_data['latin1']
    )

    # Generate the complete Rust file
    rust_code = f'''// AUTO-GENERATED: Do not edit manually
// Generated from C header files: asciitab.h, iasciitab.h, utf8tab.h, latin1tab.h

/// Character classification byte type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ByteType {{
    {enum_variants},
}}

/// ASCII character type table (0x00-0x7F)
pub const ASCII_BYTE_TYPES: [ByteType; 128] = [
    {ascii_array},
];

/// Internal ASCII character type table (0x00-0x7F)
/// Like asciitab.h, except that 0xD has code BT_S rather than BT_CR
pub const IASCII_BYTE_TYPES: [ByteType; 128] = [
    {iascii_array},
];

/// UTF-8 high byte table (0x80-0xFF)
pub const UTF8_BYTE_TYPES: [ByteType; 128] = [
    {utf8_array},
];

/// Latin-1 high byte table (0x80-0xFF)
pub const LATIN1_BYTE_TYPES: [ByteType; 128] = [
    {latin1_array},
];
'''

    with open(output_path, 'w') as f:
        f.write(rust_code)

    print(f"Generated {output_path}")


def main():
    # Paths
    expat_lib_dir = Path("/Users/robennals/broomer-repos/libexpat-rust/port/start/expat/lib")
    output_file = Path("/Users/robennals/broomer-repos/libexpat-rust/port/start/expat-rust/src/char_tables.rs")

    # Parse all C tables
    print("Parsing C header files...")

    ascii_values = parse_c_table(expat_lib_dir / "asciitab.h")
    iascii_values = parse_c_table(expat_lib_dir / "iasciitab.h")
    utf8_values = parse_c_table(expat_lib_dir / "utf8tab.h")
    latin1_values = parse_c_table(expat_lib_dir / "latin1tab.h")

    print(f"  asciitab.h: {len(ascii_values)} values")
    print(f"  iasciitab.h: {len(iascii_values)} values")
    print(f"  utf8tab.h: {len(utf8_values)} values")
    print(f"  latin1tab.h: {len(latin1_values)} values")

    # Verify we have the right number of entries
    if len(ascii_values) != 128:
        raise ValueError(f"Expected 128 ASCII values, got {len(ascii_values)}")
    if len(iascii_values) != 128:
        raise ValueError(f"Expected 128 IASCII values, got {len(iascii_values)}")
    if len(utf8_values) != 128:
        raise ValueError(f"Expected 128 UTF8 values, got {len(utf8_values)}")
    if len(latin1_values) != 128:
        raise ValueError(f"Expected 128 Latin1 values, got {len(latin1_values)}")

    table_data = {
        'ascii': ascii_values,
        'iascii': iascii_values,
        'utf8': utf8_values,
        'latin1': latin1_values,
    }

    # Generate Rust file
    print("Generating Rust file...")
    generate_rust_file(table_data, output_file)
    print("Done!")


if __name__ == '__main__':
    main()
