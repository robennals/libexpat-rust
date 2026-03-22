#!/usr/bin/env python3
"""Generate systematic comparison test cases for C vs Rust behavioral equivalence.

Generates Rust test code that runs XML inputs through both C (via expat-sys) and Rust
parsers, comparing status codes, error codes, and optionally handler output.

Usage:
    python3 scripts/generate-comparison-tests.py > expat-rust/tests/generated_comparison_tests.rs
"""

import sys

# Each test case: (name, xml_bytes_expr, description)
# xml_bytes_expr is a Rust expression that produces &[u8]
TEST_CASES = []

def add(name, xml, desc):
    """Add a test case with byte string literal."""
    TEST_CASES.append((name, xml, desc))

# ======================== Well-formed documents ========================
add("gen_empty_root", 'b"<r/>"', "minimal self-closing element")
add("gen_root_with_text", 'b"<r>text</r>"', "text content")
add("gen_root_with_whitespace", 'b"<r> \\t\\n </r>"', "whitespace content")
add("gen_nested_3_deep", 'b"<a><b><c/></b></a>"', "3 levels deep")
add("gen_multiple_children", 'b"<r><a/><b/><c/></r>"', "multiple children")
add("gen_attrs_single_quotes", "b\"<r a='1'/>\"", "single-quoted attributes")
add("gen_attrs_double_quotes", 'b"<r a=\\"1\\"/>"', "double-quoted attributes")
add("gen_multiple_attrs", 'b"<r a=\\"1\\" b=\\"2\\" c=\\"3\\"/>"', "multiple attributes")
add("gen_attr_with_entities", 'b"<r a=\\"&amp;&lt;&gt;\\"/>"', "entities in attributes")
add("gen_empty_attr", 'b"<r a=\\"\\"/>"', "empty attribute value")

# ======================== XML declarations ========================
add("gen_xmldecl_10", "b\"<?xml version='1.0'?><r/>\"", "xml decl version 1.0")
add("gen_xmldecl_utf8", "b\"<?xml version='1.0' encoding='utf-8'?><r/>\"", "xml decl with encoding")
add("gen_xmldecl_standalone_yes", "b\"<?xml version='1.0' standalone='yes'?><r/>\"", "standalone yes")
add("gen_xmldecl_standalone_no", "b\"<?xml version='1.0' standalone='no'?><r/>\"", "standalone no")
add("gen_xmldecl_all", "b\"<?xml version='1.0' encoding='utf-8' standalone='yes'?><r/>\"", "all xml decl attrs")

# ======================== Comments ========================
add("gen_comment_before", 'b"<!-- comment --><r/>"', "comment before root")
add("gen_comment_after", 'b"<r/><!-- comment -->"', "comment after root")
add("gen_comment_inside", 'b"<r><!-- inside --></r>"', "comment inside element")
add("gen_comment_empty", 'b"<r><!----></r>"', "empty comment")
add("gen_comment_with_dashes", 'b"<r><!-- a-b --></r>"', "comment with dashes")

# ======================== Processing instructions ========================
add("gen_pi_before", 'b"<?target data?><r/>"', "PI before root")
add("gen_pi_after", 'b"<r/><?target data?>"', "PI after root")
add("gen_pi_inside", 'b"<r><?target data?></r>"', "PI inside element")
add("gen_pi_no_data", 'b"<r><?target?></r>"', "PI without data")

# ======================== CDATA ========================
add("gen_cdata_simple", 'b"<r><![CDATA[hello]]></r>"', "simple CDATA")
add("gen_cdata_with_lt", 'b"<r><![CDATA[<not-a-tag>]]></r>"', "CDATA with <")
add("gen_cdata_with_amp", 'b"<r><![CDATA[a&b]]></r>"', "CDATA with &")
add("gen_cdata_empty", 'b"<r><![CDATA[]]></r>"', "empty CDATA")
add("gen_cdata_with_brackets", 'b"<r><![CDATA[a]b]]></r>"', "CDATA with ]")

