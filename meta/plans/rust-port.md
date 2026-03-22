# Plan: Port libexpat from C to Fully Safe, Idiomatic Rust

## Goal

Two goals, equally important:

1. **Port libexpat** to fully safe, idiomatic Rust with zero `unsafe`, passing all existing tests.
2. **Build a reusable C→Rust porting process** — scripts, documentation, and Claude Code skills — that can be applied to other C libraries. Libexpat is the proving ground.

## Philosophy: Scripts First, Sub-agents Where Needed

We maximize automation at every step:

- **Scripts first.** If a task can be done with a deterministic script (Python, bash, Rust build script), do it that way. Scripts are fast, cheap, reproducible, and debuggable.
- **Sub-agents for judgment calls.** Where a task requires reading code and making decisions (e.g., translating a C test to idiomatic Rust, designing a Rust type for a C struct), use Claude Code sub-agents.
- **Cheapest model that works.** Each sub-agent task should be tested with Haiku first. Only escalate to Sonnet/Opus if Haiku can't handle the quality bar. Document which model tier each task needs.
- **Everything is documented.** Each script, skill, and process step gets a doc in `plans/` so someone (human or AI) can repeat the process on a different codebase.

## Approach

Build a **parallel Rust codebase** in `expat-rust/` (separate from C in `expat/`). No FFI bridging — port module-by-module bottom-up, with Rust tests for each module. The full Rust parser only runs end-to-end once all modules are ported.

We rely on **tests for equivalence**:
1. The C test suite must be thorough enough to trust
2. We auto-translate C tests → Rust tests (scripts + sub-agents)
3. We augment C tests where coverage gaps exist

## Codebase Overview

### Core library (`expat/lib/`) — ~17K lines
| File | Lines | Role | Dependencies |
|------|-------|------|-------------|
| `xmlparse.c` | 9,267 | Main parser engine, public API | everything below |
| `xmltok.c` | 1,672 | Tokenizer (byte-level scanning) | `xmltok_impl.c`, `xmltok_ns.c`, char tables |
| `xmltok_impl.c` | 1,819 | Tokenizer impl (included by xmltok.c) | char tables |
| `xmltok_ns.c` | 123 | Namespace-aware tokenizer variants | xmltok_impl |
| `xmlrole.c` | 1,255 | DTD/prolog role state machine | ascii.h |
| `siphash.h` | 392 | SipHash for hash-flooding protection | standalone |
| `ascii.h` | 123 | ASCII character classification macros | standalone |
| `*tab.h` | ~265 | Character class lookup tables | standalone |
| `nametab.h` | 136 | Unicode name character tables | standalone |

### Test suite (`expat/tests/`) — ~16K lines
| File | Role |
|------|------|
| `basic_tests.c` | Core parsing |
| `ns_tests.c` | Namespace handling |
| `alloc_tests.c` | Allocation failure paths |
| `nsalloc_tests.c` | Namespace + alloc failures |
| `acc_tests.c` | Accounting/security |
| `misc_tests.c` | Edge cases |
| `runtests.c` | Test runner harness |
| `chardata.c`, `structdata.c` | Test helpers |
| `handlers.c` | Callback handler helpers |
| `common.c` | Shared test infra |
| `memcheck.c`, `minicheck.c` | Test framework + memory tracking |

### Other
- `xmlwf/` — CLI well-formedness checker
- `examples/` — Usage examples
- `fuzz/` — Fuzz harnesses
- `testdata/` — XML test files

## Dependency Graph (bottom-up porting order)

```
Layer 0 (leaves — no internal deps):
  ascii.h, *tab.h, nametab.h, siphash.h

Layer 1:
  xmltok_impl.c, xmlrole.c

Layer 2:
  xmltok.c (+ xmltok_ns.c)

Layer 3:
  xmlparse.c

Layer 4:
  xmlwf/, examples/
```

---

## Phase 0: Infrastructure & Test Foundation

### 0.1 — Rust project setup
**Method: script**
- [ ] Script: `scripts/init-rust-project.sh` — creates `expat-rust/Cargo.toml`, `src/lib.rs`, module stubs matching the dependency layers, `.cargo/config.toml`
- [ ] Verify `cargo check` passes on empty stubs

### 0.2 — Assess C test coverage
**Method: script + sub-agent (haiku)**
- [ ] Script: `scripts/analyze-c-tests.py` — parse C test files, extract test function names, count tests per file, identify which `XML_*` API functions are called
- [ ] Script: `scripts/build-and-run-c-tests.sh` — compile & run C tests, capture pass/fail counts
- [ ] Sub-agent (haiku): review coverage report, identify gaps, suggest which C tests to add
- [ ] Document findings in `plans/test-coverage-assessment.md`

