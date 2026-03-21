//! FFI bindings to libexpat C library.
//! Used as a reference implementation for testing the Rust port.

#![allow(non_camel_case_types, non_snake_case)]

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Opaque parser type
pub enum XML_ParserStruct {}
pub type XML_Parser = *mut XML_ParserStruct;

// Status/error types
pub type XML_Status = c_uint;
pub const XML_STATUS_ERROR: XML_Status = 0;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;

pub type XML_Error = c_uint;
pub const XML_ERROR_NONE: XML_Error = 0;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;

pub type XML_Char = c_char;
pub type XML_Bool = c_char;
pub type XML_Size = c_ulong;
pub type XML_Index = c_long;

// Handler types
pub type XML_StartElementHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut *const XML_Char)>;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
pub type XML_CharacterDataHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
pub type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char)>;
pub type XML_CommentHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
pub type XML_StartCdataSectionHandler =
    Option<unsafe extern "C" fn(*mut c_void)>;
pub type XML_EndCdataSectionHandler =
    Option<unsafe extern "C" fn(*mut c_void)>;
pub type XML_DefaultHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
pub type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        c_int,
    ),
>;
pub type XML_EndDoctypeDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void)>;
pub type XML_XmlDeclHandler = Option<
    unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, c_int),
>;
pub type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> c_int,
>;

// Parsing status struct
#[repr(C)]
pub struct XML_ParsingStatus {
    pub parsing: c_uint,
    pub finalBuffer: XML_Bool,
}

pub const XML_INITIALIZED: c_uint = 0;
pub const XML_PARSING: c_uint = 1;
pub const XML_FINISHED: c_uint = 2;
pub const XML_SUSPENDED: c_uint = 3;

// Parameter entity parsing
pub type XML_ParamEntityParsing = c_uint;
pub const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;
pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;
pub const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;

extern "C" {
    // Parser lifecycle
    pub fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
    pub fn XML_ParserCreateNS(encoding: *const XML_Char, namespaceSeparator: XML_Char) -> XML_Parser;
    pub fn XML_ParserReset(parser: XML_Parser, encoding: *const XML_Char) -> XML_Bool;
    pub fn XML_ParserFree(parser: XML_Parser);

    // Parsing
    pub fn XML_Parse(parser: XML_Parser, s: *const c_char, len: c_int, isFinal: c_int) -> XML_Status;
    pub fn XML_GetBuffer(parser: XML_Parser, len: c_int) -> *mut c_void;
    pub fn XML_ParseBuffer(parser: XML_Parser, len: c_int, isFinal: c_int) -> XML_Status;

    // Stop/Resume
    pub fn XML_StopParser(parser: XML_Parser, resumable: XML_Bool) -> XML_Status;
    pub fn XML_ResumeParser(parser: XML_Parser) -> XML_Status;

    // Error handling
    pub fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    pub fn XML_ErrorString(code: XML_Error) -> *const XML_Char;

    // Position info
    pub fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
    pub fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size;
    pub fn XML_GetCurrentByteIndex(parser: XML_Parser) -> XML_Index;
    pub fn XML_GetCurrentByteCount(parser: XML_Parser) -> c_int;

    // Status
    pub fn XML_GetParsingStatus(parser: XML_Parser, status: *mut XML_ParsingStatus);

    // Configuration
    pub fn XML_SetEncoding(parser: XML_Parser, encoding: *const XML_Char) -> XML_Status;
    pub fn XML_SetBase(parser: XML_Parser, base: *const XML_Char) -> XML_Status;
    pub fn XML_GetBase(parser: XML_Parser) -> *const XML_Char;
    pub fn XML_SetHashSalt(parser: XML_Parser, hash_salt: c_ulong) -> c_int;
    pub fn XML_SetParamEntityParsing(parser: XML_Parser, parsing: XML_ParamEntityParsing) -> c_int;
    pub fn XML_UseForeignDTD(parser: XML_Parser, useDTD: XML_Bool) -> XML_Error;
    pub fn XML_SetReturnNSTriplet(parser: XML_Parser, do_nst: c_int);
    pub fn XML_SetReparseDeferralEnabled(parser: XML_Parser, enabled: XML_Bool) -> XML_Bool;

    // Handler setters
    pub fn XML_SetUserData(parser: XML_Parser, userData: *mut c_void);
    pub fn XML_SetElementHandler(
        parser: XML_Parser,
        start: XML_StartElementHandler,
        end: XML_EndElementHandler,
    );
    pub fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    pub fn XML_SetProcessingInstructionHandler(
        parser: XML_Parser,
        handler: XML_ProcessingInstructionHandler,
    );
    pub fn XML_SetCommentHandler(parser: XML_Parser, handler: XML_CommentHandler);
    pub fn XML_SetCdataSectionHandler(
        parser: XML_Parser,
        start: XML_StartCdataSectionHandler,
        end: XML_EndCdataSectionHandler,
    );
    pub fn XML_SetDefaultHandler(parser: XML_Parser, handler: XML_DefaultHandler);
    pub fn XML_SetDefaultHandlerExpand(parser: XML_Parser, handler: XML_DefaultHandler);
    pub fn XML_SetDoctypeDeclHandler(
        parser: XML_Parser,
        start: XML_StartDoctypeDeclHandler,
        end: XML_EndDoctypeDeclHandler,
    );
    pub fn XML_SetXmlDeclHandler(parser: XML_Parser, handler: XML_XmlDeclHandler);
    pub fn XML_SetExternalEntityRefHandler(
        parser: XML_Parser,
        handler: XML_ExternalEntityRefHandler,
    );

    // Attribute info
    pub fn XML_GetSpecifiedAttributeCount(parser: XML_Parser) -> c_int;
    pub fn XML_GetIdAttributeIndex(parser: XML_Parser) -> c_int;

    // Version
    pub fn XML_ExpatVersion() -> *const XML_Char;

    // External entity parser
    pub fn XML_ExternalEntityParserCreate(
        parser: XML_Parser,
        context: *const XML_Char,
        encoding: *const XML_Char,
    ) -> XML_Parser;

    // Billion laughs protection
    pub fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
        parser: XML_Parser,
        maximumAmplificationFactor: f32,
    ) -> XML_Bool;
    pub fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
        parser: XML_Parser,
        activationThresholdBytes: c_ulong,
    ) -> XML_Bool;

    pub fn XML_UseParserAsHandlerArg(parser: XML_Parser);
}

