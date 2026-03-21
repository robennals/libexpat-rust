---
name: feedback_no_parallel_same_file
description: Never run parallel agents that edit the same file - they clobber each other's changes
type: feedback
---

Never run multiple parallel agents that edit the same source file. They overwrite each other's changes and cause regressions.

**Why:** Three parallel haiku agents all editing xmlparse.rs caused the test count to drop from 213 passing to 179 passing. Each agent's edits overwrote the previous agent's fixes.

**How to apply:** Either (1) use worktree isolation so each agent works on its own copy, or (2) run agents sequentially on the same file, or (3) only parallelize agents that work on different files.
