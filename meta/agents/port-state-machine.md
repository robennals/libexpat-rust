# Agent: Port State Machine (C → Rust)

**Model tier: Start with Haiku, escalate to Sonnet if needed**

## When to Use
- Porting C state machines (switch/case, function-pointer dispatch)
- Modules like xmlrole.c that are naturally suited to Rust enums + match

## Prompt Template

```
You are porting a C state machine to safe, idiomatic Rust.

1. Read the C source: {C_SOURCE_PATH}
2. Read the header: {HEADER_PATH}

3. Analyze the state machine:
   - Identify all states (enum values, #defines, or implicit states)
   - Identify all transitions (switch/case arms, function pointer changes)
   - Identify inputs that drive transitions

4. Design the Rust equivalent:
   - States → Rust enum
   - Transitions → match arms
   - Function pointers → enum dispatch or trait methods
   - Replace #define constants with Rust enums or consts

5. Write the Rust port at: {RUST_OUTPUT_PATH}

Requirements:
- Zero `unsafe`
- Use Rust enums + match instead of integer states + switch
- Preserve all state transitions exactly
- Return values should use Result<T, E> where C returns error codes

6. Verify:
   source "$HOME/.cargo/env" && cd {RUST_PROJECT_PATH} && cargo check
```

## Notes
- xmlrole.c is the first candidate — 1,255 lines, clean state machine
- If Haiku produces incorrect state transitions, escalate to Sonnet
- Document which model tier works in plans/process-log.md
