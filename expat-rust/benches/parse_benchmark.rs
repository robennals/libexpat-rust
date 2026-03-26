//! Criterion benchmarks comparing Rust (expat_rust) vs C (expat_sys) XML parsing.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::ffi::c_char;
use std::ptr;

// ---------------------------------------------------------------------------
// XML document generators
// ---------------------------------------------------------------------------

fn small_document() -> Vec<u8> {
    b"<root><child attr=\"val\">text</child></root>".to_vec()
}

fn medium_document() -> Vec<u8> {
    // ~10 KB XML with many elements and attributes
    let mut xml = String::with_capacity(12_000);
    xml.push_str("<root>\n");
    for i in 0..200 {
        xml.push_str(&format!(
            "  <item id=\"{}\" name=\"item{}\" category=\"cat{}\" status=\"active\">\n",
            i,
            i,
            i % 10
        ));
        xml.push_str(&format!(
            "    <description>This is the description for item number {}</description>\n",
            i
        ));
        xml.push_str("  </item>\n");
    }
    xml.push_str("</root>");
    xml.into_bytes()
}

fn large_document() -> Vec<u8> {
    // ~100 KB XML
    let mut xml = String::with_capacity(110_000);
    xml.push_str("<root>\n");
    for i in 0..1500 {
        xml.push_str(&format!(
            "  <record id=\"{}\" type=\"type{}\" priority=\"{}\" enabled=\"true\" version=\"1.0\">\n",
            i,
            i % 20,
            i % 5
        ));
        xml.push_str(&format!(
            "    <field1>Value for field one in record {}</field1>\n",
            i
        ));
        xml.push_str(&format!(
            "    <field2>Another value for record {} with more text</field2>\n",
            i
        ));
        xml.push_str("  </record>\n");
    }
    xml.push_str("</root>");
    xml.into_bytes()
}

fn very_large_document() -> Vec<u8> {
    // ~100 MB XML — realistic streaming workload
    let mut xml = String::with_capacity(105_000_000);
    xml.push_str("<dataset>\n");
    for i in 0..450_000 {
        xml.push_str(&format!(
            "  <record id=\"{}\" type=\"type{}\" priority=\"{}\" enabled=\"true\" version=\"1.0\">\n",
            i,
            i % 20,
            i % 5
        ));
        xml.push_str(&format!(
            "    <field1>Value for field one in record {}</field1>\n",
            i
        ));
        xml.push_str(&format!(
            "    <field2>Another value for record {} with more text content here</field2>\n",
            i
        ));
        xml.push_str("  </record>\n");
    }
    xml.push_str("</dataset>");
    xml.into_bytes()
}

fn parse_rust_streaming(xml: &[u8], chunk_size: usize) {
    use expat_rust::xmlparse::{Parser, XmlStatus};
    let mut parser = Parser::new(None).unwrap();
    parser.set_start_element_handler(Some(Box::new(|_name, _attrs| {})));
    parser.set_end_element_handler(Some(Box::new(|_name| {})));
    parser.set_character_data_handler(Some(Box::new(|_data| {})));
    let chunks: Vec<&[u8]> = xml.chunks(chunk_size).collect();
    let last = chunks.len() - 1;
    for (i, chunk) in chunks.iter().enumerate() {
        let is_final = i == last;
        let status = parser.parse(chunk, is_final);
        assert_eq!(status, XmlStatus::Ok);
    }
}

fn parse_c_streaming(xml: &[u8], chunk_size: usize) {
    unsafe {
        let parser = expat_sys::XML_ParserCreate(ptr::null());
        assert!(!parser.is_null());
        expat_sys::XML_SetElementHandler(parser, Some(c_start_handler), Some(c_end_handler));
        expat_sys::XML_SetCharacterDataHandler(parser, Some(c_chardata_handler));
        let chunks: Vec<&[u8]> = xml.chunks(chunk_size).collect();
        let last = chunks.len() - 1;
        for (i, chunk) in chunks.iter().enumerate() {
            let is_final = if i == last { 1 } else { 0 };
            let status = expat_sys::XML_Parse(
                parser,
                chunk.as_ptr() as *const c_char,
                chunk.len() as std::ffi::c_int,
                is_final,
            );
            assert_eq!(status, expat_sys::XML_STATUS_OK);
        }
        expat_sys::XML_ParserFree(parser);
    }
}

