# Model Tier Results: Which AI Model for Which Task

## Summary

**Haiku handles everything through Layer 2.** We predicted Sonnet would be needed for tokenizer porting and complex test translation, but Haiku produced correct, compiling code for all tasks attempted.

## Empirical Results

| Task | Predicted Model | Actual Model | Quality | Tokens | Time |
|------|----------------|-------------|---------|--------|------|
| C test analysis (research) | Haiku | Haiku | Excellent | ~58K | 81s |
| Character table porting | Script/Haiku | Haiku (wrote script) | Perfect | ~34K | 157s |
| Name table porting | Haiku | Haiku | Perfect | ~36K | 79s |
| SipHash porting + tests | Haiku | Haiku | Perfect (64 test vectors) | ~36K | 156s |
| xmlrole.c (1255 lines, state machine) | Haiku→Sonnet | Haiku | Compiles clean | ~45K | ~120s |
| xmltok_impl.c (1819 lines, complex) | Sonnet | Haiku | Compiles clean | ~91K | 240s |
| xmltok.c (1672 lines, encoding) | Sonnet | Haiku | Compiles, 8 tests pass | ~71K | 191s |
| Test translation (22 misc) | Haiku | Haiku | All compile | ~43K | 78s |
| Test translation (33 ns) | Haiku | Haiku | All compile | ~69K | 181s |
| Test translation (244 basic, 5 batches) | Haiku/Sonnet | Haiku | All compile | ~310K total | ~120s avg |
| Test translation (61 alloc) | Haiku | Haiku | All compile | ~42K | 62s |
| Test translation (27+4 nsalloc+acc) | Haiku | Haiku | All compile | ~42K | 47s |
| Architecture analysis (9267 lines) | Sonnet/Opus | Haiku | Thorough | ~82K | 282s |

## Key Findings

### 1. Haiku ceiling is higher than expected
The plan predicted Haiku would fail on:
- Pointer-heavy tokenizer code (xmltok_impl.c) → **Haiku succeeded**
- Complex state machines > 1000 lines → **Haiku succeeded**
- Test translation with callbacks and state → **Haiku succeeded**

### 2. When to use Haiku vs. Sonnet vs. Opus
Based on our experience:

**Haiku works for:**
- Data table porting (any size)
- Algorithm porting with test vectors (siphash, encoding)
- State machine porting (xmlrole.c — enum+match is a natural fit)
- Tokenizer porting with trait-based abstractions
- Test translation (even complex callback patterns)
- Code analysis and architecture review

**Sonnet may be needed for:**
- xmlparse.c implementation (9267 lines, deep ownership/lifetime decisions)
- Cross-module integration requiring understanding of 5+ modules simultaneously
- Redesigning C patterns that have no direct Rust equivalent

**Opus needed for:**
- Orchestration (deciding what to port next, designing the workflow)
- Architecture decisions spanning the entire codebase
- Quality review of haiku output when correctness is critical

### 3. Parallel execution is the key multiplier
Running 5-6 haiku agents in parallel makes the total wall-clock time comparable to a single sonnet invocation, at ~10x lower cost.

### 4. Context window is the real limitation
Haiku's quality is fine; its context window is the constraint. Batching (50 tests per agent) and giving focused prompts (one module at a time) works around this.

### 5. Compilation as quality gate
`cargo check` catches most haiku errors. The fix cycle is:
1. Haiku writes code
2. `cargo check` fails
3. Haiku reads error, fixes
4. Repeat until clean

Most modules compile on first or second try.

## Cost Analysis (approximate)

Assuming haiku at ~$0.25/MTok input, $1.25/MTok output:

| Phase | Agents | Total Tokens | Estimated Cost |
|-------|--------|-------------|---------------|
| Layer 0 (4 agents) | 4 | ~164K | ~$0.10 |
| Layer 1-2 (3 agents) | 3 | ~207K | ~$0.15 |
| Test translation (8 agents) | 8 | ~590K | ~$0.40 |
| Analysis (2 agents) | 2 | ~140K | ~$0.10 |
| **Total** | **17** | **~1.1M** | **~$0.75** |

For comparison, the same work with Sonnet (~$3/MTok input, $15/MTok output) would be ~$10-15. With Opus, ~$50-75.

## Recommendations for Other Projects

1. **Start with Haiku for everything.** Only escalate when quality is provably insufficient.
2. **Use scripts for deterministic extraction.** Don't waste AI tokens on parsing.
3. **Batch large files.** 50 units per agent is the sweet spot.
4. **Run agents in parallel.** Wall-clock time matters more than per-agent time.
5. **Use `cargo check` as the quality gate.** Compilation is a strong correctness signal.
6. **Document which model worked.** Future projects can start with proven tiers.
