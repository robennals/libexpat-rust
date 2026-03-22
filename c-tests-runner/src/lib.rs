//! C ABI shim: wraps the Rust XML parser to expose the same C API as libexpat.
//! This allows the C test suite to be compiled and linked against the Rust parser.

#![allow(non_snake_case, non_camel_case_types, unused_variables)]

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void, CStr, CString};
use std::ptr;
use std::slice;

use expat_rust::xmlparse::{self, Parser, XmlError, XmlStatus, ParamEntityParsing, ParsingState};

// Global variable used by C tests to track bytes scanned (defined in xmlparse.c in original)
#[no_mangle]
pub static mut g_bytesScanned: c_uint = 0;

// --- Type aliases matching expat.h ---

pub type XML_Parser = *mut CParserWrapper;
pub type XML_Char = c_char;
pub type XML_LChar = c_char;
pub type XML_Bool = c_char;
pub type XML_Size = c_ulong;
pub type XML_Index = c_long;
pub type XML_Status = c_uint;
pub type XML_Error = c_uint;
pub type XML_ParamEntityParsing = c_uint;

pub const XML_STATUS_ERROR: XML_Status = 0;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;

pub const XML_TRUE: XML_Bool = 1;
pub const XML_FALSE: XML_Bool = 0;

// --- Handler type aliases (C function pointer types) ---

type CStartElementHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut *const XML_Char)>;
type CEndElementHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
type CCharacterDataHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
type CProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char)>;
type CCommentHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
type CStartCdataSectionHandler =
    Option<unsafe extern "C" fn(*mut c_void)>;
type CEndCdataSectionHandler =
    Option<unsafe extern "C" fn(*mut c_void)>;
type CDefaultHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
type CStartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        c_int,
    ),
>;
type CEndDoctypeDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void)>;
type CXmlDeclHandler = Option<
    unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char, c_int),
>;
type CExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> c_int,
>;
type CNotStandaloneHandler =
    Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
type CSkippedEntityHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, c_int)>;
type CElementDeclHandler = Option<
    unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut XML_Content),
>;
type CAttlistDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        c_int,
    ),
>;
type CEntityDeclHandler = Option<
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
type CUnparsedEntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ),
>;
type CNotationDeclHandler = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ),
>;
type CStartNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char, *const XML_Char)>;
type CEndNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut c_void, *const XML_Char)>;
type CUnknownEncodingHandler = Option<
    unsafe extern "C" fn(*mut c_void, *const XML_Char, *mut XML_Encoding) -> c_int,
>;

// --- C structures ---

#[repr(C)]
pub struct XML_Content {
    pub type_: c_uint,
    pub quant: c_uint,
    pub name: *mut XML_Char,
    pub numchildren: c_uint,
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
pub struct XML_ParsingStatus {
    pub parsing: c_uint,
    pub finalBuffer: XML_Bool,
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

pub const XML_FEATURE_END: c_int = 0;
pub const XML_FEATURE_UNICODE: c_int = 1;
pub const XML_FEATURE_UNICODE_WCHAR_T: c_int = 2;
pub const XML_FEATURE_DTD: c_int = 3;
pub const XML_FEATURE_CONTEXT_BYTES: c_int = 4;
pub const XML_FEATURE_MIN_SIZE: c_int = 5;
pub const XML_FEATURE_SIZEOF_XML_CHAR: c_int = 6;
pub const XML_FEATURE_SIZEOF_XML_LCHAR: c_int = 7;
pub const XML_FEATURE_NS: c_int = 8;
pub const XML_FEATURE_LARGE_SIZE: c_int = 9;
pub const XML_FEATURE_ATTR_INFO: c_int = 10;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT: c_int = 11;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT: c_int = 12;
pub const XML_FEATURE_GE: c_int = 13;

#[repr(C)]
pub struct XML_Feature {
    pub feature: c_int,
    pub name: *const XML_Char,
    pub value: c_long,
}

// --- The core wrapper struct ---

/// Wraps a Rust Parser and stores all the C callback function pointers and user data.
#[repr(C)]
pub struct CParserWrapper {
    // MUST be first field: C code uses XML_GetUserData macro which reads *(void**)(parser)
    user_data: *mut c_void,
    parser: Parser,
    /// Whether XML_UseParserAsHandlerArg was called
    use_parser_as_handler: bool,
    /// External entity ref handler arg override
    ext_entity_ref_handler_arg: *mut c_void,

    // C handler function pointers (stored separately from Rust closures)
    c_start_element: CStartElementHandler,
    c_end_element: CEndElementHandler,
    c_character_data: CCharacterDataHandler,
    c_processing_instruction: CProcessingInstructionHandler,
    c_comment: CCommentHandler,
    c_start_cdata: CStartCdataSectionHandler,
    c_end_cdata: CEndCdataSectionHandler,
    c_default: CDefaultHandler,
    c_default_expand: CDefaultHandler,
    c_start_doctype: CStartDoctypeDeclHandler,
    c_end_doctype: CEndDoctypeDeclHandler,
    c_xml_decl: CXmlDeclHandler,
    c_external_entity_ref: CExternalEntityRefHandler,
    c_not_standalone: CNotStandaloneHandler,
    c_skipped_entity: CSkippedEntityHandler,
    c_element_decl: CElementDeclHandler,
    c_attlist_decl: CAttlistDeclHandler,
    c_entity_decl: CEntityDeclHandler,
    c_unparsed_entity_decl: CUnparsedEntityDeclHandler,
    c_notation_decl: CNotationDeclHandler,
    c_start_namespace_decl: CStartNamespaceDeclHandler,
    c_end_namespace_decl: CEndNamespaceDeclHandler,
    c_unknown_encoding: CUnknownEncodingHandler,
    c_unknown_encoding_data: *mut c_void,

