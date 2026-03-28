# Plan: AST Rewrite-to-Canonical-Form Comparison (v4)

## Core Idea

Before comparing C and Rust, rewrite both into a **canonical form** using
language-specific rewrite rules. The canonical form doesn't need to be valid
in either language — it just needs to be the SAME for equivalent code.

Then comparison is trivial: if the canonical forms are identical (modulo
identifier normalization), the code is equivalent.

## Architecture

```
C source → tree-sitter → common AST → C rewrite rules → canonical AST ─┐
                                                                         ├→ compare
Rust source → tree-sitter → common AST → Rust rewrite rules → canonical AST ─┘
```

### Step 1: Parse to common AST (existing v2/v3 code)
Use tree-sitter to parse both languages into common AST nodes.
This is already implemented.

### Step 2: Normalize identifiers (hardcoded)
Apply deterministic identifier normalization to both sides:
- C `parser->m_fooBar` → `self.foo_bar` (strip parser->, strip m_, camelCase→snake_case)
- C `XML_TOK_TRAILING_CR` → `XmlTok::TrailingCr`
- C `XML_ERROR_NO_MEMORY` → `XmlError::NoMemory`
- C `XML_ROLE_INSTANCE_START` → `Role::InstanceStart`
- Rust `self.foo_bar` → `self.foo_bar` (already canonical)
- Rust `xmltok_impl::content_tok` → `content_tok` (strip module prefix)

This is a fixed set of transforms, not configurable.

### Step 3: Apply language-specific rewrite rules
Rules are YAML patterns with `$variable` holes. A `$variable` matches any
subtree that doesn't contain unbalanced delimiters. Rules are applied
repeatedly (bottom-up, to fixpoint) until no more rules match.

### Step 4: Compare canonical ASTs
After rewriting, both sides should have the same AST structure. Compare
node-by-node. Any difference is a real mismatch.

## Rewrite Rule Format

```yaml
rust_rewrite_rules:
  - name: handler_borrow_wrapper
    description: "Guarded method call — borrow checker requires if-let"
    before: "if let Some($handler) = &mut self.$field { $handler($args); }"
    after: "self.$field($args);"
    principled: true
    why: "Rust borrow checker requires if-let to call Option<fn> handlers"

  - name: result_unwrap
    description: "Result match unwrap"
    before: "match $expr { Ok($val) => $body, Err($err) => { return $ret; } }"
    after: "let $val = $expr; $body"
    principled: true
    why: "Rust uses Result where C uses return codes"

  - name: drop_iteration_guard
    description: "Drop stall detection guard"
    before: "if $iter > $max { return $err; }"
    drop: true
    principled: true
    why: "Rust adds iteration limits for safety; C relies on finite input"

c_rewrite_rules:
  - name: drop_pool_clear
    description: "Drop pool lifecycle"
    before: "poolClear($args);"
    drop: true
    principled: true
    why: "C pool lifecycle. Rust uses RAII."

  - name: drop_free
    description: "Drop free() calls"
    before: "free($args);"
    drop: true
    principled: true
    why: "C explicit deallocation. Rust uses RAII."

  - name: handler_dispatch_inline
    description: "Inline handler null check + call"
    before: "if ($handler) { $handler($handlerArg, $args); }"
    after: "$handler($args);"
    principled: true
    why: "C checks handler then calls. Canonical form: just call."

  - name: handler_dispatch_with_default
    description: "Handler dispatch with default fallback"
    before: "if ($handler) { $handler($handlerArg, $args); } else if ($default) { reportDefault($dargs); }"
    after: "$handler($args); reportDefault($dargs);"
    principled: true
    why: "Canonical form: both calls, without conditional wrapping"

  - name: drop_break_in_switch
    description: "Drop break statements in switch cases"
    before: "break;"
    drop: true
    principled: true
    why: "C switch requires explicit break. Canonical form omits it."

  - name: must_convert_to_else
    description: "MUST_CONVERT encoding branch — keep else only"
    before: "if (MUST_CONVERT($args)) { $then } else { $else }"
    after: "$else"
    principled: true
    why: "Rust is UTF-8 only. Only the else branch (no conversion) applies."

  - name: drop_position_tracking
    description: "Drop event pointer writes"
    before: "*$eventPP = $pos;"
    drop: true
    principled: true
    why: "C writes position via output pointers. Rust returns in tuples."
```

## Pattern Matching Algorithm

### Pattern syntax

A pattern is a string that looks like code but with `$variable` holes:

```
"if let Some($handler) = &mut self.$field { $handler($args); }"
```

Tokens:
- **Literal tokens**: `if`, `let`, `Some`, `(`, `)`, `{`, `}`, `.`, `&`, `mut`, `;`, `=`, etc.
- **Variables**: `$name` — matches one or more tokens up to the next literal that follows in the pattern
- **Delimited variables**: A `$var` followed by a closing delimiter `)`, `}`, `]`, `;` matches everything up to that delimiter (respecting nesting)

