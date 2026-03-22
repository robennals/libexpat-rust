//! Coverage-driven comparison tests: systematically exercise uncovered code paths
//! while verifying C and Rust parsers produce identical results.
//!
//! Every test here runs the same XML through both the C (expat-sys) and Rust (expat-rust)
//! parsers, comparing status codes, error codes, and where applicable, handler callback
//! sequences.

use expat_rust::xmlparse::{Parser, XmlError, XmlStatus};
use expat_sys::CParser;
use std::cell::RefCell;
use std::ffi::{c_char, c_int, c_void, CStr};

// ============================================================================
// Comparison helpers
// ============================================================================

/// Compare parse result (status + error code) between Rust and C
fn compare(xml: &[u8], desc: &str) {
    let mut r_parser = Parser::new(None).unwrap();
    let r_status = r_parser.parse(xml, true) as u32;
    let r_error = r_parser.error_code() as u32;

    let c_parser = CParser::new(None).unwrap();
    let (c_status, c_error) = c_parser.parse(xml, true);

    assert!(
        r_status == c_status && r_error == c_error,
        "MISMATCH {desc}: Rust status={r_status} err={r_error}, C status={c_status} err={c_error}, input={:?}",
        std::str::from_utf8(xml).unwrap_or("<binary>")
    );
}

/// Compare incremental parsing: split input at every byte position
fn compare_incremental(xml: &[u8], desc: &str) {
    compare(xml, desc);
    for split in 1..xml.len() {
        let mut r_parser = Parser::new(None).unwrap();
        let r1 = r_parser.parse(&xml[..split], false);
        let r_final = if r1 == XmlStatus::Ok {
            r_parser.parse(&xml[split..], true)
        } else {
            r1
        };
        let r_err = r_parser.error_code();

        let c_parser = CParser::new(None).unwrap();
        let (c1, _) = c_parser.parse(&xml[..split], false);
        let (c_final, c_err) = if c1 == 1 {
            c_parser.parse(&xml[split..], true)
        } else {
            (c1, c_parser.parse(&xml[split..], true).1)
        };

        assert!(
            r_final as u32 == c_final && r_err as u32 == c_err,
            "INCR MISMATCH {desc} split@{split}: Rust s={} e={}, C s={c_final} e={c_err}",
            r_final as u32,
            r_err as u32
        );
    }
}

// ============================================================================
// Handler event collection for deep comparison
// ============================================================================

/// Collects all SAX events from the Rust parser
fn collect_rust_events(xml: &[u8]) -> (XmlStatus, Vec<String>) {
    let events: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let mut parser = Parser::new(None).unwrap();

    // Use raw pointers for the handlers since they need to capture events
    let ev_ptr = &events as *const RefCell<Vec<String>>;

    parser.set_start_element_handler(Some(Box::new(move |name, attrs| unsafe {
        let mut s = format!("SE:{}", name);
        for (k, v) in attrs {
            s.push_str(&format!(" {}={}", k, v));
        }
        (*ev_ptr).borrow_mut().push(s);
    })));

    let ev_ptr2 = &events as *const RefCell<Vec<String>>;
    parser.set_end_element_handler(Some(Box::new(move |name| unsafe {
        (*ev_ptr2).borrow_mut().push(format!("EE:{}", name));
    })));

    let ev_ptr3 = &events as *const RefCell<Vec<String>>;
    parser.set_character_data_handler(Some(Box::new(move |data: &[u8]| unsafe {
        let text = std::str::from_utf8(data).unwrap_or("<binary>");
        (*ev_ptr3).borrow_mut().push(format!("CD:{}", text));
    })));

    let ev_ptr4 = &events as *const RefCell<Vec<String>>;
    parser.set_processing_instruction_handler(Some(Box::new(move |target, data| unsafe {
        (*ev_ptr4)
            .borrow_mut()
            .push(format!("PI:{}:{}", target, data));
    })));

    let ev_ptr5 = &events as *const RefCell<Vec<String>>;
    parser.set_comment_handler(Some(Box::new(move |text: &[u8]| unsafe {
        let t = std::str::from_utf8(text).unwrap_or("<binary>");
        (*ev_ptr5).borrow_mut().push(format!("CM:{}", t));
    })));

    let ev_ptr6 = &events as *const RefCell<Vec<String>>;
    parser.set_start_cdata_section_handler(Some(Box::new(move || unsafe {
        (*ev_ptr6).borrow_mut().push("SCD".to_string());
    })));

    let ev_ptr7 = &events as *const RefCell<Vec<String>>;
    parser.set_end_cdata_section_handler(Some(Box::new(move || unsafe {
        (*ev_ptr7).borrow_mut().push("ECD".to_string());
    })));

    let ev_ptr8 = &events as *const RefCell<Vec<String>>;
    parser.set_start_doctype_decl_handler(Some(Box::new(
        move |name, sysid, pubid, has_internal| unsafe {
            (*ev_ptr8).borrow_mut().push(format!(
                "SDT:{}:{}:{}:{}",
                name,
                sysid.unwrap_or(""),
                pubid.unwrap_or(""),
                has_internal
            ));
        },
    )));

    let ev_ptr9 = &events as *const RefCell<Vec<String>>;
    parser.set_end_doctype_decl_handler(Some(Box::new(move || unsafe {
        (*ev_ptr9).borrow_mut().push("EDT".to_string());
    })));

    let status = parser.parse(xml, true);
    (status, events.into_inner())
}

