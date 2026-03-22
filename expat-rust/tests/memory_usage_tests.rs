//! Memory usage comparison tests: expat-rust vs C libexpat.
//!
//! Measures peak heap allocation for both implementations across various
//! document sizes, shapes, and streaming scenarios. The Rust side is tracked
//! via a custom global allocator; the C side is tracked via XML_ParserCreate_MM
//! with custom malloc/realloc/free functions.
//!
//! Run with: RUST_TEST_THREADS=1 cargo test -p expat-rust --test memory_usage_tests -- --nocapture

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::ffi::{c_char, c_int, c_void};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Tracking global allocator — measures Rust-side allocations
// ---------------------------------------------------------------------------

struct TrackingAllocator;

static RUST_ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static RUST_PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            let current =
                RUST_ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
            RUST_PEAK.fetch_max(current, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        RUST_ALLOCATED.fetch_sub(layout.size(), Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
        if !new_ptr.is_null() {
            if new_size > layout.size() {
                let diff = new_size - layout.size();
                let current = RUST_ALLOCATED.fetch_add(diff, Ordering::Relaxed) + diff;
                RUST_PEAK.fetch_max(current, Ordering::Relaxed);
            } else {
                RUST_ALLOCATED.fetch_sub(layout.size() - new_size, Ordering::Relaxed);
            }
        }
        new_ptr
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn rust_reset_tracking() {
    RUST_PEAK.store(RUST_ALLOCATED.load(Ordering::Relaxed), Ordering::Relaxed);
}

fn rust_current() -> usize {
    RUST_ALLOCATED.load(Ordering::Relaxed)
}

fn rust_peak() -> usize {
    RUST_PEAK.load(Ordering::Relaxed)
}

/// Returns (peak_delta, retained_delta) for a Rust-side operation.
fn measure_rust<F: FnOnce()>(f: F) -> (usize, usize) {
    let baseline_current = rust_current();
    rust_reset_tracking();
    let baseline_peak = rust_peak();
    f();
    let peak_delta = rust_peak().saturating_sub(baseline_peak);
    let retained_delta = rust_current().saturating_sub(baseline_current);
    (peak_delta, retained_delta)
}

// ---------------------------------------------------------------------------
// C-side allocation tracking via XML_ParserCreate_MM
// ---------------------------------------------------------------------------

thread_local! {
    static C_ALLOCATED: Cell<usize> = const { Cell::new(0) };
    static C_PEAK: Cell<usize> = const { Cell::new(0) };
}

const HEADER_SIZE: usize = 16;

unsafe extern "C" fn tracking_malloc(size: usize) -> *mut c_void {
    let total = size + HEADER_SIZE;
    let ptr = unsafe { libc::malloc(total) };
    if ptr.is_null() {
        return ptr::null_mut();
    }
    unsafe { *(ptr as *mut usize) = size };
    C_ALLOCATED.with(|a| {
        let current = a.get() + size;
        a.set(current);
        C_PEAK.with(|p| {
            if current > p.get() {
                p.set(current);
            }
        });
    });
    unsafe { (ptr as *mut u8).add(HEADER_SIZE) as *mut c_void }
}

unsafe extern "C" fn tracking_realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    if ptr.is_null() {
        return unsafe { tracking_malloc(new_size) };
    }
    let real_ptr = unsafe { (ptr as *mut u8).sub(HEADER_SIZE) as *mut c_void };
    let old_size = unsafe { *(real_ptr as *const usize) };
    let total = new_size + HEADER_SIZE;
    let new_ptr = unsafe { libc::realloc(real_ptr, total) };
    if new_ptr.is_null() {
        return ptr::null_mut();
    }
    unsafe { *(new_ptr as *mut usize) = new_size };
    C_ALLOCATED.with(|a| {
        let current = a.get() - old_size + new_size;
        a.set(current);
        C_PEAK.with(|p| {
            if current > p.get() {
                p.set(current);
            }
        });
    });
    unsafe { (new_ptr as *mut u8).add(HEADER_SIZE) as *mut c_void }
}

unsafe extern "C" fn tracking_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let real_ptr = unsafe { (ptr as *mut u8).sub(HEADER_SIZE) as *mut c_void };
    let size = unsafe { *(real_ptr as *const usize) };
    C_ALLOCATED.with(|a| a.set(a.get() - size));
    unsafe { libc::free(real_ptr) };
}

fn c_reset_tracking() {
    C_ALLOCATED.with(|a| a.set(0));
    C_PEAK.with(|p| p.set(0));
}

fn c_peak() -> usize {
    C_PEAK.with(|p| p.get())
}

fn c_current() -> usize {
    C_ALLOCATED.with(|a| a.get())
}

