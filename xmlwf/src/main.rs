extern crate expat;

use std::ffi::{c_char, c_int, CString};

extern "C" {
    fn c_xmlwf_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

#[link(name = "xmlwf_c", kind = "static")]
extern "C" {}

fn main() {
    let args: Vec<CString> = std::env::args().map(|a| CString::new(a).unwrap()).collect();
    let mut arg_ptrs: Vec<*mut c_char> =
        args.iter().map(|a| a.as_ptr() as *mut c_char).collect();

    let ret =
        unsafe { c_xmlwf_main(arg_ptrs.len() as c_int, arg_ptrs.as_mut_ptr()) };

    std::process::exit(ret);
}