/// Collects all SAX events from the C parser
fn collect_c_events(xml: &[u8]) -> (u32, Vec<String>) {
    let events: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let c_parser = CParser::new(None).unwrap();

    unsafe extern "C" fn c_start_el(ud: *mut c_void, name: *const c_char, atts: *mut *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let n = CStr::from_ptr(name).to_str().unwrap();
        let mut s = format!("SE:{}", n);
        let mut i = 0;
        loop {
            let key = *atts.add(i);
            if key.is_null() {
                break;
            }
            let val = *atts.add(i + 1);
            let k = CStr::from_ptr(key).to_str().unwrap();
            let v = CStr::from_ptr(val).to_str().unwrap();
            s.push_str(&format!(" {}={}", k, v));
            i += 2;
        }
        ev.borrow_mut().push(s);
    }

    unsafe extern "C" fn c_end_el(ud: *mut c_void, name: *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let n = CStr::from_ptr(name).to_str().unwrap();
        ev.borrow_mut().push(format!("EE:{}", n));
    }

    unsafe extern "C" fn c_chardata(ud: *mut c_void, s: *const c_char, len: c_int) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let sl = std::slice::from_raw_parts(s as *const u8, len as usize);
        let text = std::str::from_utf8(sl).unwrap_or("<binary>");
        ev.borrow_mut().push(format!("CD:{}", text));
    }

    unsafe extern "C" fn c_pi(ud: *mut c_void, target: *const c_char, data: *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let t = CStr::from_ptr(target).to_str().unwrap();
        let d = if data.is_null() {
            ""
        } else {
            CStr::from_ptr(data).to_str().unwrap()
        };
        ev.borrow_mut().push(format!("PI:{}:{}", t, d));
    }

    unsafe extern "C" fn c_comment(ud: *mut c_void, text: *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let t = CStr::from_ptr(text).to_str().unwrap();
        ev.borrow_mut().push(format!("CM:{}", t));
    }

    unsafe extern "C" fn c_start_cdata(ud: *mut c_void) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        ev.borrow_mut().push("SCD".to_string());
    }

    unsafe extern "C" fn c_end_cdata(ud: *mut c_void) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        ev.borrow_mut().push("ECD".to_string());
    }

    unsafe extern "C" fn c_start_doctype(
        ud: *mut c_void,
        name: *const c_char,
        sysid: *const c_char,
        pubid: *const c_char,
        has_internal: c_int,
    ) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let n = CStr::from_ptr(name).to_str().unwrap();
        let s = if sysid.is_null() {
            ""
        } else {
            CStr::from_ptr(sysid).to_str().unwrap()
        };
        let p = if pubid.is_null() {
            ""
        } else {
            CStr::from_ptr(pubid).to_str().unwrap()
        };
        ev.borrow_mut()
            .push(format!("SDT:{}:{}:{}:{}", n, s, p, has_internal != 0));
    }

    unsafe extern "C" fn c_end_doctype(ud: *mut c_void) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        ev.borrow_mut().push("EDT".to_string());
    }

    unsafe {
        let ud = &events as *const RefCell<Vec<String>> as *mut c_void;
        expat_sys::XML_SetUserData(c_parser.raw_parser(), ud);
        expat_sys::XML_SetElementHandler(c_parser.raw_parser(), Some(c_start_el), Some(c_end_el));
        expat_sys::XML_SetCharacterDataHandler(c_parser.raw_parser(), Some(c_chardata));
        expat_sys::XML_SetProcessingInstructionHandler(c_parser.raw_parser(), Some(c_pi));
        expat_sys::XML_SetCommentHandler(c_parser.raw_parser(), Some(c_comment));
        expat_sys::XML_SetCdataSectionHandler(
            c_parser.raw_parser(),
            Some(c_start_cdata),
            Some(c_end_cdata),
        );
        expat_sys::XML_SetDoctypeDeclHandler(
            c_parser.raw_parser(),
            Some(c_start_doctype),
            Some(c_end_doctype),
        );
    }

    let (status, _error) = c_parser.parse(xml, true);
    (status, events.into_inner())
}

/// Compare full SAX event sequences between C and Rust
fn compare_events(xml: &[u8], desc: &str) {
    let (r_status, r_events) = collect_rust_events(xml);
    let (c_status, c_events) = collect_c_events(xml);

    assert_eq!(
        r_status as u32, c_status,
        "Status mismatch for {desc}: Rust={}, C={c_status}",
        r_status as u32
    );
    // Normalize character data: merge adjacent CD events for comparison
    // (C and Rust may chunk character data differently, which is allowed by SAX)
    let r_merged = merge_chardata(&r_events);
    let c_merged = merge_chardata(&c_events);
    assert_eq!(
        r_merged, c_merged,
        "Event mismatch for {desc}:\n  Rust: {:?}\n  C:    {:?}",
        r_merged, c_merged
    );
}

/// Merge adjacent CD: events into one (SAX allows splitting character data)
fn merge_chardata(events: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for ev in events {
        if ev.starts_with("CD:") {
            if let Some(last) = result.last_mut() {
                if last.starts_with("CD:") {
                    last.push_str(&ev[3..]);
                    continue;
                }
            }
        }
        result.push(ev.clone());
    }
    result
}

// ============================================================================
// Processing Instructions
// ============================================================================

#[test]
fn cov_pi_in_content() {
    compare_events(b"<r><?target data?></r>", "PI in content");
}

