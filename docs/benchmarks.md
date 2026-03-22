# Benchmark Results

Performance comparison between `expat-rust` and C libexpat 2.7.5.

## Environment

- **Platform**: macOS (Apple Silicon)
- **Rust**: stable (release profile, optimized)
- **C**: libexpat 2.7.5 compiled via `cc` crate with `-O2`
- **Tool**: [criterion.rs](https://github.com/bheisler/criterion.rs) with 100 samples per benchmark
- **Methodology**: Both parsers register start-element, end-element, and character-data handlers (no-op closures / C callbacks) to force real parsing work. Each iteration creates a fresh parser.

## Results

| Scenario | expat-rust | libexpat (C) | Ratio | Notes |
|----------|-----------|-------------|-------|-------|
| Small document (44 B) | 1.25 us | 2.48 us | **0.50x** | Rust wins — lower per-parser overhead |
| Medium document (~10 KB) | 171 us | 79 us | 2.16x | C wins — arena allocator advantage |
| Large document (~100 KB) | 1.65 ms | 898 us | 1.84x | C wins — consistent with medium |
| Deep nesting (100 levels) | 7.13 us | 20.8 us | **0.34x** | Rust wins — efficient stack/Vec handling |
| Many attributes (25/elem) | 35.1 us | 17.9 us | 1.96x | C wins — interned strings vs String clones |
| Error detection (malformed) | 416 ns | 986 ns | **0.42x** | Rust wins — fast early-exit paths |

## Analysis

### Where Rust is faster (2-3x)

**Small documents and parser creation**: Rust's `Parser::new()` is lighter than C's `XML_ParserCreate()` because it doesn't need to set up a memory allocator, string pool, or hash table infrastructure. For applications that create many short-lived parsers, this is a significant advantage.

**Deep nesting**: Rust's `Vec`-based element stack is more cache-friendly than C's linked-list-style structure. Push/pop operations on a contiguous vector are cheaper than pointer chasing.

**Error detection**: Rust's pattern matching and early-return paths in the tokenizer are slightly more efficient than C's switch/goto error handling.

### Where C is faster (~2x)

**Medium and large documents**: C's `STRING_POOL` arena allocator amortizes allocation costs by allocating large blocks and carving out strings from them. Rust's `String` and `Vec` allocate individually for each element name, attribute name/value, and text chunk. For documents with thousands of elements, this adds up.

**Attribute-heavy documents**: C interns attribute names in a hash table with pool-allocated strings. Rust creates new `String` objects for each attribute, which involves allocation and copying.

### The trade-off

The ~2x gap on larger documents is a deliberate design choice. Replacing C's arena allocator and interning hash table with Rust's standard library types was essential for:

1. **Memory safety**: Arena allocators in Rust would require `unsafe` or complex lifetime annotations
2. **Simplicity**: Standard types are well-tested and easy to reason about
3. **Correctness**: Simpler code means fewer bugs

For most real-world applications, even the "slower" Rust path processes 100KB of XML in 1.6ms — well under any perceptible threshold.

### Future optimization opportunities

If the performance gap needs to be closed:

- **Arena allocator**: A safe arena crate (e.g., `bumpalo`) could replace individual `String` allocations
- **String interning**: Commonly-repeated element/attribute names could be interned
- **Buffer reuse**: Parser reset could reuse allocated buffers instead of dropping them

These optimizations would likely bring Rust within 10-20% of C on all benchmarks while maintaining zero `unsafe`.

## Reproducing

```bash
# Run all benchmarks
cargo bench -p expat-rust

# Results are saved to target/criterion/ with HTML reports
# Open target/criterion/report/index.html for detailed analysis
```
