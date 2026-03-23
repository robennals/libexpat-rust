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
//!
//! ## Implementation notes
//!
//! - **`XML_GetCurrentByteIndex`**: Returns byte offsets in the original input encoding,
//!   matching C libexpat behavior. For non-UTF-8 input, this requires a re-scan of the
//!   current parse chunk (O(chunk_size)) to convert internal UTF-8 positions back to
//!   original byte offsets. This only happens when the function is actually called.
//!   For UTF-8 input (>99% of real-world XML), the conversion is O(1).

#![allow(non_camel_case_types, non_snake_case, dead_code, private_interfaces)]

use expat_rust::xmlparse::{self, ParamEntityParsing, Parser, ParsingState, XmlError, XmlStatus};
use std::ffi::{c_char, c_int, c_long, c_ulong, c_void, CStr};
use std::ptr;

// --- Opaque parser handle ---

/// Opaque parser handle exposed to C. Wraps a boxed Rust Parser plus user data pointer.
///
/// CRITICAL: user_data MUST be the first field because the C macro
/// `XML_GetUserData(parser)` is defined as `(*(void **)(parser))`,
/// which reads the first word of the struct directly.
#[repr(C)]
struct ParserHandle {
    user_data: *mut c_void,
    parser: Parser,
    /// When true, handler_arg is the parser pointer itself (for XML_UseParserAsHandlerArg)
    use_parser_as_handler_arg: bool,
    /// Stored base URI as a null-terminated C string (for XML_GetBase)
    base_c_string: Option<Vec<u8>>,
    /// External entity ref handler arg override (NULL = use parser pointer)
    ext_entity_ref_handler_arg: *mut c_void,
    /// Stored C handler function pointers (needed for handler inheritance in ext entity parsers)
    c_ext_entity_ref_handler: XML_ExternalEntityRefHandler,
    c_not_standalone_handler: XML_NotStandaloneHandler,
    c_skipped_entity_handler: XML_SkippedEntityHandler,
    c_element_decl_handler: XML_ElementDeclHandler,
    c_attlist_decl_handler: XML_AttlistDeclHandler,
    c_entity_decl_handler: XML_EntityDeclHandler,
    c_unparsed_entity_decl_handler: XML_UnparsedEntityDeclHandler,
    c_notation_decl_handler: XML_NotationDeclHandler,
    c_start_ns_handler: XML_StartNamespaceDeclHandler,
    c_end_ns_handler: XML_EndNamespaceDeclHandler,
    c_unknown_encoding_handler: XML_UnknownEncodingHandler,
    c_unknown_encoding_data: *mut c_void,
    // Content handlers (needed for ext entity parser inheritance)
    c_start_element_handler: XML_StartElementHandler,
    c_end_element_handler: XML_EndElementHandler,
    c_character_data_handler: XML_CharacterDataHandler,
    c_processing_instruction_handler: XML_ProcessingInstructionHandler,
    c_comment_handler: XML_CommentHandler,
    c_default_handler: XML_DefaultHandler,
    c_start_cdata_handler: XML_StartCdataSectionHandler,
    c_end_cdata_handler: XML_EndCdataSectionHandler,
    c_xml_decl_handler: XML_XmlDeclHandler,
    c_start_doctype_handler: XML_StartDoctypeDeclHandler,
    c_end_doctype_handler: XML_EndDoctypeDeclHandler,
    /// Last built content model as a C-allocated array (for XML_FreeContentModel)
    last_content_model_array: Option<Box<[XML_Content]>>,
    /// Name buffers for the content model (kept alive with the array)
    last_content_model_names: Vec<Vec<u8>>,
    /// Custom encoding map from unknown encoding handler
    custom_encoding_map: Option<Box<[i32; 256]>>,
    /// Custom encoding converter function from unknown encoding handler
    custom_encoding_converter: Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int>,
    /// User data for custom encoding converter
    custom_encoding_data: *mut c_void,
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
type XML_CharacterDataHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char)>;
type XML_CommentHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
type XML_StartCdataSectionHandler = Option<unsafe extern "C" fn(*mut c_void)>;
type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(*mut c_void)>;
type XML_DefaultHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, *const XML_Char, c_int),
>;
type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(*mut c_void)>;
type XML_XmlDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, c_int)>;
type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> c_int,
>;
type XML_NotStandaloneHandler = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
type XML_SkippedEntityHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
type XML_ElementDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut XML_Content)>;
type XML_AttlistDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        c_int,
    ),
>;
type XML_EntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        c_int,
        *const XML_Char,
        c_int,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ),
>;
type XML_UnparsedEntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ),
>;
type XML_NotationDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ),
>;
type XML_StartNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char)>;
type XML_EndNamespaceDeclHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
type XML_UnknownEncodingHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut XML_Encoding) -> c_int>;

// --- C structures ---

#[repr(C)]
pub struct XML_Content {
    pub type_: c_int,
    pub quant: c_int,
    pub name: *mut XML_Char,
    pub numchildren: c_int,
    pub children: *mut XML_Content,
}

