//! Pathological benchmarks designed to expose extreme Rust-vs-C slowdowns.
//!
//! These inputs target known overhead patterns in the Rust port:
//! - Buffer cloning on every parse() call (quadratic with long tokens + streaming)
//! - O(n²) namespace attribute duplicate checking
//! - Excessive String allocations in attribute/namespace processing
//! - attrs.remove() causing O(n²) shifting in namespace processing

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::ffi::c_char;
use std::ptr;

// ---------------------------------------------------------------------------
// Pathological XML generators
// ---------------------------------------------------------------------------

/// A very long comment parsed in small streaming chunks.
/// The comment can't be tokenized until `-->` is seen, so the buffer
/// accumulates the entire comment, and gets cloned on every parse() call.
/// This triggers quadratic behavior: O(n²/chunk_size) total bytes copied.
fn long_comment(size_kb: usize) -> Vec<u8> {
    let payload_size = size_kb * 1024;
    let mut xml = Vec::with_capacity(payload_size + 100);
    xml.extend_from_slice(b"<r><!--");
    // Fill with 'x' characters (valid in comments, avoids -- sequence)
    xml.resize(xml.len() + payload_size, b'x');
    xml.extend_from_slice(b"--></r>");
    xml
}

/// Worst case: long comment with '>' in every chunk, defeating scan-resume.
/// The scan-resume optimization checks for '>' in new data; if found, it
/// falls through to the full tokenizer. This tests the O(n²) worst case.
fn long_comment_with_gt(size_kb: usize) -> Vec<u8> {
    let payload_size = size_kb * 1024;
    let mut xml = Vec::with_capacity(payload_size + 100);
    xml.extend_from_slice(b"<r><!--");
    // Fill with pattern that has '>' every ~100 bytes (defeats scan-resume)
    for i in 0..payload_size {
        if i % 100 == 99 {
            xml.push(b'>'); // valid inside a comment (only -- is forbidden)
        } else {
            xml.push(b'x');
        }
    }
    xml.extend_from_slice(b"--></r>");
    xml
}

/// A very long CDATA section — same quadratic buffer issue as comments.
fn long_cdata(size_kb: usize) -> Vec<u8> {
    let payload_size = size_kb * 1024;
    let mut xml = Vec::with_capacity(payload_size + 100);
    xml.extend_from_slice(b"<r><![CDATA[");
    xml.resize(xml.len() + payload_size, b'a');
    xml.extend_from_slice(b"]]></r>");
    xml
}

/// A very long attribute value parsed in streaming chunks.
/// The start tag can't complete until `>` is seen.
fn long_attribute_value(size_kb: usize) -> Vec<u8> {
    let payload_size = size_kb * 1024;
    let mut xml = Vec::with_capacity(payload_size + 100);
    xml.extend_from_slice(b"<r a=\"");
    // Fill with 'v' (valid attribute value char)
    xml.resize(xml.len() + payload_size, b'v');
    xml.extend_from_slice(b"\"/>");
    xml
}

/// A very long processing instruction.
fn long_pi(size_kb: usize) -> Vec<u8> {
    let payload_size = size_kb * 1024;
    let mut xml = Vec::with_capacity(payload_size + 100);
    xml.extend_from_slice(b"<r><?target ");
    xml.resize(xml.len() + payload_size, b'x');
    xml.extend_from_slice(b"?></r>");
    xml
}

/// Many attributes on a single element — tests O(n²) duplicate checking.
fn many_attrs_single_element(n_attrs: usize) -> Vec<u8> {
    let mut xml = String::with_capacity(n_attrs * 30);
    xml.push_str("<r");
    for i in 0..n_attrs {
        xml.push_str(&format!(" a{}=\"v\"", i));
    }
    xml.push_str("/>");
    xml.into_bytes()
}

/// Many namespace declarations + many prefixed attributes.
/// Triggers O(n²) in process_namespaces duplicate check AND
/// O(n²) from attrs.remove() shifting.
fn many_ns_attrs(n_ns: usize, n_attrs_per_ns: usize) -> Vec<u8> {
    let mut xml = String::with_capacity(n_ns * n_attrs_per_ns * 60);
    xml.push_str("<r");
    // Declare namespaces
    for i in 0..n_ns {
        xml.push_str(&format!(" xmlns:ns{}=\"http://example.com/ns{}\"", i, i));
    }
    // Add prefixed attributes (these trigger expand_name + duplicate check)
    for i in 0..n_ns {
        for j in 0..n_attrs_per_ns {
            xml.push_str(&format!(" ns{}:a{}=\"v\"", i, j));
        }
    }
    xml.push_str("/>");
    xml.into_bytes()
}

