# v2 Engine Action Items

## Status: 4/15 function pairs fully verified, 34 remaining mismatches

## Principled rules needed (8 mismatches)

These are Rust-idiomatic patterns that require simple rules:

1. **C `for(;;)` = Rust `let + let + while`** (3 cases: epilogProcessor, doCdataSection, doProlog)
   - C uses `for(;;)` with break/return inside
   - Rust needs `let mut pos = 0; let len = data.len(); while pos < len { ... }`
   - Rule: multi-child match, body still recursively compared

2. **C `enc == encoding` branch** (1 case: doProlog)
   - C selects event pointer pair based on encoding. Rust is UTF-8 only.
   - Already have rule but not matching pattern correctly.

3. **C event pointer struct field assigns** (2 cases: extEntInitProc3)
   - `parser->m_eventPtr = start; parser->m_eventEndPtr = next;`
   - Rust tracks position via return values, not struct mutation.

4. **Rust if-let handler dispatch** (1 case: reportComment)
   - `if let Some(handler) = &mut self.comment_handler { handler(...) }`
   - C calls handler directly via function pointer.

5. **C `openEntity` field init** (1 case: processEntity)
   - `openEntity->internalEventEndPtr = NULL;`
   - Rust initializes differently.

## Rust code to fix (4 functions)

These have extra or restructured logic that should match C more closely:

### contentProcessor (4 extra Rust nodes)
```
R@2751: let (error, next_pos) = self.do_content(...)
R@2754: if error == XmlError::None { self.event_pos = next_pos; }
R@2758: if error != XmlError::None { self.error_code = error; }
R@2763: if next_pos < data.len() && error == XmlError::None { self.buffer = ... }
```
C just does: `result = doContent(...); if(result==NONE) storeRawNames(); return result;`
**Fix**: Restructure to match C's simpler flow. Remove extra error checks.

### prologProcessor (5 extra Rust nodes)
```
R@1373: if data.is_empty() { ... }       — extra empty guard
R@1395: let (error, next_pos) = ...       — destructuring
R@1397: if error != XmlError::None { ... } — error handling
R@1403: if self.processor == Processor::Content && next_pos < data.len() { ... }
R@1416: if next_pos < data.len() { ... }  — buffer management
```
C just does: `tok = prologTok(...); return doProlog(...);`
**Fix**: Restructure to match C's simpler flow.

### reportDefault (2 extra Rust nodes)
```
R@4726: let chunk = &data[start..end];
R@4727: if let Some(handler) = &mut self.default_handler_expand { ... }
```
C has: `if(MUST_CONVERT) { ... } else { defaultHandler(handlerArg, s, len) }`
**Fix**: Remove `default_handler_expand` variant (not in C). Use direct
handler call matching C's pattern.

### externalEntityInitProcessor3 (4 extra C nodes beyond position tracking)
```
C@3260: switch (tok) { ... }
C@3294: parser->m_processor = externalEntityContentProcessor;
C@3295: parser->m_tagLevel = 1;
C@3296: return externalEntityContentProcessor(parser, start, end, endPtr);
```
**Fix**: Rust should have equivalent switch/processor/tagLevel/tail-call.

## Structural mapping issue (1 function)

### processXmlDecl → handle_prolog_role (5 mismatches)
C's `processXmlDecl` is a standalone 87-line function. It's mapped to Rust's
`handle_prolog_role` which is much larger and contains the equivalent logic
inside a specific match arm (lines 1740-1758).
**Fix**: Map to the specific arm, not the whole function. Or extract the logic
into a separate Rust function matching C's structure.
