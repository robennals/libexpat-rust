# C-to-Rust Porting Process Log

This project is primarily a **test case for learning the best methods** for porting C to
Rust using AI agents. The transferable insights about methodology matter more than the
port itself.

## Transferable Lessons for C-to-Rust Porting

### Lesson 1: AI-written ports look correct but have subtle behavioral bugs

Agent-generated Rust code that ports C tends to:
- Use the right function names and follow the right structure
- Handle the happy path correctly
- Miss edge cases around finalization, empty input, error propagation, and state transitions

These bugs are **invisible to code review** — the code reads as a plausible translation.
You can only catch them by running the C and Rust code side by side on the same inputs.

### Lesson 2: Build a C FFI test harness first, before porting anything

The single most valuable thing you can do when porting C to Rust is:
1. Create a `-sys` crate that builds the C library from source (using the `cc` crate)
2. Write a safe Rust wrapper around the C API
3. Write comparison tests that run identical inputs through both implementations

This took ~30 minutes to set up and immediately found 3 bugs that had survived multiple
rounds of agent-written code and manual review. **Do this before writing any port code.**

The pattern is:
```rust
fn compare(input: &[u8]) {
    let rust_result = rust_parser.parse(input, true);
    let c_result = c_parser.parse(input, true);
    assert_eq!(rust_result, c_result);
}
```

Extend to compare error codes, positions, and handler callback output — not just
success/failure.

### Lesson 3: C2Rust adds modest value; the C FFI harness is what matters

C2Rust (mechanical C→Rust transpiler) sounds appealing but in practice:
- **Installation is painful** (requires specific LLVM version, macOS compatibility issues)
- **Output is unusable directly** (39K lines of unsafe Rust with raw pointers and C naming)
- **Reading C source is just as good** as reading C2Rust output for understanding behavior
- **The real C library** (linked via FFI) is a better reference than any transpilation

If you only have time for one thing, build the FFI test harness. Skip C2Rust.

### Lesson 4: Haiku agents are excellent for leaf modules, not for core logic

Haiku reliably ported: data tables, hash functions, character classification, simple state
machines. These are well-defined, have clear test vectors, and don't require understanding
the broader system.

Haiku-ported code for the core parser (xmlparse.c) had the behavioral bugs described above.
The core logic involves complex state machines, cross-function invariants, and subtle
finalization semantics that require more careful verification.

### Lesson 5: The verification workflow matters more than the generation method

Whether code is written by Haiku, Sonnet, Opus, or a human, the same verification workflow
applies:
1. Write a comparison test for the specific behavior
2. Run it — does Rust match C?
3. If not, read the C source to understand the correct behavior
4. Fix the Rust code
5. Re-run to confirm, check for regressions

The quality of the initial generation matters less than having a tight feedback loop.
A mediocre first draft with good verification beats a careful first draft with no verification.

### Lesson 6: Test the C library's actual behavior, not what the C source says

The C source can be misleading due to preprocessor macros, implicit conversions, and
platform-specific behavior. The only reliable ground truth is what the compiled C library
actually does when you feed it input. This is why the FFI harness (running real C code)
is more valuable than reading C source or C2Rust output.

Example: We initially thought `<doc>\r` with is_final=true should succeed (the Rust test
said so). The C test suite explicitly expects it to fail. Only by running the C library
did we discover the correct behavior.

### Lesson 7: Port data structures idiomatically, not literally

C2Rust preserves C's manual hash tables, string pools, and linked lists. The Rust port
should use `HashMap`, `String`, `Vec` instead. Don't port the C data structure
implementations — they exist because C lacks standard library equivalents.

### Lesson 8: Comparison tests should cover handlers, not just status codes

Parsing can "succeed" (return Ok) but deliver wrong data to handlers. Test that character
data, element names, and attribute values delivered to callbacks match between C and Rust.

## What We've Done So Far

### Phase 0.1: Rust Project Scaffolding
**Method: Direct (Opus)**
- Created `expat-rust/` with Cargo.toml, module stubs matching dependency layers
- Installed Rust toolchain (rustup)
- Verified `cargo check` passes on empty stubs
- **Time: ~2 minutes**

### Phase 0.2: C Test Coverage Assessment
**Method: Haiku subagent (research-only)**
- Analyzed all 6 C test files, counted 391 tests total
- Mapped XML_* API functions under test (67 functions)
- Identified coverage gaps and complexity tiers
- Recommended translation priority order
- **Model: Haiku worked perfectly** — this is pure analysis, no creativity needed

### Phase 1.1: Port Character Tables (Layer 0)
**Method: Haiku subagent**
- Haiku wrote a Python script (`scripts/port-char-tables.py`) to parse C headers
- Script generated `char_tables.rs` with ByteType enum + 4 const arrays
- All 512 values (4 tables × 128 entries) parsed and generated correctly
- **Model: Haiku worked perfectly** — mechanical transformation