#[test]
fn cov_pi_empty_data() {
    compare_events(b"<r><?target?></r>", "PI with empty data");
}

#[test]
fn cov_pi_before_root() {
    compare_events(b"<?xml-stylesheet type='text/xsl'?><r/>", "PI before root");
}

#[test]
fn cov_pi_multiple() {
    compare_events(
        b"<?pi1 d1?><r><?pi2 d2?></r><?pi3 d3?>",
        "multiple PIs",
    );
}

// ============================================================================
// Comments
// ============================================================================

#[test]
fn cov_comment_in_content() {
    compare_events(b"<r><!-- hello --></r>", "comment in content");
}

#[test]
fn cov_comment_empty() {
    compare_events(b"<r><!----></r>", "empty comment");
}

#[test]
fn cov_comment_multiline() {
    compare_events(
        b"<r><!-- line1\nline2\nline3 --></r>",
        "multiline comment",
    );
}

#[test]
fn cov_comment_in_prolog() {
    compare_events(b"<!-- prolog comment --><r/>", "comment in prolog");
}

#[test]
fn cov_comment_in_epilog() {
    compare_events(b"<r/><!-- epilog comment -->", "comment in epilog");
}

// ============================================================================
// CDATA Sections
// ============================================================================

#[test]
fn cov_cdata_simple() {
    compare_events(b"<r><![CDATA[hello]]></r>", "simple CDATA");
}

#[test]
fn cov_cdata_empty() {
    compare_events(b"<r><![CDATA[]]></r>", "empty CDATA");
}

#[test]
fn cov_cdata_with_special_chars() {
    compare_events(
        b"<r><![CDATA[<not>&element;]]></r>",
        "CDATA with XML special chars",
    );
}

#[test]
fn cov_cdata_with_brackets() {
    compare_events(
        b"<r><![CDATA[text]more]]></r>",
        "CDATA with single bracket",
    );
}

#[test]
fn cov_cdata_multiline() {
    compare_events(
        b"<r><![CDATA[line1\nline2\r\nline3]]></r>",
        "multiline CDATA",
    );
}

#[test]
fn cov_cdata_incremental() {
    compare_incremental(b"<r><![CDATA[hello world]]></r>", "CDATA incremental");
}

#[test]
fn cov_cdata_with_newlines_in_content() {
    compare_events(
        b"<r><![CDATA[\r\n\r]]></r>",
        "CDATA with CR/LF",
    );
}

// ============================================================================
// DOCTYPE Declarations
// ============================================================================

#[test]
fn cov_doctype_simple() {
    compare_events(
        b"<!DOCTYPE root><root/>",
        "simple DOCTYPE",
    );
}

#[test]
fn cov_doctype_with_internal_subset() {
    compare_events(
        b"<!DOCTYPE root [<!ELEMENT root (#PCDATA)>]><root/>",
        "DOCTYPE with internal subset",
    );
}

#[test]
fn cov_doctype_system() {
    compare_events(
        b"<!DOCTYPE root SYSTEM \"test.dtd\"><root/>",
        "DOCTYPE with SYSTEM",
    );
}

#[test]
fn cov_doctype_public() {
    compare_events(
        b"<!DOCTYPE root PUBLIC \"-//Test//DTD Test//EN\" \"test.dtd\"><root/>",
        "DOCTYPE with PUBLIC",
    );
}

#[test]
fn cov_doctype_entity_decl() {
    compare_events(
        b"<!DOCTYPE r [<!ENTITY e 'val'>]><r>&e;</r>",
        "DOCTYPE entity declaration + use",
    );
}

// ============================================================================
// NOTATION Declarations (xmlrole.rs coverage)
// ============================================================================

#[test]
fn cov_notation_system() {
    compare(
        b"<!DOCTYPE r [<!NOTATION n SYSTEM \"http://example.com\">]><r/>",
        "NOTATION SYSTEM",
    );
}

#[test]
fn cov_notation_public() {
    compare(
        b"<!DOCTYPE r [<!NOTATION n PUBLIC \"-//Test//N//EN\">]><r/>",
        "NOTATION PUBLIC",
    );
}

#[test]
fn cov_notation_public_system() {
    compare(
        b"<!DOCTYPE r [<!NOTATION n PUBLIC \"-//Test//N//EN\" \"http://example.com\">]><r/>",
        "NOTATION PUBLIC SYSTEM",
    );
}

// ============================================================================
// Complex ELEMENT Declarations (xmlrole.rs content model coverage)
// ============================================================================

#[test]
fn cov_element_empty() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r EMPTY>]><r/>",
        "ELEMENT EMPTY",
    );
}

#[test]
fn cov_element_any() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r ANY>]><r/>",
        "ELEMENT ANY",
    );
}

#[test]
fn cov_element_mixed() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (#PCDATA|a|b)*><!ELEMENT a (#PCDATA)><!ELEMENT b (#PCDATA)>]><r><a/><b/></r>",
        "ELEMENT mixed content model",
    );
}

#[test]
fn cov_element_seq() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (a,b,c)><!ELEMENT a (#PCDATA)><!ELEMENT b (#PCDATA)><!ELEMENT c (#PCDATA)>]><r><a/><b/><c/></r>",
        "ELEMENT sequence content model",
    );
}

#[test]
fn cov_element_choice() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (a|b|c)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><a/></r>",
        "ELEMENT choice content model",
    );
}

