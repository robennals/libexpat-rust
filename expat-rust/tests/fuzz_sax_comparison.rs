//! Check if any fuzz corpus files produce OK from both parsers but different
//! SAX event traces. This catches silent parse divergence — the scariest kind.
//!
//! Only compares files where BOTH parsers return OK (status=1).

use expat_rust::xmlparse::{Parser, XmlStatus};
use expat_sys::*;
use std::cell::RefCell;
use std::ffi::{c_char, c_int, c_void, CStr};
use std::path::Path;

fn collect_corpus_files(dir: &Path) -> Vec<std::path::PathBuf> {
    if !dir.is_dir() {
        return Vec::new();
    }
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    files.sort();
    files
}

fn collect_rust_events(xml: &[u8], encoding: Option<&str>) -> (XmlStatus, Vec<String>) {
    let events: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let mut parser = Parser::new(encoding).unwrap();

    let ev = &events as *const RefCell<Vec<String>>;

    parser.set_start_element_handler(Some(Box::new(move |name, attrs| unsafe {
        let mut s = format!("SE:{}", name);
        for (k, v) in attrs {
            s.push_str(&format!(" {}={}", k, v));
        }
        (*ev).borrow_mut().push(s);
    })));
    let ev2 = &events as *const RefCell<Vec<String>>;
    parser.set_end_element_handler(Some(Box::new(move |name| unsafe {
        (*ev2).borrow_mut().push(format!("EE:{}", name));
    })));
    let ev3 = &events as *const RefCell<Vec<String>>;
    parser.set_character_data_handler(Some(Box::new(move |data: &[u8]| unsafe {
        let text = std::str::from_utf8(data).unwrap_or("<binary>");
        (*ev3).borrow_mut().push(format!("CD:{}", text));
    })));
    let ev4 = &events as *const RefCell<Vec<String>>;
    parser.set_processing_instruction_handler(Some(Box::new(move |target, data| unsafe {
        (*ev4).borrow_mut().push(format!("PI:{}:{}", target, data));
    })));
    let ev5 = &events as *const RefCell<Vec<String>>;
    parser.set_comment_handler(Some(Box::new(move |text: &[u8]| unsafe {
        let t = std::str::from_utf8(text).unwrap_or("<binary>");
        (*ev5).borrow_mut().push(format!("CM:{}", t));
    })));

    let status = parser.parse(xml, true);
    (status, events.into_inner())
}

fn collect_c_events(xml: &[u8], encoding: Option<&str>) -> (u32, Vec<String>) {
    let events: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let c_parser = CParser::new(encoding).unwrap();

    unsafe extern "C" fn c_start_el(
        ud: *mut c_void,
        name: *const c_char,
        atts: *mut *const c_char,
    ) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let n = CStr::from_ptr(name).to_string_lossy();
        let mut s = format!("SE:{}", n);
        let mut i = 0;
        loop {
            let key = *atts.add(i);
            if key.is_null() {
                break;
            }
            let val = *atts.add(i + 1);
            let k = CStr::from_ptr(key).to_string_lossy();
            let v = CStr::from_ptr(val).to_string_lossy();
            s.push_str(&format!(" {}={}", k, v));
            i += 2;
        }
        ev.borrow_mut().push(s);
    }
    unsafe extern "C" fn c_end_el(ud: *mut c_void, name: *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let n = CStr::from_ptr(name).to_string_lossy();
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
        let t = CStr::from_ptr(target).to_string_lossy();
        let d = if data.is_null() {
            "".into()
        } else {
            CStr::from_ptr(data).to_string_lossy()
        };
        ev.borrow_mut().push(format!("PI:{}:{}", t, d));
    }
    unsafe extern "C" fn c_comment(ud: *mut c_void, text: *const c_char) {
        let ev = &*(ud as *const RefCell<Vec<String>>);
        let t = CStr::from_ptr(text).to_string_lossy();
        ev.borrow_mut().push(format!("CM:{}", t));
    }

    let ev_ptr = &events as *const RefCell<Vec<String>> as *mut c_void;
    unsafe {
        XML_SetUserData(c_parser.parser, ev_ptr);
        XML_SetElementHandler(c_parser.parser, Some(c_start_el), Some(c_end_el));
        XML_SetCharacterDataHandler(c_parser.parser, Some(c_chardata));
        XML_SetProcessingInstructionHandler(c_parser.parser, Some(c_pi));
        XML_SetCommentHandler(c_parser.parser, Some(c_comment));
    }

    let (status, _) = c_parser.parse(xml, true);
    (status, events.into_inner())
}

/// Merge adjacent CD events (SAX allows different chunking).
fn merge_chardata(events: &[String]) -> Vec<String> {
    let mut merged = Vec::new();
    let mut pending_cd = String::new();
    for ev in events {
        if let Some(text) = ev.strip_prefix("CD:") {
            pending_cd.push_str(text);
        } else {
            if !pending_cd.is_empty() {
                merged.push(format!("CD:{pending_cd}"));
                pending_cd.clear();
            }
            merged.push(ev.clone());
        }
    }
    if !pending_cd.is_empty() {
        merged.push(format!("CD:{pending_cd}"));
    }
    merged
}

fn check_sax(corpus_name: &str, encoding: Option<&str>) {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let corpus_dir = workspace_root.join("corpus").join(corpus_name);
    let files = collect_corpus_files(&corpus_dir);
    if files.is_empty() {
        eprintln!("SKIPPED: no corpus for {corpus_name}");
        return;
    }

    std::panic::set_hook(Box::new(|_| {}));

    let mut both_ok = 0usize;
    let mut sax_match = 0usize;
    let mut sax_differ = 0usize;
    let mut diffs: Vec<String> = Vec::new();

    for path in &files {
        let data = std::fs::read(path).unwrap();
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let (rs, r_events) = collect_rust_events(&data, encoding);
            let (cs, c_events) = collect_c_events(&data, encoding);
            (rs as u32, cs, r_events, c_events)
        }));

        if let Ok((rs, cs, r_events, c_events)) = result {
            if rs == 1 && cs == 1 {
                both_ok += 1;
                let r_merged = merge_chardata(&r_events);
                let c_merged = merge_chardata(&c_events);
                if r_merged == c_merged {
                    sax_match += 1;
                } else {
                    sax_differ += 1;
                    if diffs.len() < 20 {
                        let r_preview: Vec<_> = r_merged.iter().take(5).cloned().collect();
                        let c_preview: Vec<_> = c_merged.iter().take(5).cloned().collect();
                        diffs.push(format!(
                            "{name} (len={}): Rust[{}]={:?}  C[{}]={:?}",
                            data.len(),
                            r_merged.len(),
                            r_preview,
                            c_merged.len(),
                            c_preview
                        ));
                    }
                }
            }
        }
    }

    eprintln!("\n=== {corpus_name} ===");
    eprintln!("Both OK: {both_ok}, SAX match: {sax_match}, SAX differ: {sax_differ}");

    if !diffs.is_empty() {
        eprintln!("SAX divergences (first {}):", diffs.len());
        for d in &diffs {
            eprintln!("  {d}");
        }
    }

    assert_eq!(
        sax_differ, 0,
        "{sax_differ} files had SAX event divergence on valid XML"
    );
}

#[test]
fn sax_utf8() {
    check_sax("xml_parse_fuzzer_UTF-8", Some("UTF-8"));
}

#[test]
fn sax_utf16le() {
    check_sax("xml_parsebuffer_fuzzer_UTF-16LE", Some("UTF-16LE"));
}
