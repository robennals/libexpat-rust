# Verification

How we build confidence that `expat-rust` faithfully reproduces libexpat's behavior.

## Approach: Three Complementary Layers

No single verification technique can guarantee that a cross-language port is correct.
Instead, we use three independent layers that reinforce each other:

1. **AST structural comparison** — verifies the Rust code's *structure* matches the C
   code, function-by-function, with every difference explicitly justified
2. **Original C test suite** — the same 291 tests libexpat uses, compiled against
   the Rust parser's C FFI layer
3. **Behavioral comparison tests** — 463 tests that run identical XML through both
   parsers and confirm identical SAX event traces, error codes, and parse results

Each layer catches different classes of bugs:

- The AST comparison catches **structural omissions** — a missing error check, a
  forgotten handler call, a dropped match arm — even if no test input triggers the
  missing path. It starts from the assumption that C and Rust must be identical,
  and only allows differences that match an explicit rewrite rule. So semantic
  differences can only slip through if a rewrite rule is wrong — i.e., it
  suppresses a difference that is actually a bug, not a legitimate language
  difference. The risk is concentrated in the rewrite rules, not in the comparison.
- The C test suite catches **behavioral regressions** against libexpat's own quality
  bar, including edge cases the expat maintainers specifically wrote tests for.
- The comparison tests catch **behavioral divergences** on a broad corpus of inputs,
  including SAX event ordering, attribute normalization, encoding, and incremental
  parsing boundaries.

Together, these make behavioral differences *unlikely* without guaranteeing their
absence. The AST comparison requires identical structure except where a rewrite
rule explicitly allows a difference, the original tests verify known-important
behaviors, and the comparison tests sweep a wide input space. A bug would need to
either hide behind a wrong rewrite rule that is also not exercised by any of
~750 tests, or exist in argument details that the AST tool doesn't compare
(e.g., wrong slice bounds on a structurally-matched call).

## Layer 1: AST Structural Comparison

See [../validator/README.md](../validator/README.md) for full tool documentation.

The strict AST verifier (`validator/strict-ast-compare.py`) parses both C and Rust
with tree-sitter, converts them into a common "skeleton" intermediate representation,
and structurally compares them. Every C operation (function call, error return,
handler dispatch, branch condition, match arm) must have a corresponding Rust
operation in the same structural position.

Language differences are handled by **explicit rewrite rules** stored in JSON:

- **`structural-rewrites.json`** — Verified rules for known language differences
  (e.g., C `break` removed in Rust match arms, C pool operations replaced by
  Rust Vec, C position tracking via output pointers replaced by Rust return tuples).
  Each rule has an input pattern, output pattern, and justification.

- **`temporary-rewrites.json`** — Rules for patterns we *believe* are equivalent
  but haven't fully verified (e.g., C OOM return checks vs Rust panic-on-OOM,
  C `MUST_CONVERT` encoding branches vs Rust UTF-8-only assumption). These are
  tracked separately so they can be reviewed and either promoted or fixed.

The tool currently validates 12 function pairs covering the main parser loop,
content/prolog/epilog/CDATA processors, handler dispatch, and entity processing.

