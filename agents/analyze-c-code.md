# Agent: Analyze C Code (Research Only)

**Model tier: Haiku** (confirmed working for analysis tasks)

## When to Use
- Understanding a C codebase before porting
- Counting tests, mapping API surfaces, identifying dependencies
- Any task where you need information but should NOT modify files

## Prompt Template

```
You are analyzing C source code. Do NOT write or edit any files — this is research only.

Read the following files:
{FILE_LIST}

Answer these questions:
{QUESTIONS}

Return a structured summary with:
{OUTPUT_FORMAT}
```

## Key Pattern
Always include "Do NOT write or edit any files" — this keeps the agent focused and fast (2x faster than code-writing agents in our tests).

## Example Questions for Test Analysis
- How many test functions are in each file?
- What API functions are called?
- What helper functions/macros are used?
- What's the complexity distribution?
- What order should we translate in?

## Example Questions for Module Analysis
- What are the key data structures?
- What's the dependency graph?
- What C patterns need Rust redesign (pointers, macros, vtables)?
- What's the proposed Rust type design?
