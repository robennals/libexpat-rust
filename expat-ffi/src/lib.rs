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

use expat_rust::xmlparse::{
    self, ParamEntityParsing, Parser, ParsingState, XmlError, XmlStatus,
};
use std::ffi::{c_char, c_int, c_long, c_ulong, c_void, CStr};
use std::ptr;

// --- Opaque parser handle ---

/// Opaque parser handle exposed to C. Wraps a boxed Rust Parser plus user data pointer.
struct ParserHandle {
    parser: Parser,
    user_data: *mut c_void,
    /// When true, handler_arg is the parser pointer itself (for XML_UseParserAsHandlerArg)
    use_parser_as_handler_arg: bool,
    /// Stored base URI as a null-terminated C string (for XML_GetBase)
    base_c_string: Option<Vec<u8>>,
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
type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char)>;
type XML_CommentHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
type XML_StartCdataSectionHandler = Option<unsafe extern "C" fn(*mut c_void)>;
type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(*mut c_void)>;
type XML_DefaultHandler = Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        c_int,
    ),
>;
type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(*mut c_void)>;
type XML_XmlDeclHandler = Option<
    unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, c_int),
>;
type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> c_int,
>;

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
        parser,
        user_data: ptr::null_mut(),
        use_parser_as_handler_arg: false,
        base_c_string: None,
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