# ======================== Entity references ========================
add("gen_entity_amp", 'b"<r>&amp;</r>"', "amp entity")
add("gen_entity_lt", 'b"<r>&lt;</r>"', "lt entity")
add("gen_entity_gt", 'b"<r>&gt;</r>"', "gt entity")
add("gen_entity_quot", 'b"<r>&quot;</r>"', "quot entity")
add("gen_entity_apos", 'b"<r>&apos;</r>"', "apos entity")
add("gen_charref_decimal", 'b"<r>&#65;</r>"', "decimal char ref A")
add("gen_charref_hex", 'b"<r>&#x41;</r>"', "hex char ref A")
add("gen_charref_multibyte", 'b"<r>&#x00e9;</r>"', "char ref for é")
add("gen_multiple_entities", 'b"<r>&amp;&lt;&gt;</r>"', "multiple entities")

# ======================== DOCTYPE ========================
add("gen_doctype_simple", 'b"<!DOCTYPE r><r/>"', "simple DOCTYPE")
add("gen_doctype_system", 'b"<!DOCTYPE r SYSTEM \\"test.dtd\\"><r/>"', "DOCTYPE with SYSTEM")
add("gen_doctype_public", 'b"<!DOCTYPE r PUBLIC \\"-//Test\\" \\"test.dtd\\"><r/>"', "DOCTYPE with PUBLIC")
add("gen_doctype_internal_empty", 'b"<!DOCTYPE r []><r/>"', "DOCTYPE with empty internal subset")
add("gen_doctype_entity_decl", 'b"<!DOCTYPE r [<!ENTITY e \\"val\\">]><r/>"', "DOCTYPE with entity")
add("gen_doctype_attlist", 'b"<!DOCTYPE r [<!ATTLIST r a CDATA #IMPLIED>]><r/>"', "DOCTYPE with ATTLIST")
add("gen_doctype_element_any", 'b"<!DOCTYPE r [<!ELEMENT r ANY>]><r/>"', "DOCTYPE with ELEMENT ANY")
add("gen_doctype_element_empty", 'b"<!DOCTYPE r [<!ELEMENT r EMPTY>]><r/>"', "DOCTYPE with ELEMENT EMPTY")
add("gen_doctype_notation", 'b"<!DOCTYPE r [<!NOTATION n SYSTEM \\"test\\">]><r/>"', "DOCTYPE with NOTATION")
add("gen_doctype_comment", 'b"<!DOCTYPE r [<!-- comment -->]><r/>"', "DOCTYPE with comment")
add("gen_doctype_pi", 'b"<!DOCTYPE r [<?pi data?>]><r/>"', "DOCTYPE with PI")

# ======================== Error cases ========================
add("gen_err_empty_final", 'b""', "empty input with is_final")
add("gen_err_no_root", 'b"   "', "whitespace only")
add("gen_err_unclosed_tag", 'b"<r>"', "unclosed tag")
add("gen_err_mismatched_tags", 'b"<a></b>"', "mismatched tags")
add("gen_err_double_root", 'b"<r/><r/>"', "two root elements")
add("gen_err_text_after_root", 'b"<r/>text"', "text after root")
add("gen_err_invalid_char", 'b"<r>\\x00</r>"', "null byte in content")
add("gen_err_bad_entity", 'b"<r>&nosuch;</r>"', "undefined entity")
add("gen_err_bad_charref", 'b"<r>&#xFFFFFF;</r>"', "invalid char ref")
add("gen_err_unclosed_entity", 'b"<r>&amp</r>"', "unclosed entity ref")
add("gen_err_duplicate_attr", 'b"<r a=\\"1\\" a=\\"2\\"/>"', "duplicate attribute")
add("gen_err_junk_in_prolog", 'b"xxx<r/>"', "junk before root")
add("gen_err_misplaced_xmldecl", "b\"<r/><?xml version='1.0'?>\"", "xml decl after root")
add("gen_err_trailing_cr", 'b"<r>\\r"', "trailing CR in content")
add("gen_err_partial_tag", 'b"<r"', "partial opening tag")

