//! Debug tool: dump detailed parse info for specific fuzz corpus files.
//! Run with: cargo test -p expat-rust --test fuzz_debug -- --nocapture

use expat_rust::xmlparse::{Parser, XmlError};
use expat_sys::CParser;

fn debug_one(file_path: &str, encoding: Option<&str>) {
    let Ok(data) = std::fs::read(file_path) else {
        eprintln!("  SKIP: {file_path} not present (corpus drift)");
        return;
    };
    let name = std::path::Path::new(file_path)
        .file_name()
        .unwrap()
        .to_string_lossy();

    let mut r_parser = Parser::new(encoding).unwrap();
    let r_status = r_parser.parse(&data, true) as u32;
    let r_error = r_parser.error_code() as u32;
    let r_line = r_parser.current_line_number();
    let r_col = r_parser.current_column_number();

    let c_parser = CParser::new(encoding).unwrap();
    let (c_status, c_error) = c_parser.parse(&data, true);
    let c_line = c_parser.current_line_number();
    let c_col = c_parser.current_column_number();

    eprintln!("=== {name} (encoding={encoding:?}, len={}) ===", data.len());
    eprintln!("  Rust: status={r_status} error={r_error} line={r_line} col={r_col}");
    eprintln!("  C:    status={c_status} error={c_error} line={c_line} col={c_col}");
    if data.len() <= 100 {
        eprintln!("  Hex:  {:02x?}", &data);
    } else {
        eprintln!("  Hex (first 100): {:02x?}", &data[..100]);
    }
    if r_status == c_status && r_error == c_error {
        eprintln!("  MATCH");
    } else {
        eprintln!("  MISMATCH");
    }
    eprintln!();
}

#[test]
fn debug_samples() {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();

    // UTF-8: Rust(0,4) vs C(0,5) - INVALID_TOKEN vs UNCLOSED_TOKEN
    let corpus_utf8 = workspace.join("corpus/xml_parse_fuzzer_UTF-8");
    if corpus_utf8.exists() {
        eprintln!("\n--- UTF-8: Rust=INVALID_TOKEN vs C=UNCLOSED_TOKEN ---");
        debug_one(
            &corpus_utf8
                .join("000696143241db7c297a42b9f1f7f736573c3cc9")
                .to_string_lossy(),
            Some("UTF-8"),
        );
        // Rust(0,4) vs C(0,3) - INVALID_TOKEN vs NO_ELEMENTS
        eprintln!("--- UTF-8: Rust=INVALID_TOKEN vs C=NO_ELEMENTS ---");
        debug_one(
            &corpus_utf8
                .join("001d497bbf9c752ed5e9ebde7cabaece16b42222")
                .to_string_lossy(),
            Some("UTF-8"),
        );
        // Rust(0,4) vs C(0,2) - INVALID_TOKEN vs SYNTAX
        eprintln!("--- UTF-8: Rust=INVALID_TOKEN vs C=SYNTAX ---");
        debug_one(
            &corpus_utf8
                .join("003675a3f89f6d49994b1e857e3dba120bae9ec1")
                .to_string_lossy(),
            Some("UTF-8"),
        );
    }

    // UTF-16LE: Rust(0,4) vs C(0,5) - INVALID_TOKEN vs UNCLOSED_TOKEN
    let corpus_utf16 = workspace.join("corpus/xml_parsebuffer_fuzzer_UTF-16LE");
    if corpus_utf16.exists() {
        eprintln!("\n--- UTF-16LE: Rust=INVALID_TOKEN vs C=UNCLOSED_TOKEN ---");
        debug_one(
            &corpus_utf16
                .join("00125e4b348500718d6f3b09182ae806e1640efa")
                .to_string_lossy(),
            Some("UTF-16LE"),
        );
        // Rust(0,4) vs C(0,2) - INVALID_TOKEN vs SYNTAX
        eprintln!("--- UTF-16LE: Rust=INVALID_TOKEN vs C=SYNTAX ---");
        debug_one(
            &corpus_utf16
                .join("000270f4575326d20da72c69319239206e33d9fb")
                .to_string_lossy(),
            Some("UTF-16LE"),
        );
        // Rust(0,4) vs C(0,3) - INVALID_TOKEN vs NO_ELEMENTS
        eprintln!("--- UTF-16LE: Rust=INVALID_TOKEN vs C=NO_ELEMENTS ---");
        debug_one(
            &corpus_utf16
                .join("0055270e3074839f49a1d9be940f168047219b6a")
                .to_string_lossy(),
            Some("UTF-16LE"),
        );
    }
}