#[repr(C)]
pub struct XML_Encoding {
    pub map: [c_int; 256],
    pub data: *mut c_void,
    pub convert: Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct XML_Memory_Handling_Suite {
    pub malloc_fcn: Option<unsafe extern "C" fn(usize) -> *mut c_void>,
    pub realloc_fcn: Option<unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void>,
    pub free_fcn: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct XML_Expat_Version {
    pub major: c_int,
    pub minor: c_int,
    pub micro: c_int,
}

#[repr(C)]
pub struct XML_Feature {
    pub feature: c_int,
    pub name: *const XML_Char,
    pub value: c_long,
}
unsafe impl Sync for XML_Feature {}

// --- Parsing status ---

#[repr(C)]
pub struct XML_ParsingStatus {
    pub parsing: c_int,
    pub finalBuffer: XML_Bool,
}

const XML_INITIALIZED: c_int = 0;
const XML_PARSING: c_int = 1;
const XML_FINISHED: c_int = 2;
const XML_SUSPENDED: c_int = 3;

// --- Parameter entity parsing ---

type XML_ParamEntityParsing = c_int;
const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;
const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;
const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;

// --- Status constants ---

const XML_STATUS_ERROR: XML_Status_t = 0;
const XML_STATUS_OK: XML_Status_t = 1;
const XML_STATUS_SUSPENDED: XML_Status_t = 2;

const XML_TRUE: XML_Bool = 1;
const XML_FALSE: XML_Bool = 0;

// --- Helper conversions ---

/// Check if an ASCII byte has XML significance (not BT_OTHER/BT_NONXML in C terms)
/// Matches the check in C's XmlInitUnknownEncoding
fn is_significant_ascii(b: u8) -> bool {
    // XML-significant characters: letters, digits, whitespace, punctuation used in XML
    matches!(b,
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' |
        b' ' | b'\t' | b'\n' | b'\r' |
        b'<' | b'>' | b'&' | b'\'' | b'"' | b'=' | b'/' | b'?' | b'!' |
        b'-' | b'.' | b'_' | b':' | b';' | b'[' | b']' | b'#' | b'%'
    )
}

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

fn new_handle(parser: Parser) -> XML_Parser {
    let handle = Box::new(ParserHandle {
        user_data: ptr::null_mut(),
        parser,
        use_parser_as_handler_arg: false,
        base_c_string: None,
        ext_entity_ref_handler_arg: ptr::null_mut(),
        c_ext_entity_ref_handler: None,
        c_not_standalone_handler: None,
        c_skipped_entity_handler: None,
        c_element_decl_handler: None,
        c_attlist_decl_handler: None,
        c_entity_decl_handler: None,
        c_unparsed_entity_decl_handler: None,
        c_notation_decl_handler: None,
        c_start_ns_handler: None,
        c_end_ns_handler: None,
        c_unknown_encoding_handler: None,
        c_unknown_encoding_data: ptr::null_mut(),
        c_start_element_handler: None,
        c_end_element_handler: None,
        c_character_data_handler: None,
        c_processing_instruction_handler: None,
        c_comment_handler: None,
        c_default_handler: None,
        c_start_cdata_handler: None,
        c_end_cdata_handler: None,
        c_xml_decl_handler: None,
        c_start_doctype_handler: None,
        c_end_doctype_handler: None,
        last_content_model_array: None,
        last_content_model_names: Vec::new(),
        custom_encoding_map: None,
        custom_encoding_converter: None,
        custom_encoding_data: ptr::null_mut(),
    });
    Box::into_raw(handle)
}

// ============================================================================
// Parser lifecycle
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser {
    let enc = if encoding.is_null() {
        None
    } else {
        CStr::from_ptr(encoding).to_str().ok()
    };
    match Parser::new(enc) {
        Some(parser) => new_handle(parser),
        None => ptr::null_mut(),
    }
}

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
        Some(parser) => new_handle(parser),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserReset(
    parser: XML_Parser,
    encoding: *const XML_Char,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    let handle = &mut *parser;
    let enc = if encoding.is_null() {
        None
    } else {
        CStr::from_ptr(encoding).to_str().ok()
    };
    if handle.parser.reset(enc) {
        handle.user_data = ptr::null_mut();
        handle.use_parser_as_handler_arg = false;
        handle.base_c_string = None;
        XML_TRUE
    } else {
        XML_FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserFree(parser: XML_Parser) {
    if !parser.is_null() {
        drop(Box::from_raw(parser));
    }
}

// ============================================================================
// Parsing
// ============================================================================

/// Transcode data from custom encoding to UTF-8 using the encoding map and converter.
/// Returns Ok(transcoded_data) or Err(error_code) if transcoding fails.
fn transcode_custom_encoding(
    data: &[u8],
    map: &[i32; 256],
    converter: Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int>,
    conv_data: *mut c_void,
) -> Result<Vec<u8>, i32> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let byte = data[i] as u8;
        let map_val = map[byte as usize];

        if map_val == -1 {
            // Malformed byte — skip it but continue
            i += 1;
            continue;
        } else if map_val >= 0 {
            // Single-byte mapping to Unicode codepoint
            let codepoint = map_val as u32;
            // Encode codepoint to UTF-8
            if codepoint <= 0x7F {
                result.push(codepoint as u8);
            } else if codepoint <= 0x7FF {
                result.push(0xC0 | ((codepoint >> 6) as u8));
                result.push(0x80 | ((codepoint & 0x3F) as u8));
            } else if codepoint <= 0xFFFF {
                result.push(0xE0 | ((codepoint >> 12) as u8));
                result.push(0x80 | (((codepoint >> 6) & 0x3F) as u8));
                result.push(0x80 | ((codepoint & 0x3F) as u8));
            } else {
                result.push(0xF0 | ((codepoint >> 18) as u8));
                result.push(0x80 | (((codepoint >> 12) & 0x3F) as u8));
                result.push(0x80 | (((codepoint >> 6) & 0x3F) as u8));
                result.push(0x80 | ((codepoint & 0x3F) as u8));
            }
            i += 1;
        } else if map_val < -4 {
            // Invalid multi-byte length indicator
            return Err(-1);
        } else {
            // Multi-byte sequence: map_val in [-4, -2]
            let n_bytes = (-map_val) as usize;
            if i + n_bytes > data.len() {
                // Not enough bytes
                break;
            }

            if let Some(conv_fn) = converter {
                // Build a buffer for the converter: first byte + remaining bytes
                let mut conv_buf = [0u8; 4];
                for j in 0..n_bytes {
                    conv_buf[j] = data[i + j];
                }

                let codepoint = unsafe { conv_fn(conv_data, conv_buf.as_ptr() as *const c_char) };

                if codepoint < 0 {
                    // Converter failed
                    return Err(codepoint as i32);
                }

                // Encode codepoint to UTF-8
                let codepoint = codepoint as u32;
                if codepoint <= 0x7F {
                    result.push(codepoint as u8);
                } else if codepoint <= 0x7FF {
                    result.push(0xC0 | ((codepoint >> 6) as u8));
                    result.push(0x80 | ((codepoint & 0x3F) as u8));
                } else if codepoint <= 0xFFFF {
                    result.push(0xE0 | ((codepoint >> 12) as u8));
                    result.push(0x80 | (((codepoint >> 6) & 0x3F) as u8));
                    result.push(0x80 | ((codepoint & 0x3F) as u8));
                } else {
                    result.push(0xF0 | ((codepoint >> 18) as u8));
                    result.push(0x80 | (((codepoint >> 12) & 0x3F) as u8));
                    result.push(0x80 | (((codepoint >> 6) & 0x3F) as u8));
                    result.push(0x80 | ((codepoint & 0x3F) as u8));
                }
                i += n_bytes;
            } else {
                // No converter but multi-byte needed — this shouldn't happen after validation
                return Err(-1);
            }
        }
    }

    Ok(result)
}

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
    // Negative len is an error (XML_ERROR_INVALID_ARGUMENT)
    if len < 0 {
        handle.parser.set_error(XmlError::InvalidArgument);
        return XML_STATUS_ERROR;
    }
    let data = if s.is_null() || len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(s as *const u8, len as usize)
    };

    // If custom encoding is pre-set (from a previous parse call), transcode the data
    let transcoded: Vec<u8>;
    let parse_data = if let Some(ref map) = handle.custom_encoding_map {
        match transcode_custom_encoding(data, map, handle.custom_encoding_converter, handle.custom_encoding_data) {
            Ok(t) => {
                transcoded = t;
                transcoded.as_slice()
            }
            Err(_err_code) => {
                // Transcoding failed — treat as a parse error
                handle.parser.set_error(XmlError::InvalidToken);
                return XML_STATUS_ERROR;
            }
        }
    } else {
        data
    };

