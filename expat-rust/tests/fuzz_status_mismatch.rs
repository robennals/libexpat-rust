//! Find and display all cases where C and Rust disagree on OK vs ERROR.
//! These are the real correctness bugs.

use expat_rust::xmlparse::Parser;
use expat_sys::CParser;
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

fn check_status_mismatches(corpus_name: &str, encoding: Option<&str>) {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let corpus_dir = workspace_root.join("corpus").join(corpus_name);
    let files = collect_corpus_files(&corpus_dir);
    if files.is_empty() {
        return;
    }

    std::panic::set_hook(Box::new(|_| {}));

    let mut rust_ok_c_err = Vec::new();
    let mut rust_err_c_ok = Vec::new();

    for path in &files {
        let data = std::fs::read(path).unwrap();
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut r = Parser::new(encoding).unwrap();
            let rs = r.parse(&data, true) as u32;
            let re = r.error_code() as u32;

            let c = CParser::new(encoding).unwrap();
            let (cs, ce) = c.parse(&data, true);

            (rs, re, cs, ce)
        }));

        if let Ok((rs, re, cs, ce)) = result {
            if rs == 1 && cs == 0 {
                // Rust says OK, C says ERROR
                rust_ok_c_err.push((name, ce, data.len()));
            } else if rs == 0 && cs == 1 {
                // Rust says ERROR, C says OK
                rust_err_c_ok.push((name, re, data.len()));
            }
        }
    }

    eprintln!("\n=== {corpus_name} ===");
    if !rust_ok_c_err.is_empty() {
        eprintln!("Rust=OK but C=ERROR ({} files) — Rust accepts invalid XML:", rust_ok_c_err.len());
        for (name, c_err, len) in &rust_ok_c_err {
            eprintln!("  {name}  C_error={c_err}  len={len}");
        }
    }
    if !rust_err_c_ok.is_empty() {
        eprintln!("Rust=ERROR but C=OK ({} files) — Rust rejects valid XML:", rust_err_c_ok.len());
        for (name, r_err, len) in &rust_err_c_ok {
            eprintln!("  {name}  Rust_error={r_err}  len={len}");
        }
    }
    if rust_ok_c_err.is_empty() && rust_err_c_ok.is_empty() {
        eprintln!("No status mismatches!");
    }
}

#[test]
fn status_mismatches_utf8() {
    check_status_mismatches("xml_parse_fuzzer_UTF-8", Some("UTF-8"));
}

#[test]
fn status_mismatches_utf16le() {
    check_status_mismatches("xml_parsebuffer_fuzzer_UTF-16LE", Some("UTF-16LE"));
}
