//! Automated coverage comparison tests.
//!
//! Programmatically generates XML inputs targeting uncovered code paths,
//! runs them through both C and Rust parsers, and compares full SAX output.
//! Every test case verifies exact behavioral equivalence.

use expat_rust::xmlparse::{Parser, XmlError, XmlStatus};
use expat_sys::CParser;
use std::cell::RefCell;
use std::ffi::{c_char, c_int, c_void, CStr};

// ============================================================================
// Infrastructure: collect ALL SAX events from both parsers and compare
// ============================================================================

fn collect_rust_events_full(xml: &[u8]) -> (u32, u32, Vec<String>) {
    let events: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let mut parser = Parser::new(None).unwrap();
    let ev = &events as *const RefCell<Vec<String>>;

    parser.set_start_element_handler(Some(Box::new(move |name, attrs| unsafe {
        let mut s = format!("SE:{}", name);
        for (k, v) in attrs {
            s.push_str(&format!(" {}={}", k, v));
        }
        (*ev).borrow_mut().push(s);
    })));
    let ev2 = ev;
    parser.set_end_element_handler(Some(Box::new(move |name| unsafe {
        (*ev2).borrow_mut().push(format!("EE:{}", name));
    })));
    let ev3 = ev;
    parser.set_character_data_handler(Some(Box::new(move |data: &[u8]| unsafe {
        let text = std::str::from_utf8(data).unwrap_or("<bin>");
        (*ev3).borrow_mut().push(format!("CD:{}", text));
    })));
    let ev4 = ev;
    parser.set_processing_instruction_handler(Some(Box::new(move |target, data| unsafe {
        (*ev4).borrow_mut().push(format!("PI:{}:{}", target, data));
    })));
    let ev5 = ev;
    parser.set_comment_handler(Some(Box::new(move |text: &[u8]| unsafe {
        let t = std::str::from_utf8(text).unwrap_or("<bin>");
        (*ev5).borrow_mut().push(format!("CM:{}", t));
    })));
    let ev6 = ev;
    parser.set_start_cdata_section_handler(Some(Box::new(move || unsafe {
        (*ev6).borrow_mut().push("SCD".into());
    })));
    let ev7 = ev;
    parser.set_end_cdata_section_handler(Some(Box::new(move || unsafe {
        (*ev7).borrow_mut().push("ECD".into());
    })));
    let ev8 = ev;
    parser.set_start_doctype_decl_handler(Some(Box::new(
        move |name, sysid, pubid, has_internal| unsafe {
            (*ev8).borrow_mut().push(format!(
                "SDT:{}:{}:{}:{}",
                name,
                sysid.unwrap_or(""),
                pubid.unwrap_or(""),
                has_internal
            ));
        },
    )));
    let ev9 = ev;
    parser.set_end_doctype_decl_handler(Some(Box::new(move || unsafe {
        (*ev9).borrow_mut().push("EDT".into());
    })));

    let status = parser.parse(xml, true) as u32;
    let error = parser.error_code() as u32;
    (status, error, events.into_inner())
}

