# Plan: Source-Level Rewrite-to-Canonical-Form Comparison (v4)

## Core Idea

Rewrite both C and Rust **source text** using pattern rules to a canonical
form. Then parse both canonical sources into ASTs and compare. The rewrite
rules operate on raw token strings, not ASTs.

## Pipeline

```
C source text
  → identifier normalization (camelCase→snake_case, XML_TOK→XmlTok, etc.)
  → C string rewrite rules (applied to source text)
  → canonical C text
  → parse to AST (tree-sitter)
  → compare ──────────────────────┐
                                   ├→ mismatches
Rust source text                   │
  → identifier normalization       │
  → Rust string rewrite rules      │
  → canonical Rust text            │
  → parse to AST (tree-sitter) ───┘
```

## Rewrite Rule Format

Rules match and replace substrings in the source text. `$variable` captures
any balanced substring (no unbalanced delimiters).

```yaml
rust_rewrite_rules:
  - name: handler_borrow_wrapper
    description: "Guarded method call — borrow checker requires if-let"
    before: "if let Some($handler) = &mut self.$field { $handler($args); }"
    after: "self.$field($args);"

  - name: result_unwrap
    description: "Result match unwrap"
    before: "match $expr { Ok($val) => { $body }, Err($err) => { return $ret; } }"
    after: "let $val = $expr; $body"

  - name: drop_iteration_guard
    description: "Drop stall detection"
    before: "if $iter > $max { return $err; }"
    drop: true

c_rewrite_rules:
  - name: drop_free
    description: "Drop free() calls"
    before: "free($args);"
    drop: true

  - name: handler_dispatch
    description: "Inline handler null check"
    before: "if ($handler) { $handler($handlerArg, $args); }"
    after: "$handler($args);"

  - name: drop_break
    description: "Drop break in switch"
    before: "break;"
    drop: true
```

## Pattern Matching Algorithm

### Tokenization

Split the source text (or the relevant function body) into tokens:
- Identifiers: `[a-zA-Z_][a-zA-Z0-9_]*`
- Operators: `==`, `!=`, `<=`, `>=`, `&&`, `||`, `->`, `::`, `..`, etc.
- Single chars: `(`, `)`, `{`, `}`, `[`, `]`, `;`, `,`, `.`, `!`, `*`, `&`, `=`, etc.
- Literals: numbers, strings, char literals
- `$variable`: pattern variables (only in patterns, not in source)

Whitespace and comments are stripped.

### Matching

Given pattern tokens and source tokens starting at position `pos`:

```
match(pattern, source, pos):
    for each pattern token:
        if token is literal:
            if source[pos] != token: return None
            pos += 1
        if token is $variable:
            next_literal = next literal token in pattern (or end)
            capture = consume source tokens until next_literal found
                      (respecting balanced delimiters)
            captures[$variable] = capture
    return captures
```

**Balanced delimiter rule**: When a `$variable` is followed by `)`, `}`, or `]`,
capture everything up to the MATCHING closing delimiter (not just the first one).
When followed by `;`, capture up to the next `;` at the same nesting level.
When followed by another identifier/keyword, capture a single token or a single
balanced group.

### Substitution

Given captures and an `after` template:
```
substitute(template, captures):
    for each token in template:
        if token is $variable:
            emit captures[$variable]
        else:
            emit token
    return joined tokens
```

### Application

For a function body:
1. Extract the function body text
2. Normalize identifiers
3. For each rule, scan the text for matches (left to right, non-overlapping)
4. Apply all matches (replace or drop)
5. Repeat until no rules fire (fixpoint, max 10 iterations)

## Identifier Normalization (hardcoded, applied first)

```
# C normalization:
parser->m_fooBar     → self.foo_bar
enc->minBytesPerChar → enc.min_bytes_per_char
handlerArg           → [removed — Rust closures capture]
camelCase            → snake_case
XML_TOK_TRAILING_CR  → XmlTok::TrailingCr
XML_ERROR_NO_MEMORY  → XmlError::NoMemory
XML_ROLE_XML_DECL    → Role::XmlDecl
NULL                 → None
XML_T('\0')          → b'\0'

# Rust normalization:
self.foo_bar              → self.foo_bar (already canonical)
xmltok_impl::content_tok  → content_tok
xmltok::Utf8Encoding      → [encoding constant — normalize]
Self::normalize_lines     → normalize_lines
```

## Expected Rules

### Rust (~15 rules)