### 0.3 — Build test translation pipeline
**Method: script + sub-agent (haiku for simple tests, sonnet for complex)**

This is the critical-path tool. It works in stages:

1. **Script: `scripts/extract-c-tests.py`** — parse C test files, extract each test function as a standalone block with its dependencies (callbacks, helpers, XML strings). Output: JSON list of test descriptors.

2. **Script: `scripts/translate-test-helpers.py`** — mechanically translate the test infrastructure:
   - `minicheck.h` macros → Rust `#[test]` boilerplate
   - `chardata`/`structdata` helpers → Rust equivalents
   - Common callback patterns → Rust closures
   - Output: `expat-rust/tests/helpers/` module

3. **Sub-agent (haiku first, escalate if needed): translate individual tests** — for each extracted C test, produce a Rust `#[test]` function that tests the same behavior against the Rust API. The sub-agent gets:
   - The C test source
   - The Rust test helper module
   - The Rust API signatures (stubs)
   - Instructions on idiomatic Rust test patterns

4. **Script: `scripts/validate-test-translation.py`** — check translated tests for:
   - All compile (against stubs that panic with `todo!()`)
   - No raw pointer usage
   - All original assertions preserved
   - Test count matches C test count

- [ ] Build and test this pipeline on `misc_tests.c` first (smallest, simplest)
- [ ] Then run on all test files
- [ ] Document the pipeline in `plans/test-translation-pipeline.md`
- [ ] Record which model tier was needed for each test file

### 0.4 — Golden output generator
**Method: script**
- [ ] Script: `scripts/generate-golden-outputs.py` — compile C expat, feed a corpus of XML inputs (from `testdata/` + generated edge cases), record:
  - Sequence of callback events (element start/end, character data, etc.)
  - Error codes and positions
  - Final parser state
- [ ] Output: `expat-rust/tests/golden/` directory with JSON golden files
- [ ] These provide an additional safety net beyond translated unit tests

### 0.5 — Set up fuzz infrastructure
**Method: script**
- [ ] Script: `scripts/init-fuzz.sh` — set up `cargo-fuzz` targets mirroring `expat/fuzz/`
- [ ] Targets won't run until parser is complete, but infrastructure is ready

---

## Phase 1: Port Leaf Modules (Layer 0)

### 1.1 — Character tables
**Method: script (fully mechanical)**
- [ ] Script: `scripts/port-char-tables.py` — parse the C `*tab.h` files and `ascii.h`, emit Rust const arrays and const fns
- [ ] These are pure data + simple macros — no judgment needed, fully scriptable
- [ ] Activate translated tests, verify they pass

### 1.2 — SipHash
**Method: sub-agent (haiku)**
- [ ] Sub-agent: port `siphash.h` to Rust, or evaluate `siphasher` crate
- [ ] Script: `scripts/verify-siphash.py` — generate test vectors from C version, verify Rust matches
- [ ] Decision: if `siphasher` crate output matches exactly, use it; otherwise port

---

## Phase 2: Port Tokenizer (Layers 1-2)

### 2.1 — Analyze tokenizer design
**Method: sub-agent (sonnet) — one-time analysis**
- [ ] Sub-agent: read `xmltok.c`, `xmltok_impl.c`, `xmltok.h`, document:
  - The `ENCODING` vtable pattern
  - The `#define`-based code generation
  - Proposed Rust trait design
- [ ] Output: `plans/tokenizer-design.md`
- [ ] This is a judgment-heavy task — needs sonnet

### 2.2 — Port xmlrole.c
**Method: sub-agent (haiku, escalate to sonnet if needed)**
- [ ] Sub-agent: port the state machine to Rust enum + match
- [ ] Script: run translated tests, verify pass
- [ ] This is a natural Rust fit — haiku should handle it

### 2.3 — Port xmltok_impl.c + xmltok_ns.c
**Method: sub-agent (sonnet) — complex pointer arithmetic**
- [ ] Sub-agent: port scanning functions, replace pointer arithmetic with slices
- [ ] This is the hardest tokenizer piece — pointer-heavy, macro-heavy
- [ ] Script: run translated tests, verify pass

### 2.4 — Port xmltok.c
**Method: sub-agent (sonnet)**
- [ ] Sub-agent: port tokenizer front-end and encoding detection
- [ ] Depends on 2.2 and 2.3 being done
- [ ] Script: run translated tests, verify pass

