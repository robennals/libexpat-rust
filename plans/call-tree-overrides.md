# Call Tree Override Documentation

These are cases where the Rust port intentionally diverges from the C call tree.
Each override has a documented reason.

## hash_and_lookup

**C functions:** hash, lookup, keyeq, keylen, copy_salt_to_sipkey

**Rust replacement:** std::collections::HashMap

**Reason:** Rust's standard HashMap replaces the custom hash table implementation. The SipHash implementation is preserved for compatibility but HashMap provides equivalent functionality with better safety guarantees.

## memory_management

**C functions:** expat_malloc, expat_free, expat_realloc, XML_MemMalloc, XML_MemRealloc, XML_MemFree

**Rust replacement:** Rust's ownership system and standard allocator

**Reason:** Rust's ownership model eliminates the need for manual memory management. Vec, String, Box etc. handle allocation and deallocation automatically.

## string_pool

**C functions:** poolInit, poolClear, poolDestroy, poolCopyString, poolStoreString, poolAppend, poolAppendString, poolGrow, poolCopyStringN, poolCopyStringNoFinish, poolBytesToAllocateFor

**Rust replacement:** String and Vec<u8>

**Reason:** The C string pool is a memory optimization for arena-style allocation. Rust's String and Vec provide the same functionality with automatic memory management. No performance-critical path requires arena allocation.

## random_entropy

**C functions:** generate_hash_secret_salt, writeRandomBytes_getrandom_nonblock, writeRandomBytes_dev_urandom, writeRandomBytes_arc4random, writeRandomBytes_rand_s, gather_time_entropy

**Rust replacement:** std::collections::hash_map::RandomState or rand crate

**Reason:** Rust's HashMap uses randomized hashing by default, providing equivalent DoS protection without manual entropy gathering.

## dtd_lifetime

**C functions:** dtdCreate, dtdReset, dtdDestroy, dtdCopy

**Rust replacement:** DTD struct with Default, Clone, Drop traits

**Reason:** Rust's ownership and trait system replaces manual DTD lifecycle management.