fn collect_c_events_full(xml: &[u8]) -> (u32, u32, Vec<String>) {
    let events: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let c_parser = CParser::new(None).unwrap();

    unsafe extern "C" fn se(ud: *mut c_void, name: *const c_char, atts: *mut *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let n = CStr::from_ptr(name).to_str().unwrap();
        let mut s = format!("SE:{}", n);
        let mut i = 0;
        loop {
            let key = *atts.add(i);
            if key.is_null() { break; }
            let val = *atts.add(i + 1);
            let k = CStr::from_ptr(key).to_str().unwrap();
            let v = CStr::from_ptr(val).to_str().unwrap();
            s.push_str(&format!(" {}={}", k, v));
            i += 2;
        }
        ev.borrow_mut().push(s);
    }
    unsafe extern "C" fn ee(ud: *mut c_void, name: *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        ev.borrow_mut().push(format!("EE:{}", CStr::from_ptr(name).to_str().unwrap()));
    }
    unsafe extern "C" fn cd(ud: *mut c_void, s: *const c_char, len: c_int) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let sl = std::slice::from_raw_parts(s as *const u8, len as usize);
        ev.borrow_mut().push(format!("CD:{}", std::str::from_utf8(sl).unwrap_or("<bin>")));
    }
    unsafe extern "C" fn pi(ud: *mut c_void, target: *const c_char, data: *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let t = CStr::from_ptr(target).to_str().unwrap();
        let d = if data.is_null() { "" } else { CStr::from_ptr(data).to_str().unwrap() };
        ev.borrow_mut().push(format!("PI:{}:{}", t, d));
    }
    unsafe extern "C" fn cm(ud: *mut c_void, text: *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        ev.borrow_mut().push(format!("CM:{}", CStr::from_ptr(text).to_str().unwrap()));
    }
    unsafe extern "C" fn scd(ud: *mut c_void) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        ev.borrow_mut().push("SCD".into());
    }
    unsafe extern "C" fn ecd(ud: *mut c_void) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        ev.borrow_mut().push("ECD".into());
    }
    unsafe extern "C" fn sdt(
        ud: *mut c_void, name: *const c_char, sysid: *const c_char,
        pubid: *const c_char, has_internal: c_int,
    ) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let n = CStr::from_ptr(name).to_str().unwrap();
        let s = if sysid.is_null() { "" } else { CStr::from_ptr(sysid).to_str().unwrap() };
        let p = if pubid.is_null() { "" } else { CStr::from_ptr(pubid).to_str().unwrap() };
        ev.borrow_mut().push(format!("SDT:{}:{}:{}:{}", n, s, p, has_internal != 0));
    }
    unsafe extern "C" fn edt(ud: *mut c_void) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        ev.borrow_mut().push("EDT".into());
    }

    unsafe {
        let ud = &events as *const RefCell<Vec<String>> as *mut c_void;
        expat_sys::XML_SetUserData(c_parser.raw_parser(), ud);
        expat_sys::XML_SetElementHandler(c_parser.raw_parser(), Some(se), Some(ee));
        expat_sys::XML_SetCharacterDataHandler(c_parser.raw_parser(), Some(cd));
        expat_sys::XML_SetProcessingInstructionHandler(c_parser.raw_parser(), Some(pi));
        expat_sys::XML_SetCommentHandler(c_parser.raw_parser(), Some(cm));
        expat_sys::XML_SetCdataSectionHandler(c_parser.raw_parser(), Some(scd), Some(ecd));
        expat_sys::XML_SetDoctypeDeclHandler(c_parser.raw_parser(), Some(sdt), Some(edt));
    }

    let (status, error) = c_parser.parse(xml, true);
    (status, error, events.into_inner())
}

