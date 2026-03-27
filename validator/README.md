# AST Structural Validator

This tool is the primary mechanism for verifying that the Rust port of libexpat
faithfully replicates the C implementation's behavior. It uses tree-sitter to
parse both C and Rust source code into ASTs, then structurally compares them
function-by-function.

## Why this matters

A line-by-line C-to-Rust port is only trustworthy if you can demonstrate that
every switch case, error code, handler call, and function call in the C code has
a corresponding construct in the Rust code. This tool automates that comparison.

Every difference between C and Rust is either:
- **Flagged as a divergence** that needs to be fixed, or
- **Explicitly suppressed** with a written justification in `deliberate-divergences.json`

There are no silent filters. If a C function call doesn't appear in Rust, the
tool will flag it unless the divergences file explains why.

## Quick start

```bash
# Install dependencies
pip install tree-sitter tree-sitter-c tree-sitter-rust

# Initialize the C source (needed for comparison)
git submodule update --init

# Run the full comparison
python3 validator/ast-compare.py --ci

# See what's suppressed and why
python3 validator/ast-compare.py --audit
```

## Commands

| Command | Purpose |
|---------|---------|
| `--ci` | Compare all function pairs, exit 1 on any divergence. Used in CI. |
| `--all` | Compare all function pairs, print report. |
| `--audit` | Show every suppression with its justification. |
| `<c_func> <rust_func>` | Compare a specific function pair. |
| `--list-cases <func> c\|rust` | List switch/match cases in a function. |
| `--prompt <c_func> <rust_func>` | Generate an AI porting prompt for divergences. |
| `--prompt-all` | Generate prompts for all divergent pairs. |
| `--missing-functions` | List C functions with no Rust equivalent. |
| `--json` | Output comparison results as JSON (add to any pair comparison). |

## What it compares

For each tracked function pair (C function vs Rust function):

1. **Error codes** — every `XML_ERROR_*` in C should have a corresponding `XmlError::*` in Rust
2. **Handler calls** — every `parser->m_*Handler` in C should have a corresponding `self.*_handler` in Rust
3. **Match arm coverage** — every `case XML_TOK_*:` in C should have a corresponding `XmlTok::*` arm in Rust
4. **Function calls** — every function call in C should have a Rust equivalent (auto-converts camelCase to snake_case)
5. **Per-case detail** — checks errors, handlers, and calls within each individual case/arm

## Suppression system

All suppressions live in `deliberate-divergences.json`. Every suppression must have:

- **`justification`** — why this difference is acceptable
- **`status`** — `"accepted"` (suppressed) or `"must_port"` (not suppressed, tracked as work to do)
- **`category`** — what kind of difference this is

### Safety mechanism: NOT_SUPPRESSED

The divergences file has a `NOT_SUPPRESSED` section listing calls and errors that
must NEVER be suppressed. If anyone adds one of these to a suppression category,
the tool exits with a `FATAL` error. This prevents accidental re-suppression of
security-critical features like the accounting system.

Currently protected:
- `accountingDiffTolerated`, `accountingOnAbort`, `accountingReportDiff`, `accountingGetCurrentAmplification` — byte-counting amplification detection
- `entityTrackingOnOpen`, `entityTrackingOnClose`, `entityTrackingReportStats` — entity expansion tracking
- `AmplificationLimitBreach` error code — returned when amplification attack is detected

### Suppression categories

| Category | Count | Why suppressed |
|----------|-------|----------------|
| `memory_management` | 25 | C pools/malloc → Rust Vec/String |
| `hash_tables` | 5 | C hash tables → Rust HashMap |
| `entropy` | 2 | C manual entropy → Rust RandomState |
| `c_string_ops` | 10 | memcpy/strcmp → Rust slice ops |
| `c_type_system` | 7 | C casting macros → Rust types |
| `c_internal_macros` | 8 | C-specific helpers |
| `content_model_building` | 2 | C scaffold array → Rust ContentNode stack |
| `c_preprocessor_macros` | ~15 | Regex noise from C macros |
| `allocator_tracking` | 5 | C MALLOC_TRACKER (allocator-specific) |

### Adding a new suppression

1. Edit `deliberate-divergences.json`
2. Add the call to an existing category, or create a new one
3. Include a `justification` explaining why this is acceptable
4. Set `status: "accepted"`
5. Run `python3 validator/ast-compare.py --audit` to verify
6. If the call is in `NOT_SUPPRESSED`, the tool will refuse — you must remove it from `NOT_SUPPRESSED` first (and have a very good reason)

## Files

| File | Purpose |
|------|---------|
| `ast-compare.py` | The comparison tool (single-file, ~1100 lines) |
| `deliberate-divergences.json` | All suppressions, function pairs, and must-port items |
| `README.md` | This file |

## CI integration

The AST validator runs as part of CI in `.github/workflows/ci.yml`:

```yaml
ast-validator:
  name: AST Structural Validation
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
      with:
        submodules: true
    - uses: actions/setup-python@v5
      with:
        python-version: '3.12'
    - run: pip install tree-sitter tree-sitter-c tree-sitter-rust
    - run: python3 validator/ast-compare.py --ci
    - run: python3 validator/ast-compare.py --audit
      if: always()
```

The `--ci` step fails the build if any divergences are found. The `--audit` step
always runs to provide a complete suppression report in the build log.
