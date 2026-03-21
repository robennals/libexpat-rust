#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type XML_Size = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position {
    pub lineNumber: XML_Size,
    pub columnNumber: XML_Size,
}
pub type POSITION = position;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ATTRIBUTE {
    pub name: *const ::core::ffi::c_char,
    pub valuePtr: *const ::core::ffi::c_char,
    pub valueEnd: *const ::core::ffi::c_char,
    pub normalized: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding {
    pub scanners: [SCANNER; 4],
    pub literalScanners: [SCANNER; 2],
    pub nameMatchesAscii: Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub nameLength: Option<
        unsafe extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub skipS: Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
    pub getAtts: Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ATTRIBUTE,
        ) -> ::core::ffi::c_int,
    >,
    pub charRefNumber: Option<
        unsafe extern "C" fn(*const ENCODING, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub predefinedEntityName: Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub updatePosition: Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut POSITION,
        ) -> (),
    >,
    pub isPublicId: Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub utf8Convert: Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> XML_Convert_Result,
    >,
    pub utf16Convert: Option<
        unsafe extern "C" fn(
            *const ENCODING,
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_ushort,
            *const ::core::ffi::c_ushort,
        ) -> XML_Convert_Result,
    >,
    pub minBytesPerChar: ::core::ffi::c_int,
    pub isUtf8: ::core::ffi::c_char,
    pub isUtf16: ::core::ffi::c_char,
}
pub type ENCODING = encoding;
pub type XML_Convert_Result = ::core::ffi::c_uint;
pub const XML_CONVERT_OUTPUT_EXHAUSTED: XML_Convert_Result = 2;
pub const XML_CONVERT_INPUT_INCOMPLETE: XML_Convert_Result = 1;
pub const XML_CONVERT_COMPLETED: XML_Convert_Result = 0;
pub type SCANNER = Option<
    unsafe extern "C" fn(
        *const ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
pub type C2RustUnnamed = ::core::ffi::c_int;
pub const XML_ROLE_PARAM_ENTITY_REF: C2RustUnnamed = 60;
pub const XML_ROLE_INNER_PARAM_ENTITY_REF: C2RustUnnamed = 59;
pub const XML_ROLE_IGNORE_SECT: C2RustUnnamed = 58;
pub const XML_ROLE_TEXT_DECL: C2RustUnnamed = 57;
pub const XML_ROLE_COMMENT: C2RustUnnamed = 56;
pub const XML_ROLE_PI: C2RustUnnamed = 55;
pub const XML_ROLE_CONTENT_ELEMENT_PLUS: C2RustUnnamed = 54;
pub const XML_ROLE_CONTENT_ELEMENT_OPT: C2RustUnnamed = 53;
pub const XML_ROLE_CONTENT_ELEMENT_REP: C2RustUnnamed = 52;
pub const XML_ROLE_CONTENT_ELEMENT: C2RustUnnamed = 51;
pub const XML_ROLE_GROUP_SEQUENCE: C2RustUnnamed = 50;
pub const XML_ROLE_GROUP_CHOICE: C2RustUnnamed = 49;
pub const XML_ROLE_GROUP_CLOSE_PLUS: C2RustUnnamed = 48;
pub const XML_ROLE_GROUP_CLOSE_OPT: C2RustUnnamed = 47;
pub const XML_ROLE_GROUP_CLOSE_REP: C2RustUnnamed = 46;
pub const XML_ROLE_GROUP_CLOSE: C2RustUnnamed = 45;
pub const XML_ROLE_GROUP_OPEN: C2RustUnnamed = 44;
pub const XML_ROLE_CONTENT_PCDATA: C2RustUnnamed = 43;
pub const XML_ROLE_CONTENT_EMPTY: C2RustUnnamed = 42;
pub const XML_ROLE_CONTENT_ANY: C2RustUnnamed = 41;
pub const XML_ROLE_ELEMENT_NAME: C2RustUnnamed = 40;
pub const XML_ROLE_ELEMENT_NONE: C2RustUnnamed = 39;
pub const XML_ROLE_FIXED_ATTRIBUTE_VALUE: C2RustUnnamed = 38;
pub const XML_ROLE_DEFAULT_ATTRIBUTE_VALUE: C2RustUnnamed = 37;
pub const XML_ROLE_REQUIRED_ATTRIBUTE_VALUE: C2RustUnnamed = 36;
pub const XML_ROLE_IMPLIED_ATTRIBUTE_VALUE: C2RustUnnamed = 35;
pub const XML_ROLE_ATTLIST_ELEMENT_NAME: C2RustUnnamed = 34;
pub const XML_ROLE_ATTLIST_NONE: C2RustUnnamed = 33;
pub const XML_ROLE_ATTRIBUTE_NOTATION_VALUE: C2RustUnnamed = 32;
pub const XML_ROLE_ATTRIBUTE_ENUM_VALUE: C2RustUnnamed = 31;
pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS: C2RustUnnamed = 30;
pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN: C2RustUnnamed = 29;
pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITIES: C2RustUnnamed = 28;
pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITY: C2RustUnnamed = 27;
pub const XML_ROLE_ATTRIBUTE_TYPE_IDREFS: C2RustUnnamed = 26;
pub const XML_ROLE_ATTRIBUTE_TYPE_IDREF: C2RustUnnamed = 25;
pub const XML_ROLE_ATTRIBUTE_TYPE_ID: C2RustUnnamed = 24;
pub const XML_ROLE_ATTRIBUTE_TYPE_CDATA: C2RustUnnamed = 23;
pub const XML_ROLE_ATTRIBUTE_NAME: C2RustUnnamed = 22;
pub const XML_ROLE_NOTATION_PUBLIC_ID: C2RustUnnamed = 21;
pub const XML_ROLE_NOTATION_NO_SYSTEM_ID: C2RustUnnamed = 20;
pub const XML_ROLE_NOTATION_SYSTEM_ID: C2RustUnnamed = 19;
pub const XML_ROLE_NOTATION_NAME: C2RustUnnamed = 18;
pub const XML_ROLE_NOTATION_NONE: C2RustUnnamed = 17;
pub const XML_ROLE_ENTITY_NOTATION_NAME: C2RustUnnamed = 16;
pub const XML_ROLE_ENTITY_COMPLETE: C2RustUnnamed = 15;
pub const XML_ROLE_ENTITY_PUBLIC_ID: C2RustUnnamed = 14;
pub const XML_ROLE_ENTITY_SYSTEM_ID: C2RustUnnamed = 13;
pub const XML_ROLE_ENTITY_VALUE: C2RustUnnamed = 12;
pub const XML_ROLE_ENTITY_NONE: C2RustUnnamed = 11;
pub const XML_ROLE_PARAM_ENTITY_NAME: C2RustUnnamed = 10;
pub const XML_ROLE_GENERAL_ENTITY_NAME: C2RustUnnamed = 9;
pub const XML_ROLE_DOCTYPE_CLOSE: C2RustUnnamed = 8;
pub const XML_ROLE_DOCTYPE_INTERNAL_SUBSET: C2RustUnnamed = 7;
pub const XML_ROLE_DOCTYPE_PUBLIC_ID: C2RustUnnamed = 6;
pub const XML_ROLE_DOCTYPE_SYSTEM_ID: C2RustUnnamed = 5;
pub const XML_ROLE_DOCTYPE_NAME: C2RustUnnamed = 4;
pub const XML_ROLE_DOCTYPE_NONE: C2RustUnnamed = 3;
pub const XML_ROLE_INSTANCE_START: C2RustUnnamed = 2;
pub const XML_ROLE_XML_DECL: C2RustUnnamed = 1;
pub const XML_ROLE_NONE: C2RustUnnamed = 0;
pub const XML_ROLE_ERROR: C2RustUnnamed = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prolog_state {
    pub handler: Option<
        unsafe extern "C" fn(
            *mut prolog_state,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ENCODING,
        ) -> ::core::ffi::c_int,
    >,
    pub level: ::core::ffi::c_uint,
    pub role_none: ::core::ffi::c_int,
    pub includeLevel: ::core::ffi::c_uint,
    pub documentEntity: ::core::ffi::c_int,
    pub inEntityValue: ::core::ffi::c_int,
}
pub type PROLOG_STATE = prolog_state;
pub type PROLOG_HANDLER = unsafe extern "C" fn(
    *mut PROLOG_STATE,
    ::core::ffi::c_int,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *const ENCODING,
) -> ::core::ffi::c_int;
pub const XML_TOK_NONE: ::core::ffi::c_int = -4;
pub const XML_TOK_PI: ::core::ffi::c_int = 11;
pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12;
pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13;
pub const XML_TOK_BOM: ::core::ffi::c_int = 14;
pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15;
pub const XML_TOK_DECL_OPEN: ::core::ffi::c_int = 16;
pub const XML_TOK_DECL_CLOSE: ::core::ffi::c_int = 17;
pub const XML_TOK_NAME: ::core::ffi::c_int = 18;
pub const XML_TOK_NMTOKEN: ::core::ffi::c_int = 19;
pub const XML_TOK_POUND_NAME: ::core::ffi::c_int = 20;
pub const XML_TOK_OR: ::core::ffi::c_int = 21;
pub const XML_TOK_PERCENT: ::core::ffi::c_int = 22;
pub const XML_TOK_OPEN_PAREN: ::core::ffi::c_int = 23;
pub const XML_TOK_CLOSE_PAREN: ::core::ffi::c_int = 24;
pub const XML_TOK_OPEN_BRACKET: ::core::ffi::c_int = 25;
pub const XML_TOK_CLOSE_BRACKET: ::core::ffi::c_int = 26;
pub const XML_TOK_LITERAL: ::core::ffi::c_int = 27;
pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29;
pub const XML_TOK_NAME_QUESTION: ::core::ffi::c_int = 30;
pub const XML_TOK_NAME_ASTERISK: ::core::ffi::c_int = 31;
pub const XML_TOK_NAME_PLUS: ::core::ffi::c_int = 32;
pub const XML_TOK_COND_SECT_OPEN: ::core::ffi::c_int = 33;
pub const XML_TOK_COND_SECT_CLOSE: ::core::ffi::c_int = 34;
pub const XML_TOK_CLOSE_PAREN_QUESTION: ::core::ffi::c_int = 35;
pub const XML_TOK_CLOSE_PAREN_ASTERISK: ::core::ffi::c_int = 36;
pub const XML_TOK_CLOSE_PAREN_PLUS: ::core::ffi::c_int = 37;
pub const XML_TOK_COMMA: ::core::ffi::c_int = 38;
pub const XML_TOK_PREFIXED_NAME: ::core::ffi::c_int = 41;
pub const ASCII_A: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const ASCII_B: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const ASCII_C: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const ASCII_D: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const ASCII_E: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const ASCII_F: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;
pub const ASCII_G: ::core::ffi::c_int = 0x47 as ::core::ffi::c_int;
pub const ASCII_I: ::core::ffi::c_int = 0x49 as ::core::ffi::c_int;
pub const ASCII_K: ::core::ffi::c_int = 0x4b as ::core::ffi::c_int;
pub const ASCII_L: ::core::ffi::c_int = 0x4c as ::core::ffi::c_int;
pub const ASCII_M: ::core::ffi::c_int = 0x4d as ::core::ffi::c_int;
pub const ASCII_N: ::core::ffi::c_int = 0x4e as ::core::ffi::c_int;
pub const ASCII_O: ::core::ffi::c_int = 0x4f as ::core::ffi::c_int;
pub const ASCII_P: ::core::ffi::c_int = 0x50 as ::core::ffi::c_int;
pub const ASCII_Q: ::core::ffi::c_int = 0x51 as ::core::ffi::c_int;
pub const ASCII_R: ::core::ffi::c_int = 0x52 as ::core::ffi::c_int;
pub const ASCII_S: ::core::ffi::c_int = 0x53 as ::core::ffi::c_int;
pub const ASCII_T: ::core::ffi::c_int = 0x54 as ::core::ffi::c_int;
pub const ASCII_U: ::core::ffi::c_int = 0x55 as ::core::ffi::c_int;
pub const ASCII_X: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;
pub const ASCII_Y: ::core::ffi::c_int = 0x59 as ::core::ffi::c_int;
static mut KW_ANY: [::core::ffi::c_char; 4] = [
    ASCII_A as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_ATTLIST: [::core::ffi::c_char; 8] = [
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_CDATA: [::core::ffi::c_char; 6] = [
    ASCII_C as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_DOCTYPE: [::core::ffi::c_char; 8] = [
    ASCII_D as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_ELEMENT: [::core::ffi::c_char; 8] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_EMPTY: [::core::ffi::c_char; 6] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_ENTITIES: [::core::ffi::c_char; 9] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_ENTITY: [::core::ffi::c_char; 7] = [
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_FIXED: [::core::ffi::c_char; 6] = [
    ASCII_F as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_X as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_ID: [::core::ffi::c_char; 3] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_IDREF: [::core::ffi::c_char; 6] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_IDREFS: [::core::ffi::c_char; 7] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_F as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_IGNORE: [::core::ffi::c_char; 7] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_G as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_IMPLIED: [::core::ffi::c_char; 8] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_P as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_INCLUDE: [::core::ffi::c_char; 8] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_U as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_NDATA: [::core::ffi::c_char; 6] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_NMTOKEN: [::core::ffi::c_char; 8] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_K as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_NMTOKENS: [::core::ffi::c_char; 9] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_K as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_NOTATION: [::core::ffi::c_char; 9] = [
    ASCII_N as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_N as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_PCDATA: [::core::ffi::c_char; 7] = [
    ASCII_P as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_PUBLIC: [::core::ffi::c_char; 7] = [
    ASCII_P as ::core::ffi::c_char,
    ASCII_U as ::core::ffi::c_char,
    ASCII_B as ::core::ffi::c_char,
    ASCII_L as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_REQUIRED: [::core::ffi::c_char; 9] = [
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_Q as ::core::ffi::c_char,
    ASCII_U as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_R as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_D as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
static mut KW_SYSTEM: [::core::ffi::c_char; 7] = [
    ASCII_S as ::core::ffi::c_char,
    ASCII_Y as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_E as ::core::ffi::c_char,
    ASCII_M as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];
unsafe extern "C" fn prolog0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => {
            (*state).handler = Some(
                prolog1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NONE as ::core::ffi::c_int;
        }
        XML_TOK_XML_DECL => {
            (*state).handler = Some(
                prolog1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_XML_DECL as ::core::ffi::c_int;
        }
        XML_TOK_PI => {
            (*state).handler = Some(
                prolog1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_PI as ::core::ffi::c_int;
        }
        XML_TOK_COMMENT => {
            (*state).handler = Some(
                prolog1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_COMMENT as ::core::ffi::c_int;
        }
        XML_TOK_BOM => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_DOCTYPE as *const ::core::ffi::c_char,
            ) == 0)
            {
                (*state).handler = Some(
                    doctype0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(
                error
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_INSTANCE_START as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn prolog1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_PI => return XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_BOM => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_DOCTYPE as *const ::core::ffi::c_char,
            ) == 0)
            {
                (*state).handler = Some(
                    doctype0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(
                error
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_INSTANCE_START as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn prolog2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_PI => return XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_INSTANCE_START => {
            (*state).handler = Some(
                error
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_INSTANCE_START as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn doctype0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                doctype1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn doctype1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                internalSubset
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET as ::core::ffi::c_int;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int;
        }
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    doctype3
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    doctype2
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn doctype2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                doctype3
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_PUBLIC_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn doctype3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                doctype4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn doctype4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                internalSubset
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET as ::core::ffi::c_int;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn doctype5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_CLOSE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn internalSubset(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_OPEN => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ENTITY as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ATTLIST as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ELEMENT as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    element0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((2 as ::core::ffi::c_int * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_NOTATION as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    notation0
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_PI => return XML_ROLE_PI as ::core::ffi::c_int,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT as ::core::ffi::c_int,
        XML_TOK_PARAM_ENTITY_REF => {
            return XML_ROLE_PARAM_ENTITY_REF as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_BRACKET => {
            (*state).handler = Some(
                doctype5
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DOCTYPE_NONE as ::core::ffi::c_int;
        }
        XML_TOK_NONE => return XML_ROLE_NONE as ::core::ffi::c_int,
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn externalSubset0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    (*state).handler = Some(
        externalSubset1
            as unsafe extern "C" fn(
                *mut PROLOG_STATE,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >;
    if tok == XML_TOK_XML_DECL {
        return XML_ROLE_TEXT_DECL as ::core::ffi::c_int;
    }
    return externalSubset1(state, tok, ptr, end, enc);
}
unsafe extern "C" fn externalSubset1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_COND_SECT_OPEN => {
            (*state).handler = Some(
                condSect0
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NONE as ::core::ffi::c_int;
        }
        XML_TOK_COND_SECT_CLOSE => {
            if !((*state).includeLevel == 0 as ::core::ffi::c_uint) {
                (*state).includeLevel =
                    (*state).includeLevel.wrapping_sub(1 as ::core::ffi::c_uint);
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_BRACKET => {}
        XML_TOK_NONE => {
            if !((*state).includeLevel != 0) {
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }
        }
        _ => return internalSubset(state, tok, ptr, end, enc),
    }
    return common(state, tok);
}
unsafe extern "C" fn entity0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_PERCENT => {
            (*state).handler = Some(
                entity1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
        }
        XML_TOK_NAME => {
            (*state).handler = Some(
                entity2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_GENERAL_ENTITY_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            (*state).handler = Some(
                entity7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_PARAM_ENTITY_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity4
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity3
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            return XML_ROLE_ENTITY_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_PUBLIC_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity5
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_COMPLETE as ::core::ffi::c_int;
        }
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_NDATA as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity6
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity6(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            return XML_ROLE_ENTITY_NOTATION_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity7(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity9
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity8
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ENTITY_NONE as ::core::ffi::c_int;
            return XML_ROLE_ENTITY_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity8(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity9
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_PUBLIC_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity9(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity10
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn entity10(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ENTITY_COMPLETE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn notation0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            (*state).handler = Some(
                notation1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NOTATION_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn notation1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_SYSTEM as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    notation3
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_PUBLIC as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    notation2
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn notation2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                notation4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NOTATION_PUBLIC_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn notation3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            return XML_ROLE_NOTATION_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn notation4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_NOTATION_NONE as ::core::ffi::c_int;
            return XML_ROLE_NOTATION_SYSTEM_ID as ::core::ffi::c_int;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_NOTATION_NO_SYSTEM_ID as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_ELEMENT_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTRIBUTE_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            static mut types: [*const ::core::ffi::c_char; 8] = unsafe {
                [
                    &raw const KW_CDATA as *const ::core::ffi::c_char,
                    &raw const KW_ID as *const ::core::ffi::c_char,
                    &raw const KW_IDREF as *const ::core::ffi::c_char,
                    &raw const KW_IDREFS as *const ::core::ffi::c_char,
                    &raw const KW_ENTITY as *const ::core::ffi::c_char,
                    &raw const KW_ENTITIES as *const ::core::ffi::c_char,
                    &raw const KW_NMTOKEN as *const ::core::ffi::c_char,
                    &raw const KW_NMTOKENS as *const ::core::ffi::c_char,
                ]
            };
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i
                < (::core::mem::size_of::<[*const ::core::ffi::c_char; 8]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
                    as ::core::ffi::c_int
            {
                if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                    enc,
                    ptr,
                    end,
                    types[i as usize],
                ) != 0
                {
                    (*state).handler = Some(
                        attlist8
                            as unsafe extern "C" fn(
                                *mut PROLOG_STATE,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *const ENCODING,
                            )
                                -> ::core::ffi::c_int,
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut prolog_state,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *const ENCODING,
                            ) -> ::core::ffi::c_int,
                        >;
                    return XML_ROLE_ATTRIBUTE_TYPE_CDATA as ::core::ffi::c_int + i;
                }
                i += 1;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_NOTATION as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist5
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                attlist3
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NMTOKEN | XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTRIBUTE_ENUM_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                attlist8
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                attlist3
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                attlist6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist6(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            (*state).handler = Some(
                attlist7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTRIBUTE_NOTATION_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist7(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                attlist8
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                attlist6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist8(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_IMPLIED as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_IMPLIED_ATTRIBUTE_VALUE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_REQUIRED as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_REQUIRED_ATTRIBUTE_VALUE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_FIXED as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist9
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                attlist1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_DEFAULT_ATTRIBUTE_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn attlist9(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE as ::core::ffi::c_int,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                attlist1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_FIXED_ATTRIBUTE_VALUE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn element0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ELEMENT_NAME as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn element1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_EMPTY as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
                return XML_ROLE_CONTENT_EMPTY as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_ANY as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
                return XML_ROLE_CONTENT_ANY as ::core::ffi::c_int;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                element2
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).level = 1 as ::core::ffi::c_uint;
            return XML_ROLE_GROUP_OPEN as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn element2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_PCDATA as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    element3
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_CONTENT_PCDATA as ::core::ffi::c_int;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).level = 2 as ::core::ffi::c_uint;
            (*state).handler = Some(
                element6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_GROUP_OPEN as ::core::ffi::c_int;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT as ::core::ffi::c_int;
        }
        XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_OPT as ::core::ffi::c_int;
        }
        XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_REP as ::core::ffi::c_int;
        }
        XML_TOK_NAME_PLUS => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_PLUS as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn element3(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            return XML_ROLE_GROUP_CLOSE as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            return XML_ROLE_GROUP_CLOSE_REP as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn element4(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element5
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn element5(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(
                declClose
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            return XML_ROLE_GROUP_CLOSE_REP as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element4
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn element6(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_PAREN => {
            (*state).level = (*state).level.wrapping_add(1 as ::core::ffi::c_uint);
            return XML_ROLE_GROUP_OPEN as ::core::ffi::c_int;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT as ::core::ffi::c_int;
        }
        XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_OPT as ::core::ffi::c_int;
        }
        XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_REP as ::core::ffi::c_int;
        }
        XML_TOK_NAME_PLUS => {
            (*state).handler = Some(
                element7
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_CONTENT_ELEMENT_PLUS as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn element7(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int,
        XML_TOK_CLOSE_PAREN => {
            (*state).level = (*state).level.wrapping_sub(1 as ::core::ffi::c_uint);
            if (*state).level == 0 as ::core::ffi::c_uint {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            return XML_ROLE_GROUP_CLOSE as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).level = (*state).level.wrapping_sub(1 as ::core::ffi::c_uint);
            if (*state).level == 0 as ::core::ffi::c_uint {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            return XML_ROLE_GROUP_CLOSE_REP as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_PAREN_QUESTION => {
            (*state).level = (*state).level.wrapping_sub(1 as ::core::ffi::c_uint);
            if (*state).level == 0 as ::core::ffi::c_uint {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            return XML_ROLE_GROUP_CLOSE_OPT as ::core::ffi::c_int;
        }
        XML_TOK_CLOSE_PAREN_PLUS => {
            (*state).level = (*state).level.wrapping_sub(1 as ::core::ffi::c_uint);
            if (*state).level == 0 as ::core::ffi::c_uint {
                (*state).handler = Some(
                    declClose
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                (*state).role_none = XML_ROLE_ELEMENT_NONE as ::core::ffi::c_int;
            }
            return XML_ROLE_GROUP_CLOSE_PLUS as ::core::ffi::c_int;
        }
        XML_TOK_COMMA => {
            (*state).handler = Some(
                element6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_GROUP_SEQUENCE as ::core::ffi::c_int;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element6
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_GROUP_CHOICE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn condSect0(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_INCLUDE as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    condSect1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }
            if (*enc).nameMatchesAscii.expect("non-null function pointer")(
                enc,
                ptr,
                end,
                &raw const KW_IGNORE as *const ::core::ffi::c_char,
            ) != 0
            {
                (*state).handler = Some(
                    condSect2
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut prolog_state,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                    >;
                return XML_ROLE_NONE as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn condSect1(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                externalSubset1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            (*state).includeLevel = (*state).includeLevel.wrapping_add(1 as ::core::ffi::c_uint);
            return XML_ROLE_NONE as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn condSect2(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE as ::core::ffi::c_int,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                externalSubset1
                    as unsafe extern "C" fn(
                        *mut PROLOG_STATE,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return XML_ROLE_IGNORE_SECT as ::core::ffi::c_int;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn declClose(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    match tok {
        XML_TOK_PROLOG_S => return (*state).role_none,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = (if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe extern "C" fn(
                            *mut PROLOG_STATE,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ENCODING,
                        ) -> ::core::ffi::c_int,
                )
            })
                as Option<
                    unsafe extern "C" fn(
                        *mut prolog_state,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ENCODING,
                    ) -> ::core::ffi::c_int,
                >;
            return (*state).role_none;
        }
        _ => {}
    }
    return common(state, tok);
}
unsafe extern "C" fn error(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut enc: *const ENCODING,
) -> ::core::ffi::c_int {
    return XML_ROLE_NONE as ::core::ffi::c_int;
}
unsafe extern "C" fn common(
    mut state: *mut PROLOG_STATE,
    mut tok: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*state).documentEntity == 0 && tok == XML_TOK_PARAM_ENTITY_REF {
        return XML_ROLE_INNER_PARAM_ENTITY_REF as ::core::ffi::c_int;
    }
    (*state).handler = Some(
        error
            as unsafe extern "C" fn(
                *mut PROLOG_STATE,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >;
    return XML_ROLE_ERROR as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInit(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(
        prolog0
            as unsafe extern "C" fn(
                *mut PROLOG_STATE,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >;
    (*state).documentEntity = 1 as ::core::ffi::c_int;
    (*state).includeLevel = 0 as ::core::ffi::c_uint;
    (*state).inEntityValue = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInitExternalEntity(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(
        externalSubset0
            as unsafe extern "C" fn(
                *mut PROLOG_STATE,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut prolog_state,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ENCODING,
            ) -> ::core::ffi::c_int,
        >;
    (*state).documentEntity = 0 as ::core::ffi::c_int;
    (*state).includeLevel = 0 as ::core::ffi::c_uint;
}
