# C2Rust to Idiomatic Rust Transformation Guide

This document defines the patterns and rules for transforming C2Rust-generated Rust code
into safe, idiomatic Rust that matches the existing port's conventions.

## Context

The C2Rust output is a mechanically correct translation of libexpat's C code into unsafe Rust.
Our goal is to transform it into safe, idiomatic Rust that:
1. Maintains 1:1 function correspondence with the C call tree
2. Uses Rust's type system for safety (no raw pointers, no unsafe)
3. Follows the naming/structure conventions of the existing port
4. Passes all the same tests

## Transformation Rules

### 1. Parser Access Pattern
**C2Rust**: `(*parser).m_fieldName`
**Idiomatic**: `self.field_name` (as method on `Parser` struct)

The `parser: XML_Parser` parameter (which is `*mut XML_ParserStruct`) becomes `&mut self`.
All `(*parser).m_foo` accesses become `self.foo` (with snake_case field names, dropping `m_` prefix).

### 2. Raw Pointers to References/Slices
**C2Rust**: `*const c_char` / `*mut c_char` with pointer arithmetic
**Idiomatic**: `&[u8]` for read-only byte data, `&mut [u8]` for mutable, `&str` for string data

Common patterns:
- `(s, end)` pointer pairs → `data: &[u8]` slice
- `*nextPtr = next` → return the position/offset instead
- Pointer arithmetic `s.offset(n)` → `&data[n..]`

### 3. Integer-as-Boolean
**C2Rust**: `XML_Bool` (unsigned char), `!= 0`, `== 0`
**Idiomatic**: `bool`, direct boolean expressions

### 4. Error Handling
**C2Rust**: Returns `XML_Error` enum values, checks `== XML_ERROR_NONE`
**Idiomatic**: Keep the `XmlError` enum but use `Result<T, XmlError>` where appropriate

### 5. String Handling
**C2Rust**: `*const c_char` + length, `memcpy`, pointer arithmetic
**Idiomatic**: `&str`, `String`, `&[u8]` as appropriate

### 6. Memory Management
**C2Rust**: `malloc(size)`, `free(ptr)`, `realloc(ptr, size)`
**Idiomatic**: `Vec<T>`, `Box<T>`, `String`

### 7. Function Pointers
**C2Rust**: `Option<unsafe extern "C" fn(...) -> ...>`
**Idiomatic**: `Option<Box<dyn FnMut(...) -> ...>>` for handlers

### 8. Goto/current_block Patterns
**C2Rust**: `let mut current_block: u64 = ...; match current_block { ... }`
**Idiomatic**: Restructure as `loop`/`match`/early returns/`?` operator

### 9. Type Aliases
- `XML_Char` → `u8` (UTF-8 mode) or `char`
- `XML_Size` → `u64`
- `XML_Index` → `i64`
- `XML_Bool` → `bool`

### 10. Enum Constants
**C2Rust**: `pub const XML_STATUS_ERROR: XML_Status = 0;`
**Idiomatic**: `enum XmlStatus { Error = 0, Ok = 1, Suspended = 2 }`

### 11. Struct Field Naming
**C2Rust**: `m_errorCode`, `m_parsingStatus`, `m_encoding`
**Idiomatic**: `error_code`, `parsing_status`, `encoding` (snake_case, no `m_` prefix)

## Function Name Mapping (C → Rust)

| C Function | Rust Method/Function |
|-----------|---------------------|
| XML_ParserCreate | Parser::new |
| XML_ParserCreateNS | Parser::new_ns |
| XML_ParserCreate_MM | (internal to new/new_ns) |
| XML_Parse | parser.parse |
| XML_ParseBuffer | parser.parse_buffer |
| XML_GetBuffer | parser.get_buffer |
| XML_ParserReset | parser.reset |
| XML_ParserFree | Drop impl |
| XML_SetXxxHandler | parser.set_xxx_handler |
| XML_StopParser | parser.stop |
| XML_ResumeParser | parser.resume |
| XML_GetErrorCode | parser.error_code |
| XML_ErrorString | error_string (free fn) |
| contentProcessor | parser.content_processor |
| prologProcessor | parser.prolog_processor |
| doContent | parser.do_content |
| doProlog | parser.do_prolog |
| storeAtts | parser.store_atts |
| etc. | etc. |

## Processor Architecture

The C code uses function pointers for processors:
```c
(*parser).m_processor = contentProcessor;
result = (*parser).m_processor(parser, s, end, &next);
```

The Rust port uses an enum:
```rust
enum Processor {
    PrologInit,
    Prolog,
    Content,
    Epilog,
    CdataSection,
    IgnoreSection,
    // etc.
}
```

And dispatches through `run_processor()` method.

## Important Conventions

1. All functions that take `parser: XML_Parser` as first arg become methods on `Parser`
2. Static/internal functions become private methods (no `pub`)
3. Public API functions (XML_*) become pub methods with snake_case names
4. Preserve the exact same control flow and error paths as C
5. Don't optimize or restructure the logic - just make it safe Rust
