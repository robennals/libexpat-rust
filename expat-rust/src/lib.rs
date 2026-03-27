#![forbid(unsafe_code)]

//! A safe, idiomatic Rust port of the [libexpat](https://libexpat.github.io/) XML parser.
//!
//! This crate provides a streaming (SAX-style) XML parser with callback-based event handling.
//! The public API lives in [`xmlparse`]: create a [`xmlparse::Parser`], register handlers for
//! elements, character data, processing instructions, etc., then feed XML data incrementally
//! via [`xmlparse::Parser::parse`].
//!
//! Module structure follows the C dependency layers:
//!   Layer 0 (leaves): [`ascii`], [`char_tables`], [`nametab`], [`siphash`]
//!   Layer 1: [`xmltok_impl`], [`xmlrole`]
//!   Layer 2: [`xmltok`]
//!   Layer 3: [`xmlparse`] (public API)

pub mod ascii;
pub mod char_tables;
pub mod nametab;
pub mod siphash;
pub mod xmlparse;
pub mod xmlrole;
pub mod xmltok;
pub mod xmltok_impl;
