// Force linking expat-ffi (lib name "expat") which provides all XML_* C ABI symbols
extern crate expat;

use std::ffi::{c_char, c_int, CString};

// The C test suite's main(), renamed via -Dmain=c_test_main in build.rs
extern "C" {
    fn c_test_main(argc: c_int, argv: *const *const c_char) -> c_int;
}

// Force the linker to include the C test library
#[link(name = "expat_tests", kind = "static")]
extern "C" {}

fn main() {
    let args: Vec<CString> = std::env::args().map(|a| CString::new(a).unwrap()).collect();
    let arg_ptrs: Vec<*const c_char> = args.iter().map(|a| a.as_ptr()).collect();

    let ret = unsafe { c_test_main(arg_ptrs.len() as c_int, arg_ptrs.as_ptr()) };

    std::process::exit(ret);
}
