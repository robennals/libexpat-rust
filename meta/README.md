# Porting Process Artifacts

This directory contains the tooling, plans, and documentation from the C-to-Rust porting process.
These are preserved for transparency and reference — they are not required to build or use `expat-rust`.

## Directory Structure

### `scripts/` — Porting and verification tooling

The custom tools built to ensure correctness during the port:

| Script | Purpose |
|--------|---------|
| `ast-compare.py` | **Primary verification tool.** Compares C functions against Rust counterparts — validates switch cases, error paths, handler calls, and function calls all match. Generates Haiku agent prompts with C source. See detailed usage below. |
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

## AST Compare Tool — Detailed Usage

`ast-compare.py` is the primary tool for identifying what's missing in the Rust port. It compares:
- **Match/switch cases** — which token/role cases C handles vs Rust
- **Error codes per case** — which `XML_ERROR_*` appear in C but not Rust
- **Handler calls per case** — which `parser->m_*Handler` calls are missing
- **Function calls per case** — which C functions are called but have no Rust equivalent

### Commands

```bash
# Show all divergences across all tracked function pairs
python3 meta/scripts/ast-compare.py --all

# Compare a specific C↔Rust function pair
python3 meta/scripts/ast-compare.py doContent do_content

# For split functions (doProlog → do_prolog + handle_prolog_role)
python3 meta/scripts/ast-compare.py doProlog do_prolog handle_prolog_role

# List all cases in a function (C or Rust)
python3 meta/scripts/ast-compare.py --list-cases doContent c
python3 meta/scripts/ast-compare.py --list-cases do_content rust

# Generate a Haiku-ready prompt with C source for each divergence
python3 meta/scripts/ast-compare.py --prompt doContent do_content

# Generate prompts for ALL divergent function pairs
python3 meta/scripts/ast-compare.py --prompt-all

# JSON output for programmatic use
python3 meta/scripts/ast-compare.py doContent do_content --json
```

### Interpreting Output

Severity levels:
- **HIGH**: Missing match arms — entire C cases with no Rust counterpart
- **MEDIUM**: Overall missing errors/handlers/calls — present somewhere in C but not Rust
- **LOW**: Per-case divergences — specific case handles an error/handler/call in C but not Rust

### Using `--prompt` to Direct Haiku Agents

The `--prompt` command generates a complete task description for a Haiku agent:
1. Lists which file and function to modify
2. Shows each divergent case with the **full C source code**
3. Lists missing errors, handlers, and function calls
4. Includes build verification command

**Workflow:**
```bash
# 1. Generate the prompt
python3 meta/scripts/ast-compare.py --prompt doContent do_content > /tmp/task.md

# 2. Review the prompt to prioritize which fixes to include
cat /tmp/task.md

# 3. Dispatch a Haiku agent with the prompt (sequentially, not parallel!)
# The agent runs in the current working branch, building on previous commits

# 4. After the agent completes, verify:
cargo build -p c-tests-runner 2>&1 | tail -3
./target/debug/c-tests-runner 2>/dev/null | grep -c "^PASS:"

# 5. Commit with test count in message, then repeat
```

### How Function Call Mapping Works

C function names are auto-converted to Rust snake_case:
- `doContent` → `do_content`
- `processEntity` → `process_entity`
- `callStoreEntityValue` → `call_store_entity_value`

Exceptions (in `CALL_MAP`) are only for names that don't follow the standard rule:
- `storeAtts` → `process_namespaces` (Rust uses a different architecture)
- `XmlNameLength` → `name_length` (Xml prefix stripped)

C-only operations are suppressed (`SUPPRESSED_CALLS`):
- Memory management: `poolStoreString`, `MALLOC`, `FREE`, etc.
- Hash tables: `lookup`, `hashTableInit`, etc.
- Accounting: `accountingDiffTolerated`, etc.
- ALL_CAPS macros are auto-suppressed

### Limitations

The AST tool does NOT currently check:
- Functions that exist in C but have no Rust counterpart at all
- FFI completeness (which C API functions are stubs vs implemented)
- State field read/write patterns
- Control flow structure beyond case-level (if/else branches, loops)