---

## Phase 3: Port Main Parser (Layer 3)

### 3.1 — Analyze and design
**Method: sub-agent (opus) — architectural decisions**
- [ ] Sub-agent: read `xmlparse.c`, produce:
  - Struct/type map with proposed Rust equivalents
  - Callback system design
  - Memory pool replacement strategy
  - Error type design
- [ ] Output: `plans/parser-design.md`
- [ ] This needs opus — it's 9K lines of complex stateful code with subtle ownership patterns

### 3.2 — Port parser in sub-sections
**Method: sub-agent (sonnet) per sub-section**

Break `xmlparse.c` into ~8 logical units, port each as a separate sub-agent task:
- [ ] Parser creation/config/destruction → `Parser::new()`, `Drop`
- [ ] Hash table → `HashMap`
- [ ] String pool → `String` / `Vec<u8>` / arena
- [ ] Content parsing (elements, attributes, text, CDATA)
- [ ] DTD parsing (entity declarations, notation, etc.)
- [ ] Namespace processing
- [ ] External entity handling
- [ ] Error handling, position tracking, suspend/resume

Each sub-agent gets:
- The relevant C source section
- The parser design doc from 3.1
- The already-ported lower-layer Rust modules
- Instruction to produce safe, idiomatic Rust

### 3.3 — Full integration testing
**Method: script**
- [ ] Activate all translated tests, verify they pass
- [ ] Run golden output comparison
- [ ] Fuzz testing with `cargo-fuzz`
- [ ] Miri for memory safety

---

## Phase 4: Port Utilities and CLI (Layer 4)

### 4.1 — Port xmlwf
**Method: sub-agent (haiku)**
- [ ] Sub-agent: rewrite as Rust binary using the Rust API
- [ ] Script: run `xmltest.sh` equivalent, compare golden outputs

### 4.2 — Port examples
**Method: sub-agent (haiku)**
- [ ] Sub-agent: rewrite examples as idiomatic Rust

---

## Phase 5: Polish and Harden

### 5.1 — Remove all `unsafe`
**Method: script + sub-agent (haiku)**
- [ ] Script: `scripts/find-unsafe.sh` — `grep -rn "unsafe"` in Rust code, report locations
- [ ] Sub-agent (haiku): for each `unsafe` block, propose safe alternative
- [ ] Iterate until zero `unsafe` remains

### 5.2 — Idiomatic Rust cleanup
**Method: script + sub-agent (haiku)**
- [ ] Script: run `cargo clippy -- -W clippy::pedantic`, collect warnings
- [ ] Sub-agent (haiku): fix clippy warnings, improve idioms
- [ ] Sub-agent (haiku): add doc comments to public API

### 5.3 — Optional: C FFI compatibility layer
**Method: sub-agent (sonnet)**
- [ ] Sub-agent: generate `extern "C"` wrappers matching `expat.h`
- [ ] Script: compile original C tests against Rust FFI, run

### 5.4 — Performance validation
**Method: script**
- [ ] Script: `scripts/benchmark.sh` — run benchmarks on C and Rust versions, compare
- [ ] Profile hot paths if regression > 2x

### 5.5 — Fuzz testing
**Method: script**
- [ ] Run `cargo fuzz` campaigns
- [ ] Compare findings with C version

---

## Scripts Inventory

All scripts live in `scripts/` and are documented with usage in their headers.

| Script | Phase | Purpose | Deterministic? |
|--------|-------|---------|---------------|
| `init-rust-project.sh` | 0.1 | Scaffold Rust crate | Yes |
| `analyze-c-tests.py` | 0.2 | Extract test metadata from C | Yes |
| `build-and-run-c-tests.sh` | 0.2 | Compile & run C tests | Yes |
| `extract-c-tests.py` | 0.3 | Parse C tests into descriptors | Yes |
| `translate-test-helpers.py` | 0.3 | Mechanical test infra translation | Yes |
| `validate-test-translation.py` | 0.3 | Check translated tests compile | Yes |
| `generate-golden-outputs.py` | 0.4 | Record C parser outputs as JSON | Yes |
| `init-fuzz.sh` | 0.5 | Set up cargo-fuzz targets | Yes |
| `port-char-tables.py` | 1.1 | Convert C tables to Rust consts | Yes |
| `verify-siphash.py` | 1.2 | Cross-check hash outputs | Yes |
| `find-unsafe.sh` | 5.1 | Locate unsafe blocks | Yes |
| `benchmark.sh` | 5.4 | Compare C vs Rust perf | Yes |

