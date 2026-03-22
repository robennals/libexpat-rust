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

## What's Implemented

The following libexpat functions are currently exposed:

| Function | Status |
|----------|--------|
| `XML_ParserCreate` | Implemented |
| `XML_ParserCreateNS` | Implemented |
| `XML_ParserFree` | Implemented |
| `XML_Parse` | Implemented |
| `XML_GetErrorCode` | Implemented |
| `XML_ErrorString` | Implemented |
| `XML_GetCurrentLineNumber` | Implemented |
| `XML_GetCurrentColumnNumber` | Implemented |
| `XML_SetUserData` | Implemented |
| `XML_SetElementHandler` | Implemented |
| `XML_SetCharacterDataHandler` | Implemented |
| `XML_SetCommentHandler` | Implemented |
| `XML_ExpatVersion` | Implemented |

Additional handlers (PI, CDATA, DTD, namespace, etc.) can be added incrementally. The Rust parser supports the full API — only the FFI wrappers need to be written.

## Examples

See the [`examples/`](examples/) directory:

```bash
# Build the Rust library first
cargo build --release -p expat-ffi

# Build and run the C example
cd expat-ffi/examples
make run
```

## Building from source

```bash
# Debug build
cargo build -p expat-ffi

# Release build (optimized)
cargo build --release -p expat-ffi

# Static library (for static linking into C/C++ apps)
# The staticlib is also produced: target/release/libexpat.a
```