# ======================== Whitespace handling ========================
add("gen_ws_before_root", 'b"  \\n  <r/>"', "whitespace before root")
add("gen_ws_after_root", 'b"<r/>  \\n  "', "whitespace after root")
add("gen_ws_crlf", 'b"<r>a\\r\\nb</r>"', "CRLF in content")
add("gen_ws_cr_only", 'b"<r>a\\rb</r>"', "bare CR in content")
add("gen_ws_in_attr", 'b"<r a = \\"1\\" />"', "whitespace around = in attrs")

# ======================== Namespace tests ========================
add("gen_ns_default", 'b"<r xmlns=\\"http://example.com\\"/>"', "default namespace")
add("gen_ns_prefix", 'b"<x:r xmlns:x=\\"http://example.com\\"/>"', "prefixed namespace")
add("gen_ns_nested_override", 'b"<r xmlns=\\"http://a\\"><c xmlns=\\"http://b\\"/></r>"', "nested namespace override")
add("gen_ns_unbound_prefix_err", 'b"<x:r/>"', "unbound prefix error")
add("gen_ns_on_attribute", 'b"<r xmlns:x=\\"http://example.com\\" x:a=\\"1\\"/>"', "namespace on attribute")

# ======================== Incremental parsing (two chunks) ========================
# These test split parsing - would need special handling

# ======================== Multibyte UTF-8 ========================
add("gen_utf8_2byte", 'b"<r>\\xc3\\xa9</r>"', "2-byte UTF-8 (é)")
add("gen_utf8_3byte", 'b"<r>\\xe4\\xb8\\xad</r>"', "3-byte UTF-8 (中)")
add("gen_utf8_4byte", 'b"<r>\\xf0\\x9f\\x98\\x80</r>"', "4-byte UTF-8 (😀)")
add("gen_utf8_in_attr", 'b"<r a=\\"\\xc3\\xa9\\"/>"', "UTF-8 in attribute")
add("gen_utf8_in_name", 'b"<r\\xc3\\xa9/>"', "UTF-8 in element name")


def main():
    print('//! Auto-generated comparison tests for C vs Rust behavioral equivalence.')
    print('//! Generated by scripts/generate-comparison-tests.py')
    print('//! ')
    print('//! DO NOT EDIT — regenerate with: python3 scripts/generate-comparison-tests.py')
    print()
    print('use expat_rust::xmlparse::Parser;')
    print('use expat_sys::CParser;')
    print()
    print('fn parse_rust(xml: &[u8]) -> (u32, u32) {')
    print('    let mut parser = Parser::new(None).unwrap();')
    print('    let status = parser.parse(xml, true);')
    print('    let error = parser.error_code();')
    print('    (status as u32, error as u32)')
    print('}')
    print()
    print('fn parse_c(xml: &[u8]) -> (u32, u32) {')
    print('    let parser = CParser::new(None).unwrap();')
    print('    let (status, error) = parser.parse(xml, true);')
    print('    (status, error)')
    print('}')
    print()
    print('fn compare(xml: &[u8], desc: &str) {')
    print('    let (rs, re) = parse_rust(xml);')
    print('    let (cs, ce) = parse_c(xml);')
    print('    assert!(rs == cs && re == ce,')
    print('        "MISMATCH {desc}: Rust status={rs} err={re}, C status={cs} err={ce}, input={:?}",')
    print('        std::str::from_utf8(xml).unwrap_or("<binary>"));')
    print('}')
    print()

    # NS comparison uses regular parser for now (no CParser::new_ns yet)
    # TODO: Add CParser::new_ns to expat-sys, then enable proper NS comparison
    print('fn compare_ns(xml: &[u8], desc: &str) {')
    print('    // NS tests use non-NS parser for now — still catches basic parse errors')
    print('    compare(xml, desc);')
    print('}')
    print()

    for name, xml_expr, desc in TEST_CASES:
        is_ns = name.startswith("gen_ns_")
        fn_name = "compare_ns" if is_ns else "compare"
        print(f'#[test]')
        print(f'fn {name}() {{')
        print(f'    {fn_name}({xml_expr}, "{desc}");')
        print(f'}}')
        print()

    print(f'// Total: {len(TEST_CASES)} generated comparison tests')


if __name__ == '__main__':
    main()