#[test]
fn cov_element_nested_groups() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r ((a|b),c)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><a/><c/></r>",
        "ELEMENT nested groups",
    );
}

#[test]
fn cov_element_quantifiers() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (a?,b*,c+)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><c/></r>",
        "ELEMENT with quantifiers ?,*,+",
    );
}

#[test]
fn cov_element_group_close_rep() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (a)*><!ELEMENT a EMPTY>]><r><a/><a/></r>",
        "ELEMENT group close with *",
    );
}

#[test]
fn cov_element_group_close_opt() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (a)?><!ELEMENT a EMPTY>]><r/>",
        "ELEMENT group close with ?",
    );
}

#[test]
fn cov_element_group_close_plus() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (a)+><!ELEMENT a EMPTY>]><r><a/></r>",
        "ELEMENT group close with +",
    );
}

// ============================================================================
// ATTLIST Declarations
// ============================================================================

#[test]
fn cov_attlist_cdata() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #IMPLIED>]><r a=\"hello\"/>",
        "ATTLIST CDATA",
    );
}

#[test]
fn cov_attlist_required() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #REQUIRED>]><r a=\"val\"/>",
        "ATTLIST REQUIRED",
    );
}

#[test]
fn cov_attlist_default() {
    compare_events(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA \"default\">]><r/>",
        "ATTLIST with default value",
    );
}

#[test]
fn cov_attlist_fixed() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #FIXED \"fixed\">]><r/>",
        "ATTLIST FIXED",
    );
}

#[test]
fn cov_attlist_enum() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a (x|y|z) #IMPLIED>]><r a=\"x\"/>",
        "ATTLIST enumeration",
    );
}

#[test]
fn cov_attlist_multiple_attrs() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #IMPLIED b CDATA #IMPLIED>]><r a=\"1\" b=\"2\"/>",
        "ATTLIST multiple attributes",
    );
}

// ============================================================================
// External Entity Declarations
// ============================================================================

#[test]
fn cov_entity_external_system() {
    compare(
        b"<!DOCTYPE r [<!ENTITY e SYSTEM \"file.xml\">]><r/>",
        "external entity SYSTEM",
    );
}

#[test]
fn cov_entity_external_public() {
    compare(
        b"<!DOCTYPE r [<!ENTITY e PUBLIC \"-//Test//E//EN\" \"file.xml\">]><r/>",
        "external entity PUBLIC",
    );
}

#[test]
fn cov_entity_unparsed() {
    compare(
        b"<!DOCTYPE r [<!NOTATION n SYSTEM \"x\"><!ENTITY e SYSTEM \"file.xml\" NDATA n>]><r/>",
        "unparsed entity with NDATA",
    );
}

// ============================================================================
// Attribute Value Edge Cases (xmlparse.rs attribute processing coverage)
// ============================================================================

#[test]
fn cov_attr_with_char_ref() {
    compare_events(
        b"<r a=\"&#65;&#x42;\"/>",
        "attribute with char refs (A, B)",
    );
}

#[test]
fn cov_attr_with_entity_ref() {
    compare_events(
        b"<r a=\"&lt;&gt;&amp;&apos;&quot;\"/>",
        "attribute with all predefined entity refs",
    );
}

#[test]
fn cov_attr_with_newlines() {
    compare_events(
        b"<r a=\"hello\nworld\"/>",
        "attribute with newline (should normalize)",
    );
}

#[test]
fn cov_attr_with_cr() {
    compare_events(
        b"<r a=\"hello\rworld\"/>",
        "attribute with CR",
    );
}

#[test]
fn cov_attr_with_crlf() {
    compare_events(
        b"<r a=\"hello\r\nworld\"/>",
        "attribute with CRLF",
    );
}

#[test]
fn cov_attr_with_tab() {
    compare_events(
        b"<r a=\"hello\tworld\"/>",
        "attribute with tab",
    );
}

// ============================================================================
// Character References
// ============================================================================

#[test]
fn cov_charref_decimal() {
    compare_events(b"<r>&#65;&#66;&#67;</r>", "decimal char refs ABC");
}

#[test]
fn cov_charref_hex() {
    compare_events(b"<r>&#x41;&#x42;&#x43;</r>", "hex char refs ABC");
}

#[test]
fn cov_charref_high_unicode() {
    compare_events(b"<r>&#x20AC;</r>", "Euro sign char ref");
}

#[test]
fn cov_charref_invalid() {
    compare(b"<r>&#xFFFFFF;</r>", "invalid char ref");
}

#[test]
fn cov_charref_zero() {
    compare(b"<r>&#0;</r>", "zero char ref (invalid)");
}

// ============================================================================
// XML Declaration
// ============================================================================

#[test]
fn cov_xmldecl_version_only() {
    compare(b"<?xml version='1.0'?><r/>", "XML decl version only");
}

#[test]
fn cov_xmldecl_with_encoding() {
    compare(
        b"<?xml version='1.0' encoding='UTF-8'?><r/>",
        "XML decl with encoding",
    );
}

#[test]
fn cov_xmldecl_standalone_yes() {
    compare(
        b"<?xml version='1.0' standalone='yes'?><r/>",
        "XML decl standalone=yes",
    );
}

#[test]
fn cov_xmldecl_standalone_no() {
    compare(
        b"<?xml version='1.0' standalone='no'?><r/>",
        "XML decl standalone=no",
    );
}

