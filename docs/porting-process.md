# Porting Process

How `expat-rust` was built: a complete C-to-Rust reimplementation of libexpat, using AI-assisted development with rigorous automated verification.

## Motivation

libexpat is the world's most widely deployed XML parser — embedded in Python, Apache, Firefox, Git, D-Bus, and hundreds of other projects. It has an excellent track record, but as a C99 library with ~9,200 lines of manual memory management, it has been subject to recurring memory safety CVEs.

A Rust reimplementation eliminates entire classes of vulnerabilities (buffer overflows, use-after-free, double-free) while preserving the proven parsing logic and SAX callback API that millions of users depend on.

We chose to reimplement rather than wrap because:
- FFI wrappers still execute the C code — they don't fix the underlying safety issues
- Existing Rust XML parsers (quick-xml, xml-rs, roxmltree) don't replicate libexpat's behavior, especially around DTD processing, entity expansion, and error handling
- A behavioral-equivalent Rust implementation can serve as a genuine drop-in replacement

## Methodology

### Phase 1: Architecture Analysis

The C codebase was analyzed to understand module boundaries, data flows, and state machines before any code was written.

Key findings that shaped the porting strategy:
- **Layered architecture**: The parser has clean layers — character tables → tokenizer → role state machine → main parser — that could be ported bottom-up
- **State machine as function pointers**: C uses function pointers for state transitions; Rust uses enums
- **Manual memory management everywhere**: String pools, hash tables, and buffer management — all replaced with `String`, `Vec`, `HashMap`
- **Preprocessor complexity**: Heavy use of `#ifdef`, `#include` for code reuse (e.g., `xmltok_impl.c` is included by `xmltok.c` three times with different macros). The Rust port uses explicit encoding parameters instead.

The full analysis is preserved in [`meta/analysis/ARCHITECTURE_ANALYSIS.md`](../meta/analysis/ARCHITECTURE_ANALYSIS.md).

### Phase 2: Bottom-Up Porting

Modules were ported in dependency order, ensuring each layer was verified before building on it:

```
Layer 0 (leaves):  ascii, char_tables, nametab, siphash
Layer 1:           xmltok_impl (tokenizer), xmlrole (prolog state machine)
Layer 2:           xmltok (encoding detection, token interface)
Layer 3:           xmlparse (main parser, public API)
```

Each module was ported by:
1. Extracting the C function source
2. Understanding its behavior (including preprocessor-expanded form)
3. Writing the Rust equivalent using idiomatic data structures
4. Verifying structural equivalence with AST comparison
5. Running behavioral comparison tests

### Phase 3: Structural Verification

A custom AST comparison tool (`meta/scripts/ast-compare.py`) was built to verify that each Rust function structurally matched its C counterpart. It compared:

- **Switch/match arms**: Every case in a C `switch` had a corresponding arm in the Rust `match`
- **Error paths**: Every error code returned by C was returned by Rust in equivalent conditions
- **Handler calls**: Every callback invocation in C had a corresponding call in Rust
- **Control flow**: Break/continue/return patterns matched

This caught many subtle bugs that behavioral tests alone would have missed — for example, a missing error path that happened to be unreachable with current test inputs but could be triggered by future inputs.

### Phase 4: Behavioral Verification

463 FFI comparison tests drive the correctness guarantee. Each test:

1. Creates both a C parser (via `expat-sys` FFI bindings) and a Rust parser
2. Registers equivalent handlers on both
3. Feeds identical XML input to both parsers
4. Compares every output: handler call sequence, error codes, byte positions, parsed text

Test categories cover:
- Well-formed XML (elements, attributes, text, CDATA, PIs, comments)
- Malformed XML (every error code path)
- Character encodings (UTF-8, UTF-16 LE/BE, ISO-8859-1)
- DTD processing (entity declarations, element/attribute declarations, notations)
- Namespace handling
- Security features (billion laughs protection, entity expansion limits)
- Edge cases (empty documents, huge attributes, deeply nested elements)

### Phase 5: C2Rust as Correctness Reference

The [C2Rust](https://c2rust.com/) transpiler was used to produce a mechanically-correct Rust translation of the C source. This translation was unsafe and non-idiomatic, but it served as ground truth when questions arose about C behavior.

This was particularly valuable for:
- Implicit type conversions that change semantics
- Preprocessor macro expansions that aren't visible in the source
- Platform-specific behavior (e.g., signed vs unsigned char)
- Pointer arithmetic and buffer boundary conditions

## AI-Assisted Development

AI (Claude) was used throughout the porting process as a development accelerator, not as an unsupervised code generator. The key principles:

- **Human-guided architecture**: All design decisions (data structure choices, module boundaries, API design) were made by humans. AI implemented those decisions.
- **Automated verification at every step**: No AI-generated code was accepted without passing both structural comparison and behavioral tests.
- **Bottom-up trust**: AI ported leaf modules first. Only after those were verified did it work on higher-level modules that depended on them.
- **Specialized agents**: Different AI agent configurations were used for different tasks — leaf module porting, state machine translation, test translation — each with task-specific instructions and constraints.

## Statistics

| Metric | Value |
|--------|-------|
| C source analyzed | ~9,200 lines (`xmlparse.c`) + ~5,000 lines (tokenizer, role, support) |
| Rust code produced | ~11,800 lines |
| Comparison tests | 463 |
| C test suite | 286/291 pass (5 skipped: C allocator APIs) |
| `unsafe` blocks | 0 |
| Production dependencies | 0 |
| Modules ported | 8 (ascii, char_tables, nametab, siphash, xmltok_impl, xmlrole, xmltok, xmlparse) |
| Verified against | libexpat 2.7.5 (tag R_2_7_5) |

## Lessons Learned

1. **Structural comparison catches bugs that behavioral tests miss.** Tests only cover exercised paths. Structural comparison ensures the code handles every case the C code handles, even if no test hits it yet.

2. **C2Rust is invaluable as a reference, not a starting point.** The transpiler output is too unsafe and unidiomatic to use directly, but it resolves ambiguities about what the C actually does.

3. **Bottom-up porting with verification at each layer works well.** Bugs in leaf modules would have cascaded upward, making higher-level debugging nearly impossible. Verifying each layer first kept the problem tractable.

4. **The preprocessor is the hardest part of C-to-Rust porting.** libexpat uses `#include` to include C files multiple times with different macro definitions (a form of generics). Understanding these patterns required careful manual analysis.

5. **AI-assisted development with rigorous verification produces high-quality results.** The AI accelerated implementation significantly, but the verification tooling — not the AI — is what provides confidence in correctness.
