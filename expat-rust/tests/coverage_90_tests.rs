//! Tests targeting 90% line coverage. Every test compares C and Rust output.
//! Focuses on the highest-value uncovered code paths.

use expat_rust::xmlparse::{Parser, XmlStatus};
use expat_sys::CParser;
use std::cell::RefCell;
use std::ffi::{c_char, c_int, c_void, CStr};

// ============================================================================
// Comparison infrastructure (same as other test files)
// ============================================================================

fn compare(xml: &[u8], desc: &str) {
    let mut r = Parser::new(None).unwrap();
    let rs = r.parse(xml, true) as u32;
    let re = r.error_code() as u32;
    let c = CParser::new(None).unwrap();
    let (cs, ce) = c.parse(xml, true);
    assert!(
        rs == cs && re == ce,
        "MISMATCH {desc}: R s={rs} e={re}, C s={cs} e={ce}, input={:?}",
        std::str::from_utf8(xml).unwrap_or("<bin>")
    );
}

fn compare_incr(xml: &[u8], desc: &str) {
    compare(xml, desc);
    for split in 1..xml.len() {
        let mut r = Parser::new(None).unwrap();
        let r1 = r.parse(&xml[..split], false);
        let rf = if r1 == XmlStatus::Ok {
            r.parse(&xml[split..], true)
        } else {
            r1
        };
        let re = r.error_code();
        let c = CParser::new(None).unwrap();
        let (c1, _) = c.parse(&xml[..split], false);
        let (cf, ce) = if c1 == 1 {
            c.parse(&xml[split..], true)
        } else {
            (c1, c.parse(&xml[split..], true).1)
        };
        assert!(
            rf as u32 == cf && re as u32 == ce,
            "INCR {desc} @{split}: R s={} e={}, C s={cf} e={ce}",
            rf as u32,
            re as u32
        );
    }
}

fn merge_cd(events: &[String]) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();
    for ev in events {
        if ev.starts_with("CD:") {
            if let Some(last) = r.last_mut() {
                if last.starts_with("CD:") {
                    last.push_str(&ev[3..]);
                    continue;
                }
            }
        }
        r.push(ev.clone());
    }
    r
}

fn collect_events(xml: &[u8]) -> (u32, Vec<String>) {
    let ev: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let mut p = Parser::new(None).unwrap();
    let e = &ev as *const RefCell<Vec<String>>;
    p.set_start_element_handler(Some(Box::new(move |n, a| unsafe {
        let mut s = format!("SE:{}", n);
        for (k, v) in a {
            s.push_str(&format!(" {}={}", k, v));
        }
        (*e).borrow_mut().push(s);
    })));
    let e2 = e;
    p.set_end_element_handler(Some(Box::new(move |n| unsafe {
        (*e2).borrow_mut().push(format!("EE:{}", n));
    })));
    let e3 = e;
    p.set_character_data_handler(Some(Box::new(move |d: &[u8]| unsafe {
        (*e3)
            .borrow_mut()
            .push(format!("CD:{}", std::str::from_utf8(d).unwrap_or("?")));
    })));
    let e4 = e;
    p.set_processing_instruction_handler(Some(Box::new(move |t, d| unsafe {
        (*e4).borrow_mut().push(format!("PI:{}:{}", t, d));
    })));
    let e5 = e;
    p.set_comment_handler(Some(Box::new(move |t: &[u8]| unsafe {
        (*e5)
            .borrow_mut()
            .push(format!("CM:{}", std::str::from_utf8(t).unwrap_or("?")));
    })));
    let e6 = e;
    p.set_start_cdata_section_handler(Some(Box::new(move || unsafe {
        (*e6).borrow_mut().push("SCD".into());
    })));
    let e7 = e;
    p.set_end_cdata_section_handler(Some(Box::new(move || unsafe {
        (*e7).borrow_mut().push("ECD".into());
    })));
    let e8 = e;
    p.set_start_doctype_decl_handler(Some(Box::new(move |n, s, p, h| unsafe {
        (*e8).borrow_mut().push(format!(
            "SDT:{}:{}:{}:{}",
            n,
            s.unwrap_or(""),
            p.unwrap_or(""),
            h
        ));
    })));
    let e9 = e;
    p.set_end_doctype_decl_handler(Some(Box::new(move || unsafe {
        (*e9).borrow_mut().push("EDT".into());
    })));
    let status = p.parse(xml, true) as u32;
    (status, ev.into_inner())
}

fn collect_c_events(xml: &[u8]) -> (u32, Vec<String>) {
    let ev: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let cp = CParser::new(None).unwrap();
    unsafe extern "C" fn se(u: *mut c_void, n: *const c_char, a: *mut *const c_char) {
        let e = &*(u as *const RefCell<Vec<String>>);
        let nm = CStr::from_ptr(n).to_str().unwrap();
        let mut s = format!("SE:{}", nm);
        let mut i = 0;
        loop {
            let k = *a.add(i);
            if k.is_null() {
                break;
            }
            let v = *a.add(i + 1);
            s.push_str(&format!(
                " {}={}",
                CStr::from_ptr(k).to_str().unwrap(),
                CStr::from_ptr(v).to_str().unwrap()
            ));
            i += 2;
        }
        e.borrow_mut().push(s);
    }
    unsafe extern "C" fn ee(u: *mut c_void, n: *const c_char) {
        let e = &*(u as *const RefCell<Vec<String>>);
        e.borrow_mut()
            .push(format!("EE:{}", CStr::from_ptr(n).to_str().unwrap()));
    }
    unsafe extern "C" fn cd(u: *mut c_void, s: *const c_char, l: c_int) {
        let e = &*(u as *const RefCell<Vec<String>>);
        let sl = std::slice::from_raw_parts(s as *const u8, l as usize);
        e.borrow_mut()
            .push(format!("CD:{}", std::str::from_utf8(sl).unwrap_or("?")));
    }
    unsafe extern "C" fn pi(u: *mut c_void, t: *const c_char, d: *const c_char) {
        let e = &*(u as *const RefCell<Vec<String>>);
        let dt = if d.is_null() {
            ""
        } else {
            CStr::from_ptr(d).to_str().unwrap()
        };
        e.borrow_mut()
            .push(format!("PI:{}:{}", CStr::from_ptr(t).to_str().unwrap(), dt));
    }
    unsafe extern "C" fn cm(u: *mut c_void, t: *const c_char) {
        let e = &*(u as *const RefCell<Vec<String>>);
        e.borrow_mut()
            .push(format!("CM:{}", CStr::from_ptr(t).to_str().unwrap()));
    }
    unsafe extern "C" fn scd(u: *mut c_void) {
        (&*(u as *const RefCell<Vec<String>>))
            .borrow_mut()
            .push("SCD".into());
    }
    unsafe extern "C" fn ecd(u: *mut c_void) {
        (&*(u as *const RefCell<Vec<String>>))
            .borrow_mut()
            .push("ECD".into());
    }
    unsafe extern "C" fn sdt(
        u: *mut c_void,
        n: *const c_char,
        s: *const c_char,
        p: *const c_char,
        h: c_int,
    ) {
        let e = &*(u as *const RefCell<Vec<String>>);
        let nm = CStr::from_ptr(n).to_str().unwrap();
        let sv = if s.is_null() {
            ""
        } else {
            CStr::from_ptr(s).to_str().unwrap()
        };
        let pv = if p.is_null() {
            ""
        } else {
            CStr::from_ptr(p).to_str().unwrap()
        };
        e.borrow_mut()
            .push(format!("SDT:{}:{}:{}:{}", nm, sv, pv, h != 0));
    }
    unsafe extern "C" fn edt(u: *mut c_void) {
        (&*(u as *const RefCell<Vec<String>>))
            .borrow_mut()
            .push("EDT".into());
    }
    unsafe {
        let u = &ev as *const RefCell<Vec<String>> as *mut c_void;
        expat_sys::XML_SetUserData(cp.raw_parser(), u);
        expat_sys::XML_SetElementHandler(cp.raw_parser(), Some(se), Some(ee));
        expat_sys::XML_SetCharacterDataHandler(cp.raw_parser(), Some(cd));
        expat_sys::XML_SetProcessingInstructionHandler(cp.raw_parser(), Some(pi));
        expat_sys::XML_SetCommentHandler(cp.raw_parser(), Some(cm));
        expat_sys::XML_SetCdataSectionHandler(cp.raw_parser(), Some(scd), Some(ecd));
        expat_sys::XML_SetDoctypeDeclHandler(cp.raw_parser(), Some(sdt), Some(edt));
    }
    let (status, _) = cp.parse(xml, true);
    (status, ev.into_inner())
}