### Phase 1.1b: Port Name Tables (Layer 0)
**Method: Haiku subagent**
- Direct port of `nametab.h` → `nametab.rs`
- Three const arrays: NAMING_BITMAP (256 u32), NMSTRT_PAGES (240 u8), NAME_PAGES (240 u8)
- **Model: Haiku worked perfectly** — pure data copy

### Phase 1.2: Port SipHash (Layer 0)
**Method: Haiku subagent**
- Ported siphash.h to idiomatic Rust with zero `unsafe`
- Both function API (`sip_hash()`) and struct API (`SipHasher`)
- All 64 official SipHash-2-4 test vectors pass
- Incremental hashing tests pass
- **Model: Haiku worked perfectly** — well-defined algorithm with clear test vectors

## Process Insights

### What Worked Well

1. **Parallel Haiku subagents are extremely effective for Layer 0 modules**
   - All 4 agents ran in parallel (~2.5 minutes wall time)
   - Total tokens: ~164K across all agents (cheap)
   - Zero quality issues — all code compiled and tests passed first try

2. **The "research-only vs. write-code" distinction matters**
   - Test analysis agent was told explicitly "Do NOT write files"
   - This kept it focused and fast (81s vs 157s for code-writing agents)

3. **Clear, self-contained prompts are key**
   - Each agent got: exact file paths, the task, verification command
   - No agent needed to ask questions or make judgment calls

4. **Scripts for mechanical transforms, agents for judgment calls**
   - The char-tables agent chose to write a Python script (good!)
   - The siphash agent wrote Rust directly (also good — algorithm translation needs judgment)

### Model Tier Findings

| Task | Model Used | Quality | Notes |
|------|-----------|---------|-------|
| Test coverage analysis | Haiku | Excellent | Pure analysis |
| Character table porting | Haiku | Excellent | Wrote a Python script + ran it |
| Name table porting | Haiku | Excellent | Direct data copy |
| SipHash porting | Haiku | Excellent | Algorithm with test vectors |
| Project scaffolding | Opus (orchestrator) | N/A | Needed for planning + coordination |

**Key finding: Haiku handles ALL Layer 0 tasks.** The plan suggested Haiku for these, and that was confirmed. We haven't yet tested Haiku on Layers 1-2 (tokenizer, state machine) where the plan suggests Sonnet may be needed.

### Prompt Patterns That Work

1. **For data/table porting**: "Read [C file], write Rust equivalent at [path], verify with cargo check"
2. **For algorithm porting**: "Read [C file], write idiomatic safe Rust at [path], add tests, verify with cargo test"
3. **For analysis**: "Read [files], return summary of [specific questions]. Do NOT write files."
4. **Always include**: Exact file paths, verification commands, clear success criteria

### Phase 2.2: Port xmlrole.c (Layer 1)
**Method: Haiku subagent (background)**
- Ported 1255 lines C → 1227 lines idiomatic Rust
- State machine → Rust enum + match dispatch
- **Model: Haiku worked** — clean state machine is a natural fit
- **Key: Running as background agent** while other work proceeded in parallel

### Phase 0.3: Test Translation Pipeline
**Method: Scripts + Haiku subagents**
- Built `scripts/extract-c-tests.py` — extracts all 391 tests to JSON descriptors
- Built `scripts/prepare-test-translation-prompt.py` — generates prompts for AI agents
- Generated Rust API facade (xmlparse.rs) with todo!() stubs — tests compile against this
- Launched parallel haiku agents for misc_tests (22) and ns_tests (33) translation
- **Pipeline**: extract JSON → generate prompt → haiku translates → cargo check validates

### Phase 2.1: Tokenizer Analysis
**Method: Haiku subagent (research-only)**
- Analyzed xmltok_impl.c structure: 28 functions, template file included 3x
- Identified trait-based Rust design (generic over encoding)
- Currently testing Haiku on this — it's the complexity threshold test

### Quality Audit Process
**Method: clippy + Haiku subagents**

After porting, code quality must be audited:
1. Run `cargo clippy` to get baseline warnings
2. Launch parallel haiku agents to fix clippy warnings per-file
3. Verify tests still pass after fixes
4. Run `scripts/verify-test-coverage.py` to ensure 1:1 test mapping

**Finding**: Initial haiku output had 58 clippy warnings. Main categories:
- Complex handler types → fix with type aliases
- Manual bit rotation → use .rotate_left()
- Manual range checks → use .contains()
- Identity operations (<<0) → remove

**Lesson**: Haiku optimizes for "compiles and works" not "idiomatic Rust."
Add clippy as a verification step AFTER each port, not just cargo check.

### Revised Workflow (Proven)