/// Safe wrapper around the C XML parser for comparison testing
pub struct CParser {
    parser: XML_Parser,
}

impl CParser {
    pub fn new(encoding: Option<&str>) -> Option<Self> {
        let parser = unsafe {
            match encoding {
                Some(enc) => {
                    let c_enc = std::ffi::CString::new(enc).ok()?;
                    XML_ParserCreate(c_enc.as_ptr())
                }
                None => XML_ParserCreate(std::ptr::null()),
            }
        };
        if parser.is_null() {
            None
        } else {
            Some(CParser { parser })
        }
    }

    pub fn parse(&self, data: &[u8], is_final: bool) -> (u32, u32) {
        let status = unsafe {
            XML_Parse(
                self.parser,
                data.as_ptr() as *const c_char,
                data.len() as c_int,
                if is_final { 1 } else { 0 },
            )
        };
        let error = unsafe { XML_GetErrorCode(self.parser) };
        (status, error)
    }

    pub fn current_line_number(&self) -> u64 {
        unsafe { XML_GetCurrentLineNumber(self.parser) as u64 }
    }

    pub fn current_column_number(&self) -> u64 {
        unsafe { XML_GetCurrentColumnNumber(self.parser) as u64 }
    }

    pub fn current_byte_index(&self) -> i64 {
        unsafe { XML_GetCurrentByteIndex(self.parser) as i64 }
    }

    pub fn reset(&self, encoding: Option<&str>) -> bool {
        unsafe {
            let result = match encoding {
                Some(enc) => {
                    let c_enc = std::ffi::CString::new(enc).ok();
                    match c_enc {
                        Some(c) => XML_ParserReset(self.parser, c.as_ptr()),
                        None => return false,
                    }
                }
                None => XML_ParserReset(self.parser, std::ptr::null()),
            };
            result != 0
        }
    }

    pub fn set_hash_salt(&self, salt: u64) -> bool {
        unsafe { XML_SetHashSalt(self.parser, salt as c_ulong) != 0 }
    }

    pub fn raw_parser(&self) -> XML_Parser {
        self.parser
    }
}

impl Drop for CParser {
    fn drop(&mut self) {
        unsafe { XML_ParserFree(self.parser) }
    }
}
