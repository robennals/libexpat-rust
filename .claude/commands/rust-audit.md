Audit all Rust source files in expat-rust/src/ for quality and idiomatic Rust.

## Process

1. Run `cargo clippy` on the project and capture all warnings
2. For each source file with warnings, launch a parallel haiku subagent to fix them
3. After fixes, re-run `cargo clippy` to verify zero warnings
4. Run `cargo test --lib` to verify no regressions
5. Run `cargo test --test misc_tests` to verify integration tests still pass

## Quality Checklist (for each .rs file)

- **Clippy clean**: Zero warnings from `cargo clippy`
- **Type aliases**: Complex types like `Box<dyn FnMut(...) + 'static>` should have type aliases
- **Naming**: snake_case for functions/variables, CamelCase for types, SCREAMING_SNAKE for constants
- **Idioms**: Use `.rotate_left()` not manual bit ops, `.contains()` not manual range checks, `?` operator for error propagation, iterator chains over manual loops where cleaner
- **No unwrap() in library code**: Use `?` or return Option/Result
- **Standard library**: Use `u64::from_le_bytes()` not manual assembly, `str::eq_ignore_ascii_case()` not manual comparison
- **Safety**: No potential panics from indexing — use `.get()` or bounds checks
- **Dead code**: No unused imports, functions, or fields

## Haiku Agent Prompt Template

For each file needing fixes:
```
Fix clippy warnings and improve Rust idioms in [FILE_PATH].

Run: cargo clippy 2>&1 | grep [module_name]

Fix all warnings. Also check for:
1. Manual implementations of std library functions
2. Unnecessary allocations or clones
3. if/else chains that should be match
4. for loops that should be iterator chains

CRITICAL: All existing tests must still pass after changes.

Verify: cargo clippy 2>&1 | grep [module_name] && cargo test --lib
```

## Report

After all fixes, output a summary:
- Warnings before/after per file
- Tests before/after
- Key changes made
