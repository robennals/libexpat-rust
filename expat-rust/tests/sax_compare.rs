//! Shared SAX event comparison infrastructure for C vs Rust parser tests.
//!
//! Provides `compare()`, `compare_incr()`, `compare_incremental()`, and `compare_ns()`
//! that verify both parsers produce identical status codes AND identical SAX event sequences.
//!
//! Usage in test files:
//!   mod sax_compare;
//!   use sax_compare::{compare, compare_incr};

use expat_rust::xmlparse::{Parser, XmlStatus};
use expat_sys::CParser;
use std::cell::RefCell;
use std::ffi::{c_char, c_int, c_void, CStr};

/// Collect all SAX events from the Rust parser.
/// Returns (status, error_code, events).
fn collect_rust_events(xml: &[u8]) -> (u32, u32, Vec<String>) {
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
        let text = std::str::from_utf8(data).unwrap_or("<binary>");
        (*ev3).borrow_mut().push(format!("CD:{}", text));
    })));

    let ev4 = ev;
    parser.set_processing_instruction_handler(Some(Box::new(move |target, data| unsafe {
        (*ev4).borrow_mut().push(format!("PI:{}:{}", target, data));
    })));

    let ev5 = ev;
    parser.set_comment_handler(Some(Box::new(move |text: &[u8]| unsafe {
        let t = std::str::from_utf8(text).unwrap_or("<binary>");
        (*ev5).borrow_mut().push(format!("CM:{}", t));
    })));

    let ev6 = ev;
    parser.set_start_cdata_section_handler(Some(Box::new(move || unsafe {
        (*ev6).borrow_mut().push("SCD".to_string());
    })));

    let ev7 = ev;
    parser.set_end_cdata_section_handler(Some(Box::new(move || unsafe {
        (*ev7).borrow_mut().push("ECD".to_string());
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
        (*ev9).borrow_mut().push("EDT".to_string());
    })));

    let status = parser.parse(xml, true);
    let error = parser.error_code();
    (status as u32, error as u32, events.into_inner())
}

/// Collect all SAX events from the C parser.
/// Returns (status, error_code, events).
fn collect_c_events(xml: &[u8]) -> (u32, u32, Vec<String>) {
    let events: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let c_parser = CParser::new(None).unwrap();

    unsafe extern "C" fn c_start_el(
        ud: *mut c_void,
        name: *const c_char,
        atts: *mut *const c_char,
    ) {
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

    let (status, error) = c_parser.parse(xml, true);
    (status, error, events.into_inner())
}

/// Merge adjacent CD: events into one (SAX allows splitting character data differently).
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

/// Compare full SAX event sequences and status/error codes between C and Rust parsers.
pub fn compare(xml: &[u8], desc: &str) {
    let (rs, re, r_events) = collect_rust_events(xml);
    let (cs, ce, c_events) = collect_c_events(xml);

    let r_merged = merge_chardata(&r_events);
    let c_merged = merge_chardata(&c_events);

    assert!(
        rs == cs && re == ce && r_merged == c_merged,
        "MISMATCH {desc}:\n  status: R={rs} C={cs}\n  error:  R={re} C={ce}\n  R events: {:?}\n  C events: {:?}\n  input: {:?}",
        r_merged, c_merged,
        std::str::from_utf8(xml).unwrap_or("<binary>")
    );
}

/// Compare incremental parsing: full SAX comparison on single-shot, then split at every
/// byte position checking status/error codes match.
#[allow(dead_code)]
pub fn compare_incr(xml: &[u8], desc: &str) {
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

/// Alias for compare_incr (used by some test files).
#[allow(dead_code)]
pub fn compare_incremental(xml: &[u8], desc: &str) {
    compare_incr(xml, desc);
}

/// Namespace-aware comparison (currently uses non-NS parser, still checks full SAX events).
#[allow(dead_code)]
pub fn compare_ns(xml: &[u8], desc: &str) {
    compare(xml, desc);
}