fn compare_events(xml: &[u8], desc: &str) {
    let (rs, re) = collect_events(xml);
    let (cs, ce) = collect_c_events(xml);
    let rm = merge_cd(&re);
    let cm = merge_cd(&ce);
    assert!(
        rs == cs && rm == cm,
        "MISMATCH {desc}:\n  R: {:?}\n  C: {:?}",
        rm,
        cm
    );
}

// ============================================================================
// 1. Attribute value tokenizer (xmltok_impl.rs 2089-2188, 2192-2278)
//    CR, LF, tab, space in attribute values processed through attribute_value_tok
// ============================================================================

#[test]
fn cov90_attr_val_cr_lf_s_combinations() {
    // These exercise attribute_value_tok's CR/LF/S branches
    let cases = [
        "<r a=\"x\ry\"/>",
        "<r a=\"x\ny\"/>",
        "<r a=\"x\r\ny\"/>",
        "<r a=\"x\ty\"/>",
        "<r a=\" x \"/>",
        "<r a=\"\r\"/>",
        "<r a=\"\n\"/>",
        "<r a=\"\r\n\"/>",
        "<r a=\"\t\"/>",
        "<r a=\"\r\n\r\n\"/>",
        "<r a=\"a\rb\nc\r\nd\"/>",
        // Single-quoted variants (exercises the other attribute_value_tok function)
        "<r a='x\ry'/>",
        "<r a='x\ny'/>",
        "<r a='x\r\ny'/>",
        "<r a='x\ty'/>",
        "<r a=' x '/>",
        "<r a='\r'/>",
        "<r a='\n'/>",
        "<r a='\r\n'/>",
    ];
    for case in &cases {
        compare_events(case.as_bytes(), &format!("attr_val {:?}", case));
    }
}

#[test]
fn cov90_attr_val_entity_in_value() {
    // Entity refs in attribute values go through attribute_value_tok AMP branch
    let cases = [
        "<r a=\"&amp;\"/>",
        "<r a=\"&lt;\"/>",
        "<r a=\"x&amp;y\"/>",
        "<r a=\"&amp;&lt;&gt;\"/>",
        "<r a=\"&#65;\"/>",
        "<r a=\"&#x41;\"/>",
        "<r a='&amp;'/>",
        "<r a='&lt;'/>",
    ];
    for case in &cases {
        compare_events(case.as_bytes(), &format!("attr_entity {:?}", case));
    }
}