### Matching algorithm

Given a pattern and an AST node (serialized to tokens):

1. Serialize the AST node to a token stream (the same serialization used for patterns)
2. Walk both streams in parallel:
   - Literal token: must match exactly
   - `$variable`: consume tokens from the AST until the next literal in the pattern is found
     (respecting balanced delimiters)
3. If all pattern tokens are consumed and all AST tokens are consumed, it's a match
4. Captured variables are available for the `after` template

### Applying rewrites

For each AST node (bottom-up):
1. Serialize the node to tokens
2. Try each rule's `before` pattern
3. If a rule matches:
   - If `drop: true`, delete the node
   - Otherwise, substitute captures into the `after` template
   - Parse the result back into an AST node
   - Replace the original node with the result
4. Repeat until no rules fire (fixpoint)

### Implementation detail: token serialization

Serialize common AST nodes to a flat token stream:

```
Node("if", children=[
    Node("call", children=[Node("ident", value="is_none")]),
    Node("block", children=[...])
])
```
→ tokens: `if is_none ( ) { ... }`

The serialization preserves structure via delimiter tokens `(){}[]` but
flattens the tree. This makes pattern matching simpler — it's string
matching with balanced-delimiter-aware variable capture.

### Corner cases

**1. Nested rewrites**: A rewrite may produce an AST that matches another rule.
Bottom-up application + fixpoint handles this: inner rewrites fire first,
then outer ones, repeated until stable.

**2. Multiple matches**: If a rule can match at multiple positions in the same
parent (e.g., multiple pool_clear calls), apply to all of them.

**3. Statement vs expression context**: A `drop` rule in statement context removes
the statement. In expression context (e.g., inside an if condition), it's
an error — we shouldn't drop parts of expressions.

**4. $args matching across delimiters**: `$args` in `foo($args)` matches everything
between `(` and `)`, including nested `()`. The pattern parser knows that
`$args` before `)` should capture up to the matching `)`.

**5. Multi-statement patterns**: A pattern like `$handler($args); reportDefault($dargs);`
matches two consecutive statements. This requires matching against a SEQUENCE
of sibling AST nodes, not just a single node.

**6. Whitespace/formatting**: Token comparison ignores whitespace. The serialization
strips all whitespace; matching is on token sequences.

## Expected Rules

### Rust rewrite rules (~15)

```yaml
# Borrow checker patterns
- before: "if let Some($h) = &mut self.$f { $h($a); }"
  after: "self.$f($a);"

- before: "if let Some($h) = &mut self.$f { $h($a); } else { $else }"
  after: "self.$f($a); $else"

# Result unwrapping
- before: "match $e { Ok($v) => { $body }, Err($err) => { return $ret; } }"
  after: "let $v = $e; $body"

# Iteration safety
- before: "if $i > $max { return $err; }"
  drop: true

- before: "let mut $i = 0;"  # only when paired with iteration guard
  drop: true

# Memory management
- before: "std::mem::take(&mut self.$f)"
  after: "self.$f"

- before: "Vec::new()"
  drop: true  # in let context

- before: "Vec::with_capacity($n)"
  drop: true

# String conversion
- before: "String::from_utf8($v).unwrap_or_default()"
  after: "$v"

# Slice creation (matches C pool operations)
- before: "&$data[$start..$end]"
  after: "poolStoreString($data, $start, $end)"  # rewrite TO C-like form
```

### C rewrite rules (~20)

```yaml
# Memory management
- before: "poolClear($a);"
  drop: true
- before: "poolDiscard($a);"
  drop: true
- before: "poolStoreString($pool, $enc, $start, $end)"
  after: "poolStoreString($start, $end)"  # normalize args
- before: "free($a);"
  drop: true
- before: "REALLOC($a);"
  drop: true

# Handler dispatch
- before: "if ($h) { $h($harg, $a); }"
  after: "$h($a);"
- before: "if ($h) { $h($harg, $a); } else if ($d) { reportDefault($da); }"
  after: "$h($a); reportDefault($da);"

# Control flow
- before: "break;"
  drop: true
- before: "goto $label;"
  drop: true

# Position tracking
- before: "*$pp = $v;"
  drop: true

# Encoding
- before: "if (MUST_CONVERT($a)) { $then } else { $else }"
  after: "$else"
- before: "if (enc == parser->m_encoding) { $then } else { $else }"
  after: "$then"  # keep the "same encoding" path

# Accounting
- before: "if (!accountingDiffTolerated($a)) { $body }"
  drop: true

# Parsing status
- before: "switch (parser->m_parsingStatus.parsing) { $arms }"
  drop: true

# Disabled code
- before: "if ((0) && $cond) { $body }"
  drop: true

# Success returns
- before: "return XML_ERROR_NONE;"
  after: "return;"

# OOM checks
- before: "if (!$ptr) { return XML_ERROR_NO_MEMORY; }"
  drop: true
- before: "if (!$ptr) return 0;"
  drop: true
```

