// expat-rust: Safe, idiomatic Rust port of libexpat XML parser
//
// Module structure follows the C dependency layers:
//   Layer 0 (leaves): ascii, char_tables, nametab, siphash
//   Layer 1: xmltok_impl, xmlrole
//   Layer 2: xmltok
//   Layer 3: xmlparse (public API)

pub mod ascii;
pub mod char_tables;
pub mod nametab;
pub mod siphash;
pub mod xmlrole;
pub mod xmltok_impl;
pub mod xmltok;
pub mod xmlparse;