| Pattern | Replacement | Why |
|---------|-------------|-----|
| `if let Some($h) = &mut self.$f { $h($a); }` | `self.$f($a);` | Borrow checker wrapper |
| `if let Some($h) = &mut self.$f { $h($a); } else { $e }` | `self.$f($a); $e` | Wrapper with fallback |
| `match $e { Ok($v) => { $b }, Err($err) => { return $r; } }` | `let $v = $e; $b` | Result unwrap |
| `match $e { Ok($v) => $v, Err($err) => { return $r; } }` | `let $v = $e;` | Result unwrap (value) |
| `if $i > $max { return $e; }` | [drop] | Iteration guard |
| `let mut $i = 0;` | [drop] | Iteration counter (paired with guard) |
| `std::mem::take(&mut self.$f)` | `self.$f` | Ownership transfer |
| `$v.to_vec()` | `$v` | Clone (Rust-specific) |
| `String::from_utf8($v).unwrap_or_default()` | `$v` | UTF-8 conversion |
| `Vec::with_capacity($n)` | [drop in let context] | Allocation hint |
| `.len()` | [keep — might need for loop bounds] | |

### C (~20 rules)

| Pattern | Replacement | Why |
|---------|-------------|-----|
| `poolClear($a);` | [drop] | Pool lifecycle (RAII) |
| `poolDiscard($a);` | [drop] | Pool lifecycle |
| `poolStoreString($p, $enc, $start, $end)` | `&data[$start..$end]` | Pool → slice |
| `free($a);` | [drop] | Explicit dealloc |
| `REALLOC($a);` | [drop] | Explicit realloc |
| `if ($h) { $h($ha, $a); }` | `$h($a);` | Handler null check |
| `if ($h) { $h($ha, $a); } else if ($d) { reportDefault($da); }` | `$h($a); reportDefault($da);` | Handler + default |
| `break;` | [drop] | Switch break |
| `goto $l;` | [drop] | Goto |
| `*$pp = $v;` | [drop] | Position tracking |
| `if (MUST_CONVERT($a)) { $t } else { $e }` | `$e` | Encoding (else only) |
| `if (enc == self.m_encoding) { $t } else { $e }` | `$t` | Encoding selection |
| `if (!accountingDiffTolerated($a)) { $b }` | [drop] | Per-token accounting |
| `switch (self.parsing_status.parsing) { $a }` | [drop] | Suspend/resume |
| `if ((0) && $c) { $b }` | [drop] | Disabled code |
| `return XmlError::None;` | `return;` | Success sentinel |
| `if (!$p) { return XmlError::NoMemory; }` | [drop] | OOM check |
| `if (!$p) return 0;` | [drop] | OOM check (variant) |

## After Rewriting: What Remains

Both sides should have:
- The same function calls (same names, similar args)
- The same control flow (if/loop/match with same conditions/arms)
- The same error returns
- The same variable assignments

What's been removed:
- C: pool ops, handler null checks, break, goto, position tracking, encoding
- Rust: if-let wrappers, Result matching, iteration guards, ownership ops

## Corner Cases

1. **Nested matches**: `match result { Ok(val) => match val.token { ... } }` —
   the outer match is rewritten to a let, leaving `let val = result; match val.token { ... }`.
   The inner match is NOT rewritten (it's a real dispatch, not Result unwrapping).

2. **Multiple handler args**: C's `handler(handlerArg, data, len)` — the
   `handlerArg` removal happens in identifier normalization (handlerArg → removed),
   so the rewrite just sees `handler(data, len)`.

3. **Fixpoint convergence**: A rewrite might produce text that matches another rule.
   E.g., Result unwrap produces a let binding, which might match a let-inlining rule.
   Limit to 10 iterations; warn if not converged.

4. **Multi-statement patterns**: `$h($a); reportDefault($da);` matches two
   consecutive statements. The pattern scanner must handle sequences, not just
   single statements.

5. **Greedy vs non-greedy $vars**: `$handler($handlerArg, $args)` — `$handlerArg`
   matches one argument, `$args` matches the rest. The delimiter is `,` between
   them. Need to handle this: `$var` before `,` captures one comma-separated item.

## Implementation

~400 lines total:
- `tokenizer.py` (~50 lines): tokenize source text
- `pattern.py` (~150 lines): match patterns against token streams
- `rewriter.py` (~100 lines): load YAML rules, apply to source text
- `canonical-compare.py` (~100 lines): main driver
