# How the Rewrite System Works

## Overview

The AST verifier compares C and Rust function skeletons by transforming the C
skeleton to look like the Rust skeleton, then checking if they match. The
transformation is one-directional: **only the C skeleton is rewritten**.

This is NOT a bidirectional search. The system does NOT:
- Delete or modify Rust nodes
- Explore multiple rule combinations to find a matching path
- Backtrack if a rule application doesn't lead to a match

It IS a simple pipeline:
1. Extract C skeleton from tree-sitter AST
2. Inline called C functions (expand call sites)
3. Apply rewrite rules top-down to C skeleton (delete/transform nodes)
4. Compare the rewritten C skeleton against the Rust skeleton
5. Report mismatches

## Rule Types

### 1. Single-node rewrite rules (`structural-rewrites.json` → `rewrite_rules`)

Match a single C skeleton node and delete or transform it.

```json
{
  "name": "break_removal",
  "input":  { "kind": "break" },
  "output": null,
  "justification": "C break is implicit in Rust match arms."
}
```

**Input pattern fields** (all optional — unspecified fields match anything):
- `kind`: exact match on node kind (call, branch, return, match, arm, etc.)
- `label`: exact match on label (`"*"` = wildcard, `"$var"` = capture)
- `label_regex`: regex match on label
- `children_count`: exact number of children

**Output**: `null` = delete the node. Or a dict with `kind`/`label` to transform.

**Applied**: bottom-up (leaves first), every rule checked against every node.

### 2. Tree rewrite rules (`structural-rewrites.json` → `tree_rewrite_rules`)

Match a contiguous subsequence of sibling nodes using a text DSL and replace
with a new tree structure. Captures allow preserving matched subtrees.

```json
{
  "name": "tokenizer_result_wrap",
  "input_pattern": "call($tok where /.*_tok$/); match($m)",
  "output_pattern": "$m",
  "justification": "C calls tokenizer then switches. Rust uses let-match."
}
```

**DSL syntax**:
- `kind(label)` — match node with this kind and exact label
- `kind($var)` — match and capture as $var
- `kind($var where /regex/)` — match with regex, capture as $var
- `kind(label) { child1; child2 }` — match with children
- `$var` — in output, substitute the captured node
- `pat1; pat2` — match contiguous siblings (top level)

**Applied**: bottom-up, after single-node rules. The pattern's `children` list
is matched against a contiguous subsequence of the parent node's children.
Matched siblings are replaced with the output tree.

### 3. Expression rewrite rules (`structural-rewrites.json` → `expression_rewrites`)

Transform argument strings in function calls before comparison.

```json
{
  "name": "c_s_pointer_to_pos",
  "side": "c",
  "function": "*",
  "match": "^s$",
  "replace": "pos",
  "justification": "C 's' pointer = Rust 'pos' offset"
}
```

**Fields**:
- `side`: `"c"`, `"r"`, or `"both"` — which argument list to modify
- `function`: function name filter (`"*"` = all)
- `match`: regex to match against each argument string
- `replace`: replacement string, `null` (delete), or `["alt1", "alt2"]` (alternatives)

When `replace` is a list, the argument becomes multiple candidates. Matching
succeeds if ANY candidate matches the other side.

**Applied**: to both C and Rust argument lists before subsequence comparison.

### 4. Per-function suppressions (`structural-rewrites.json` → `per_function_suppressions`)

Suppress specific mismatches for specific functions. **Should only contain
principled architectural choices**, not temporary workarounds.

```json
{
  "do_content": {
    "suppressed_calls": ["accounting_diff_tolerated"],
    "suppressed_errors": ["AmplificationLimitBreach"],
    "justification": "Accounting at parse() entry, not per-token."
  }
}
```

### 5. Temporary rules (`temporary-rewrites.json`)

Same format as single-node rules but explicitly temporary. Each has:
- `risk`: what could go wrong if this rule is wrong
- `status`: `"believed_equivalent"` or `"needs_verification"`

## What the System Cannot Do

1. **No backtracking**: If a rule fires and produces a result that doesn't
   match Rust, the system doesn't try alternative rules. Rules are applied
   greedily in order.

2. **No Rust-side rewriting**: Only the C skeleton is transformed. If Rust
   has extra nodes (bounds checks, stall detection), the comparison must
   find C nodes as a subsequence of Rust nodes — Rust extras are ignored.

3. **No cross-function matching**: If C has `call(foo)` and Rust inlined
   foo's body, we handle this via explicit call inlining (listing foo in
   `inline_functions`). The system doesn't automatically detect inlining.

4. **No semantic matching**: Rules match on syntax (kind, label, children),
   not on semantics. The system can't tell that `*s == 0xD` and
   `input[i] == b'\r'` mean the same thing unless a rule explicitly maps one
   to the other.

## Files

| File | Contains |
|------|----------|
| `structural-rewrites.json` | Verified single-node rules, tree rules, expression rules, principled suppressions |
| `temporary-rewrites.json` | Temporary single-node rules (needs verification) |
| `deliberate-divergences.json` | Function pair list, inline functions, legacy suppressions |
| `algorithm-divergences.json` | Functions using different algorithms (needs fix) |

## Current Gaps

The system works well for functions that were **transliterated** (same algorithm,
different syntax). It struggles with functions that were **restructured** or
**reimplemented**:

- **Restructured**: Same logic but different nesting/ordering. Tree rewrite rules
  can sometimes handle this. Per-function suppressions are the fallback.
- **Reimplemented**: Different algorithm entirely. These are in
  `algorithm-divergences.json` and skipped.

The 10 remaining reported errors are mostly restructured patterns where the
tree rewrite rules can't quite express the transformation.
