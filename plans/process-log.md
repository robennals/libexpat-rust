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

### What to Try Next

- **Phase 2**: Port xmlrole.c (state machine) with Haiku first, see if quality holds
- **Phase 0.3**: Build test translation pipeline — this is the critical-path tool
- Try Haiku for translating simple C tests (misc_tests.c first)
