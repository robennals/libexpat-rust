# Plan: Source-Level Rewrite-to-Canonical-Form Comparison (v4)

## Status: Pattern matcher DONE and tested. Building rewriter next.

## Pipeline

```
C source text
  → extract function body
  → apply C rewrite rules (YAML patterns, applied to source text)
  → canonical text
  → parse to AST
  → compare (ignoring case, _, :: in identifiers) ──┐
                                                      ├→ mismatches
Rust source text                                      │
  → extract function body                             │
  → apply Rust rewrite rules                          │
  → canonical text                                    │
  → parse to AST ────────────────────────────────────┘
```

## Identifier Matching

AST comparison ignores case, `_`, and `::` when comparing identifiers.
So `m_commentHandler`, `comment_handler`, and `CommentHandler` all match.

Additionally, rewrite rules normalize structural patterns:
- `parser->$foo` → `self.$foo` (C struct access → Rust self)
- `self.$foo` → `self.$foo` (Rust identity)

## Rewrite Rule Format

```yaml
rust_rewrite_rules:
  - name: handler_borrow_wrapper
    before: "if let Some($handler) = &mut self.$field { $handler($args); }"
    after: "self.$field($args);"

  - name: drop_iteration_guard
    before: "if $iter > $max { return $err; }"
    drop: true

c_rewrite_rules:
  - name: struct_access
    before: "parser->$field"
    after: "self.$field"

  - name: drop_free
    before: "free($args);"
    drop: true
```

## Completed

- [x] Tokenizer (source text → token stream)
- [x] Pattern matcher (non-greedy, balanced delimiters, same-var constraint)
- [x] Template substitution ($var replacement)
- [x] Comprehensive tests (all passing)

## Next Steps

1. **Rewriter**: Apply rules to function body text. Find all pattern matches,
   apply replacements/drops, repeat to fixpoint.
2. **YAML rules**: ~20 C rules, ~15 Rust rules
3. **Driver**: Extract function bodies, apply rewrites, compare canonical ASTs
4. **Identifier-insensitive comparison**: Ignore case/underscores/:: in AST comparison