#[test]
fn cov_xmldecl_full() {
    compare(
        b"<?xml version='1.0' encoding='UTF-8' standalone='yes'?><r/>",
        "full XML declaration",
    );
}

// ============================================================================
// UTF-16 Encoding
// ============================================================================

fn make_utf16le(s: &str) -> Vec<u8> {
    let mut out = vec![0xFF, 0xFE]; // BOM
    for c in s.encode_utf16() {
        out.push(c as u8);
        out.push((c >> 8) as u8);
    }
    out
}

fn make_utf16be(s: &str) -> Vec<u8> {
    let mut out = vec![0xFE, 0xFF]; // BOM
    for c in s.encode_utf16() {
        out.push((c >> 8) as u8);
        out.push(c as u8);
    }
    out
}

#[test]
fn cov_utf16le_simple() {
    let xml = make_utf16le("<r/>");
    compare(&xml, "UTF-16LE simple");
}

#[test]
fn cov_utf16be_simple() {
    let xml = make_utf16be("<r/>");
    compare(&xml, "UTF-16BE simple");
}

#[test]
fn cov_utf16le_with_content() {
    let xml = make_utf16le("<r>hello</r>");
    compare(&xml, "UTF-16LE with content");
}

#[test]
fn cov_utf16be_with_content() {
    let xml = make_utf16be("<r>hello</r>");
    compare(&xml, "UTF-16BE with content");
}

#[test]
fn cov_utf16le_with_attrs() {
    let xml = make_utf16le("<r a=\"1\" b=\"2\"/>");
    compare(&xml, "UTF-16LE with attributes");
}

#[test]
fn cov_utf16le_nested() {
    let xml = make_utf16le("<r><a><b/></a></r>");
    compare(&xml, "UTF-16LE nested elements");
}

#[test]
fn cov_utf16le_incremental() {
    let xml = make_utf16le("<r>text</r>");
    compare_incremental(&xml, "UTF-16LE incremental");
}

// ============================================================================
// Error Cases
// ============================================================================

#[test]
fn cov_error_cdata_close_in_content() {
    compare(b"<r>]]></r>", "]]> in content (error)");
}

#[test]
fn cov_error_unclosed_cdata() {
    compare(b"<r><![CDATA[unclosed", "unclosed CDATA");
}

#[test]
fn cov_error_unclosed_comment() {
    compare(b"<r><!-- unclosed", "unclosed comment");
}

#[test]
fn cov_error_unclosed_pi() {
    compare(b"<r><?target unclosed", "unclosed PI");
}

#[test]
fn cov_error_double_root() {
    compare(b"<r/><s/>", "double root element");
}

#[test]
fn cov_error_text_after_root() {
    compare(b"<r/>text", "text after root");
}

#[test]
fn cov_error_invalid_tag_name() {
    compare(b"<123/>", "tag starting with digit");
}

#[test]
fn cov_error_ampersand_alone() {
    compare(b"<r>&</r>", "bare ampersand");
}

#[test]
fn cov_error_malformed_entity() {
    compare(b"<r>&entity</r>", "entity without semicolon");
}

#[test]
fn cov_error_duplicate_attr() {
    compare(b"<r a=\"1\" a=\"2\"/>", "duplicate attribute");
}

#[test]
fn cov_error_lt_in_attr() {
    compare(b"<r a=\"<\"/>", "< in attribute value");
}

#[test]
fn cov_error_no_attr_value() {
    compare(b"<r a/>", "attribute without value");
}

#[test]
fn cov_error_malformed_close_tag() {
    compare(b"<r></r extra>", "close tag with extra content");
}

#[test]
fn cov_error_invalid_char_ref() {
    compare(b"<r>&#xD800;</r>", "surrogate char ref");
}

#[test]
fn cov_error_publicid_bad_char() {
    compare(
        b"<!DOCTYPE r PUBLIC \"bad{char\" \"test.dtd\"><r/>",
        "invalid char in PUBLIC ID",
    );
}

// ============================================================================
// Epilog Edge Cases
// ============================================================================

#[test]
fn cov_epilog_whitespace() {
    compare(b"<r/>  \n\t  ", "whitespace in epilog");
}

#[test]
fn cov_epilog_comment() {
    compare_events(b"<r/><!-- epilog -->", "comment in epilog");
}

#[test]
fn cov_epilog_pi() {
    compare_events(b"<r/><?pi data?>", "PI in epilog");
}

#[test]
fn cov_epilog_multiple() {
    compare_events(
        b"<r/><!-- c1 --><?pi d?>\n<!-- c2 -->",
        "multiple items in epilog",
    );
}

// ============================================================================
// Incremental Parsing (cross-boundary splits)
// ============================================================================

#[test]
fn cov_incr_comment() {
    compare_incremental(b"<r><!-- comment --></r>", "incremental comment");
}

#[test]
fn cov_incr_pi() {
    compare_incremental(b"<r><?target data?></r>", "incremental PI");
}

#[test]
fn cov_incr_entity() {
    compare_incremental(b"<r>&amp;&lt;</r>", "incremental entities");
}

#[test]
fn cov_incr_charref() {
    compare_incremental(b"<r>&#65;&#x42;</r>", "incremental char refs");
}

#[test]
fn cov_incr_attrs() {
    compare_incremental(
        b"<r a=\"hello\" b=\"world\">text</r>",
        "incremental with attrs",
    );
}

#[test]
fn cov_incr_doctype() {
    compare_incremental(
        b"<!DOCTYPE r [<!ENTITY e 'v'>]><r>&e;</r>",
        "incremental DOCTYPE",
    );
}