For each C module:
1. **Analyze** (haiku, research-only) → understand structure
2. **Port** (haiku, write code) → translate to Rust
3. **Compile** (`cargo check`) → catch type/syntax errors
4. **Audit** (`cargo clippy` + haiku fix agent) → enforce idioms
5. **Test** (`cargo test`) → verify behavior
6. **Verify coverage** (`verify-test-coverage.py`) → ensure 1:1 test mapping

### Phase 3: C2Rust Pipeline for Correctness Verification
**Method: Opus orchestration + tooling**

The agent-written port had correctness issues that were hard to catch by review. We
needed a systematic way to verify behavior matches C exactly.

#### What was done:
1. **Installed C2Rust 0.22.1** with LLVM 17 (brew install llvm@17)
   - C2Rust requires LLVM 14-17, not the latest — LLVM 22 fails with `ElaboratedType` errors
   - cmake generates `compile_commands.json`; must strip `-arch arm64` and `-target` flags
   - macOS ARM `long double` (f128) needed a manual fix → replaced with `f64`
2. **Transpiled all 3 C source files** → 39K lines of mechanically correct unsafe Rust
   - xmlparse.rs (12,621 lines), xmlrole.rs (3,695 lines), xmltok.rs (22,702 lines)
   - Output compiles on nightly Rust (needs `#![feature(extern_types, raw_ref_op)]`)
3. **Created expat-sys crate** — builds C library from source via `cc` crate, provides safe `CParser` wrapper
4. **Created 59 comparison tests** — run same XML through both Rust port and C library
   - 56/59 passing, 3 failures all DOCTYPE (not yet implemented)
5. **Found and fixed 3 bugs** via comparison tests:
   - Empty input with is_final=true: was returning Ok, should be Error(NoElements)
   - Unclosed tags with is_final=true: was returning Ok, should be Error(NoElements)
   - Two test expectations were testing old incorrect behavior → corrected
6. **Created transformation scripts** for future porting work

#### Key insight: The real value was expat-sys + comparison tests, not C2Rust itself

The **most valuable thing built** was the `expat-sys` crate and the comparison test suite.
C2Rust was the motivating reason to set this up, but in hindsight could have been skipped.

**What expat-sys does**: It builds the real C libexpat from source using the `cc` crate
and provides a safe Rust wrapper (`CParser`) with the same API shape as our Rust port.
This means you can write a test that parses the same XML through both parsers and compare
the results — status codes, error codes, line/column positions, even handler callback output.

**Why this matters**: Agent-written Rust code that ports C *looks* correct — it uses the
right function names, follows the right control flow, handles the same cases. But it has
subtle behavioral differences that are invisible to code review. The only reliable way to
catch these is to run the C code and the Rust code side by side on the same inputs and
compare outputs.

**What C2Rust added on top**: Honestly, modest value. The C2Rust output is a searchable
Rust-syntax version of the C code, which is slightly easier to read than C when you're
already in Rust-brain mode. But reading the C source directly works fine too. The C2Rust
installation was painful (LLVM version compatibility, macOS ARM issues) and the output is
39K lines of unsafe unidiomatic Rust that can't be used directly. If doing this again,
skip C2Rust and go straight to building expat-sys + comparison tests.

#### The bug-finding method in detail

Here is exactly how comparison tests found and fixed bugs:

**Step 1: Write comparison tests.** Each test parses a snippet of XML through both parsers:
```rust
fn compare_status(xml: &[u8], description: &str) {
    let (r_status, r_error, _, _) = parse_rust(xml);  // our Rust port
    let (c_status, c_error, _, _) = parse_c(xml);      // real C library
    assert_eq!(rust_status_to_u32(r_status), c_status,
        "STATUS MISMATCH for {description}");
}
```

**Step 2: Run them.** Three tests failed immediately:

```
cmp_empty_input:     Rust: status=1(Ok) error=0  |  C: status=0(Error) error=3(NoElements)
cmp_unclosed_tag:    Rust: status=1(Ok) error=0  |  C: status=0(Error) error=3(NoElements)
cmp_simple_doctype:  Rust: status=0(Error) error=2(Syntax)  |  C: status=1(Ok) error=0
```

**Step 3: Diagnose using the C source (or C2Rust output) as reference.**

For `cmp_empty_input` (empty string with is_final=true): The Rust `scan_buffer()` had:
```rust
if self.is_final && !self.seen_root && !data.is_empty() {  // BUG: !data.is_empty()
```
The `!data.is_empty()` guard was wrong — empty final input should still be an error.
The C code in `contentProcessor` has no such guard; it checks `startTagLevel == 0` and
returns `XML_ERROR_NO_ELEMENTS` unconditionally when final and no root element seen.