The approach — normalizing two programs in different languages to a common IR and
verifying equivalence modulo explicit rewrite rules — is derived from the
multi-language synchronization algorithm described in
[Ennals (2007), "Multi-language synchronization"](https://dl.acm.org/doi/10.5555/1762174.1762217).

```bash
# Run the strict structural comparison
python3 validator/strict-ast-compare.py --ci

# Dump skeletons for a specific function pair (debugging)
python3 validator/strict-ast-compare.py --dump doContent do_content
```

### Where the AST comparison has residual risk

The AST comparison is strict by default: it requires the C and Rust skeletons
to be identical, and only allows differences that match an explicit rewrite rule.
This means semantic differences can only occur in two places:

- **Wrong rewrite rules**: A rule suppresses a C operation that should actually
  have a Rust equivalent. This is why temporary rules are tracked separately in
  `temporary-rewrites.json` — they are believed correct but not fully verified.
  Each should be reviewed and either promoted or removed.
- **Argument-level differences**: The tool matches calls by name but does not
  compare argument values. A call to `handler(data)` matches any call to
  `handler(...)`, even if the actual data differs (e.g., wrong slice bounds).
  Similarly, branch conditions are matched by the core variable being checked,
  not by the full expression.

These are deliberate trade-offs. Comparing argument expressions across languages
(C pointer arithmetic vs Rust slice indexing) would require a semantic model of
both languages. Instead, we constrain the structure tightly and rely on behavioral
testing to catch argument-level bugs.

### Value for AI-assisted porting

The AST comparison tool significantly simplifies AI-assisted porting work. When an
agent needs to port a C function to Rust, the tool provides:

- **Exactly which operations are missing** — specific call names, error codes,
  match arms, and handler dispatches that exist in C but not in Rust
- **Structural context** — where in the control flow each missing operation belongs
  (which match arm, which branch, what nesting level)
- **Prompt generation** — `--prompt` mode produces targeted porting instructions
  for each divergent function pair

This turns "port doContent to Rust" (a ~500-line function with ~50 match arms)
into a series of precise, verifiable tasks: "add XmlError::AsyncEntity return in
the TrailingCr arm after the tag_level check."

## Layer 2: Original C Test Suite

## Layer 3: Behavioral Comparison Tests

C is the ground truth. We run the same XML through both parsers and confirm
identical output. We don't write expected outputs by hand — we let the C library
define what "correct" means, then verify the Rust parser produces the same result.

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

## Fuzz Corpus Comparison

We run the OSS-Fuzz public corpora through both parsers — ~48,000 files
(24k UTF-8, 24k UTF-16LE) of raw inputs from continuous fuzzing of the C library.
This is the broadest behavioral comparison, covering malformed inputs, encoding
edge cases, and pathological byte sequences that handwritten tests would never
produce.

### What we verify

Three levels of comparison, in order of importance:

1. **SAX event identity** — for every input where both parsers return OK, we
   compare full SAX event traces (StartElement, EndElement, CharacterData,
   ProcessingInstruction, Comment) with adjacent CharacterData events merged.
   **Result: 0 divergences** across 109 files where both parsers accept the input.

2. **Status agreement** — both parsers must agree on whether each input is valid
   (OK) or invalid (ERROR). No false accepts, no false rejects.
   **Result: 0 disagreements** across all 47,934 files.

3. **Error code fidelity** — when both parsers reject an input, we compare the
   specific error code. This is a known issue (see below).
   **Result: ~85% exact match** (84% UTF-8, 86% UTF-16LE).

### Known issue: error code differences for invalid XML

About 7,200 of the ~48,000 fuzz corpus files produce different error codes from
the two parsers despite both agreeing the input is invalid. For example, Rust
may return `INVALID_TOKEN` where C returns `UNCLOSED_TOKEN`, or `INVALID_TOKEN`
where C returns `SYNTAX`.

The root causes:

1. **Prolog tokenizer coverage**: The Rust prolog tokenizer (`prolog_tok`)
   handles fewer sub-token states than C's. When it encounters a byte sequence
   it doesn't recognize, it returns `Invalid` or `Err` instead of `Partial`.
   C's tokenizer parses further into the DTD/prolog before giving up, so it
   reaches different error points and reports more specific errors like `SYNTAX`,
   `NO_ELEMENTS`, or `UNCLOSED_TOKEN`.

2. **UTF-8 normalization**: The Rust parser transcodes non-UTF-8 input to UTF-8
   before tokenizing. Both parsers perform the same encoding auto-detection and
   work with the same logical characters, but the UTF-8 tokenizer can hit error
   conditions at different token boundaries than C's encoding-specific
   tokenizers, producing different error codes for the same invalid input.

These differences **never** affect:
- Whether valid XML is accepted (SAX events always match)
- Whether invalid XML is rejected (status always agrees)
- The content of parsed output (element names, attributes, character data)

They only affect which specific error code is reported for XML that both parsers
agree is invalid. Applications that switch on specific error codes (rather than
just checking OK vs ERROR) may see different behavior.

**To fix this**, the main work needed is auditing `prolog_tok` in
`xmltok_impl.rs` against C's `normal_prologTok` in `xmltok_impl.c`, ensuring
all sub-token states (partial DTD constructs, conditional sections, etc.) return
the same token types. This is a tokenizer accuracy issue, not an architectural
one — the UTF-8 normalization approach is sound.

### Running

```bash
# Download corpora (one-time, ~100MB)
bash scripts/download-fuzz-corpus.sh corpus

# Run fuzz corpus comparison (status + SAX)
RUST_TEST_THREADS=1 cargo test -p expat-rust --test fuzz_corpus_comparison
RUST_TEST_THREADS=1 cargo test -p expat-rust --test fuzz_sax_comparison

# Run error code analysis (informational, not a pass/fail gate)
RUST_TEST_THREADS=1 cargo test -p expat-rust --test fuzz_corpus_analysis -- --nocapture
```

## Note on Encoding

The Rust parser transcodes all non-UTF-8 input to UTF-8 before tokenizing (unlike C, which tokenizes in the native encoding). This produces identical results for all inputs — see [design-decisions.md](design-decisions.md) for the rationale and [architecture.md](architecture.md) for details on byte offset handling.

## Limitations and Honest Assessment

This verification approach builds high confidence but does not constitute a proof
of equivalence. Known limitations:

1. **The AST comparison's guarantees are only as good as its rewrite rules.**
   The verifier requires identical structure by default, but each rewrite rule
   creates a documented gap where a difference is allowed. If a rule is wrong,
   it masks a real bug. The verified rules in `structural-rewrites.json` have
   been reviewed, but the temporary rules in `temporary-rewrites.json` are
   believed correct without full verification. For example, the rule that
   suppresses C's OOM return checks assumes Rust's panic-on-OOM is acceptable;
   if graceful OOM handling matters, this is a real behavioral difference.

2. **Behavioral tests cover a finite input space.** 463 comparison tests is a lot,
   but XML is infinitely varied. A bug that only manifests with a specific combination
   of DTD declarations, entity nesting, and encoding is unlikely to be caught by
   the test corpus.

3. **Some C features are not yet ported.** Suspend/resume (`XML_StopParser` /
   `XML_ResumeParser`) and multi-encoding external entity parsing are not
   implemented. The AST tool flags these as temporary suppressions rather than
   verified equivalences.

4. **The three layers are not fully independent.** The same developer wrote the
   Rust code, the comparison tests, and the rewrite rules. A systematic
   misunderstanding of C's semantics could propagate through all three.

Despite these limitations, the combination of structural constraint + behavioral
testing makes accidental divergence unlikely. The AST tool ensures the Rust code
is structurally very close to C, the behavioral tests verify outputs match across
a wide input space, and the original C test suite provides an independent quality
bar. A bug would need to hide in a structural gap that's also not exercised by
any of ~750 tests.
