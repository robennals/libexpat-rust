//! Fuzz corpus comparison tests: run every file from the OSS-Fuzz public
//! corpora through both the C and Rust parsers, verifying identical behavior.
//!
//! Corpora are downloaded by `scripts/download-fuzz-corpus.sh` into `corpus/`.
//! If the corpus directory is absent, these tests are skipped (not failed).
//!
//! We cover the two raw-input fuzzers:
//!   - xml_parse_fuzzer_UTF-8       (XML_Parse API, UTF-8 encoding)
//!   - xml_parsebuffer_fuzzer_UTF-16LE (XML_ParseBuffer API, UTF-16LE encoding)
//!
//! The xml_lpm_fuzzer uses protobuf-structured input and is excluded.

use expat_rust::xmlparse::Parser;
use expat_sys::CParser;
use std::path::Path;
use std::sync::Once;

static SUPPRESS_PANIC_OUTPUT: Once = Once::new();

/// Suppress the default panic hook so catch_unwind doesn't spam stderr.
fn suppress_panic_output() {
    SUPPRESS_PANIC_OUTPUT.call_once(|| {
        std::panic::set_hook(Box::new(|_| {}));
    });
}

/// Compare a single input: parse with both C and Rust, assert same status+error.
fn compare_one(data: &[u8], encoding: Option<&str>, file_name: &str) {
    // Rust parser
    let mut r_parser = Parser::new(encoding).unwrap();
    let r_status = r_parser.parse(data, true) as u32;
    let r_error = r_parser.error_code() as u32;

    // C parser
    let c_parser = CParser::new(encoding).unwrap();
    let (c_status, c_error) = c_parser.parse(data, true);

    assert!(
        r_status == c_status && r_error == c_error,
        "MISMATCH on fuzz corpus file {file_name} (encoding={encoding:?}):\n  \
         Rust: status={r_status} error={r_error}\n  \
         C:    status={c_status} error={c_error}\n  \
         Input length: {} bytes",
        data.len()
    );
}

/// Collect all files in a corpus directory, sorted for determinism.
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

/// Run comparison on all files in a corpus directory.
fn run_corpus(corpus_name: &str, encoding: Option<&str>) {
    suppress_panic_output();

    // Look for corpus relative to the workspace root (where Cargo.toml is)
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let corpus_dir = workspace_root.join("corpus").join(corpus_name);

    let files = collect_corpus_files(&corpus_dir);
    if files.is_empty() {
        eprintln!(
            "SKIPPED: no corpus files found at {}. Run scripts/download-fuzz-corpus.sh first.",
            corpus_dir.display()
        );
        return;
    }

    let mut pass = 0;
    let mut fail = 0;
    let mut errors: Vec<String> = Vec::new();

    for path in &files {
        let data = std::fs::read(path).unwrap();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            compare_one(&data, encoding, &file_name);
        }));

        match result {
            Ok(()) => pass += 1,
            Err(e) => {
                fail += 1;
                if errors.len() < 10 {
                    let msg = if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "unknown panic".to_string()
                    };
                    errors.push(msg);
                }
            }
        }
    }

    eprintln!(
        "Fuzz corpus {corpus_name}: {pass} pass, {fail} fail out of {} files",
        files.len()
    );

    if !errors.is_empty() {
        let msg = errors.join("\n---\n");
        panic!(
            "{fail} fuzz corpus files had C/Rust mismatches (first {} shown):\n{msg}",
            errors.len()
        );
    }
}

#[test]
fn fuzz_corpus_xml_parse_utf8() {
    run_corpus("xml_parse_fuzzer_UTF-8", Some("UTF-8"));
}

#[test]
fn fuzz_corpus_xml_parsebuffer_utf16le() {
    run_corpus("xml_parsebuffer_fuzzer_UTF-16LE", Some("UTF-16LE"));
}