## Sub-agent Model Tiers

Each sub-agent task is assigned the cheapest model that produces acceptable output. This table records decisions made during the port — start with the suggested tier, escalate only if quality is insufficient.

| Task | Suggested Model | Reason |
|------|----------------|--------|
| Review test coverage gaps | Haiku | Simple analysis, low creativity |
| Translate simple C tests | Haiku | Mechanical transformation |
| Translate complex C tests (callbacks, state) | Sonnet | Needs judgment on Rust idioms |
| Port character tables | Script | Pure data, no LLM needed |
| Port siphash | Haiku | Small, well-defined algorithm |
| Analyze tokenizer design | Sonnet | Complex C patterns → Rust design |
| Port xmlrole.c (state machine) | Haiku → Sonnet | Try haiku first, escalate if needed |
| Port xmltok_impl.c (pointer-heavy) | Sonnet | Complex pointer arithmetic |
| Port xmltok.c (front-end) | Sonnet | Depends on trait design |
| Design parser architecture | Opus | 9K lines, subtle ownership |
| Port parser sub-sections | Sonnet | Large, stateful code |
| Port xmlwf CLI | Haiku | Simple consumer of API |
| Port examples | Haiku | Trivial |
| Fix clippy warnings | Haiku | Mechanical fixes |
| Add doc comments | Haiku | Formulaic |
| Generate C FFI wrappers | Sonnet | Needs exact ABI matching |

## Deliverables (Reusable for Other Ports)

At the end of this project, we'll have:

### Process documentation (`plans/`)
- `rust-port.md` — this plan (the master process doc)
- `test-translation-pipeline.md` — how to auto-translate C tests to Rust
- `test-coverage-assessment.md` — how to assess and fill C test gaps
- `tokenizer-design.md` — design decisions for tokenizer port
- `parser-design.md` — design decisions for parser port
- `model-tier-results.md` — which LLM model tier worked for each task type
- `c-to-rust-idiom-guide.md` — common C→Rust pattern translations

### Scripts (`scripts/`)
- Test extraction, translation, and validation pipeline
- Golden output generation
- Character table / data porting
- Coverage analysis
- Benchmark comparison
- Project scaffolding

### Skills (Claude Code)
- C test → Rust test translator (with model tier recommendation)
- C module → Rust module porter (with model tier recommendation)
- Rust safety auditor (find and fix `unsafe`)
- C→Rust design advisor (analyze C code, propose Rust architecture)

---

## Principles

1. **Scripts first.** If it can be scripted deterministically, script it. Sub-agents are for judgment calls only.
2. **Cheapest model that works.** Start with haiku, escalate only when quality demands it. Document what works.
3. **Parallel codebase.** Rust in `expat-rust/`, C untouched in `expat/`. No FFI bridging during development.
4. **Tests are the contract.** Behavioral equivalence through comprehensive translated tests + golden outputs.
5. **Bottom-up.** Port leaves first. Never port a module before its dependencies.
6. **Idiomatic from the start.** Redesign for Rust, don't mechanically translate. Validate via tests.
7. **Safe by default.** Goal: zero `unsafe`.
8. **Preserve semantics.** Expat's behavior on malformed input is part of its contract.
9. **Build tools as we go.** Anything done twice becomes a script or skill.
10. **Document everything.** The process is as valuable as the code.

## Open Questions

- **c2rust as starting point vs. clean rewrite?** For small modules, clean rewrite. For `xmlparse.c` (9K lines), c2rust might scaffold — but its output is so un-idiomatic it may not save time.
- **Encoding handling?** Rust is UTF-8 native. Expat handles UTF-8, UTF-16, Latin-1. Design the abstraction carefully.
- **Parser state machine?** `xmlparse.c` uses function-pointer dispatch. Rust options: enum + match, trait objects, or direct methods.
- **External entity handling?** User callbacks return new parsers. Ownership model needs careful design.
- **Test coverage gaps?** Assess first, then decide investment level.
- **Haiku ceiling?** We'll discover empirically which tasks haiku can handle. Document findings in `model-tier-results.md`.

## Success Criteria

1. All C tests pass as translated Rust tests against the Rust implementation
2. Zero `unsafe` blocks in final code
3. Fuzz testing finds no new bugs vs. C version
4. Performance within 2x of C (ideally ~1x)
5. Clean, documented, idiomatic Rust API
6. **Reusable**: scripts, skills, and process docs that work on a different C library with minimal adaptation
