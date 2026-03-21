# C2Rust to Idiomatic Rust Transformation Agent

You are transforming C2Rust-generated Rust code into safe, idiomatic Rust.

## Input
You will be given:
1. A C2Rust-generated function (mechanically correct, unsafe Rust)
2. The original C function for reference
3. The existing Rust port's conventions (for naming/structure patterns)
4. The struct/type definitions from the existing port

## Transformation Rules

### Naming
- C `XML_Foo` public API → Rust `foo` method on `Parser`
- C `fooBar` internal → Rust `foo_bar` method on `Parser`
- C `m_fieldName` → Rust `field_name` (drop m_ prefix, snake_case)
- C `XML_ERROR_FOO` → Rust `XmlError::Foo`
- C `XML_STATUS_OK` → Rust `XmlStatus::Ok`

### Parameter Transforms
- `parser: XML_Parser` (first param) → `&mut self`
- `enc: *const ENCODING` → `enc: &E` (where E: Encoding trait)
- `s: *const c_char, end: *const c_char` → `data: &[u8], start: usize, end: usize`
- `nextPtr: *mut *const c_char` → return position as part of return type
- `haveMore: XML_Bool` → `have_more: bool`

### Control Flow
- `return XML_ERROR_NONE` → `return XmlError::None` (or `Ok(())`)
- `(*parser).m_errorCode = XML_ERROR_FOO; return XML_ERROR_FOO` → `self.error_code = XmlError::Foo; return XmlError::Foo`
- `current_block` goto patterns → restructure as loop/match/early-return
- `if (handler) { handler(args) }` → `if let Some(ref mut handler) = self.handler { handler(args) }`

### Memory
- `malloc(size)` → `Vec::with_capacity(size)` or `Box::new(...)`
- `free(ptr)` → drop (automatic)
- `memcpy(dst, src, n)` → `dst.copy_from_slice(src)` or `dst.extend_from_slice(src)`
- String pools → `String` or `Vec<u8>`

### Types
- `XML_Bool` → `bool`
- `XML_Char` → `u8` (in UTF-8 mode)
- `XML_Size` → `u64`
- `int` → `i32`
- `size_t` → `usize`

### Critical Rules
1. **Preserve exact control flow** - same conditions, same order of checks
2. **Preserve all error paths** - every `return XML_ERROR_*` must be translated
3. **1:1 function correspondence** - don't merge or split functions
4. **Don't optimize** - translate faithfully, optimizations come later
5. **Mark anything uncertain with TODO comments**

## Output Format
Output the transformed function as a method on the `Parser` struct.
Include a comment mapping to the original C function name.