#[test]
fn cov_incr_mixed() {
    compare_incremental(
        b"<?xml version='1.0'?><!-- c --><r a=\"1\">text<![CDATA[cd]]>&amp;</r><!-- end -->",
        "incremental mixed content",
    );
}

// ============================================================================
// Parser Reset
// ============================================================================

#[test]
fn cov_parser_reset() {
    // Parse, reset, parse again — compare with fresh C parser each time
    let xml1 = b"<a>text1</a>";
    let xml2 = b"<b>text2</b>";

    // First parse
    let mut r_parser = Parser::new(None).unwrap();
    let r1_status = r_parser.parse(xml1, true) as u32;
    let r1_error = r_parser.error_code() as u32;
    let c1 = CParser::new(None).unwrap();
    let (c1_status, c1_error) = c1.parse(xml1, true);
    assert_eq!(r1_status, c1_status, "first parse status");
    assert_eq!(r1_error, c1_error, "first parse error");

    // Reset
    r_parser.reset(None);

    // Second parse
    let r2_status = r_parser.parse(xml2, true) as u32;
    let r2_error = r_parser.error_code() as u32;
    let c2 = CParser::new(None).unwrap();
    let (c2_status, c2_error) = c2.parse(xml2, true);
    assert_eq!(r2_status, c2_status, "reset parse status");
    assert_eq!(r2_error, c2_error, "reset parse error");
}

// ============================================================================
// Entity Expansion in Content
// ============================================================================

#[test]
fn cov_entity_in_content() {
    compare_events(
        b"<!DOCTYPE r [<!ENTITY e 'expanded'>]><r>&e;</r>",
        "entity expansion in content",
    );
}

#[test]
fn cov_entity_with_markup() {
    compare_events(
        b"<!DOCTYPE r [<!ENTITY e '<child/>'>]><r>&e;</r>",
        "entity with embedded markup",
    );
}

#[test]
fn cov_entity_recursive() {
    compare(
        b"<!DOCTYPE r [<!ENTITY a '&b;'><!ENTITY b '&a;'>]><r>&a;</r>",
        "recursive entity (error)",
    );
}

#[test]
fn cov_entity_predefined() {
    compare_events(
        b"<r>&amp;&lt;&gt;&apos;&quot;</r>",
        "predefined entities",
    );
}

// ============================================================================
// Whitespace and Newline Handling
// ============================================================================

#[test]
fn cov_cr_lf_normalization() {
    compare_events(b"<r>a\r\nb\rc\nd</r>", "CR/LF normalization in content");
}

#[test]
fn cov_whitespace_only_content() {
    compare_events(b"<r>  \t\n  </r>", "whitespace-only content");
}

#[test]
fn cov_trailing_cr_in_content() {
    // CR at end of a chunk — tests TrailingCr token
    compare_incremental(b"<r>hello\r</r>", "trailing CR incremental");
}

// ============================================================================
// ]]> in Content (tests content_tok RSQB path in xmltok_impl.rs)
// ============================================================================

#[test]
fn cov_single_rsqb_in_content() {
    compare_events(b"<r>a]b</r>", "single ] in content");
}

#[test]
fn cov_double_rsqb_in_content() {
    compare_events(b"<r>a]]b</r>", "double ]] in content (no >)");
}

// ============================================================================
// Deeply Nested / Large Inputs
// ============================================================================

#[test]
fn cov_deep_nesting() {
    let mut xml = Vec::new();
    for i in 0..50 {
        xml.extend_from_slice(format!("<e{}>", i).as_bytes());
    }
    xml.extend_from_slice(b"text");
    for i in (0..50).rev() {
        xml.extend_from_slice(format!("</e{}>", i).as_bytes());
    }
    compare(&xml, "50-level nesting");
}

#[test]
fn cov_many_attrs() {
    let mut xml = b"<r".to_vec();
    for i in 0..50 {
        xml.extend_from_slice(format!(" a{}=\"v{}\"", i, i).as_bytes());
    }
    xml.extend_from_slice(b"/>");
    compare(&xml, "50 attributes");
}

// ============================================================================
// Mixed Content (thorough)
// ============================================================================