/// Deep nesting with namespace declarations at each level.
fn deep_ns_nesting(depth: usize) -> Vec<u8> {
    let mut xml = String::with_capacity(depth * 100);
    for i in 0..depth {
        xml.push_str(&format!(
            "<ns{}:e xmlns:ns{}=\"http://example.com/{}\">",
            i, i, i
        ));
    }
    xml.push_str("leaf");
    for i in (0..depth).rev() {
        xml.push_str(&format!("</ns{}:e>", i));
    }
    xml.into_bytes()
}

/// Many small elements parsed in tiny chunks — maximizes parse() call overhead
/// (buffer clone per call).
fn many_tiny_elements(n: usize) -> Vec<u8> {
    let mut xml = Vec::with_capacity(n * 10);
    xml.extend_from_slice(b"<r>");
    for _ in 0..n {
        xml.extend_from_slice(b"<e/>");
    }
    xml.extend_from_slice(b"</r>");
    xml
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

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
        assert_eq!(status, XmlStatus::Ok, "Rust parse failed at chunk {}", i);
    }
}

fn parse_rust_ns_streaming(xml: &[u8], chunk_size: usize) {
    use expat_rust::xmlparse::{Parser, XmlStatus};
    let mut parser = Parser::new_ns(None, '\n').unwrap();
    parser.set_start_element_handler(Some(Box::new(|_name, _attrs| {})));
    parser.set_end_element_handler(Some(Box::new(|_name| {})));
    parser.set_character_data_handler(Some(Box::new(|_data| {})));
    let chunks: Vec<&[u8]> = xml.chunks(chunk_size).collect();
    let last = chunks.len() - 1;
    for (i, chunk) in chunks.iter().enumerate() {
        let is_final = i == last;
        let status = parser.parse(chunk, is_final);
        assert_eq!(status, XmlStatus::Ok, "Rust NS parse failed at chunk {}", i);
    }
}

fn parse_rust_one_shot(xml: &[u8]) {
    use expat_rust::xmlparse::{Parser, XmlStatus};
    let mut parser = Parser::new(None).unwrap();
    parser.set_start_element_handler(Some(Box::new(|_name, _attrs| {})));
    parser.set_end_element_handler(Some(Box::new(|_name| {})));
    parser.set_character_data_handler(Some(Box::new(|_data| {})));
    let status = parser.parse(xml, true);
    assert_eq!(status, XmlStatus::Ok);
}

fn parse_rust_ns_one_shot(xml: &[u8]) {
    use expat_rust::xmlparse::{Parser, XmlStatus};
    let mut parser = Parser::new_ns(None, '\n').unwrap();
    parser.set_start_element_handler(Some(Box::new(|_name, _attrs| {})));
    parser.set_end_element_handler(Some(Box::new(|_name| {})));
    parser.set_character_data_handler(Some(Box::new(|_data| {})));
    let status = parser.parse(xml, true);
    assert_eq!(status, XmlStatus::Ok);
}

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
            assert_eq!(
                status,
                expat_sys::XML_STATUS_OK,
                "C parse failed at chunk {}",
                i
            );
        }
        expat_sys::XML_ParserFree(parser);
    }
}

fn parse_c_ns_streaming(xml: &[u8], chunk_size: usize) {
    unsafe {
        let sep = b"\n\0";
        let parser = expat_sys::XML_ParserCreateNS(ptr::null(), sep[0] as c_char);
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
            assert_eq!(
                status,
                expat_sys::XML_STATUS_OK,
                "C NS parse failed at chunk {}",
                i
            );
        }
        expat_sys::XML_ParserFree(parser);
    }
}

fn parse_c_one_shot(xml: &[u8]) {
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

fn parse_c_ns_one_shot(xml: &[u8]) {
    unsafe {
        let sep = b"\n\0";
        let parser = expat_sys::XML_ParserCreateNS(ptr::null(), sep[0] as c_char);
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

// ---------------------------------------------------------------------------
// Benchmark groups
// ---------------------------------------------------------------------------

/// Test streaming with long incomplete tokens (quadratic buffer clone).
fn bench_long_tokens_streaming(c: &mut Criterion) {
    let chunk = 4096; // 4KB chunks — smaller chunks = more parse() calls = worse

    for &size_kb in &[64, 256, 1024] {
        let label = format!("long_comment_{}kb_stream_4k", size_kb);
        let xml = long_comment(size_kb);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| {
            b.iter(|| parse_rust_streaming(black_box(&xml), chunk))
        });
        group.bench_function("c", |b| {
            b.iter(|| parse_c_streaming(black_box(&xml), chunk))
        });
        group.finish();
    }

    for &size_kb in &[64, 256, 1024] {
        let label = format!("long_cdata_{}kb_stream_4k", size_kb);
        let xml = long_cdata(size_kb);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| {
            b.iter(|| parse_rust_streaming(black_box(&xml), chunk))
        });
        group.bench_function("c", |b| {
            b.iter(|| parse_c_streaming(black_box(&xml), chunk))
        });
        group.finish();
    }

    for &size_kb in &[64, 256, 1024] {
        let label = format!("long_attr_{}kb_stream_4k", size_kb);
        let xml = long_attribute_value(size_kb);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| {
            b.iter(|| parse_rust_streaming(black_box(&xml), chunk))
        });
        group.bench_function("c", |b| {
            b.iter(|| parse_c_streaming(black_box(&xml), chunk))
        });
        group.finish();
    }

    for &size_kb in &[64, 256, 1024] {
        let label = format!("long_pi_{}kb_stream_4k", size_kb);
        let xml = long_pi(size_kb);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| {
            b.iter(|| parse_rust_streaming(black_box(&xml), chunk))
        });
        group.bench_function("c", |b| {
            b.iter(|| parse_c_streaming(black_box(&xml), chunk))
        });
        group.finish();
    }
}