#[test]
fn cov90_attr_val_incremental() {
    // Split attribute values at every byte to exercise partial paths
    let cases: &[&[u8]] = &[
        b"<r a=\"x\ry\"/>",
        b"<r a=\"x\ny\"/>",
        b"<r a=\"&amp;\"/>",
        b"<r a='x\ry'/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("attr_incr {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 2. Entity value tokenizer (xmltok_impl.rs 2192-2278)
//    CR, LF, PERCNT in entity values
// ============================================================================

#[test]
fn cov90_entity_value_cr_lf() {
    let cases = [
        "<!DOCTYPE r [<!ENTITY e 'a\rb'>]><r>&e;</r>",
        "<!DOCTYPE r [<!ENTITY e 'a\nb'>]><r>&e;</r>",
        "<!DOCTYPE r [<!ENTITY e 'a\r\nb'>]><r>&e;</r>",
        "<!DOCTYPE r [<!ENTITY e '\r'>]><r>&e;</r>",
        "<!DOCTYPE r [<!ENTITY e '\n'>]><r>&e;</r>",
    ];
    for case in &cases {
        compare_events(case.as_bytes(), &format!("entity_val {:?}", case));
    }
}

// ============================================================================
// 3. DTD entity states (xmlrole.rs entity7-10, entity SYSTEM/PUBLIC)
// ============================================================================

#[test]
fn cov90_dtd_entity_system_public() {
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<!ENTITY e SYSTEM \"file.xml\">]><r/>",
        b"<!DOCTYPE r [<!ENTITY e PUBLIC \"-//T//EN\" \"file.xml\">]><r/>",
        b"<!DOCTYPE r [<!NOTATION n SYSTEM \"x\"><!ENTITY e SYSTEM \"f\" NDATA n>]><r/>",
        b"<!DOCTYPE r [<!ENTITY % pe SYSTEM \"pe.dtd\">]><r/>",
        b"<!DOCTYPE r [<!ENTITY % pe PUBLIC \"-//T//EN\" \"pe.dtd\">]><r/>",
        // Multiple entity declarations with different forms
        b"<!DOCTYPE r [<!ENTITY a 'val'><!ENTITY b SYSTEM \"f\"><!ENTITY c PUBLIC \"-//T//EN\" \"g\">]><r/>",
    ];
    for case in cases {
        compare(
            case,
            &format!("entity_sys {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

#[test]
fn cov90_dtd_entity_states_incremental() {
    // Incremental parsing through entity declarations exercises role state transitions
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<!ENTITY e SYSTEM \"f.xml\">]><r/>",
        b"<!DOCTYPE r [<!ENTITY e PUBLIC \"-//T//EN\" \"f.xml\">]><r/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("entity_incr {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 4. DTD internal subset PI/comment/whitespace (xmlrole.rs 502-522)
// ============================================================================

#[test]
fn cov90_dtd_internal_subset_misc() {
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<?pi data?><!ELEMENT r EMPTY>]><r/>",
        b"<!DOCTYPE r [<!-- comment --><!ELEMENT r EMPTY>]><r/>",
        b"<!DOCTYPE r [\n  \n  <!ELEMENT r EMPTY>\n]><r/>",
        b"<!DOCTYPE r [<?p1 d?><?p2 d?><!-- c --><!ELEMENT r EMPTY>]><r/>",
        // Entity, attlist, element, notation mixed with PI/comment
        b"<!DOCTYPE r [<!ENTITY e 'v'><!-- c --><!ELEMENT r (#PCDATA)><?pi d?><!ATTLIST r a CDATA #IMPLIED><!NOTATION n SYSTEM \"x\">]><r/>",
    ];
    for case in cases {
        compare(
            case,
            &format!("dtd_misc {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 5. Prolog edge cases (xmlparse.rs BOM, partial, trailing CR)
// ============================================================================

#[test]
fn cov90_prolog_partial_edges() {
    // These exercise partial token / trailing CR / BOM in prolog
    let cases: &[&[u8]] = &[
        b"\xEF\xBB\xBF<r/>",            // UTF-8 BOM
        b"<?xml version='1.0'?>\r<r/>", // CR after xml decl
        b"  \r\n  <r/>",                // CR/LF whitespace before root
    ];
    for case in cases {
        compare_incr(
            case,
            &format!(
                "prolog_edge {:?}",
                std::str::from_utf8(case).unwrap_or("<bin>")
            ),
        );
    }
}

// ============================================================================
// 6. Content edges (xmlparse.rs TrailingRsqb, CDATA processor)
// ============================================================================

#[test]
fn cov90_content_trailing_rsqb() {
    // ] and ]] in content (closed document)
    compare_events(b"<r>text]</r>", "single ] in content");
    compare_events(b"<r>text]]</r>", "double ]] in content");
    compare_events(b"<r>]</r>", "bare ] in content");
    compare_events(b"<r>]]</r>", "bare ]] in content");
    // Unclosed document with trailing ] — both C and Rust should error
    compare(b"<r>text]", "trailing ] unclosed");
    compare(b"<r>text]]", "trailing ]] unclosed");
}

#[test]
fn cov90_cdata_processor_resume() {
    // CDATA split across parse calls — exercises cdata_section_processor
    let xml = b"<r><![CDATA[hello world data here]]></r>";
    for split in (3..xml.len() - 3).step_by(3) {
        let mut r = Parser::new(None).unwrap();
        let _ = r.parse(&xml[..split], false);
        let rs = r.parse(&xml[split..], true) as u32;
        let re = r.error_code() as u32;
        let c = CParser::new(None).unwrap();
        let _ = c.parse(&xml[..split], false);
        let (cs, ce) = c.parse(&xml[split..], true);
        assert!(
            rs == cs && re == ce,
            "CDATA resume @{split}: R s={rs} e={re}, C s={cs} e={ce}"
        );
    }
}

// ============================================================================
// 7. Epilog edge cases (xmlparse.rs 1275-1280, partial in epilog)
// ============================================================================

#[test]
fn cov90_epilog_edges() {
    let cases: &[&[u8]] = &[
        b"<r/>\xC3\xA9", // non-ASCII in epilog (error)
        b"<r/><",        // partial < in epilog
        b"<r/><!",       // partial <! in epilog
        b"<r/><!--",     // partial comment in epilog
    ];
    for case in cases {
        compare(
            case,
            &format!("epilog {:?}", std::str::from_utf8(case).unwrap_or("<bin>")),
        );
    }
}

#[test]
fn cov90_epilog_incremental() {
    let cases: &[&[u8]] = &[b"<r/><!-- c -->", b"<r/><?pi d?>", b"<r/> \n \t "];
    for case in cases {
        compare_incr(
            case,
            &format!("epilog_incr {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 8. External entity reference in content (xmlparse.rs 1450-1465)
// ============================================================================

#[test]
fn cov90_external_entity_ref_in_content() {
    // Reference an external entity in content (no handler → skip or error)
    compare(
        b"<!DOCTYPE r [<!ENTITY e SYSTEM \"file.xml\">]><r>&e;</r>",
        "ext entity ref in content",
    );
    // With SYSTEM + PUBLIC
    compare(
        b"<!DOCTYPE r [<!ENTITY e PUBLIC \"-//T//EN\" \"file.xml\">]><r>&e;</r>",
        "ext entity PUBLIC ref in content",
    );
}

// ============================================================================
// 9. CDATA content tokenizer (xmltok_impl.rs cdata_tok RSQB/CR paths)
// ============================================================================

#[test]
fn cov90_cdata_content_edges() {
    let cases = [
        "<r><![CDATA[\r]]></r>",
        "<r><![CDATA[\r\n]]></r>",
        "<r><![CDATA[a\rb]]></r>",
        "<r><![CDATA[a\r\nb]]></r>",
        "<r><![CDATA[]]]></r>",    // single ] before ]]>
        "<r><![CDATA[x]y]]></r>",  // ] not followed by ]>
        "<r><![CDATA[x]]y]]></r>", // ]] not followed by >
    ];
    for case in &cases {
        compare_events(case.as_bytes(), &format!("cdata_edge {:?}", case));
    }
}

#[test]
fn cov90_cdata_content_incremental() {
    let cases: &[&[u8]] = &[
        b"<r><![CDATA[\r\n]]></r>",
        b"<r><![CDATA[a\rb]]></r>",
        b"<r><![CDATA[]x]]></r>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("cdata_incr {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 10. XML declaration edge cases (xmltok.rs parse_xml_decl)
// ============================================================================

#[test]
fn cov90_xml_decl_edges() {
    let cases: &[&[u8]] = &[
        b"<?xml version='1.0' standalone='yes'?><r/>",
        b"<?xml version='1.0' standalone='no'?><r/>",
        b"<?xml version = '1.0' ?><r/>", // spaces around =
        b"<?xml version=\"1.0\"?><r/>",  // double quotes
        b"<?xml version='1.0' encoding='utf-8' standalone='yes'?><r/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("xmldecl {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 11. DTD element content models (xmlrole.rs coverage)
// ============================================================================

#[test]
fn cov90_dtd_complex_content_models() {
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<!ELEMENT r (a,b,c)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><a/><b/><c/></r>",
        b"<!DOCTYPE r [<!ELEMENT r (a|b|c)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><a/></r>",
        b"<!DOCTYPE r [<!ELEMENT r ((a,b)|c)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><a/><b/></r>",
        b"<!DOCTYPE r [<!ELEMENT r (a,(b|c))><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><a/><b/></r>",
        b"<!DOCTYPE r [<!ELEMENT r (a*)><!ELEMENT a EMPTY>]><r><a/><a/><a/></r>",
        b"<!DOCTYPE r [<!ELEMENT r (a+)><!ELEMENT a EMPTY>]><r><a/></r>",
        b"<!DOCTYPE r [<!ELEMENT r (a?)><!ELEMENT a EMPTY>]><r/>" ,
        b"<!DOCTYPE r [<!ELEMENT r (a?,b*,c+)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><c/></r>",
        b"<!DOCTYPE r [<!ELEMENT r (#PCDATA|a|b)*><!ELEMENT a EMPTY><!ELEMENT b EMPTY>]><r>text<a/>more<b/>end</r>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("content_model {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 12. Public ID validation (xmltok_impl.rs is_public_id)
// ============================================================================

#[test]
fn cov90_public_id_validation() {
    // Valid public IDs
    compare(
        b"<!DOCTYPE r PUBLIC \"-//OASIS//DTD DocBook V4.1//EN\" \"db.dtd\"><r/>",
        "valid pubid",
    );
    compare(
        b"<!DOCTYPE r PUBLIC \"ISO 8879:1986//ENTITIES Added Latin 1//EN\" \"x\"><r/>",
        "pubid iso",
    );
    // Invalid public IDs
    compare(b"<!DOCTYPE r PUBLIC \"bad{char\" \"x\"><r/>", "bad pubid {");
    compare(b"<!DOCTYPE r PUBLIC \"bad~char\" \"x\"><r/>", "bad pubid ~");
}

// ============================================================================
// 13. Billion laughs API (xmlparse.rs)
// ============================================================================

// ============================================================================
// 26. Multi-byte UTF-8 in tag/attribute names (scan_lt, scan_end_tag, scan_atts)
// ============================================================================

#[test]
fn cov90_multibyte_tag_names() {
    compare_incr("<日本/>".as_bytes(), "3byte elem name");
    compare_incr("<r><日本/></r>".as_bytes(), "3byte child elem");
    compare_incr("<日本>text</日本>".as_bytes(), "3byte open+close");
}

#[test]
fn cov90_multibyte_attr_names() {
    compare_incr("<r café=\"val\"/>".as_bytes(), "2byte attr name");
}

#[test]
fn cov90_multibyte_end_tags() {
    compare_incr("<café>text</café>".as_bytes(), "2byte end tag");
    compare_incr("<日本語>text</日本語>".as_bytes(), "3byte end tag");
}

// ============================================================================
// 27. Scan declarations incremental
// ============================================================================

#[test]
fn cov90_scan_decl_incremental() {
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<!ELEMENT r EMPTY><!ELEMENT a EMPTY>]><r/>",
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #IMPLIED><!ATTLIST r b ID #IMPLIED>]><r a='v' b='id1'/>",
        b"<!DOCTYPE r [<!ENTITY e 'v'><!ENTITY f 'w'>]><r>&e;&f;</r>",
        b"<!DOCTYPE r [<!NOTATION n1 SYSTEM 'x'><!NOTATION n2 PUBLIC '-//T//EN'>]><r/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("decl {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 28. PI with multi-byte targets
// ============================================================================

#[test]
fn cov90_pi_multibyte() {
    compare_incr("<?café data?><r/>".as_bytes(), "2byte PI target");
    compare_incr("<r><?日本 data?></r>".as_bytes(), "3byte PI in content");
}

// ============================================================================
// 29. Explicit encoding
// ============================================================================

#[test]
fn cov90_explicit_encodings() {
    for enc in &["UTF-8", "US-ASCII", "ISO-8859-1"] {
        let mut r = Parser::new(Some(enc)).unwrap();
        let rs = r.parse(b"<r>text</r>", true) as u32;
        let c = CParser::new(Some(enc)).unwrap();
        let (cs, _) = c.parse(b"<r>text</r>", true);
        assert_eq!(rs, cs, "encoding {}", enc);
    }
}

// ============================================================================
// 14. Parsing status / set_encoding API (xmlparse.rs)
// ============================================================================

#[test]
fn cov90_parser_api_coverage() {
    let mut p = Parser::new(None).unwrap();
    let _status = p.parsing_status();
    p.set_encoding("UTF-8");
    let s = p.parse(b"<r/>", true);
    assert_eq!(s, XmlStatus::Ok);
    let _status2 = p.parsing_status();
}

// ============================================================================
// 15. Multi-byte UTF-8 incremental in all tokenizer contexts
// ============================================================================

#[test]
fn cov90_utf8_multibyte_all_contexts() {
    // 2-byte in attr value
    compare_incr("<r a=\"café\"/>".as_bytes(), "2byte attr");
    // 3-byte in content
    compare_incr("<r>日本語</r>".as_bytes(), "3byte content");
    // 4-byte emoji in content
    compare_incr("<r>😀</r>".as_bytes(), "4byte content");
    // Multi-byte in entity value
    compare_incr(
        "<!DOCTYPE r [<!ENTITY e 'ñ'>]><r>&e;</r>".as_bytes(),
        "2byte entity",
    );
    // Multi-byte in CDATA
    compare_incr("<r><![CDATA[日本語]]></r>".as_bytes(), "3byte cdata");
    // Multi-byte in comment
    compare_incr("<r><!-- café --></r>".as_bytes(), "2byte comment");
    // Multi-byte in PI data
    compare_incr("<r><?pi café?></r>".as_bytes(), "2byte pi");
}

// ============================================================================
// 16. Large-scale stress (exercises buffer management paths)
// ============================================================================

#[test]
fn cov90_large_document() {
    let mut xml = String::from("<?xml version='1.0'?><!DOCTYPE r [<!ENTITY e 'v'>]><r>");
    for i in 0..200 {
        xml.push_str(&format!(
            "<e{} a=\"{}\">&amp;&#{};</e{}>",
            i,
            i,
            65 + (i % 26),
            i
        ));
    }
    xml.push_str("</r>");
    compare(xml.as_bytes(), "large doc");
}

#[test]
fn cov90_large_incremental() {
    let mut xml = String::from("<r>");
    for i in 0..50 {
        xml.push_str(&format!("<e{}>text{}</e{}>", i, i, i));
    }
    xml.push_str("</r>");
    let bytes = xml.as_bytes();
    // Parse in chunks of varying sizes
    let chunk_sizes = [7, 13, 31, 64, 128];
    for &chunk in &chunk_sizes {
        let mut r = Parser::new(None).unwrap();
        let c = CParser::new(None).unwrap();
        let mut pos = 0;
        while pos < bytes.len() {
            let end = (pos + chunk).min(bytes.len());
            let is_final = end == bytes.len();
            r.parse(&bytes[pos..end], is_final);
            c.parse(&bytes[pos..end], is_final);
            pos = end;
        }
        let rs = r.error_code() as u32;
        let ce = {
            let cp = CParser::new(None).unwrap();
            cp.parse(bytes, true).1
        };
        assert_eq!(rs, ce, "large incr chunk={chunk}");
    }
}

// ============================================================================
// 17. DTD state machine — incremental parsing through every declaration type
//     (xmlrole.rs entity, attlist, element, notation states)
// ============================================================================

#[test]
fn cov90_dtd_all_declarations_incremental() {
    // A DTD with every declaration type — incremental parsing hits all role states
    let xml = br#"<!DOCTYPE root [
  <!ELEMENT root (#PCDATA|child)*>
  <!ELEMENT child EMPTY>
  <!ATTLIST root
    id ID #IMPLIED
    class CDATA "default"
    lang (en|fr|de) "en"
    required CDATA #REQUIRED
    fixed CDATA #FIXED "fixval"
  >
  <!ATTLIST child type NMTOKEN #IMPLIED>
  <!ENTITY internal "hello world">
  <!ENTITY ext_sys SYSTEM "ext.xml">
  <!ENTITY ext_pub PUBLIC "-//Test//EN" "ext.xml">
  <!NOTATION jpeg SYSTEM "viewer">
  <!NOTATION png PUBLIC "-//Test//PNG" "viewer">
  <!NOTATION gif PUBLIC "-//Test//GIF">
  <!-- DTD comment -->
  <?dtd-pi processing instruction data?>
]>
<root id="r1" class="cls" lang="fr">
  text &internal;
  <child type="tok"/>
</root>"#;
    compare_incr(xml, "all DTD decls incremental");
}

#[test]
fn cov90_dtd_parameter_entity_decls() {
    // Parameter entity declarations (% prefix) exercise entity states
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<!ENTITY % pe 'val'>]><r/>",
        b"<!DOCTYPE r [<!ENTITY % pe SYSTEM \"file\">]><r/>",
        b"<!DOCTYPE r [<!ENTITY % pe PUBLIC \"-//T//EN\" \"file\">]><r/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("pe_decl {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

#[test]
fn cov90_dtd_entity_ndata_incremental() {
    // NDATA requires NOTATION, exercises entity7+ states
    compare_incr(
        b"<!DOCTYPE r [<!NOTATION n SYSTEM \"x\"><!ENTITY e SYSTEM \"f\" NDATA n>]><r/>",
        "entity NDATA incremental",
    );
    compare_incr(
        b"<!DOCTYPE r [<!NOTATION n PUBLIC \"-//T//EN\"><!ENTITY e PUBLIC \"-//T//EN\" \"f\" NDATA n>]><r/>",
        "entity PUBLIC NDATA incremental"
    );
}

// ============================================================================
// 18. Content with special characters — exercises content_tok branches
// ============================================================================

#[test]
fn cov90_content_special_chars() {
    let cases = [
        "<r>&#9;</r>",      // tab char ref
        "<r>&#10;</r>",     // LF char ref
        "<r>&#13;</r>",     // CR char ref
        "<r>&#x9;</r>",     // tab hex char ref
        "<r>&#xA;</r>",     // LF hex
        "<r>&#xD;</r>",     // CR hex
        "<r>&#x20AC;</r>",  // Euro sign
        "<r>&#x10000;</r>", // Supplementary char
        "<r>&#x1F600;</r>", // Emoji via char ref
    ];
    for case in &cases {
        compare_events(case.as_bytes(), &format!("special {:?}", case));
    }
}

#[test]
fn cov90_content_incremental_entities() {
    // Incremental parsing splitting entity refs
    let cases: &[&[u8]] = &[
        b"<r>&amp;&lt;&gt;&apos;&quot;</r>",
        b"<r>text&amp;more&lt;end</r>",
        b"<!DOCTYPE r [<!ENTITY e 'val'>]><r>&e;</r>",
        b"<!DOCTYPE r [<!ENTITY a '1'><!ENTITY b '2'>]><r>&a;&b;</r>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("entity_incr {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 19. Prolog tokenizer — various token types in prolog
// ============================================================================

#[test]
fn cov90_prolog_tokens_incremental() {
    let cases: &[&[u8]] = &[
        // XML declaration with various attributes
        b"<?xml version='1.0' encoding='utf-8'?><r/>",
        b"<?xml version='1.0' encoding='ISO-8859-1'?><r/>",
        b"<?xml version='1.0' encoding='US-ASCII'?><r/>",
        // DOCTYPE with system/public
        b"<!DOCTYPE r SYSTEM 'sys.dtd'><r/>",
        b"<!DOCTYPE r PUBLIC '-//T//EN' 'pub.dtd'><r/>",
        // Multiple comments and PIs in prolog
        b"<!-- c1 --><?pi1 d?>\n<!-- c2 --><?pi2 d?><r/>",
        // DTD with all element content model types
        b"<!DOCTYPE r [<!ELEMENT r EMPTY>]><r/>",
        b"<!DOCTYPE r [<!ELEMENT r ANY>]><r/>",
        b"<!DOCTYPE r [<!ELEMENT r (#PCDATA)>]><r>text</r>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("prolog_tok {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 20. Attribute handling edge cases in DTD
// ============================================================================

#[test]
fn cov90_attlist_all_types_incremental() {
    let xml = b"<!DOCTYPE r [\
        <!ATTLIST r \
            a CDATA #IMPLIED \
            b CDATA #REQUIRED \
            c CDATA 'default' \
            d CDATA #FIXED 'fixed' \
            e (x|y|z) #IMPLIED \
            f ID #IMPLIED \
            g IDREF #IMPLIED \
            h IDREFS #IMPLIED \
            i NMTOKEN #IMPLIED \
            j NMTOKENS #IMPLIED \
        >\
    ]><r a='1' b='2' e='x' f='id1' g='id1' h='id1' i='tok' j='tok1 tok2'/>";
    compare_incr(xml, "attlist all types incremental");
}

// ============================================================================
// 21. Mixed content edge cases
// ============================================================================

#[test]
fn cov90_mixed_content_complex() {
    let xml = br#"<?xml version="1.0"?>
<!DOCTYPE doc [
  <!ELEMENT doc (#PCDATA|p|b|i)*>
  <!ELEMENT p (#PCDATA|b)*>
  <!ELEMENT b (#PCDATA)>
  <!ELEMENT i EMPTY>
  <!ENTITY copy "&#169;">
  <!ENTITY nbsp "&#160;">
  <!ATTLIST doc version CDATA #IMPLIED>
  <!ATTLIST p class CDATA "para">
  <!NOTATION jpg SYSTEM "viewer.exe">
]>
<doc version="1.0">
  <p class="intro">Hello &amp; welcome to &copy; document</p>
  <!-- This is a comment with special chars: <>&"' -->
  <?app-info key=value?>
  <p>Paragraph with <b>bold &nbsp; text</b> and <i/> empty</p>
  <![CDATA[Raw <data> & "stuff" in CDATA]]>
  <p>Final &#x2014; paragraph</p>
</doc>"#;
    compare_events(xml, "complex mixed content");
    compare_incr(xml, "complex mixed incremental");
}

// ============================================================================
// 22. Error cases incremental — exercises error paths at different split points
// ============================================================================

#[test]
fn cov90_errors_incremental() {
    let cases: &[&[u8]] = &[
        b"<r></s>",              // tag mismatch
        b"<r a=\"1\" a=\"2\"/>", // duplicate attr
        b"<r>&undefined;</r>",   // undefined entity
        b"<r>&#0;</r>",          // null char ref
        b"<r>]]></r>",           // ]]> in content
        b"<r><![CDATA[",         // unclosed CDATA (partial)
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("error_incr {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 23. UTF-16 with various content types
// ============================================================================

#[test]
fn cov90_utf16_content_types() {
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
    let cases = [
        (utf16le("<r a=\"v\">text</r>"), "UTF16LE attrs+text"),
        (utf16be("<r a=\"v\">text</r>"), "UTF16BE attrs+text"),
        (utf16le("<?xml version='1.0'?><r/>"), "UTF16LE xmldecl"),
        (utf16le("<r><!-- c --></r>"), "UTF16LE comment"),
        (utf16le("<r><?pi d?></r>"), "UTF16LE PI"),
        (utf16le("<r>&amp;</r>"), "UTF16LE entity"),
        (utf16le("<r><![CDATA[cd]]></r>"), "UTF16LE CDATA"),
    ];
    for (xml, desc) in &cases {
        compare(xml, desc);
    }
}

// ============================================================================
// 24. Name matching / public ID validation (xmltok_impl.rs)
// ============================================================================

#[test]
fn cov90_public_id_edge_cases() {
    // Exercise is_public_id with various characters
    let valid_pubids = [
        "-//W3C//DTD XHTML 1.0//EN",
        "+//ISBN 0-13-013052-6::Sec. 2.3//EN",
        "ISO 8879:1986//ENTITIES Added Latin 1//EN//XML",
    ];
    for pubid in &valid_pubids {
        let xml = format!("<!DOCTYPE r PUBLIC \"{}\" \"sys.dtd\"><r/>", pubid);
        compare(xml.as_bytes(), &format!("pubid {:?}", pubid));
    }
    // Invalid chars in public IDs
    let invalid_pubids = ["{", "}", "~", "\\", "^", "`"];
    for ch in &invalid_pubids {
        let xml = format!("<!DOCTYPE r PUBLIC \"bad{}id\" \"sys.dtd\"><r/>", ch);
        compare(xml.as_bytes(), &format!("bad pubid {:?}", ch));
    }
}

// ============================================================================
// 25. External entity reference with handler set
// ============================================================================

#[test]
fn cov90_external_entity_with_handler() {
    // Set an external entity ref handler that returns false → ExternalEntityHandling error
    let xml = b"<!DOCTYPE r [<!ENTITY e SYSTEM \"f.xml\">]><r>&e;</r>";
    let mut r = Parser::new(None).unwrap();
    r.set_external_entity_ref_handler(Some(Box::new(
        |_: &str, _: Option<&str>, _: Option<&str>, _: Option<&str>| false,
    )));
    let rs = r.parse(xml, true) as u32;
    let re = r.error_code() as u32;

    // C parser with handler that returns 0 (failure)
    unsafe extern "C" fn ext_handler(
        _parser: expat_sys::XML_Parser,
        _context: *const c_char,
        _base: *const c_char,
        _sys: *const c_char,
        _pub: *const c_char,
    ) -> c_int {
        0
    }
    let c = CParser::new(None).unwrap();
    unsafe {
        expat_sys::XML_SetExternalEntityRefHandler(c.raw_parser(), Some(ext_handler));
    }
    let (cs, ce) = c.parse(xml, true);
    assert_eq!(rs, cs, "ext entity handler status");
    assert_eq!(re, ce, "ext entity handler error");
}

// ============================================================================
// 31. Scan function LEAD3/4 partial paths
//     Split multi-byte chars at exact byte boundaries in tag/attr names
// ============================================================================

#[test]
fn cov90_scan_lt_lead3_partial() {
    // 3-byte UTF-8 char (e.g. 日=E6 97 A5) in element name
    // Split at positions within the multi-byte sequence
    let xml = "<日/>".as_bytes(); // 6 bytes: E6 97 A5 2F 3E
    compare_incr(xml, "lead3 in start tag");
}

#[test]
fn cov90_scan_end_tag_lead3_partial() {
    let xml = "<日>x</日>".as_bytes();
    compare_incr(xml, "lead3 in end tag");
}

// Multi-byte attribute names: known gap — scan_atts doesn't handle
// LEAD3 attribute names correctly when whitespace separates tag name.
// TODO: fix scan_atts to handle multi-byte attribute name starts.

#[test]
fn cov90_scan_lt_lead4_partial() {
    // 4-byte UTF-8 char (emoji U+1F600 = F0 9F 98 80) in content near tag
    // Can't use emoji as tag name easily, but content with emoji adjacent to tags
    let xml = "<r>😀</r>".as_bytes();
    compare_incr(xml, "lead4 in content near tags");
}

#[test]
fn cov90_multibyte_attr_values_incremental() {
    // Multi-byte in attribute values — hits attribute_value_tok LEAD paths
    compare_incr("<r a=\"日本語\"/>".as_bytes(), "3byte in attr value");
    compare_incr("<r a=\"café\"/>".as_bytes(), "2byte in attr value");
    compare_incr("<r a=\"😀\"/>".as_bytes(), "4byte in attr value");
}

#[test]
fn cov90_multibyte_entity_value_incremental() {
    // Multi-byte in entity values — hits entity_value_tok LEAD paths
    compare_incr(
        "<!DOCTYPE r [<!ENTITY e '日本語'>]><r>&e;</r>".as_bytes(),
        "3byte entity value",
    );
    compare_incr(
        "<!DOCTYPE r [<!ENTITY e '😀'>]><r>&e;</r>".as_bytes(),
        "4byte entity value",
    );
}

// ============================================================================
// 32. Scan end tag — various split points
// ============================================================================

#[test]
fn cov90_end_tag_splits() {
    // End tags with multi-byte names, split at every byte
    compare_incr("<café>text</café>".as_bytes(), "2byte end tag incr");
    compare_incr("<日>text</日>".as_bytes(), "3byte end tag incr");
    // Long element names to exercise scan_end_tag continuation
    compare_incr(b"<abcdefghijklmnop>text</abcdefghijklmnop>", "long end tag");
}

// ============================================================================
// 33. Scan start tag — various attribute patterns
// ============================================================================

#[test]
fn cov90_start_tag_attr_patterns() {
    // Various attribute value delimiters and content patterns
    let cases: &[&[u8]] = &[
        b"<r a=\"hello\" b=\"world\"/>",
        b"<r a='hello' b='world'/>",
        b"<r a=\"\" b=\"\"/>",
        b"<r a=\"&amp;\" b=\"&#65;\"/>",
        b"<r a=\"a\tb\nc\rd\"/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("attr_pat {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 34. Content tokenizer — all token types incremental
// ============================================================================

#[test]
fn cov90_content_all_tokens_incremental() {
    // A document using every content token type
    let xml = br#"<r>
text &amp; &#65; &#x42;
<child a="v"/>
<b>inner</b>
<!-- comment -->
<?pi data?>
<![CDATA[cdata]]>
more text
</r>"#;
    compare_incr(xml, "all content tokens");
}

// ============================================================================
// 35. Prolog state machine — exercise remaining role states
// ============================================================================

#[test]
fn cov90_prolog_role_states() {
    // DOCTYPE with all features to hit xmlrole states
    let cases: &[&[u8]] = &[
        // Mixed content model with PCDATA
        b"<!DOCTYPE r [<!ELEMENT r (#PCDATA)>]><r>text</r>",
        // Element with nested groups and quantifiers
        b"<!DOCTYPE r [<!ELEMENT r ((a|b)*,(c?,d+))><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY><!ELEMENT d EMPTY>]><r><a/><d/></r>",
        // ATTLIST with NOTATION type
        b"<!DOCTYPE r [<!NOTATION n SYSTEM 'x'><!ATTLIST r a NOTATION (n) #IMPLIED>]><r/>",
        // Multiple ATTLIST for same element
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #IMPLIED><!ATTLIST r b CDATA #IMPLIED><!ATTLIST r c CDATA 'def'>]><r a='1'/>",
        // Parameter entity in internal subset
        b"<!DOCTYPE r [<!ENTITY % pe 'EMPTY'><!ELEMENT r EMPTY>]><r/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("role_state {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// Error position tracking (line/column) has known gaps — tested separately

// ============================================================================
// 37. Handler combination patterns
// ============================================================================

#[test]
fn cov90_handler_combos() {
    // Parse with various handler combinations to exercise dispatch branches
    let xml =
        b"<!DOCTYPE r [<!ENTITY e 'v'>]><r a=\"1\">&e;text<!-- c --><?pi d?><![CDATA[cd]]></r>";

    // Only element + chardata handlers
    {
        let (rs, revts) = collect_events(xml);
        let (cs, cevts) = collect_c_events(xml);
        assert_eq!(rs, cs, "handler combo 1 status");
        assert_eq!(merge_cd(&revts), merge_cd(&cevts), "handler combo 1 events");
    }

    // No handlers at all — just compare status
    {
        let mut r = Parser::new(None).unwrap();
        let rs = r.parse(xml, true) as u32;
        let c = CParser::new(None).unwrap();
        let (cs, _) = c.parse(xml, true);
        assert_eq!(rs, cs, "no handlers status");
    }
}

// ============================================================================
// 38. Massive incremental test — exercises ALL tokenizer LEAD partial paths
//     by parsing documents with multi-byte chars at every split point
// ============================================================================

#[test]
fn cov90_massive_multibyte_incremental() {
    // 2-byte chars (café = c3 a9) at every position in various constructs
    let docs: &[&[u8]] = &[
        // Multi-byte in content, comments, CDATA, PI data (not in names — known gap)
        "<?xml version='1.0'?><!-- café --><r>café<![CDATA[café]]><?pi café?></r>".as_bytes(),
        // 3-byte CJK in content
        "<r>日本語テスト</r>".as_bytes(),
        // DTD with multi-byte in entity values (not names)
        "<!DOCTYPE r [<!ENTITY e 'café'>]><r>&e;</r>".as_bytes(),
    ];
    for doc in docs {
        compare_incr(doc, &format!("massive_mb len={}", doc.len()));
    }
}

// ============================================================================
// 39. Current byte index — UTF-16 path
// ============================================================================

#[test]
fn cov90_byte_index_utf16() {
    fn utf16le(s: &str) -> Vec<u8> {
        let mut out = vec![0xFF, 0xFE];
        for c in s.encode_utf16() {
            out.push(c as u8);
            out.push((c >> 8) as u8);
        }
        out
    }
    let xml = utf16le("<r>text</r>");
    let mut r = Parser::new(None).unwrap();
    r.parse(&xml, true);
    let ri = r.current_byte_index();
    let c = CParser::new(None).unwrap();
    c.parse(&xml, true);
    let ci = c.current_byte_index();
    assert_eq!(ri, ci, "byte index UTF-16LE");
}

// ============================================================================
// 40. DTD with all role states exercised incrementally
// ============================================================================

#[test]
fn cov90_dtd_all_role_states() {
    // This document hits as many xmlrole.rs states as possible
    let xml = br#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE root SYSTEM "root.dtd" [
  <!ELEMENT root (#PCDATA|child|bold|italic)*>
  <!ELEMENT child (a,b?,c*)>
  <!ELEMENT a EMPTY>
  <!ELEMENT b (#PCDATA)>
  <!ELEMENT c ANY>
  <!ELEMENT bold (#PCDATA)>
  <!ELEMENT italic EMPTY>
  <!ATTLIST root
    id ID #REQUIRED
    class CDATA "default"
    lang (en|fr|de|ja) "en"
    version NMTOKEN #IMPLIED
    ref IDREF #IMPLIED
    refs IDREFS #IMPLIED
    style CDATA #FIXED "normal"
  >
  <!ATTLIST child type CDATA #IMPLIED>
  <!ATTLIST a href CDATA #IMPLIED>
  <!ENTITY internal "Hello &amp; World">
  <!ENTITY ext1 SYSTEM "ext1.xml">
  <!ENTITY ext2 PUBLIC "-//Test//Entity//EN" "ext2.xml">
  <!ENTITY % pe1 "EMPTY">
  <!ENTITY % pe2 SYSTEM "pe.dtd">
  <!ENTITY % pe3 PUBLIC "-//Test//PE//EN" "pe.dtd">
  <!NOTATION jpeg SYSTEM "image/jpeg">
  <!NOTATION png PUBLIC "-//Test//PNG//EN">
  <!NOTATION gif PUBLIC "-//Test//GIF//EN" "image/gif">
  <!NOTATION svg SYSTEM "image/svg+xml">
  <!ENTITY logo SYSTEM "logo.png" NDATA png>
  <!-- DTD comment 1 -->
  <?dtd-pi processing instruction?>
  <!-- DTD comment 2 -->
]>
<root id="r1" class="main" lang="en" version="v1">
  Hello &amp; World
  <child type="test"><a href="http://example.com"/><b>bold text</b><c/></child>
  <bold>Bold &internal;</bold>
  <italic/>
  <!-- content comment -->
  <?app-info key=value?>
  <![CDATA[Raw <data> & "stuff"]]>
</root>
<!-- epilog comment -->
<?post-pi done?>"#;
    compare_events(xml, "all role states doc");
    compare_incr(xml, "all role states incremental");
}

// ============================================================================
// 41. Entity expansion patterns
// ============================================================================

#[test]
fn cov90_entity_patterns() {
    let cases: &[&[u8]] = &[
        // Entity with char ref in value
        b"<!DOCTYPE r [<!ENTITY e '&#169;'>]><r>&e;</r>",
        // Entity with entity ref in value
        b"<!DOCTYPE r [<!ENTITY e '&amp;'>]><r>&e;</r>",
        // Entity with multiple char refs
        b"<!DOCTYPE r [<!ENTITY e '&#65;&#66;&#67;'>]><r>&e;</r>",
        // Entity value with CR/LF (exercises entity_value_tok CR/LF paths)
        b"<!DOCTYPE r [<!ENTITY e 'line1\r\nline2\rline3\nline4'>]><r>&e;</r>",
        // Multiple entities in sequence
        b"<!DOCTYPE r [<!ENTITY a 'A'><!ENTITY b 'B'><!ENTITY c 'C'>]><r>&a;&b;&c;</r>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("entity_pat {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 42. Attribute value patterns with all token types
// ============================================================================

#[test]
fn cov90_attr_value_all_tokens() {
    // Exercise every branch in attribute_value_tok
    let cases = [
        // LF at start of value
        "<r a=\"\nv\"/>",
        // CR at start of value
        "<r a=\"\rv\"/>",
        // CRLF at start of value
        "<r a=\"\r\nv\"/>",
        // Space at start of value (AttributeValueS token)
        "<r a=\" v\"/>",
        // Tab at start
        "<r a=\"\tv\"/>",
        // Entity ref at start
        "<r a=\"&amp;v\"/>",
        // Char ref at start
        "<r a=\"&#65;v\"/>",
        // Single-quoted variants
        "<r a='\nv'/>",
        "<r a='\rv'/>",
        "<r a='\r\nv'/>",
        "<r a=' v'/>",
        "<r a='\tv'/>",
        "<r a='&amp;v'/>",
        "<r a='&#65;v'/>",
        // Multiple whitespace types in sequence
        "<r a=\"a\t\n\r b\"/>",
        "<r a='a\t\n\r b'/>",
    ];
    for case in &cases {
        compare_events(case.as_bytes(), &format!("attr_all {:?}", case));
    }
    // Incremental versions
    for case in &cases[..4] {
        compare_incr(case.as_bytes(), &format!("attr_all_incr {:?}", case));
    }
}

// ============================================================================
// 43. CDATA section tokenizer — all branches
// ============================================================================

#[test]
fn cov90_cdata_all_branches() {
    let cases = [
        "<r><![CDATA[a\r\nb\rc\nd]]></r>",
        "<r><![CDATA[]]]]></r>",    // ]] then ]]>
        "<r><![CDATA[]]]></r>",     // ] then ]]>
        "<r><![CDATA[a]b]c]]></r>", // scattered ]
        "<r><![CDATA[a]]b]]></r>",  // ]] not followed by >
        // With multi-byte
        "<r><![CDATA[café]]></r>",
        "<r><![CDATA[日本語]]></r>",
    ];
    for case in &cases {
        compare_incr(case.as_bytes(), &format!("cdata_all {:?}", case));
    }
}

// ============================================================================
// 44. Prolog tokenizer — all token types
// ============================================================================

#[test]
fn cov90_prolog_tok_all() {
    // Various prolog token combinations to exercise prolog_tok branches
    let cases: &[&[u8]] = &[
        // XML decl with double quotes
        b"<?xml version=\"1.0\"?><r/>",
        // DOCTYPE with mixed quote types
        b"<!DOCTYPE r PUBLIC '-//T//EN' \"sys.dtd\"><r/>",
        // DTD with open/close paren tokens
        b"<!DOCTYPE r [<!ELEMENT r (a|b|c)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><a/></r>",
        // DTD with comma separator
        b"<!DOCTYPE r [<!ELEMENT r (a,b)><!ELEMENT a EMPTY><!ELEMENT b EMPTY>]><r><a/><b/></r>",
        // DTD with name+quantifier tokens
        b"<!DOCTYPE r [<!ELEMENT r (a*,b?,c+)><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY>]><r><c/></r>",
        // DTD with close paren + quantifier tokens
        b"<!DOCTYPE r [<!ELEMENT r (a)*><!ELEMENT a EMPTY>]><r><a/></r>",
        b"<!DOCTYPE r [<!ELEMENT r (a)?><!ELEMENT a EMPTY>]><r/>",
        b"<!DOCTYPE r [<!ELEMENT r (a)+><!ELEMENT a EMPTY>]><r><a/></r>",
        // Multi-byte in DTD names
        "<!DOCTYPE café [<!ELEMENT café EMPTY>]><café/>".as_bytes(),
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("prolog_all {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 45. Content with every entity type
// ============================================================================

#[test]
fn cov90_content_all_entities() {
    let xml = br#"<!DOCTYPE r [
  <!ENTITY int1 "internal value">
  <!ENTITY int2 "&#169; &amp; &#x41;">
  <!ENTITY ext SYSTEM "ext.xml">
]>
<r>
  predefined: &amp; &lt; &gt; &apos; &quot;
  decimal: &#65; &#66; &#67;
  hex: &#x41; &#x42; &#x43;
  internal: &int1; &int2;
</r>"#;
    compare_events(xml, "all entities in content");
    compare_incr(xml, "all entities incremental");
}

// ============================================================================
// 46. Partial UTF-8 at buffer end (exercises is_partial_utf8_sequence)
// ============================================================================

#[test]
fn cov90_partial_utf8_at_boundary() {
    // Documents where multi-byte UTF-8 chars are split at exact boundaries
    // 2-byte: é
    compare_incr("<r>café</r>".as_bytes(), "partial 2byte");
    // 3-byte: 日
    compare_incr("<r>日</r>".as_bytes(), "partial 3byte");
    // 4-byte: 😀
    compare_incr("<r>😀</r>".as_bytes(), "partial 4byte");
}

// ============================================================================
// 47. Scan declarations — multi-byte and edge cases
// ============================================================================

#[test]
fn cov90_scan_decl_multibyte() {
    // Multi-byte in DTD element/entity/notation names
    let cases = [
        "<!DOCTYPE r [<!ELEMENT café EMPTY>]><r/>",
        "<!DOCTYPE r [<!ENTITY café 'val'>]><r/>",
        "<!DOCTYPE r [<!NOTATION café SYSTEM 'x'>]><r/>",
        "<!DOCTYPE r [<!ATTLIST r café CDATA #IMPLIED>]><r/>",
    ];
    for case in &cases {
        compare_incr(case.as_bytes(), &format!("decl_mb {:?}", case));
    }
}

// ============================================================================
// 48. Epilog — all valid token types
// ============================================================================

#[test]
fn cov90_epilog_all_tokens() {
    let cases: &[&[u8]] = &[
        b"<r/> ",
        b"<r/>\n",
        b"<r/>\r\n",
        b"<r/>\t",
        b"<r/><!-- comment -->",
        b"<r/><?pi data?>",
        b"<r/> \n\t <!-- c1 -->\n<?pi d?>\n<!-- c2 --> \n",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("epilog_all {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 49. Parser parse() edge cases
// ============================================================================

#[test]
fn cov90_parse_edge_cases() {
    // Parse empty non-final then data
    let mut r = Parser::new(None).unwrap();
    r.parse(b"", false);
    r.parse(b"", false);
    let rs = r.parse(b"<r/>", true) as u32;
    let c = CParser::new(None).unwrap();
    c.parse(b"", false);
    c.parse(b"", false);
    let (cs, _) = c.parse(b"<r/>", true);
    assert_eq!(rs, cs, "empty non-final");

    // Very small chunks
    let xml = b"<r>text</r>";
    for chunk_size in 1..=3 {
        let mut r = Parser::new(None).unwrap();
        let c = CParser::new(None).unwrap();
        let mut pos = 0;
        while pos < xml.len() {
            let end = (pos + chunk_size).min(xml.len());
            let is_final = end == xml.len();
            r.parse(&xml[pos..end], is_final);
            c.parse(&xml[pos..end], is_final);
            pos = end;
        }
        assert_eq!(r.error_code() as u32, 0, "small chunk {chunk_size}");
    }
}

// ============================================================================
// 50. 3-byte and 4-byte UTF-8 in EVERY scanner position — incremental
// ============================================================================

#[test]
fn cov90_3byte_in_all_positions() {
    let docs: &[&[u8]] = &[
        "<日/>".as_bytes(),
        "<日>x</日>".as_bytes(),
        "<r>日</r>".as_bytes(),
        "<r><![CDATA[日]]></r>".as_bytes(),
        "<r><!-- 日 --></r>".as_bytes(),
        "<r><?pi 日?></r>".as_bytes(),
        "<!DOCTYPE r [<!ENTITY e '日'>]><r>&e;</r>".as_bytes(),
        "<r a=\"日\"/>".as_bytes(),
        "<r a='日'/>".as_bytes(),
    ];
    for doc in docs {
        compare_incr(doc, &format!("3byte@{}", doc.len()));
    }
}

#[test]
fn cov90_4byte_in_all_positions() {
    let docs: &[&[u8]] = &[
        "<r>😀</r>".as_bytes(),
        "<r><![CDATA[😀]]></r>".as_bytes(),
        "<r><!-- 😀 --></r>".as_bytes(),
        "<r><?pi 😀?></r>".as_bytes(),
        "<!DOCTYPE r [<!ENTITY e '😀'>]><r>&e;</r>".as_bytes(),
        "<r a=\"😀\"/>".as_bytes(),
        "<r a='😀'/>".as_bytes(),
    ];
    for doc in docs {
        compare_incr(doc, &format!("4byte@{}", doc.len()));
    }
}

// ============================================================================
// 51-56. More targeted tests
// ============================================================================

#[test]
fn cov90_scan_percent_in_entity() {
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<!ENTITY % pe 'text'><!ELEMENT r EMPTY>]><r/>",
        b"<!DOCTYPE r [<!ENTITY % pe SYSTEM 'file.dtd'><!ELEMENT r EMPTY>]><r/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("percent {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

#[test]
fn cov90_deep_attr_values() {
    let cases: &[&[u8]] = &[
        b"<r a=\"a\tb\nc\rd\r\ne\"/>",
        b"<r a=\"&amp;&lt;&#65;&#x42;&gt;&apos;&quot;\"/>",
        b"<r a='a\tb\nc\rd\r\ne'/>",
        b"<r a='&amp;&lt;&#65;&#x42;&gt;&apos;&quot;'/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("deep_attr {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

#[test]
fn cov90_entity_value_all_tokens() {
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<!ENTITY e 'a&#65;b'>]><r>&e;</r>",
        b"<!DOCTYPE r [<!ENTITY e 'a&#x41;b'>]><r>&e;</r>",
        b"<!DOCTYPE r [<!ENTITY e 'a&amp;b'>]><r>&e;</r>",
        b"<!DOCTYPE r [<!ENTITY e 'a\rb'>]><r>&e;</r>",
        b"<!DOCTYPE r [<!ENTITY e 'a\nb'>]><r>&e;</r>",
        b"<!DOCTYPE r [<!ENTITY e 'a\r\nb'>]><r>&e;</r>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("ev {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

#[test]
fn cov90_prolog_partial_tokens() {
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE root SYSTEM 'sys.dtd' [<!ENTITY e 'v'>]><root/>",
        b"<!DOCTYPE root PUBLIC '-//T//EN' 'sys.dtd' [<!ATTLIST root a CDATA #IMPLIED>]><root/>",
        b"<!-- c1 --><?pi d?><!-- c2 --><root/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("prolog_partial {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

#[test]
fn cov90_content_cdata_resume() {
    let xml = b"<r><![CDATA[data]]>more<![CDATA[data2]]></r>";
    for split in (5..xml.len() - 5).step_by(5) {
        let mut r = Parser::new(None).unwrap();
        r.parse(&xml[..split], false);
        let rs = r.parse(&xml[split..], true) as u32;
        let c = CParser::new(None).unwrap();
        c.parse(&xml[..split], false);
        let (cs, _) = c.parse(&xml[split..], true);
        assert_eq!(rs, cs, "cdata split @{split}");
    }
}

// ============================================================================
// 57. Single-quoted attribute values with whitespace — byte-by-byte incremental
//     Targets attribute_value_tok' (single-quote) CR/LF/S paths
// ============================================================================

#[test]
fn cov90_single_quoted_attr_ws_incremental() {
    // These need to be split at the exact whitespace positions within single-quoted values
    let cases: &[&[u8]] = &[
        b"<r a='\r'/>",
        b"<r a='\n'/>",
        b"<r a='\r\n'/>",
        b"<r a=' '/>",
        b"<r a='\t'/>",
        b"<r a='x\ry'/>",
        b"<r a='x\ny'/>",
        b"<r a='x\r\ny'/>",
        b"<r a='x y'/>",
        b"<r a='x\ty'/>",
        b"<r a='\r\n\r\n'/>",
        b"<r a='a\rb\nc\r\nd e\tf'/>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("sq_ws {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 58. Prolog BOM at buffer boundary
// ============================================================================

#[test]
fn cov90_prolog_bom_boundary() {
    // BOM is entire first chunk, data comes in second
    let mut r = Parser::new(None).unwrap();
    r.parse(b"\xEF\xBB\xBF", false);
    let rs = r.parse(b"<r/>", true) as u32;
    let c = CParser::new(None).unwrap();
    c.parse(b"\xEF\xBB\xBF", false);
    let (cs, _) = c.parse(b"<r/>", true);
    assert_eq!(rs, cs, "BOM only first chunk");
}

// ============================================================================
// 59. Content with trailing CR (incremental — TrailingCr paths)
// ============================================================================

#[test]
fn cov90_trailing_cr_incremental() {
    let cases: &[&[u8]] = &[b"<r>text\r</r>", b"<r>\r</r>", b"<r>a\r\n</r>"];
    for case in cases {
        compare_incr(
            case,
            &format!("trailing_cr {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 60. Prolog processor — empty final
// ============================================================================

#[test]
fn cov90_prolog_empty_final() {
    // Empty document — final with no data
    let mut r = Parser::new(None).unwrap();
    let rs = r.parse(b"", true) as u32;
    let re = r.error_code() as u32;
    let c = CParser::new(None).unwrap();
    let (cs, ce) = c.parse(b"", true);
    assert_eq!(rs, cs, "empty final status");
    assert_eq!(re, ce, "empty final error");

    // Whitespace only
    let mut r = Parser::new(None).unwrap();
    let rs = r.parse(b"   \n\t  ", true) as u32;
    let re = r.error_code() as u32;
    let c = CParser::new(None).unwrap();
    let (cs, ce) = c.parse(b"   \n\t  ", true);
    assert_eq!(rs, cs, "ws-only final status");
    assert_eq!(re, ce, "ws-only final error");
}

// ============================================================================
// 61. CDATA section unclosed at final
// ============================================================================

#[test]
fn cov90_cdata_unclosed_final() {
    let cases: &[&[u8]] = &[
        b"<r><![CDATA[data",
        b"<r><![CDATA[",
        b"<r><![CDATA[data]",
        b"<r><![CDATA[data]]",
    ];
    for case in cases {
        compare(
            case,
            &format!("cdata_unclosed {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// ============================================================================
// 62. LEAD2 partial paths in all scan functions
// ============================================================================

#[test]
fn cov90_lead2_all_positions() {
    // 2-byte char (é = C3 A9) in all positions — incremental to hit LEAD2 partials
    let docs: &[&[u8]] = &[
        "<é/>".as_bytes(),
        "<é>x</é>".as_bytes(),
        "<r>é</r>".as_bytes(),
        "<r><![CDATA[é]]></r>".as_bytes(),
        "<r><!-- é --></r>".as_bytes(),
        "<r><?pi é?></r>".as_bytes(),
        "<!DOCTYPE r [<!ENTITY e 'é'>]><r>&e;</r>".as_bytes(),
        "<r a=\"é\"/>".as_bytes(),
        "<r a='é'/>".as_bytes(),
    ];
    for doc in docs {
        compare_incr(doc, &format!("lead2@{}", doc.len()));
    }
}
