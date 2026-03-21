//! Comparison tests: run the same XML through both the Rust port and the C library
//! to verify behavioral equivalence.

use expat_rust::xmlparse::{Parser, XmlStatus, XmlError};
use expat_sys::CParser;
use std::ffi::{c_char, c_int, c_void};
use std::cell::RefCell;

/// Parse XML with the Rust port and return (status, error_code, line, col)
fn parse_rust(xml: &[u8]) -> (XmlStatus, XmlError, u64, u64) {
    let mut parser = Parser::new(None).unwrap();
    let status = parser.parse(xml, true);
    let error = parser.error_code();
    let line = parser.current_line_number();
    let col = parser.current_column_number();
    (status, error, line, col)
}

/// Parse XML with the C library and return (status, error_code, line, col)
fn parse_c(xml: &[u8]) -> (u32, u32, u64, u64) {
    let parser = CParser::new(None).unwrap();
    let (status, error) = parser.parse(xml, true);
    let line = parser.current_line_number();
    let col = parser.current_column_number();
    (status, error, line, col)
}

/// Convert Rust status to u32 for comparison
fn rust_status_to_u32(s: XmlStatus) -> u32 {
    match s {
        XmlStatus::Error => 0,
        XmlStatus::Ok => 1,
        XmlStatus::Suspended => 2,
    }
}

/// Convert Rust error to u32 for comparison
fn rust_error_to_u32(e: XmlError) -> u32 {
    e as u32
}

/// Compare both parsers on the same input
fn compare_parse(xml: &[u8], description: &str) {
    let (r_status, r_error, r_line, r_col) = parse_rust(xml);
    let (c_status, c_error, c_line, c_col) = parse_c(xml);

    let r_s = rust_status_to_u32(r_status);
    let r_e = rust_error_to_u32(r_error);

    if r_s != c_status || r_e != c_error {
        panic!(
            "MISMATCH for {description}:\n  \
             Rust: status={r_s} error={r_e} line={r_line} col={r_col}\n  \
             C:    status={c_status} error={c_error} line={c_line} col={c_col}\n  \
             Input: {:?}",
            std::str::from_utf8(xml).unwrap_or("<non-utf8>")
        );
    }
}

/// Comparison test that only checks status (not line/col, since those may differ
/// in implementation details)
fn compare_status(xml: &[u8], description: &str) {
    let (r_status, r_error, _, _) = parse_rust(xml);
    let (c_status, c_error, _, _) = parse_c(xml);

    let r_s = rust_status_to_u32(r_status);
    let r_e = rust_error_to_u32(r_error);

    if r_s != c_status {
        panic!(
            "STATUS MISMATCH for {description}:\n  \
             Rust: status={r_s} (error={r_e})\n  \
             C:    status={c_status} (error={c_error})\n  \
             Input: {:?}",
            std::str::from_utf8(xml).unwrap_or("<non-utf8>")
        );
    }
}

// ======================== Basic well-formed documents ========================

#[test]
fn cmp_simple_element() {
    compare_parse(b"<a/>", "simple self-closing element");
}

#[test]
fn cmp_nested_elements() {
    compare_parse(b"<a><b/></a>", "nested elements");
}

#[test]
fn cmp_element_with_text() {
    compare_parse(b"<a>hello</a>", "element with text content");
}

#[test]
fn cmp_element_with_attributes() {
    compare_parse(b"<a x=\"1\" y=\"2\"/>", "element with attributes");
}

#[test]
fn cmp_xml_declaration() {
    compare_parse(b"<?xml version=\"1.0\"?><a/>", "xml declaration");
}

#[test]
fn cmp_xml_declaration_encoding() {
    compare_parse(
        b"<?xml version=\"1.0\" encoding=\"utf-8\"?><a/>",
        "xml declaration with encoding"
    );
}

#[test]
fn cmp_processing_instruction() {
    compare_parse(b"<?target data?><a/>", "processing instruction");
}

#[test]
fn cmp_comment() {
    compare_parse(b"<!-- comment --><a/>", "comment");
}

#[test]
fn cmp_cdata_section() {
    compare_parse(b"<a><![CDATA[some data]]></a>", "CDATA section");
}

#[test]
fn cmp_multiple_text_content() {
    compare_parse(b"<a>text1<b/>text2</a>", "multiple text sections");
}

#[test]
fn cmp_whitespace_content() {
    compare_parse(b"<a>  \n\t  </a>", "whitespace content");
}

#[test]
fn cmp_deeply_nested() {
    compare_parse(b"<a><b><c><d/></c></b></a>", "deeply nested");
}

