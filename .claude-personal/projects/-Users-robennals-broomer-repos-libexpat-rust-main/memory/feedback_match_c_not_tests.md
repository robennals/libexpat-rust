---
name: feedback_match_c_not_tests
description: Goal is C behavioral equivalence verified by comparison tests, not ad-hoc test fixing
type: feedback
---

The goal is matching C behavior exactly. Tests are an indicator, not the goal itself.

**Why:** Ad-hoc fixing to make tests pass can introduce subtle behavioral divergences that pass the test but don't match C. The comparison test infrastructure (expat-sys + CParser) is the ground truth.

**How to apply:**
1. Use comparison tests (Rust vs C library via FFI) as the primary verification method
2. When fixing failures, read the C source to understand correct behavior, don't just tweak until the test passes
3. Use scripts to systematically generate comparison tests for untested behaviors
4. Prefer expanding comparison test coverage over fixing individual unit tests
