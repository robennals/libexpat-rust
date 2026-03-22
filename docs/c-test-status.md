# C Test Suite Status

Running the original C libexpat test suite against our Rust parser via `expat-ffi`.

**Current results: 78 pass, 53 fail** of 131 tests reached (process aborts at test #132 `test_bad_cdata_utf16` due to C `assert()`).

## How to run

```bash
cargo build -p c-tests-runner
lldb -b -o "run" -o "quit" ./target/debug/c-tests-runner 2>&1 | grep -c "^PASS:"
```

Note: `lldb` is needed because one C test uses `assert()` instead of `fail()`, which aborts the process and prevents remaining tests from running.

## Failure Categories

### Namespace Processing (21 tests)

The Rust parser doesn't fully process namespace URIs. When namespace-aware parsing is enabled (`XML_ParserCreateNS`), the C library rewrites element/attribute names to `{uri}localname` format and validates prefix bindings. Our parser passes names through without URI resolution.

Failing: `test_return_ns_triplet` (x2), `test_ns_tagname_overwrite`, `test_ns_tagname_overwrite_triplet`, `test_start_ns_clears_start_element`, `test_ns_double_colon`, `test_ns_double_colon_element`, `test_ns_duplicate_attrs_diff_prefixes`, `test_ns_long_element`, `test_ns_prefix_with_empty_uri_{1-4}`, `test_ns_reserved_attributes{,_2}`, `test_ns_separator_in_uri`, `test_ns_unbound_prefix{,_on_attribute,_on_element}`, `test_ns_utf16_{doctype,element_leafname,leafname}`, `test_default_ns_from_ext_subset_and_ext_ge`

### Parser Behavior Bugs (17 tests)

Genuine behavioral differences where the Rust parser produces different results than C libexpat:

| Test | Issue |
|------|-------|
| `test_attr_whitespace_normalization` | DTD-aware attribute normalization (NMTOKENS→space collapse) not implemented |
| `test_illegal_utf8` | Some invalid UTF-8 sequences not rejected |
| `test_utf8_auto_align` | UTF-8 alignment/trimming at chunk boundaries not implemented |
| `test_nobom_utf16_le` | UTF-16 LE detection without BOM |
| `test_utf16_le_epilog_newline` | UTF-16 LE newline handling in epilog |
| `test_latin1_umlauts` | Character data handler output differs for Latin-1 |
| `test_long_utf8_character` | 4-byte UTF-8 chars in element names not rejected |
| `test_really_long_encoded_lines` | `XML_ParseBuffer` not implemented |
| `test_line_and_column_numbers_inside_handlers` | Position tracking inaccurate inside handler callbacks |
| `test_not_standalone_handler_reject` | Not-standalone handler not invoked during DTD processing |
| `test_no_indirectly_recursive_entity_refs` | Indirect entity recursion not detected |
| `test_misc_general_entities_support` | Entity expansion in content produces wrong data length |
| `test_misc_deny_internal_entity_closing_doctype_issue_317` | Parameter entity closing DOCTYPE not detected |
| `test_misc_expected_event_ptr_issue_980` | Entity expansion event pointer tracking |
| `test_misc_no_infinite_loop_issue_1161` | External entity recursion detection |
| `test_misc_attribute_leak` | Attribute parsing with namespace prefix issues |
| `test_misc_async_entity_rejected` | Async entity rejection |

### Stubs (7 tests)

Features not implemented because they don't apply to Rust or are deliberately omitted:

| Test | Reason |
|------|--------|
| `test_accounting_precision` | `g_bytesScanned` counter always returns 0 |
| `test_billion_laughs_attack_protection_api` | Protection API accepts but doesn't enforce limits |
| `test_amplification_isolated_external_parser` | Amplification tracking not implemented |
| `test_misc_alloc_create_parser` | Custom memory allocators (N/A for Rust) |
| `test_misc_alloc_create_parser_with_encoding` | Custom memory allocators (N/A for Rust) |
| `test_ext_entity_set_encoding` | External entity encoding not fully implemented |
| `test_ext_entity_set_bom` | External entity BOM handling not implemented |

### Suspend/Resume (3 tests — 2 now fixed)

| Test | Status |
|------|--------|
| `test_suspend_parser_between_char_data_calls` | **FIXED** |
| `test_repeated_stop_parser_between_char_data_calls` | **FIXED** |
| `test_renter_loop_finite_content` | Internal entity processor reentrancy |

### Not Yet Reached (~172 tests)

The process aborts at test #132 due to `test_bad_cdata_utf16` using C `assert()` instead of `fail()`. This prevents ~172 tests from running. These are later basic tests, plus all alloc_tests and nsalloc_tests.
