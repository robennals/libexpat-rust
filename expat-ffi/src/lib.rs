//! C-compatible FFI layer for expat-rust.
//!
//! This crate exposes the Rust XML parser through the same C API as libexpat,
//! allowing it to serve as a drop-in replacement for `libexpat.so` / `libexpat.dylib`.
//!
//! Build with:
//! ```sh
//! cargo build --release -p expat-ffi
//! ```
//!
//! This produces `libexpat.so` (Linux), `libexpat.dylib` (macOS), or `expat.dll` (Windows).

#![allow(non_camel_case_types, non_snake_case, dead_code, private_interfaces)]

use expat_rust::xmlparse::{Parser, XmlError, XmlStatus};
use std::ffi::{c_char, c_int, c_ulong, c_void, CStr};
use std::ptr;

// --- Opaque parser handle ---

/// Opaque parser handle exposed to C. Wraps a boxed Rust Parser plus user data pointer.
struct ParserHandle {
    parser: Parser,
    user_data: *mut c_void,
    // Store C callback function pointers
    start_element_handler: XML_StartElementHandler,
    end_element_handler: XML_EndElementHandler,
    character_data_handler: XML_CharacterDataHandler,
    comment_handler: XML_CommentHandler,
}

type XML_Parser = *mut ParserHandle;

// --- C type aliases ---

type XML_Char = c_char;
type XML_Bool = c_char;
type XML_Status_t = c_int;
type XML_Error_t = c_int;

// --- C callback types ---

type XML_StartElementHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut *const XML_Char)>;
type XML_EndElementHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
type XML_CharacterDataHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
type XML_CommentHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;

// --- Status constants ---

const XML_STATUS_ERROR: XML_Status_t = 0;
const XML_STATUS_OK: XML_Status_t = 1;
const XML_STATUS_SUSPENDED: XML_Status_t = 2;

// --- Helper conversions ---

fn status_to_c(s: XmlStatus) -> XML_Status_t {
    match s {
        XmlStatus::Error => XML_STATUS_ERROR,
        XmlStatus::Ok => XML_STATUS_OK,
        XmlStatus::Suspended => XML_STATUS_SUSPENDED,
    }
}

fn error_to_c(e: XmlError) -> XML_Error_t {
    e as XML_Error_t
}

// --- Parser lifecycle ---

/// Create a new XML parser.
///
/// Equivalent to `XML_ParserCreate()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser {
    let enc = if encoding.is_null() {
        None
    } else {
        CStr::from_ptr(encoding).to_str().ok()
    };

    match Parser::new(enc) {
        Some(parser) => {
            let handle = Box::new(ParserHandle {
                parser,
                user_data: ptr::null_mut(),
                start_element_handler: None,
                end_element_handler: None,
                character_data_handler: None,
                comment_handler: None,
            });
            Box::into_raw(handle)
        }
        None => ptr::null_mut(),
    }
}

/// Create a new namespace-aware XML parser.
///
/// Equivalent to `XML_ParserCreateNS()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreateNS(
    encoding: *const XML_Char,
    separator: XML_Char,
) -> XML_Parser {
    let enc = if encoding.is_null() {
        None
    } else {
        CStr::from_ptr(encoding).to_str().ok()
    };

    match Parser::new_ns(enc, separator as u8 as char) {
        Some(parser) => {
            let handle = Box::new(ParserHandle {
                parser,
                user_data: ptr::null_mut(),
                start_element_handler: None,
                end_element_handler: None,
                character_data_handler: None,
                comment_handler: None,
            });
            Box::into_raw(handle)
        }
        None => ptr::null_mut(),
    }
}

/// Free a parser.
///
/// Equivalent to `XML_ParserFree()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_ParserFree(parser: XML_Parser) {
    if !parser.is_null() {
        drop(Box::from_raw(parser));
    }
}

// --- Parsing ---

/// Parse a chunk of XML data.
///
/// Equivalent to `XML_Parse()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_Parse(
    parser: XML_Parser,
    s: *const c_char,
    len: c_int,
    is_final: c_int,
) -> XML_Status_t {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let handle = &mut *parser;
    let data = if s.is_null() || len <= 0 {
        &[]
    } else {
        std::slice::from_raw_parts(s as *const u8, len as usize)
    };
    let status = handle.parser.parse(data, is_final != 0);
    status_to_c(status)
}

// --- Error handling ---

/// Get the error code from the last failed parse.
///
/// Equivalent to `XML_GetErrorCode()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error_t {
    if parser.is_null() {
        return 0;
    }
    let handle = &*parser;
    error_to_c(handle.parser.error_code())
}