/// Merge adjacent CD: events (SAX allows different chunking)
fn merge_cd(events: &[String]) -> Vec<String> {
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

/// Full comparison: status, error code, and all SAX events
fn assert_equivalent(xml: &[u8], desc: &str) {
    let (rs, re, r_ev) = collect_rust_events_full(xml);
    let (cs, ce, c_ev) = collect_c_events_full(xml);

    let r_merged = merge_cd(&r_ev);
    let c_merged = merge_cd(&c_ev);

    assert!(
        rs == cs && re == ce && r_merged == c_merged,
        "MISMATCH {desc}:\n  status: R={rs} C={cs}\n  error:  R={re} C={ce}\n  R events: {:?}\n  C events: {:?}\n  input: {:?}",
        r_merged, c_merged,
        std::str::from_utf8(xml).unwrap_or("<binary>")
    );
}

/// Status-only comparison (for error cases where handlers don't fire)
fn assert_status_equivalent(xml: &[u8], desc: &str) {
    let mut r = Parser::new(None).unwrap();
    let rs = r.parse(xml, true) as u32;
    let re = r.error_code() as u32;
    let c = CParser::new(None).unwrap();
    let (cs, ce) = c.parse(xml, true);
    assert!(
        rs == cs && re == ce,
        "MISMATCH {desc}: R s={rs} e={re}, C s={cs} e={ce}, input={:?}",
        std::str::from_utf8(xml).unwrap_or("<binary>")
    );
}

/// Incremental comparison: split at every byte boundary
fn assert_incremental_equivalent(xml: &[u8], desc: &str) {
    assert_status_equivalent(xml, desc);
    for split in 1..xml.len() {
        let mut r = Parser::new(None).unwrap();
        let r1 = r.parse(&xml[..split], false);
        let r_final = if r1 == XmlStatus::Ok {
            r.parse(&xml[split..], true)
        } else { r1 };
        let r_err = r.error_code();

        let c = CParser::new(None).unwrap();
        let (c1, _) = c.parse(&xml[..split], false);
        let (c_final, c_err) = if c1 == 1 {
            c.parse(&xml[split..], true)
        } else {
            (c1, c.parse(&xml[split..], true).1)
        };

        assert!(
            r_final as u32 == c_final && r_err as u32 == c_err,
            "INCR MISMATCH {desc} @{split}: R s={} e={}, C s={c_final} e={c_err}",
            r_final as u32, r_err as u32
        );
    }
}

// ============================================================================
// Test: Combinatorial XML feature matrix
// ============================================================================

/// Generate a variety of valid XML documents with different feature combinations
fn valid_xml_corpus() -> Vec<(Vec<u8>, &'static str)> {
    vec![
        // Basic elements
        (b"<r/>".to_vec(), "empty self-closing"),
        (b"<r></r>".to_vec(), "empty open-close"),
        (b"<r>text</r>".to_vec(), "text content"),
        (b"<r> </r>".to_vec(), "space content"),
        (b"<r>\n</r>".to_vec(), "newline content"),
        (b"<r>\r\n</r>".to_vec(), "crlf content"),
        (b"<r>\r</r>".to_vec(), "cr content"),
        (b"<r>\t</r>".to_vec(), "tab content"),
        // Nested elements
        (b"<r><a/></r>".to_vec(), "child element"),
        (b"<r><a><b><c/></b></a></r>".to_vec(), "nested 3 deep"),
        (b"<r><a/><b/><c/></r>".to_vec(), "siblings"),
        (b"<r>text<a/>more<b/>end</r>".to_vec(), "mixed content"),
        // Attributes
        (b"<r a=\"v\"/>".to_vec(), "single attr"),
        (b"<r a='v'/>".to_vec(), "single-quoted attr"),
        (b"<r a=\"1\" b=\"2\" c=\"3\"/>".to_vec(), "multiple attrs"),
        (b"<r a = \"v\" />".to_vec(), "spaced attr"),
        (b"<r a=\"&amp;&lt;&gt;&apos;&quot;\"/>".to_vec(), "entity refs in attr"),
        (b"<r a=\"&#65;&#x42;\"/>".to_vec(), "char refs in attr"),
        (b"<r a=\"hello\tworld\"/>".to_vec(), "tab in attr"),
        (b"<r a=\"hello\nworld\"/>".to_vec(), "newline in attr"),
        (b"<r a=\"hello\rworld\"/>".to_vec(), "cr in attr"),
        (b"<r a=\"hello\r\nworld\"/>".to_vec(), "crlf in attr"),
        // Processing instructions
        (b"<?pi data?><r/>".to_vec(), "PI before root"),
        (b"<r><?pi data?></r>".to_vec(), "PI in content"),
        (b"<r/><?pi data?>".to_vec(), "PI in epilog"),
        (b"<r><?pi?></r>".to_vec(), "PI no data"),
        (b"<?p1 d1?><?p2 d2?><r/><?p3 d3?>".to_vec(), "multiple PIs"),
        // Comments
        (b"<!-- c --><r/>".to_vec(), "comment before root"),
        (b"<r><!-- c --></r>".to_vec(), "comment in content"),
        (b"<r/><!-- c -->".to_vec(), "comment in epilog"),
        (b"<r><!----></r>".to_vec(), "empty comment"),
        (b"<r><!-- multi\nline\ncomment --></r>".to_vec(), "multiline comment"),
        // CDATA
        (b"<r><![CDATA[text]]></r>".to_vec(), "CDATA"),
        (b"<r><![CDATA[]]></r>".to_vec(), "empty CDATA"),
        (b"<r><![CDATA[<not>&xml;]]></r>".to_vec(), "CDATA with markup"),
        (b"<r><![CDATA[a]b]]></r>".to_vec(), "CDATA with ]"),
        (b"<r><![CDATA[a]]b]]></r>".to_vec(), "CDATA with ]]"),
        (b"<r><![CDATA[\r\n\r]]></r>".to_vec(), "CDATA with crlf"),
        // Entities
        (b"<r>&amp;&lt;&gt;&apos;&quot;</r>".to_vec(), "predefined entities"),
        (b"<r>&#65;&#66;</r>".to_vec(), "decimal char refs"),
        (b"<r>&#x41;&#x42;</r>".to_vec(), "hex char refs"),
        (b"<r>&#x20AC;</r>".to_vec(), "euro sign char ref"),
        // XML declaration
        (b"<?xml version='1.0'?><r/>".to_vec(), "xml decl"),
        (b"<?xml version='1.0' encoding='UTF-8'?><r/>".to_vec(), "xml decl encoding"),
        (b"<?xml version='1.0' standalone='yes'?><r/>".to_vec(), "xml decl standalone=yes"),
        (b"<?xml version='1.0' standalone='no'?><r/>".to_vec(), "xml decl standalone=no"),
        (b"<?xml version='1.0' encoding='UTF-8' standalone='yes'?><r/>".to_vec(), "xml decl full"),
        // DOCTYPE
        (b"<!DOCTYPE r><r/>".to_vec(), "simple DOCTYPE"),
        (b"<!DOCTYPE r SYSTEM \"t.dtd\"><r/>".to_vec(), "DOCTYPE SYSTEM"),
        (b"<!DOCTYPE r PUBLIC \"-//T//EN\" \"t.dtd\"><r/>".to_vec(), "DOCTYPE PUBLIC"),
        (b"<!DOCTYPE r [<!ELEMENT r (#PCDATA)>]><r/>".to_vec(), "DOCTYPE ELEMENT PCDATA"),
        (b"<!DOCTYPE r [<!ELEMENT r EMPTY>]><r/>".to_vec(), "DOCTYPE ELEMENT EMPTY"),
        (b"<!DOCTYPE r [<!ELEMENT r ANY>]><r/>".to_vec(), "DOCTYPE ELEMENT ANY"),
        (b"<!DOCTYPE r [<!ENTITY e 'v'>]><r>&e;</r>".to_vec(), "internal entity"),
        (b"<!DOCTYPE r [<!ENTITY a '1'><!ENTITY b '2'>]><r>&a;&b;</r>".to_vec(), "multiple entities"),
        (b"<!DOCTYPE r [<!ENTITY e SYSTEM \"f.xml\">]><r/>".to_vec(), "external entity SYSTEM"),
        (b"<!DOCTYPE r [<!ENTITY e PUBLIC \"-//T//EN\" \"f.xml\">]><r/>".to_vec(), "external entity PUBLIC"),
        (b"<!DOCTYPE r [<!NOTATION n SYSTEM \"x\">]><r/>".to_vec(), "NOTATION SYSTEM"),
        (b"<!DOCTYPE r [<!NOTATION n PUBLIC \"-//T//EN\">]><r/>".to_vec(), "NOTATION PUBLIC"),
        (b"<!DOCTYPE r [<!NOTATION n PUBLIC \"-//T//EN\" \"x\">]><r/>".to_vec(), "NOTATION PUB+SYS"),
        (b"<!DOCTYPE r [<!ATTLIST r a CDATA #IMPLIED>]><r a=\"v\"/>".to_vec(), "ATTLIST IMPLIED"),
        (b"<!DOCTYPE r [<!ATTLIST r a CDATA #REQUIRED>]><r a=\"v\"/>".to_vec(), "ATTLIST REQUIRED"),
        (b"<!DOCTYPE r [<!ATTLIST r a CDATA \"dv\">]><r/>".to_vec(), "ATTLIST default"),
        (b"<!DOCTYPE r [<!ATTLIST r a CDATA #FIXED \"fv\">]><r/>".to_vec(), "ATTLIST FIXED"),
        (b"<!DOCTYPE r [<!ATTLIST r a (x|y|z) #IMPLIED>]><r a=\"x\"/>".to_vec(), "ATTLIST enum"),
        (b"<!DOCTYPE r [<!ATTLIST r a CDATA #IMPLIED b CDATA \"d\">]><r a=\"1\"/>".to_vec(), "ATTLIST mixed"),
        // Content model quantifiers
        (b"<!DOCTYPE r [<!ELEMENT r (a)*><!ELEMENT a EMPTY>]><r><a/><a/></r>".to_vec(), "ELEMENT (a)*"),
        (b"<!DOCTYPE r [<!ELEMENT r (a)?><!ELEMENT a EMPTY>]><r/>".to_vec(), "ELEMENT (a)?"),
        (b"<!DOCTYPE r [<!ELEMENT r (a)+><!ELEMENT a EMPTY>]><r><a/></r>".to_vec(), "ELEMENT (a)+"),
        (b"<!DOCTYPE r [<!ELEMENT r (a,b)><!ELEMENT a EMPTY><!ELEMENT b EMPTY>]><r><a/><b/></r>".to_vec(), "ELEMENT seq"),
        (b"<!DOCTYPE r [<!ELEMENT r (a|b)><!ELEMENT a EMPTY><!ELEMENT b EMPTY>]><r><a/></r>".to_vec(), "ELEMENT choice"),
        (b"<!DOCTYPE r [<!ELEMENT r (#PCDATA|a)*><!ELEMENT a EMPTY>]><r>text<a/>more</r>".to_vec(), "ELEMENT mixed"),
        // DTD misc
        (b"<!DOCTYPE r [<!-- dtd comment -->]><r/>".to_vec(), "DTD comment"),
        (b"<!DOCTYPE r [<?dtd-pi data?>]><r/>".to_vec(), "DTD PI"),
        (b"<!DOCTYPE r [\n  \n]><r/>".to_vec(), "DTD whitespace"),
        // Unparsed entity with NDATA
        (b"<!DOCTYPE r [<!NOTATION n SYSTEM \"x\"><!ENTITY e SYSTEM \"f\" NDATA n>]><r/>".to_vec(), "unparsed entity"),
        // UTF-8 multi-byte
        ("<!DOCTYPE r [<!ENTITY e 'ñ'>]><r>&e;</r>".as_bytes().to_vec(), "entity 2-byte UTF-8"),
        ("<r>日本語</r>".as_bytes().to_vec(), "CJK content"),
        ("<r>😀</r>".as_bytes().to_vec(), "emoji content"),
        ("<r a=\"日本語\"/>".as_bytes().to_vec(), "CJK in attr"),
        ("<r><!-- 日本語 --></r>".as_bytes().to_vec(), "CJK in comment"),
        // Encoding declarations
        (b"<?xml version='1.0' encoding='ISO-8859-1'?><r/>".to_vec(), "latin1 encoding"),
        (b"<?xml version='1.0' encoding='US-ASCII'?><r/>".to_vec(), "ascii encoding"),
        // BOM
        (b"\xEF\xBB\xBF<r/>".to_vec(), "UTF-8 BOM"),
        (b"\xEF\xBB\xBF<?xml version='1.0'?><r/>".to_vec(), "UTF-8 BOM + xmldecl"),
        // Whitespace around root
        (b"  \n  <r/>  \n  ".to_vec(), "whitespace around root"),
        (b"<r/>  \n\t  ".to_vec(), "whitespace epilog"),
        // Complex documents
        (br#"<?xml version="1.0"?>
<!DOCTYPE doc [
  <!ELEMENT doc (#PCDATA|p)*>
  <!ELEMENT p (#PCDATA)>
  <!ENTITY copy "&#169;">
  <!ATTLIST doc v CDATA #IMPLIED>
]>
<doc v="1">
  <!-- comment -->
  <p>Hello &amp; &copy;</p>
  <?app info?>
  <![CDATA[raw <data>]]>
</doc>"#.to_vec(), "complex document"),
    ]
}

/// Generate a variety of invalid/error XML inputs
fn error_xml_corpus() -> Vec<(Vec<u8>, &'static str)> {
    vec![
        (b"".to_vec(), "empty"),
        (b"   ".to_vec(), "whitespace only"),
        (b"hello".to_vec(), "bare text"),
        (b"<".to_vec(), "lone <"),
        (b"<r".to_vec(), "unclosed start tag"),
        (b"<r>".to_vec(), "unclosed element"),
        (b"<r>hello".to_vec(), "unclosed with content"),
        (b"<r></s>".to_vec(), "tag mismatch"),
        (b"<r/><s/>".to_vec(), "double root"),
        (b"<r/>text".to_vec(), "text after root"),
        (b"<r/><s/>".to_vec(), "element after root"),
        (b"<123/>".to_vec(), "digit tag name"),
        (b"<r>&</r>".to_vec(), "bare &"),
        (b"<r>&entity</r>".to_vec(), "entity no semicolon"),
        (b"<r>&undefined;</r>".to_vec(), "undefined entity"),
        (b"<r a=\"1\" a=\"2\"/>".to_vec(), "duplicate attr"),
        (b"<r a=\"<\"/>".to_vec(), "< in attr"),
        (b"<r a/>".to_vec(), "attr no value"),
        (b"<r a>text</r>".to_vec(), "attr no value with >"),
        (b"</r>".to_vec(), "end tag no start"),
        (b"<r></r extra>".to_vec(), "close tag extra"),
        (b"<r>&#0;</r>".to_vec(), "null char ref"),
        (b"<r>&#xD800;</r>".to_vec(), "surrogate char ref"),
        (b"<r>&#xFFFFFF;</r>".to_vec(), "out-of-range char ref"),
        (b"<r><![CDATA[unclosed".to_vec(), "unclosed CDATA"),
        (b"<r><!-- unclosed".to_vec(), "unclosed comment"),
        (b"<r><?pi unclosed".to_vec(), "unclosed PI"),
        (b"<r>]]></r>".to_vec(), "]]> in content"),
        (b"<?XML bad?>".to_vec(), "reserved PI target"),
        (b"<?xMl bad?>".to_vec(), "reserved PI target mixed"),
        (b"<!DOCTYPE r PUBLIC \"bad{char\" \"t.dtd\"><r/>".to_vec(), "bad publicid"),
        (b"<r>\r".to_vec(), "trailing CR"),
        (b"<r>\xC3".to_vec(), "partial UTF-8 2-byte"),
        (b"<r>\xE4\xB8".to_vec(), "partial UTF-8 3-byte"),
        // Entity recursion
        (b"<!DOCTYPE r [<!ENTITY a '&b;'><!ENTITY b '&a;'>]><r>&a;</r>".to_vec(), "recursive entity"),
    ]
}

/// UTF-16 test inputs
fn utf16_corpus() -> Vec<(Vec<u8>, &'static str)> {
    fn utf16le(s: &str) -> Vec<u8> {
        let mut out = vec![0xFF, 0xFE];
        for c in s.encode_utf16() {
            out.push(c as u8);
            out.push((c >> 8) as u8);
        }
        out
    }
    fn utf16be(s: &str) -> Vec<u8> {
        let mut out = vec![0xFE, 0xFF];
        for c in s.encode_utf16() {
            out.push((c >> 8) as u8);
            out.push(c as u8);
        }
        out
    }
    vec![
        (utf16le("<r/>"), "UTF-16LE simple"),
        (utf16be("<r/>"), "UTF-16BE simple"),
        (utf16le("<r>hello</r>"), "UTF-16LE content"),
        (utf16be("<r>hello</r>"), "UTF-16BE content"),
        (utf16le("<r a=\"v\"/>"), "UTF-16LE attr"),
        (utf16be("<r a=\"v\"/>"), "UTF-16BE attr"),
        (utf16le("<r><a/></r>"), "UTF-16LE nested"),
        (utf16le("<!-- c --><r/>"), "UTF-16LE comment"),
        (utf16le("<?pi d?><r/>"), "UTF-16LE PI"),
    ]
}

// ============================================================================
// Tests: valid XML corpus — full SAX comparison
// ============================================================================

#[test]
fn auto_valid_xml_full_sax_comparison() {
    for (xml, desc) in valid_xml_corpus() {
        assert_equivalent(&xml, desc);
    }
}

// ============================================================================
// Tests: error XML corpus — status/error comparison
// ============================================================================

#[test]
fn auto_error_xml_status_comparison() {
    for (xml, desc) in error_xml_corpus() {
        assert_status_equivalent(&xml, desc);
    }
}

// ============================================================================
// Tests: UTF-16 corpus — status comparison
// ============================================================================

#[test]
fn auto_utf16_status_comparison() {
    for (xml, desc) in utf16_corpus() {
        assert_status_equivalent(&xml, desc);
    }
}

// ============================================================================
// Tests: incremental parsing of valid corpus
// ============================================================================

#[test]
fn auto_incremental_valid() {
    // Test incremental parsing for shorter inputs (byte-by-byte split is expensive)
    for (xml, desc) in valid_xml_corpus() {
        if xml.len() <= 100 {
            assert_incremental_equivalent(&xml, desc);
        }
    }
}

// ============================================================================
// Tests: incremental parsing of error corpus
// ============================================================================

#[test]
fn auto_incremental_errors() {
    for (xml, desc) in error_xml_corpus() {
        if xml.len() <= 50 {
            assert_incremental_equivalent(&xml, desc);
        }
    }
}

// ============================================================================
// Tests: incremental UTF-16
// ============================================================================

#[test]
fn auto_incremental_utf16() {
    // Only test even split points for UTF-16 to avoid splitting mid-code-unit
    for (xml, desc) in utf16_corpus() {
        if xml.len() <= 30 {
            assert_status_equivalent(&xml, desc);
            for split in (2..xml.len()).step_by(2) {
                let mut r = Parser::new(None).unwrap();
                let r1 = r.parse(&xml[..split], false);
                let r_final = if r1 == XmlStatus::Ok {
                    r.parse(&xml[split..], true)
                } else { r1 };
                let r_err = r.error_code();

                let c = CParser::new(None).unwrap();
                let (c1, _) = c.parse(&xml[..split], false);
                let (c_final, c_err) = if c1 == 1 {
                    c.parse(&xml[split..], true)
                } else {
                    (c1, c.parse(&xml[split..], true).1)
                };

                assert!(
                    r_final as u32 == c_final && r_err as u32 == c_err,
                    "INCR MISMATCH {desc} @{split}: R s={} e={}, C s={c_final} e={c_err}",
                    r_final as u32, r_err as u32
                );
            }
        }
    }
}

// ============================================================================
// Tests: parser reset + reparse
// ============================================================================

#[test]
fn auto_reset_reparse() {
    let inputs: &[&[u8]] = &[
        b"<a/>",
        b"<a>text</a>",
        b"<!DOCTYPE r [<!ENTITY e 'v'>]><r>&e;</r>",
        b"<?xml version='1.0'?><r/>",
    ];
    for (i, xml1) in inputs.iter().enumerate() {
        for (j, xml2) in inputs.iter().enumerate() {
            let mut r = Parser::new(None).unwrap();
            let _ = r.parse(xml1, true);
            r.reset(None);
            let rs = r.parse(xml2, true) as u32;
            let re = r.error_code() as u32;

            let c = CParser::new(None).unwrap();
            let (cs, ce) = c.parse(xml2, true);

            assert_eq!(rs, cs, "reset reparse {i}→{j} status");
            assert_eq!(re, ce, "reset reparse {i}→{j} error");
        }
    }
}

// ============================================================================
// Tests: generated element/attribute combinations
// ============================================================================

#[test]
fn auto_element_attribute_combos() {
    // Generate elements with various attribute count and value types
    let attr_values = [
        ("plain", "hello"),
        ("empty", ""),
        ("space", "a b"),
        ("amp", "&amp;"),
        ("lt", "&lt;"),
        ("charref", "&#65;"),
        ("hexref", "&#x42;"),
        ("multi", "&amp;&lt;&#65;"),
    ];

    for (name, val) in &attr_values {
        let xml = format!("<r {}=\"{}\"/>", name, val);
        assert_equivalent(xml.as_bytes(), &format!("attr combo {}", name));
    }
}

// ============================================================================
// Tests: generated DTD combinations
// ============================================================================

#[test]
fn auto_dtd_element_models() {
    let models = [
        ("EMPTY", "<r/>"),
        ("ANY", "<r/>"),
        ("(#PCDATA)", "<r>text</r>"),
        ("(#PCDATA)*", "<r>text</r>"),
        ("(a,b)", "<r><a/><b/></r>"),
        ("(a|b)", "<r><a/></r>"),
        ("(a*,b?)", "<r><b/></r>"),
        ("(a+)", "<r><a/></r>"),
        ("((a|b),c)", "<r><a/><c/></r>"),
    ];

    for (model, body) in &models {
        let xml = format!(
            "<!DOCTYPE r [<!ELEMENT r {}><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]>{}",
            model, body
        );
        assert_status_equivalent(xml.as_bytes(), &format!("element model {}", model));
    }
}

// ============================================================================
// Tests: generated entity expansion cases
// ============================================================================

#[test]
fn auto_entity_expansion() {
    let entity_values = [
        ("simple", "hello"),
        ("spaces", "hello world"),
        ("charref", "&#65;"),
        ("amp", "&amp;"),
        ("nested_entity", "a&amp;b"),
        ("unicode_2byte", "caf\u{00e9}"),
        ("unicode_3byte", "\u{20AC}"),
    ];

    for (name, value) in &entity_values {
        let xml = format!(
            "<!DOCTYPE r [<!ENTITY e '{}'>]><r>&e;</r>",
            value
        );
        assert_equivalent(xml.as_bytes(), &format!("entity {}", name));
    }
}

// ============================================================================
// Tests: whitespace and newline normalization edge cases
// ============================================================================

#[test]
fn auto_whitespace_combinations() {
    let contents = [
        "a\nb", "a\rb", "a\r\nb", "a\r\n\rb",
        "\n", "\r", "\r\n", "\t",
        "  a  ", "\n\n\n", "\r\r\r",
        "a\r\nb\rc\nd",
    ];

    for content in &contents {
        let xml = format!("<r>{}</r>", content);
        assert_equivalent(xml.as_bytes(), &format!("ws {:?}", content));
    }
}

// ============================================================================
// Tests: CDATA content combinations
// ============================================================================

#[test]
fn auto_cdata_combinations() {
    let cdata_contents = [
        "", "text", "<not>xml</not>", "&amp;", "a]b", "a]]b",
        "line1\nline2", "line1\rline2", "line1\r\nline2",
        "   ", "\t\t\t",
    ];

    for content in &cdata_contents {
        let xml = format!("<r><![CDATA[{}]]></r>", content);
        assert_equivalent(xml.as_bytes(), &format!("cdata {:?}", content));
    }
}

// ============================================================================
// Tests: comment content combinations
// ============================================================================

#[test]
fn auto_comment_combinations() {
    let comment_contents = [
        " ", "simple", " multi\nline ", "  spaced  ",
        " a-b-c ", " 1234 ",
    ];

    for content in &comment_contents {
        let xml = format!("<r><!--{}--></r>", content);
        assert_equivalent(xml.as_bytes(), &format!("comment {:?}", content));
    }
}

// ============================================================================
// Tests: PI content combinations
// ============================================================================

#[test]
fn auto_pi_combinations() {
    let pi_cases = [
        ("target", "data"),
        ("target", ""),
        ("t", "multi word data"),
        ("my-pi", "key=value"),
    ];

    for (target, data) in &pi_cases {
        let xml = if data.is_empty() {
            format!("<r><?{}?></r>", target)
        } else {
            format!("<r><?{} {}?></r>", target, data)
        };
        assert_equivalent(xml.as_bytes(), &format!("pi {}:{}", target, data));
    }
}

// ============================================================================
// Tests: deep nesting stress test
// ============================================================================

#[test]
fn auto_nesting_depth() {
    for depth in [1, 5, 10, 25, 50, 100] {
        let mut xml = String::new();
        for i in 0..depth {
            xml.push_str(&format!("<e{}>", i));
        }
        xml.push_str("text");
        for i in (0..depth).rev() {
            xml.push_str(&format!("</e{}>", i));
        }
        assert_status_equivalent(xml.as_bytes(), &format!("depth {}", depth));
    }
}

// ============================================================================
// Tests: many attributes stress test
// ============================================================================

#[test]
fn auto_many_attrs() {
    for count in [1, 5, 10, 25, 50] {
        let mut xml = String::from("<r");
        for i in 0..count {
            xml.push_str(&format!(" a{}=\"v{}\"", i, i));
        }
        xml.push_str("/>");
        assert_status_equivalent(xml.as_bytes(), &format!("{} attrs", count));
    }
}

// ============================================================================
// Tests: long text content stress test
// ============================================================================

#[test]
fn auto_long_content() {
    for len in [100, 1000, 10000] {
        let content: String = "abcdefghij".repeat(len / 10);
        let xml = format!("<r>{}</r>", content);
        assert_status_equivalent(xml.as_bytes(), &format!("{}B content", len));
    }
}

// ============================================================================
// Tests: epilog combinations
// ============================================================================

#[test]
fn auto_epilog_combinations() {
    let epilogs = [
        " ", "\n", "\t", "  \n  ",
        "<!-- c -->", "<?pi d?>",
        "<!-- c1 -->\n<!-- c2 -->",
        "<?p1 d1?>\n<?p2 d2?>",
        "\n<!-- c -->\n<?pi d?>\n",
    ];

    for epilog in &epilogs {
        let xml = format!("<r/>{}", epilog);
        assert_equivalent(xml.as_bytes(), &format!("epilog {:?}", epilog));
    }
}

// ============================================================================
// Tests: prolog combinations
// ============================================================================

#[test]
fn auto_prolog_combinations() {
    let prologs = [
        "<!-- c -->",
        "<?pi d?>",
        "\n  \n",
        "<!-- c1 -->\n<!-- c2 -->",
        "<?p1 d1?>\n<?p2 d2?>",
        "<!-- c -->\n<?pi d?>\n",
    ];

    for prolog in &prologs {
        let xml = format!("{}<r/>", prolog);
        assert_equivalent(xml.as_bytes(), &format!("prolog {:?}", prolog));
    }
}

// ============================================================================
// Tests: error_string coverage
// ============================================================================

#[test]
fn auto_error_string_coverage() {
    use expat_rust::xmlparse::error_string;
    // Exercise every error code through error_string
    let errors = [
        XmlError::None, XmlError::NoMemory, XmlError::Syntax,
        XmlError::NoElements, XmlError::InvalidToken, XmlError::UnclosedToken,
        XmlError::PartialChar, XmlError::TagMismatch, XmlError::DuplicateAttribute,
        XmlError::JunkAfterDocElement, XmlError::ParamEntityRef,
        XmlError::UndefinedEntity, XmlError::RecursiveEntityRef,
        XmlError::AsyncEntity, XmlError::BadCharRef, XmlError::BinaryEntityRef,
        XmlError::AttributeExternalEntityRef, XmlError::MisplacedXmlPi,
        XmlError::UnknownEncoding, XmlError::IncorrectEncoding,
        XmlError::UnclosedCdataSection, XmlError::ExternalEntityHandling,
        XmlError::NotStandalone, XmlError::UnexpectedState,
        XmlError::EntityDeclaredInPe, XmlError::FeatureRequiresXmlDtd,
        XmlError::CantChangeFeatureOnceParsing, XmlError::UnboundPrefix,
        XmlError::UndeclaringPrefix, XmlError::IncompletePe,
        XmlError::XmlDecl, XmlError::TextDecl, XmlError::Publicid,
        XmlError::Suspended, XmlError::NotSuspended, XmlError::Aborted,
        XmlError::Finished, XmlError::SuspendPe,
        XmlError::ReservedPrefixXml, XmlError::ReservedPrefixXmlns,
        XmlError::ReservedNamespaceUri, XmlError::InvalidArgument,
        XmlError::NoBuffer, XmlError::AmplificationLimitBreach,
        XmlError::NotStarted,
    ];
    for e in errors {
        let s = error_string(e);
        assert!(!s.is_empty());
    }
}

// ============================================================================
// Tests: expat_version / feature list coverage
// ============================================================================

#[test]
fn auto_version_info() {
    let v = expat_rust::xmlparse::expat_version_info();
    assert_eq!(v.major, 2);
    assert_eq!(v.minor, 7);
    assert_eq!(v.micro, 5);

    let ver = expat_rust::xmlparse::expat_version();
    assert!(ver.contains("2.7.5"));

    let features = expat_rust::xmlparse::get_feature_list();
    assert!(!features.is_empty());
}
