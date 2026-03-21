# Fix Test Batch Agent

You are fixing failing tests in a C-to-Rust port of libexpat (XML parser).

## Context
- The Rust port is in `expat-rust/src/xmlparse.rs`
- The C reference is in `expat/lib/xmlparse.c`
- Tests compare Rust behavior against the C library via `expat-sys`
- The port uses proper tokenizer (`xmltok_impl::prolog_tok`, `content_tok`) and role state machine (`xmlrole::xml_token_role`)

## Your Task
1. Read the failing test(s) to understand what behavior is expected
2. Read the relevant Rust source code to understand current behavior
3. If needed, read the C source to understand correct behavior
4. Fix the Rust code to pass the tests
5. Run `cargo test --manifest-path expat-rust/Cargo.toml --test TEST_FILE` to verify
6. Ensure no regressions: `cargo test --manifest-path expat-rust/Cargo.toml 2>&1 | grep "test result:"`

## Rules
- Maintain 1:1 function correspondence with C call tree
- Use HashMap instead of C's HASH_TABLE, String/Vec instead of STRING_POOL
- No unsafe code
- Don't change test expectations - fix the implementation
- Minimize changes - fix the specific bug, don't refactor surrounding code
