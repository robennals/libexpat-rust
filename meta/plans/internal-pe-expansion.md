# Plan: Internal Parameter Entity Expansion

## Problem

When `%entity_name;` appears in the DTD and the entity has a stored text value (internal PE), C calls `processEntity()` to expand it inline. This sets the processor to `internalEntityProcessor`, which calls `doProlog` on the entity text. The entity text may itself contain declarations, PE references (causing recursion detection), etc.

Our Rust code currently stubs this out — it sets `handler_called = true` but doesn't actually expand the PE. This means:
- Recursive PE detection doesn't work (no `entity->open` tracking during expansion)
- PE text isn't parsed, so declarations inside PE values are lost
- ~7 tests fail

## How C Does It

### processEntity (xmlparse.c:6360)
1. Sets `parser->m_processor = internalEntityProcessor`
2. Pushes an `OPEN_INTERNAL_ENTITY` onto `m_openInternalEntities` linked list
3. Sets `entity->open = true`, `entity->processed = 0`, `entity->hasMore = true`
4. Calls `triggerReenter(parser)` which sets `m_reenter = true`
5. Returns `XML_ERROR_NONE`

### triggerReenter mechanism
After `processEntity` returns, `doProlog` continues to the parsing-status switch at the bottom of its main loop. Line 6268: if `m_reenter` is true and parsing status is `XML_PARSING`, doProlog returns `XML_ERROR_NONE` with `*nextPtr = next` (it consumed the PE ref token).

Control returns to `callProcessor`'s loop (xmlparse.c:1292). The loop checks `m_reenter` (true), clears it, and loops. Now `m_processor` is `internalEntityProcessor`, so that runs next.

### internalEntityProcessor (xmlparse.c:6420)
1. Gets the top `OPEN_INTERNAL_ENTITY`
2. If `entity->hasMore`:
   - If `entity->is_param`: calls `doProlog(parser, internalEncoding, textStart, textEnd, &next, ...)`
   - Else: calls `doContent(...)`
   - If entity text is fully consumed: sets `hasMore = false`, calls `triggerReenter`
   - If not fully consumed (suspended): saves `entity->processed`, returns
3. If `!entity->hasMore` (cleanup pass — second call after triggerReenter):
   - Sets `entity->open = false`
   - Pops entity from `m_openInternalEntities`
   - Restores processor: `is_param ? prologProcessor : contentProcessor`
   - Calls `triggerReenter` (so callProcessor loops again with restored processor)

### Key observation
The C mechanism uses a two-pass approach per entity:
- **Pass 1**: `hasMore=true` — process entity text via doProlog/doContent
- **Pass 2**: `hasMore=false` — cleanup (close entity, restore processor)

Both passes are driven by `callProcessor`'s reenter loop, not by recursion. This prevents stack overflow for deeply nested entities.

## Available Approaches

### Approach A: Port the reenter flag (match C exactly)

Add `reenter: bool` to Parser. After `processEntity`, `do_prolog` checks `reenter` at the bottom of its main loop and returns `(XmlError::None, next)`. `run_processor` checks `reenter` after each processor call and loops.

**Pros:**
- Exact C match — easiest to verify correctness
- Non-recursive — bounded stack depth
- The reenter mechanism is also used by `doContent`'s entity expansion

**Cons:**
- Adds a mutable flag that's hard to reason about in Rust
- The flag is "spooky action at a distance" — set in one function, checked in another

**Complexity:** ~30 lines changed in `run_processor` + `do_prolog` + `do_content`

### Approach B: Use processor enum change as implicit reenter signal

Instead of a separate flag, detect that the processor changed to `InternalEntity` inside `do_prolog`/`do_content` and return immediately. `run_processor` already re-dispatches when the processor changes. This is what we attempted but it caused regressions.

**The regression root cause:** When `do_prolog` returns early after `process_entity` sets the processor to `InternalEntity`, the prolog state machine hasn't consumed the token properly. The `next` position is set, but `do_prolog` skips the `report_default` call and the `advance_pos_slice` call for the PE ref token. When `internal_entity_processor` later restores the processor to `Prolog` and `run_processor` re-dispatches, `do_prolog` starts at a position that has already been partially processed.

**Fix:** The return from `do_prolog` must return `next` (not `pos`), and the position tracking (advance_pos_slice, report_default) must happen BEFORE the InternalEntity check, not after. The current code checks InternalEntity immediately after `handle_prolog_role`, before doing position tracking.

**Pros:**
- No extra mutable state
- Type-safe (processor is already an enum)
- Consistent with how we handle Content processor transition

**Cons:**
- Requires careful ordering of position tracking vs processor check
- Slightly different from C's mechanism (but equivalent behavior)

**Complexity:** ~15 lines changed — fix ordering in `do_prolog` and `do_content`

### Approach C: Expand inline (recursive call)

Instead of switching processors, directly call `do_prolog` on the entity text from within the PE ref handler.

**Pros:**
- Simple to implement

**Cons:**
- **Recursive** — deep entity nesting causes stack overflow (C explicitly avoids this)
- Doesn't match C architecture
- Makes suspend/resume impossible during entity expansion

**Not recommended.**

## Recommendation: Approach B (processor enum, fix the ordering)

Approach B is the closest to what we already have and avoids adding new mutable state. The specific fix needed:

```rust
// In do_prolog, after handle_prolog_role returns:
// 1. FIRST: handle default reporting and position tracking
if (self.default_handler.is_some() || ...) && !suppress_default {
    self.report_default(&enc, data, pos, next);
}
self.advance_pos_slice(&data[pos..next]);

// 2. THEN: check if processor changed
if self.processor == Processor::Content {
    return (XmlError::None, pos);  // Content needs to re-tokenize
}
if self.processor == Processor::InternalEntity {
    return (XmlError::None, next);  // Entity expansion consumes the PE ref
}

// 3. Continue loop
pos = next;
```

The key insight: report_default and advance_pos must happen BEFORE the InternalEntity check, because the PE ref token was consumed and needs to be accounted for.

Similarly, in `do_content`, the EntityRef expansion via `process_entity` needs the same pattern.

### Two-pass cleanup
The `internal_entity_processor` needs to match C's two-pass approach:
1. First call: process entity text, on completion set a `has_more = false` equivalent
2. Second call: cleanup (pop entity, mark closed, restore processor)

Our current code tries to do both in one call, which doesn't work because it needs to let `run_processor` re-dispatch between passes.

Add `has_more: bool` to `OpenInternalEntity` struct, matching C.

## Tests Expected to Pass After This Fix

- `test_recursive_external_parameter_entity` — PE recursion detection via `open` flag
- `test_recursive_external_parameter_entity_2` — same
- `test_no_indirectly_recursive_entity_refs` — indirect recursion detection
- `test_param_entity_with_trailing_cr` — PE expansion with CR handling
- `test_standalone_parameter_entity` — should stop regressing
- `test_standalone_internal_entity` — should stop regressing