fn make_c_memsuite() -> expat_sys::XML_Memory_Handling_Suite {
    expat_sys::XML_Memory_Handling_Suite {
        malloc_fcn: Some(tracking_malloc),
        realloc_fcn: Some(tracking_realloc),
        free_fcn: Some(tracking_free),
    }
}

/// Returns (peak, retained) for a C-side parse using tracking allocator.
fn measure_c_oneshot(xml: &[u8]) -> (usize, usize) {
    c_reset_tracking();
    let memsuite = make_c_memsuite();
    unsafe {
        let parser =
            expat_sys::XML_ParserCreate_MM(ptr::null(), &memsuite as *const _, ptr::null());
        assert!(!parser.is_null());
        expat_sys::XML_SetElementHandler(parser, Some(c_start), Some(c_end));
        expat_sys::XML_SetCharacterDataHandler(parser, Some(c_chardata));
        let status =
            expat_sys::XML_Parse(parser, xml.as_ptr() as *const c_char, xml.len() as c_int, 1);
        assert_eq!(status, expat_sys::XML_STATUS_OK);
        let peak = c_peak();
        expat_sys::XML_ParserFree(parser);
        let retained = c_current();
        (peak, retained)
    }
}

// ---------------------------------------------------------------------------
// XML document generators
// ---------------------------------------------------------------------------

fn generate_document(num_elements: usize) -> Vec<u8> {
    let mut xml = String::with_capacity(num_elements * 120);
    xml.push_str("<root>\n");
    for i in 0..num_elements {
        xml.push_str(&format!(
            "  <item id=\"{}\" name=\"item{}\" category=\"cat{}\" status=\"active\">\n\
             \x20   <description>This is the description for item number {}</description>\n\
             \x20 </item>\n",
            i,
            i,
            i % 10,
            i
        ));
    }
    xml.push_str("</root>");
    xml.into_bytes()
}

