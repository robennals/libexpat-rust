use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let expat_dir = manifest_dir.join("..").join("expat");
    let tests_dir = expat_dir.join("tests");
    let lib_dir = expat_dir.join("lib");

    // We generate expat_config.h in OUT_DIR
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Write expat_config.h
    std::fs::write(
        out_dir.join("expat_config.h"),
        r#"
#ifndef EXPAT_CONFIG_H
#define EXPAT_CONFIG_H 1
#define BYTEORDER 1234
#define HAVE_ARC4RANDOM_BUF
#define HAVE_DLFCN_H
#define HAVE_FCNTL_H
#define HAVE_GETPAGESIZE
#define HAVE_INTTYPES_H
#define HAVE_MEMORY_H
#define HAVE_MMAP
#define HAVE_STDINT_H
#define HAVE_STDLIB_H
#define HAVE_STRINGS_H
#define HAVE_STRING_H
#define HAVE_SYS_STAT_H
#define HAVE_SYS_TYPES_H
#define HAVE_UNISTD_H
#define PACKAGE "expat"
#define PACKAGE_VERSION "2.7.5"
#define PACKAGE_STRING "expat 2.7.5"
#define PACKAGE_BUGREPORT "https://github.com/libexpat/libexpat/issues"
#define PACKAGE_NAME "expat"
#define PACKAGE_TARNAME "expat"
#define PACKAGE_URL ""
#ifndef STDC_HEADERS
#define STDC_HEADERS
#endif
#define XML_CONTEXT_BYTES 1024
#define XML_DEV_URANDOM
#define XML_DTD
#define XML_GE 1
#define XML_NS
#endif
"#,
    )
    .expect("Failed to write expat_config.h");

    // Compile the C test suite files
    // These will link against our Rust staticlib (which provides all XML_* symbols)
    cc::Build::new()
        // Test framework
        .file(tests_dir.join("minicheck.c"))
        .file(tests_dir.join("chardata.c"))
        .file(tests_dir.join("structdata.c"))
        .file(tests_dir.join("memcheck.c"))
        // Common test infrastructure
        .file(tests_dir.join("common.c"))
        .file(tests_dir.join("dummy.c"))
        .file(tests_dir.join("handlers.c"))
        // Test suites
        .file(tests_dir.join("basic_tests.c"))
        .file(tests_dir.join("ns_tests.c"))
        .file(tests_dir.join("misc_tests.c"))
        .file(tests_dir.join("alloc_tests.c"))
        .file(tests_dir.join("nsalloc_tests.c"))
        .file(tests_dir.join("acc_tests.c"))
        // Main - use our simplified runner
        .file(manifest_dir.join("runtests_basic_only.c"))
        // Include paths
        .include(&out_dir) // for expat_config.h
        .include(&lib_dir) // for expat.h, internal.h, expat_external.h
        .include(&tests_dir) // for test headers
        // Defines matching the expat build
        .define("HAVE_ARC4RANDOM_BUF", None)
        .define("XML_DEV_URANDOM", None)
        .define("XML_DTD", None)
        .define("XML_GE", "1")
        .define("XML_NS", None)
        .define("XML_CONTEXT_BYTES", "1024")
        .define("XML_TESTING", None)
        // Rename main so we can call it from Rust
        .define("main", "c_test_main")
        .std("c99")
        .warnings(false)
        .compile("expat_tests");
}
