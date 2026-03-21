# Agent: Translate C Tests to Rust

**Model tier: Haiku for simple tests, Sonnet for complex (callbacks + state)**

## When to Use
- Translating C test functions to Rust #[test] functions
- Works best when given a batch of related tests + the helpers they use

## Prompt Template

```
You are translating C tests to idiomatic Rust tests.

C test file: {C_TEST_FILE}
C test helpers used: {HELPER_FILES}
Rust API module: {RUST_API_MODULE}
Rust test output: {RUST_TEST_OUTPUT}

For each START_TEST(test_name) ... END_TEST block in the C file:

1. Create a Rust #[test] fn test_name() that tests the same behavior
2. Replace C patterns with Rust equivalents:
   - XML_ParserCreate → Parser::new() (or whatever the Rust API is)
   - XML_Parse → parser.parse()
   - fail("message") → panic!("message")
   - assert_true(x) → assert!(x)
   - CharData helpers → Rust String collection
   - Callback registration → Rust closure/callback API

3. Do NOT use unsafe. If the C test relies on pointer tricks, redesign for Rust.
4. Preserve ALL assertions — every assert in C must have a Rust equivalent.
5. Tests may not pass yet (API may be stubs). That's OK — they should compile.

Verify: source "$HOME/.cargo/env" && cd {RUST_PROJECT_PATH} && cargo check --tests
```

## Translation Priority
1. misc_tests.c (22 tests, simplest, no callbacks)
2. ns_tests.c (33 tests, basic callbacks)
3. basic_tests.c (244 tests, phased — start with first 50)
4. alloc_tests.c (61 tests, needs custom allocator infra)
5. nsalloc_tests.c (27 tests)
6. acc_tests.c (4 tests, needs XML_GE feature flag)