fn deep_nesting_document(depth: usize) -> Vec<u8> {
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

fn many_attributes_document(num_elements: usize, attrs_per_element: usize) -> Vec<u8> {
    let mut xml = String::with_capacity(num_elements * attrs_per_element * 25);
    xml.push_str("<root>\n");
    for i in 0..num_elements {
        xml.push_str(&format!("  <element id=\"{}\"", i));
        for a in 0..attrs_per_element {
            xml.push_str(&format!(" attr{}=\"value{}\"", a, a));
        }
        xml.push_str(">content</element>\n");
    }
    xml.push_str("</root>");
    xml.into_bytes()
}

// ---------------------------------------------------------------------------
// Parser helpers
// ---------------------------------------------------------------------------

fn parse_rust(xml: &[u8]) {
    use expat_rust::xmlparse::{Parser, XmlStatus};
    let mut parser = Parser::new(None).unwrap();
    parser.set_start_element_handler(Some(Box::new(|_name, _attrs| {})));
    parser.set_end_element_handler(Some(Box::new(|_name| {})));
    parser.set_character_data_handler(Some(Box::new(|_data| {})));
    let status = parser.parse(xml, true);
    assert_eq!(status, XmlStatus::Ok);
}

/// Parse in chunks, feeding `chunk_size` bytes at a time.
fn parse_rust_chunked(xml: &[u8], chunk_size: usize) {
    use expat_rust::xmlparse::{Parser, XmlStatus};
    let mut parser = Parser::new(None).unwrap();
    parser.set_start_element_handler(Some(Box::new(|_name, _attrs| {})));
    parser.set_end_element_handler(Some(Box::new(|_name| {})));
    parser.set_character_data_handler(Some(Box::new(|_data| {})));
    let mut offset = 0;
    while offset < xml.len() {
        let end = (offset + chunk_size).min(xml.len());
        let is_final = end == xml.len();
        let status = parser.parse(&xml[offset..end], is_final);
        assert_eq!(status, XmlStatus::Ok);
        offset = end;
    }
}

fn parse_c_chunked(xml: &[u8], chunk_size: usize, memsuite: &expat_sys::XML_Memory_Handling_Suite) {
    unsafe {
        let parser = expat_sys::XML_ParserCreate_MM(ptr::null(), memsuite as *const _, ptr::null());
        assert!(!parser.is_null());
        expat_sys::XML_SetElementHandler(parser, Some(c_start), Some(c_end));
        expat_sys::XML_SetCharacterDataHandler(parser, Some(c_chardata));
        let mut offset = 0;
        while offset < xml.len() {
            let end = (offset + chunk_size).min(xml.len());
            let is_final = if end == xml.len() { 1 } else { 0 };
            let status = expat_sys::XML_Parse(
                parser,
                xml[offset..end].as_ptr() as *const c_char,
                (end - offset) as c_int,
                is_final,
            );
            assert_eq!(status, expat_sys::XML_STATUS_OK);
            offset = end;
        }
        expat_sys::XML_ParserFree(parser);
    }
}

unsafe extern "C" fn c_start(_: *mut c_void, _: *const c_char, _: *mut *const c_char) {}
unsafe extern "C" fn c_end(_: *mut c_void, _: *const c_char) {}
unsafe extern "C" fn c_chardata(_: *mut c_void, _: *const c_char, _: c_int) {}

// ---------------------------------------------------------------------------
// Comparison runner
// ---------------------------------------------------------------------------

struct MemoryResult {
    name: String,
    doc_size: usize,
    rust_peak: usize,
    c_peak: usize,
}

fn run_comparison(name: &str, xml: &[u8]) -> MemoryResult {
    let mut rust_peaks = Vec::new();
    for _ in 0..3 {
        let (peak, _) = measure_rust(|| parse_rust(xml));
        rust_peaks.push(peak);
    }

    let mut c_peaks = Vec::new();
    for _ in 0..3 {
        let (peak, _) = measure_c_oneshot(xml);
        c_peaks.push(peak);
    }

    MemoryResult {
        name: name.to_string(),
        doc_size: xml.len(),
        rust_peak: *rust_peaks.iter().min().unwrap(),
        c_peak: *c_peaks.iter().min().unwrap(),
    }
}

fn fmt_bytes(n: usize) -> String {
    if n >= 1_073_741_824 {
        format!("{:.1} GB", n as f64 / 1_073_741_824.0)
    } else if n >= 1_048_576 {
        format!("{:.1} MB", n as f64 / 1_048_576.0)
    } else if n >= 1024 {
        format!("{:.1} KB", n as f64 / 1024.0)
    } else {
        format!("{} B", n)
    }
}

fn print_table(title: &str, results: &[MemoryResult]) {
    println!();
    println!("{}", title);
    println!(
        "  {:<28} {:>10} {:>14} {:>14} {:>8}",
        "Scenario", "Doc Size", "Rust Peak", "C Peak", "Ratio"
    );
    println!("  {}", "-".repeat(78));
    for r in results {
        let ratio = if r.c_peak > 0 {
            r.rust_peak as f64 / r.c_peak as f64
        } else {
            f64::INFINITY
        };
        println!(
            "  {:<28} {:>10} {:>14} {:>14} {:>7.2}x",
            r.name,
            fmt_bytes(r.doc_size),
            fmt_bytes(r.rust_peak),
            fmt_bytes(r.c_peak),
            ratio
        );
    }
    println!();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Compare peak memory across document sizes from 44 bytes to 10 MB (one-shot),
/// plus streaming tests for larger documents.
#[test]
fn memory_usage_comparison() {
    let scenarios: Vec<(&str, Vec<u8>)> = vec![
        (
            "44 B  flat",
            b"<root><child attr=\"val\">text</child></root>".to_vec(),
        ),
        ("10 KB flat", generate_document(200)),
        ("100 KB flat", generate_document(1_500)),
        ("1 MB flat", generate_document(15_000)),
        ("10 MB flat", generate_document(150_000)),
        ("deep nesting 100", deep_nesting_document(100)),
        ("deep nesting 1000", deep_nesting_document(1000)),
        ("many attrs 25/elem", many_attributes_document(50, 25)),
        ("many attrs 100/elem", many_attributes_document(50, 100)),
    ];

    let mut results = Vec::new();
    for (name, xml) in &scenarios {
        results.push(run_comparison(name, xml));
    }

    print_table("Peak memory: expat-rust vs C libexpat", &results);

    // Assertions: Rust should not use more than 5x the memory of C
    for r in &results {
        if r.c_peak > 0 {
            let ratio = r.rust_peak as f64 / r.c_peak as f64;
            assert!(
                ratio < 5.0,
                "Rust uses {:.1}x more memory than C for '{}' ({} vs {})",
                ratio,
                r.name,
                fmt_bytes(r.rust_peak),
                fmt_bytes(r.c_peak)
            );
        }
    }
}

/// Streaming: feed large documents in small chunks.
/// Memory should stay bounded regardless of total bytes parsed,
/// since expat is a streaming parser that doesn't buffer the whole document.
///
/// This is the most important test — it proves that both implementations
/// can parse arbitrarily large inputs with bounded memory.
#[test]
fn streaming_memory_bounded() {
    // Generate documents of increasing size and stream them through
    // with a fixed 8 KB chunk size.
    let chunk_size = 8192;
    let doc_sizes = [
        ("10 MB", 150_000),
        ("50 MB", 750_000),
        ("200 MB", 3_000_000),
    ];

    println!();
    println!("Streaming memory ({} chunks):", fmt_bytes(chunk_size));
    println!(
        "  {:<12} {:>12} {:>14} {:>14} {:>8}",
        "Target Size", "Actual Size", "Rust Peak", "C Peak", "Ratio"
    );
    println!("  {}", "-".repeat(64));

    let mut prev_rust_peak = 0;

    for (label, num_elements) in &doc_sizes {
        let xml = generate_document(*num_elements);
        let total_bytes = xml.len();

        // Measure Rust
        let (rust_peak, _) = measure_rust(|| parse_rust_chunked(&xml, chunk_size));

        // Measure C
        c_reset_tracking();
        let memsuite = make_c_memsuite();
        parse_c_chunked(&xml, chunk_size, &memsuite);
        let c_peak_val = c_peak();

        let ratio = if c_peak_val > 0 {
            rust_peak as f64 / c_peak_val as f64
        } else {
            f64::INFINITY
        };

        println!(
            "  {:<12} {:>12} {:>14} {:>14} {:>7.2}x",
            label,
            fmt_bytes(total_bytes),
            fmt_bytes(rust_peak),
            fmt_bytes(c_peak_val),
            ratio
        );

        // Memory should be a tiny fraction of the document size
        assert!(
            rust_peak < total_bytes / 10,
            "Rust peak {} is more than 10% of doc size {} — not streaming properly",
            fmt_bytes(rust_peak),
            fmt_bytes(total_bytes)
        );
        assert!(
            c_peak_val < total_bytes / 10,
            "C peak {} is more than 10% of doc size {} — not streaming properly",
            fmt_bytes(c_peak_val),
            fmt_bytes(total_bytes)
        );

        // Memory should NOT grow significantly as document size increases
        // (since we're streaming with fixed chunk size)
        if prev_rust_peak > 0 {
            assert!(
                rust_peak < prev_rust_peak * 3,
                "Rust peak grew from {} to {} as doc size increased — memory not bounded",
                fmt_bytes(prev_rust_peak),
                fmt_bytes(rust_peak)
            );
        }
        prev_rust_peak = rust_peak;
    }
    println!();
}

/// Verify memory doesn't grow when parsing the same-shape document repeatedly.
/// This catches leaks and unbounded buffer growth.
#[test]
fn no_memory_leak_on_repeated_parsing() {
    let xml = generate_document(1_000);

    // Warm up
    for _ in 0..10 {
        parse_rust(&xml);
    }

    let baseline = rust_current();

    for _ in 0..200 {
        parse_rust(&xml);
    }

    let after = rust_current();
    let growth = after.saturating_sub(baseline);

    assert!(
        growth < 4096,
        "Memory grew by {} after 200 parse cycles — possible leak",
        fmt_bytes(growth)
    );
}

/// Verify memory scales linearly with element count, not quadratically.
/// Uses chunked parsing (8 KB chunks) so document size isn't limited by
/// XML_Parse's c_int length parameter.
#[test]
fn memory_scales_linearly() {
    let element_counts = [100, 1_000, 10_000, 100_000, 1_000_000];
    let chunk_size = 8192;
    let mut measurements = Vec::new();

    println!();
    println!(
        "Memory scaling with document size (Rust vs C, {} chunks):",
        fmt_bytes(chunk_size)
    );
    println!(
        "  {:>10}  {:>10}  {:>14}  {:>12}  {:>14}  {:>12}",
        "Elements", "Doc Size", "Rust Peak", "Rust B/Elem", "C Peak", "C B/Elem"
    );
    println!("  {}", "-".repeat(80));

    for &n in &element_counts {
        let xml = generate_document(n);
        let doc_size = xml.len();

        let (rust_peak, _) = measure_rust(|| parse_rust_chunked(&xml, chunk_size));

        c_reset_tracking();
        let memsuite = make_c_memsuite();
        parse_c_chunked(&xml, chunk_size, &memsuite);
        let c_peak_val = c_peak();

        println!(
            "  {:>10}  {:>10}  {:>14}  {:>12.1}  {:>14}  {:>12.1}",
            n,
            fmt_bytes(doc_size),
            fmt_bytes(rust_peak),
            rust_peak as f64 / n as f64,
            fmt_bytes(c_peak_val),
            c_peak_val as f64 / n as f64
        );

        measurements.push((n, doc_size, rust_peak, c_peak_val));
    }
    println!();

    // Check linearity: bytes-per-element at 1M should be within 2x of bytes-per-element at 1K
    // (skip 100 since fixed overhead dominates at tiny sizes)
    let (n_small, _, peak_small, _) = measurements[1]; // 1K elements
    let (n_large, _, peak_large, _) = measurements[measurements.len() - 1]; // 1M elements
    let bpe_small = peak_small as f64 / n_small as f64;
    let bpe_large = peak_large as f64 / n_large as f64;

    assert!(
        bpe_large < bpe_small * 2.0,
        "Rust memory scales super-linearly: {:.0} B/elem at {} elems vs {:.0} B/elem at {} elems",
        bpe_large,
        n_large,
        bpe_small,
        n_small
    );
}
