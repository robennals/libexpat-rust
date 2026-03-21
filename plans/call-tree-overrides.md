# Standard Call Tree Divergences

These are the allowed categories where Rust intentionally diverges from
the C call tree. Each is a global rule that applies to ALL functions.

## memory_management

**C calls omitted:** FREE, MALLOC, REALLOC, XML_MemFree, XML_MemMalloc, XML_MemRealloc, expat_free, expat_malloc, expat_realloc

**Reason:** Rust's ownership model handles allocation/deallocation via Vec, String, Box.

## string_pools

**C calls omitted:** poolAppend, poolAppendString, poolBytesToAllocateFor, poolChop, poolClear, poolCopyString, poolCopyStringN, poolCopyStringNoFinish, poolDestroy, poolDiscard, poolFinish, poolGrow, poolInit, poolLastString, poolLength, poolStart, poolStoreString

**Reason:** Rust uses String/Vec instead of C's arena-style string pool.

## hash_tables

**C calls omitted:** copy_salt_to_sipkey, hash, hashTableClear, hashTableDestroy, hashTableInit, hashTableIterInit, hashTableIterNext, keyeq, keylen, lookup

**Reason:** Rust uses std::collections::HashMap with built-in SipHash.

## entropy

**C calls omitted:** ENTROPY_DEBUG, gather_time_entropy, generate_hash_secret_salt, get_hash_secret_salt, writeRandomBytes_arc4random, writeRandomBytes_dev_urandom, writeRandomBytes_getrandom_nonblock, writeRandomBytes_rand_s

**Reason:** Rust's HashMap has built-in randomized hashing for DoS protection.

## dtd_lifecycle

**C calls omitted:** dtdCopy, dtdCreate, dtdDestroy, dtdReset

**Reason:** Rust uses struct Default/Clone/Drop traits for lifecycle management.

## error_handling_style

**C calls omitted:** parserBusy

**Reason:** Rust uses match on ParsingState enum instead of a separate check function.