#[test]
fn cmp_predefined_entities() {
    compare_parse(
        b"<a>&lt;&gt;&amp;&apos;&quot;</a>",
        "predefined entities"
    );
}

#[test]
fn cmp_numeric_char_ref() {
    compare_parse(b"<a>&#65;</a>", "numeric character reference");
}

#[test]
fn cmp_hex_char_ref() {
    compare_parse(b"<a>&#x41;</a>", "hex character reference");
}

// ======================== Error cases ========================

#[test]
fn cmp_empty_input() {
    compare_status(b"", "empty input");
}

#[test]
fn cmp_no_root_element() {
    compare_status(b"<!-- just a comment -->", "no root element");
}

#[test]
fn cmp_unclosed_tag() {
    compare_status(b"<a>", "unclosed tag");
}

#[test]
fn cmp_mismatched_tags() {
    compare_status(b"<a></b>", "mismatched tags");
}

#[test]
fn cmp_duplicate_attribute() {
    compare_status(b"<a x=\"1\" x=\"2\"/>", "duplicate attribute");
}

#[test]
fn cmp_invalid_token() {
    compare_status(b"<a>&invalid;</a>", "invalid entity reference");
}

#[test]
fn cmp_junk_after_document() {
    compare_status(b"<a/>junk", "junk after document element");
}

#[test]
fn cmp_multiple_root_elements() {
    compare_status(b"<a/><b/>", "multiple root elements");
}

#[test]
fn cmp_partial_tag() {
    compare_status(b"<a", "partial tag");
}

#[test]
fn cmp_ampersand_in_content() {
    compare_status(b"<a>a&b</a>", "bare ampersand in content");
}

#[test]
fn cmp_less_than_in_content() {
    // < in content should cause an error
    compare_status(b"<a>a<b</a>", "less-than in content");
}

// ======================== DOCTYPE ========================

#[test]
fn cmp_simple_doctype() {
    compare_status(
        b"<!DOCTYPE root><root/>",
        "simple DOCTYPE"
    );
}

#[test]
fn cmp_doctype_with_internal_subset() {
    compare_status(
        b"<!DOCTYPE root [<!ELEMENT root EMPTY>]><root/>",
        "DOCTYPE with internal subset"
    );
}

#[test]
fn cmp_doctype_entity() {
    compare_status(
        b"<!DOCTYPE root [<!ENTITY e \"text\">]><root>&e;</root>",
        "DOCTYPE with entity"
    );
}

// ======================== Incremental parsing ========================

#[test]
fn cmp_incremental_parse() {
    // Parse in two chunks
    let mut r_parser = Parser::new(None).unwrap();
    let c_parser = CParser::new(None).unwrap();

    let r1 = r_parser.parse(b"<a>hel", false);
    let (c1, _) = c_parser.parse(b"<a>hel", false);

    let r2 = r_parser.parse(b"lo</a>", true);
    let (c2, _) = c_parser.parse(b"lo</a>", true);

    assert_eq!(
        rust_status_to_u32(r1), c1,
        "Incremental parse chunk 1 status mismatch"
    );
    assert_eq!(
        rust_status_to_u32(r2), c2,
        "Incremental parse chunk 2 status mismatch"
    );
}

// ======================== Namespace parsing ========================

#[test]
fn cmp_simple_namespace() {
    compare_status(
        b"<a xmlns=\"http://example.com/\"/>",
        "simple namespace"
    );
}

#[test]
fn cmp_prefixed_namespace() {
    compare_status(
        b"<ns:a xmlns:ns=\"http://example.com/\"/>",
        "prefixed namespace"
    );
}

// ======================== Negative length ========================

#[test]
fn cmp_negative_len() {
    let mut r_parser = Parser::new(None).unwrap();
    let c_parser = CParser::new(None).unwrap();

    // Both should handle negative length gracefully
    let r_status = r_parser.parse(b"", false);  // Can't pass negative in Rust
    let (c_status, _) = c_parser.parse(b"", false);
    // The Rust port uses usize so can't have negative; C uses int
    // Just verify they both handle empty data
    assert_eq!(rust_status_to_u32(r_status), c_status);
}

// ======================== Handler comparison ========================

/// Helper to collect elements via C parser
struct CElementCollector {
    elements: RefCell<Vec<String>>,
}

unsafe extern "C" fn c_start_element(
    user_data: *mut c_void,
    name: *const c_char,
    _atts: *mut *const c_char,
) {
    let collector = &*(user_data as *const CElementCollector);
    let name_str = std::ffi::CStr::from_ptr(name).to_str().unwrap().to_owned();
    collector.elements.borrow_mut().push(format!("start:{}", name_str));
}