/// Get a human-readable error string.
///
/// Equivalent to `XML_ErrorString()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_ErrorString(code: XML_Error_t) -> *const XML_Char {
    // Convert integer to XmlError
    let error = match code {
        0 => XmlError::None,
        1 => XmlError::NoMemory,
        2 => XmlError::Syntax,
        3 => XmlError::NoElements,
        4 => XmlError::InvalidToken,
        5 => XmlError::UnclosedToken,
        6 => XmlError::PartialChar,
        7 => XmlError::TagMismatch,
        _ => XmlError::None,
    };
    let msg = expat_rust::xmlparse::error_string(error);
    msg.as_ptr() as *const XML_Char
}

// --- Position info ---

/// Get the current line number (1-based).
///
/// Equivalent to `XML_GetCurrentLineNumber()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentLineNumber(parser: XML_Parser) -> c_ulong {
    if parser.is_null() {
        return 0;
    }
    let handle = &*parser;
    handle.parser.current_line_number() as c_ulong
}

/// Get the current column number (0-based).
///
/// Equivalent to `XML_GetCurrentColumnNumber()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> c_ulong {
    if parser.is_null() {
        return 0;
    }
    let handle = &*parser;
    handle.parser.current_column_number() as c_ulong
}

// --- User data ---

/// Set user data pointer passed to all callbacks.
///
/// Equivalent to `XML_SetUserData()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_SetUserData(parser: XML_Parser, user_data: *mut c_void) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.user_data = user_data;
}

// --- Handler setters ---

/// Set element handlers.
///
/// Equivalent to `XML_SetElementHandler()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_SetElementHandler(
    parser: XML_Parser,
    start: XML_StartElementHandler,
    end: XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.start_element_handler = start;
    handle.end_element_handler = end;

    // Wire up Rust closures that call the C callbacks
    if let Some(start_fn) = start {
        let user_data = handle.user_data;
        handle
            .parser
            .set_start_element_handler(Some(Box::new(move |name, attrs| {
                // Convert name to C string
                let mut name_bytes: Vec<u8> = name.as_bytes().to_vec();
                name_bytes.push(0);

                // Convert attrs to null-terminated C string array
                let mut c_attrs: Vec<*const XML_Char> = Vec::new();
                for (k, v) in attrs {
                    let mut kb: Vec<u8> = k.as_bytes().to_vec();
                    kb.push(0);
                    let mut vb: Vec<u8> = v.as_bytes().to_vec();
                    vb.push(0);
                    c_attrs.push(kb.as_ptr() as *const XML_Char);
                    c_attrs.push(vb.as_ptr() as *const XML_Char);
                    // Leak to keep valid for duration of callback
                    std::mem::forget(kb);
                    std::mem::forget(vb);
                }
                c_attrs.push(ptr::null());

                start_fn(
                    user_data,
                    name_bytes.as_ptr() as *const XML_Char,
                    c_attrs.as_mut_ptr(),
                );
            })));
    } else {
        handle.parser.set_start_element_handler(None);
    }

    if let Some(end_fn) = end {
        let user_data = handle.user_data;
        handle
            .parser
            .set_end_element_handler(Some(Box::new(move |name| {
                let mut name_bytes: Vec<u8> = name.as_bytes().to_vec();
                name_bytes.push(0);
                end_fn(user_data, name_bytes.as_ptr() as *const XML_Char);
            })));
    } else {
        handle.parser.set_end_element_handler(None);
    }
}

/// Set character data handler.
///
/// Equivalent to `XML_SetCharacterDataHandler()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    parser: XML_Parser,
    handler: XML_CharacterDataHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.character_data_handler = handler;

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_character_data_handler(Some(Box::new(move |data| {
                handler_fn(user_data, data.as_ptr() as *const XML_Char, data.len() as c_int);
            })));
    } else {
        handle.parser.set_character_data_handler(None);
    }
}

/// Set comment handler.
///
/// Equivalent to `XML_SetCommentHandler()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_SetCommentHandler(
    parser: XML_Parser,
    handler: XML_CommentHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.comment_handler = handler;

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_comment_handler(Some(Box::new(move |data| {
                let mut bytes: Vec<u8> = data.to_vec();
                bytes.push(0);
                handler_fn(user_data, bytes.as_ptr() as *const XML_Char);
            })));
    } else {
        handle.parser.set_comment_handler(None);
    }
}

// --- Version ---

static VERSION_STRING: &[u8] = b"expat-rust_0.1.0\0";

/// Get the expat-rust version string.
///
/// Equivalent to `XML_ExpatVersion()` in libexpat.
#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersion() -> *const XML_Char {
    VERSION_STRING.as_ptr() as *const XML_Char
}
