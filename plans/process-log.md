# C-to-Rust Porting Process Log

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

#### Key insight: The most effective workflow for correctness
The previous approach (agent writes Rust from reading C) produces code that *looks* right
but has subtle behavioral differences. The C2Rust approach provides a verification loop:

1. Write a **comparison test** for the behavior
2. If it fails, look at the **C2Rust output** to see what C actually does
3. Fix the **idiomatic Rust port** to match
4. Re-run comparison tests to verify

The C2Rust output is NOT used directly — it's 39K lines of unsafe Rust with raw pointers
and C naming. It's a searchable, compilable reference for understanding C behavior when
it's unclear.

#### Key insight: C2Rust output naming vs Rust port naming
Zero functions overlap by name between C2Rust output and existing port:
- C2Rust preserves C names: `XML_Parse`, `doContent`, `(*parser).m_errorCode`
- Rust port uses idiomatic names: `parse`, `do_content`, `self.error_code`
Use `scripts/extract-c2rust-functions.py --compare` to map between them.

#### Key insight: C data structures → Rust idioms
C2Rust preserves C's custom hash tables (open-addressing, manual malloc). In the Rust
port, use `HashMap<String, T>` instead. Same for STRING_POOL → `String`/`Vec<u8>`.
Don't port the C data structure implementations.

#### Tools created:
- `scripts/c2rust-pipeline.sh` — orchestrator (compare, extract, analyze, functions, cleanup)
- `scripts/c2rust-cleanup.py` — mechanical type cleanup (c_int→i32, c_char→i8, etc.)
- `scripts/extract-c2rust-functions.py` — extract functions, compare with existing port
- `scripts/c2rust-analyze-patterns.py` — count patterns (2,729 ptr derefs, 120 fn ptr calls, etc.)
- `scripts/transform-function.py` — prepare transformation prompts with context
- `agents/c2rust-transform.md` — haiku agent prompt for function transformation

#### Pattern analysis results (xmlparse.rs):
| Pattern | Count | Rust replacement |
|---------|-------|-----------------|
| `(*parser).m_field` | 2,729 | `self.field_name` |
| `as c_int` casts | 1,508 | Native types |
| Function pointer calls | 120 | Trait/enum dispatch |
| `current_block` (goto) | 119 | Loop/match/return |
| malloc/free | 66 | Vec/Box/String |

### What to Try Next

- **Port DTD support** (~98 tests blocked) — see `plans/c2rust-port-plan.md`
- **Port external entity support** (~30 tests)
- **Port stop/resume** (~10 tests)
- Continue using comparison tests as the verification mechanism