    let status = handle.parser.parse(parse_data, is_final != 0);
    status_to_c(status)
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBuffer(parser: XML_Parser, len: c_int) -> *mut c_void {
    if parser.is_null() || len < 0 {
        return ptr::null_mut();
    }
    // Reject excessively large buffers (matches C overflow detection)
    // C allows up to INT_MAX/2 + (INT_MAX & 1) which rounds up to (INT_MAX+1)/2
    if len as usize > ((i32::MAX as usize) + 1) / 2 {
        return ptr::null_mut();
    }
    let handle = &mut *parser;
    match handle.parser.get_buffer(len as usize) {
        Some(buf) => buf.as_mut_ptr() as *mut c_void,
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParseBuffer(
    parser: XML_Parser,
    len: c_int,
    _is_final: c_int,
) -> XML_Status_t {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if len < 0 {
        (*parser).parser.set_error(XmlError::InvalidArgument);
        return XML_STATUS_ERROR;
    }
    let handle = &mut *parser;
    status_to_c(handle.parser.parse_buffer(len as usize, _is_final != 0))
}

#[no_mangle]
pub unsafe extern "C" fn XML_StopParser(parser: XML_Parser, resumable: XML_Bool) -> XML_Status_t {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let handle = &mut *parser;
    status_to_c(handle.parser.stop(resumable != 0))
}

#[no_mangle]
pub unsafe extern "C" fn XML_ResumeParser(parser: XML_Parser) -> XML_Status_t {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let handle = &mut *parser;
    status_to_c(handle.parser.resume())
}

// ============================================================================
// Error handling
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error_t {
    if parser.is_null() {
        return 0;
    }
    let handle = &*parser;
    error_to_c(handle.parser.error_code())
}

#[no_mangle]
pub unsafe extern "C" fn XML_ErrorString(code: XML_Error_t) -> *const XML_Char {
    // Return null-terminated C string for each error code
    // Can't use error_string() directly since Rust &str isn't null-terminated
    match code {
        0 => b"no error\0".as_ptr() as *const XML_Char,
        1 => b"out of memory\0".as_ptr() as *const XML_Char,
        2 => b"syntax error\0".as_ptr() as *const XML_Char,
        3 => b"no element found\0".as_ptr() as *const XML_Char,
        4 => b"not well-formed (invalid token)\0".as_ptr() as *const XML_Char,
        5 => b"unclosed token\0".as_ptr() as *const XML_Char,
        6 => b"partial character\0".as_ptr() as *const XML_Char,
        7 => b"mismatched tag\0".as_ptr() as *const XML_Char,
        8 => b"duplicate attribute\0".as_ptr() as *const XML_Char,
        9 => b"junk after document element\0".as_ptr() as *const XML_Char,
        10 => b"illegal parameter entity reference\0".as_ptr() as *const XML_Char,
        11 => b"undefined entity\0".as_ptr() as *const XML_Char,
        12 => b"recursive entity reference\0".as_ptr() as *const XML_Char,
        13 => b"asynchronous entity\0".as_ptr() as *const XML_Char,
        14 => b"reference to invalid character number\0".as_ptr() as *const XML_Char,
        15 => b"reference to binary entity\0".as_ptr() as *const XML_Char,
        16 => b"reference to external entity in attribute\0".as_ptr() as *const XML_Char,
        17 => b"XML or text declaration not at start of entity\0".as_ptr() as *const XML_Char,
        18 => b"unknown encoding\0".as_ptr() as *const XML_Char,
        19 => b"encoding specified in XML declaration is incorrect\0".as_ptr() as *const XML_Char,
        20 => b"unclosed CDATA section\0".as_ptr() as *const XML_Char,
        21 => b"error in processing external entity reference\0".as_ptr() as *const XML_Char,
        22 => b"document is not standalone\0".as_ptr() as *const XML_Char,
        23 => b"unexpected parser state - please send a bug report\0".as_ptr() as *const XML_Char,
        24 => b"entity declared in parameter entity\0".as_ptr() as *const XML_Char,
        25 => b"requested feature requires XML_DTD support in Expat\0".as_ptr() as *const XML_Char,
        26 => b"cannot change setting once parsing has begun\0".as_ptr() as *const XML_Char,
        27 => b"unbound prefix\0".as_ptr() as *const XML_Char,
        28 => b"must not undeclare prefix\0".as_ptr() as *const XML_Char,
        29 => b"incomplete markup in parameter entity\0".as_ptr() as *const XML_Char,
        30 => b"XML declaration not well-formed\0".as_ptr() as *const XML_Char,
        31 => b"text declaration not well-formed\0".as_ptr() as *const XML_Char,
        32 => b"illegal character(s) in public id\0".as_ptr() as *const XML_Char,
        33 => b"parser suspended\0".as_ptr() as *const XML_Char,
        34 => b"parser not suspended\0".as_ptr() as *const XML_Char,
        35 => b"parsing aborted\0".as_ptr() as *const XML_Char,
        36 => b"parsing finished\0".as_ptr() as *const XML_Char,
        37 => b"cannot suspend in external parameter entity\0".as_ptr() as *const XML_Char,
        38 => b"reserved prefix (xml) must not be undeclared or bound to another namespace name\0"
            .as_ptr() as *const XML_Char,
        39 => b"reserved prefix (xmlns) must not be declared or undeclared\0".as_ptr()
            as *const XML_Char,
        40 => b"reserved namespace URI must not be used\0".as_ptr() as *const XML_Char,
        41 => b"invalid argument\0".as_ptr() as *const XML_Char,
        42 => b"no buffer\0".as_ptr() as *const XML_Char,
        43 => b"amplification limit breach\0".as_ptr() as *const XML_Char,
        44 => b"parser not started\0".as_ptr() as *const XML_Char,
        _ => ptr::null(),
    }
}

// ============================================================================
// Position info
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentLineNumber(parser: XML_Parser) -> c_ulong {
    if parser.is_null() {
        return 0;
    }
    let handle = &*parser;
    handle.parser.current_line_number() as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> c_ulong {
    if parser.is_null() {
        return 0;
    }
    let handle = &*parser;
    handle.parser.current_column_number() as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteIndex(parser: XML_Parser) -> c_long {
    if parser.is_null() {
        return -1;
    }
    let handle = &*parser;
    handle.parser.current_byte_index() as c_long
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteCount(parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return 0;
    }
    let handle = &*parser;
    handle.parser.current_byte_count() as c_int
}

// ============================================================================
// Status
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_GetParsingStatus(parser: XML_Parser, status: *mut XML_ParsingStatus) {
    if parser.is_null() || status.is_null() {
        return;
    }
    let handle = &*parser;
    let ps = handle.parser.parsing_status();
    let parsing = match ps.state {
        ParsingState::Initialized => XML_INITIALIZED,
        ParsingState::Parsing => XML_PARSING,
        ParsingState::Finished => XML_FINISHED,
        ParsingState::Suspended => XML_SUSPENDED,
    };
    (*status).parsing = parsing;
    (*status).finalBuffer = if ps.final_buffer { XML_TRUE } else { XML_FALSE };
}

// ============================================================================
// Configuration
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_SetEncoding(
    parser: XML_Parser,
    encoding: *const XML_Char,
) -> XML_Status_t {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let handle = &mut *parser;
    if encoding.is_null() {
        // NULL encoding clears the encoding setting (matches C behavior)
        handle.parser.clear_encoding();
        return XML_STATUS_OK;
    }
    let enc_str = match CStr::from_ptr(encoding).to_str() {
        Ok(s) => s,
        Err(_) => return XML_STATUS_ERROR,
    };
    status_to_c(handle.parser.set_encoding(enc_str))
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetBase(parser: XML_Parser, base: *const XML_Char) -> XML_Status_t {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let handle = &mut *parser;
    if base.is_null() {
        handle.base_c_string = None;
        return XML_STATUS_OK;
    }
    let base_str = match CStr::from_ptr(base).to_str() {
        Ok(s) => s,
        Err(_) => return XML_STATUS_ERROR,
    };
    let result = handle.parser.set_base(base_str);
    // Store null-terminated copy for XML_GetBase
    let mut bytes = base_str.as_bytes().to_vec();
    bytes.push(0);
    handle.base_c_string = Some(bytes);
    status_to_c(result)
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBase(parser: XML_Parser) -> *const XML_Char {
    if parser.is_null() {
        return ptr::null();
    }
    let handle = &*parser;
    match &handle.base_c_string {
        Some(bytes) => bytes.as_ptr() as *const XML_Char,
        None => ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetHashSalt(parser: XML_Parser, salt: c_ulong) -> c_int {
    if parser.is_null() {
        return 0;
    }
    let handle = &mut *parser;
    if handle.parser.set_hash_salt(salt as u64) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetParamEntityParsing(
    parser: XML_Parser,
    parsing: XML_ParamEntityParsing,
) -> c_int {
    if parser.is_null() {
        return 0;
    }
    let handle = &mut *parser;
    let mode = match parsing {
        XML_PARAM_ENTITY_PARSING_NEVER => ParamEntityParsing::Never,
        XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE => ParamEntityParsing::UnlessStandalone,
        XML_PARAM_ENTITY_PARSING_ALWAYS => ParamEntityParsing::Always,
        _ => return 0,
    };
    if handle.parser.set_param_entity_parsing(mode) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_UseForeignDTD(parser: XML_Parser, use_dtd: XML_Bool) -> XML_Error_t {
    if parser.is_null() {
        return error_to_c(XmlError::InvalidArgument);
    }
    let handle = &mut *parser;
    // use_foreign_dtd may panic (todo!), so catch that
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        handle.parser.use_foreign_dtd(use_dtd != 0)
    }));
    match result {
        Ok(err) => error_to_c(err),
        Err(_) => error_to_c(XmlError::UnexpectedState),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetReturnNSTriplet(parser: XML_Parser, do_nst: c_int) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.parser.set_return_ns_triplet(do_nst != 0);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetReparseDeferralEnabled(
    parser: XML_Parser,
    enabled: XML_Bool,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    let handle = &mut *parser;
    if handle.parser.set_reparse_deferral_enabled(enabled != 0) {
        XML_TRUE
    } else {
        XML_FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_UseParserAsHandlerArg(parser: XML_Parser) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.use_parser_as_handler_arg = true;
}

// ============================================================================
// User data
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_SetUserData(parser: XML_Parser, user_data: *mut c_void) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.user_data = user_data;
}

// ============================================================================
// Handler setters
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_SetElementHandler(
    parser: XML_Parser,
    start: XML_StartElementHandler,
    end: XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    // Set start and end individually
    XML_SetStartElementHandler(parser, start);
    XML_SetEndElementHandler(parser, end);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartElementHandler(
    parser: XML_Parser,
    handler: XML_StartElementHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_start_element_handler = handler;

    if let Some(start_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_start_element_handler(Some(Box::new(move |name, attrs| {
                let ud = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                let mut name_bytes: Vec<u8> = name.as_bytes().to_vec();
                name_bytes.push(0);

                let mut c_attrs: Vec<*const XML_Char> = Vec::new();
                for (k, v) in attrs {
                    let mut kb: Vec<u8> = k.as_bytes().to_vec();
                    kb.push(0);
                    let mut vb: Vec<u8> = v.as_bytes().to_vec();
                    vb.push(0);
                    c_attrs.push(kb.as_ptr() as *const XML_Char);
                    c_attrs.push(vb.as_ptr() as *const XML_Char);
                    std::mem::forget(kb);
                    std::mem::forget(vb);
                }
                c_attrs.push(ptr::null());

                start_fn(
                    ud,
                    name_bytes.as_ptr() as *const XML_Char,
                    c_attrs.as_mut_ptr(),
                );
            })));
    } else {
        handle.parser.set_start_element_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndElementHandler(
    parser: XML_Parser,
    handler: XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_end_element_handler = handler;

    if let Some(end_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_end_element_handler(Some(Box::new(move |name| {
                let mut name_bytes: Vec<u8> = name.as_bytes().to_vec();
                name_bytes.push(0);
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                end_fn(
                    handler_arg,
                    name_bytes.as_ptr() as *const XML_Char,
                );
            })));
    } else {
        handle.parser.set_end_element_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    parser: XML_Parser,
    handler: XML_CharacterDataHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_character_data_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_character_data_handler(Some(Box::new(move |data| {
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                handler_fn(
                    handler_arg,
                    data.as_ptr() as *const XML_Char,
                    data.len() as c_int,
                );
            })));
    } else {
        handle.parser.set_character_data_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    parser: XML_Parser,
    handler: XML_ProcessingInstructionHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_processing_instruction_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_processing_instruction_handler(Some(Box::new(move |target, data| {
                let mut target_bytes: Vec<u8> = target.as_bytes().to_vec();
                target_bytes.push(0);
                let mut data_bytes: Vec<u8> = data.as_bytes().to_vec();
                data_bytes.push(0);
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                handler_fn(
                    handler_arg,
                    target_bytes.as_ptr() as *const XML_Char,
                    data_bytes.as_ptr() as *const XML_Char,
                );
            })));
    } else {
        handle.parser.set_processing_instruction_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCommentHandler(parser: XML_Parser, handler: XML_CommentHandler) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_comment_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_comment_handler(Some(Box::new(move |data| {
                let mut bytes: Vec<u8> = data.to_vec();
                bytes.push(0);
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                handler_fn(handler_arg, bytes.as_ptr() as *const XML_Char);
            })));
    } else {
        handle.parser.set_comment_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCdataSectionHandler(
    parser: XML_Parser,
    start: XML_StartCdataSectionHandler,
    end: XML_EndCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    XML_SetStartCdataSectionHandler(parser, start);
    XML_SetEndCdataSectionHandler(parser, end);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartCdataSectionHandler(
    parser: XML_Parser,
    handler: XML_StartCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_start_cdata_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_start_cdata_section_handler(Some(Box::new(move || {
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                handler_fn(handler_arg);
            })));
    } else {
        handle.parser.set_start_cdata_section_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndCdataSectionHandler(
    parser: XML_Parser,
    handler: XML_EndCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_end_cdata_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_end_cdata_section_handler(Some(Box::new(move || {
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                handler_fn(handler_arg);
            })));
    } else {
        handle.parser.set_end_cdata_section_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandler(parser: XML_Parser, handler: XML_DefaultHandler) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_default_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_default_handler(Some(Box::new(move |data| {
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                handler_fn(
                    handler_arg,
                    data.as_ptr() as *const XML_Char,
                    data.len() as c_int,
                );
            })));
    } else {
        handle.parser.set_default_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandlerExpand(
    parser: XML_Parser,
    handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_default_handler_expand(Some(Box::new(move |data| {
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                handler_fn(
                    handler_arg,
                    data.as_ptr() as *const XML_Char,
                    data.len() as c_int,
                );
            })));
    } else {
        handle.parser.set_default_handler_expand(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDoctypeDeclHandler(
    parser: XML_Parser,
    start: XML_StartDoctypeDeclHandler,
    end: XML_EndDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    XML_SetStartDoctypeDeclHandler(parser, start);
    XML_SetEndDoctypeDeclHandler(parser, end);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler(
    parser: XML_Parser,
    handler: XML_StartDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_start_doctype_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle.parser.set_start_doctype_decl_handler(Some(Box::new(
            move |doctype_name, sysid, pubid, has_internal_subset| {
                let mut name_bytes: Vec<u8> = doctype_name.as_bytes().to_vec();
                name_bytes.push(0);

                let sysid_bytes: Option<Vec<u8>> = sysid.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                let pubid_bytes: Option<Vec<u8>> = pubid.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });

                let sysid_ptr = sysid_bytes
                    .as_ref()
                    .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                let pubid_ptr = pubid_bytes
                    .as_ref()
                    .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);

                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };

                handler_fn(
                    handler_arg,
                    name_bytes.as_ptr() as *const XML_Char,
                    sysid_ptr,
                    pubid_ptr,
                    if has_internal_subset { 1 } else { 0 },
                );
            },
        )));
    } else {
        handle.parser.set_start_doctype_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler(
    parser: XML_Parser,
    handler: XML_EndDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_end_doctype_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_end_doctype_decl_handler(Some(Box::new(move || {
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };
                handler_fn(handler_arg);
            })));
    } else {
        handle.parser.set_end_doctype_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetXmlDeclHandler(parser: XML_Parser, handler: XML_XmlDeclHandler) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_xml_decl_handler = handler;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_xml_decl_handler(Some(Box::new(move |version, encoding, standalone| {
                let version_bytes: Option<Vec<u8>> = version.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                let encoding_bytes: Option<Vec<u8>> = encoding.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });

                let version_ptr = version_bytes
                    .as_ref()
                    .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                let encoding_ptr = encoding_bytes
                    .as_ref()
                    .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);

                // standalone: Some(1) = yes, Some(0) = no, None = -1
                let standalone_int = match standalone {
                    Some(val) => val,
                    None => -1,
                };

                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    (*parser_ptr).user_data
                };

                handler_fn(
                    handler_arg,
                    version_ptr,
                    encoding_ptr,
                    standalone_int,
                );
            })));
    } else {
        handle.parser.set_xml_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandler(
    parser: XML_Parser,
    handler: XML_ExternalEntityRefHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;

    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle.parser.set_external_entity_ref_handler(Some(Box::new(
            move |context, base, system_id, public_id| {
                // Empty context from foreign DTD → pass NULL to C
                // (C distinguishes NULL context from "" context)
                let ctx_bytes: Option<Vec<u8>> = if context.is_empty() {
                    None
                } else {
                    let mut b = context.as_bytes().to_vec();
                    b.push(0);
                    Some(b)
                };

                let base_bytes: Option<Vec<u8>> = base.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                let sysid_bytes: Option<Vec<u8>> = system_id.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                let pubid_bytes: Option<Vec<u8>> = public_id.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });

                let base_ptr = base_bytes
                    .as_ref()
                    .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                let sysid_ptr = sysid_bytes
                    .as_ref()
                    .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);
                let pubid_ptr = pubid_bytes
                    .as_ref()
                    .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);

                let ctx_ptr = ctx_bytes
                    .as_ref()
                    .map_or(ptr::null(), |b| b.as_ptr() as *const XML_Char);

                // Get the handler arg at call time - if NULL, use parser itself
                let handler_arg = unsafe {
                    let handle = &*parser_ptr;
                    if handle.ext_entity_ref_handler_arg.is_null() {
                        parser_ptr as *mut c_void
                    } else {
                        handle.ext_entity_ref_handler_arg
                    }
                };

                let result = handler_fn(handler_arg as XML_Parser, ctx_ptr, base_ptr, sysid_ptr, pubid_ptr);
                result != 0
            },
        )));
    } else {
        handle.parser.set_external_entity_ref_handler(None);
    }
}

// ============================================================================
// Attribute info
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount(parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -1;
    }
    let handle = &*parser;
    // specified_attribute_count may panic (todo!), catch it
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        handle.parser.specified_attribute_count()
    }));
    match result {
        Ok(count) => count as c_int,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetIdAttributeIndex(parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -1;
    }
    let handle = &*parser;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        handle.parser.id_attribute_index()
    }));
    match result {
        Ok(idx) => idx as c_int,
        Err(_) => -1,
    }
}

// ============================================================================
// External entity parser
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_ExternalEntityParserCreate(
    parser: XML_Parser,
    context: *const XML_Char,
    encoding: *const XML_Char,
) -> XML_Parser {
    if parser.is_null() {
        return ptr::null_mut();
    }
    let handle = &mut *parser;

    let ctx_str = if context.is_null() {
        ""
    } else {
        match CStr::from_ptr(context).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };

    let enc = if encoding.is_null() {
        None
    } else {
        CStr::from_ptr(encoding).to_str().ok()
    };

    match handle.parser.create_external_entity_parser(ctx_str, enc) {
        Some(ext_parser) => {
            // If creating a DTD subset parser (empty context), mark param entity as read
            // This matches C's behavior where creating a child parser implies DTD content will be processed
            if ctx_str.is_empty() {
                handle.parser.param_entity_read = true;
            }
            let new_ptr = new_handle(ext_parser);
            // Copy user_data and handler settings from parent (matches C behavior)
            (*new_ptr).user_data = handle.user_data;
            (*new_ptr).use_parser_as_handler_arg = handle.use_parser_as_handler_arg;

            // Copy ext_entity_ref_handler_arg
            if handle.ext_entity_ref_handler_arg == parser as *mut c_void {
                (*new_ptr).ext_entity_ref_handler_arg = new_ptr as *mut c_void;
            } else {
                (*new_ptr).ext_entity_ref_handler_arg = handle.ext_entity_ref_handler_arg;
            }

            // Copy content handlers
            (*new_ptr).c_start_element_handler = handle.c_start_element_handler;
            (*new_ptr).c_end_element_handler = handle.c_end_element_handler;
            (*new_ptr).c_character_data_handler = handle.c_character_data_handler;
            (*new_ptr).c_processing_instruction_handler = handle.c_processing_instruction_handler;
            (*new_ptr).c_comment_handler = handle.c_comment_handler;
            (*new_ptr).c_default_handler = handle.c_default_handler;
            (*new_ptr).c_start_cdata_handler = handle.c_start_cdata_handler;
            (*new_ptr).c_end_cdata_handler = handle.c_end_cdata_handler;
            (*new_ptr).c_xml_decl_handler = handle.c_xml_decl_handler;
            (*new_ptr).c_start_doctype_handler = handle.c_start_doctype_handler;
            (*new_ptr).c_end_doctype_handler = handle.c_end_doctype_handler;

            // Copy DTD handlers
            (*new_ptr).c_element_decl_handler = handle.c_element_decl_handler;
            (*new_ptr).c_attlist_decl_handler = handle.c_attlist_decl_handler;
            (*new_ptr).c_entity_decl_handler = handle.c_entity_decl_handler;
            (*new_ptr).c_notation_decl_handler = handle.c_notation_decl_handler;
            (*new_ptr).c_unparsed_entity_decl_handler = handle.c_unparsed_entity_decl_handler;

            // Copy namespace handlers
            (*new_ptr).c_start_ns_handler = handle.c_start_ns_handler;
            (*new_ptr).c_end_ns_handler = handle.c_end_ns_handler;

            // Re-register handlers on child parser (creates closures with correct parser pointer)
            XML_SetElementHandler(new_ptr, handle.c_start_element_handler, handle.c_end_element_handler);
            XML_SetCharacterDataHandler(new_ptr, handle.c_character_data_handler);
            XML_SetProcessingInstructionHandler(new_ptr, handle.c_processing_instruction_handler);
            XML_SetCommentHandler(new_ptr, handle.c_comment_handler);
            if handle.c_default_handler.is_some() {
                XML_SetDefaultHandler(new_ptr, handle.c_default_handler);
            }
            XML_SetCdataSectionHandler(new_ptr, handle.c_start_cdata_handler, handle.c_end_cdata_handler);
            XML_SetXmlDeclHandler(new_ptr, handle.c_xml_decl_handler);
            XML_SetStartDoctypeDeclHandler(new_ptr, handle.c_start_doctype_handler);
            XML_SetEndDoctypeDeclHandler(new_ptr, handle.c_end_doctype_handler);
            if handle.c_ext_entity_ref_handler.is_some() {
                XML_SetExternalEntityRefHandler(new_ptr, handle.c_ext_entity_ref_handler);
            }
            if handle.c_not_standalone_handler.is_some() {
                XML_SetNotStandaloneHandler(new_ptr, handle.c_not_standalone_handler);
            }
            if handle.c_skipped_entity_handler.is_some() {
                XML_SetSkippedEntityHandler(new_ptr, handle.c_skipped_entity_handler);
            }

            // Re-register DTD handlers on child parser
            if handle.c_element_decl_handler.is_some() {
                XML_SetElementDeclHandler(new_ptr, handle.c_element_decl_handler);
            }
            if handle.c_attlist_decl_handler.is_some() {
                XML_SetAttlistDeclHandler(new_ptr, handle.c_attlist_decl_handler);
            }
            if handle.c_entity_decl_handler.is_some() {
                XML_SetEntityDeclHandler(new_ptr, handle.c_entity_decl_handler);
            }
            if handle.c_notation_decl_handler.is_some() {
                XML_SetNotationDeclHandler(new_ptr, handle.c_notation_decl_handler);
            }
            if handle.c_unparsed_entity_decl_handler.is_some() {
                XML_SetUnparsedEntityDeclHandler(new_ptr, handle.c_unparsed_entity_decl_handler);
            }

            // Re-register namespace handlers on child parser
            XML_SetStartNamespaceDeclHandler(new_ptr, handle.c_start_ns_handler);
            XML_SetEndNamespaceDeclHandler(new_ptr, handle.c_end_ns_handler);

            new_ptr
        }
        None => ptr::null_mut(),
    }
}

// ============================================================================
// Billion laughs protection
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
    parser: XML_Parser,
    maximum_amplification_factor: f32,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    let handle = &mut *parser;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        handle
            .parser
            .set_billion_laughs_attack_protection_maximum_amplification(
                maximum_amplification_factor,
            )
    }));
    match result {
        Ok(true) => XML_TRUE,
        _ => XML_FALSE,
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
    parser: XML_Parser,
    activation_threshold_bytes: c_ulong,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    let handle = &mut *parser;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        handle
            .parser
            .set_billion_laughs_attack_protection_activation_threshold(
                activation_threshold_bytes as u64,
            )
    }));
    match result {
        Ok(true) => XML_TRUE,
        _ => XML_FALSE,
    }
}