#[test]
fn cov_complex_document() {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE doc [
  <!ELEMENT doc (#PCDATA|p|b)*>
  <!ELEMENT p (#PCDATA)>
  <!ELEMENT b (#PCDATA)>
  <!ENTITY copy "&#169;">
  <!ENTITY greeting "Hello World">
  <!ATTLIST doc version CDATA #IMPLIED>
]>
<doc version="1.0">
  <!-- This is a comment -->
  <p>Paragraph with &amp; entity and &#169; char ref</p>
  <?app-info processing instruction?>
  <b>Bold &greeting;</b>
  <![CDATA[Raw <data> & stuff]]>
</doc>"#;
    compare_events(xml, "complex document");
}

#[test]
fn cov_complex_incremental() {
    let xml = b"<?xml version='1.0'?><!DOCTYPE r [<!ENTITY e 'val'>]><r a=\"1\">&e;<![CDATA[cd]]><!-- c --><?p d?></r>";
    compare_incremental(xml, "complex incremental");
}

// ============================================================================
// Edge: empty input, BOM only, etc.
// ============================================================================

#[test]
fn cov_empty_input_final() {
    compare(b"", "empty input is_final");
}

#[test]
fn cov_utf8_bom() {
    compare(b"\xEF\xBB\xBF<r/>", "UTF-8 BOM");
}

#[test]
fn cov_utf8_bom_with_xmldecl() {
    compare(
        b"\xEF\xBB\xBF<?xml version='1.0' encoding='UTF-8'?><r/>",
        "UTF-8 BOM with XML decl",
    );
}

// ============================================================================
// Encoding edge cases
// ============================================================================

#[test]
fn cov_latin1_declared() {
    let xml = b"<?xml version='1.0' encoding='ISO-8859-1'?><r/>";
    compare(xml, "latin1 declared encoding");
}

#[test]
fn cov_usascii_declared() {
    let xml = b"<?xml version='1.0' encoding='US-ASCII'?><r/>";
    compare(xml, "us-ascii declared encoding");
}

// ============================================================================
// Various token types in prolog (xmlrole.rs + xmltok_impl prolog_tok coverage)
// ============================================================================

#[test]
fn cov_prolog_comment() {
    compare(
        b"<!-- c1 --><!-- c2 --><r/>",
        "multiple prolog comments",
    );
}

#[test]
fn cov_prolog_pi() {
    compare(
        b"<?pi1 d1?><?pi2 d2?><r/>",
        "multiple prolog PIs",
    );
}

#[test]
fn cov_prolog_whitespace() {
    compare(b"  \n\t  <r/>", "prolog whitespace");
}

#[test]
fn cov_prolog_complex() {
    let xml = br#"<?xml version="1.0"?>
<!-- comment -->
<?stylesheet type="text/xsl"?>
<!DOCTYPE root [
  <!ELEMENT root (a|b)*>
  <!ELEMENT a EMPTY>
  <!ELEMENT b EMPTY>
  <!ATTLIST root id CDATA #IMPLIED>
  <!ENTITY greeting "hello">
  <!NOTATION jpeg SYSTEM "viewer.exe">
]>
<root id="1"><a/><b/></root>"#;
    compare(xml, "complex prolog");
}

// ============================================================================
// DTD internal subset edge cases (xmlrole.rs coverage)
// ============================================================================

#[test]
fn cov_dtd_comment_in_subset() {
    compare(
        b"<!DOCTYPE r [<!-- dtd comment -->]><r/>",
        "comment in DTD subset",
    );
}

#[test]
fn cov_dtd_pi_in_subset() {
    compare(
        b"<!DOCTYPE r [<?dtd-pi data?>]><r/>",
        "PI in DTD subset",
    );
}

#[test]
fn cov_dtd_whitespace_in_subset() {
    compare(
        b"<!DOCTYPE r [\n  \n]><r/>",
        "whitespace in DTD subset",
    );
}

#[test]
fn cov_dtd_multiple_element_decls() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (a,b,c)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c (#PCDATA)>]><r><a/><b/><c/></r>",
        "multiple ELEMENT declarations",
    );
}

// ============================================================================
// Attribute value processing in DTD (xmlparse.rs validate_attribute_value coverage)
// ============================================================================

#[test]
fn cov_attlist_default_with_charref() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA \"&#65;\">]><r/>",
        "ATTLIST default with char ref",
    );
}

#[test]
fn cov_attlist_default_with_entity() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA \"&amp;\">]><r/>",
        "ATTLIST default with entity ref",
    );
}

// ============================================================================
// Multi-byte UTF-8 characters in various positions
// ============================================================================

#[test]
fn cov_utf8_2byte_content() {
    // First test without entities to isolate the issue
    compare_events("<r>café</r>".as_bytes(), "2-byte UTF-8 in content no entity");
}

#[test]
fn cov_utf8_2byte_entity() {
    compare_events("<!DOCTYPE r [<!ENTITY e 'ñ'>]><r>&e;</r>".as_bytes(), "2-byte UTF-8 entity value");
}

#[test]
fn cov_utf8_3byte_content() {
    compare_events("<r>日本語</r>".as_bytes(), "3-byte UTF-8 CJK");
}

#[test]
fn cov_utf8_4byte_content() {
    compare_events("<r>😀🎉</r>".as_bytes(), "4-byte UTF-8 emoji");
}

#[test]
fn cov_utf8_in_attr() {
    compare_events("<r a=\"日本語\"/>".as_bytes(), "UTF-8 in attribute");
}

#[test]
fn cov_utf8_in_comment() {
    compare_events("<r><!-- 日本語 --></r>".as_bytes(), "UTF-8 in comment");
}

// ============================================================================
// API: error_string coverage
// ============================================================================

