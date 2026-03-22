// Force linking the library crate which provides all XML_* C ABI symbols
extern crate c_tests_runner;

use std::ffi::{c_char, c_int, CString};

extern "C" {
    fn c_test_main(argc: c_int, argv: *const *const c_char) -> c_int;
}

fn main() {
    // Set panic hook to print nicely and abort instead of unwinding
    std::panic::set_hook(Box::new(|info| {
        eprintln!("RUST PANIC: {}", info);
    }));

    let args: Vec<CString> = std::env::args()
        .map(|a| CString::new(a).unwrap())
        .collect();
    let arg_ptrs: Vec<*const c_char> = args.iter().map(|a| a.as_ptr()).collect();

    let ret = unsafe { c_test_main(arg_ptrs.len() as c_int, arg_ptrs.as_ptr()) };

    std::process::exit(ret);
}