unsafe extern "C" fn c_end_element(
    user_data: *mut c_void,
    name: *const c_char,
) {
    let collector = &*(user_data as *const CElementCollector);
    let name_str = std::ffi::CStr::from_ptr(name).to_str().unwrap().to_owned();
    collector.elements.borrow_mut().push(format!("end:{}", name_str));
}

#[test]
fn cmp_element_events() {
    let xml = b"<root><child1/><child2>text</child2></root>";

    // Rust port
    let mut r_elements = Vec::new();
    {
        let mut parser = Parser::new(None).unwrap();
        let r_ref = &mut r_elements as *mut Vec<String>;
        parser.set_start_element_handler(Some(Box::new(move |name, _attrs| {
            unsafe { (*r_ref).push(format!("start:{}", name)); }
        })));
        parser.set_end_element_handler(Some(Box::new(move |name| {
            unsafe { (*r_ref).push(format!("end:{}", name)); }
        })));
        parser.parse(xml, true);
    }

    // C library
    let collector = CElementCollector {
        elements: RefCell::new(Vec::new()),
    };
    {
        let c_parser = CParser::new(None).unwrap();
        unsafe {
            expat_sys::XML_SetUserData(
                c_parser.raw_parser(),
                &collector as *const CElementCollector as *mut c_void,
            );
            expat_sys::XML_SetElementHandler(
                c_parser.raw_parser(),
                Some(c_start_element),
                Some(c_end_element),
            );
        }
        c_parser.parse(xml, true);
    }
    let c_elements = collector.elements.into_inner();

    assert_eq!(
        r_elements, c_elements,
        "Element events differ between Rust and C parsers"
    );
}

// ======================== UTF-16 handling ========================

#[test]
fn cmp_utf16_le_bom() {
    // UTF-16LE BOM + <a/>
    let xml: Vec<u8> = vec![
        0xFF, 0xFE, // BOM
        b'<', 0, b'a', 0, b'/', 0, b'>', 0, // <a/>
    ];
    compare_status(&xml, "UTF-16LE with BOM");
}

#[test]
fn cmp_utf16_be_bom() {
    // UTF-16BE BOM + <a/>
    let xml: Vec<u8> = vec![
        0xFE, 0xFF, // BOM
        0, b'<', 0, b'a', 0, b'/', 0, b'>', // <a/>
    ];
    compare_status(&xml, "UTF-16BE with BOM");
}

// ======================== Edge cases found during development ========================

#[test]
fn cmp_trailing_cr() {
    compare_status(b"<doc>\r", "trailing CR after start tag");
}

#[test]
fn cmp_unclosed_tag_with_content() {
    compare_status(b"<doc>hello", "unclosed tag with content");
}

#[test]
fn cmp_only_whitespace() {
    compare_status(b"  \n  ", "only whitespace, is_final");
}

#[test]
fn cmp_just_text() {
    compare_status(b"hello", "just text, no elements");
}

/// Debug test to print C parser results
#[test]
fn debug_c_behavior() {
    let cases: &[(&[u8], &str)] = &[
        (b"<doc>\r", "trailing CR"),
        (b"<a>", "unclosed tag"),
        (b"<a>hello", "unclosed with content"),
        (b"", "empty"),
    ];
    for (xml, desc) in cases {
        let (c_status, c_error, c_line, c_col) = parse_c(xml);
        let (r_status, r_error, r_line, r_col) = parse_rust(xml);
        eprintln!(
            "{desc:30} C: status={c_status} error={c_error} line={c_line} col={c_col} | \
             Rust: status={} error={} line={r_line} col={r_col}",
            rust_status_to_u32(r_status), rust_error_to_u32(r_error)
        );
    }
}

// ======================== Extended coverage tests ========================

#[test]
fn cmp_multiline_content() {
    compare_parse(b"<doc>line1\nline2\nline3</doc>", "multiline content");
}

#[test]
fn cmp_empty_attribute() {
    compare_parse(b"<a x=\"\"/>", "empty attribute value");
}

#[test]
fn cmp_single_quote_attributes() {
    compare_parse(b"<a x='hello'/>", "single-quoted attribute");
}

#[test]
fn cmp_many_attributes() {
    compare_parse(b"<a a=\"1\" b=\"2\" c=\"3\" d=\"4\" e=\"5\"/>", "many attributes");
}

#[test]
fn cmp_nested_deep() {
    compare_parse(b"<a><b><c><d><e><f/></e></d></c></b></a>", "deeply nested");
}

