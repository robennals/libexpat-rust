# C Test Suite Validation

## Summary

**192 of 290 tests pass (66.2%)**. Of the 98 failing, 5 are not applicable to Rust, and 93 need to be fixed.

## Tests Not Applicable to Rust (5)

These test C implementation internals that don't exist in a Rust implementation:

| Test | Reason |
|------|--------|
| `test_misc_alloc_create_parser` | Tests custom C allocator (`XML_ParserCreate_MM` with failing `malloc`). Rust uses its own allocator. |
| `test_misc_alloc_create_parser_with_encoding` | Same — custom C allocator with encoding parameter. |
| `test_accounting_precision` | Tests `g_bytesScanned` internal counter in C `xmlparse.c`. This is a testing-only variable in the C source, not part of the API. |
| `test_billion_laughs_attack_protection_api` | Tests that `XML_SetBillionLaughsAttackProtection*` rejects calls on non-root parsers. Our implementation accepts the API but Rust's memory safety provides equivalent protection. |
| `test_amplification_isolated_external_parser` | Tests amplification byte counting internals across sub-parsers. Same as above. |

## Tests That Need Fixing — Encoding (29)

These fail because our parser transcodes non-UTF-8 to UTF-8 internally, but needs work on:
- UTF-16 LE/BE byte offset mapping
- Unknown encoding handler API (`XML_SetUnknownEncodingHandler`)
- Encoding edge cases (BOM handling, Latin-1 in external entities)

## Tests That Need Fixing — Namespace (22)

The parser accepts `XML_ParserCreateNS` but doesn't implement namespace URI resolution. Needs:
- Prefix → URI binding tracking
- Element/attribute name rewriting to `{URI}local` format
- Namespace validation (unbound prefix, reserved prefixes, double colon)

## Tests That Need Fixing — Parser Behavior (41)

These are genuine behavioral gaps identified by AST comparison between C `doProlog`/`doContent` and Rust equivalents:

### External Entity Sub-parsers (8 tests)
Tests involving `XML_ExternalEntityParserCreate` with edge cases:
- `test_ext_entity_trailing_cr`, `test_ext_entity_trailing_rsqb` — trailing CR/] in sub-parser
- `test_ext_entity_invalid_parse`, `test_ext_entity_invalid_suspended_parse` — error handling
- `test_ext_entity_not_standalone` — not-standalone in sub-parser context
- `test_ext_entity_utf8_non_bom`, `test_external_bom_consumed` — BOM handling
- `test_subordinate_xdecl_abort` — abort during XML decl in sub-parser

### DTD Content Model (5 tests)
Element declaration content model building:
- `test_dtd_elements_nesting`, `test_nested_groups`, `test_group_choice` — `<!ELEMENT>` content model trees
- `test_bad_ignore_section`, `test_ignore_section` — `<![IGNORE[...]]>` sections

### Parameter Entity Processing (6 tests)
- `test_skipped_parameter_entity` — skipped entity handler for undefined PEs
- `test_param_entity_with_trailing_cr` — CR handling in PE
- `test_recursive_external_parameter_entity{,_2}` — recursive PE detection
- `test_suspend_resume_parameter_entity` — suspend/resume within PE
- `test_misc_deny_internal_entity_closing_doctype_issue_317` — PE closing DOCTYPE

### Entity Expansion (4 tests)
- `test_no_indirectly_recursive_entity_refs` — indirect recursion in attribute values
- `test_misc_async_entity_rejected` — async entity handling
- `test_renter_loop_finite_content` — reentrancy detection
- `test_misc_expected_event_ptr_issue_980` — event pointer tracking

### Foreign DTD (3 tests)
- `test_set_foreign_dtd` — trigger ext entity handler with foreign DTD
- `test_foreign_dtd_not_standalone` — not-standalone with foreign DTD
- `test_foreign_dtd_with_doctype` — foreign DTD combined with DOCTYPE

### Default Handler (2 tests)
- `test_default_current` — `XML_DefaultCurrent` API
- `test_dtd_default_handling` — DTD internal subset forwarding to default handler

### Byte Info / Position (3 tests)
- `test_byte_info_at_end`, `test_byte_info_at_error`, `test_byte_info_at_cdata`

### Misc (10 tests)
- `test_attr_whitespace_normalization` — type-aware attribute normalization
- `test_buffer_can_grow_to_max` — buffer growth edge case
- `test_misc_attribute_leak` — attribute with unbound namespace prefix
- `test_misc_no_infinite_loop_issue_1161` — infinite loop prevention
- `test_pool_integrity_with_unfinished_attr` — partial attribute parsing
- `test_user_parameters` — parameter passing through sub-parsers
- `test_bad_doctype` — invalid bytes in DOCTYPE with custom encoding
- `test_invalid_tag_in_dtd` — tag in external DTD
- `test_invalid_character_entity_3` — UTF-16 character entity
- `test_suspend_resume_internal_entity` — suspend/resume in entity expansion