// ============================================================================
// Version
// ============================================================================

static VERSION_STRING: &[u8] = b"expat_2.7.5\0";

#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersion() -> *const XML_Char {
    VERSION_STRING.as_ptr() as *const XML_Char
}

#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersionInfo() -> XML_Expat_Version {
    XML_Expat_Version {
        major: 2,
        minor: 7,
        micro: 5,
    }
}

// ============================================================================
// Feature list
// ============================================================================

static FEATURE_DTD_NAME: &[u8] = b"XML_DTD\0";
static FEATURE_NS_NAME: &[u8] = b"XML_NS\0";
static FEATURE_CTX_NAME: &[u8] = b"XML_CONTEXT_BYTES\0";
static FEATURE_CHAR_NAME: &[u8] = b"sizeof(XML_Char)\0";
static FEATURE_LCHAR_NAME: &[u8] = b"sizeof(XML_LChar)\0";
static FEATURE_GE_NAME: &[u8] = b"XML_GE\0";
static FEATURE_BLAP_AMP: &[u8] = b"XML_BLAP_MAX_AMP\0";
static FEATURE_BLAP_THR: &[u8] = b"XML_BLAP_ACT_THRES\0";
static FEATURE_END_NAME: &[u8] = b"\0";

static FEATURES: [XML_Feature; 9] = [
    XML_Feature {
        feature: 3,
        name: FEATURE_DTD_NAME.as_ptr() as *const XML_Char,
        value: 0,
    },
    XML_Feature {
        feature: 8,
        name: FEATURE_NS_NAME.as_ptr() as *const XML_Char,
        value: 0,
    },
    XML_Feature {
        feature: 4,
        name: FEATURE_CTX_NAME.as_ptr() as *const XML_Char,
        value: 1024,
    },
    XML_Feature {
        feature: 6,
        name: FEATURE_CHAR_NAME.as_ptr() as *const XML_Char,
        value: 1,
    },
    XML_Feature {
        feature: 7,
        name: FEATURE_LCHAR_NAME.as_ptr() as *const XML_Char,
        value: 1,
    },
    XML_Feature {
        feature: 13,
        name: FEATURE_GE_NAME.as_ptr() as *const XML_Char,
        value: 1,
    },
    XML_Feature {
        feature: 11,
        name: FEATURE_BLAP_AMP.as_ptr() as *const XML_Char,
        value: 100,
    },
    XML_Feature {
        feature: 12,
        name: FEATURE_BLAP_THR.as_ptr() as *const XML_Char,
        value: 8388608,
    },
    XML_Feature {
        feature: 0,
        name: FEATURE_END_NAME.as_ptr() as *const XML_Char,
        value: 0,
    },
];