/// Compare one-shot vs streaming to isolate per-call overhead.
fn bench_long_tokens_oneshot(c: &mut Criterion) {
    for &size_kb in &[64, 256, 1024] {
        let label = format!("long_comment_{}kb_oneshot", size_kb);
        let xml = long_comment(size_kb);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| b.iter(|| parse_rust_one_shot(black_box(&xml))));
        group.bench_function("c", |b| b.iter(|| parse_c_one_shot(black_box(&xml))));
        group.finish();
    }
}

/// Test worst case: long comment with '>' defeating scan-resume.
fn bench_long_comment_with_gt(c: &mut Criterion) {
    let chunk = 4096;
    for &size_kb in &[64, 256, 1024] {
        let label = format!("long_comment_gt_{}kb_stream_4k", size_kb);
        let xml = long_comment_with_gt(size_kb);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| {
            b.iter(|| parse_rust_streaming(black_box(&xml), chunk))
        });
        group.bench_function("c", |b| {
            b.iter(|| parse_c_streaming(black_box(&xml), chunk))
        });
        group.finish();
    }
}

/// Test O(n²) namespace attribute duplicate checking.
fn bench_many_ns_attrs(c: &mut Criterion) {
    for &n in &[50, 200, 500] {
        let label = format!("ns_attrs_{}_attrs", n);
        let xml = many_ns_attrs(n, 1);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| {
            b.iter(|| parse_rust_ns_one_shot(black_box(&xml)))
        });
        group.bench_function("c", |b| b.iter(|| parse_c_ns_one_shot(black_box(&xml))));
        group.finish();
    }
}

/// Test many plain attributes (O(n²) duplicate check in extract_attrs).
fn bench_many_plain_attrs(c: &mut Criterion) {
    // Note: C expat also has O(n²) duplicate check so ratio may be similar
    for &n in &[50, 200, 500] {
        let label = format!("plain_attrs_{}", n);
        let xml = many_attrs_single_element(n);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| b.iter(|| parse_rust_one_shot(black_box(&xml))));
        group.bench_function("c", |b| b.iter(|| parse_c_one_shot(black_box(&xml))));
        group.finish();
    }
}

/// Test deep nesting with namespaces.
fn bench_deep_ns(c: &mut Criterion) {
    for &depth in &[100, 500, 1000] {
        let label = format!("deep_ns_{}", depth);
        let xml = deep_ns_nesting(depth);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| {
            b.iter(|| parse_rust_ns_one_shot(black_box(&xml)))
        });
        group.bench_function("c", |b| b.iter(|| parse_c_ns_one_shot(black_box(&xml))));
        group.finish();
    }
}

/// Test tiny-chunk streaming to maximize per-call overhead.
fn bench_tiny_chunks(c: &mut Criterion) {
    let xml = many_tiny_elements(10_000);
    for &chunk in &[64, 256, 1024] {
        let label = format!("tiny_elements_10k_chunk_{}", chunk);
        let mut group = c.benchmark_group(&label);
        group.sample_size(10);
        group.bench_function("rust", |b| {
            b.iter(|| parse_rust_streaming(black_box(&xml), chunk))
        });
        group.bench_function("c", |b| {
            b.iter(|| parse_c_streaming(black_box(&xml), chunk))
        });
        group.finish();
    }
}

criterion_group!(
    benches,
    bench_long_tokens_streaming,
    bench_long_tokens_oneshot,
    bench_long_comment_with_gt,
    bench_many_ns_attrs,
    bench_many_plain_attrs,
    bench_deep_ns,
    bench_tiny_chunks,
);
criterion_main!(benches);