#[test]
fn cov_error_string() {
    use expat_rust::xmlparse::error_string;
    // Just verify it doesn't panic for all error codes
    let errors = [
        XmlError::None,
        XmlError::NoMemory,
        XmlError::Syntax,
        XmlError::NoElements,
        XmlError::InvalidToken,
        XmlError::UnclosedToken,
        XmlError::PartialChar,
        XmlError::TagMismatch,
        XmlError::DuplicateAttribute,
        XmlError::JunkAfterDocElement,
        XmlError::ParamEntityRef,
        XmlError::UndefinedEntity,
        XmlError::RecursiveEntityRef,
        XmlError::AsyncEntity,
        XmlError::BadCharRef,
        XmlError::BinaryEntityRef,
        XmlError::AttributeExternalEntityRef,
        XmlError::MisplacedXmlPi,
        XmlError::UnknownEncoding,
        XmlError::IncorrectEncoding,
        XmlError::UnclosedCdataSection,
        XmlError::ExternalEntityHandling,
        XmlError::NotStandalone,
        XmlError::UnexpectedState,
        XmlError::EntityDeclaredInPe,
        XmlError::FeatureRequiresXmlDtd,
        XmlError::CantChangeFeatureOnceParsing,
        XmlError::UnboundPrefix,
        XmlError::UndeclaringPrefix,
        XmlError::IncompletePe,
        XmlError::XmlDecl,
        XmlError::TextDecl,
        XmlError::Publicid,
        XmlError::Suspended,
        XmlError::NotSuspended,
        XmlError::Aborted,
        XmlError::Finished,
        XmlError::SuspendPe,
        XmlError::ReservedPrefixXml,
        XmlError::ReservedPrefixXmlns,
        XmlError::ReservedNamespaceUri,
        XmlError::InvalidArgument,
        XmlError::NoBuffer,
        XmlError::AmplificationLimitBreach,
        XmlError::NotStarted,
    ];
    for e in errors {
        let s = error_string(e);
        assert!(!s.is_empty(), "error_string({:?}) should not be empty", e);
    }
}

// ============================================================================
// Split parsing at token boundaries (exercises TrailingCr, Partial, PartialChar)
// ============================================================================

#[test]
fn cov_split_in_tag() {
    // Split in the middle of a tag name
    let mut r = Parser::new(None).unwrap();
    let r1 = r.parse(b"<ro", false);
    assert_eq!(r1, XmlStatus::Ok);
    let r2 = r.parse(b"ot/>", true) as u32;
    let r_err = r.error_code() as u32;

    let c = CParser::new(None).unwrap();
    let (c1, _) = c.parse(b"<ro", false);
    assert_eq!(c1, 1);
    let (c2, c_err) = c.parse(b"ot/>", true);
    assert_eq!(r2, c2, "split in tag status");
    assert_eq!(r_err, c_err, "split in tag error");
}

#[test]
fn cov_split_in_attr_value() {
    let mut r = Parser::new(None).unwrap();
    let _ = r.parse(b"<r a=\"hel", false);
    let r2 = r.parse(b"lo world\"/>", true) as u32;
    let r_err = r.error_code() as u32;

    let c = CParser::new(None).unwrap();
    let _ = c.parse(b"<r a=\"hel", false);
    let (c2, c_err) = c.parse(b"lo world\"/>", true);
    assert_eq!(r2, c2, "split in attr value status");
    assert_eq!(r_err, c_err, "split in attr value error");
}

#[test]
fn cov_split_in_entity() {
    let mut r = Parser::new(None).unwrap();
    let _ = r.parse(b"<r>&am", false);
    let r2 = r.parse(b"p;</r>", true) as u32;
    let r_err = r.error_code() as u32;

    let c = CParser::new(None).unwrap();
    let _ = c.parse(b"<r>&am", false);
    let (c2, c_err) = c.parse(b"p;</r>", true);
    assert_eq!(r2, c2, "split in entity status");
    assert_eq!(r_err, c_err, "split in entity error");
}

// ============================================================================
// Non-final empty parse followed by final
// ============================================================================

#[test]
fn cov_empty_non_final_then_data() {
    let mut r = Parser::new(None).unwrap();
    let _ = r.parse(b"", false);
    let r_s = r.parse(b"<r/>", true) as u32;
    let r_e = r.error_code() as u32;

    let c = CParser::new(None).unwrap();
    let _ = c.parse(b"", false);
    let (c_s, c_e) = c.parse(b"<r/>", true);
    assert_eq!(r_s, c_s, "empty non-final status");
    assert_eq!(r_e, c_e, "empty non-final error");
}

// ============================================================================
// CDATA in attribute default (DTD) — validate_attribute_value
// ============================================================================

#[test]
fn cov_attlist_default_complex() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA \"hello world\">]><r/>",
        "ATTLIST default with spaces",
    );
}

// ============================================================================
// Whitespace-significant positions
// ============================================================================

#[test]
fn cov_space_before_eq() {
    compare_events(b"<r a =\"v\"/>", "space before = in attr");
}

#[test]
fn cov_space_after_eq() {
    compare_events(b"<r a= \"v\"/>", "space after = in attr");
}

#[test]
fn cov_space_around_eq() {
    compare_events(b"<r a = \"v\"/>", "space around = in attr");
}

// ============================================================================
// Various malformed XML for error path coverage
// ============================================================================

#[test]
fn cov_error_unclosed_start_tag() {
    compare(b"<r", "unclosed start tag");
}

#[test]
fn cov_error_unclosed_end_tag() {
    compare(b"<r></r", "unclosed end tag");
}

#[test]
fn cov_error_missing_gt() {
    compare(b"<r a=\"v\"", "missing > in start tag");
}

#[test]
fn cov_error_double_lt() {
    compare(b"<<r/>", "double <");
}

#[test]
fn cov_error_amp_in_tag() {
    compare(b"<r&/>", "& in tag name");
}

#[test]
fn cov_error_malformed_comment() {
    compare(b"<r><!- not a comment --></r>", "malformed comment start");
}

#[test]
fn cov_error_malformed_cdata() {
    compare(b"<r><![CDAT[x]]></r>", "malformed CDATA start");
}

#[test]
fn cov_error_pi_with_xml_target() {
    compare(b"<?XML bad?>", "PI with XML target (case variant)");
}