For `cmp_unclosed_tag` (`<a>` with is_final=true): The Rust code pushed the tag onto
`tag_stack` but never checked at end-of-parse whether tags were still open. The C code
checks `parser->m_tagLevel != startTagLevel` in `doContent` and returns `XML_ERROR_NO_ELEMENTS`.
Fix: Added `if self.is_final && !self.tag_stack.is_empty()` check at end of `scan_buffer`.

For `cmp_simple_doctype`: Expected failure — DOCTYPE is not implemented yet.

**Step 4: Fix and verify.** After each fix, re-run the full test suite to confirm no
regressions and the comparison test now passes.

**Step 5: Also found 2 tests with wrong expectations.** The comparison tests revealed
that `test_trailing_cr` and `test_utf8_auto_align` in the existing test suite were
testing the old *incorrect* Rust behavior. The C test suite explicitly expects these to
fail (`if (... == XML_STATUS_OK) fail("Failed to fault unclosed doc")`). Fixed the Rust
test expectations to match C.

#### Recommendation for future porting work

1. **Always build comparison tests first** before porting new functions. Write the test,
   confirm it fails (because the feature isn't implemented), then implement until it passes.
2. **Use expat-sys for ground truth**, not C2Rust output or reading the C source. The C
   library's actual behavior is the only reliable reference — even the C source can be
   misleading due to preprocessor macros, implicit type conversions, and platform-specific
   behavior.
3. **Test handlers, not just status codes.** The `cmp_chardata_entities` test compares
   actual character data delivered to handlers, not just whether parsing succeeded. This
   catches cases where parsing "succeeds" but delivers wrong data.

#### C2Rust output — limited but available

The C2Rust output exists in `c2rust-output/src/` and compiles on nightly. It's useful for:
- Grepping function names to find where logic lives
- Getting a rough line count for porting effort estimation
- Pattern analysis (how many pointer derefs, function pointer calls, etc.)

Not useful for:
- Direct use (unsafe, unidiomatic, C naming)
- Understanding intent (the C source with comments is clearer)

#### Tools created:
- `scripts/c2rust-pipeline.sh` — orchestrator (compare, extract, analyze, functions, cleanup)
- `scripts/c2rust-cleanup.py` — mechanical type cleanup (c_int→i32, c_char→i8, etc.)
- `scripts/extract-c2rust-functions.py` — extract functions, compare with existing port
- `scripts/c2rust-analyze-patterns.py` — count patterns (2,729 ptr derefs, 120 fn ptr calls, etc.)
- `scripts/transform-function.py` — prepare transformation prompts with context
- `agents/c2rust-transform.md` — haiku agent prompt for function transformation

#### Key insight: C data structures → Rust idioms
C2Rust preserves C's custom hash tables (open-addressing, manual malloc). In the Rust
port, use `HashMap<String, T>` instead. Same for STRING_POOL → `String`/`Vec<u8>`.
Don't port the C data structure implementations.

### Phase 4: Systematic Test Fixing (Current)
**Method: Parallel haiku agents with categorized batches**

#### Lesson 9: Don't jump in and manually port — build infrastructure first
When faced with a large porting task (9K line C function), the instinct is to start reading
code and writing the port. This doesn't scale. Instead:
1. Build extraction scripts that produce agent-ready prompts
2. Build verification scripts that check compile + test status
3. Categorize failures to find root causes
4. Fix root causes in parallel via haiku agents

#### Lesson 10: Fix root causes, not individual tests
The prolog_tok whitespace bug caused 7+ test failures across multiple files. Fixing one
function unlocked 7 tests — much more efficient than debugging each test individually.

#### Lesson 11: The port was further along than it appeared
Before starting Phase 4, the code already had:
- DTD data structures (Entity, ElementType, AttributeId, Dtd)
- Prolog state machine fields (prolog_state, doctype_name, etc.)
- A do_prolog function handling all role cases
- Working content_processor and epilog_processor using the proper tokenizer

The main issues were tokenizer bugs (prolog_tok whitespace), encoding handling, and
position tracking — not missing architecture.

#### Tools created:
- `scripts/extract-c-function.py` — extract C functions with context, list functions
- `scripts/verify-port.sh` — quick/full verification (compile, clippy, tests, comparison, unsafe audit)
- `scripts/list-failures.sh` — list all failing tests grouped by file
- `agents/fix-test-batch.md` — haiku agent prompt template for fixing test batches

#### Test status progression:
| Phase | Passed | Failed | Ignored |
|-------|--------|--------|---------|
| Before Phase 4 | 122 | 62 | 289 |
| After prolog_tok whitespace fix | 131 | 53 | 289 |
| After parallel fix agents | TBD | TBD | TBD |

### What to Try Next

- **Fix remaining ~50 test failures** via parallel haiku agents (encoding, position, ns)
- **Enable ignored tests in batches** by feature (DTD, external entities, stop/resume)
- **Port missing features** for ignored tests (unknown encoding, custom allocator)
- Continue using comparison tests as the verification mechanism
