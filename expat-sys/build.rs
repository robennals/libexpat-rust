use std::env;
use std::path::PathBuf;

fn main() {
    // The libexpat submodule is at ../expat, with source under the expat/ subdirectory
    let expat_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("..")
        .join("expat")
        .join("expat");
    let build_dir = expat_dir.join("build");

    // Build expat from source using cc
    cc::Build::new()
        .file(expat_dir.join("lib/xmlparse.c"))
        .file(expat_dir.join("lib/xmlrole.c"))
        .file(expat_dir.join("lib/xmltok.c"))
        .include(expat_dir.join("lib"))
        .include(&build_dir) // for expat_config.h
        .define("XML_ENABLE_VISIBILITY", "1")
        .define("HAVE_ARC4RANDOM_BUF", None)
        .define("XML_DEV_URANDOM", None)
        .define("XML_DTD", None)
        .define("XML_GE", "1")
        .define("XML_NS", None)
        .define("XML_CONTEXT_BYTES", "1024")
        .std("c99")
        .warnings(false)
        .compile("expat");
}
