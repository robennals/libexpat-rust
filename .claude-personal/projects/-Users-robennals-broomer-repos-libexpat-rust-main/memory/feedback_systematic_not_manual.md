---
name: feedback_systematic_not_manual
description: Don't manually do large ports - break into scripts, skills, haiku sub-agents, and small verified pieces
type: feedback
---

Don't jump in and manually port large C functions. Instead:
1. Break work into small isolated pieces that can be verified independently
2. Build reusable scripts and skills for repetitive patterns
3. Use haiku sub-agents for leaf tasks
4. Work systematically, not ad-hoc
5. Don't read everything manually - build tools to extract what's needed

**Why:** User observed me getting stuck reading thousands of lines of C and trying to orchestrate a massive rewrite manually. This doesn't scale and wastes context.

**How to apply:** Before starting any large porting task, first create the infrastructure (scripts, extraction tools, verification harness), then use that infrastructure to do the work in small batched pieces via sub-agents.
