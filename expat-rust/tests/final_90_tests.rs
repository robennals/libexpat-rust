//! Final push to 90% line coverage. All tests compare C and Rust.

use expat_rust::xmlparse::{Parser, XmlError, XmlStatus};
use expat_sys::CParser;
use std::ffi::{c_char, c_void};

fn compare(xml: &[u8], desc: &str) {
    let mut r = Parser::new(None).unwrap();
    let rs = r.parse(xml, true) as u32;
    let re = r.error_code() as u32;
    let c = CParser::new(None).unwrap();
    let (cs, ce) = c.parse(xml, true);
    assert!(
        rs == cs && re == ce,
        "MISMATCH {desc}: R s={rs} e={re}, C s={cs} e={ce}"
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

// 1. stop() API — exercise without handler callback (can't call stop from inside handler
// due to borrow rules, but can call before/after parsing)
#[test]
fn final90_stop_before_parse() {
    let mut r = Parser::new(None).unwrap();
    // Stop before parsing starts — should return error (not started)
    let rs = r.stop(true);
    assert_eq!(rs, XmlStatus::Error);
    assert_eq!(r.error_code(), XmlError::NotStarted);

    // C comparison
    let c = CParser::new(None).unwrap();
    let cs = unsafe { expat_sys::XML_StopParser(c.raw_parser(), 1) };
    assert_eq!(rs as u32, cs, "stop before parse");
}

// 2. resume() without suspend — should error
#[test]
fn final90_resume_not_suspended() {
    let mut r = Parser::new(None).unwrap();
    let rs = r.resume();
    assert_eq!(rs, XmlStatus::Error);
    assert_eq!(r.error_code(), XmlError::NotSuspended);

    let c = CParser::new(None).unwrap();
    let cs = unsafe { expat_sys::XML_ResumeParser(c.raw_parser()) };
    assert_eq!(rs as u32, cs, "resume not suspended");
}

// 3. stop() after parsing finished
#[test]
fn final90_stop_after_finish() {
    let mut r = Parser::new(None).unwrap();
    r.parse(b"<r/>", true);
    let rs = r.stop(true);
    assert_eq!(rs, XmlStatus::Error);
    assert_eq!(r.error_code(), XmlError::Finished);
}

// 4. XmlError::message() — covers lines 108-110
#[test]
fn final90_error_message() {
    let errors = [
        XmlError::Syntax,
        XmlError::InvalidToken,
        XmlError::TagMismatch,
    ];
    for e in errors {
        let msg = e.message();
        assert!(!msg.is_empty());
    }
}

// 5. Encoding specified as UTF-16 with BOM — covers encoding paths
#[test]
fn final90_explicit_utf16_encoding() {
    fn utf16le(s: &str) -> Vec<u8> {
        let mut out = vec![0xFF, 0xFE];
        for c in s.encode_utf16() {
            out.push(c as u8);
            out.push((c >> 8) as u8);
        }
        out
    }
    // Explicit UTF-16LE encoding
    let xml = utf16le("<r/>");
    let mut r = Parser::new(Some("UTF-16LE")).unwrap();
    let rs = r.parse(&xml, true) as u32;
    let c = CParser::new(Some("UTF-16LE")).unwrap();
    let (cs, _) = c.parse(&xml, true);
    assert_eq!(rs, cs, "explicit UTF-16LE");
}

// 6. CDATA section: non-final chunk then empty final — covers 1162-1165
#[test]
fn final90_cdata_empty_final() {
    // Start CDATA, then empty final = UnclosedCdataSection
    let mut r = Parser::new(None).unwrap();
    r.parse(b"<r><![CDATA[data", false);
    let rs = r.parse(b"", true) as u32;
    let re = r.error_code() as u32;
    let c = CParser::new(None).unwrap();
    c.parse(b"<r><![CDATA[data", false);
    let (cs, ce) = c.parse(b"", true);
    assert_eq!(rs, cs, "cdata empty final status");
    assert_eq!(re, ce, "cdata empty final error");
}

// 7. Content processor: non-final with empty data then final empty — covers 1197-1200
#[test]
fn final90_content_empty_final_no_root() {
    // Non-final empty, then final empty = NoElements
    let mut r = Parser::new(None).unwrap();
    r.parse(b"", false);
    let rs = r.parse(b"", true) as u32;
    let re = r.error_code() as u32;
    let c = CParser::new(None).unwrap();
    c.parse(b"", false);
    let (cs, ce) = c.parse(b"", true);
    assert_eq!(rs, cs, "empty final no root status");
    assert_eq!(re, ce, "empty final no root error");
}

// 8. Epilog PartialChar — covers 1343-1348
#[test]
fn final90_epilog_partial_char() {
    // Valid doc followed by partial UTF-8 in epilog
    // C3 without A9 = incomplete 2-byte sequence
    let xml = b"<r/>\xC3";
    let mut r = Parser::new(None).unwrap();
    r.parse(xml, false);
    let rs = r.parse(b"", true) as u32; // final with partial char pending
    let re = r.error_code() as u32;
    let c = CParser::new(None).unwrap();
    c.parse(xml, false);
    let (cs, ce) = c.parse(b"", true);
    assert_eq!(rs, cs, "epilog partial char status");
    assert_eq!(re, ce, "epilog partial char error");
}

// 9. Attribute value with unknown entity (covers normalize_attribute_value unknown entity path 1999-2001)
#[test]
fn final90_attr_unknown_entity() {
    // DOCTYPE with SYSTEM (has_param_entity_refs=true) + undefined entity in attr
    compare(
        b"<!DOCTYPE r SYSTEM 'ext.dtd'><r a='&unknown;'/>",
        "attr unknown entity with ext subset",
    );
}

// 10. Entity value with unhandled token (covers store_entity_value catch-all 2132-2135)
#[test]
fn final90_entity_value_edge() {
    // Entity value with char ref
    compare_incr(
        b"<!DOCTYPE r [<!ENTITY e '&#x41;&#x42;&#x43;'>]><r>&e;</r>",
        "entity value char refs",
    );
}

// 11. Latin-1 encoding byte index (covers current_byte_index Latin-1 path 2702-2708)
#[test]
fn final90_byte_index_latin1() {
    let mut r = Parser::new(Some("ISO-8859-1")).unwrap();
    r.parse(b"<r>text</r>", true);
    let ri = r.current_byte_index();
    let c = CParser::new(Some("ISO-8859-1")).unwrap();
    c.parse(b"<r>text</r>", true);
    let ci = c.current_byte_index();
    assert_eq!(ri, ci, "byte index latin1");
}

// 12. UTF-16 byte index with surrogate pair (covers 2670-2672, 2688-2691)
#[test]
fn final90_byte_index_utf16_surrogate() {
    fn utf16le(s: &str) -> Vec<u8> {
        let mut out = vec![0xFF, 0xFE];
        for c in s.encode_utf16() {
            out.push(c as u8);
            out.push((c >> 8) as u8);
        }
        out
    }
    let xml = utf16le("<r>😀</r>");
    let mut r = Parser::new(None).unwrap();
    r.parse(&xml, true);
    let ri = r.current_byte_index();
    let c = CParser::new(None).unwrap();
    c.parse(&xml, true);
    let ci = c.current_byte_index();
    assert_eq!(ri, ci, "byte index utf16 surrogate");
}

// 13. scan_atts whitespace between attrs (covers 1064-1067)
#[test]
fn final90_attrs_whitespace_between() {
    compare_incr(b"<r a=\"1\"  b=\"2\"  c=\"3\"/>", "attrs double space");
    compare_incr(b"<r a=\"1\"\n\nb=\"2\"/>", "attrs newlines between");
    compare_incr(b"<r a=\"1\"\r\nb=\"2\"/>", "attrs crlf between");
}

// 14. Prolog BOM handling — BOM at exact buffer boundary (covers 691-697)
#[test]
fn final90_prolog_bom_exact_boundary() {
    // UTF-8 BOM as entire first chunk, then XML decl in second
    let bom = b"\xEF\xBB\xBF";
    let rest = b"<?xml version='1.0'?><r/>";
    let mut r = Parser::new(None).unwrap();
    r.parse(bom, false);
    let rs = r.parse(rest, true) as u32;
    let c = CParser::new(None).unwrap();
    c.parse(bom, false);
    let (cs, _) = c.parse(rest, true);
    assert_eq!(rs, cs, "BOM exact boundary");
}

// 15. CDATA partial at boundary (covers 1874-1877)
#[test]
fn final90_cdata_partial_boundary() {
    // CDATA section split so the ]]> is partial
    let xml = b"<r><![CDATA[text]]></r>";
    // Split right before ]]> — positions 15,16,17 are ], ], >
    for split in 15..=17 {
        let mut r = Parser::new(None).unwrap();
        r.parse(&xml[..split], false);
        let rs = r.parse(&xml[split..], true) as u32;
        let c = CParser::new(None).unwrap();
        c.parse(&xml[..split], false);
        let (cs, _) = c.parse(&xml[split..], true);
        assert_eq!(rs, cs, "cdata partial @{split}");
    }
}

// 16. Attribute value tokenizer — entity ref at exact split (covers 2115-2118, 2136-2140)
#[test]
fn final90_attr_entity_at_boundary() {
    // Attribute with entity ref, incremental — hits attribute_value_tok partial paths
    compare_incr(b"<r a=\"text&amp;more&lt;end\"/>", "attr entity boundary");
    compare_incr(b"<r a='text&amp;more&lt;end'/>", "attr entity boundary sq");
}

// 17. Entity value tokenizer — LF/CR at start (covers 2146-2177)
#[test]
fn final90_entity_value_cr_lf_at_start() {
    // Entity values where \r or \n are at exact chunk split boundaries
    let cases: &[&[u8]] = &[
        b"<!DOCTYPE r [<!ENTITY e '\rtext'>]><r>&e;</r>",
        b"<!DOCTYPE r [<!ENTITY e '\ntext'>]><r>&e;</r>",
        b"<!DOCTYPE r [<!ENTITY e '\r\ntext'>]><r>&e;</r>",
    ];
    for case in cases {
        compare_incr(
            case,
            &format!("ev_cr_start {:?}", std::str::from_utf8(case).unwrap()),
        );
    }
}

// 18. Scan percent in prolog (covers 248-254, 1594-1622)
#[test]
fn final90_scan_percent_incremental() {
    compare_incr(
        b"<!DOCTYPE r [<!ENTITY % pe 'val'><!ELEMENT r EMPTY>]><r/>",
        "percent entity incremental",
    );
}

// 19. Element content model with #PCDATA at nested level (covers xmlrole 942-944, 1004-1006)
#[test]
fn final90_pcdata_nested() {
    compare_incr(
        b"<!DOCTYPE r [<!ELEMENT r (#PCDATA|a|b)*><!ELEMENT a EMPTY><!ELEMENT b (#PCDATA)>]><r>text<a/><b>inner</b></r>",
        "PCDATA nested model"
    );
    // Open paren in nested group
    compare_incr(
        b"<!DOCTYPE r [<!ELEMENT r ((a|b),(c|d))><!ELEMENT a EMPTY><!ELEMENT b EMPTY><!ELEMENT c EMPTY><!ELEMENT d EMPTY>]><r><a/><c/></r>",
        "nested group open paren"
    );
}

// 20. scan_end_tag whitespace (covers 759-761)
#[test]
fn final90_end_tag_whitespace() {
    // End tag with whitespace before >
    compare_incr(b"<r></r >", "end tag trailing space");
    compare_incr(b"<r></r\n>", "end tag trailing newline");
    compare_incr(b"<r></r\t>", "end tag trailing tab");
}

// 21. parse_buffer basic usage
#[test]
fn final90_parse_buffer_usage() {
    // parse_buffer delegates to parse, so just verify it works
    let mut r = Parser::new(None).unwrap();
    // parse_buffer with 0 bytes non-final should be OK
    let rs = r.parse_buffer(0, false);
    assert_eq!(rs, XmlStatus::Ok);
    // Then parse normally
    let rs2 = r.parse(b"<r/>", true) as u32;
    let c = CParser::new(None).unwrap();
    let (cs, _) = c.parse(b"<r/>", true);
    assert_eq!(rs2, cs, "parse after parse_buffer");
}