#[no_mangle]
pub unsafe extern "C" fn XML_GetBuffer(parser: XML_Parser, len: c_int) -> *mut c_void {
    if parser.is_null() || len < 0 {
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
    _len: c_int,
    _is_final: c_int,
) -> XML_Status_t {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    // parse_buffer is a stub in the Rust parser — return error
    XML_STATUS_ERROR
}

#[no_mangle]
pub unsafe extern "C" fn XML_StopParser(
    parser: XML_Parser,
    resumable: XML_Bool,
) -> XML_Status_t {
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
    let error = match code {
        0 => XmlError::None,
        1 => XmlError::NoMemory,
        2 => XmlError::Syntax,
        3 => XmlError::NoElements,
        4 => XmlError::InvalidToken,
        5 => XmlError::UnclosedToken,
        6 => XmlError::PartialChar,
        7 => XmlError::TagMismatch,
        8 => XmlError::DuplicateAttribute,
        9 => XmlError::JunkAfterDocElement,
        10 => XmlError::ParamEntityRef,
        11 => XmlError::UndefinedEntity,
        12 => XmlError::RecursiveEntityRef,
        13 => XmlError::AsyncEntity,
        14 => XmlError::BadCharRef,
        15 => XmlError::BinaryEntityRef,
        16 => XmlError::AttributeExternalEntityRef,
        17 => XmlError::MisplacedXmlPi,
        18 => XmlError::UnknownEncoding,
        19 => XmlError::IncorrectEncoding,
        20 => XmlError::UnclosedCdataSection,
        21 => XmlError::ExternalEntityHandling,
        22 => XmlError::NotStandalone,
        23 => XmlError::UnexpectedState,
        24 => XmlError::EntityDeclaredInPe,
        25 => XmlError::FeatureRequiresXmlDtd,
        26 => XmlError::CantChangeFeatureOnceParsing,
        27 => XmlError::UnboundPrefix,
        28 => XmlError::UndeclaringPrefix,
        29 => XmlError::IncompletePe,
        30 => XmlError::XmlDecl,
        31 => XmlError::TextDecl,
        32 => XmlError::Publicid,
        33 => XmlError::Suspended,
        34 => XmlError::NotSuspended,
        35 => XmlError::Aborted,
        36 => XmlError::Finished,
        37 => XmlError::SuspendPe,
        38 => XmlError::ReservedPrefixXml,
        39 => XmlError::ReservedPrefixXmlns,
        40 => XmlError::ReservedNamespaceUri,
        41 => XmlError::InvalidArgument,
        42 => XmlError::NoBuffer,
        43 => XmlError::AmplificationLimitBreach,
        44 => XmlError::NotStarted,
        _ => return ptr::null(),
    };
    let msg = xmlparse::error_string(error);
    msg.as_ptr() as *const XML_Char
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
pub unsafe extern "C" fn XML_GetParsingStatus(
    parser: XML_Parser,
    status: *mut XML_ParsingStatus,
) {
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
        return XML_STATUS_ERROR;
    }
    let enc_str = match CStr::from_ptr(encoding).to_str() {
        Ok(s) => s,
        Err(_) => return XML_STATUS_ERROR,
    };
    status_to_c(handle.parser.set_encoding(enc_str))
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetBase(
    parser: XML_Parser,
    base: *const XML_Char,
) -> XML_Status_t {
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
pub unsafe extern "C" fn XML_UseForeignDTD(
    parser: XML_Parser,
    use_dtd: XML_Bool,
) -> XML_Error_t {
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

    if let Some(start_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_start_element_handler(Some(Box::new(move |name, attrs| {
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
                    user_data,
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

    if let Some(end_fn) = handler {
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

#[no_mangle]
pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    parser: XML_Parser,
    handler: XML_CharacterDataHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;

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

#[no_mangle]
pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    parser: XML_Parser,
    handler: XML_ProcessingInstructionHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_processing_instruction_handler(Some(Box::new(move |target, data| {
                let mut target_bytes: Vec<u8> = target.as_bytes().to_vec();
                target_bytes.push(0);
                let mut data_bytes: Vec<u8> = data.as_bytes().to_vec();
                data_bytes.push(0);
                handler_fn(
                    user_data,
                    target_bytes.as_ptr() as *const XML_Char,
                    data_bytes.as_ptr() as *const XML_Char,
                );
            })));
    } else {
        handle.parser.set_processing_instruction_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCommentHandler(
    parser: XML_Parser,
    handler: XML_CommentHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;

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

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_start_cdata_section_handler(Some(Box::new(move || {
                handler_fn(user_data);
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

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_end_cdata_section_handler(Some(Box::new(move || {
                handler_fn(user_data);
            })));
    } else {
        handle.parser.set_end_cdata_section_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandler(
    parser: XML_Parser,
    handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_default_handler(Some(Box::new(move |data| {
                handler_fn(user_data, data.as_ptr() as *const XML_Char, data.len() as c_int);
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
        let user_data = handle.user_data;
        handle
            .parser
            .set_default_handler_expand(Some(Box::new(move |data| {
                handler_fn(user_data, data.as_ptr() as *const XML_Char, data.len() as c_int);
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

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_start_doctype_decl_handler(Some(Box::new(
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

                    handler_fn(
                        user_data,
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

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
        handle
            .parser
            .set_end_doctype_decl_handler(Some(Box::new(move || {
                handler_fn(user_data);
            })));
    } else {
        handle.parser.set_end_doctype_decl_handler(None);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetXmlDeclHandler(
    parser: XML_Parser,
    handler: XML_XmlDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    let handle = &mut *parser;

    if let Some(handler_fn) = handler {
        let user_data = handle.user_data;
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

                handler_fn(user_data, version_ptr, encoding_ptr, standalone_int);
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
        handle
            .parser
            .set_external_entity_ref_handler(Some(Box::new(
                move |context, base, system_id, public_id| {
                    let mut ctx_bytes: Vec<u8> = context.as_bytes().to_vec();
                    ctx_bytes.push(0);

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

                    let result = handler_fn(
                        parser_ptr,
                        ctx_bytes.as_ptr() as *const XML_Char,
                        base_ptr,
                        sysid_ptr,
                        pubid_ptr,
                    );
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
    let handle = &*parser;

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
        Some(ext_parser) => new_handle(ext_parser),
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

static VERSION_STRING: &[u8] = b"expat-rust_0.1.0\0";

#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersion() -> *const XML_Char {
    VERSION_STRING.as_ptr() as *const XML_Char
}
