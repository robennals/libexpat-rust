# expat-ffi — C Drop-in Replacement for libexpat

This crate builds a shared library (`libexpat.so` / `libexpat.dylib` / `expat.dll`) that implements the libexpat C API using `expat-rust` under the hood.

## Quick Migration Guide

If you have a C or C++ application that uses libexpat and you want to switch to expat-rust:

### Step 1: Build the library

```bash
git clone --recurse-submodules https://github.com/robennals/libexpat-rust.git
cd libexpat-rust
cargo build --release -p expat-ffi
```

This produces:
- **Linux**: `target/release/libexpat.so`
- **macOS**: `target/release/libexpat.dylib`
- **Windows**: `target/release/expat.dll`

### Step 2: Replace libexpat

**Option A — Replace the system library** (not recommended for production):
```bash
# Find where your app looks for libexpat
ldd your_app | grep expat         # Linux
otool -L your_app | grep expat    # macOS

# Replace it
sudo cp target/release/libexpat.so /usr/lib/libexpat.so.1
```

**Option B — Use LD_LIBRARY_PATH** (recommended for testing):
```bash
# Linux
LD_LIBRARY_PATH=target/release ./your_app

# macOS
DYLD_LIBRARY_PATH=target/release ./your_app
```

**Option C — Relink your application**:
```bash
# Recompile linking against the new library
cc -o your_app your_app.c -Ltarget/release -lexpat -Wl,-rpath,target/release
```

### Step 3: There is no step 3

The API is the same. `XML_ParserCreate`, `XML_Parse`, `XML_SetElementHandler` — they all work identically. Your code doesn't need to change.

## API Coverage

The full libexpat public API is exposed — 79 functions covering:

- **Parser lifecycle**: `XML_ParserCreate`, `XML_ParserCreateNS`, `XML_ParserReset`, `XML_ParserFree`
- **Parsing**: `XML_Parse`, `XML_GetBuffer`, `XML_ParseBuffer`, `XML_StopParser`, `XML_ResumeParser`
- **Error handling**: `XML_GetErrorCode`, `XML_ErrorString`
- **Position**: `XML_GetCurrentLineNumber`, `XML_GetCurrentColumnNumber`, `XML_GetCurrentByteIndex`, `XML_GetCurrentByteCount`
- **Configuration**: `XML_SetEncoding`, `XML_SetBase`, `XML_GetBase`, `XML_SetHashSalt`, `XML_SetParamEntityParsing`, `XML_UseForeignDTD`, `XML_SetReturnNSTriplet`, `XML_SetReparseDeferralEnabled`, `XML_UseParserAsHandlerArg`
- **All handler setters**: Element, character data, processing instruction, comment, CDATA section, default, DOCTYPE declaration, XML declaration, external entity ref, plus individual start/end variants
- **Attribute info**: `XML_GetSpecifiedAttributeCount`, `XML_GetIdAttributeIndex`
- **External entities**: `XML_ExternalEntityParserCreate`
- **Security**: `XML_SetBillionLaughsAttackProtectionMaximumAmplification`, `XML_SetBillionLaughsAttackProtectionActivationThreshold`
- **Version**: `XML_ExpatVersion`

## Examples

See the [`examples/`](examples/) directory:

```bash
# Build the Rust library first
cargo build --release -p expat-ffi

# Build and run the C example
cd expat-ffi/examples
make run
```

## Testing

22 integration tests written in C verify the FFI layer end-to-end — real C code calling the Rust library through the standard libexpat API:

```bash
cargo build --release -p expat-ffi
make -C expat-ffi/tests
```

Tests cover parser lifecycle, all handler types (elements, character data, comments, PIs, CDATA, XML declarations, DOCTYPE), error handling, incremental parsing, parser reset, and configuration.

## Implementation Notes

### Memory management APIs

The C libexpat API includes `XML_MemMalloc`, `XML_MemRealloc`, and `XML_MemFree`, which let C code allocate memory through the parser's allocator. In C libexpat, parsers can be created with a custom allocator via `XML_ParserCreate_MM`, and these functions use that custom allocator.

In our Rust implementation, the parser uses Rust's standard allocator — there is no custom allocator support. The `XML_MemMalloc`/`XML_MemRealloc`/`XML_MemFree` functions are implemented in the FFI layer by forwarding directly to libc `malloc`/`realloc`/`free`. They exist solely for C API compatibility (some C code, including libexpat's own test suite, calls them). The Rust `Parser` struct has no knowledge of these functions.

Similarly, `XML_ParserCreate_MM` accepts a `XML_Memory_Handling_Suite` parameter but ignores it — the Rust parser always uses its own allocator. The 5 C test failures related to custom allocators (`test_misc_alloc_*`, `test_accounting_*`, `test_amplification_*`) are expected and documented as not applicable.

### ParserHandle layout

The `ParserHandle` struct uses `#[repr(C)]` with `user_data` as its first field. This is required because the C macro `XML_GetUserData(parser)` is defined as `(*(void **)(parser))`, which reads the first word of the struct directly.

### Handler user_data timing

All handler closures read `user_data` from the `ParserHandle` at call time (via a raw pointer to the handle), not at registration time. This matches C behavior where `XML_SetUserData` can be called after handler registration.

## Building from source

```bash
# Debug build
cargo build -p expat-ffi

# Release build (optimized)
cargo build --release -p expat-ffi

# Static library (for static linking into C/C++ apps)
# The staticlib is also produced: target/release/libexpat.a
```