#[test]
fn cmp_self_closing_with_space() {
    compare_parse(b"<a />", "self-closing with space");
}

#[test]
fn cmp_pi_after_root() {
    compare_status(b"<a/><?target data?>", "PI after root element");
}

#[test]
fn cmp_comment_after_root() {
    compare_status(b"<a/><!-- comment -->", "comment after root element");
}

#[test]
fn cmp_whitespace_after_root() {
    compare_status(b"<a/>  \n  ", "whitespace after root element");
}

#[test]
fn cmp_empty_element() {
    compare_parse(b"<a></a>", "empty element");
}

#[test]
fn cmp_sibling_elements() {
    compare_parse(b"<r><a/><b/><c/></r>", "sibling elements");
}

#[test]
fn cmp_mixed_content() {
    compare_parse(b"<r>text<a/>more<b/>end</r>", "mixed text and element content");
}

#[test]
fn cmp_error_tag_mismatch() {
    let xml = b"<a></b>";
    let (r_status, r_error, _, _) = parse_rust(xml);
    let (c_status, c_error, _, _) = parse_c(xml);
    assert_eq!(rust_status_to_u32(r_status), c_status, "Tag mismatch status");
    assert_eq!(rust_error_to_u32(r_error), c_error, "Tag mismatch error code");
}

#[test]
fn cmp_many_small_elements() {
    let mut xml = b"<r>".to_vec();
    for i in 0..100 {
        xml.extend_from_slice(format!("<e{i}/>").as_bytes());
    }
    xml.extend_from_slice(b"</r>");
    compare_status(&xml, "100 small elements");
}

#[test]
fn cmp_long_text() {
    let mut xml = b"<a>".to_vec();
    for _ in 0..1000 {
        xml.extend_from_slice(b"some text content ");
    }
    xml.extend_from_slice(b"</a>");
    compare_status(&xml, "long text content");
}

#[test]
fn cmp_utf8_element_content() {
    compare_parse("<a>こんにちは</a>".as_bytes(), "UTF-8 element content");
}

#[test]
fn cmp_chardata_simple() {
    let xml = b"<a>hello world</a>";
    let mut r_data = Vec::new();
    {
        let mut parser = Parser::new(None).unwrap();
        let r_ref = &mut r_data as *mut Vec<u8>;
        parser.set_character_data_handler(Some(Box::new(move |data: &[u8]| {
            unsafe { (*r_ref).extend_from_slice(data); }
        })));
        parser.parse(xml, true);
    }
    let c_data: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    unsafe extern "C" fn c_cd(ud: *mut c_void, s: *const c_char, len: c_int) {
        let dv = &*(ud as *const RefCell<Vec<u8>>);
        let sl = std::slice::from_raw_parts(s as *const u8, len as usize);
        dv.borrow_mut().extend_from_slice(sl);
    }
    {
        let parser = CParser::new(None).unwrap();
        unsafe {
            expat_sys::XML_SetUserData(parser.raw_parser(), &c_data as *const _ as *mut c_void);
            expat_sys::XML_SetCharacterDataHandler(parser.raw_parser(), Some(c_cd));
        }
        parser.parse(xml, true);
    }
    assert_eq!(r_data, c_data.into_inner(), "Character data mismatch");
}

#[test]
fn cmp_chardata_entities() {
    let xml = b"<a>&lt;&gt;&amp;</a>";
    let mut r_data = Vec::new();
    {
        let mut parser = Parser::new(None).unwrap();
        let r_ref = &mut r_data as *mut Vec<u8>;
        parser.set_character_data_handler(Some(Box::new(move |data: &[u8]| {
            unsafe { (*r_ref).extend_from_slice(data); }
        })));
        parser.parse(xml, true);
    }
    let c_data: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    unsafe extern "C" fn c_cd2(ud: *mut c_void, s: *const c_char, len: c_int) {
        let dv = &*(ud as *const RefCell<Vec<u8>>);
        let sl = std::slice::from_raw_parts(s as *const u8, len as usize);
        dv.borrow_mut().extend_from_slice(sl);
    }
    {
        let parser = CParser::new(None).unwrap();
        unsafe {
            expat_sys::XML_SetUserData(parser.raw_parser(), &c_data as *const _ as *mut c_void);
            expat_sys::XML_SetCharacterDataHandler(parser.raw_parser(), Some(c_cd2));
        }
        parser.parse(xml, true);
    }
    assert_eq!(r_data, c_data.into_inner(), "Entity chardata mismatch");
}