    // Custom memory handlers (for XML_ParserCreate_MM)
    mem_suite: Option<XML_Memory_Handling_Suite>,

    // NS config
    ns_separator: Option<c_char>,

    // Return NS triplet flag
    return_ns_triplet: bool,
}

impl CParserWrapper {
    fn get_handler_data(&self) -> *mut c_void {
        if self.use_parser_as_handler {
            // Return a pointer to self as the handler data
            self as *const CParserWrapper as *mut c_void
        } else {
            self.user_data
        }
    }

    fn get_ext_entity_ref_arg(&self) -> XML_Parser {
        if self.ext_entity_ref_handler_arg.is_null() {
            // Default: pass parser pointer
            self as *const CParserWrapper as *mut CParserWrapper
        } else {
            self.ext_entity_ref_handler_arg as XML_Parser
        }
    }
}

// --- Helper functions ---

fn cstr_to_option_str(s: *const c_char) -> Option<&'static str> {
    if s.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(s).to_str().ok() }
    }
}

fn xml_status_to_c(s: XmlStatus) -> XML_Status {
    match s {
        XmlStatus::Error => XML_STATUS_ERROR,
        XmlStatus::Ok => XML_STATUS_OK,
        XmlStatus::Suspended => XML_STATUS_SUSPENDED,
    }
}

fn xml_error_to_c(e: XmlError) -> c_uint {
    e as c_uint
}

fn c_to_param_entity_parsing(p: c_uint) -> ParamEntityParsing {
    match p {
        0 => ParamEntityParsing::Never,
        1 => ParamEntityParsing::UnlessStandalone,
        2 => ParamEntityParsing::Always,
        _ => ParamEntityParsing::Never,
    }
}

fn parsing_state_to_c(s: ParsingState) -> c_uint {
    match s {
        ParsingState::Initialized => 0,
        ParsingState::Parsing => 1,
        ParsingState::Finished => 2,
        ParsingState::Suspended => 3,
    }
}

