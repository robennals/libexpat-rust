# Behavioral Verification

How we confirm that `expat-rust` exactly matches libexpat's behavior.

## Approach: C Is the Ground Truth

Our verification strategy is simple: **run the same XML through both parsers and confirm identical output**. We don't write expected outputs by hand — we let the C library define what "correct" means, then verify the Rust parser produces the same result.

This means:
- If C returns `XML_STATUS_OK`, Rust must return `XmlStatus::Ok`
- If C returns `XML_ERROR_INVALID_TOKEN`, Rust must return `XmlError::InvalidToken`
- If C fires `startElement("root", ["id", "1"])`, Rust must fire `startElement("root", [("id", "1")])`
- If C normalizes `\t` in an attribute value to a space, Rust must do the same

Every test is a comparison, not an assertion against a hardcoded expected value.

## What We Compare

Each comparison test exercises both parsers with the same input and compares:

### 1. Parse Status and Error Code

```
Rust: status=Ok, error=None
C:    status=1 (OK), error=0 (NONE)
→ Match ✓
```

Both parsers must agree on whether parsing succeeded or failed, and if it failed, what the error was.

### 2. Full SAX Event Traces

Both parsers have the full set of SAX handlers registered. We collect every callback invocation into an ordered event log, then compare the two logs:

```
Rust events: ["SE:root id=1", "CD:Hello", "SE:child", "EE:child", "EE:root"]
C events:    ["SE:root id=1", "CD:Hello", "SE:child", "EE:child", "EE:root"]
→ Match ✓
```

Event types compared:
- **SE** (StartElement): element name + all attribute name=value pairs
- **EE** (EndElement): element name
- **CD** (CharacterData): text content (adjacent chunks merged before comparison, since SAX allows different chunking)
- **PI** (ProcessingInstruction): target + data
- **CM** (Comment): comment text
- **SCD/ECD** (StartCdataSection/EndCdataSection)
- **SDT/EDT** (StartDoctypeDecl/EndDoctypeDecl): name, system ID, public ID, has_internal_subset flag

### 3. Incremental Parsing

Many tests also verify incremental (chunked) parsing by splitting the input at every byte boundary:

```
Input: "<r>text</r>" (11 bytes)
  Split at byte 1:  parse("<", false) then parse("r>text</r>", true)
  Split at byte 2:  parse("<r", false) then parse(">text</r>", true)
  Split at byte 3:  parse("<r>", false) then parse("text</r>", true)
  ...
  Split at byte 10: parse("<r>text</r", false) then parse(">", true)
```

At each split point, both parsers must produce the same final status and error code. This catches bugs where the Rust parser mishandles token boundaries across buffer chunks.

## Test Categories

### Valid XML Corpus (~95 documents)

Programmatically generated XML documents covering every feature combination:

- **Basic elements**: empty, self-closing, nested, siblings, mixed content
- **Attributes**: single, multiple, single-quoted, entity refs, char refs, whitespace normalization (tab/CR/LF/CRLF → space)
- **Processing instructions**: before/in/after root, with/without data
- **Comments**: empty, multiline, with UTF-8 content
- **CDATA sections**: empty, with markup chars, with brackets, with CR/LF
- **Entities**: predefined (`&amp;` etc.), internal general, char refs (decimal/hex), entity-in-attribute, multi-expansion, empty entity
- **XML declaration**: version only, with encoding, with standalone, full
- **DOCTYPE**: simple, SYSTEM, PUBLIC, with internal subset
- **DTD declarations**: ELEMENT (EMPTY/ANY/PCDATA/mixed/sequence/choice/nested groups with quantifiers), ATTLIST (all types, defaults, FIXED, enumerations), NOTATION, external entities, unparsed entities with NDATA
- **Encodings**: UTF-8 (with BOM), UTF-16 LE/BE (with BOM), ISO-8859-1, US-ASCII
- **Multi-byte UTF-8**: 2-byte (café, ñ), 3-byte (日本語), 4-byte (😀), in content/attributes/comments/entity values
- **Whitespace**: CR/LF/CRLF/tab normalization in content and attributes
- **Complex documents**: combining all of the above

### Error XML Corpus (~35 inputs)

Invalid XML inputs that must produce the same error in both parsers:

- Empty input, whitespace only, bare text
- Unclosed tags, mismatched tags, duplicate root
- Malformed entities, undefined entities, recursive entities
- Duplicate attributes, `<` in attribute values, valueless attributes
- Invalid character references (null, surrogates, out-of-range)
- Unclosed CDATA/comments/PIs
- `]]>` in content, reserved PI targets (`<?XML?>`)
- Invalid PUBLIC ID characters
- Partial UTF-8 sequences

### UTF-16 Corpus (~9 inputs)

UTF-16 LE and BE inputs with BOM, including content, attributes, comments, PIs, and surrogate pairs (characters above U+FFFF).

### Incremental Parsing Tests

Every input under 100 bytes is tested with byte-by-byte split parsing. Longer inputs are tested at selected split points.

### Parser State Tests

