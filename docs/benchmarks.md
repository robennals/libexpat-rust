# Benchmark Results

Performance comparison between `expat-rust` and C libexpat 2.7.5.

## Environment

- **Platform**: macOS (Apple Silicon)
- **Rust**: stable (release profile, LTO enabled, codegen-units=1)
- **C**: libexpat 2.7.5 compiled via `cc` crate with `-O2`
- **Tool**: [criterion.rs](https://github.com/bheisler/criterion.rs) with 100 samples per benchmark. Each sample contains thousands of auto-calibrated iterations — sub-microsecond timings are the median of hundreds of thousands of runs. The 100 MB benchmarks use 10 samples due to per-iteration cost.
- **Methodology**: Both parsers register start-element, end-element, and character-data handlers (no-op closures / C callbacks) to force real parsing work. Each iteration creates a fresh parser.

## Results

| Scenario | expat-rust | libexpat (C) | Ratio | Notes |
|----------|-----------|-------------|-------|-------|
| Small document (44 B) | 498 ns | 1.04 us | **0.48x** | Rust wins — 2x faster, lower per-parser overhead |
| Medium document (~10 KB) | 134 us | 81 us | 1.65x | C wins — arena allocator advantage |
| 100 KB document | 1.38 ms | 904 us | 1.53x | C wins — consistent with medium |
| 100 MB document | 438 ms | 278 ms | 1.58x | Scales linearly — ratio stays constant |
| 100 MB streamed (8 KB chunks) | 428 ms | 345 ms | 1.24x | Realistic streaming workload |
| Deep nesting (100 levels) | 7.9 us | 21.0 us | **0.38x** | Rust wins — 2.7x faster |
| Many attributes (25/elem) | 23.1 us | 17.9 us | 1.29x | C wins — interned strings vs String reuse |
| Error detection (malformed) | 445 ns | 1.01 us | **0.44x** | Rust wins — 2.3x faster early-exit paths |

## Analysis

### Where Rust is faster

**Small documents and parser creation**: Rust's `Parser::new()` is lighter than C's `XML_ParserCreate()` because it doesn't need to set up a memory allocator, string pool, or hash table infrastructure. Rust is 2x faster for small documents. For applications that create many short-lived parsers, this is a significant advantage.

**Deep nesting**: Rust's `Vec`-based element stack is more efficient than C's linked structures. Rust is 2.7x faster for deeply nested documents.

**Error detection**: Rust's pattern matching and early-return paths in the tokenizer are more efficient than C's switch/goto error handling. Rust is 2.3x faster at detecting malformed XML.

### Where C is faster (1.2-1.6x)

**Element-heavy documents**: C's `STRING_POOL` arena allocator amortizes allocation costs by allocating large blocks and carving out strings from them. Rust uses standard `String` and `Vec` types with per-element allocation, mitigated by buffer reuse across elements. For documents with thousands of elements, C's arena approach still has an edge. The ratio stays constant as documents scale from 10 KB to 100 MB.

**Attribute-heavy documents**: C interns attribute names in a hash table with pool-allocated strings. Rust reuses `String` capacity across elements but still allocates per-attribute. The gap is modest (1.29x).

### The trade-off

The 1.2-1.6x gap on element-heavy documents is a deliberate design choice. Replacing C's arena allocator and interning hash table with Rust's standard library types was essential for:

1. **Memory safety**: Arena allocators in Rust would require `unsafe` or complex lifetime annotations
2. **Simplicity**: Standard types are well-tested and easy to reason about
3. **Correctness**: Simpler code means fewer bugs

For most real-world applications, the Rust parser processes 100 MB of XML in under half a second. In streaming mode (how expat is designed to be used), Rust is within 24% of C.

### Optimizations applied

The following optimizations bring Rust close to C performance while maintaining zero `unsafe` in the core parser:

- **Lazy event data**: Token byte data for `XML_DefaultCurrent` is captured by position only; the copy is deferred until actually needed
- **Fast-path attribute normalization**: Attribute values without special characters (`&`, `\r`, `\n`, `\t`) skip the normalization allocator entirely
- **Zero-copy tag names**: In the common non-namespace path, tag names borrow directly from the input buffer instead of allocating `String`s
- **Reusable attribute buffers**: The attribute `Vec` and its `String` entries are reused across elements, preserving heap capacity
- **Consolidated DTD borrows**: A single `RefCell` borrow per element replaces multiple borrows for ATTLIST defaults, type normalization, and ID attribute lookup
- **LTO and codegen-units=1**: Link-time optimization enables cross-crate inlining of the tokenizer hot path

---

## Memory Usage

Peak heap allocation comparison between `expat-rust` and C libexpat 2.7.5.

### Environment

- **Platform**: macOS (Apple Silicon)
- **Methodology**: Rust allocations tracked via a custom `GlobalAlloc` wrapper. C allocations tracked via `XML_ParserCreate_MM` with custom malloc/realloc/free functions that prepend a size header. Both parsers register start-element, end-element, and character-data handlers. Minimum of 3 runs per scenario, minimum peak reported.

### Streaming (the headline result)

expat is a streaming parser — it's designed to parse arbitrarily large inputs with bounded memory. This is the most important memory property to verify. Both parsers are fed data in fixed 8 KB chunks.

| Total Data Parsed | Rust Peak | C Peak | Ratio |
|-------------------|-----------|--------|-------|
| 10 MB (150K elements) | **32.7 KB** | 30.7 KB | 1.07x |
| 50 MB (750K elements) | **32.7 KB** | 30.7 KB | 1.07x |
| 200 MB (3M elements) | **32.7 KB** | 30.7 KB | 1.07x |

**Both parsers use ~33 KB regardless of whether they parse 10 MB or 200 MB.** Memory is bounded by chunk size and parser state (including nesting depth), not by total bytes parsed. For flat or shallowly-nested documents, memory stays constant. Deeply nested structures will increase stack memory proportionally to depth. Rust is within 7% of C in streaming mode.

### One-shot parsing (entire document in memory)

When the full document is passed to `parse()` in a single call, the parser must buffer internal state proportional to the input.

| Scenario | Doc Size | expat-rust | libexpat (C) | Ratio |
|----------|----------|-----------|-------------|-------|
| Small document | 44 B | 3.2 KB | 8.4 KB | **0.39x** |
| 10 KB | 29 KB | 90 KB | 39 KB | 2.33x |
| 100 KB | 221 KB | 666 KB | 263 KB | 2.53x |
| 1 MB | 2.2 MB | 6.6 MB | 4.0 MB | 1.65x |
| 10 MB | 22.4 MB | 67 MB | 32 MB | 2.10x |
| Deep nesting (100) | 1.8 KB | 9.6 KB | 25.9 KB | **0.37x** |
| Deep nesting (1000) | 20.3 KB | 93.9 KB | 239.8 KB | **0.39x** |
| Many attrs (25/elem) | 21.6 KB | 69.5 KB | 40.0 KB | 1.74x |
| Many attrs (100/elem) | 83.8 KB | 258 KB | 142 KB | 1.82x |

### Scaling verification

With chunked parsing (8 KB chunks), memory stays flat regardless of document size:

| Elements | Doc Size | Rust Peak | C Peak |
|----------|----------|-----------|--------|
| 100 | 14 KB | 33 KB | 15 KB |
| 1,000 | 146 KB | 33 KB | 31 KB |
| 10,000 | 1.5 MB | 33 KB | 31 KB |
| 100,000 | 15 MB | 33 KB | 31 KB |
| 1,000,000 | 151 MB | 33 KB | 31 KB |

Both parsers achieve O(1) memory with respect to document size when streaming.

### Analysis

**Streaming mode (the common case)**: When parsing in chunks — the way expat is designed to be used — Rust and C are nearly identical. Both use ~33 KB regardless of total document size for flat/shallow documents. Memory does scale with nesting depth (each open element adds to the stack), but not with document length. The 7% overhead comes from Rust's `Vec` and `String` metadata for the small amount of in-flight parser state.

**One-shot mode**: When the entire document is passed at once, Rust uses 1.6-2.5x more memory than C. This is because:
- C's `STRING_POOL` arena allocator has zero per-allocation overhead
- Rust allocates individual `String`/`Vec` objects with allocator metadata (16-32 bytes each)
- C interns repeated attribute names; Rust reuses `String` capacity across elements but still allocates per-attribute

**Where Rust wins**: Small documents and deep nesting. Rust's `Parser::new()` allocates less upfront infrastructure (no hash tables, string pools, or arena blocks). Rust's `Vec`-based element stack is more compact than C's linked structures.

**No leaks**: Verified over 200 consecutive parse cycles with zero memory growth.

### The trade-off

The 1.6-2.5x overhead in one-shot mode comes from the same design choice as the speed trade-off: standard library types instead of hand-rolled arenas. In streaming mode (how expat is meant to be used), the difference nearly vanishes (7%).

For real-world usage: stream large documents in chunks (even 8 KB is enough) and memory stays at ~33 KB regardless of input size.

---

## Reproducing

```bash
# Run performance benchmarks (criterion)
cargo bench -p expat-rust

# Results are saved to target/criterion/ with HTML reports
# Open target/criterion/report/index.html for detailed analysis

# Run all memory tests
RUST_TEST_THREADS=1 cargo test -p expat-rust --test memory_usage_tests --release -- --nocapture

# Run individual memory tests
RUST_TEST_THREADS=1 cargo test -p expat-rust --test memory_usage_tests --release streaming_memory_bounded -- --nocapture
RUST_TEST_THREADS=1 cargo test -p expat-rust --test memory_usage_tests --release memory_usage_comparison -- --nocapture
RUST_TEST_THREADS=1 cargo test -p expat-rust --test memory_usage_tests --release memory_scales_linearly -- --nocapture
RUST_TEST_THREADS=1 cargo test -p expat-rust --test memory_usage_tests --release no_memory_leak -- --nocapture
```
