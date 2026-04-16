use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let expat_dir = manifest_dir.join("..").join("expat").join("expat");
    let xmlwf_dir = expat_dir.join("xmlwf");
    let lib_dir = expat_dir.join("lib");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

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

    cc::Build::new()
        .file(xmlwf_dir.join("xmlwf.c"))
        .file(xmlwf_dir.join("xmlfile.c"))
        .file(xmlwf_dir.join("unixfilemap.c"))
        .file(xmlwf_dir.join("codepage.c"))
        .file(xmlwf_dir.join("xmlmime.c"))
        .include(&out_dir)
        .include(&lib_dir)
        .include(&xmlwf_dir)
        .define("HAVE_ARC4RANDOM_BUF", None)
        .define("XML_DEV_URANDOM", None)
        .define("XML_DTD", None)
        .define("XML_GE", "1")
        .define("XML_NS", None)
        .define("XML_CONTEXT_BYTES", "1024")
        .define("main", "c_xmlwf_main")
        .std("c99")
        .warnings(false)
        .compile("xmlwf_c");
}