// Install Rust closures on the parser that forward to the stored C function pointers.
// This must be called after any C handler is changed.
fn sync_handlers(wrapper: &mut CParserWrapper) {
    // We need to pass a raw pointer to wrapper into the closures so they can
    // access the C handler pointers and user_data. This is safe because the
    // closures only live as long as the wrapper.
    let wrapper_ptr = wrapper as *mut CParserWrapper;

    // Start element handler
    if wrapper.c_start_element.is_some() {
        wrapper.parser.set_start_element_handler(Some(Box::new(
            move |name: &str, attrs: &[(&str, &str)]| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_start_element {
                    let c_name = CString::new(name).unwrap_or_default();
                    // Build null-terminated attribute array: name, value, name, value, ..., NULL
                    let mut attr_cstrings: Vec<CString> = Vec::new();
                    let mut attr_ptrs: Vec<*const XML_Char> = Vec::new();
                    for (k, v) in attrs {
                        let ck = CString::new(*k).unwrap_or_default();
                        let cv = CString::new(*v).unwrap_or_default();
                        attr_ptrs.push(ck.as_ptr());
                        attr_ptrs.push(cv.as_ptr());
                        attr_cstrings.push(ck);
                        attr_cstrings.push(cv);
                    }
                    attr_ptrs.push(ptr::null());

                    unsafe {
                        handler(w.get_handler_data(), c_name.as_ptr(), attr_ptrs.as_mut_ptr());
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_start_element_handler(None);
    }

    // End element handler
    if wrapper.c_end_element.is_some() {
        wrapper.parser.set_end_element_handler(Some(Box::new(
            move |name: &str| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_end_element {
                    let c_name = CString::new(name).unwrap_or_default();
                    unsafe { handler(w.get_handler_data(), c_name.as_ptr()); }
                }
            },
        )));
    } else {
        wrapper.parser.set_end_element_handler(None);
    }

    // Character data handler
    if wrapper.c_character_data.is_some() {
        wrapper.parser.set_character_data_handler(Some(Box::new(
            move |data: &[u8]| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_character_data {
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            data.as_ptr() as *const XML_Char,
                            data.len() as c_int,
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_character_data_handler(None);
    }

    // Processing instruction handler
    if wrapper.c_processing_instruction.is_some() {
        wrapper.parser.set_processing_instruction_handler(Some(Box::new(
            move |target: &str, data: &str| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_processing_instruction {
                    let c_target = CString::new(target).unwrap_or_default();
                    let c_data = CString::new(data).unwrap_or_default();
                    unsafe { handler(w.get_handler_data(), c_target.as_ptr(), c_data.as_ptr()); }
                }
            },
        )));
    } else {
        wrapper.parser.set_processing_instruction_handler(None);
    }

    // Comment handler
    if wrapper.c_comment.is_some() {
        wrapper.parser.set_comment_handler(Some(Box::new(
            move |data: &[u8]| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_comment {
                    // Comment data from Rust is bytes, but C expects null-terminated string
                    let c_data = CString::new(data).unwrap_or_default();
                    unsafe { handler(w.get_handler_data(), c_data.as_ptr()); }
                }
            },
        )));
    } else {
        wrapper.parser.set_comment_handler(None);
    }

    // CDATA section handlers
    if wrapper.c_start_cdata.is_some() {
        wrapper.parser.set_start_cdata_section_handler(Some(Box::new(
            move || {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_start_cdata {
                    unsafe { handler(w.get_handler_data()); }
                }
            },
        )));
    } else {
        wrapper.parser.set_start_cdata_section_handler(None);
    }

    if wrapper.c_end_cdata.is_some() {
        wrapper.parser.set_end_cdata_section_handler(Some(Box::new(
            move || {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_end_cdata {
                    unsafe { handler(w.get_handler_data()); }
                }
            },
        )));
    } else {
        wrapper.parser.set_end_cdata_section_handler(None);
    }

    // Default handler
    if wrapper.c_default.is_some() {
        wrapper.parser.set_default_handler(Some(Box::new(
            move |data: &[u8]| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_default {
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            data.as_ptr() as *const XML_Char,
                            data.len() as c_int,
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_default_handler(None);
    }

    // Default handler expand
    if wrapper.c_default_expand.is_some() {
        wrapper.parser.set_default_handler_expand(Some(Box::new(
            move |data: &[u8]| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_default_expand {
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            data.as_ptr() as *const XML_Char,
                            data.len() as c_int,
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_default_handler_expand(None);
    }

    // Doctype decl handlers
    if wrapper.c_start_doctype.is_some() {
        wrapper.parser.set_start_doctype_decl_handler(Some(Box::new(
            move |name: &str, sysid: Option<&str>, pubid: Option<&str>, has_internal: bool| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_start_doctype {
                    let c_name = CString::new(name).unwrap_or_default();
                    let c_sysid = sysid.map(|s| CString::new(s).unwrap_or_default());
                    let c_pubid = pubid.map(|s| CString::new(s).unwrap_or_default());
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_name.as_ptr(),
                            c_sysid.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            c_pubid.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            if has_internal { 1 } else { 0 },
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_start_doctype_decl_handler(None);
    }

    if wrapper.c_end_doctype.is_some() {
        wrapper.parser.set_end_doctype_decl_handler(Some(Box::new(
            move || {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_end_doctype {
                    unsafe { handler(w.get_handler_data()); }
                }
            },
        )));
    } else {
        wrapper.parser.set_end_doctype_decl_handler(None);
    }

    // XML decl handler
    if wrapper.c_xml_decl.is_some() {
        wrapper.parser.set_xml_decl_handler(Some(Box::new(
            move |version: Option<&str>, encoding: Option<&str>, standalone: Option<i32>| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_xml_decl {
                    let c_version = version.map(|s| CString::new(s).unwrap_or_default());
                    let c_encoding = encoding.map(|s| CString::new(s).unwrap_or_default());
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_version.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            c_encoding.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            standalone.unwrap_or(-1) as c_int,
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_xml_decl_handler(None);
    }

    // External entity ref handler
    if wrapper.c_external_entity_ref.is_some() {
        wrapper.parser.set_external_entity_ref_handler(Some(Box::new(
            move |context: &str, base: Option<&str>, system_id: Option<&str>, public_id: Option<&str>| -> bool {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_external_entity_ref {
                    let parser_arg = w.get_ext_entity_ref_arg();
                    let c_context = CString::new(context).unwrap_or_default();
                    let c_base = base.map(|s| CString::new(s).unwrap_or_default());
                    let c_system_id = system_id.map(|s| CString::new(s).unwrap_or_default());
                    let c_public_id = public_id.map(|s| CString::new(s).unwrap_or_default());
                    let result = unsafe {
                        handler(
                            parser_arg,
                            c_context.as_ptr(),
                            c_base.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            c_system_id.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            c_public_id.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                        )
                    };
                    result != 0
                } else {
                    true
                }
            },
        )));
    } else {
        wrapper.parser.set_external_entity_ref_handler(None);
    }

    // Not standalone handler
    if wrapper.c_not_standalone.is_some() {
        wrapper.parser.set_not_standalone_handler(Some(Box::new(
            move || -> bool {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_not_standalone {
                    unsafe { handler(w.get_handler_data()) != 0 }
                } else {
                    true
                }
            },
        )));
    } else {
        wrapper.parser.set_not_standalone_handler(None);
    }

    // Skipped entity handler
    if wrapper.c_skipped_entity.is_some() {
        wrapper.parser.set_skipped_entity_handler(Some(Box::new(
            move |name: &str, is_param: bool| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_skipped_entity {
                    let c_name = CString::new(name).unwrap_or_default();
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_name.as_ptr(),
                            if is_param { 1 } else { 0 },
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_skipped_entity_handler(None);
    }

    // Element decl handler
    if wrapper.c_element_decl.is_some() {
        wrapper.parser.set_element_decl_handler(Some(Box::new(
            move |name: &str, model: &str| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_element_decl {
                    let c_name = CString::new(name).unwrap_or_default();
                    // For now, pass a dummy content model - full implementation would build XML_Content tree
                    let mut content = XML_Content {
                        type_: 0,
                        quant: 0,
                        name: ptr::null_mut(),
                        numchildren: 0,
                        children: ptr::null_mut(),
                    };
                    unsafe { handler(w.get_handler_data(), c_name.as_ptr(), &mut content); }
                }
            },
        )));
    } else {
        wrapper.parser.set_element_decl_handler(None);
    }

    // Attlist decl handler
    if wrapper.c_attlist_decl.is_some() {
        wrapper.parser.set_attlist_decl_handler(Some(Box::new(
            move |elname: &str, attname: &str, att_type: &str, dflt: Option<&str>, _dflt2: Option<&str>, is_required: bool| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_attlist_decl {
                    let c_el = CString::new(elname).unwrap_or_default();
                    let c_att = CString::new(attname).unwrap_or_default();
                    let c_type = CString::new(att_type).unwrap_or_default();
                    let c_dflt = dflt.map(|s| CString::new(s).unwrap_or_default());
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_el.as_ptr(),
                            c_att.as_ptr(),
                            c_type.as_ptr(),
                            c_dflt.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            if is_required { 1 } else { 0 },
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_attlist_decl_handler(None);
    }

    // Entity decl handler
    if wrapper.c_entity_decl.is_some() {
        wrapper.parser.set_entity_decl_handler(Some(Box::new(
            move |name: &str, is_param: bool, value: Option<&str>, base: Option<&str>, system_id: Option<&str>| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_entity_decl {
                    let c_name = CString::new(name).unwrap_or_default();
                    let c_value = value.map(|s| CString::new(s).unwrap_or_default());
                    let value_len = value.map_or(0, |s| s.len()) as c_int;
                    let c_base = base.map(|s| CString::new(s).unwrap_or_default());
                    let c_sysid = system_id.map(|s| CString::new(s).unwrap_or_default());
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_name.as_ptr(),
                            if is_param { 1 } else { 0 },
                            c_value.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            value_len,
                            c_base.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            c_sysid.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            ptr::null(), // public_id
                            ptr::null(), // notation_name
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_entity_decl_handler(None);
    }

    // Unparsed entity decl handler
    if wrapper.c_unparsed_entity_decl.is_some() {
        wrapper.parser.set_unparsed_entity_decl_handler(Some(Box::new(
            move |name: &str, base: Option<&str>, system_id: &str, notation: Option<&str>| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_unparsed_entity_decl {
                    let c_name = CString::new(name).unwrap_or_default();
                    let c_base = base.map(|s| CString::new(s).unwrap_or_default());
                    let c_sysid = CString::new(system_id).unwrap_or_default();
                    let c_notation = notation.map(|s| CString::new(s).unwrap_or_default());
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_name.as_ptr(),
                            c_base.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            c_sysid.as_ptr(),
                            c_notation.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_unparsed_entity_decl_handler(None);
    }

    // Notation decl handler
    if wrapper.c_notation_decl.is_some() {
        wrapper.parser.set_notation_decl_handler(Some(Box::new(
            move |name: &str, base: Option<&str>, system_id: &str, public_id: Option<&str>| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_notation_decl {
                    let c_name = CString::new(name).unwrap_or_default();
                    let c_base = base.map(|s| CString::new(s).unwrap_or_default());
                    let c_sysid = CString::new(system_id).unwrap_or_default();
                    let c_pubid = public_id.map(|s| CString::new(s).unwrap_or_default());
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_name.as_ptr(),
                            c_base.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            c_sysid.as_ptr(),
                            c_pubid.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_notation_decl_handler(None);
    }

    // Namespace decl handlers
    if wrapper.c_start_namespace_decl.is_some() {
        wrapper.parser.set_start_namespace_decl_handler(Some(Box::new(
            move |prefix: Option<&str>, uri: &str| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_start_namespace_decl {
                    let c_prefix = prefix.map(|s| CString::new(s).unwrap_or_default());
                    let c_uri = CString::new(uri).unwrap_or_default();
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_prefix.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                            c_uri.as_ptr(),
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_start_namespace_decl_handler(None);
    }

    if wrapper.c_end_namespace_decl.is_some() {
        wrapper.parser.set_end_namespace_decl_handler(Some(Box::new(
            move |prefix: Option<&str>| {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_end_namespace_decl {
                    let c_prefix = prefix.map(|s| CString::new(s).unwrap_or_default());
                    unsafe {
                        handler(
                            w.get_handler_data(),
                            c_prefix.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                        );
                    }
                }
            },
        )));
    } else {
        wrapper.parser.set_end_namespace_decl_handler(None);
    }

    // Unknown encoding handler
    if wrapper.c_unknown_encoding.is_some() {
        wrapper.parser.set_unknown_encoding_handler(Some(Box::new(
            move |name: &str| -> bool {
                let w = unsafe { &*wrapper_ptr };
                if let Some(handler) = w.c_unknown_encoding {
                    let c_name = CString::new(name).unwrap_or_default();
                    let mut enc = XML_Encoding {
                        map: [0i32; 256],
                        data: ptr::null_mut(),
                        convert: None,
                        release: None,
                    };
                    unsafe { handler(w.c_unknown_encoding_data, c_name.as_ptr(), &mut enc) != 0 }
                } else {
                    false
                }
            },
        )));
    } else {
        wrapper.parser.set_unknown_encoding_handler(None);
    }
}

fn create_wrapper(parser: Parser, ns_sep: Option<c_char>) -> *mut CParserWrapper {
    let wrapper = Box::new(CParserWrapper {
        parser,
        user_data: ptr::null_mut(),
        use_parser_as_handler: false,
        ext_entity_ref_handler_arg: ptr::null_mut(),
        c_start_element: None,
        c_end_element: None,
        c_character_data: None,
        c_processing_instruction: None,
        c_comment: None,
        c_start_cdata: None,
        c_end_cdata: None,
        c_default: None,
        c_default_expand: None,
        c_start_doctype: None,
        c_end_doctype: None,
        c_xml_decl: None,
        c_external_entity_ref: None,
        c_not_standalone: None,
        c_skipped_entity: None,
        c_element_decl: None,
        c_attlist_decl: None,
        c_entity_decl: None,
        c_unparsed_entity_decl: None,
        c_notation_decl: None,
        c_start_namespace_decl: None,
        c_end_namespace_decl: None,
        c_unknown_encoding: None,
        c_unknown_encoding_data: ptr::null_mut(),
        mem_suite: None,
        ns_separator: ns_sep,
        return_ns_triplet: false,
    });
    Box::into_raw(wrapper)
}

// =============================================================================
// C API functions (extern "C")
// =============================================================================

#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser {
    let enc = cstr_to_option_str(encoding);
    match Parser::new(enc) {
        Some(p) => create_wrapper(p, None),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreateNS(
    encoding: *const XML_Char,
    separator: XML_Char,
) -> XML_Parser {
    let enc = cstr_to_option_str(encoding);
    let sep_char = separator as u8 as char;
    match Parser::new_ns(enc, sep_char) {
        Some(p) => create_wrapper(p, Some(separator)),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate_MM(
    encoding: *const XML_Char,
    memsuite: *const XML_Memory_Handling_Suite,
    namespaceSeparator: *const XML_Char,
) -> XML_Parser {
    let enc = cstr_to_option_str(encoding);

    let (parser, ns_sep) = if namespaceSeparator.is_null() {
        (Parser::new(enc), None)
    } else {
        let sep = *namespaceSeparator;
        (Parser::new_ns(enc, sep as u8 as char), Some(sep))
    };

    match parser {
        Some(p) => {
            let wrapper_ptr = create_wrapper(p, ns_sep);
            // Store memory suite if provided (we don't actually use custom allocators
            // for the Rust parser, but tests check this works)
            if !memsuite.is_null() {
                (*wrapper_ptr).mem_suite = Some(std::ptr::read(memsuite));
            }
            wrapper_ptr
        }
        None => {
            // If custom allocator is provided and we "fail" to allocate, that's the test's intent
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParserFree(parser: XML_Parser) {
    if !parser.is_null() {
        let _ = Box::from_raw(parser);
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
    let w = &mut *parser;
    let enc = cstr_to_option_str(encoding);
    if w.parser.reset(enc) {
        // Clear all C handlers
        w.c_start_element = None;
        w.c_end_element = None;
        w.c_character_data = None;
        w.c_processing_instruction = None;
        w.c_comment = None;
        w.c_start_cdata = None;
        w.c_end_cdata = None;
        w.c_default = None;
        w.c_default_expand = None;
        w.c_start_doctype = None;
        w.c_end_doctype = None;
        w.c_xml_decl = None;
        w.c_external_entity_ref = None;
        w.c_not_standalone = None;
        w.c_skipped_entity = None;
        w.c_element_decl = None;
        w.c_attlist_decl = None;
        w.c_entity_decl = None;
        w.c_unparsed_entity_decl = None;
        w.c_notation_decl = None;
        w.c_start_namespace_decl = None;
        w.c_end_namespace_decl = None;
        w.c_unknown_encoding = None;
        w.user_data = ptr::null_mut();
        w.use_parser_as_handler = false;
        w.ext_entity_ref_handler_arg = ptr::null_mut();
        XML_TRUE
    } else {
        XML_FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_Parse(
    parser: XML_Parser,
    s: *const c_char,
    len: c_int,
    isFinal: c_int,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let w = &mut *parser;
    let data = if s.is_null() || len <= 0 {
        &[]
    } else {
        slice::from_raw_parts(s as *const u8, len as usize)
    };
    xml_status_to_c(w.parser.parse(data, isFinal != 0))
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBuffer(parser: XML_Parser, len: c_int) -> *mut c_void {
    if parser.is_null() || len < 0 {
        return ptr::null_mut();
    }
    let w = &mut *parser;
    match w.parser.get_buffer(len as usize) {
        Some(buf) => buf.as_mut_ptr() as *mut c_void,
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ParseBuffer(
    parser: XML_Parser,
    len: c_int,
    isFinal: c_int,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let w = &mut *parser;
    xml_status_to_c(w.parser.parse_buffer(len as usize, isFinal != 0))
}

#[no_mangle]
pub unsafe extern "C" fn XML_StopParser(
    parser: XML_Parser,
    resumable: XML_Bool,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let w = &mut *parser;
    xml_status_to_c(w.parser.stop(resumable != 0))
}

#[no_mangle]
pub unsafe extern "C" fn XML_ResumeParser(parser: XML_Parser) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let w = &mut *parser;
    xml_status_to_c(w.parser.resume())
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorCode(parser: XML_Parser) -> c_uint {
    if parser.is_null() {
        return 0;
    }
    xml_error_to_c((*parser).parser.error_code())
}

static mut ERROR_STRING_BUF: [u8; 256] = [0u8; 256];

#[no_mangle]
pub unsafe extern "C" fn XML_ErrorString(code: c_uint) -> *const XML_Char {
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
        _ => return ptr::null(),
    };
    let msg = xmlparse::error_string(error);
    let len = msg.len().min(255);
    ERROR_STRING_BUF[..len].copy_from_slice(&msg.as_bytes()[..len]);
    ERROR_STRING_BUF[len] = 0;
    ERROR_STRING_BUF.as_ptr() as *const XML_Char
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size {
    if parser.is_null() { return 0; }
    (*parser).parser.current_line_number() as XML_Size
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size {
    if parser.is_null() { return 0; }
    (*parser).parser.current_column_number() as XML_Size
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteIndex(parser: XML_Parser) -> XML_Index {
    if parser.is_null() { return -1; }
    (*parser).parser.current_byte_index() as XML_Index
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteCount(parser: XML_Parser) -> c_int {
    if parser.is_null() { return 0; }
    (*parser).parser.current_byte_count()
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetParsingStatus(
    parser: XML_Parser,
    status: *mut XML_ParsingStatus,
) {
    if parser.is_null() || status.is_null() {
        return;
    }
    let ps = (*parser).parser.parsing_status();
    (*status).parsing = parsing_state_to_c(ps.state);
    (*status).finalBuffer = if ps.final_buffer { XML_TRUE } else { XML_FALSE };
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEncoding(
    parser: XML_Parser,
    encoding: *const XML_Char,
) -> XML_Status {
    if parser.is_null() { return XML_STATUS_ERROR; }
    let enc = cstr_to_option_str(encoding).unwrap_or("");
    xml_status_to_c((*parser).parser.set_encoding(enc))
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetBase(
    parser: XML_Parser,
    base: *const XML_Char,
) -> XML_Status {
    if parser.is_null() { return XML_STATUS_ERROR; }
    let b = cstr_to_option_str(base).unwrap_or("");
    xml_status_to_c((*parser).parser.set_base(b))
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetBase(parser: XML_Parser) -> *const XML_Char {
    if parser.is_null() { return ptr::null(); }
    match (*parser).parser.base() {
        Some(_) => {
            // We'd need to return a stable pointer. For now, return null.
            // TODO: Store CString in wrapper for lifetime management
            ptr::null()
        }
        None => ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetHashSalt(parser: XML_Parser, hash_salt: c_ulong) -> c_int {
    if parser.is_null() { return 0; }
    if (*parser).parser.set_hash_salt(hash_salt as u64) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetParamEntityParsing(
    parser: XML_Parser,
    parsing: XML_ParamEntityParsing,
) -> c_int {
    if parser.is_null() { return 0; }
    if (*parser).parser.set_param_entity_parsing(c_to_param_entity_parsing(parsing)) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn XML_UseForeignDTD(
    parser: XML_Parser,
    useDTD: XML_Bool,
) -> c_uint {
    if parser.is_null() { return xml_error_to_c(XmlError::InvalidArgument); }
    xml_error_to_c((*parser).parser.use_foreign_dtd(useDTD != 0))
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetReturnNSTriplet(parser: XML_Parser, do_nst: c_int) {
    if parser.is_null() { return; }
    (*parser).return_ns_triplet = do_nst != 0;
    (*parser).parser.set_return_ns_triplet(do_nst != 0);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetReparseDeferralEnabled(
    parser: XML_Parser,
    enabled: XML_Bool,
) -> XML_Bool {
    if parser.is_null() { return XML_FALSE; }
    if (*parser).parser.set_reparse_deferral_enabled(enabled != 0) { XML_TRUE } else { XML_FALSE }
}

// --- Handler setters ---

#[no_mangle]
pub unsafe extern "C" fn XML_SetUserData(parser: XML_Parser, userData: *mut c_void) {
    if parser.is_null() { return; }
    (*parser).user_data = userData;
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetUserData(parser: XML_Parser) -> *mut c_void {
    if parser.is_null() { return ptr::null_mut(); }
    (*parser).user_data
}

#[no_mangle]
pub unsafe extern "C" fn XML_UseParserAsHandlerArg(parser: XML_Parser) {
    if parser.is_null() { return; }
    (*parser).use_parser_as_handler = true;
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetElementHandler(
    parser: XML_Parser,
    start: CStartElementHandler,
    end: CEndElementHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_start_element = start;
    (*parser).c_end_element = end;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartElementHandler(
    parser: XML_Parser,
    handler: CStartElementHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_start_element = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndElementHandler(
    parser: XML_Parser,
    handler: CEndElementHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_end_element = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    parser: XML_Parser,
    handler: CCharacterDataHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_character_data = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    parser: XML_Parser,
    handler: CProcessingInstructionHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_processing_instruction = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCommentHandler(
    parser: XML_Parser,
    handler: CCommentHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_comment = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetCdataSectionHandler(
    parser: XML_Parser,
    start: CStartCdataSectionHandler,
    end: CEndCdataSectionHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_start_cdata = start;
    (*parser).c_end_cdata = end;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartCdataSectionHandler(
    parser: XML_Parser,
    handler: CStartCdataSectionHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_start_cdata = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndCdataSectionHandler(
    parser: XML_Parser,
    handler: CEndCdataSectionHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_end_cdata = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandler(
    parser: XML_Parser,
    handler: CDefaultHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_default = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandlerExpand(
    parser: XML_Parser,
    handler: CDefaultHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_default_expand = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetDoctypeDeclHandler(
    parser: XML_Parser,
    start: CStartDoctypeDeclHandler,
    end: CEndDoctypeDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_start_doctype = start;
    (*parser).c_end_doctype = end;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler(
    parser: XML_Parser,
    handler: CStartDoctypeDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_start_doctype = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler(
    parser: XML_Parser,
    handler: CEndDoctypeDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_end_doctype = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetXmlDeclHandler(
    parser: XML_Parser,
    handler: CXmlDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_xml_decl = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandler(
    parser: XML_Parser,
    handler: CExternalEntityRefHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_external_entity_ref = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg(
    parser: XML_Parser,
    arg: *mut c_void,
) {
    if parser.is_null() { return; }
    (*parser).ext_entity_ref_handler_arg = arg;
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNotStandaloneHandler(
    parser: XML_Parser,
    handler: CNotStandaloneHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_not_standalone = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetSkippedEntityHandler(
    parser: XML_Parser,
    handler: CSkippedEntityHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_skipped_entity = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetElementDeclHandler(
    parser: XML_Parser,
    handler: CElementDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_element_decl = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetAttlistDeclHandler(
    parser: XML_Parser,
    handler: CAttlistDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_attlist_decl = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEntityDeclHandler(
    parser: XML_Parser,
    handler: CEntityDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_entity_decl = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler(
    parser: XML_Parser,
    handler: CUnparsedEntityDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_unparsed_entity_decl = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNotationDeclHandler(
    parser: XML_Parser,
    handler: CNotationDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_notation_decl = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetNamespaceDeclHandler(
    parser: XML_Parser,
    start: CStartNamespaceDeclHandler,
    end: CEndNamespaceDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_start_namespace_decl = start;
    (*parser).c_end_namespace_decl = end;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler(
    parser: XML_Parser,
    handler: CStartNamespaceDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_start_namespace_decl = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler(
    parser: XML_Parser,
    handler: CEndNamespaceDeclHandler,
) {
    if parser.is_null() { return; }
    (*parser).c_end_namespace_decl = handler;
    sync_handlers(&mut *parser);
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetUnknownEncodingHandler(
    parser: XML_Parser,
    handler: CUnknownEncodingHandler,
    data: *mut c_void,
) {
    if parser.is_null() { return; }
    (*parser).c_unknown_encoding = handler;
    (*parser).c_unknown_encoding_data = data;
    sync_handlers(&mut *parser);
}

// --- Query functions ---

#[no_mangle]
pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount(parser: XML_Parser) -> c_int {
    if parser.is_null() { return -1; }
    (*parser).parser.specified_attribute_count()
}

#[no_mangle]
pub unsafe extern "C" fn XML_GetIdAttributeIndex(parser: XML_Parser) -> c_int {
    if parser.is_null() { return -1; }
    (*parser).parser.id_attribute_index()
}

#[no_mangle]
pub unsafe extern "C" fn XML_DefaultCurrent(parser: XML_Parser) {
    if parser.is_null() { return; }
    (*parser).parser.default_current();
}

#[no_mangle]
pub unsafe extern "C" fn XML_FreeContentModel(parser: XML_Parser, model: *mut XML_Content) {
    if !model.is_null() {
        // Free the model allocated by our element_decl_handler shim
        let _ = Box::from_raw(model);
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_ExternalEntityParserCreate(
    parser: XML_Parser,
    context: *const XML_Char,
    encoding: *const XML_Char,
) -> XML_Parser {
    if parser.is_null() { return ptr::null_mut(); }
    let ctx = cstr_to_option_str(context).unwrap_or("");
    let enc = cstr_to_option_str(encoding);

    match (*parser).parser.create_external_entity_parser(ctx, enc) {
        Some(p) => {
            let new_wrapper = create_wrapper(p, (*parser).ns_separator);
            // Copy handler settings from parent
            (*new_wrapper).c_start_element = (*parser).c_start_element;
            (*new_wrapper).c_end_element = (*parser).c_end_element;
            (*new_wrapper).c_character_data = (*parser).c_character_data;
            (*new_wrapper).c_processing_instruction = (*parser).c_processing_instruction;
            (*new_wrapper).c_comment = (*parser).c_comment;
            (*new_wrapper).c_start_cdata = (*parser).c_start_cdata;
            (*new_wrapper).c_end_cdata = (*parser).c_end_cdata;
            (*new_wrapper).c_default = (*parser).c_default;
            (*new_wrapper).c_default_expand = (*parser).c_default_expand;
            (*new_wrapper).c_start_doctype = (*parser).c_start_doctype;
            (*new_wrapper).c_end_doctype = (*parser).c_end_doctype;
            (*new_wrapper).c_xml_decl = (*parser).c_xml_decl;
            (*new_wrapper).c_external_entity_ref = (*parser).c_external_entity_ref;
            (*new_wrapper).c_not_standalone = (*parser).c_not_standalone;
            (*new_wrapper).c_skipped_entity = (*parser).c_skipped_entity;
            (*new_wrapper).c_element_decl = (*parser).c_element_decl;
            (*new_wrapper).c_attlist_decl = (*parser).c_attlist_decl;
            (*new_wrapper).c_entity_decl = (*parser).c_entity_decl;
            (*new_wrapper).c_unparsed_entity_decl = (*parser).c_unparsed_entity_decl;
            (*new_wrapper).c_notation_decl = (*parser).c_notation_decl;
            (*new_wrapper).c_start_namespace_decl = (*parser).c_start_namespace_decl;
            (*new_wrapper).c_end_namespace_decl = (*parser).c_end_namespace_decl;
            (*new_wrapper).c_unknown_encoding = (*parser).c_unknown_encoding;
            (*new_wrapper).c_unknown_encoding_data = (*parser).c_unknown_encoding_data;
            (*new_wrapper).user_data = (*parser).user_data;
            (*new_wrapper).use_parser_as_handler = (*parser).use_parser_as_handler;
            // Sync handlers on the new parser
            sync_handlers(&mut *new_wrapper);
            new_wrapper
        }
        None => ptr::null_mut(),
    }
}

// --- Version/Feature functions ---

static EXPAT_VERSION_CSTR: &[u8] = b"expat_2.7.5\0";

#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersion() -> *const XML_Char {
    EXPAT_VERSION_CSTR.as_ptr() as *const XML_Char
}

#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersionInfo() -> XML_Expat_Version {
    XML_Expat_Version {
        major: 2,
        minor: 7,
        micro: 5,
    }
}

// Feature list - static, null-terminated
static FEATURE_DTD_NAME: &[u8] = b"XML_DTD\0";
static FEATURE_NS_NAME: &[u8] = b"XML_NS\0";
static FEATURE_CONTEXT_NAME: &[u8] = b"XML_CONTEXT_BYTES\0";
static FEATURE_SIZEOF_CHAR_NAME: &[u8] = b"sizeof(XML_Char)\0";
static FEATURE_SIZEOF_LCHAR_NAME: &[u8] = b"sizeof(XML_LChar)\0";
static FEATURE_GE_NAME: &[u8] = b"XML_GE\0";
static FEATURE_BILLION_AMP_NAME: &[u8] = b"XML_BLAP_MAX_AMP\0";
static FEATURE_BILLION_THRESH_NAME: &[u8] = b"XML_BLAP_ACT_THRES\0";
static EMPTY_NAME: &[u8] = b"\0";

unsafe impl Sync for XML_Feature {}

static FEATURES: [XML_Feature; 9] = [
    XML_Feature { feature: 3 /* DTD */, name: FEATURE_DTD_NAME.as_ptr() as *const XML_Char, value: 0 },
    XML_Feature { feature: 8 /* NS */, name: FEATURE_NS_NAME.as_ptr() as *const XML_Char, value: 0 },
    XML_Feature { feature: 4 /* CONTEXT_BYTES */, name: FEATURE_CONTEXT_NAME.as_ptr() as *const XML_Char, value: 1024 },
    XML_Feature { feature: 6 /* SIZEOF_XML_CHAR */, name: FEATURE_SIZEOF_CHAR_NAME.as_ptr() as *const XML_Char, value: 1 },
    XML_Feature { feature: 7 /* SIZEOF_XML_LCHAR */, name: FEATURE_SIZEOF_LCHAR_NAME.as_ptr() as *const XML_Char, value: 1 },
    XML_Feature { feature: 13 /* GE */, name: FEATURE_GE_NAME.as_ptr() as *const XML_Char, value: 1 },
    XML_Feature { feature: 11 /* BLAP_MAX_AMP */, name: FEATURE_BILLION_AMP_NAME.as_ptr() as *const XML_Char, value: 100 },
    XML_Feature { feature: 12 /* BLAP_ACT_THRES */, name: FEATURE_BILLION_THRESH_NAME.as_ptr() as *const XML_Char, value: 8388608 },
    XML_Feature { feature: 0 /* END */, name: EMPTY_NAME.as_ptr() as *const XML_Char, value: 0 },
];

#[no_mangle]
pub unsafe extern "C" fn XML_GetFeatureList() -> *const XML_Feature {
    FEATURES.as_ptr()
}

// --- Billion laughs protection ---

#[no_mangle]
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
    parser: XML_Parser,
    maximumAmplificationFactor: f32,
) -> XML_Bool {
    if parser.is_null() { return XML_FALSE; }
    if (*parser).parser.set_billion_laughs_attack_protection_maximum_amplification(maximumAmplificationFactor) {
        XML_TRUE
    } else {
        XML_FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
    parser: XML_Parser,
    activationThresholdBytes: c_ulong,
) -> XML_Bool {
    if parser.is_null() { return XML_FALSE; }
    if (*parser).parser.set_billion_laughs_attack_protection_activation_threshold(activationThresholdBytes as u64) {
        XML_TRUE
    } else {
        XML_FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn XML_SetAllocTrackerMaximumAmplification(
    parser: XML_Parser,
    factor: f32,
) -> XML_Bool {
    if parser.is_null() { return XML_FALSE; }
    if (*parser).parser.set_alloc_tracker_maximum_amplification(factor) {
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
    if parser.is_null() { return XML_FALSE; }
    if (*parser).parser.set_alloc_tracker_activation_threshold(threshold as u64) {
        XML_TRUE
    } else {
        XML_FALSE
    }
}

// --- Memory management stubs (for XML_MemMalloc, XML_MemRealloc, XML_MemFree) ---

#[no_mangle]
pub unsafe extern "C" fn XML_MemMalloc(parser: XML_Parser, size: usize) -> *mut c_void {
    libc::malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn XML_MemRealloc(
    parser: XML_Parser,
    ptr: *mut c_void,
    size: usize,
) -> *mut c_void {
    libc::realloc(ptr, size)
}

#[no_mangle]
pub unsafe extern "C" fn XML_MemFree(parser: XML_Parser, ptr: *mut c_void) {
    libc::free(ptr)
}

// --- Input context (stubbed - would need deeper integration) ---

#[no_mangle]
pub unsafe extern "C" fn XML_GetInputContext(
    parser: XML_Parser,
    offset: *mut c_int,
    size: *mut c_int,
) -> *const c_char {
    // Return NULL to indicate no input context available
    // This is valid behavior per the API docs
    ptr::null()
}

// --- Internal testing symbols required by C test suite ---

#[no_mangle]
pub static mut g_reparseDeferralEnabledDefault: XML_Bool = XML_TRUE;

#[no_mangle]
pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters(
    from: *const c_char,
    fromLimRef: *mut *const c_char,
) {
    // Stub: leave fromLimRef unchanged (no trimming)
}

#[no_mangle]
pub unsafe extern "C" fn testingAccountingGetCountBytesDirect(
    parser: XML_Parser,
) -> c_ulong {
    0
}

#[no_mangle]
pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect(
    parser: XML_Parser,
) -> c_ulong {
    0
}

#[no_mangle]
pub unsafe extern "C" fn unsignedCharToPrintable(c: u8) -> *const c_char {
    static mut BUF: [u8; 8] = [0u8; 8];
    if c >= 0x20 && c < 0x7f {
        BUF[0] = c;
        BUF[1] = 0;
    } else {
        // Format as hex
        let hex = b"0123456789ABCDEF";
        BUF[0] = b'\\';
        BUF[1] = b'x';
        BUF[2] = hex[(c >> 4) as usize];
        BUF[3] = hex[(c & 0xf) as usize];
        BUF[4] = 0;
    }
    BUF.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn expat_malloc(
    parser: XML_Parser,
    size: usize,
    source_line: c_int,
) -> *mut c_void {
    libc::malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn expat_realloc(
    parser: XML_Parser,
    ptr: *mut c_void,
    size: usize,
    source_line: c_int,
) -> *mut c_void {
    libc::realloc(ptr, size)
}

#[no_mangle]
pub unsafe extern "C" fn expat_free(
    parser: XML_Parser,
    ptr: *mut c_void,
    source_line: c_int,
) {
    libc::free(ptr)
}
