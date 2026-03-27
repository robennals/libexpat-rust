//! Analysis tool: categorize fuzz corpus mismatches by error code pattern.
//! Run with: cargo test -p expat-rust --test fuzz_corpus_analysis -- --nocapture

use expat_rust::xmlparse::Parser;
use expat_sys::CParser;
use std::collections::HashMap;
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

fn analyze_corpus(corpus_name: &str, encoding: Option<&str>) {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let corpus_dir = workspace_root.join("corpus").join(corpus_name);
    let files = collect_corpus_files(&corpus_dir);
    if files.is_empty() {
        eprintln!("No corpus files for {corpus_name}");
        return;
    }

    // Map from (rust_status, rust_error, c_status, c_error) -> count
    let mut patterns: HashMap<(u32, u32, u32, u32), (usize, String)> = HashMap::new();
    let mut pass = 0usize;

    for path in &files {
        let data = std::fs::read(path).unwrap();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut r_parser = Parser::new(encoding).unwrap();
            let r_status = r_parser.parse(&data, true) as u32;
            let r_error = r_parser.error_code() as u32;

            let c_parser = CParser::new(encoding).unwrap();
            let (c_status, c_error) = c_parser.parse(&data, true);

            (r_status, r_error, c_status, c_error)
        }));

        match result {
            Ok((rs, re, cs, ce)) => {
                if rs == cs && re == ce {
                    pass += 1;
                } else {
                    let entry = patterns
                        .entry((rs, re, cs, ce))
                        .or_insert((0, file_name.clone()));
                    entry.0 += 1;
                }
            }
            Err(_) => {
                // Parser crashed - count as special pattern
                let entry = patterns
                    .entry((99, 99, 99, 99))
                    .or_insert((0, file_name.clone()));
                entry.0 += 1;
            }
        }
    }

    eprintln!("\n=== {corpus_name} (encoding={encoding:?}) ===");
    eprintln!("Total files: {}", files.len());
    eprintln!("Pass: {pass}");
    eprintln!("Fail: {}", files.len() - pass);
    eprintln!("\nMismatch patterns (rust_status, rust_err -> c_status, c_err : count):");

    let mut sorted: Vec<_> = patterns.into_iter().collect();
    sorted.sort_by(|a, b| b.1 .0.cmp(&a.1 .0)); // Sort by count descending

    for ((rs, re, cs, ce), (count, example)) in &sorted {
        eprintln!("  Rust({rs},{re:>2}) vs C({cs},{ce:>2}): {count:>6} files  (e.g. {example})");
    }
}

#[test]
fn analyze_utf8() {
    std::panic::set_hook(Box::new(|_| {}));
    analyze_corpus("xml_parse_fuzzer_UTF-8", Some("UTF-8"));
}

#[test]
fn analyze_utf16le() {
    std::panic::set_hook(Box::new(|_| {}));
    analyze_corpus("xml_parsebuffer_fuzzer_UTF-16LE", Some("UTF-16LE"));
}
