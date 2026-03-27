//! Fuzz corpus comparison tests: run every file from the OSS-Fuzz public
//! corpora through both the C and Rust parsers, verifying behavioral equivalence.
//!
//! Corpora are downloaded by `scripts/download-fuzz-corpus.sh` into `corpus/`.
//! If the corpus directory is absent, these tests are skipped (not failed).
//!
//! We cover the two raw-input fuzzers:
//!   - xml_parse_fuzzer_UTF-8       (XML_Parse API, UTF-8 encoding)
//!   - xml_parsebuffer_fuzzer_UTF-16LE (XML_ParseBuffer API, UTF-16LE encoding)
//!
//! The xml_lpm_fuzzer uses protobuf-structured input and is excluded.
//!
//! ## What we check
//!
//! We verify that C and Rust **agree on parse status** (OK vs ERROR). Both parsers
//! must accept or reject the same inputs. Specific error codes (e.g. INVALID_TOKEN
//! vs UNCLOSED_TOKEN) are allowed to differ — the Rust tokenizer uses UTF-8
//! normalization which can produce different error codes for invalid XML. See
//! `docs/verification.md` for details.

use expat_rust::xmlparse::Parser;
use expat_sys::CParser;
use std::path::Path;
use std::sync::Once;

static SUPPRESS_PANIC_OUTPUT: Once = Once::new();

fn suppress_panic_output() {
    SUPPRESS_PANIC_OUTPUT.call_once(|| {
        std::panic::set_hook(Box::new(|_| {}));
    });
}

/// Compare a single input: parse with both C and Rust.
/// Returns (status_match, error_match) — status_match is the critical check.
fn compare_one(data: &[u8], encoding: Option<&str>) -> (bool, bool) {
    let mut r_parser = Parser::new(encoding).unwrap();
    let r_status = r_parser.parse(data, true) as u32;
    let r_error = r_parser.error_code() as u32;

    let c_parser = CParser::new(encoding).unwrap();
    let (c_status, c_error) = c_parser.parse(data, true);

    let status_match = r_status == c_status;
    let error_match = r_error == c_error;
    (status_match, error_match)
}

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

fn run_corpus(corpus_name: &str, encoding: Option<&str>) {
    suppress_panic_output();

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

    let mut status_pass = 0usize;
    let mut status_fail = 0usize;
    let mut error_match = 0usize;
    let mut error_differ = 0usize;
    let mut status_errors: Vec<String> = Vec::new();

    for path in &files {
        let data = std::fs::read(path).unwrap();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            compare_one(&data, encoding)
        }));

        match result {
            Ok((true, true)) => {
                status_pass += 1;
                error_match += 1;
            }
            Ok((true, false)) => {
                // Status agrees (both OK or both ERROR), error code differs — acceptable
                status_pass += 1;
                error_differ += 1;
            }
            Ok((false, _)) => {
                // Status disagrees (one OK, one ERROR) — this is a real bug
                status_fail += 1;
                if status_errors.len() < 20 {
                    let mut r = Parser::new(encoding).unwrap();
                    let rs = r.parse(&data, true) as u32;
                    let re = r.error_code() as u32;
                    let c = CParser::new(encoding).unwrap();
                    let (cs, ce) = c.parse(&data, true);
                    status_errors.push(format!(
                        "{file_name}: Rust({rs},{re}) vs C({cs},{ce}) len={}",
                        data.len()
                    ));
                }
            }
            Err(_) => {
                // Parser panicked — count as status failure
                status_fail += 1;
                if status_errors.len() < 20 {
                    status_errors.push(format!("{file_name}: PANIC"));
                }
            }
        }
    }

    let total = files.len();
    eprintln!(
        "Fuzz corpus {corpus_name}: {total} files, \
         status OK={status_pass} FAIL={status_fail}, \
         error exact={error_match} differ={error_differ}"
    );

    if !status_errors.is_empty() {
        let msg = status_errors.join("\n  ");
        panic!(
            "{status_fail} fuzz corpus files had OK/ERROR status disagreement \
             (first {} shown):\n  {msg}",
            status_errors.len()
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
