# AST Structural Validator

Verifies that the Rust port structurally corresponds to the C original,
function-by-function. Uses tree-sitter to parse both languages into ASTs,
converts them to a common "skeleton" IR, and compares with explicit rewrite
rules for known language differences.

## How it works

The verifier has two tools:

- **`strict-ast-compare.py`** (recommended) — Skeleton-based structural comparison.
  Converts both ASTs to a language-agnostic intermediate representation, applies
  rewrite rules for known C/Rust differences, and does ordered structural matching.
  Every difference must be covered by a rewrite rule or it's reported as a mismatch.

- **`ast-compare.py`** (legacy) — Name-set comparison. Checks that the same error
  codes, handler calls, function calls, and match arm labels appear in both languages.
  Less precise but useful for quick audits.

### What the strict verifier checks

For each tracked function pair, the skeleton comparison verifies:

1. **Match arm correspondence** — every `case XML_TOK_*:` in C has a `XmlTok::*`
   arm in Rust, and the arm bodies structurally match
2. **Call ordering** — function calls, handler dispatches, and error returns appear
   in the same structural positions (ordered subsequence within each scope)
3. **Branch structure** — if/else branches correspond by the condition being checked
   (handler null checks, variable comparisons, etc.)
4. **Loop structure** — C `for(;;)` loops match Rust `loop {}` at the same nesting
5. **Return values** — error returns match (`XmlError::InvalidToken` in both)

### What it does NOT check

The verifier ensures structural correspondence, not semantic equivalence:

- **Argument values** are not compared (a call to `handler(data)` matches any
  call to `handler(...)`)
- **Expression details** within conditions are normalized but not fully parsed
  (e.g., `tag_level == 0` matches any branch checking `tag_level`)
- **Side effects** are not tracked (the order of calls is checked, but not what
  data flows between them)

This is intentional. The rewrite rules allow enough flexibility for C-to-Rust
language differences while constraining the code tightly enough that, combined
with behavioral testing, semantic divergences become unlikely.

## Rewrite rules

Language differences are handled by explicit rewrite rules in three files:

| File | Purpose | Review status |
|------|---------|---------------|
| `structural-rewrites.json` | Verified rules for known language differences | Confirmed correct |
| `temporary-rewrites.json` | Rules believed equivalent but not fully verified | Needs review |
| `deliberate-divergences.json` | Per-function suppressions and legacy name-set rules | Legacy + accepted |

Each rule in the JSON files has:
- **`input`** — what the C skeleton looks like (pattern to match)
- **`output`** — what the Rust equivalent looks like (`null` = deleted)
- **`justification`** — why this difference is acceptable
- **`category`** — what kind of difference (language_syntax, memory_management, etc.)

Example rules:

```json
{
  "name": "break_removal",
  "input": { "kind": "break" },
  "output": null,
  "justification": "C switch cases need explicit break; Rust match arms end implicitly."
}

{
  "name": "pool_store_string_removal",
  "input": { "kind": "call", "label_regex": "^pool_store_string$" },
  "output": null,
  "justification": "C allocates from a pool via poolStoreString(). Rust indexes the buffer slice directly."
}
```

Temporary rules are tracked separately so they can be reviewed and either promoted
to `structural-rewrites.json` (confirmed correct) or removed (found to be a real bug).

## Quick start

```bash
# Install dependencies
pip install tree-sitter tree-sitter-c tree-sitter-rust

# Initialize the C source (needed for comparison)
git submodule update --init

# Run the strict structural comparison (recommended)
python3 validator/strict-ast-compare.py --ci

# Run the legacy name-set comparison
python3 validator/ast-compare.py --ci

# See what's suppressed and why (legacy tool)
python3 validator/ast-compare.py --audit
```

## Commands

### strict-ast-compare.py

| Command | Purpose |
|---------|---------|
| `--ci` | Compare all function pairs, exit 1 on any mismatch. |
| `--all` | Compare all function pairs, print report. |
| `--json` | Output results as JSON. |
| `--dump <c_func> <rust_func>` | Show both skeletons and comparison. |
| `--dump-c <func>` | Show C skeleton (raw + after rewrites). |
| `--dump-rust <func>` | Show Rust skeleton. |
| `<c_func> <rust_func>` | Compare a specific pair. |

### ast-compare.py (legacy)

| Command | Purpose |
|---------|---------|
| `--ci` | Compare all pairs, exit 1 on divergence. |
| `--all` | Compare all pairs, print report. |
| `--audit` | Show all suppressions with justifications. |
| `--prompt <c_func> <rust_func>` | Generate an AI porting prompt for divergences. |
| `--prompt-all` | Generate prompts for all divergent pairs. |
| `--missing-functions` | List C functions with no Rust equivalent. |

## Value for AI-assisted porting

The AST comparison tool significantly simplifies AI-assisted porting. When an agent
needs to port or fix a C function in Rust, the tool provides:

- **Exactly which operations are missing** — specific call names, error codes,
  match arms, and handler dispatches, with source line numbers in both files
- **Structural context** — where each missing operation belongs (which match arm,
  which branch, what nesting level)
- **Prompt generation** — `ast-compare.py --prompt` produces targeted porting
  instructions with the relevant C code, Rust code, file paths, line numbers,
  and a description of what needs to change

This turns "port this 500-line function" into a series of precise tasks like
"add XmlError::AsyncEntity return after the tag_level check in the TrailingCr arm
of do_content (C line 3363, Rust line 3166)."

## Safety mechanism: NOT_SUPPRESSED

The divergences file has a `NOT_SUPPRESSED` section listing security-critical
calls and errors that must never be suppressed:

- `accountingDiffTolerated`, `accountingOnAbort`, `accountingReportDiff`,
  `accountingGetCurrentAmplification` — amplification attack detection
- `entityTrackingOnOpen`, `entityTrackingOnClose`, `entityTrackingReportStats`
  — entity expansion tracking
- `AmplificationLimitBreach` error code

If anyone adds one of these to a suppression category, both tools exit with
a `FATAL` error.

## Files

| File | Purpose |
|------|---------|
| `strict-ast-compare.py` | Strict skeleton-based structural comparison |
| `ast-compare.py` | Legacy name-set comparison |
| `strict_compare/` | Python package: skeleton extraction, rewrite engine, matcher |
| `structural-rewrites.json` | Verified rewrite rules (input/output patterns) |
| `temporary-rewrites.json` | Temporary rewrite rules (believed but unverified) |
| `deliberate-divergences.json` | Per-function suppressions and legacy config |

## CI integration

Both tools run in CI (`.github/workflows/ci.yml`):

```yaml
ast-validator:
  steps:
    - run: pip install tree-sitter tree-sitter-c tree-sitter-rust
    - run: python3 validator/strict-ast-compare.py --ci
    - run: python3 validator/ast-compare.py --ci
    - run: python3 validator/ast-compare.py --audit
      if: always()
```