## Identifier Normalization (hardcoded)

Applied to both sides before rewrite rules:

```python
# C → canonical
"parser->m_fooBar"     → "self.foo_bar"
"enc->minBytesPerChar" → "enc.min_bytes_per_char"
"handlerArg"           → removed (Rust closures capture)
"XML_TOK_X"            → "XmlTok::X" (PascalCase)
"XML_ERROR_X"          → "XmlError::X" (PascalCase)
"XML_ROLE_X"           → "Role::X" (PascalCase)
"XML_T('\\0')"         → "b'\\0'"
"NULL"                 → "None"
camelCase identifiers  → snake_case

# Rust → canonical
"self.foo_bar"         → "self.foo_bar" (already canonical)
"xmltok_impl::foo"     → "foo" (strip module prefix)
"xmltok::foo"          → "foo"
"Self::foo"            → "foo"
"XmlTok::X"            → "XmlTok::X" (already canonical)
```

## What This Approach Solves

| Problem | How solved |
|---------|-----------|
| Handler dispatch pattern | C rewrite: inline handler check. Rust rewrite: unwrap if-let. Both become direct call. |
| Result unwrapping | Rust rewrite: match Ok/Err → let binding. C already has direct assignment. |
| Pool operations | C rewrite: drop pool lifecycle. Rust: no pools. |
| Position tracking | C rewrite: drop *eventPP writes. Rust: returns in tuples. |
| Encoding branches | C rewrite: keep else branch of MUST_CONVERT. Rust: no encoding branch. |
| OOM checks | C rewrite: drop null checks. Rust: panics on OOM. |
| Borrow checker wrappers | Rust rewrite: unwrap if-let. Both become direct call. |
| Iteration guards | Rust rewrite: drop. C: no guards. |
| Expression decomposition | C/Rust: single-use let inlining (existing). |
| Different identifier names | Hardcoded normalization. Both become snake_case with canonical prefixes. |

## Implementation Plan

### File structure
```
validator/strict_compare/tokenizer.py    — AST ↔ token stream serialization
validator/strict_compare/pattern.py      — Pattern matching (tokens with $vars)
validator/strict_compare/rewriter.py     — Apply rules to AST (bottom-up, fixpoint)
validator/canonical-compare.py           — Main driver
validator/c-rewrites.yaml                — C rewrite rules
validator/rust-rewrites.yaml             — Rust rewrite rules
```

### Step-by-step implementation

1. **Tokenizer** (~100 lines): Serialize common AST to token stream and back.
   Each token is a string. Delimiters `(){}[]` are single tokens. Identifiers,
   operators, literals are single tokens. Variables `$name` are special tokens.

2. **Pattern matcher** (~150 lines): Given a pattern (token stream with $vars)
   and a target (token stream), try to match. Returns captures dict or None.
   Key: balanced-delimiter-aware variable capture.

3. **Rule applier** (~100 lines): Load YAML rules, apply bottom-up to AST.
   For each node, serialize → try patterns → if match, substitute → parse back.
   Repeat to fixpoint.

4. **Driver** (~50 lines): Parse both files, normalize identifiers, apply
   language-specific rewrites, compare canonical ASTs.

5. **Rules** (~30 C rules, ~15 Rust rules): Written in YAML as described above.

### Testing approach

For each rule, create a minimal test case:
```python
def test_handler_dispatch():
    c_ast = parse("if (handler) { handler(arg, data); }")
    r_ast = parse("if let Some(h) = &mut self.handler { h(data); }")
    c_canonical = apply_rules(c_ast, c_rules)
    r_canonical = apply_rules(r_ast, r_rules)
    assert c_canonical == r_canonical  # both become: handler(data);
```

### Estimated complexity

- Tokenizer: straightforward (serialize/deserialize common AST)
- Pattern matcher: moderate (balanced delimiter tracking)
- Rule applier: straightforward (iterate rules, apply matches)
- Rules: the bulk of the work — each needs careful testing

Total: ~400 lines of new code + ~50 YAML rules.

## Risk Assessment

**Low risk**: The token-based pattern matching is well-understood (similar to
macro expansion). The rules are individually testable.

**Medium risk**: Fixpoint convergence — rules might interact in unexpected ways.
Mitigation: limit iterations, detect infinite loops.

**High risk**: Getting the rules right. Each rule is an assertion that "these
two patterns are semantically equivalent." A wrong rule hides a real bug.
Mitigation: mark each rule principled/temporary, require justification.

## How This Differs from v3

v3 compares scoping trees + content sets. It's good at finding which functions
call which other functions, but struggles with HOW the calls are made (handler
dispatch patterns, Result wrapping, etc.).

v4 rewrites both sides to a canonical form BEFORE comparing. The comparison
itself is trivial — just check AST equality. All the intelligence is in the
rewrite rules, which are individually testable and auditable.

The key insight: instead of trying to match two different ASTs, transform both
to the SAME AST. Each transformation is a principled assertion about what's
equivalent.