- **Reset**: Parse document A, reset, parse document B — must match a fresh parser on B
- **Parse after finish**: Second `parse()` call after `is_final=true` — must return same error
- **Multi-chunk**: Multiple non-final `parse()` calls followed by final
- **Stop/resume**: API state machine edge cases

### API Coverage Tests

- All handler setter APIs exercised
- XML declaration handler: version/encoding/standalone values compared
- Default handler output compared
- Position tracking (line/column) compared
- `expat_version()`, `expat_version_info()`, `get_feature_list()` exercised

## How to Run

```bash
# Run all comparison tests (requires C compiler for expat-sys)
RUST_TEST_THREADS=1 cargo test -p expat-rust

# Run only C-vs-Rust comparison tests
RUST_TEST_THREADS=1 cargo test -p expat-rust --test c_comparison_tests \
  --test comprehensive_comparison --test generated_comparison_tests \
  --test coverage_comparison_tests --test auto_coverage_tests

# Run with line coverage measurement (requires cargo-llvm-cov)
cargo llvm-cov clean --workspace
RUST_TEST_THREADS=1 cargo llvm-cov --no-report -p expat-rust \
  --test auto_coverage_tests --test coverage_comparison_tests \
  --test c_comparison_tests --test comprehensive_comparison \
  --test generated_comparison_tests
cargo llvm-cov report -p expat-rust --summary-only
```

## Coverage

Line coverage of the Rust parser code, measured by `cargo-llvm-cov`:

| File | Lines | Covered | Coverage |
|------|-------|---------|----------|
| xmlparse.rs | 1770 | 1410 | 79.7% |
| xmltok_impl.rs | 1805 | 1298 | 71.9% |
| xmlrole.rs | 687 | 467 | 68.0% |
| xmltok.rs | 336 | 197 | 58.6% |
| **Total** | **4598** | **3372** | **73.3%** |

The remaining uncovered code falls into two categories:

1. **Unreachable internal utilities** (~140 lines in xmltok.rs): Functions like `utf8_encode()` and `parse_pseudo_attribute()` that were ported from C but are not called by the parser. The Rust parser uses Rust's native char encoding and a different XML declaration parsing path.

2. **Scattered branch coverage** (~1090 lines): Individual branch arms for handler-not-set dispatch, rare DTD construct parsing, error edge cases, and prolog state machine transitions. Each uncovered block is typically 1-5 lines.

Excluding unreachable utilities, effective coverage of API-reachable code is approximately **76%**.

## Test File Overview

| File | Tests | Description |
|------|-------|-------------|
| `auto_coverage_tests.rs` | 69 | Automated: combinatorial XML generation, full SAX comparison, incremental parsing, stress tests |
| `coverage_comparison_tests.rs` | 144 | Targeted: specific code path coverage with full SAX event comparison |
| `comprehensive_comparison.rs` | 82 | Systematic: DTD, entity, encoding, incremental split comparison |
| `generated_comparison_tests.rs` | 109 | Generated: XML feature matrix with incremental byte-split testing |
| `c_comparison_tests.rs` | 59 | Original: foundational status/error/handler comparison tests |

## Bugs Found and Fixed

The comparison tests discovered 8 behavioral differences (bugs in the Rust parser) which were all fixed:

1. **Valueless attribute panic**: `<r a/>` caused a slice bounds panic instead of returning `InvalidToken`
2. **Attribute value normalization**: Tab, newline, CR, CRLF in attribute values were not normalized to spaces; entity/char refs were not expanded
3. **Reserved PI target**: `<?XML?>` (case-insensitive) was not rejected as `InvalidToken`
4. **Multi-byte UTF-8 in DTD literals**: `scan_lit()` rejected TRAIL bytes from multi-byte characters
5. **Multi-byte UTF-8 in comments**: `scan_comment()` had the same TRAIL byte issue
6. **UTF-16 incremental BOM split**: Splitting a UTF-16 BOM across parse calls caused incorrect error
7. **ATTLIST default attributes**: Default attribute values from DTD were not applied to elements
8. **Close-paren split**: `)` at the end of a parse chunk returned `CloseParen` instead of `Partial`, breaking `)*`/`)?`/`)+` across chunks

Every fix was verified by the comparison test that discovered it — the test passes only when both parsers produce identical output.

## Design Decision: Transcode-to-UTF-8

One deliberate architectural difference from C: the Rust parser transcodes all non-UTF-8 input (UTF-16, Latin-1) to UTF-8 before tokenizing. C libexpat tokenizes in the native encoding using encoding-specific byte-type tables.

Both approaches produce identical results:
- SAX events are identical for all XML-legal inputs (confirmed by comparison tests)
- `XML_GetCurrentByteIndex` returns byte offsets in the **original** input encoding — for non-UTF-8 input, the parser re-scans the current chunk to convert internal UTF-8 positions back to original byte offsets (O(chunk_size) per call, no per-byte overhead during normal parsing)
- Line and column numbers are encoding-independent and always match

See [architecture.md](architecture.md) and [design-decisions.md](design-decisions.md) for the full rationale.