fn deep_nesting_document() -> Vec<u8> {
    let depth = 100;
    let mut xml = String::with_capacity(depth * 30);
    for i in 0..depth {
        xml.push_str(&format!("<level{}>", i));
    }
    xml.push_str("leaf");
    for i in (0..depth).rev() {
        xml.push_str(&format!("</level{}>", i));
    }
    xml.into_bytes()
}

fn many_attributes_document() -> Vec<u8> {
    let mut xml = String::with_capacity(8_000);
    xml.push_str("<root>\n");
    for i in 0..10 {
        xml.push_str(&format!("  <element id=\"{}\"", i));
        for a in 0..25 {
            xml.push_str(&format!(" attr{}=\"value{}\"", a, a));
        }
        xml.push_str(">content</element>\n");
    }
    xml.push_str("</root>");
    xml.into_bytes()
}

fn malformed_document() -> Vec<u8> {
    // Malformed: mismatched closing tag
    b"<root><child attr=\"val\">text</wrong></root>".to_vec()
}

// ---------------------------------------------------------------------------
// Rust parser helpers
// ---------------------------------------------------------------------------

fn parse_rust_ok(xml: &[u8]) {
    use expat_rust::xmlparse::{Parser, XmlStatus};
    let mut parser = Parser::new(None).unwrap();
    parser.set_start_element_handler(Some(Box::new(|_name, _attrs| {})));
    parser.set_end_element_handler(Some(Box::new(|_name| {})));
    parser.set_character_data_handler(Some(Box::new(|_data| {})));
    let status = parser.parse(xml, true);
    assert_eq!(status, XmlStatus::Ok);
}

fn parse_rust_error(xml: &[u8]) {
    use expat_rust::xmlparse::{Parser, XmlStatus};
    let mut parser = Parser::new(None).unwrap();
    parser.set_start_element_handler(Some(Box::new(|_name, _attrs| {})));
    parser.set_end_element_handler(Some(Box::new(|_name| {})));
    parser.set_character_data_handler(Some(Box::new(|_data| {})));
    let status = parser.parse(xml, true);
    assert_eq!(status, XmlStatus::Error);
}

// ---------------------------------------------------------------------------
// C parser helpers (via expat_sys FFI)
// ---------------------------------------------------------------------------

unsafe extern "C" fn c_start_handler(
    _user_data: *mut std::ffi::c_void,
    _name: *const c_char,
    _attrs: *mut *const c_char,
) {
}

unsafe extern "C" fn c_end_handler(_user_data: *mut std::ffi::c_void, _name: *const c_char) {}

unsafe extern "C" fn c_chardata_handler(
    _user_data: *mut std::ffi::c_void,
    _s: *const c_char,
    _len: std::ffi::c_int,
) {
}

fn parse_c_ok(xml: &[u8]) {
    unsafe {
        let parser = expat_sys::XML_ParserCreate(ptr::null());
        assert!(!parser.is_null());
        expat_sys::XML_SetElementHandler(parser, Some(c_start_handler), Some(c_end_handler));
        expat_sys::XML_SetCharacterDataHandler(parser, Some(c_chardata_handler));
        let status = expat_sys::XML_Parse(
            parser,
            xml.as_ptr() as *const c_char,
            xml.len() as std::ffi::c_int,
            1,
        );
        assert_eq!(status, expat_sys::XML_STATUS_OK);
        expat_sys::XML_ParserFree(parser);
    }
}