#[no_mangle]
pub unsafe extern "C" fn XML_GetFeatureList() -> *const XML_Feature {
    FEATURES.as_ptr()
}

// ============================================================================
// Missing handler setters
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_GetUserData(parser: XML_Parser) -> *mut c_void {
    if parser.is_null() {
        return ptr::null_mut();
    }
    (*parser).user_data
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg(parser: XML_Parser, arg: *mut c_void) {
    if parser.is_null() {
        return;
    }
    (*parser).ext_entity_ref_handler_arg = arg;
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNotStandaloneHandler(
    parser: XML_Parser,
    handler: XML_NotStandaloneHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_not_standalone_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_not_standalone_handler(Some(Box::new(move || -> bool {
                let h = &*parser_ptr;
                handler_fn(h.user_data) != 0
            })));
    } else {
        handle.parser.set_not_standalone_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetSkippedEntityHandler(
    parser: XML_Parser,
    handler: XML_SkippedEntityHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_skipped_entity_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle.parser.set_skipped_entity_handler(Some(Box::new(
            move |name: &str, is_param: bool| {
                let h = &*parser_ptr;
                let mut nb: Vec<u8> = name.as_bytes().to_vec();
                nb.push(0);
                let handler_arg = if (*parser_ptr).use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    h.user_data
                };
                handler_fn(
                    handler_arg,
                    nb.as_ptr() as *const XML_Char,
                    if is_param { 1 } else { 0 },
                );
            },
        )));
    } else {
        handle.parser.set_skipped_entity_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetElementDeclHandler(
    parser: XML_Parser,
    handler: XML_ElementDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_element_decl_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_element_decl_handler(Some(Box::new(move |name: &str, _model: &str| {
                let h = unsafe { &mut *(parser_ptr as *mut ParserHandle) };
                let mut nb: Vec<u8> = name.as_bytes().to_vec();
                nb.push(0);

                // Build the flat content model array from the Rust tree
                let model_ptr = if let Some(rust_model) = h.parser.last_content_model() {
                    if rust_model.is_empty() {
                        // EMPTY or ANY element - pass NULL
                        ptr::null_mut()
                    } else {
                        // First, pre-collect all name buffers to avoid reallocation issues
                        h.last_content_model_names.clear();
                        h.last_content_model_names.reserve(rust_model.len());

                        for (_, _, name_bytes, _) in rust_model.iter() {
                            if let Some(nb) = name_bytes {
                                h.last_content_model_names.push(nb.clone());
                            } else {
                                h.last_content_model_names.push(Vec::new());
                            }
                        }

                        // Now build the array with stable name pointers
                        let mut array: Vec<XML_Content> = Vec::with_capacity(rust_model.len());
                        for (idx, (type_u, quant_u, name_bytes, numchildren)) in rust_model.iter().enumerate() {
                            let name_ptr = if !h.last_content_model_names[idx].is_empty() {
                                h.last_content_model_names[idx].as_ptr() as *mut c_char
                            } else {
                                ptr::null_mut()
                            };

                            array.push(XML_Content {
                                type_: *type_u as c_int,
                                quant: *quant_u as c_int,
                                name: name_ptr,
                                numchildren: 0,
                                children: ptr::null_mut(),
                            });
                        }

                        // Now set numchildren and children pointers
                        // The children are laid out in the order produced by flatten_content_node
                        let array_ptr = array.as_mut_ptr();
                        let mut next_idx = 1; // next available index for children

                        for (idx, (_, _, _, numchildren)) in rust_model.iter().enumerate() {
                            (*array_ptr.add(idx)).numchildren = *numchildren as c_int;
                            if *numchildren > 0 && next_idx < rust_model.len() {
                                (*array_ptr.add(idx)).children = array_ptr.add(next_idx);
                                // Skip to next sibling group
                                // Children are laid out sequentially, then we recursively process their children
                                next_idx += *numchildren as usize;
                            }
                        }

                        // Store the array on the handle
                        h.last_content_model_array = Some(array.into_boxed_slice() as Box<[XML_Content]>);

                        // Return pointer to the stored array
                        if let Some(ref boxed) = h.last_content_model_array {
                            boxed.as_ptr() as *mut XML_Content
                        } else {
                            ptr::null_mut()
                        }
                    }
                } else {
                    ptr::null_mut()
                };

                unsafe {
                    handler_fn(h.user_data, nb.as_ptr() as *const XML_Char, model_ptr);
                }
            })));
    } else {
        handle.parser.set_element_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetAttlistDeclHandler(
    parser: XML_Parser,
    handler: XML_AttlistDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_attlist_decl_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle.parser.set_attlist_decl_handler(Some(Box::new(
            move |elname: &str,
                  attname: &str,
                  att_type: &str,
                  dflt: Option<&str>,
                  _dflt2: Option<&str>,
                  is_required: bool| {
                let h = &*parser_ptr;
                let mut el = elname.as_bytes().to_vec();
                el.push(0);
                let mut att = attname.as_bytes().to_vec();
                att.push(0);
                let mut tp = att_type.as_bytes().to_vec();
                tp.push(0);
                let df = dflt.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                handler_fn(
                    h.user_data,
                    el.as_ptr() as _,
                    att.as_ptr() as _,
                    tp.as_ptr() as _,
                    df.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                    if is_required { 1 } else { 0 },
                );
            },
        )));
    } else {
        handle.parser.set_attlist_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEntityDeclHandler(
    parser: XML_Parser,
    handler: XML_EntityDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_entity_decl_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle.parser.set_entity_decl_handler(Some(Box::new(
            move |name: &str,
                  is_param: bool,
                  value: Option<&str>,
                  base: Option<&str>,
                  system_id: Option<&str>| {
                let h = &*parser_ptr;
                let mut nb = name.as_bytes().to_vec();
                nb.push(0);
                let vb = value.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                let vlen = value.map_or(0, |s| s.len()) as c_int;
                let bb = base.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                let sb = system_id.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                handler_fn(
                    h.user_data,
                    nb.as_ptr() as _,
                    if is_param { 1 } else { 0 },
                    vb.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                    vlen,
                    bb.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                    sb.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                    ptr::null(),
                    ptr::null(),
                );
            },
        )));
    } else {
        handle.parser.set_entity_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler(
    parser: XML_Parser,
    handler: XML_UnparsedEntityDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_unparsed_entity_decl_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_unparsed_entity_decl_handler(Some(Box::new(
                move |name: &str, base: Option<&str>, system_id: &str, notation: Option<&str>| {
                    let h = &*parser_ptr;
                    let mut nb = name.as_bytes().to_vec();
                    nb.push(0);
                    let bb = base.map(|s| {
                        let mut b = s.as_bytes().to_vec();
                        b.push(0);
                        b
                    });
                    let mut sb = system_id.as_bytes().to_vec();
                    sb.push(0);
                    let nt = notation.map(|s| {
                        let mut b = s.as_bytes().to_vec();
                        b.push(0);
                        b
                    });
                    handler_fn(
                        h.user_data,
                        nb.as_ptr() as _,
                        bb.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                        sb.as_ptr() as _,
                        nt.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                    );
                },
            )));
    } else {
        handle.parser.set_unparsed_entity_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNotationDeclHandler(
    parser: XML_Parser,
    handler: XML_NotationDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_notation_decl_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle.parser.set_notation_decl_handler(Some(Box::new(
            move |name: &str, base: Option<&str>, system_id: &str, public_id: Option<&str>| {
                let h = &*parser_ptr;
                let mut nb = name.as_bytes().to_vec();
                nb.push(0);
                let bb = base.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                let mut sb = system_id.as_bytes().to_vec();
                sb.push(0);
                let pb = public_id.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                handler_fn(
                    h.user_data,
                    nb.as_ptr() as _,
                    bb.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                    sb.as_ptr() as _,
                    pb.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                );
            },
        )));
    } else {
        handle.parser.set_notation_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNamespaceDeclHandler(
    parser: XML_Parser,
    start: XML_StartNamespaceDeclHandler,
    end: XML_EndNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    XML_SetStartNamespaceDeclHandler(parser, start);
    XML_SetEndNamespaceDeclHandler(parser, end);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler(
    parser: XML_Parser,
    handler: XML_StartNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_start_ns_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle
            .parser
            .set_start_namespace_decl_handler(Some(Box::new(
                move |prefix: Option<&str>, uri: &str| {
                    let h = &*parser_ptr;
                    let ud = if h.use_parser_as_handler_arg {
                        parser_ptr as *mut c_void
                    } else {
                        h.user_data
                    };
                    let pb = prefix.map(|s| {
                        let mut b = s.as_bytes().to_vec();
                        b.push(0);
                        b
                    });
                    let mut ub = uri.as_bytes().to_vec();
                    ub.push(0);
                    handler_fn(
                        ud,
                        pb.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                        ub.as_ptr() as _,
                    );
                },
            )));
    } else {
        handle.parser.set_start_namespace_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler(
    parser: XML_Parser,
    handler: XML_EndNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_end_ns_handler = handler;
    if let Some(handler_fn) = handler {
        let parser_ptr = parser;
        handle.parser.set_end_namespace_decl_handler(Some(Box::new(
            move |prefix: Option<&str>| {
                let h = &*parser_ptr;
                let ud = if h.use_parser_as_handler_arg {
                    parser_ptr as *mut c_void
                } else {
                    h.user_data
                };
                let pb = prefix.map(|s| {
                    let mut b = s.as_bytes().to_vec();
                    b.push(0);
                    b
                });
                handler_fn(
                    ud,
                    pb.as_ref().map_or(ptr::null(), |b| b.as_ptr() as _),
                );
            },
        )));
    } else {
        handle.parser.set_end_namespace_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetUnknownEncodingHandler(
    parser: XML_Parser,
    handler: XML_UnknownEncodingHandler,
    data: *mut c_void,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;
    handle.c_unknown_encoding_handler = handler;
    handle.c_unknown_encoding_data = data;
    if let Some(handler_fn) = handler {
        let enc_data = data;
        let parser_ptr = parser;
        handle
            .parser
            .set_unknown_encoding_handler(Some(Box::new(move |name: &str| -> bool {
                let mut nb = name.as_bytes().to_vec();
                nb.push(0);
                let mut enc = XML_Encoding {
                    map: [0i32; 256],
                    data: ptr::null_mut(),
                    convert: None,
                    release: None,
                };
                if handler_fn(enc_data, nb.as_ptr() as _, &mut enc) == 0 {
                    return false;
                }
                // Validate the encoding map (matches C xmltok.c XmlInitUnknownEncoding)
                let has_converter = enc.convert.is_some();
                for i in 0..256 {
                    let c = enc.map[i];
                    if c == -1 {
                        // Unmapped byte — will be treated as malformed during parsing
                        // This is OK — C marks these as BT_MALFORM and continues
                    } else if c < -4 {
                        return false; // Invalid multi-byte indicator
                    } else if c < 0 {
                        if !has_converter {
                            return false; // Multi-byte needs converter
                        }
                    } else if c > 0xFFFF {
                        return false; // Out of Unicode range
                    } else if c < 0x80 && is_significant_ascii(c as u8) && c != i as i32 {
                        // ASCII character with XML significance mapped to wrong position
                        return false;
                    } else if i < 128 && is_significant_ascii(i as u8) && c != i as i32 {
                        // Significant ASCII byte mapped to different value
                        return false;
                    }
                }
                // Store the encoding map and converter on the ParserHandle and Rust Parser
                let ffi_handle = &mut *(parser_ptr as *mut ParserHandle);
                ffi_handle.custom_encoding_map = Some(Box::new(enc.map));
                ffi_handle.custom_encoding_converter = enc.convert;
                ffi_handle.custom_encoding_data = enc.data;

                // Also store on the Rust Parser so the transcoding logic can use it
                ffi_handle.parser.custom_encoding_map = Some(enc.map);
                ffi_handle.parser.custom_encoding_converter = enc.convert;
                ffi_handle.parser.custom_encoding_data = enc.data;

                true
            })));
    } else {
        handle.parser.set_unknown_encoding_handler(None);
    }
}

// ============================================================================
// Misc API functions
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_DefaultCurrent(parser: XML_Parser) {
    if parser.is_null() {
        return;
    }
    (*parser).parser.default_current();
}

#[no_mangle]
pub unsafe extern "C" fn XML_FreeContentModel(parser: XML_Parser, model: *mut XML_Content) {
    let _ = parser;
    // Model pointer may be NULL (our shim doesn't build XML_Content trees yet)
    // When non-null, it was heap-allocated and should be freed
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate_MM(
    encoding: *const XML_Char,
    memsuite: *const XML_Memory_Handling_Suite,
    ns_separator: *const XML_Char,
) -> XML_Parser {
    let enc = if encoding.is_null() {
        None
    } else {
        CStr::from_ptr(encoding).to_str().ok()
    };
    let parser = if ns_separator.is_null() {
        Parser::new(enc)
    } else {
        let sep = *ns_separator as u8 as char;
        Parser::new_ns(enc, sep)
    };
    match parser {
        Some(p) => new_handle(p),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetInputContext(
    parser: XML_Parser,
    offset: *mut c_int,
    size: *mut c_int,
) -> *const c_char {
    if parser.is_null() {
        return ptr::null();
    }
    let handle = &*parser;
    let (buf, event_offset) = handle.parser.get_input_context();
    if buf.is_empty() {
        return ptr::null();
    }
    if !offset.is_null() {
        *offset = event_offset as c_int;
    }
    if !size.is_null() {
        *size = buf.len() as c_int;
    }
    buf.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn XML_MemMalloc(_parser: XML_Parser, size: usize) -> *mut c_void {
    libc_malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn XML_MemRealloc(
    _parser: XML_Parser,
    ptr: *mut c_void,
    size: usize,
) -> *mut c_void {
    libc_realloc(ptr, size)
}

#[no_mangle]
pub unsafe extern "C" fn XML_MemFree(_parser: XML_Parser, ptr: *mut c_void) {
    libc_free(ptr)
}

// Minimal libc wrappers
extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

unsafe fn libc_malloc(size: usize) -> *mut c_void {
    malloc(size)
}
unsafe fn libc_realloc(p: *mut c_void, size: usize) -> *mut c_void {
    realloc(p, size)
}
unsafe fn libc_free(p: *mut c_void) {
    free(p)
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetAllocTrackerMaximumAmplification(
    parser: XML_Parser,
    factor: f32,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    if (*parser)
        .parser
        .set_alloc_tracker_maximum_amplification(factor)
    {
        XML_TRUE
    } else {
        XML_FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetAllocTrackerActivationThreshold(
    parser: XML_Parser,
    threshold: c_ulong,
) -> XML_Bool {
    if parser.is_null() {
        return XML_FALSE;
    }
    if (*parser)
        .parser
        .set_alloc_tracker_activation_threshold(threshold as u64)
    {
        XML_TRUE
    } else {
        XML_FALSE
    }
}

// ============================================================================
// Internal testing symbols required by C test suite
// ============================================================================

#[no_mangle]
pub static mut g_bytesScanned: c_int = 0;

#[no_mangle]
pub static mut g_reparseDeferralEnabledDefault: XML_Bool = 1; // XML_TRUE

#[no_mangle]
pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters(
    from: *const c_char,
    from_lim_ref: *mut *const c_char,
) {
    if from.is_null() || from_lim_ref.is_null() || (*from_lim_ref).is_null() {
        return;
    }
    let from_ptr = from as *const u8;
    let lim_ptr = *from_lim_ref as *const u8;
    let len = lim_ptr.offset_from(from_ptr) as usize;
    if len == 0 {
        return;
    }
    let data = std::slice::from_raw_parts(from_ptr, len);
    let new_len = expat_rust::xmltok::trim_to_complete_utf8_characters(data);
    *from_lim_ref = from.add(new_len);
}

#[no_mangle]
pub unsafe extern "C" fn testingAccountingGetCountBytesDirect(_parser: XML_Parser) -> c_ulong {
    0
}

#[no_mangle]
pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect(_parser: XML_Parser) -> c_ulong {
    0
}

#[no_mangle]
pub unsafe extern "C" fn unsignedCharToPrintable(c: u8) -> *const c_char {
    static mut BUF: [u8; 8] = [0u8; 8];
    match c {
        0 => return b"\\0\0".as_ptr() as *const c_char,
        9 => return b"\\t\0".as_ptr() as *const c_char,
        10 => return b"\\n\0".as_ptr() as *const c_char,
        13 => return b"\\r\0".as_ptr() as *const c_char,
        b'\\' => return b"\\\\\0".as_ptr() as *const c_char,
        b'"' => return b"\\\"\0".as_ptr() as *const c_char,
        0x20..=0x7e => {
            BUF[0] = c;
            BUF[1] = 0;
        }
        _ => {
            let hex = b"0123456789ABCDEF";
            BUF[0] = b'\\';
            BUF[1] = b'x';
            BUF[2] = hex[(c >> 4) as usize];
            BUF[3] = hex[(c & 0xf) as usize];
            BUF[4] = 0;
        }
    }
    BUF.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn expat_malloc(
    _parser: XML_Parser,
    size: usize,
    _line: c_int,
) -> *mut c_void {
    malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn expat_realloc(
    _parser: XML_Parser,
    ptr: *mut c_void,
    size: usize,
    _line: c_int,
) -> *mut c_void {
    realloc(ptr, size)
}

#[no_mangle]
pub unsafe extern "C" fn expat_free(_parser: XML_Parser, ptr: *mut c_void, _line: c_int) {
    free(ptr)
}
