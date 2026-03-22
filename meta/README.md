# Porting Process Artifacts

This directory contains the tooling, plans, and documentation from the C-to-Rust porting process.
These are preserved for transparency and reference — they are not required to build or use `expat-rust`.

## Directory Structure

### `scripts/` — Porting and verification tooling

The custom tools built to ensure correctness during the port:

| Script | Purpose |
|--------|---------|
| `ast-compare.py` | **Primary verification tool.** Compares the AST structure of C functions against their Rust counterparts — validates that switch cases, error paths, and handler calls match. |
| `port-function.py` | Call-tree analyzer. Shows which functions are ready to port (all dependencies already ported), extracts C source, generates porting prompts. |
| `generate-comparison-tests.py` | Generates FFI comparison tests that run the same XML input through both C and Rust parsers. |
| `extract-c-function.py` | Extracts individual C functions from `xmlparse.c` for analysis. |
| `extract-c-tests.py` | Extracts test cases from libexpat's C test suite for translation to Rust. |
| `validate-call-tree.py` | Validates that the Rust port maintains a 1:1 function correspondence with the C call tree. |
| `verify-test-coverage.py` | Checks which libexpat test cases have corresponding Rust tests. |
| `structural-compare.py` / `structural-compare-v2.py` | Earlier versions of the structural comparison tool. |
| `compare-functions.py` | Side-by-side function comparison utility. |
| `c2rust-pipeline.sh` | Orchestrator for the C2Rust transpilation pipeline. |
| `c2rust-cleanup.py` | Phase 1 cleanup of C2Rust output (mechanical type/syntax fixes). |
| `c2rust-analyze-patterns.py` | Analyzes patterns in C2Rust output to inform the porting strategy. |
| `extract-c2rust-functions.py` | Extracts functions from C2Rust output for comparison. |
| `transform-function.py` | Prepares functions for AI-assisted transformation. |
| `port-char-tables.py` | Specialized script for porting character classification tables. |
| `prepare-test-translation-prompt.py` | Generates prompts for translating C tests to Rust. |
| `deliberate-divergences.json` | Documented intentional differences between C and Rust behavior. |
| `xmlparse-compare.json` | C↔Rust function name mapping for AST comparison. |
| `run-tests.sh` / `run-tests-safe.sh` / `list-failures.sh` / `verify-port.sh` | Test runner utilities. |

### `plans/` — Porting plans and tracking

| Document | Purpose |
|----------|---------|
| `PORTING-PROCESS.md` | Master document describing the porting methodology. |
| `porting-status.md` | Function-by-function porting status tracker. |
| `rust-port.md` | Original porting plan and architecture decisions. |
| `c2rust-port-plan.md` | Plan for using C2Rust as a correctness reference. |
| `call-tree-overrides.md` | Exceptions to the 1:1 call-tree mapping rule. |
| `process-log.md` | Chronological log of porting progress. |
| `test-coverage-assessment.md` | Analysis of test coverage gaps. |
| `test-translation-pipeline.md` | Plan for translating C tests to Rust. |
| `model-tier-results.md` | Comparison of different AI models for porting tasks. |

### `agents/` — AI agent prompts

Prompts used to direct AI agents during the porting process:

| Prompt | Purpose |
|--------|---------|
| `port-leaf-module.md` | Agent instructions for porting leaf modules (ascii, char_tables, etc.). |
| `port-state-machine.md` | Agent instructions for porting state machine code (xmlrole, processors). |
| `port-data-tables.md` | Agent instructions for porting data tables. |
| `fix-test-batch.md` | Agent instructions for fixing batches of failing tests. |
| `translate-c-tests.md` | Agent instructions for translating C test cases to Rust. |
| `analyze-c-code.md` | Agent instructions for analyzing C code structure. |
| `c2rust-transform.md` | Agent instructions for transforming C2Rust output to idiomatic Rust. |

### `c2rust-output/` — C2Rust transpiler output

Mechanically-generated Rust code produced by the C2Rust transpiler. This code is unsafe and non-idiomatic,
but served as a correctness reference during the porting process — when questions arose about what the C code
actually does (vs. what it appears to do), this machine-generated translation provided ground truth.

### `analysis/` — Architecture analysis

- `ARCHITECTURE_ANALYSIS.md` — Comprehensive analysis of the C codebase: data structures, state machines, control flow, memory management patterns. This informed the porting strategy.

### `c2rust-pipeline/` — C2Rust pipeline documentation

- `C2RUST_README.md` — Documentation for the C2Rust transpilation pipeline and how it was used.
