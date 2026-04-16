# Performance Analysis

Where time goes in expat-rust, why C is faster on element-heavy documents, and why the gap is a structural cost of safe Rust rather than an optimization opportunity.

## Profiling methodology

We profiled the medium document benchmark (200 elements, 4 attributes each, ~10 KB) using the macOS `sample` tool at 1 ms intervals over 1,203 samples. The medium document is representative of the workloads where C wins — element-heavy XML with repeated attribute extraction. The profile was taken on Apple Silicon with LTO and `codegen-units=1`.

## Where time goes

| Category | Samples | % of total | What it does |
|----------|---------|-----------|--------------|
| **Tokenizer** (content_tok, scan_atts, scan_attr_value) | 240 | 20% | Lexes XML bytes into tokens — the irreducible parsing work |
| **malloc/free** (_xzm_free, _xzm_xzone_malloc_tiny, etc.) | 218 | 18% | Heap allocation and deallocation for Strings and Vecs |
| **Attribute extraction** (extract_attrs, normalize_attribute_value) | 167 | 14% | Builds (String, String) pairs for each attribute |
| **Parser overhead** (main loop, parser create/drop, buffer clone) | 189 | 16% | Per-parse setup, processor dispatch, position tracking |
| **UTF-8 validation** (from_utf8) | 81 | 7% | Validates that byte slices are valid UTF-8 on every &str conversion |
| **Content dispatch** (do_content match arms) | 82 | 7% | Dispatches tokens to handlers, manages element stack |
| **Other** (memcpy, memcmp, mach_absolute_time) | 226 | 18% | Memory operations, allocator timestamps, misc |

### Allocator breakdown

The 18% in malloc/free breaks down further:

- **Attribute-related**: Each element with attributes builds a `Vec<(&str, &str)>` for the handler callback, then drops it. Over 200 elements x 4 attributes, this is 200 small heap allocations and 200 frees.
- **Tag stack**: Each start element pushes a `String` onto the tag stack; each end element pops and frees it. Over 400 elements (200 start + 200 inner), this is 400 String allocations and 400 frees.
- **Attribute normalization**: The first element allocates fresh Strings for attribute values. Subsequent elements reuse capacity via the `attr_buf` buffer, so this cost is amortized.

### Comparison with C

C's libexpat avoids nearly all of this allocation overhead through its `STRING_POOL` arena allocator: it allocates large blocks upfront and carves out strings by bumping a pointer. Individual strings are never allocated or freed — the entire arena is released when the parser is destroyed. This is effectively O(1) per-string cost vs Rust's O(1) amortized but with constant-factor overhead from allocator metadata, fragmentation, and free-list management.

## Why this overhead is structural

### 1. UTF-8 validation (~7% of time)

Every call to `std::str::from_utf8()` scans the byte slice to confirm it's valid UTF-8. This happens on every element name, every attribute name, and every attribute value — hundreds of times per document.

The data is already valid UTF-8: we transcoded all input to UTF-8 before processing, and the tokenizer only produces valid byte ranges. The validation is redundant but **required by Rust's type system** — there is no safe way to construct a `&str` from `&[u8]` without validation. The only alternative is `std::str::from_utf8_unchecked`, which requires `unsafe`.

With `#![forbid(unsafe_code)]`, this cost is permanent. It is the price of Rust's guarantee that every `&str` is valid UTF-8.

### 2. Per-allocation heap metadata (~18% of time)

Rust's `String` and `Vec` types store three words each (pointer, length, capacity) and go through the system allocator for every allocation. C's arena allocator stores no per-string metadata — a string is just a pointer into the arena block.

The `attr_buf` reuse pattern mitigates this by reusing `String` heap capacity across elements. But several allocations remain unavoidable:

- **`Vec<(&str, &str)>` for handler callbacks**: The start element handler signature is `FnMut(&str, &[(&str, &str)])`. We must build a `&[(&str, &str)]` slice, which requires a temporary `Vec` because `Vec<(String, String)>` and `&[(&str, &str)]` have different memory layouts. This Vec is allocated and freed on every element. A reusable buffer is impossible because the `&str` references have local lifetimes that can't be stored across calls.

- **Tag stack strings**: Each open element pushes its name as a `String` onto the tag stack for end-tag matching. The String is freed when the element closes. (A string-pool approach can eliminate this, but only improves the deep-nesting case that Rust already wins.)

These allocations exist because Rust's ownership model requires each piece of data to have a clear owner. C's arena model — where strings are just pointers into a shared block with no individual ownership — has no safe Rust equivalent without `unsafe` or a third-party arena crate, and even arena crates introduce lifetime complexity that conflicts with the callback-based API.

### 3. Handler API constraint

The SAX callback model requires passing attribute data to user closures during parsing. In C, the handler receives a `char**` array pointing directly into the parser's internal string pool — zero-copy, zero-allocation. In Rust, the handler receives `&[(&str, &str)]`, which must be constructed from the internal `Vec<(String, String)>`. This construction allocates a new `Vec` per element.

Changing the handler signature to avoid this allocation would break API compatibility with the C callback model, which is a core design goal.

## What we tried

| Optimization | Result | Why |
|-------------|--------|-----|
| `bumpalo` arena for per-element temporaries | No measurable improvement on element-heavy benchmarks | The attr_buf reuse pattern already covers most allocations; bumpalo only helped the attr_refs Vec, which is a small allocation |
| Eliminate `parse_data` clone | Saves ~2ms on 100MB (0.5%) | On Apple Silicon, 100MB memcpy is ~2ms vs ~440ms total parse time |
| Tag name string pool | 14% improvement on deep nesting (already Rust-wins case) | Eliminates per-element String alloc/dealloc for tag stack, but doesn't help element-heavy benchmarks where most time is in attr extraction |
| In-place `normalize_attribute_value_into` | No measurable improvement | The fast path (no entity refs) was already efficient; the slow path is rarely hit in benchmarks |
| String interning | Not attempted | Would help with repeated attribute names but adds HashMap lookup overhead; C's interning is cheap because it uses arena-allocated hash tables |

## Conclusion

The 1.6x gap on element-heavy documents is the cost of three properties that define expat-rust:

1. **`#![forbid(unsafe_code)]`** — forces UTF-8 validation on every string conversion (~7% overhead)
2. **Rust's ownership model** — requires per-allocation heap metadata and prevents C-style arena sharing across callback boundaries (~18% overhead)
3. **API compatibility** — the SAX callback model requires constructing temporary slices for handler invocation

These are not incidental costs that can be optimized away. They are the direct consequences of memory safety, type safety, and behavioral compatibility — the three goals that justify the project's existence.

For context: Rust processes 100 MB of XML in ~440ms (vs C's ~278ms). In streaming mode (how expat is designed to be used), the gap narrows to ~28%. For applications that create many short-lived parsers or parse deeply nested documents, Rust is 1.9-3.0x *faster* than C.