fn parse_c_error(xml: &[u8]) {
    unsafe {
        let parser = expat_sys::XML_ParserCreate(ptr::null());
        assert!(!parser.is_null());
        expat_sys::XML_SetElementHandler(parser, Some(c_start_handler), Some(c_end_handler));
        expat_sys::XML_SetCharacterDataHandler(parser, Some(c_chardata_handler));
        let status = expat_sys::XML_Parse(
            parser,
            xml.as_ptr() as *const c_char,
            xml.len() as std::ffi::c_int,
            1,
        );
        assert_eq!(status, expat_sys::XML_STATUS_ERROR);
        expat_sys::XML_ParserFree(parser);
    }
}

// ---------------------------------------------------------------------------
// Benchmark groups
// ---------------------------------------------------------------------------

fn bench_small(c: &mut Criterion) {
    let xml = small_document();
    let mut group = c.benchmark_group("small_document");
    group.bench_function("rust", |b| b.iter(|| parse_rust_ok(black_box(&xml))));
    group.bench_function("c", |b| b.iter(|| parse_c_ok(black_box(&xml))));
    group.finish();
}

fn bench_medium(c: &mut Criterion) {
    let xml = medium_document();
    let mut group = c.benchmark_group("medium_document");
    group.bench_function("rust", |b| b.iter(|| parse_rust_ok(black_box(&xml))));
    group.bench_function("c", |b| b.iter(|| parse_c_ok(black_box(&xml))));
    group.finish();
}

fn bench_large(c: &mut Criterion) {
    let xml = large_document();
    let mut group = c.benchmark_group("large_document");
    group.bench_function("rust", |b| b.iter(|| parse_rust_ok(black_box(&xml))));
    group.bench_function("c", |b| b.iter(|| parse_c_ok(black_box(&xml))));
    group.finish();
}

fn bench_deep_nesting(c: &mut Criterion) {
    let xml = deep_nesting_document();
    let mut group = c.benchmark_group("deep_nesting");
    group.bench_function("rust", |b| b.iter(|| parse_rust_ok(black_box(&xml))));
    group.bench_function("c", |b| b.iter(|| parse_c_ok(black_box(&xml))));
    group.finish();
}

fn bench_many_attributes(c: &mut Criterion) {
    let xml = many_attributes_document();
    let mut group = c.benchmark_group("many_attributes");
    group.bench_function("rust", |b| b.iter(|| parse_rust_ok(black_box(&xml))));
    group.bench_function("c", |b| b.iter(|| parse_c_ok(black_box(&xml))));
    group.finish();
}

fn bench_error_handling(c: &mut Criterion) {
    let xml = malformed_document();
    let mut group = c.benchmark_group("error_handling");
    group.bench_function("rust", |b| b.iter(|| parse_rust_error(black_box(&xml))));
    group.bench_function("c", |b| b.iter(|| parse_c_error(black_box(&xml))));
    group.finish();
}

fn bench_very_large(c: &mut Criterion) {
    let xml = very_large_document();
    let size_mb = xml.len() as f64 / 1_000_000.0;
    eprintln!("Very large document: {:.1} MB", size_mb);
    let mut group = c.benchmark_group("very_large_document_100mb");
    group.sample_size(10); // fewer iterations for 100MB doc
    group.bench_function("rust", |b| b.iter(|| parse_rust_ok(black_box(&xml))));
    group.bench_function("c", |b| b.iter(|| parse_c_ok(black_box(&xml))));
    group.finish();
}

fn bench_streaming_chunks(c: &mut Criterion) {
    let xml = very_large_document();
    let chunk_size = 8192; // 8 KB chunks — typical streaming read size
    let mut group = c.benchmark_group("streaming_8kb_chunks_100mb");
    group.sample_size(10);
    group.bench_function("rust", |b| {
        b.iter(|| parse_rust_streaming(black_box(&xml), chunk_size))
    });
    group.bench_function("c", |b| {
        b.iter(|| parse_c_streaming(black_box(&xml), chunk_size))
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_small,
    bench_medium,
    bench_large,
    bench_deep_nesting,
    bench_many_attributes,
    bench_error_handling,
    bench_very_large,
    bench_streaming_chunks,
);
criterion_main!(benches);
