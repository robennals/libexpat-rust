#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, raw_ref_op)]
extern "C" {
    pub type __sFILEX;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strtoul(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn arc4random_buf(__buf: *mut ::core::ffi::c_void, __nbytes: size_t);
    fn __error() -> *mut ::core::ffi::c_int;
    fn XmlParseXmlDecl(
        isGeneralTextEntity: ::core::ffi::c_int,
        enc: *const ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        badPtr: *mut *const ::core::ffi::c_char,
        versionPtr: *mut *const ::core::ffi::c_char,
        versionEndPtr: *mut *const ::core::ffi::c_char,
        encodingNamePtr: *mut *const ::core::ffi::c_char,
        namedEncodingPtr: *mut *const ENCODING,
        standalonePtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XmlInitEncoding(
        p: *mut INIT_ENCODING,
        encPtr: *mut *const ENCODING,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn XmlGetUtf8InternalEncoding() -> *const ENCODING;
    fn XmlUtf8Encode(
        charNumber: ::core::ffi::c_int,
        buf: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn XmlSizeOfUnknownEncoding() -> ::core::ffi::c_int;
    fn XmlInitUnknownEncoding(
        mem: *mut ::core::ffi::c_void,
        table: *const ::core::ffi::c_int,
        convert: CONVERTER,
        userData: *mut ::core::ffi::c_void,
    ) -> *mut ENCODING;
    fn XmlParseXmlDeclNS(
        isGeneralTextEntity: ::core::ffi::c_int,
        enc: *const ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        badPtr: *mut *const ::core::ffi::c_char,
        versionPtr: *mut *const ::core::ffi::c_char,
        versionEndPtr: *mut *const ::core::ffi::c_char,
        encodingNamePtr: *mut *const ::core::ffi::c_char,
        namedEncodingPtr: *mut *const ENCODING,
        standalonePtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn XmlInitEncodingNS(
        p: *mut INIT_ENCODING,
        encPtr: *mut *const ENCODING,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn XmlGetUtf8InternalEncodingNS() -> *const ENCODING;
    fn XmlInitUnknownEncodingNS(
        mem: *mut ::core::ffi::c_void,
        table: *const ::core::ffi::c_int,
        convert: CONVERTER,
        userData: *mut ::core::ffi::c_void,
    ) -> *mut ENCODING;
    fn XmlPrologStateInit(state: *mut PROLOG_STATE);
    fn XmlPrologStateInitExternalEntity(state: *mut PROLOG_STATE);
}
pub type __int64_t = i64;
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type uint64_t = u64;
pub type XML_Char = ::core::ffi::c_char;
pub type XML_LChar = ::core::ffi::c_char;
pub type XML_Index = ::core::ffi::c_long;
pub type XML_Size = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_ParserStruct {
    pub m_userData: *mut ::core::ffi::c_void,
    pub m_handlerArg: *mut ::core::ffi::c_void,
    pub m_buffer: *mut ::core::ffi::c_char,
    pub m_mem: XML_Memory_Handling_Suite,
    pub m_bufferPtr: *const ::core::ffi::c_char,
    pub m_bufferEnd: *mut ::core::ffi::c_char,
    pub m_bufferLim: *const ::core::ffi::c_char,
    pub m_parseEndByteIndex: XML_Index,
    pub m_parseEndPtr: *const ::core::ffi::c_char,
    pub m_partialTokenBytesBefore: size_t,
    pub m_reparseDeferralEnabled: XML_Bool,
    pub m_lastBufferRequestSize: ::core::ffi::c_int,
    pub m_dataBuf: *mut XML_Char,
    pub m_dataBufEnd: *mut XML_Char,
    pub m_startElementHandler: XML_StartElementHandler,
    pub m_endElementHandler: XML_EndElementHandler,
    pub m_characterDataHandler: XML_CharacterDataHandler,
    pub m_processingInstructionHandler: XML_ProcessingInstructionHandler,
    pub m_commentHandler: XML_CommentHandler,
    pub m_startCdataSectionHandler: XML_StartCdataSectionHandler,
    pub m_endCdataSectionHandler: XML_EndCdataSectionHandler,
    pub m_defaultHandler: XML_DefaultHandler,
    pub m_startDoctypeDeclHandler: XML_StartDoctypeDeclHandler,
    pub m_endDoctypeDeclHandler: XML_EndDoctypeDeclHandler,
    pub m_unparsedEntityDeclHandler: XML_UnparsedEntityDeclHandler,
    pub m_notationDeclHandler: XML_NotationDeclHandler,
    pub m_startNamespaceDeclHandler: XML_StartNamespaceDeclHandler,
    pub m_endNamespaceDeclHandler: XML_EndNamespaceDeclHandler,
    pub m_notStandaloneHandler: XML_NotStandaloneHandler,
    pub m_externalEntityRefHandler: XML_ExternalEntityRefHandler,
    pub m_externalEntityRefHandlerArg: XML_Parser,
    pub m_skippedEntityHandler: XML_SkippedEntityHandler,
    pub m_unknownEncodingHandler: XML_UnknownEncodingHandler,
    pub m_elementDeclHandler: XML_ElementDeclHandler,
    pub m_attlistDeclHandler: XML_AttlistDeclHandler,
    pub m_entityDeclHandler: XML_EntityDeclHandler,
    pub m_xmlDeclHandler: XML_XmlDeclHandler,
    pub m_encoding: *const ENCODING,
    pub m_initEncoding: INIT_ENCODING,
    pub m_internalEncoding: *const ENCODING,
    pub m_protocolEncodingName: *const XML_Char,
    pub m_ns: XML_Bool,
    pub m_ns_triplets: XML_Bool,
    pub m_unknownEncodingMem: *mut ::core::ffi::c_void,
    pub m_unknownEncodingData: *mut ::core::ffi::c_void,
    pub m_unknownEncodingHandlerData: *mut ::core::ffi::c_void,
    pub m_unknownEncodingRelease: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub m_prologState: PROLOG_STATE,
    pub m_processor: Option<Processor>,
    pub m_errorCode: XML_Error,
    pub m_eventPtr: *const ::core::ffi::c_char,
    pub m_eventEndPtr: *const ::core::ffi::c_char,
    pub m_positionPtr: *const ::core::ffi::c_char,
    pub m_openInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_openAttributeEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeAttributeEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_openValueEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeValueEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_defaultExpandInternalEntities: XML_Bool,
    pub m_tagLevel: ::core::ffi::c_int,
    pub m_declEntity: *mut ENTITY,
    pub m_doctypeName: *const XML_Char,
    pub m_doctypeSysid: *const XML_Char,
    pub m_doctypePubid: *const XML_Char,
    pub m_declAttributeType: *const XML_Char,
    pub m_declNotationName: *const XML_Char,
    pub m_declNotationPublicId: *const XML_Char,
    pub m_declElementType: *mut ELEMENT_TYPE,
    pub m_declAttributeId: *mut ATTRIBUTE_ID,
    pub m_declAttributeIsCdata: XML_Bool,
    pub m_declAttributeIsId: XML_Bool,
    pub m_dtd: *mut DTD,
    pub m_curBase: *const XML_Char,
    pub m_tagStack: *mut TAG,
    pub m_freeTagList: *mut TAG,
    pub m_inheritedBindings: *mut BINDING,
    pub m_freeBindingList: *mut BINDING,
    pub m_attsSize: ::core::ffi::c_int,
    pub m_nSpecifiedAtts: ::core::ffi::c_int,
    pub m_idAttIndex: ::core::ffi::c_int,
    pub m_atts: *mut ATTRIBUTE,
    pub m_nsAtts: *mut NS_ATT,
    pub m_nsAttsVersion: ::core::ffi::c_ulong,
    pub m_nsAttsPower: ::core::ffi::c_uchar,
    pub m_position: POSITION,
    pub m_tempPool: STRING_POOL,
    pub m_temp2Pool: STRING_POOL,
    pub m_groupConnector: *mut ::core::ffi::c_char,
    pub m_groupSize: ::core::ffi::c_uint,
    pub m_namespaceSeparator: XML_Char,
    pub m_parentParser: XML_Parser,
    pub m_parsingStatus: XML_ParsingStatus,
    pub m_isParamEntity: XML_Bool,
    pub m_useForeignDTD: XML_Bool,
    pub m_paramEntityParsing: XML_ParamEntityParsing,
    pub m_hash_secret_salt: ::core::ffi::c_ulong,
    pub m_accounting: ACCOUNTING,
    pub m_alloc_tracker: MALLOC_TRACKER,
    pub m_entity_stats: ENTITY_STATS,
    pub m_reenter: XML_Bool,
}
pub type XML_Bool = ::core::ffi::c_uchar;
pub type ENTITY_STATS = entity_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_stats {
    pub countEverOpened: ::core::ffi::c_uint,
    pub currentDepth: ::core::ffi::c_uint,
    pub maximumDepthSeen: ::core::ffi::c_uint,
    pub debugLevel: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MALLOC_TRACKER {
    pub bytesAllocated: XmlBigCount,
    pub peakBytesAllocated: XmlBigCount,
    pub debugLevel: ::core::ffi::c_ulong,
    pub maximumAmplificationFactor: ::core::ffi::c_float,
    pub activationThresholdBytes: XmlBigCount,
}
pub type XmlBigCount = ::core::ffi::c_ulonglong;
pub type ACCOUNTING = accounting;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct accounting {
    pub countBytesDirect: XmlBigCount,
    pub countBytesIndirect: XmlBigCount,
    pub debugLevel: ::core::ffi::c_ulong,
    pub maximumAmplificationFactor: ::core::ffi::c_float,
    pub activationThresholdBytes: ::core::ffi::c_ulonglong,
}
pub type XML_ParamEntityParsing = ::core::ffi::c_uint;
pub const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;
pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;
pub const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_ParsingStatus {
    pub parsing: XML_Parsing,
    pub finalBuffer: XML_Bool,
}
pub type XML_Parsing = ::core::ffi::c_uint;
pub const XML_SUSPENDED: XML_Parsing = 3;
pub const XML_FINISHED: XML_Parsing = 2;
pub const XML_PARSING: XML_Parsing = 1;
pub const XML_INITIALIZED: XML_Parsing = 0;
pub type XML_Parser = *mut XML_ParserStruct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STRING_POOL {
    pub blocks: *mut BLOCK,
    pub freeBlocks: *mut BLOCK,
    pub end: *const XML_Char,
    pub ptr: *mut XML_Char,
    pub start: *mut XML_Char,
    pub parser: XML_Parser,
}
pub type BLOCK = block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block {
    pub next: *mut block,
    pub size: ::core::ffi::c_int,
    pub s: [XML_Char; 0],
}
pub type POSITION = position;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position {
    pub lineNumber: XML_Size,
    pub columnNumber: XML_Size,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NS_ATT {
    pub version: ::core::ffi::c_ulong,
    pub hash: ::core::ffi::c_ulong,
    pub uriName: *const XML_Char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ATTRIBUTE {
    pub name: *const ::core::ffi::c_char,
    pub valuePtr: *const ::core::ffi::c_char,
    pub valueEnd: *const ::core::ffi::c_char,
    pub normalized: ::core::ffi::c_char,
}
pub type BINDING = binding;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binding {
    pub prefix: *mut prefix,
    pub nextTagBinding: *mut binding,
    pub prevPrefixBinding: *mut binding,
    pub attId: *const attribute_id,
    pub uri: *mut XML_Char,
    pub uriLen: ::core::ffi::c_int,
    pub uriAlloc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attribute_id {
    pub name: *mut XML_Char,
    pub prefix: *mut PREFIX,
    pub maybeTokenized: XML_Bool,
    pub xmlns: XML_Bool,
}
pub type PREFIX = prefix;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prefix {
    pub name: *const XML_Char,
    pub binding: *mut BINDING,
}
pub type TAG = tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tag {
    pub parent: *mut tag,
    pub rawName: *const ::core::ffi::c_char,
    pub rawNameLength: ::core::ffi::c_int,
    pub name: TAG_NAME,
    pub buf: C2RustUnnamed,
    pub bufEnd: *mut ::core::ffi::c_char,
    pub bindings: *mut BINDING,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub raw: *mut ::core::ffi::c_char,
    pub str_0: *mut XML_Char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAG_NAME {
    pub str_0: *const XML_Char,
    pub localPart: *const XML_Char,
    pub prefix: *const XML_Char,
    pub strLen: ::core::ffi::c_int,
    pub uriLen: ::core::ffi::c_int,
    pub prefixLen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DTD {
    pub generalEntities: HASH_TABLE,
    pub elementTypes: HASH_TABLE,
    pub attributeIds: HASH_TABLE,
    pub prefixes: HASH_TABLE,
    pub pool: STRING_POOL,
    pub entityValuePool: STRING_POOL,
    pub keepProcessing: XML_Bool,
    pub hasParamEntityRefs: XML_Bool,
    pub standalone: XML_Bool,
    pub paramEntityRead: XML_Bool,
    pub paramEntities: HASH_TABLE,
    pub defaultPrefix: PREFIX,
    pub in_eldecl: XML_Bool,
    pub scaffold: *mut CONTENT_SCAFFOLD,
    pub contentStringLen: ::core::ffi::c_uint,
    pub scaffSize: ::core::ffi::c_uint,
    pub scaffCount: ::core::ffi::c_uint,
    pub scaffLevel: ::core::ffi::c_int,
    pub scaffIndex: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONTENT_SCAFFOLD {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *const XML_Char,
    pub firstchild: ::core::ffi::c_int,
    pub lastchild: ::core::ffi::c_int,
    pub childcnt: ::core::ffi::c_int,
    pub nextsib: ::core::ffi::c_int,
}
pub type XML_Content_Quant = ::core::ffi::c_uint;
pub const XML_CQUANT_PLUS: XML_Content_Quant = 3;
pub const XML_CQUANT_REP: XML_Content_Quant = 2;
pub const XML_CQUANT_OPT: XML_Content_Quant = 1;
pub const XML_CQUANT_NONE: XML_Content_Quant = 0;
pub type XML_Content_Type = ::core::ffi::c_uint;
pub const XML_CTYPE_SEQ: XML_Content_Type = 6;
pub const XML_CTYPE_CHOICE: XML_Content_Type = 5;
pub const XML_CTYPE_NAME: XML_Content_Type = 4;
pub const XML_CTYPE_MIXED: XML_Content_Type = 3;
pub const XML_CTYPE_ANY: XML_Content_Type = 2;
pub const XML_CTYPE_EMPTY: XML_Content_Type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASH_TABLE {
    pub v: *mut *mut NAMED,
    pub power: ::core::ffi::c_uchar,
    pub size: size_t,
    pub used: size_t,
    pub parser: XML_Parser,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NAMED {
    pub name: KEY,
}
pub type KEY = *const XML_Char;
pub type ATTRIBUTE_ID = attribute_id;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_TYPE {
    pub name: *const XML_Char,
    pub prefix: *mut PREFIX,
    pub idAtt: *const ATTRIBUTE_ID,
    pub nDefaultAtts: ::core::ffi::c_int,
    pub allocDefaultAtts: ::core::ffi::c_int,
    pub defaultAtts: *mut DEFAULT_ATTRIBUTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DEFAULT_ATTRIBUTE {
    pub id: *const ATTRIBUTE_ID,
    pub isCdata: XML_Bool,
    pub value: *const XML_Char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTITY {
    pub name: *const XML_Char,
    pub textPtr: *const XML_Char,
    pub textLen: ::core::ffi::c_int,
    pub processed: ::core::ffi::c_int,
    pub systemId: *const XML_Char,
    pub base: *const XML_Char,
    pub publicId: *const XML_Char,
    pub notation: *const XML_Char,
    pub open: XML_Bool,
    pub hasMore: XML_Bool,
    pub is_param: XML_Bool,
    pub is_internal: XML_Bool,
}
pub type OPEN_INTERNAL_ENTITY = open_internal_entity;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct open_internal_entity {
    pub internalEventPtr: *const ::core::ffi::c_char,
    pub internalEventEndPtr: *const ::core::ffi::c_char,
    pub next: *mut open_internal_entity,
    pub entity: *mut ENTITY,
    pub startTagLevel: ::core::ffi::c_int,
    pub betweenDecl: XML_Bool,
    pub type_0: EntityType,
}
pub type EntityType = ::core::ffi::c_uint;
pub const ENTITY_VALUE: EntityType = 2;
pub const ENTITY_ATTRIBUTE: EntityType = 1;
pub const ENTITY_INTERNAL: EntityType = 0;
pub type XML_Error = ::core::ffi::c_uint;
pub const XML_ERROR_NOT_STARTED: XML_Error = 44;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;
pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_NONE: XML_Error = 0;
pub type Processor = unsafe extern "C" fn(
    XML_Parser,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *mut *const ::core::ffi::c_char,
) -> XML_Error;
pub type PROLOG_STATE = prolog_state;
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
pub type ENCODING = encoding;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INIT_ENCODING {
    pub initEnc: ENCODING,
    pub encPtr: *mut *const ENCODING,
}
pub type XML_XmlDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type XML_EntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        ::core::ffi::c_int,
        *const XML_Char,
        ::core::ffi::c_int,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
>;
pub type XML_AttlistDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type XML_ElementDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut XML_Content) -> ()>;
pub type XML_Content = XML_cp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_cp {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *mut XML_Char,
    pub numchildren: ::core::ffi::c_uint,
    pub children: *mut XML_Content,
}
pub type XML_UnknownEncodingHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *mut XML_Encoding,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Encoding {
    pub map: [::core::ffi::c_int; 256],
    pub data: *mut ::core::ffi::c_void,
    pub convert: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub release: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type XML_SkippedEntityHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> ::core::ffi::c_int,
>;
pub type XML_NotStandaloneHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type XML_EndNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_StartNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *const XML_Char) -> ()>;
pub type XML_NotationDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
>;
pub type XML_UnparsedEntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
>;
pub type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type XML_DefaultHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_StartCdataSectionHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_CommentHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *const XML_Char) -> ()>;
pub type XML_CharacterDataHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_StartElementHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut *const XML_Char) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Memory_Handling_Suite {
    pub malloc_fcn: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub realloc_fcn:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
    pub free_fcn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type XML_Status = ::core::ffi::c_uint;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_ERROR: XML_Status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASH_TABLE_ITER {
    pub p: *mut *mut NAMED,
    pub end: *mut *mut NAMED,
}
pub type XML_Account = ::core::ffi::c_uint;
pub const XML_ACCOUNT_NONE: XML_Account = 2;
pub const XML_ACCOUNT_ENTITY_EXPANSION: XML_Account = 1;
pub const XML_ACCOUNT_DIRECT: XML_Account = 0;
pub type ICHAR = ::core::ffi::c_char;
pub const XML_ROLE_ELEMENT_NONE: C2RustUnnamed_0 = 39;
pub const XML_ROLE_ATTLIST_NONE: C2RustUnnamed_0 = 33;
pub const XML_ROLE_NOTATION_NONE: C2RustUnnamed_0 = 17;
pub const XML_ROLE_ENTITY_NONE: C2RustUnnamed_0 = 11;
pub const XML_ROLE_DOCTYPE_NONE: C2RustUnnamed_0 = 3;
pub const XML_ROLE_NONE: C2RustUnnamed_0 = 0;
pub const XML_ROLE_COMMENT: C2RustUnnamed_0 = 56;
pub const XML_ROLE_PI: C2RustUnnamed_0 = 55;
pub const XML_ROLE_GROUP_CLOSE_PLUS: C2RustUnnamed_0 = 48;
pub const XML_ROLE_GROUP_CLOSE_REP: C2RustUnnamed_0 = 46;
pub const XML_ROLE_GROUP_CLOSE_OPT: C2RustUnnamed_0 = 47;
pub const XML_ROLE_GROUP_CLOSE: C2RustUnnamed_0 = 45;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siphash {
    pub v0: uint64_t,
    pub v1: uint64_t,
    pub v2: uint64_t,
    pub v3: uint64_t,
    pub buf: [::core::ffi::c_uchar; 8],
    pub p: *mut ::core::ffi::c_uchar,
    pub c: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sipkey {
    pub k: [uint64_t; 2],
}
pub const XML_ROLE_CONTENT_ELEMENT_PLUS: C2RustUnnamed_0 = 54;
pub const XML_ROLE_CONTENT_ELEMENT_REP: C2RustUnnamed_0 = 52;
pub const XML_ROLE_CONTENT_ELEMENT_OPT: C2RustUnnamed_0 = 53;
pub const XML_ROLE_CONTENT_ELEMENT: C2RustUnnamed_0 = 51;
pub const XML_ROLE_CONTENT_PCDATA: C2RustUnnamed_0 = 43;
pub const XML_ROLE_CONTENT_ANY: C2RustUnnamed_0 = 41;
pub const XML_ROLE_CONTENT_EMPTY: C2RustUnnamed_0 = 42;
pub const XML_ROLE_ELEMENT_NAME: C2RustUnnamed_0 = 40;
pub const XML_ROLE_PARAM_ENTITY_REF: C2RustUnnamed_0 = 60;
pub const XML_ROLE_INNER_PARAM_ENTITY_REF: C2RustUnnamed_0 = 59;
pub const XML_ROLE_GROUP_CHOICE: C2RustUnnamed_0 = 49;
pub const XML_ROLE_GROUP_SEQUENCE: C2RustUnnamed_0 = 50;
pub const XML_ROLE_GROUP_OPEN: C2RustUnnamed_0 = 44;
pub const XML_ROLE_IGNORE_SECT: C2RustUnnamed_0 = 58;
pub const XML_ROLE_ERROR: C2RustUnnamed_0 = -1;
pub const XML_ROLE_NOTATION_NO_SYSTEM_ID: C2RustUnnamed_0 = 20;
pub const XML_ROLE_NOTATION_SYSTEM_ID: C2RustUnnamed_0 = 19;
pub const XML_ROLE_NOTATION_PUBLIC_ID: C2RustUnnamed_0 = 21;
pub const XML_ROLE_NOTATION_NAME: C2RustUnnamed_0 = 18;
pub const XML_ROLE_PARAM_ENTITY_NAME: C2RustUnnamed_0 = 10;
pub const XML_ROLE_GENERAL_ENTITY_NAME: C2RustUnnamed_0 = 9;
pub const XML_ROLE_ENTITY_NOTATION_NAME: C2RustUnnamed_0 = 16;
pub const XML_ROLE_ENTITY_COMPLETE: C2RustUnnamed_0 = 15;
pub const XML_ROLE_ENTITY_SYSTEM_ID: C2RustUnnamed_0 = 13;
pub const XML_ROLE_DOCTYPE_SYSTEM_ID: C2RustUnnamed_0 = 5;
pub const XML_ROLE_ENTITY_VALUE: C2RustUnnamed_0 = 12;
pub const XML_ROLE_FIXED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 38;
pub const XML_ROLE_DEFAULT_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 37;
pub const XML_ROLE_REQUIRED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 36;
pub const XML_ROLE_IMPLIED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 35;
pub const XML_ROLE_ATTRIBUTE_NOTATION_VALUE: C2RustUnnamed_0 = 32;
pub const XML_ROLE_ATTRIBUTE_ENUM_VALUE: C2RustUnnamed_0 = 31;
pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS: C2RustUnnamed_0 = 30;
pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN: C2RustUnnamed_0 = 29;
pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITIES: C2RustUnnamed_0 = 28;
pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ROLE_ATTRIBUTE_TYPE_IDREFS: C2RustUnnamed_0 = 26;
pub const XML_ROLE_ATTRIBUTE_TYPE_IDREF: C2RustUnnamed_0 = 25;
pub const XML_ROLE_ATTRIBUTE_TYPE_ID: C2RustUnnamed_0 = 24;
pub const XML_ROLE_ATTRIBUTE_TYPE_CDATA: C2RustUnnamed_0 = 23;
pub const XML_ROLE_ATTRIBUTE_NAME: C2RustUnnamed_0 = 22;
pub const XML_ROLE_ATTLIST_ELEMENT_NAME: C2RustUnnamed_0 = 34;
pub const XML_ROLE_INSTANCE_START: C2RustUnnamed_0 = 2;
pub const XML_ROLE_DOCTYPE_CLOSE: C2RustUnnamed_0 = 8;
pub const XML_ROLE_ENTITY_PUBLIC_ID: C2RustUnnamed_0 = 14;
pub const XML_ROLE_DOCTYPE_PUBLIC_ID: C2RustUnnamed_0 = 6;
pub type CONVERTER = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
pub const XML_ROLE_TEXT_DECL: C2RustUnnamed_0 = 57;
pub const XML_ROLE_DOCTYPE_INTERNAL_SUBSET: C2RustUnnamed_0 = 7;
pub const XML_ROLE_DOCTYPE_NAME: C2RustUnnamed_0 = 4;
pub const XML_ROLE_XML_DECL: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Expat_Version {
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub micro: ::core::ffi::c_int,
}
pub type XML_FeatureEnum = ::core::ffi::c_uint;
pub const XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: XML_FeatureEnum = 15;
pub const XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: XML_FeatureEnum = 14;
pub const XML_FEATURE_GE: XML_FeatureEnum = 13;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT:
    XML_FeatureEnum = 12;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT:
    XML_FeatureEnum = 11;
pub const XML_FEATURE_ATTR_INFO: XML_FeatureEnum = 10;
pub const XML_FEATURE_LARGE_SIZE: XML_FeatureEnum = 9;
pub const XML_FEATURE_NS: XML_FeatureEnum = 8;
pub const XML_FEATURE_SIZEOF_XML_LCHAR: XML_FeatureEnum = 7;
pub const XML_FEATURE_SIZEOF_XML_CHAR: XML_FeatureEnum = 6;
pub const XML_FEATURE_MIN_SIZE: XML_FeatureEnum = 5;
pub const XML_FEATURE_CONTEXT_BYTES: XML_FeatureEnum = 4;
pub const XML_FEATURE_DTD: XML_FeatureEnum = 3;
pub const XML_FEATURE_UNICODE_WCHAR_T: XML_FeatureEnum = 2;
pub const XML_FEATURE_UNICODE: XML_FeatureEnum = 1;
pub const XML_FEATURE_END: XML_FeatureEnum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Feature {
    pub feature: XML_FeatureEnum,
    pub name: *const XML_LChar,
    pub value: ::core::ffi::c_long,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_int;
static mut xmlLen: ::core::ffi::c_int = 0;
static mut xmlnsLen: ::core::ffi::c_int = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const UINT_MAX: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const ASCII_A: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const ASCII_C: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;
pub const ASCII_D: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;
pub const ASCII_E: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;
pub const ASCII_F: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;
pub const ASCII_I: ::core::ffi::c_int = 0x49 as ::core::ffi::c_int;
pub const ASCII_K: ::core::ffi::c_int = 0x4b as ::core::ffi::c_int;
pub const ASCII_L: ::core::ffi::c_int = 0x4c as ::core::ffi::c_int;
pub const ASCII_M: ::core::ffi::c_int = 0x4d as ::core::ffi::c_int;
pub const ASCII_N: ::core::ffi::c_int = 0x4e as ::core::ffi::c_int;
pub const ASCII_O: ::core::ffi::c_int = 0x4f as ::core::ffi::c_int;
pub const ASCII_R: ::core::ffi::c_int = 0x52 as ::core::ffi::c_int;
pub const ASCII_S: ::core::ffi::c_int = 0x53 as ::core::ffi::c_int;
pub const ASCII_T: ::core::ffi::c_int = 0x54 as ::core::ffi::c_int;
pub const ASCII_X: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;
pub const ASCII_Y: ::core::ffi::c_int = 0x59 as ::core::ffi::c_int;
pub const ASCII_a: ::core::ffi::c_int = 0x61 as ::core::ffi::c_int;
pub const ASCII_c: ::core::ffi::c_int = 0x63 as ::core::ffi::c_int;
pub const ASCII_e: ::core::ffi::c_int = 0x65 as ::core::ffi::c_int;
pub const ASCII_g: ::core::ffi::c_int = 0x67 as ::core::ffi::c_int;
pub const ASCII_h: ::core::ffi::c_int = 0x68 as ::core::ffi::c_int;
pub const ASCII_l: ::core::ffi::c_int = 0x6c as ::core::ffi::c_int;
pub const ASCII_m: ::core::ffi::c_int = 0x6d as ::core::ffi::c_int;
pub const ASCII_n: ::core::ffi::c_int = 0x6e as ::core::ffi::c_int;
pub const ASCII_o: ::core::ffi::c_int = 0x6f as ::core::ffi::c_int;
pub const ASCII_p: ::core::ffi::c_int = 0x70 as ::core::ffi::c_int;
pub const ASCII_r: ::core::ffi::c_int = 0x72 as ::core::ffi::c_int;
pub const ASCII_s: ::core::ffi::c_int = 0x73 as ::core::ffi::c_int;
pub const ASCII_t: ::core::ffi::c_int = 0x74 as ::core::ffi::c_int;
pub const ASCII_w: ::core::ffi::c_int = 0x77 as ::core::ffi::c_int;
pub const ASCII_x: ::core::ffi::c_int = 0x78 as ::core::ffi::c_int;
pub const ASCII_0: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;
pub const ASCII_1: ::core::ffi::c_int = 0x31 as ::core::ffi::c_int;
pub const ASCII_2: ::core::ffi::c_int = 0x32 as ::core::ffi::c_int;
pub const ASCII_3: ::core::ffi::c_int = 0x33 as ::core::ffi::c_int;
pub const ASCII_8: ::core::ffi::c_int = 0x38 as ::core::ffi::c_int;
pub const ASCII_9: ::core::ffi::c_int = 0x39 as ::core::ffi::c_int;
pub const ASCII_EXCL: ::core::ffi::c_int = 0x21 as ::core::ffi::c_int;
pub const ASCII_PERIOD: ::core::ffi::c_int = 0x2e as ::core::ffi::c_int;
pub const ASCII_COLON: ::core::ffi::c_int = 0x3a as ::core::ffi::c_int;
pub const ASCII_EQUALS: ::core::ffi::c_int = 0x3d as ::core::ffi::c_int;
pub const ASCII_LPAREN: ::core::ffi::c_int = 0x28 as ::core::ffi::c_int;
pub const ASCII_SLASH: ::core::ffi::c_int = 0x2f as ::core::ffi::c_int;
pub const ASCII_HASH: ::core::ffi::c_int = 0x23 as ::core::ffi::c_int;
pub const ASCII_PIPE: ::core::ffi::c_int = 0x7c as ::core::ffi::c_int;
pub const ASCII_COMMA: ::core::ffi::c_int = 0x2c as ::core::ffi::c_int;
pub const UINTPTR_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const SIZE_MAX: ::core::ffi::c_ulong = UINTPTR_MAX;
#[inline(always)]
unsafe extern "C" fn __inline_isnanf(mut __x: ::core::ffi::c_float) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnand(mut __x: ::core::ffi::c_double) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnanl(mut __x: f64) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const XML_MAJOR_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XML_MINOR_VERSION: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const XML_MICRO_VERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
unsafe extern "C" fn sip_tokey(
    mut key: *mut sipkey,
    mut src: *const ::core::ffi::c_void,
) -> *mut sipkey {
    (*key).k[0 as ::core::ffi::c_int as usize] = (*(src as *const ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint64_t)
        << 0 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar).offset(1 as ::core::ffi::c_int as isize)
            as uint64_t)
            << 8 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize)
            as uint64_t)
            << 16 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar).offset(3 as ::core::ffi::c_int as isize)
            as uint64_t)
            << 24 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar).offset(4 as ::core::ffi::c_int as isize)
            as uint64_t)
            << 32 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar).offset(5 as ::core::ffi::c_int as isize)
            as uint64_t)
            << 40 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar).offset(6 as ::core::ffi::c_int as isize)
            as uint64_t)
            << 48 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar).offset(7 as ::core::ffi::c_int as isize)
            as uint64_t)
            << 56 as ::core::ffi::c_int;
    (*key).k[1 as ::core::ffi::c_int as usize] = (*(src as *const ::core::ffi::c_uchar)
        .offset(8 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint64_t)
        << 0 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint64_t)
            << 8 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint64_t)
            << 16 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint64_t)
            << 24 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(4 as ::core::ffi::c_int as isize) as uint64_t)
            << 32 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(5 as ::core::ffi::c_int as isize) as uint64_t)
            << 40 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(6 as ::core::ffi::c_int as isize) as uint64_t)
            << 48 as ::core::ffi::c_int
        | (*(src as *const ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(7 as ::core::ffi::c_int as isize) as uint64_t)
            << 56 as ::core::ffi::c_int;
    return key;
}
unsafe extern "C" fn sip_round(mut H: *mut siphash, rounds: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < rounds {
        (*H).v0 = (*H).v0.wrapping_add((*H).v1);
        (*H).v1 = (*H).v1 << 13 as ::core::ffi::c_int
            | (*H).v1 >> 64 as ::core::ffi::c_int - 13 as ::core::ffi::c_int;
        (*H).v1 ^= (*H).v0;
        (*H).v0 = (*H).v0 << 32 as ::core::ffi::c_int
            | (*H).v0 >> 64 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
        (*H).v2 = (*H).v2.wrapping_add((*H).v3);
        (*H).v3 = (*H).v3 << 16 as ::core::ffi::c_int
            | (*H).v3 >> 64 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        (*H).v3 ^= (*H).v2;
        (*H).v0 = (*H).v0.wrapping_add((*H).v3);
        (*H).v3 = (*H).v3 << 21 as ::core::ffi::c_int
            | (*H).v3 >> 64 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        (*H).v3 ^= (*H).v0;
        (*H).v2 = (*H).v2.wrapping_add((*H).v1);
        (*H).v1 = (*H).v1 << 17 as ::core::ffi::c_int
            | (*H).v1 >> 64 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        (*H).v1 ^= (*H).v2;
        (*H).v2 = (*H).v2 << 32 as ::core::ffi::c_int
            | (*H).v2 >> 64 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
        i += 1;
    }
}
unsafe extern "C" fn sip24_init(mut H: *mut siphash, mut key: *const sipkey) -> *mut siphash {
    (*H).v0 = ((0x736f6d65 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
        | 0x70736575 as uint64_t)
        ^ (*key).k[0 as ::core::ffi::c_int as usize];
    (*H).v1 = ((0x646f7261 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
        | 0x6e646f6d as uint64_t)
        ^ (*key).k[1 as ::core::ffi::c_int as usize];
    (*H).v2 = ((0x6c796765 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
        | 0x6e657261 as uint64_t)
        ^ (*key).k[0 as ::core::ffi::c_int as usize];
    (*H).v3 = ((0x74656462 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
        | 0x79746573 as uint64_t)
        ^ (*key).k[1 as ::core::ffi::c_int as usize];
    (*H).p = &raw mut (*H).buf as *mut ::core::ffi::c_uchar;
    (*H).c = 0 as uint64_t;
    return H;
}
unsafe extern "C" fn sip24_update(
    mut H: *mut siphash,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) -> *mut siphash {
    let mut p: *const ::core::ffi::c_uchar = src as *const ::core::ffi::c_uchar;
    let mut pe: *const ::core::ffi::c_uchar = p.offset(len as isize);
    let mut m: uint64_t = 0;
    loop {
        while p < pe
            && (*H).p
                < (&raw mut (*H).buf as *mut ::core::ffi::c_uchar).offset(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize)
                        .wrapping_div(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                        as isize,
                ) as *mut ::core::ffi::c_uchar
        {
            let fresh20 = p;
            p = p.offset(1);
            let fresh21 = (*H).p;
            (*H).p = (*H).p.offset(1);
            *fresh21 = *fresh20;
        }
        if (*H).p
            < (&raw mut (*H).buf as *mut ::core::ffi::c_uchar).offset(
                (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                    as isize,
            ) as *mut ::core::ffi::c_uchar
        {
            break;
        }
        m = ((*H).buf[0 as ::core::ffi::c_int as usize] as uint64_t) << 0 as ::core::ffi::c_int
            | ((*H).buf[1 as ::core::ffi::c_int as usize] as uint64_t) << 8 as ::core::ffi::c_int
            | ((*H).buf[2 as ::core::ffi::c_int as usize] as uint64_t) << 16 as ::core::ffi::c_int
            | ((*H).buf[3 as ::core::ffi::c_int as usize] as uint64_t) << 24 as ::core::ffi::c_int
            | ((*H).buf[4 as ::core::ffi::c_int as usize] as uint64_t) << 32 as ::core::ffi::c_int
            | ((*H).buf[5 as ::core::ffi::c_int as usize] as uint64_t) << 40 as ::core::ffi::c_int
            | ((*H).buf[6 as ::core::ffi::c_int as usize] as uint64_t) << 48 as ::core::ffi::c_int
            | ((*H).buf[7 as ::core::ffi::c_int as usize] as uint64_t) << 56 as ::core::ffi::c_int;
        (*H).v3 ^= m;
        sip_round(H, 2 as ::core::ffi::c_int);
        (*H).v0 ^= m;
        (*H).p = &raw mut (*H).buf as *mut ::core::ffi::c_uchar;
        (*H).c = (*H).c.wrapping_add(8 as uint64_t);
        if !(p < pe) {
            break;
        }
    }
    return H;
}
unsafe extern "C" fn sip24_final(mut H: *mut siphash) -> uint64_t {
    let left: ::core::ffi::c_char = (*H)
        .p
        .offset_from(&raw mut (*H).buf as *mut ::core::ffi::c_uchar)
        as ::core::ffi::c_long as ::core::ffi::c_char;
    let mut b: uint64_t = (*H).c.wrapping_add(left as uint64_t) << 56 as ::core::ffi::c_int;
    let mut current_block_6: u64;
    match left as ::core::ffi::c_int {
        7 => {
            b |= ((*H).buf[6 as ::core::ffi::c_int as usize] as uint64_t)
                << 48 as ::core::ffi::c_int;
            current_block_6 = 15874625638871904877;
        }
        6 => {
            current_block_6 = 15874625638871904877;
        }
        5 => {
            current_block_6 = 5060243397358390687;
        }
        4 => {
            current_block_6 = 14296497066650258724;
        }
        3 => {
            current_block_6 = 14344297545545190320;
        }
        2 => {
            current_block_6 = 16528646733863252401;
        }
        1 => {
            current_block_6 = 8494353081971656926;
        }
        0 | _ => {
            current_block_6 = 5720623009719927633;
        }
    }
    match current_block_6 {
        15874625638871904877 => {
            b |= ((*H).buf[5 as ::core::ffi::c_int as usize] as uint64_t)
                << 40 as ::core::ffi::c_int;
            current_block_6 = 5060243397358390687;
        }
        _ => {}
    }
    match current_block_6 {
        5060243397358390687 => {
            b |= ((*H).buf[4 as ::core::ffi::c_int as usize] as uint64_t)
                << 32 as ::core::ffi::c_int;
            current_block_6 = 14296497066650258724;
        }
        _ => {}
    }
    match current_block_6 {
        14296497066650258724 => {
            b |= ((*H).buf[3 as ::core::ffi::c_int as usize] as uint64_t)
                << 24 as ::core::ffi::c_int;
            current_block_6 = 14344297545545190320;
        }
        _ => {}
    }
    match current_block_6 {
        14344297545545190320 => {
            b |= ((*H).buf[2 as ::core::ffi::c_int as usize] as uint64_t)
                << 16 as ::core::ffi::c_int;
            current_block_6 = 16528646733863252401;
        }
        _ => {}
    }
    match current_block_6 {
        16528646733863252401 => {
            b |=
                ((*H).buf[1 as ::core::ffi::c_int as usize] as uint64_t) << 8 as ::core::ffi::c_int;
            current_block_6 = 8494353081971656926;
        }
        _ => {}
    }
    match current_block_6 {
        8494353081971656926 => {
            b |=
                ((*H).buf[0 as ::core::ffi::c_int as usize] as uint64_t) << 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    (*H).v3 ^= b;
    sip_round(H, 2 as ::core::ffi::c_int);
    (*H).v0 ^= b;
    (*H).v2 ^= 0xff as uint64_t;
    sip_round(H, 4 as ::core::ffi::c_int);
    return (*H).v0 ^ (*H).v1 ^ (*H).v2 ^ (*H).v3;
}
unsafe extern "C" fn siphash24(
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
    mut key: *const sipkey,
) -> uint64_t {
    let mut state: siphash = siphash {
        v0: 0 as uint64_t,
        v1: 0 as uint64_t,
        v2: 0 as uint64_t,
        v3: 0 as uint64_t,
        buf: [
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        p: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        c: 0 as uint64_t,
    };
    return sip24_final(sip24_update(sip24_init(&raw mut state, key), src, len));
}
unsafe extern "C" fn sip24_valid() -> ::core::ffi::c_int {
    static mut vectors: [[::core::ffi::c_uchar; 8]; 64] = [
        [
            0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xfd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x74 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x2d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x85 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xab as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x82 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7a as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xfb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x86 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x84 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x69 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x9c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4b as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xbd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x88 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xc8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x85 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xde as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xa5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xac as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xde as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xef as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfa as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x78 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xae as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x81 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xeb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x74 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x6f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x81 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xec as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xbd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x83 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x3a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x2b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xe1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x91 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xb1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xec as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6d as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x9f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x1b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
        [
            0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xeb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x8a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ],
    ];
    let mut in_0: [::core::ffi::c_uchar; 64] = [0; 64];
    let mut k: sipkey = sipkey { k: [0; 2] };
    let mut i: size_t = 0;
    sip_tokey(
        &raw mut k,
        b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F\0" as *const u8
            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
    );
    i = 0 as size_t;
    while i < ::core::mem::size_of::<[::core::ffi::c_uchar; 64]>() as usize {
        in_0[i as usize] = i as ::core::ffi::c_uchar;
        if siphash24(
            &raw mut in_0 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            i,
            &raw mut k,
        ) != (vectors[i as usize][0 as ::core::ffi::c_int as usize] as uint64_t)
            << 0 as ::core::ffi::c_int
            | (vectors[i as usize][1 as ::core::ffi::c_int as usize] as uint64_t)
                << 8 as ::core::ffi::c_int
            | (vectors[i as usize][2 as ::core::ffi::c_int as usize] as uint64_t)
                << 16 as ::core::ffi::c_int
            | (vectors[i as usize][3 as ::core::ffi::c_int as usize] as uint64_t)
                << 24 as ::core::ffi::c_int
            | (vectors[i as usize][4 as ::core::ffi::c_int as usize] as uint64_t)
                << 32 as ::core::ffi::c_int
            | (vectors[i as usize][5 as ::core::ffi::c_int as usize] as uint64_t)
                << 40 as ::core::ffi::c_int
            | (vectors[i as usize][6 as ::core::ffi::c_int as usize] as uint64_t)
                << 48 as ::core::ffi::c_int
            | (vectors[i as usize][7 as ::core::ffi::c_int as usize] as uint64_t)
                << 56 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
pub const EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT:
    ::core::ffi::c_float = 100.0f32;
pub const EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT: ::core::ffi::c_int =
    8388608 as ::core::ffi::c_int;
pub const EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: ::core::ffi::c_float = 100.0f32;
pub const EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: ::core::ffi::c_int =
    67108864 as ::core::ffi::c_int;
pub const EXPAT_MALLOC_ALIGNMENT: usize = ::core::mem::size_of::<::core::ffi::c_longlong>();
pub const EXPAT_MALLOC_PADDING: usize = (::core::mem::size_of::<::core::ffi::c_longlong>()
    as usize)
    .wrapping_sub(::core::mem::size_of::<size_t>() as usize);
pub const XML_TOK_TRAILING_RSQB: ::core::ffi::c_int = -5;
pub const XML_TOK_NONE: ::core::ffi::c_int = -4;
pub const XML_TOK_TRAILING_CR: ::core::ffi::c_int = -3;
pub const XML_TOK_PARTIAL_CHAR: ::core::ffi::c_int = -2;
pub const XML_TOK_PARTIAL: ::core::ffi::c_int = -1;
pub const XML_TOK_INVALID: ::core::ffi::c_int = 0;
pub const XML_TOK_START_TAG_WITH_ATTS: ::core::ffi::c_int = 1;
pub const XML_TOK_START_TAG_NO_ATTS: ::core::ffi::c_int = 2;
pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: ::core::ffi::c_int = 3;
pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: ::core::ffi::c_int = 4;
pub const XML_TOK_END_TAG: ::core::ffi::c_int = 5;
pub const XML_TOK_DATA_CHARS: ::core::ffi::c_int = 6;
pub const XML_TOK_DATA_NEWLINE: ::core::ffi::c_int = 7;
pub const XML_TOK_CDATA_SECT_OPEN: ::core::ffi::c_int = 8;
pub const XML_TOK_ENTITY_REF: ::core::ffi::c_int = 9;
pub const XML_TOK_CHAR_REF: ::core::ffi::c_int = 10;
pub const XML_TOK_PI: ::core::ffi::c_int = 11;
pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12;
pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13;
pub const XML_TOK_BOM: ::core::ffi::c_int = 14;
pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15;
pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28;
pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const XML_TOK_ATTRIBUTE_VALUE_S: ::core::ffi::c_int = 39;
pub const XML_TOK_CDATA_SECT_CLOSE: ::core::ffi::c_int = 40;
pub const XML_TOK_IGNORE_SECT: ::core::ffi::c_int = 42;
pub const INIT_TAG_BUF_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const INIT_DATA_BUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const INIT_ATTS_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const INIT_ATTS_VERSION: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const INIT_BLOCK_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const INIT_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const EXPAND_SPARE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const INIT_SCAFFOLD_ELEMENTS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[no_mangle]
pub static mut g_reparseDeferralEnabledDefault: XML_Bool = XML_TRUE;
unsafe extern "C" fn expat_heap_stat(
    mut rootParser: XML_Parser,
    mut operator: ::core::ffi::c_char,
    mut absDiff: XmlBigCount,
    mut newTotal: XmlBigCount,
    mut peakTotal: XmlBigCount,
    mut sourceLine: ::core::ffi::c_int,
) {
    let amplification: ::core::ffi::c_float = newTotal as ::core::ffi::c_float
        / (*rootParser).m_accounting.countBytesDirect as ::core::ffi::c_float;
    fprintf(
        __stderrp,
        b"expat: Allocations(%p): Direct %10llu, allocated %c%10llu to %10llu (%10llu peak), amplification %8.2f (xmlparse.c:%d)\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        rootParser as *mut ::core::ffi::c_void,
        (*rootParser).m_accounting.countBytesDirect,
        operator as ::core::ffi::c_int,
        absDiff,
        newTotal,
        peakTotal,
        amplification as ::core::ffi::c_double,
        sourceLine,
    );
}
unsafe extern "C" fn expat_heap_increase_tolerable(
    mut rootParser: XML_Parser,
    mut increase: XmlBigCount,
    mut sourceLine: ::core::ffi::c_int,
) -> bool {
    if rootParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"expat_heap_increase_tolerable\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            817 as ::core::ffi::c_int,
            b"rootParser != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(increase > 0 as XmlBigCount) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"expat_heap_increase_tolerable\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            818 as ::core::ffi::c_int,
            b"increase > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    let mut newTotal: XmlBigCount = 0 as XmlBigCount;
    let mut tolerable: bool = true_0 != 0;
    if (-(1 as ::core::ffi::c_int) as XmlBigCount)
        .wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
        < increase
    {
        tolerable = false_0 != 0;
    } else {
        newTotal = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_add(increase);
        if newTotal >= (*rootParser).m_alloc_tracker.activationThresholdBytes {
            if !(newTotal > 0 as XmlBigCount) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"expat_heap_increase_tolerable\0" as *const u8 as *const ::core::ffi::c_char,
                    b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                    830 as ::core::ffi::c_int,
                    b"newTotal > 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            let amplification: ::core::ffi::c_float = newTotal as ::core::ffi::c_float
                / (*rootParser).m_accounting.countBytesDirect as ::core::ffi::c_float;
            if amplification > (*rootParser).m_alloc_tracker.maximumAmplificationFactor {
                tolerable = false_0 != 0;
            }
        }
    }
    if !tolerable && (*rootParser).m_alloc_tracker.debugLevel >= 1 as ::core::ffi::c_ulong {
        expat_heap_stat(
            rootParser,
            '+' as i32 as ::core::ffi::c_char,
            increase,
            newTotal,
            newTotal,
            sourceLine,
        );
    }
    return tolerable;
}
unsafe extern "C" fn expat_malloc(
    mut parser: XML_Parser,
    mut size: size_t,
    mut sourceLine: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    if (SIZE_MAX as size_t).wrapping_sub(size)
        < (::core::mem::size_of::<size_t>() as usize).wrapping_add(EXPAT_MALLOC_PADDING)
    {
        return NULL;
    }
    let rootParser: XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"expat_malloc\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            860 as ::core::ffi::c_int,
            b"rootParser->m_parentParser == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    let bytesToAllocate: size_t = (::core::mem::size_of::<size_t>() as size_t)
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(size);
    if (-(1 as ::core::ffi::c_int) as XmlBigCount)
        .wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
        < bytesToAllocate as XmlBigCount
    {
        return NULL;
    }
    if !expat_heap_increase_tolerable(rootParser, bytesToAllocate as XmlBigCount, sourceLine) {
        return NULL;
    }
    let mallocedPtr: *mut ::core::ffi::c_void = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(
        bytesToAllocate
    ) as *mut ::core::ffi::c_void;
    if mallocedPtr.is_null() {
        return NULL;
    }
    *(mallocedPtr as *mut size_t) = size;
    (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_add(bytesToAllocate as XmlBigCount);
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 as ::core::ffi::c_ulong {
        if (*rootParser).m_alloc_tracker.bytesAllocated
            > (*rootParser).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser).m_alloc_tracker.peakBytesAllocated =
                (*rootParser).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            rootParser,
            '+' as i32 as ::core::ffi::c_char,
            bytesToAllocate as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    return (mallocedPtr as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<size_t>() as usize as isize)
        .offset(EXPAT_MALLOC_PADDING as isize) as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn expat_free(
    mut parser: XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut sourceLine: ::core::ffi::c_int,
) {
    if parser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"expat_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            908 as ::core::ffi::c_int,
            b"parser != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if ptr.is_null() {
        return;
    }
    let rootParser: XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"expat_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            915 as ::core::ffi::c_int,
            b"rootParser->m_parentParser == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    let mallocedPtr: *mut ::core::ffi::c_void = (ptr as *mut ::core::ffi::c_char)
        .offset(-(EXPAT_MALLOC_PADDING as isize))
        .offset(-(::core::mem::size_of::<size_t>() as usize as isize))
        as *mut ::core::ffi::c_void;
    let bytesAllocated: size_t = (::core::mem::size_of::<size_t>() as size_t)
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(*(mallocedPtr as *mut size_t));
    if !((*rootParser).m_alloc_tracker.bytesAllocated >= bytesAllocated as XmlBigCount)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"expat_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            924 as ::core::ffi::c_int,
            b"rootParser->m_alloc_tracker.bytesAllocated >= bytesAllocated\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_sub(bytesAllocated as XmlBigCount);
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 as ::core::ffi::c_ulong {
        expat_heap_stat(
            rootParser,
            '-' as i32 as ::core::ffi::c_char,
            bytesAllocated as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(mallocedPtr);
}
unsafe extern "C" fn expat_realloc(
    mut parser: XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut sourceLine: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    if parser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"expat_realloc\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            944 as ::core::ffi::c_int,
            b"parser != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if ptr.is_null() {
        return expat_malloc(parser, size, sourceLine);
    }
    if size == 0 as size_t {
        expat_free(parser, ptr, sourceLine);
        return NULL;
    }
    let rootParser: XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"expat_realloc\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            956 as ::core::ffi::c_int,
            b"rootParser->m_parentParser == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    let mut mallocedPtr: *mut ::core::ffi::c_void = (ptr as *mut ::core::ffi::c_char)
        .offset(-(EXPAT_MALLOC_PADDING as isize))
        .offset(-(::core::mem::size_of::<size_t>() as usize as isize))
        as *mut ::core::ffi::c_void;
    let prevSize: size_t = *(mallocedPtr as *mut size_t);
    let isIncrease: bool = size > prevSize;
    let absDiff: size_t = if size > prevSize {
        size.wrapping_sub(prevSize)
    } else {
        prevSize.wrapping_sub(size)
    };
    if isIncrease {
        if !expat_heap_increase_tolerable(rootParser, absDiff as XmlBigCount, sourceLine) {
            return NULL;
        }
    }
    if !((18446744073709551615 as usize)
        .wrapping_sub(::core::mem::size_of::<size_t>() as usize)
        .wrapping_sub(
            (::core::mem::size_of::<::core::ffi::c_longlong>() as usize)
                .wrapping_sub(::core::mem::size_of::<size_t>() as usize),
        )
        >= size) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"expat_realloc\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            977 as ::core::ffi::c_int,
            b"SIZE_MAX - sizeof(size_t) - EXPAT_MALLOC_PADDING >= size\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    mallocedPtr = (*parser)
        .m_mem
        .realloc_fcn
        .expect("non-null function pointer")(
        mallocedPtr,
        (::core::mem::size_of::<size_t>() as size_t)
            .wrapping_add(EXPAT_MALLOC_PADDING)
            .wrapping_add(size),
    );
    if mallocedPtr.is_null() {
        return NULL;
    }
    if isIncrease {
        if !((-(1 as ::core::ffi::c_int) as XmlBigCount)
            .wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
            >= absDiff as XmlBigCount) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"expat_realloc\0" as *const u8 as *const ::core::ffi::c_char,
                b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                990 as ::core::ffi::c_int,
                b"(XmlBigCount)-1 - rootParser->m_alloc_tracker.bytesAllocated >= absDiff\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_add(absDiff as XmlBigCount);
    } else {
        if !((*rootParser).m_alloc_tracker.bytesAllocated >= absDiff as XmlBigCount)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"expat_realloc\0" as *const u8 as *const ::core::ffi::c_char,
                b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                993 as ::core::ffi::c_int,
                b"rootParser->m_alloc_tracker.bytesAllocated >= absDiff\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else {
        };
        (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_sub(absDiff as XmlBigCount);
    }
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 as ::core::ffi::c_ulong {
        if (*rootParser).m_alloc_tracker.bytesAllocated
            > (*rootParser).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser).m_alloc_tracker.peakBytesAllocated =
                (*rootParser).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            rootParser,
            (if isIncrease as ::core::ffi::c_int != 0 {
                '+' as i32
            } else {
                '-' as i32
            }) as ::core::ffi::c_char,
            absDiff as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    *(mallocedPtr as *mut size_t) = size;
    return (mallocedPtr as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<size_t>() as usize as isize)
        .offset(EXPAT_MALLOC_PADDING as isize) as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate(mut encodingName: *const XML_Char) -> XML_Parser {
    return XML_ParserCreate_MM(
        encodingName,
        ::core::ptr::null::<XML_Memory_Handling_Suite>(),
        ::core::ptr::null::<XML_Char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreateNS(
    mut encodingName: *const XML_Char,
    mut nsSep: XML_Char,
) -> XML_Parser {
    let mut tmp: [XML_Char; 2] = [nsSep, 0 as ::core::ffi::c_int as XML_Char];
    return XML_ParserCreate_MM(
        encodingName,
        ::core::ptr::null::<XML_Memory_Handling_Suite>(),
        &raw mut tmp as *mut XML_Char,
    );
}
static mut implicitContext: [XML_Char; 41] = [
    ASCII_x as XML_Char,
    ASCII_m as XML_Char,
    ASCII_l as XML_Char,
    ASCII_EQUALS as XML_Char,
    ASCII_h as XML_Char,
    ASCII_t as XML_Char,
    ASCII_t as XML_Char,
    ASCII_p as XML_Char,
    ASCII_COLON as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_w as XML_Char,
    ASCII_w as XML_Char,
    ASCII_w as XML_Char,
    ASCII_PERIOD as XML_Char,
    ASCII_w as XML_Char,
    ASCII_3 as XML_Char,
    ASCII_PERIOD as XML_Char,
    ASCII_o as XML_Char,
    ASCII_r as XML_Char,
    ASCII_g as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_X as XML_Char,
    ASCII_M as XML_Char,
    ASCII_L as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_1 as XML_Char,
    ASCII_9 as XML_Char,
    ASCII_9 as XML_Char,
    ASCII_8 as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_n as XML_Char,
    ASCII_a as XML_Char,
    ASCII_m as XML_Char,
    ASCII_e as XML_Char,
    ASCII_s as XML_Char,
    ASCII_p as XML_Char,
    ASCII_a as XML_Char,
    ASCII_c as XML_Char,
    ASCII_e as XML_Char,
    '\0' as i32 as XML_Char,
];
unsafe extern "C" fn ENTROPY_DEBUG(
    mut label: *const ::core::ffi::c_char,
    mut entropy: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    if getDebugLevel(
        b"EXPAT_ENTROPY_DEBUG\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_ulong,
    ) >= 1 as ::core::ffi::c_ulong
    {
        fprintf(
            __stderrp,
            b"expat: Entropy: %s --> 0x%0*lx (%lu bytes)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            label,
            ::core::mem::size_of::<::core::ffi::c_ulong>() as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int,
            entropy,
            ::core::mem::size_of::<::core::ffi::c_ulong>() as ::core::ffi::c_ulong,
        );
    }
    return entropy;
}
unsafe extern "C" fn generate_hash_secret_salt(mut parser: XML_Parser) -> ::core::ffi::c_ulong {
    let mut entropy: ::core::ffi::c_ulong = 0;
    arc4random_buf(
        &raw mut entropy as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_ulong>() as size_t,
    );
    return ENTROPY_DEBUG(
        b"arc4random_buf\0" as *const u8 as *const ::core::ffi::c_char,
        entropy,
    );
}
unsafe extern "C" fn get_hash_secret_salt(mut parser: XML_Parser) -> ::core::ffi::c_ulong {
    let rootParser: XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"get_hash_secret_salt\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            1253 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return (*rootParser).m_hash_secret_salt;
}
unsafe extern "C" fn callProcessor(
    mut parser: XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let have_now: size_t = (if !end.is_null() && !start.is_null() {
        end.offset_from(start) as ::core::ffi::c_long
    } else {
        0 as ::core::ffi::c_long
    }) as size_t;
    if (*parser).m_reparseDeferralEnabled as ::core::ffi::c_int != 0
        && (*parser).m_parsingStatus.finalBuffer == 0
    {
        let had_before: size_t = (*parser).m_partialTokenBytesBefore;
        let mut available_buffer: size_t =
            (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
            } else {
                0 as ::core::ffi::c_long
            }) as size_t;
        available_buffer = available_buffer.wrapping_sub(if available_buffer < 1024 as size_t {
            available_buffer
        } else {
            1024 as size_t
        });
        available_buffer = available_buffer.wrapping_add(
            (if !(*parser).m_bufferLim.is_null() && !(*parser).m_bufferEnd.is_null() {
                (*parser).m_bufferLim.offset_from((*parser).m_bufferEnd) as ::core::ffi::c_long
            } else {
                0 as ::core::ffi::c_long
            }) as size_t,
        );
        let enough: bool = have_now >= (2 as size_t).wrapping_mul(had_before)
            || (*parser).m_lastBufferRequestSize as size_t > available_buffer;
        if !enough {
            *endPtr = start;
            return XML_ERROR_NONE;
        }
    }
    let mut ret: XML_Error = XML_ERROR_NONE;
    *endPtr = start;
    loop {
        ret =
            (*parser).m_processor.expect("non-null function pointer")(parser, *endPtr, end, endPtr);
        if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
            != XML_PARSING as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*parser).m_reenter = XML_FALSE;
        }
        if (*parser).m_reenter == 0 {
            break;
        }
        (*parser).m_reenter = XML_FALSE;
        if ret as ::core::ffi::c_uint != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return ret;
        }
    }
    if ret as ::core::ffi::c_uint == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint {
        if *endPtr == start {
            (*parser).m_partialTokenBytesBefore = have_now;
        } else {
            (*parser).m_partialTokenBytesBefore = 0 as size_t;
        }
    }
    return ret;
}
unsafe extern "C" fn startParsing(mut parser: XML_Parser) -> XML_Bool {
    if (*parser).m_hash_secret_salt == 0 as ::core::ffi::c_ulong {
        (*parser).m_hash_secret_salt = generate_hash_secret_salt(parser);
    }
    if (*parser).m_ns != 0 {
        return setContext(parser, &raw const implicitContext as *const XML_Char);
    }
    return XML_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_ParserCreate_MM(
    mut encodingName: *const XML_Char,
    mut memsuite: *const XML_Memory_Handling_Suite,
    mut nameSep: *const XML_Char,
) -> XML_Parser {
    return parserCreate(
        encodingName,
        memsuite,
        nameSep,
        ::core::ptr::null_mut::<DTD>(),
        ::core::ptr::null_mut::<XML_ParserStruct>(),
    );
}
unsafe extern "C" fn parserCreate(
    mut encodingName: *const XML_Char,
    mut memsuite: *const XML_Memory_Handling_Suite,
    mut nameSep: *const XML_Char,
    mut dtd: *mut DTD,
    mut parentParser: XML_Parser,
) -> XML_Parser {
    let mut parser: XML_Parser = ::core::ptr::null_mut::<XML_ParserStruct>();
    let increase: size_t = (::core::mem::size_of::<size_t>() as size_t)
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(::core::mem::size_of::<XML_ParserStruct>() as size_t);
    if !parentParser.is_null() {
        let rootParser: XML_Parser =
            getRootParserOf(parentParser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
                as XML_Parser;
        if !expat_heap_increase_tolerable(
            rootParser,
            increase as XmlBigCount,
            1356 as ::core::ffi::c_int,
        ) {
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
    }
    if !memsuite.is_null() {
        let mut mtemp: *mut XML_Memory_Handling_Suite =
            ::core::ptr::null_mut::<XML_Memory_Handling_Suite>();
        let sizeAndParser: *mut ::core::ffi::c_void =
            (*memsuite).malloc_fcn.expect("non-null function pointer")(
                (::core::mem::size_of::<size_t>() as size_t)
                    .wrapping_add(EXPAT_MALLOC_PADDING)
                    .wrapping_add(::core::mem::size_of::<XML_ParserStruct>() as size_t),
            ) as *mut ::core::ffi::c_void;
        if !sizeAndParser.is_null() {
            *(sizeAndParser as *mut size_t) =
                ::core::mem::size_of::<XML_ParserStruct>() as usize as size_t;
            parser = (sizeAndParser as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<size_t>() as usize as isize)
                .offset(EXPAT_MALLOC_PADDING as isize) as XML_Parser;
            mtemp = &raw const (*parser).m_mem as *mut XML_Memory_Handling_Suite;
            (*mtemp).malloc_fcn = (*memsuite).malloc_fcn;
            (*mtemp).realloc_fcn = (*memsuite).realloc_fcn;
            (*mtemp).free_fcn = (*memsuite).free_fcn;
        }
    } else {
        let mut mtemp_0: *mut XML_Memory_Handling_Suite =
            ::core::ptr::null_mut::<XML_Memory_Handling_Suite>();
        let sizeAndParser_0: *mut ::core::ffi::c_void = malloc(
            (::core::mem::size_of::<size_t>() as size_t)
                .wrapping_add(EXPAT_MALLOC_PADDING)
                .wrapping_add(::core::mem::size_of::<XML_ParserStruct>() as size_t),
        ) as *mut ::core::ffi::c_void;
        if !sizeAndParser_0.is_null() {
            *(sizeAndParser_0 as *mut size_t) =
                ::core::mem::size_of::<XML_ParserStruct>() as usize as size_t;
            parser = (sizeAndParser_0 as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<size_t>() as usize as isize)
                .offset(EXPAT_MALLOC_PADDING as isize) as XML_Parser;
            mtemp_0 = &raw const (*parser).m_mem as *mut XML_Memory_Handling_Suite;
            (*mtemp_0).malloc_fcn =
                Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
                    as Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
            (*mtemp_0).realloc_fcn = Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
                >;
            (*mtemp_0).free_fcn = Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        }
    }
    if parser.is_null() {
        return parser;
    }
    memset(
        &raw mut (*parser).m_alloc_tracker as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MALLOC_TRACKER>() as size_t,
    );
    if parentParser.is_null() {
        (*parser).m_alloc_tracker.debugLevel = getDebugLevel(
            b"EXPAT_MALLOC_DEBUG\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_ulong,
        );
        (*parser).m_alloc_tracker.maximumAmplificationFactor =
            EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT;
        (*parser).m_alloc_tracker.activationThresholdBytes =
            EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT as XmlBigCount;
        (*parser).m_parentParser = ::core::ptr::null_mut::<XML_ParserStruct>();
        (*parser).m_accounting.countBytesDirect = 0 as XmlBigCount;
    } else {
        (*parser).m_parentParser = parentParser;
    }
    let rootParser_0: XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser_0).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"parserCreate\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            1427 as ::core::ffi::c_int,
            b"rootParser->m_parentParser == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((18446744073709551615 as XmlBigCount)
        .wrapping_sub((*rootParser_0).m_alloc_tracker.bytesAllocated)
        >= increase as XmlBigCount) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"parserCreate\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            1428 as ::core::ffi::c_int,
            b"SIZE_MAX - rootParser->m_alloc_tracker.bytesAllocated >= increase\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*rootParser_0).m_alloc_tracker.bytesAllocated = (*rootParser_0)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_add(increase as XmlBigCount);
    if (*rootParser_0).m_alloc_tracker.debugLevel >= 2 as ::core::ffi::c_ulong {
        if (*rootParser_0).m_alloc_tracker.bytesAllocated
            > (*rootParser_0).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser_0).m_alloc_tracker.peakBytesAllocated =
                (*rootParser_0).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            rootParser_0,
            '+' as i32 as ::core::ffi::c_char,
            increase as XmlBigCount,
            (*rootParser_0).m_alloc_tracker.bytesAllocated,
            (*rootParser_0).m_alloc_tracker.peakBytesAllocated,
            1441 as ::core::ffi::c_int,
        );
    }
    (*parser).m_buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*parser).m_bufferLim = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_attsSize = INIT_ATTS_SIZE;
    (*parser).m_atts = expat_malloc(
        parser,
        ((*parser).m_attsSize as size_t)
            .wrapping_mul(::core::mem::size_of::<ATTRIBUTE>() as size_t),
        1451 as ::core::ffi::c_int,
    ) as *mut ATTRIBUTE;
    if (*parser).m_atts.is_null() {
        expat_free(
            parser,
            parser as *mut ::core::ffi::c_void,
            1453 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    (*parser).m_dataBuf = expat_malloc(
        parser,
        (1024 as size_t).wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
        1464 as ::core::ffi::c_int,
    ) as *mut XML_Char;
    if (*parser).m_dataBuf.is_null() {
        expat_free(
            parser,
            (*parser).m_atts as *mut ::core::ffi::c_void,
            1466 as ::core::ffi::c_int,
        );
        expat_free(
            parser,
            parser as *mut ::core::ffi::c_void,
            1470 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    (*parser).m_dataBufEnd = (*parser).m_dataBuf.offset(INIT_DATA_BUF_SIZE as isize);
    if !dtd.is_null() {
        (*parser).m_dtd = dtd;
    } else {
        (*parser).m_dtd = dtdCreate(parser);
        if (*parser).m_dtd.is_null() {
            expat_free(
                parser,
                (*parser).m_dataBuf as *mut ::core::ffi::c_void,
                1480 as ::core::ffi::c_int,
            );
            expat_free(
                parser,
                (*parser).m_atts as *mut ::core::ffi::c_void,
                1481 as ::core::ffi::c_int,
            );
            expat_free(
                parser,
                parser as *mut ::core::ffi::c_void,
                1485 as ::core::ffi::c_int,
            );
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
    }
    (*parser).m_freeBindingList = ::core::ptr::null_mut::<BINDING>();
    (*parser).m_freeTagList = ::core::ptr::null_mut::<TAG>();
    (*parser).m_freeInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_freeAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_freeValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_groupSize = 0 as ::core::ffi::c_uint;
    (*parser).m_groupConnector = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*parser).m_unknownEncodingHandler = None;
    (*parser).m_unknownEncodingHandlerData = NULL;
    (*parser).m_namespaceSeparator = ASCII_EXCL as XML_Char;
    (*parser).m_ns = XML_FALSE;
    (*parser).m_ns_triplets = XML_FALSE;
    (*parser).m_nsAtts = ::core::ptr::null_mut::<NS_ATT>();
    (*parser).m_nsAttsVersion = 0 as ::core::ffi::c_ulong;
    (*parser).m_nsAttsPower = 0 as ::core::ffi::c_uchar;
    (*parser).m_protocolEncodingName = ::core::ptr::null::<XML_Char>();
    poolInit(&raw mut (*parser).m_tempPool, parser);
    poolInit(&raw mut (*parser).m_temp2Pool, parser);
    parserInit(parser, encodingName);
    if !encodingName.is_null() && (*parser).m_protocolEncodingName.is_null() {
        if !dtd.is_null() {
            (*parser).m_dtd = ::core::ptr::null_mut::<DTD>();
        }
        XML_ParserFree(parser);
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    if !nameSep.is_null() {
        (*parser).m_ns = XML_TRUE;
        (*parser).m_internalEncoding = XmlGetUtf8InternalEncodingNS();
        (*parser).m_namespaceSeparator = *nameSep;
    } else {
        (*parser).m_internalEncoding = XmlGetUtf8InternalEncoding();
    }
    return parser;
}
unsafe extern "C" fn parserInit(mut parser: XML_Parser, mut encodingName: *const XML_Char) {
    (*parser).m_processor = Some(
        prologInitProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> XML_Error,
    );
    XmlPrologStateInit(&raw mut (*parser).m_prologState);
    if !encodingName.is_null() {
        (*parser).m_protocolEncodingName = copyString(encodingName, parser);
    }
    (*parser).m_curBase = ::core::ptr::null::<XML_Char>();
    XmlInitEncoding(
        &raw mut (*parser).m_initEncoding,
        &raw mut (*parser).m_encoding,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    (*parser).m_userData = NULL;
    (*parser).m_handlerArg = NULL;
    (*parser).m_startElementHandler = None;
    (*parser).m_endElementHandler = None;
    (*parser).m_characterDataHandler = None;
    (*parser).m_processingInstructionHandler = None;
    (*parser).m_commentHandler = None;
    (*parser).m_startCdataSectionHandler = None;
    (*parser).m_endCdataSectionHandler = None;
    (*parser).m_defaultHandler = None;
    (*parser).m_startDoctypeDeclHandler = None;
    (*parser).m_endDoctypeDeclHandler = None;
    (*parser).m_unparsedEntityDeclHandler = None;
    (*parser).m_notationDeclHandler = None;
    (*parser).m_startNamespaceDeclHandler = None;
    (*parser).m_endNamespaceDeclHandler = None;
    (*parser).m_notStandaloneHandler = None;
    (*parser).m_externalEntityRefHandler = None;
    (*parser).m_externalEntityRefHandlerArg = parser;
    (*parser).m_skippedEntityHandler = None;
    (*parser).m_elementDeclHandler = None;
    (*parser).m_attlistDeclHandler = None;
    (*parser).m_entityDeclHandler = None;
    (*parser).m_xmlDeclHandler = None;
    (*parser).m_bufferPtr = (*parser).m_buffer;
    (*parser).m_bufferEnd = (*parser).m_buffer;
    (*parser).m_parseEndByteIndex = 0 as XML_Index;
    (*parser).m_parseEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_partialTokenBytesBefore = 0 as size_t;
    (*parser).m_reparseDeferralEnabled = g_reparseDeferralEnabledDefault;
    (*parser).m_lastBufferRequestSize = 0 as ::core::ffi::c_int;
    (*parser).m_declElementType = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    (*parser).m_declAttributeId = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
    (*parser).m_doctypeName = ::core::ptr::null::<XML_Char>();
    (*parser).m_doctypeSysid = ::core::ptr::null::<XML_Char>();
    (*parser).m_doctypePubid = ::core::ptr::null::<XML_Char>();
    (*parser).m_declAttributeType = ::core::ptr::null::<XML_Char>();
    (*parser).m_declNotationName = ::core::ptr::null::<XML_Char>();
    (*parser).m_declNotationPublicId = ::core::ptr::null::<XML_Char>();
    (*parser).m_declAttributeIsCdata = XML_FALSE;
    (*parser).m_declAttributeIsId = XML_FALSE;
    memset(
        &raw mut (*parser).m_position as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<POSITION>() as size_t,
    );
    (*parser).m_errorCode = XML_ERROR_NONE;
    (*parser).m_eventPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_eventEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_positionPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_openInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_openAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_openValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_defaultExpandInternalEntities = XML_TRUE;
    (*parser).m_tagLevel = 0 as ::core::ffi::c_int;
    (*parser).m_tagStack = ::core::ptr::null_mut::<TAG>();
    (*parser).m_inheritedBindings = ::core::ptr::null_mut::<BINDING>();
    (*parser).m_nSpecifiedAtts = 0 as ::core::ffi::c_int;
    (*parser).m_unknownEncodingMem = NULL;
    (*parser).m_unknownEncodingRelease = None;
    (*parser).m_unknownEncodingData = NULL;
    (*parser).m_parsingStatus.parsing = XML_INITIALIZED;
    (*parser).m_reenter = XML_FALSE;
    (*parser).m_isParamEntity = XML_FALSE;
    (*parser).m_useForeignDTD = XML_FALSE;
    (*parser).m_paramEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    (*parser).m_hash_secret_salt = 0 as ::core::ffi::c_ulong;
    memset(
        &raw mut (*parser).m_accounting as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ACCOUNTING>() as size_t,
    );
    (*parser).m_accounting.debugLevel = getDebugLevel(
        b"EXPAT_ACCOUNTING_DEBUG\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_ulong,
    );
    (*parser).m_accounting.maximumAmplificationFactor =
        EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
    (*parser).m_accounting.activationThresholdBytes =
        EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT
            as ::core::ffi::c_ulonglong;
    memset(
        &raw mut (*parser).m_entity_stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ENTITY_STATS>() as size_t,
    );
    (*parser).m_entity_stats.debugLevel = getDebugLevel(
        b"EXPAT_ENTITY_DEBUG\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_ulong,
    );
}
unsafe extern "C" fn moveToFreeBindingList(mut parser: XML_Parser, mut bindings: *mut BINDING) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        bindings = (*bindings).nextTagBinding as *mut BINDING;
        (*b).nextTagBinding = (*parser).m_freeBindingList as *mut binding;
        (*parser).m_freeBindingList = b;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_ParserReset(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Bool {
    let mut tStk: *mut TAG = ::core::ptr::null_mut::<TAG>();
    let mut openEntityList: *mut OPEN_INTERNAL_ENTITY =
        ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    if parser.is_null() {
        return XML_FALSE;
    }
    if !(*parser).m_parentParser.is_null() {
        return XML_FALSE;
    }
    tStk = (*parser).m_tagStack;
    while !tStk.is_null() {
        let mut tag: *mut TAG = tStk;
        tStk = (*tStk).parent as *mut TAG;
        (*tag).parent = (*parser).m_freeTagList as *mut tag;
        moveToFreeBindingList(parser, (*tag).bindings);
        (*tag).bindings = ::core::ptr::null_mut::<BINDING>();
        (*parser).m_freeTagList = tag;
    }
    openEntityList = (*parser).m_openInternalEntities;
    while !openEntityList.is_null() {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity).next as *mut OPEN_INTERNAL_ENTITY;
        (*openEntity).next = (*parser).m_freeInternalEntities as *mut open_internal_entity;
        (*parser).m_freeInternalEntities = openEntity;
    }
    openEntityList = (*parser).m_openAttributeEntities;
    while !openEntityList.is_null() {
        let mut openEntity_0: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity_0).next as *mut OPEN_INTERNAL_ENTITY;
        (*openEntity_0).next = (*parser).m_freeAttributeEntities as *mut open_internal_entity;
        (*parser).m_freeAttributeEntities = openEntity_0;
    }
    openEntityList = (*parser).m_openValueEntities;
    while !openEntityList.is_null() {
        let mut openEntity_1: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity_1).next as *mut OPEN_INTERNAL_ENTITY;
        (*openEntity_1).next = (*parser).m_freeValueEntities as *mut open_internal_entity;
        (*parser).m_freeValueEntities = openEntity_1;
    }
    moveToFreeBindingList(parser, (*parser).m_inheritedBindings);
    expat_free(
        parser,
        (*parser).m_unknownEncodingMem,
        1688 as ::core::ffi::c_int,
    );
    if (*parser).m_unknownEncodingRelease.is_some() {
        (*parser)
            .m_unknownEncodingRelease
            .expect("non-null function pointer")((*parser).m_unknownEncodingData);
    }
    poolClear(&raw mut (*parser).m_tempPool);
    poolClear(&raw mut (*parser).m_temp2Pool);
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut ::core::ffi::c_void,
        1693 as ::core::ffi::c_int,
    );
    (*parser).m_protocolEncodingName = ::core::ptr::null::<XML_Char>();
    parserInit(parser, encodingName);
    dtdReset((*parser).m_dtd, parser);
    return XML_TRUE;
}
unsafe extern "C" fn parserBusy(mut parser: XML_Parser) -> XML_Bool {
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        1 | 3 => return XML_TRUE,
        0 | 2 | _ => return XML_FALSE,
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEncoding(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if parserBusy(parser) != 0 {
        return XML_STATUS_ERROR;
    }
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut ::core::ffi::c_void,
        1725 as ::core::ffi::c_int,
    );
    if encodingName.is_null() {
        (*parser).m_protocolEncodingName = ::core::ptr::null::<XML_Char>();
    } else {
        (*parser).m_protocolEncodingName = copyString(encodingName, parser);
        if (*parser).m_protocolEncodingName.is_null() {
            return XML_STATUS_ERROR;
        }
    }
    return XML_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XML_ExternalEntityParserCreate(
    mut oldParser: XML_Parser,
    mut context: *const XML_Char,
    mut encodingName: *const XML_Char,
) -> XML_Parser {
    let mut parser: XML_Parser = oldParser;
    let mut newDtd: *mut DTD = ::core::ptr::null_mut::<DTD>();
    let mut oldDtd: *mut DTD = ::core::ptr::null_mut::<DTD>();
    let mut oldStartElementHandler: XML_StartElementHandler = None;
    let mut oldEndElementHandler: XML_EndElementHandler = None;
    let mut oldCharacterDataHandler: XML_CharacterDataHandler = None;
    let mut oldProcessingInstructionHandler: XML_ProcessingInstructionHandler = None;
    let mut oldCommentHandler: XML_CommentHandler = None;
    let mut oldStartCdataSectionHandler: XML_StartCdataSectionHandler = None;
    let mut oldEndCdataSectionHandler: XML_EndCdataSectionHandler = None;
    let mut oldDefaultHandler: XML_DefaultHandler = None;
    let mut oldUnparsedEntityDeclHandler: XML_UnparsedEntityDeclHandler = None;
    let mut oldNotationDeclHandler: XML_NotationDeclHandler = None;
    let mut oldStartNamespaceDeclHandler: XML_StartNamespaceDeclHandler = None;
    let mut oldEndNamespaceDeclHandler: XML_EndNamespaceDeclHandler = None;
    let mut oldNotStandaloneHandler: XML_NotStandaloneHandler = None;
    let mut oldExternalEntityRefHandler: XML_ExternalEntityRefHandler = None;
    let mut oldSkippedEntityHandler: XML_SkippedEntityHandler = None;
    let mut oldUnknownEncodingHandler: XML_UnknownEncodingHandler = None;
    let mut oldUnknownEncodingHandlerData: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut oldElementDeclHandler: XML_ElementDeclHandler = None;
    let mut oldAttlistDeclHandler: XML_AttlistDeclHandler = None;
    let mut oldEntityDeclHandler: XML_EntityDeclHandler = None;
    let mut oldXmlDeclHandler: XML_XmlDeclHandler = None;
    let mut oldDeclElementType: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    let mut oldUserData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut oldHandlerArg: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut oldDefaultExpandInternalEntities: XML_Bool = 0;
    let mut oldExternalEntityRefHandlerArg: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut oldParamEntityParsing: XML_ParamEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    let mut oldInEntityValue: ::core::ffi::c_int = 0;
    let mut oldns_triplets: XML_Bool = 0;
    let mut oldhash_secret_salt: ::core::ffi::c_ulong = 0;
    let mut oldReparseDeferralEnabled: XML_Bool = 0;
    if oldParser.is_null() {
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    oldDtd = (*parser).m_dtd;
    oldStartElementHandler = (*parser).m_startElementHandler;
    oldEndElementHandler = (*parser).m_endElementHandler;
    oldCharacterDataHandler = (*parser).m_characterDataHandler;
    oldProcessingInstructionHandler = (*parser).m_processingInstructionHandler;
    oldCommentHandler = (*parser).m_commentHandler;
    oldStartCdataSectionHandler = (*parser).m_startCdataSectionHandler;
    oldEndCdataSectionHandler = (*parser).m_endCdataSectionHandler;
    oldDefaultHandler = (*parser).m_defaultHandler;
    oldUnparsedEntityDeclHandler = (*parser).m_unparsedEntityDeclHandler;
    oldNotationDeclHandler = (*parser).m_notationDeclHandler;
    oldStartNamespaceDeclHandler = (*parser).m_startNamespaceDeclHandler;
    oldEndNamespaceDeclHandler = (*parser).m_endNamespaceDeclHandler;
    oldNotStandaloneHandler = (*parser).m_notStandaloneHandler;
    oldExternalEntityRefHandler = (*parser).m_externalEntityRefHandler;
    oldSkippedEntityHandler = (*parser).m_skippedEntityHandler;
    oldUnknownEncodingHandler = (*parser).m_unknownEncodingHandler;
    oldUnknownEncodingHandlerData = (*parser).m_unknownEncodingHandlerData;
    oldElementDeclHandler = (*parser).m_elementDeclHandler;
    oldAttlistDeclHandler = (*parser).m_attlistDeclHandler;
    oldEntityDeclHandler = (*parser).m_entityDeclHandler;
    oldXmlDeclHandler = (*parser).m_xmlDeclHandler;
    oldDeclElementType = (*parser).m_declElementType;
    oldUserData = (*parser).m_userData;
    oldHandlerArg = (*parser).m_handlerArg;
    oldDefaultExpandInternalEntities = (*parser).m_defaultExpandInternalEntities;
    oldExternalEntityRefHandlerArg = (*parser).m_externalEntityRefHandlerArg;
    oldParamEntityParsing = (*parser).m_paramEntityParsing;
    oldInEntityValue = (*parser).m_prologState.inEntityValue;
    oldns_triplets = (*parser).m_ns_triplets;
    oldhash_secret_salt = (*parser).m_hash_secret_salt;
    oldReparseDeferralEnabled = (*parser).m_reparseDeferralEnabled;
    if context.is_null() {
        newDtd = oldDtd;
    }
    if (*parser).m_ns != 0 {
        let mut tmp: [XML_Char; 2] = [
            (*parser).m_namespaceSeparator,
            0 as ::core::ffi::c_int as XML_Char,
        ];
        parser = parserCreate(
            encodingName,
            &raw const (*parser).m_mem,
            &raw mut tmp as *mut XML_Char,
            newDtd,
            oldParser,
        );
    } else {
        parser = parserCreate(
            encodingName,
            &raw const (*parser).m_mem,
            ::core::ptr::null::<XML_Char>(),
            newDtd,
            oldParser,
        );
    }
    if parser.is_null() {
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    (*parser).m_startElementHandler = oldStartElementHandler;
    (*parser).m_endElementHandler = oldEndElementHandler;
    (*parser).m_characterDataHandler = oldCharacterDataHandler;
    (*parser).m_processingInstructionHandler = oldProcessingInstructionHandler;
    (*parser).m_commentHandler = oldCommentHandler;
    (*parser).m_startCdataSectionHandler = oldStartCdataSectionHandler;
    (*parser).m_endCdataSectionHandler = oldEndCdataSectionHandler;
    (*parser).m_defaultHandler = oldDefaultHandler;
    (*parser).m_unparsedEntityDeclHandler = oldUnparsedEntityDeclHandler;
    (*parser).m_notationDeclHandler = oldNotationDeclHandler;
    (*parser).m_startNamespaceDeclHandler = oldStartNamespaceDeclHandler;
    (*parser).m_endNamespaceDeclHandler = oldEndNamespaceDeclHandler;
    (*parser).m_notStandaloneHandler = oldNotStandaloneHandler;
    (*parser).m_externalEntityRefHandler = oldExternalEntityRefHandler;
    (*parser).m_skippedEntityHandler = oldSkippedEntityHandler;
    (*parser).m_unknownEncodingHandler = oldUnknownEncodingHandler;
    (*parser).m_unknownEncodingHandlerData = oldUnknownEncodingHandlerData;
    (*parser).m_elementDeclHandler = oldElementDeclHandler;
    (*parser).m_attlistDeclHandler = oldAttlistDeclHandler;
    (*parser).m_entityDeclHandler = oldEntityDeclHandler;
    (*parser).m_xmlDeclHandler = oldXmlDeclHandler;
    (*parser).m_declElementType = oldDeclElementType;
    (*parser).m_userData = oldUserData;
    if oldUserData == oldHandlerArg {
        (*parser).m_handlerArg = (*parser).m_userData;
    } else {
        (*parser).m_handlerArg = parser as *mut ::core::ffi::c_void;
    }
    if oldExternalEntityRefHandlerArg != oldParser {
        (*parser).m_externalEntityRefHandlerArg = oldExternalEntityRefHandlerArg;
    }
    (*parser).m_defaultExpandInternalEntities = oldDefaultExpandInternalEntities;
    (*parser).m_ns_triplets = oldns_triplets;
    (*parser).m_hash_secret_salt = oldhash_secret_salt;
    (*parser).m_reparseDeferralEnabled = oldReparseDeferralEnabled;
    (*parser).m_parentParser = oldParser;
    (*parser).m_paramEntityParsing = oldParamEntityParsing;
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    if !context.is_null() {
        if dtdCopy(oldParser, (*parser).m_dtd, oldDtd, parser) == 0
            || setContext(parser, context) == 0
        {
            XML_ParserFree(parser);
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
        (*parser).m_processor = Some(
            externalEntityInitProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> XML_Error,
        );
    } else {
        (*parser).m_isParamEntity = XML_TRUE;
        XmlPrologStateInitExternalEntity(&raw mut (*parser).m_prologState);
        (*parser).m_processor = Some(
            externalParEntInitProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> XML_Error,
        );
    }
    return parser;
}
unsafe extern "C" fn destroyBindings(mut bindings: *mut BINDING, mut parser: XML_Parser) {
    loop {
        let mut b: *mut BINDING = bindings;
        if b.is_null() {
            break;
        }
        bindings = (*b).nextTagBinding as *mut BINDING;
        expat_free(
            parser,
            (*b).uri as *mut ::core::ffi::c_void,
            1921 as ::core::ffi::c_int,
        );
        expat_free(
            parser,
            b as *mut ::core::ffi::c_void,
            1922 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_ParserFree(mut parser: XML_Parser) {
    let mut tagList: *mut TAG = ::core::ptr::null_mut::<TAG>();
    let mut entityList: *mut OPEN_INTERNAL_ENTITY = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    if parser.is_null() {
        return;
    }
    tagList = (*parser).m_tagStack;
    loop {
        let mut p: *mut TAG = ::core::ptr::null_mut::<TAG>();
        if tagList.is_null() {
            if (*parser).m_freeTagList.is_null() {
                break;
            }
            tagList = (*parser).m_freeTagList;
            (*parser).m_freeTagList = ::core::ptr::null_mut::<TAG>();
        }
        p = tagList;
        tagList = (*tagList).parent as *mut TAG;
        expat_free(
            parser,
            (*p).buf.raw as *mut ::core::ffi::c_void,
            1944 as ::core::ffi::c_int,
        );
        destroyBindings((*p).bindings, parser);
        expat_free(
            parser,
            p as *mut ::core::ffi::c_void,
            1946 as ::core::ffi::c_int,
        );
    }
    entityList = (*parser).m_openInternalEntities;
    loop {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeInternalEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeInternalEntities;
            (*parser).m_freeInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser,
            openEntity as *mut ::core::ffi::c_void,
            1960 as ::core::ffi::c_int,
        );
    }
    entityList = (*parser).m_openAttributeEntities;
    loop {
        let mut openEntity_0: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeAttributeEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeAttributeEntities;
            (*parser).m_freeAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity_0 = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser,
            openEntity_0 as *mut ::core::ffi::c_void,
            1974 as ::core::ffi::c_int,
        );
    }
    entityList = (*parser).m_openValueEntities;
    loop {
        let mut openEntity_1: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeValueEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeValueEntities;
            (*parser).m_freeValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity_1 = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser,
            openEntity_1 as *mut ::core::ffi::c_void,
            1988 as ::core::ffi::c_int,
        );
    }
    destroyBindings((*parser).m_freeBindingList, parser);
    destroyBindings((*parser).m_inheritedBindings, parser);
    poolDestroy(&raw mut (*parser).m_tempPool);
    poolDestroy(&raw mut (*parser).m_temp2Pool);
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut ::core::ffi::c_void,
        1994 as ::core::ffi::c_int,
    );
    if (*parser).m_isParamEntity == 0 && !(*parser).m_dtd.is_null() {
        dtdDestroy(
            (*parser).m_dtd,
            (*parser).m_parentParser.is_null() as ::core::ffi::c_int as XML_Bool,
            parser,
        );
    }
    expat_free(
        parser,
        (*parser).m_atts as *mut ::core::ffi::c_void,
        2004 as ::core::ffi::c_int,
    );
    expat_free(
        parser,
        (*parser).m_groupConnector as *mut ::core::ffi::c_void,
        2008 as ::core::ffi::c_int,
    );
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_buffer as *mut ::core::ffi::c_void,
    );
    expat_free(
        parser,
        (*parser).m_dataBuf as *mut ::core::ffi::c_void,
        2013 as ::core::ffi::c_int,
    );
    expat_free(
        parser,
        (*parser).m_nsAtts as *mut ::core::ffi::c_void,
        2014 as ::core::ffi::c_int,
    );
    expat_free(
        parser,
        (*parser).m_unknownEncodingMem,
        2015 as ::core::ffi::c_int,
    );
    if (*parser).m_unknownEncodingRelease.is_some() {
        (*parser)
            .m_unknownEncodingRelease
            .expect("non-null function pointer")((*parser).m_unknownEncodingData);
    }
    expat_free(
        parser,
        parser as *mut ::core::ffi::c_void,
        2018 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn XML_UseParserAsHandlerArg(mut parser: XML_Parser) {
    if !parser.is_null() {
        (*parser).m_handlerArg = parser as *mut ::core::ffi::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_UseForeignDTD(
    mut parser: XML_Parser,
    mut useDTD: XML_Bool,
) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    if parserBusy(parser) != 0 {
        return XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
    }
    (*parser).m_useForeignDTD = useDTD;
    return XML_ERROR_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetReturnNSTriplet(
    mut parser: XML_Parser,
    mut do_nst: ::core::ffi::c_int,
) {
    if parser.is_null() {
        return;
    }
    if parserBusy(parser) != 0 {
        return;
    }
    (*parser).m_ns_triplets = (if do_nst != 0 {
        XML_TRUE as ::core::ffi::c_int
    } else {
        XML_FALSE as ::core::ffi::c_int
    }) as XML_Bool;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetUserData(mut parser: XML_Parser, mut p: *mut ::core::ffi::c_void) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_handlerArg == (*parser).m_userData {
        (*parser).m_userData = p;
        (*parser).m_handlerArg = (*parser).m_userData;
    } else {
        (*parser).m_userData = p;
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetBase(mut parser: XML_Parser, mut p: *const XML_Char) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if !p.is_null() {
        p = poolCopyString(&raw mut (*(*parser).m_dtd).pool, p);
        if p.is_null() {
            return XML_STATUS_ERROR;
        }
        (*parser).m_curBase = p;
    } else {
        (*parser).m_curBase = ::core::ptr::null::<XML_Char>();
    }
    return XML_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetBase(mut parser: XML_Parser) -> *const XML_Char {
    if parser.is_null() {
        return ::core::ptr::null::<XML_Char>();
    }
    return (*parser).m_curBase;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount(
    mut parser: XML_Parser,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return (*parser).m_nSpecifiedAtts;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetIdAttributeIndex(mut parser: XML_Parser) -> ::core::ffi::c_int {
    if parser.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return (*parser).m_idAttIndex;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetElementHandler(
    mut parser: XML_Parser,
    mut start: XML_StartElementHandler,
    mut end: XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startElementHandler = start;
    (*parser).m_endElementHandler = end;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetStartElementHandler(
    mut parser: XML_Parser,
    mut start: XML_StartElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_startElementHandler = start;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEndElementHandler(
    mut parser: XML_Parser,
    mut end: XML_EndElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_endElementHandler = end;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    mut parser: XML_Parser,
    mut handler: XML_CharacterDataHandler,
) {
    if !parser.is_null() {
        (*parser).m_characterDataHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    mut parser: XML_Parser,
    mut handler: XML_ProcessingInstructionHandler,
) {
    if !parser.is_null() {
        (*parser).m_processingInstructionHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetCommentHandler(
    mut parser: XML_Parser,
    mut handler: XML_CommentHandler,
) {
    if !parser.is_null() {
        (*parser).m_commentHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetCdataSectionHandler(
    mut parser: XML_Parser,
    mut start: XML_StartCdataSectionHandler,
    mut end: XML_EndCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startCdataSectionHandler = start;
    (*parser).m_endCdataSectionHandler = end;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetStartCdataSectionHandler(
    mut parser: XML_Parser,
    mut start: XML_StartCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_startCdataSectionHandler = start;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEndCdataSectionHandler(
    mut parser: XML_Parser,
    mut end: XML_EndCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_endCdataSectionHandler = end;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandler(
    mut parser: XML_Parser,
    mut handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_defaultHandler = handler;
    (*parser).m_defaultExpandInternalEntities = XML_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetDefaultHandlerExpand(
    mut parser: XML_Parser,
    mut handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_defaultHandler = handler;
    (*parser).m_defaultExpandInternalEntities = XML_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartDoctypeDeclHandler,
    mut end: XML_EndDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startDoctypeDeclHandler = start;
    (*parser).m_endDoctypeDeclHandler = end;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_startDoctypeDeclHandler = start;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut end: XML_EndDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_endDoctypeDeclHandler = end;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_UnparsedEntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_unparsedEntityDeclHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetNotationDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_NotationDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_notationDeclHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartNamespaceDeclHandler,
    mut end: XML_EndNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startNamespaceDeclHandler = start;
    (*parser).m_endNamespaceDeclHandler = end;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_startNamespaceDeclHandler = start;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut end: XML_EndNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_endNamespaceDeclHandler = end;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetNotStandaloneHandler(
    mut parser: XML_Parser,
    mut handler: XML_NotStandaloneHandler,
) {
    if !parser.is_null() {
        (*parser).m_notStandaloneHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandler(
    mut parser: XML_Parser,
    mut handler: XML_ExternalEntityRefHandler,
) {
    if !parser.is_null() {
        (*parser).m_externalEntityRefHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg(
    mut parser: XML_Parser,
    mut arg: *mut ::core::ffi::c_void,
) {
    if parser.is_null() {
        return;
    }
    if !arg.is_null() {
        (*parser).m_externalEntityRefHandlerArg = arg as XML_Parser;
    } else {
        (*parser).m_externalEntityRefHandlerArg = parser;
    };
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetSkippedEntityHandler(
    mut parser: XML_Parser,
    mut handler: XML_SkippedEntityHandler,
) {
    if !parser.is_null() {
        (*parser).m_skippedEntityHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetUnknownEncodingHandler(
    mut parser: XML_Parser,
    mut handler: XML_UnknownEncodingHandler,
    mut data: *mut ::core::ffi::c_void,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_unknownEncodingHandler = handler;
    (*parser).m_unknownEncodingHandlerData = data;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetElementDeclHandler(
    mut parser: XML_Parser,
    mut eldecl: XML_ElementDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_elementDeclHandler = eldecl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetAttlistDeclHandler(
    mut parser: XML_Parser,
    mut attdecl: XML_AttlistDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_attlistDeclHandler = attdecl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetEntityDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_EntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_entityDeclHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetXmlDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_XmlDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_xmlDeclHandler = handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetParamEntityParsing(
    mut parser: XML_Parser,
    mut peParsing: XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if parserBusy(parser) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).m_paramEntityParsing = peParsing;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetHashSalt(
    mut parser: XML_Parser,
    mut hash_salt: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let rootParser: XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"XML_SetHashSalt\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            2333 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if parserBusy(rootParser) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*rootParser).m_hash_secret_salt = hash_salt;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn XML_Parse(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut isFinal: ::core::ffi::c_int,
) -> XML_Status {
    if parser.is_null()
        || len < 0 as ::core::ffi::c_int
        || s.is_null() && len != 0 as ::core::ffi::c_int
    {
        if !parser.is_null() {
            (*parser).m_errorCode = XML_ERROR_INVALID_ARGUMENT;
        }
        return XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        3 => {
            (*parser).m_errorCode = XML_ERROR_SUSPENDED;
            return XML_STATUS_ERROR;
        }
        2 => {
            (*parser).m_errorCode = XML_ERROR_FINISHED;
            return XML_STATUS_ERROR;
        }
        0 => {
            if (*parser).m_parentParser.is_null() && startParsing(parser) == 0 {
                (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
                return XML_STATUS_ERROR;
            }
        }
        _ => {}
    }
    (*parser).m_parsingStatus.parsing = XML_PARSING;
    let mut buff: *mut ::core::ffi::c_void = XML_GetBuffer(parser, len);
    if buff.is_null() {
        return XML_STATUS_ERROR;
    }
    if len > 0 as ::core::ffi::c_int {
        if s.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"XML_Parse\0" as *const u8 as *const ::core::ffi::c_char,
                b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                2447 as ::core::ffi::c_int,
                b"s != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        memcpy(buff, s as *const ::core::ffi::c_void, len as size_t);
    }
    return XML_ParseBuffer(parser, len, isFinal);
}
#[no_mangle]
pub unsafe extern "C" fn XML_ParseBuffer(
    mut parser: XML_Parser,
    mut len: ::core::ffi::c_int,
    mut isFinal: ::core::ffi::c_int,
) -> XML_Status {
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut result: XML_Status = XML_STATUS_OK;
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if len < 0 as ::core::ffi::c_int {
        (*parser).m_errorCode = XML_ERROR_INVALID_ARGUMENT;
        return XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        3 => {
            (*parser).m_errorCode = XML_ERROR_SUSPENDED;
            return XML_STATUS_ERROR;
        }
        2 => {
            (*parser).m_errorCode = XML_ERROR_FINISHED;
            return XML_STATUS_ERROR;
        }
        0 => {
            if (*parser).m_bufferPtr.is_null() {
                (*parser).m_errorCode = XML_ERROR_NO_BUFFER;
                return XML_STATUS_ERROR;
            }
            if (*parser).m_parentParser.is_null() && startParsing(parser) == 0 {
                (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
                return XML_STATUS_ERROR;
            }
        }
        _ => {}
    }
    (*parser).m_parsingStatus.parsing = XML_PARSING;
    start = (*parser).m_bufferPtr;
    (*parser).m_positionPtr = start;
    (*parser).m_bufferEnd = (*parser).m_bufferEnd.offset(len as isize);
    (*parser).m_parseEndPtr = (*parser).m_bufferEnd;
    (*parser).m_parseEndByteIndex += len as XML_Index;
    (*parser).m_parsingStatus.finalBuffer = isFinal as XML_Bool;
    (*parser).m_errorCode = callProcessor(
        parser,
        start,
        (*parser).m_parseEndPtr,
        &raw mut (*parser).m_bufferPtr,
    );
    if (*parser).m_errorCode as ::core::ffi::c_uint
        != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(
            errorProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> XML_Error,
        );
        return XML_STATUS_ERROR;
    } else {
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                result = XML_STATUS_SUSPENDED;
            }
            0 | 1 => {
                if isFinal != 0 {
                    (*parser).m_parsingStatus.parsing = XML_FINISHED;
                    return result;
                }
            }
            _ => {}
        }
    }
    (*(*parser).m_encoding)
        .updatePosition
        .expect("non-null function pointer")(
        (*parser).m_encoding,
        (*parser).m_positionPtr,
        (*parser).m_bufferPtr,
        &raw mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetBuffer(
    mut parser: XML_Parser,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    if parser.is_null() {
        return NULL;
    }
    if len < 0 as ::core::ffi::c_int {
        (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
        return NULL;
    }
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        3 => {
            (*parser).m_errorCode = XML_ERROR_SUSPENDED;
            return NULL;
        }
        2 => {
            (*parser).m_errorCode = XML_ERROR_FINISHED;
            return NULL;
        }
        _ => {}
    }
    (*parser).m_lastBufferRequestSize = len;
    if len as ::core::ffi::c_long
        > (if !(*parser).m_bufferLim.is_null() && !(*parser).m_bufferEnd.is_null() {
            (*parser).m_bufferLim.offset_from((*parser).m_bufferEnd) as ::core::ffi::c_long
        } else {
            0 as ::core::ffi::c_long
        })
        || (*parser).m_buffer.is_null()
    {
        let mut keep: ::core::ffi::c_int = 0;
        let mut neededSize: ::core::ffi::c_int = (len as ::core::ffi::c_uint).wrapping_add(
            (if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr) as ::core::ffi::c_long
            } else {
                0 as ::core::ffi::c_long
            }) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
        if neededSize < 0 as ::core::ffi::c_int {
            (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
            return NULL;
        }
        keep = (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
            (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
        } else {
            0 as ::core::ffi::c_long
        }) as ::core::ffi::c_int;
        if keep > XML_CONTEXT_BYTES {
            keep = XML_CONTEXT_BYTES;
        }
        if keep > INT_MAX - neededSize {
            (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
            return NULL;
        }
        neededSize += keep;
        if !(*parser).m_buffer.is_null()
            && !(*parser).m_bufferPtr.is_null()
            && neededSize as ::core::ffi::c_long
                <= (if !(*parser).m_bufferLim.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferLim.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                } else {
                    0 as ::core::ffi::c_long
                })
        {
            if (keep as ::core::ffi::c_long)
                < (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                } else {
                    0 as ::core::ffi::c_long
                })
            {
                let mut offset: ::core::ffi::c_int =
                    (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                        (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                    } else {
                        0 as ::core::ffi::c_long
                    }) as ::core::ffi::c_int
                        - keep;
                memmove(
                    (*parser).m_buffer as *mut ::core::ffi::c_void,
                    (*parser).m_buffer.offset(offset as isize) as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    ((*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr)
                        as ::core::ffi::c_long
                        + keep as ::core::ffi::c_long) as size_t,
                );
                (*parser).m_bufferEnd = (*parser).m_bufferEnd.offset(-(offset as isize));
                (*parser).m_bufferPtr = (*parser).m_bufferPtr.offset(-(offset as isize));
            }
        } else {
            let mut newBuf: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut bufferSize: ::core::ffi::c_int =
                (if !(*parser).m_bufferLim.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferLim.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                } else {
                    0 as ::core::ffi::c_long
                }) as ::core::ffi::c_int;
            if bufferSize == 0 as ::core::ffi::c_int {
                bufferSize = INIT_BUFFER_SIZE;
            }
            loop {
                bufferSize = (2 as ::core::ffi::c_uint)
                    .wrapping_mul(bufferSize as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
                if !(bufferSize < neededSize && bufferSize > 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            if bufferSize <= 0 as ::core::ffi::c_int {
                (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
                return NULL;
            }
            newBuf = (*parser)
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer")(bufferSize as size_t)
                as *mut ::core::ffi::c_char;
            if newBuf.is_null() {
                (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
                return NULL;
            }
            (*parser).m_bufferLim = newBuf.offset(bufferSize as isize);
            if !(*parser).m_bufferPtr.is_null() {
                memcpy(
                    newBuf as *mut ::core::ffi::c_void,
                    (*parser).m_bufferPtr.offset(-keep as isize) as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    ((if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                        (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr)
                            as ::core::ffi::c_long
                    } else {
                        0 as ::core::ffi::c_long
                    }) + keep as ::core::ffi::c_long) as size_t,
                );
                (*parser).m_mem.free_fcn.expect("non-null function pointer")(
                    (*parser).m_buffer as *mut ::core::ffi::c_void,
                );
                (*parser).m_buffer = newBuf;
                (*parser).m_bufferEnd = (*parser)
                    .m_buffer
                    .offset(
                        (if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                            (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr)
                                as ::core::ffi::c_long
                        } else {
                            0 as ::core::ffi::c_long
                        }) as isize,
                    )
                    .offset(keep as isize);
                (*parser).m_bufferPtr = (*parser).m_buffer.offset(keep as isize);
            } else {
                (*parser).m_bufferEnd = newBuf;
                (*parser).m_buffer = newBuf;
                (*parser).m_bufferPtr = (*parser).m_buffer;
            }
        }
        (*parser).m_eventEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
        (*parser).m_eventPtr = (*parser).m_eventEndPtr;
        (*parser).m_positionPtr = ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*parser).m_bufferEnd as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn triggerReenter(mut parser: XML_Parser) {
    (*parser).m_reenter = XML_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_StopParser(
    mut parser: XML_Parser,
    mut resumable: XML_Bool,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        0 => {
            (*parser).m_errorCode = XML_ERROR_NOT_STARTED;
            return XML_STATUS_ERROR;
        }
        3 => {
            if resumable != 0 {
                (*parser).m_errorCode = XML_ERROR_SUSPENDED;
                return XML_STATUS_ERROR;
            }
            (*parser).m_parsingStatus.parsing = XML_FINISHED;
        }
        2 => {
            (*parser).m_errorCode = XML_ERROR_FINISHED;
            return XML_STATUS_ERROR;
        }
        1 => {
            if resumable != 0 {
                if (*parser).m_isParamEntity != 0 {
                    (*parser).m_errorCode = XML_ERROR_SUSPEND_PE;
                    return XML_STATUS_ERROR;
                }
                (*parser).m_parsingStatus.parsing = XML_SUSPENDED;
            } else {
                (*parser).m_parsingStatus.parsing = XML_FINISHED;
            }
        }
        _ => {
            if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"XML_StopParser\0" as *const u8 as *const ::core::ffi::c_char,
                    b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2694 as ::core::ffi::c_int,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
    }
    return XML_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn XML_ResumeParser(mut parser: XML_Parser) -> XML_Status {
    let mut result: XML_Status = XML_STATUS_OK;
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
        != XML_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*parser).m_errorCode = XML_ERROR_NOT_SUSPENDED;
        return XML_STATUS_ERROR;
    }
    (*parser).m_parsingStatus.parsing = XML_PARSING;
    (*parser).m_errorCode = callProcessor(
        parser,
        (*parser).m_bufferPtr,
        (*parser).m_parseEndPtr,
        &raw mut (*parser).m_bufferPtr,
    );
    if (*parser).m_errorCode as ::core::ffi::c_uint
        != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(
            errorProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> XML_Error,
        );
        return XML_STATUS_ERROR;
    } else {
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                result = XML_STATUS_SUSPENDED;
            }
            0 | 1 => {
                if (*parser).m_parsingStatus.finalBuffer != 0 {
                    (*parser).m_parsingStatus.parsing = XML_FINISHED;
                    return result;
                }
            }
            _ => {}
        }
    }
    (*(*parser).m_encoding)
        .updatePosition
        .expect("non-null function pointer")(
        (*parser).m_encoding,
        (*parser).m_positionPtr,
        (*parser).m_bufferPtr,
        &raw mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetParsingStatus(
    mut parser: XML_Parser,
    mut status: *mut XML_ParsingStatus,
) {
    if parser.is_null() {
        return;
    }
    if status.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"XML_GetParsingStatus\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            2743 as ::core::ffi::c_int,
            b"status != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    *status = (*parser).m_parsingStatus;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetErrorCode(mut parser: XML_Parser) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    return (*parser).m_errorCode;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteIndex(mut parser: XML_Parser) -> XML_Index {
    if parser.is_null() {
        return -(1 as ::core::ffi::c_int) as XML_Index;
    }
    if !(*parser).m_eventPtr.is_null() {
        return (*parser).m_parseEndByteIndex as ::core::ffi::c_long
            - (*parser).m_parseEndPtr.offset_from((*parser).m_eventPtr) as ::core::ffi::c_long;
    }
    return -(1 as ::core::ffi::c_int) as XML_Index;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentByteCount(mut parser: XML_Parser) -> ::core::ffi::c_int {
    if parser.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*parser).m_eventEndPtr.is_null() && !(*parser).m_eventPtr.is_null() {
        return (*parser).m_eventEndPtr.offset_from((*parser).m_eventPtr) as ::core::ffi::c_long
            as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetInputContext(
    mut parser: XML_Parser,
    mut offset: *mut ::core::ffi::c_int,
    mut size: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if parser.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !(*parser).m_eventPtr.is_null() && !(*parser).m_buffer.is_null() {
        if !offset.is_null() {
            *offset = (*parser).m_eventPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                as ::core::ffi::c_int;
        }
        if !size.is_null() {
            *size = (*parser).m_bufferEnd.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                as ::core::ffi::c_int;
        }
        return (*parser).m_buffer;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentLineNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0 as XML_Size;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding)
            .updatePosition
            .expect("non-null function pointer")(
            (*parser).m_encoding,
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &raw mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr;
    }
    return (*parser).m_position.lineNumber.wrapping_add(1 as XML_Size);
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0 as XML_Size;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding)
            .updatePosition
            .expect("non-null function pointer")(
            (*parser).m_encoding,
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &raw mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr;
    }
    return (*parser).m_position.columnNumber;
}
#[no_mangle]
pub unsafe extern "C" fn XML_FreeContentModel(mut parser: XML_Parser, mut model: *mut XML_Content) {
    if parser.is_null() {
        return;
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(model as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn XML_MemMalloc(
    mut parser: XML_Parser,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if parser.is_null() {
        return NULL;
    }
    return (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(size);
}
#[no_mangle]
pub unsafe extern "C" fn XML_MemRealloc(
    mut parser: XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if parser.is_null() {
        return NULL;
    }
    return (*parser)
        .m_mem
        .realloc_fcn
        .expect("non-null function pointer")(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn XML_MemFree(mut parser: XML_Parser, mut ptr: *mut ::core::ffi::c_void) {
    if parser.is_null() {
        return;
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn XML_DefaultCurrent(mut parser: XML_Parser) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_defaultHandler.is_some() {
        if !(*parser).m_openInternalEntities.is_null() {
            reportDefault(
                parser,
                (*parser).m_internalEncoding,
                (*(*parser).m_openInternalEntities).internalEventPtr,
                (*(*parser).m_openInternalEntities).internalEventEndPtr,
            );
        } else {
            reportDefault(
                parser,
                (*parser).m_encoding,
                (*parser).m_eventPtr,
                (*parser).m_eventEndPtr,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn XML_ErrorString(mut code: XML_Error) -> *const XML_LChar {
    match code as ::core::ffi::c_uint {
        0 => return ::core::ptr::null::<XML_LChar>(),
        1 => return b"out of memory\0" as *const u8 as *const XML_LChar,
        2 => return b"syntax error\0" as *const u8 as *const XML_LChar,
        3 => return b"no element found\0" as *const u8 as *const XML_LChar,
        4 => return b"not well-formed (invalid token)\0" as *const u8 as *const XML_LChar,
        5 => return b"unclosed token\0" as *const u8 as *const XML_LChar,
        6 => return b"partial character\0" as *const u8 as *const XML_LChar,
        7 => return b"mismatched tag\0" as *const u8 as *const XML_LChar,
        8 => return b"duplicate attribute\0" as *const u8 as *const XML_LChar,
        9 => return b"junk after document element\0" as *const u8 as *const XML_LChar,
        10 => {
            return b"illegal parameter entity reference\0" as *const u8 as *const XML_LChar;
        }
        11 => return b"undefined entity\0" as *const u8 as *const XML_LChar,
        12 => return b"recursive entity reference\0" as *const u8 as *const XML_LChar,
        13 => return b"asynchronous entity\0" as *const u8 as *const XML_LChar,
        14 => {
            return b"reference to invalid character number\0" as *const u8 as *const XML_LChar;
        }
        15 => return b"reference to binary entity\0" as *const u8 as *const XML_LChar,
        16 => {
            return b"reference to external entity in attribute\0" as *const u8 as *const XML_LChar;
        }
        17 => {
            return b"XML or text declaration not at start of entity\0" as *const u8
                as *const XML_LChar;
        }
        18 => return b"unknown encoding\0" as *const u8 as *const XML_LChar,
        19 => {
            return b"encoding specified in XML declaration is incorrect\0" as *const u8
                as *const XML_LChar;
        }
        20 => return b"unclosed CDATA section\0" as *const u8 as *const XML_LChar,
        21 => {
            return b"error in processing external entity reference\0" as *const u8
                as *const XML_LChar;
        }
        22 => return b"document is not standalone\0" as *const u8 as *const XML_LChar,
        23 => {
            return b"unexpected parser state - please send a bug report\0" as *const u8
                as *const XML_LChar;
        }
        24 => {
            return b"entity declared in parameter entity\0" as *const u8 as *const XML_LChar;
        }
        25 => {
            return b"requested feature requires XML_DTD support in Expat\0" as *const u8
                as *const XML_LChar;
        }
        26 => {
            return b"cannot change setting once parsing has begun\0" as *const u8
                as *const XML_LChar;
        }
        27 => return b"unbound prefix\0" as *const u8 as *const XML_LChar,
        28 => return b"must not undeclare prefix\0" as *const u8 as *const XML_LChar,
        29 => {
            return b"incomplete markup in parameter entity\0" as *const u8 as *const XML_LChar;
        }
        30 => {
            return b"XML declaration not well-formed\0" as *const u8 as *const XML_LChar;
        }
        31 => {
            return b"text declaration not well-formed\0" as *const u8 as *const XML_LChar;
        }
        32 => {
            return b"illegal character(s) in public id\0" as *const u8 as *const XML_LChar;
        }
        33 => return b"parser suspended\0" as *const u8 as *const XML_LChar,
        34 => return b"parser not suspended\0" as *const u8 as *const XML_LChar,
        35 => return b"parsing aborted\0" as *const u8 as *const XML_LChar,
        36 => return b"parsing finished\0" as *const u8 as *const XML_LChar,
        37 => {
            return b"cannot suspend in external parameter entity\0" as *const u8
                as *const XML_LChar;
        }
        38 => {
            return b"reserved prefix (xml) must not be undeclared or bound to another namespace name\0"
                as *const u8 as *const XML_LChar;
        }
        39 => {
            return b"reserved prefix (xmlns) must not be declared or undeclared\0" as *const u8
                as *const XML_LChar;
        }
        40 => {
            return b"prefix must not be bound to one of the reserved namespace names\0" as *const u8
                as *const XML_LChar;
        }
        41 => return b"invalid argument\0" as *const u8 as *const XML_LChar,
        42 => {
            return b"a successful prior call to function XML_GetBuffer is required\0" as *const u8
                as *const XML_LChar;
        }
        43 => {
            return b"limit on input amplification factor (from DTD and entities) breached\0"
                as *const u8 as *const XML_LChar;
        }
        44 => return b"parser not started\0" as *const u8 as *const XML_LChar,
        _ => {}
    }
    return ::core::ptr::null::<XML_LChar>();
}
#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersion() -> *const XML_LChar {
    return b"expat_2.7.5\0" as *const u8 as *const XML_LChar;
}
#[no_mangle]
pub unsafe extern "C" fn XML_ExpatVersionInfo() -> XML_Expat_Version {
    let mut version: XML_Expat_Version = XML_Expat_Version {
        major: 0,
        minor: 0,
        micro: 0,
    };
    version.major = XML_MAJOR_VERSION;
    version.minor = XML_MINOR_VERSION;
    version.micro = XML_MICRO_VERSION;
    return version;
}
#[no_mangle]
pub unsafe extern "C" fn XML_GetFeatureList() -> *const XML_Feature {
    static mut features: [XML_Feature; 11] = [
        XML_Feature {
            feature: XML_FEATURE_SIZEOF_XML_CHAR,
            name: b"sizeof(XML_Char)\0" as *const u8 as *const XML_LChar,
            value: ::core::mem::size_of::<XML_Char>() as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_SIZEOF_XML_LCHAR,
            name: b"sizeof(XML_LChar)\0" as *const u8 as *const XML_LChar,
            value: ::core::mem::size_of::<XML_LChar>() as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_DTD,
            name: b"XML_DTD\0" as *const u8 as *const XML_LChar,
            value: 0 as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_CONTEXT_BYTES,
            name: b"XML_CONTEXT_BYTES\0" as *const u8 as *const XML_LChar,
            value: XML_CONTEXT_BYTES as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_NS,
            name: b"XML_NS\0" as *const u8 as *const XML_LChar,
            value: 0 as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT,
            name: b"XML_BLAP_MAX_AMP\0" as *const u8 as *const XML_LChar,
            value: EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT
                as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT,
            name: b"XML_BLAP_ACT_THRES\0" as *const u8 as *const XML_LChar,
            value: EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT
                as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_GE,
            name: b"XML_GE\0" as *const u8 as *const XML_LChar,
            value: 0 as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT,
            name: b"XML_AT_MAX_AMP\0" as *const u8 as *const XML_LChar,
            value: EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT,
            name: b"XML_AT_ACT_THRES\0" as *const u8 as *const XML_LChar,
            value: EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT as ::core::ffi::c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_END,
            name: ::core::ptr::null::<XML_LChar>(),
            value: 0 as ::core::ffi::c_long,
        },
    ];
    return &raw const features as *const XML_Feature;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
    mut parser: XML_Parser,
    mut maximumAmplificationFactor: ::core::ffi::c_float,
) -> XML_Bool {
    if parser.is_null()
        || !(*parser).m_parentParser.is_null()
        || (if ::core::mem::size_of::<::core::ffi::c_float>() as usize
            == ::core::mem::size_of::<::core::ffi::c_float>() as usize
        {
            __inline_isnanf(maximumAmplificationFactor)
        } else {
            (if ::core::mem::size_of::<::core::ffi::c_float>() as usize
                == ::core::mem::size_of::<::core::ffi::c_double>() as usize
            {
                __inline_isnand(maximumAmplificationFactor as ::core::ffi::c_double)
            } else {
                __inline_isnanl(maximumAmplificationFactor as f64)
            })
        }) != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return XML_FALSE;
    }
    (*parser).m_accounting.maximumAmplificationFactor = maximumAmplificationFactor;
    return XML_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
    mut parser: XML_Parser,
    mut activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> XML_Bool {
    if parser.is_null() || !(*parser).m_parentParser.is_null() {
        return XML_FALSE;
    }
    (*parser).m_accounting.activationThresholdBytes = activationThresholdBytes;
    return XML_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetAllocTrackerMaximumAmplification(
    mut parser: XML_Parser,
    mut maximumAmplificationFactor: ::core::ffi::c_float,
) -> XML_Bool {
    if parser.is_null()
        || !(*parser).m_parentParser.is_null()
        || (if ::core::mem::size_of::<::core::ffi::c_float>() as usize
            == ::core::mem::size_of::<::core::ffi::c_float>() as usize
        {
            __inline_isnanf(maximumAmplificationFactor)
        } else {
            (if ::core::mem::size_of::<::core::ffi::c_float>() as usize
                == ::core::mem::size_of::<::core::ffi::c_double>() as usize
            {
                __inline_isnand(maximumAmplificationFactor as ::core::ffi::c_double)
            } else {
                __inline_isnanl(maximumAmplificationFactor as f64)
            })
        }) != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return XML_FALSE;
    }
    (*parser).m_alloc_tracker.maximumAmplificationFactor = maximumAmplificationFactor;
    return XML_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetAllocTrackerActivationThreshold(
    mut parser: XML_Parser,
    mut activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> XML_Bool {
    if parser.is_null() || !(*parser).m_parentParser.is_null() {
        return XML_FALSE;
    }
    (*parser).m_alloc_tracker.activationThresholdBytes = activationThresholdBytes as XmlBigCount;
    return XML_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn XML_SetReparseDeferralEnabled(
    mut parser: XML_Parser,
    mut enabled: XML_Bool,
) -> XML_Bool {
    if !parser.is_null()
        && (enabled as ::core::ffi::c_int == XML_TRUE as ::core::ffi::c_int
            || enabled as ::core::ffi::c_int == XML_FALSE as ::core::ffi::c_int)
    {
        (*parser).m_reparseDeferralEnabled = enabled;
        return XML_TRUE;
    }
    return XML_FALSE;
}
unsafe extern "C" fn storeRawNames(mut parser: XML_Parser) -> XML_Bool {
    let mut tag: *mut TAG = (*parser).m_tagStack;
    while !tag.is_null() {
        let mut bufSize: size_t = 0;
        let mut nameLen: size_t = (::core::mem::size_of::<XML_Char>() as size_t)
            .wrapping_mul(((*tag).name.strLen + 1 as ::core::ffi::c_int) as size_t);
        let mut rawNameLen: size_t = 0;
        let mut rawNameBuf: *mut ::core::ffi::c_char = (*tag).buf.raw.offset(nameLen as isize);
        if (*tag).rawName == rawNameBuf as *const ::core::ffi::c_char {
            break;
        }
        rawNameLen = (((*tag).rawNameLength as usize)
            .wrapping_add((::core::mem::size_of::<XML_Char>() as usize).wrapping_sub(1 as usize))
            & !(::core::mem::size_of::<XML_Char>() as usize).wrapping_sub(1 as usize))
            as size_t;
        if rawNameLen > (INT_MAX as size_t).wrapping_sub(nameLen) {
            return XML_FALSE;
        }
        bufSize = nameLen.wrapping_add(rawNameLen);
        if bufSize > (*tag).bufEnd.offset_from((*tag).buf.raw) as ::core::ffi::c_long as size_t {
            let mut temp: *mut ::core::ffi::c_char = expat_realloc(
                parser,
                (*tag).buf.raw as *mut ::core::ffi::c_void,
                bufSize,
                3153 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char;
            if temp.is_null() {
                return XML_FALSE;
            }
            if (*tag).name.str_0 == (*tag).buf.str_0 as *const XML_Char {
                (*tag).name.str_0 = temp as *mut XML_Char;
            }
            if !(*tag).name.localPart.is_null() {
                (*tag).name.localPart = (temp as *mut XML_Char)
                    .offset((*tag).name.localPart.offset_from((*tag).buf.str_0)
                        as ::core::ffi::c_long as isize);
            }
            (*tag).buf.raw = temp;
            (*tag).bufEnd = temp.offset(bufSize as isize);
            rawNameBuf = temp.offset(nameLen as isize);
        }
        memcpy(
            rawNameBuf as *mut ::core::ffi::c_void,
            (*tag).rawName as *const ::core::ffi::c_void,
            (*tag).rawNameLength as size_t,
        );
        (*tag).rawName = rawNameBuf;
        tag = (*tag).parent as *mut TAG;
    }
    return XML_TRUE;
}
unsafe extern "C" fn contentProcessor(
    mut parser: XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut result: XML_Error = doContent(
        parser,
        if !(*parser).m_parentParser.is_null() {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
        (*parser).m_encoding,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int as XML_Bool,
        XML_ACCOUNT_DIRECT,
    );
    if result as ::core::ffi::c_uint == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if storeRawNames(parser) == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}
unsafe extern "C" fn externalEntityInitProcessor(
    mut parser: XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut result: XML_Error = initializeEncoding(parser);
    if result as ::core::ffi::c_uint != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    (*parser).m_processor = Some(
        externalEntityInitProcessor2
            as unsafe extern "C" fn(
                XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> XML_Error,
    );
    return externalEntityInitProcessor2(parser, start, end, endPtr);
}
unsafe extern "C" fn externalEntityInitProcessor2(
    mut parser: XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut next: *const ::core::ffi::c_char = start;
    let mut tok: ::core::ffi::c_int = (*(*parser).m_encoding).scanners
        [1 as ::core::ffi::c_int as usize]
        .expect("non-null function pointer")(
        (*parser).m_encoding, start, end, &raw mut next
    );
    match tok {
        XML_TOK_BOM => {
            if accountingDiffTolerated(
                parser,
                tok,
                start,
                next,
                3210 as ::core::ffi::c_int,
                XML_ACCOUNT_DIRECT,
            ) == 0
            {
                accountingOnAbort(parser);
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            if next == end && (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = next;
                return XML_ERROR_NONE;
            }
            start = next;
        }
        XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return XML_ERROR_UNCLOSED_TOKEN;
        }
        XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(
        externalEntityInitProcessor3
            as unsafe extern "C" fn(
                XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> XML_Error,
    );
    return externalEntityInitProcessor3(parser, start, end, endPtr);
}
unsafe extern "C" fn externalEntityInitProcessor3(
    mut parser: XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut tok: ::core::ffi::c_int = 0;
    let mut next: *const ::core::ffi::c_char = start;
    (*parser).m_eventPtr = start;
    tok = (*(*parser).m_encoding).scanners[1 as ::core::ffi::c_int as usize]
        .expect("non-null function pointer")(
        (*parser).m_encoding, start, end, &raw mut next
    );
    (*parser).m_eventEndPtr = next;
    match tok {
        XML_TOK_XML_DECL => {
            let mut result: XML_Error = XML_ERROR_NONE;
            result = processXmlDecl(parser, 1 as ::core::ffi::c_int, start, next);
            if result as ::core::ffi::c_uint
                != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return result;
            }
            match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
                3 => {
                    *endPtr = next;
                    return XML_ERROR_NONE;
                }
                2 => return XML_ERROR_ABORTED,
                1 => {
                    if (*parser).m_reenter != 0 {
                        return XML_ERROR_UNEXPECTED_STATE;
                    }
                }
                _ => {}
            }
            start = next;
        }
        XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_UNCLOSED_TOKEN;
        }
        XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(
        externalEntityContentProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> XML_Error,
    );
    (*parser).m_tagLevel = 1 as ::core::ffi::c_int;
    return externalEntityContentProcessor(parser, start, end, endPtr);
}
unsafe extern "C" fn externalEntityContentProcessor(
    mut parser: XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut result: XML_Error = doContent(
        parser,
        1 as ::core::ffi::c_int,
        (*parser).m_encoding,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int as XML_Bool,
        XML_ACCOUNT_ENTITY_EXPANSION,
    );
    if result as ::core::ffi::c_uint == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if storeRawNames(parser) == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}
unsafe extern "C" fn doContent(
    mut parser: XML_Parser,
    mut startTagLevel: ::core::ffi::c_int,
    mut enc: *const ENCODING,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
    mut haveMore: XML_Bool,
    mut account: XML_Account,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    loop {
        let mut next: *const ::core::ffi::c_char = s;
        let mut tok: ::core::ffi::c_int = (*enc).scanners[1 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            enc, s, end, &raw mut next
        );
        let mut accountAfter: *const ::core::ffi::c_char =
            if tok == XML_TOK_TRAILING_RSQB || tok == XML_TOK_TRAILING_CR {
                if haveMore as ::core::ffi::c_int != 0 {
                    s
                } else {
                    end
                }
            } else {
                next
            };
        if accountingDiffTolerated(
            parser,
            tok,
            s,
            accountAfter,
            3339 as ::core::ffi::c_int,
            account,
        ) == 0
        {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        *eventEndPP = next;
        let mut current_block_281: u64;
        match tok {
            XML_TOK_TRAILING_CR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                *eventEndPP = end;
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c: XML_Char = 0xa as XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c,
                        1 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                if startTagLevel == 0 as ::core::ffi::c_int {
                    return XML_ERROR_NO_ELEMENTS;
                }
                if (*parser).m_tagLevel != startTagLevel {
                    return XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return XML_ERROR_NONE;
            }
            XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                if startTagLevel > 0 as ::core::ffi::c_int {
                    if (*parser).m_tagLevel != startTagLevel {
                        return XML_ERROR_ASYNC_ENTITY;
                    }
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_NO_ELEMENTS;
            }
            XML_TOK_INVALID => {
                *eventPP = next;
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_TOKEN;
            }
            XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            XML_TOK_ENTITY_REF => {
                let mut name: *const XML_Char = ::core::ptr::null::<XML_Char>();
                let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                let mut ch: XML_Char = (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(
                    enc,
                    s.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                ) as XML_Char;
                if ch != 0 {
                    accountingDiffTolerated(
                        parser,
                        tok,
                        &raw mut ch as *mut ::core::ffi::c_char,
                        (&raw mut ch as *mut ::core::ffi::c_char).offset(::core::mem::size_of::<
                            XML_Char,
                        >()
                            as usize
                            as isize),
                        3405 as ::core::ffi::c_int,
                        XML_ACCOUNT_ENTITY_EXPANSION,
                    );
                    if (*parser).m_characterDataHandler.is_some() {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            &raw mut ch,
                            1 as ::core::ffi::c_int,
                        );
                    } else if (*parser).m_defaultHandler.is_some() {
                        reportDefault(parser, enc, s, next);
                    }
                } else {
                    name = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    entity = lookup(
                        parser,
                        &raw mut (*dtd).generalEntities,
                        name as KEY,
                        0 as size_t,
                    ) as *mut ENTITY;
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    if (*dtd).hasParamEntityRefs == 0
                        || (*dtd).standalone as ::core::ffi::c_int != 0
                    {
                        if entity.is_null() {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity).is_internal == 0 {
                            return XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        current_block_281 = 3546145585875536353;
                    } else if entity.is_null() {
                        if (*parser).m_skippedEntityHandler.is_some() {
                            (*parser)
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                name,
                                0 as ::core::ffi::c_int,
                            );
                        } else if (*parser).m_defaultHandler.is_some() {
                            reportDefault(parser, enc, s, next);
                        }
                        current_block_281 = 1957216233951053322;
                    } else {
                        current_block_281 = 3546145585875536353;
                    }
                    match current_block_281 {
                        1957216233951053322 => {}
                        _ => {
                            if (*entity).open != 0 {
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                return XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if !(*entity).textPtr.is_null() {
                                let mut result: XML_Error = XML_ERROR_NONE;
                                if (*parser).m_defaultExpandInternalEntities == 0 {
                                    if (*parser).m_skippedEntityHandler.is_some() {
                                        (*parser)
                                            .m_skippedEntityHandler
                                            .expect("non-null function pointer")(
                                            (*parser).m_handlerArg,
                                            (*entity).name,
                                            0 as ::core::ffi::c_int,
                                        );
                                    } else if (*parser).m_defaultHandler.is_some() {
                                        reportDefault(parser, enc, s, next);
                                    }
                                } else {
                                    result =
                                        processEntity(parser, entity, XML_FALSE, ENTITY_INTERNAL);
                                    if result as ::core::ffi::c_uint
                                        != XML_ERROR_NONE as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        return result;
                                    }
                                }
                            } else if (*parser).m_externalEntityRefHandler.is_some() {
                                let mut context: *const XML_Char = ::core::ptr::null::<XML_Char>();
                                (*entity).open = XML_TRUE;
                                context = getContext(parser);
                                (*entity).open = XML_FALSE;
                                if context.is_null() {
                                    return XML_ERROR_NO_MEMORY;
                                }
                                if (*parser)
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_externalEntityRefHandlerArg,
                                    context,
                                    (*entity).base,
                                    (*entity).systemId,
                                    (*entity).publicId,
                                ) == 0
                                {
                                    return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
                            } else if (*parser).m_defaultHandler.is_some() {
                                reportDefault(parser, enc, s, next);
                            }
                        }
                    }
                }
            }
            XML_TOK_START_TAG_NO_ATTS | XML_TOK_START_TAG_WITH_ATTS => {
                let mut tag: *mut TAG = ::core::ptr::null_mut::<TAG>();
                let mut result_0: XML_Error = XML_ERROR_NONE;
                let mut toPtr: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
                if !(*parser).m_freeTagList.is_null() {
                    tag = (*parser).m_freeTagList;
                    (*parser).m_freeTagList = (*(*parser).m_freeTagList).parent as *mut TAG;
                } else {
                    tag = expat_malloc(
                        parser,
                        ::core::mem::size_of::<TAG>() as size_t,
                        3479 as ::core::ffi::c_int,
                    ) as *mut TAG;
                    if tag.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*tag).buf.raw = expat_malloc(parser, 32 as size_t, 3482 as ::core::ffi::c_int)
                        as *mut ::core::ffi::c_char;
                    if (*tag).buf.raw.is_null() {
                        expat_free(
                            parser,
                            tag as *mut ::core::ffi::c_void,
                            3484 as ::core::ffi::c_int,
                        );
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*tag).bufEnd = (*tag).buf.raw.offset(INIT_TAG_BUF_SIZE as isize);
                }
                (*tag).bindings = ::core::ptr::null_mut::<BINDING>();
                (*tag).parent = (*parser).m_tagStack as *mut tag;
                (*parser).m_tagStack = tag;
                (*tag).name.localPart = ::core::ptr::null::<XML_Char>();
                (*tag).name.prefix = ::core::ptr::null::<XML_Char>();
                (*tag).rawName = s.offset((*enc).minBytesPerChar as isize);
                (*tag).rawNameLength =
                    (*enc).nameLength.expect("non-null function pointer")(enc, (*tag).rawName);
                (*parser).m_tagLevel += 1;
                let mut rawNameEnd: *const ::core::ffi::c_char =
                    (*tag).rawName.offset((*tag).rawNameLength as isize);
                let mut fromPtr: *const ::core::ffi::c_char = (*tag).rawName;
                toPtr = (*tag).buf.str_0;
                loop {
                    let mut convLen: ::core::ffi::c_int = 0;
                    let convert_res: XML_Convert_Result =
                        (*enc).utf8Convert.expect("non-null function pointer")(
                            enc,
                            &raw mut fromPtr,
                            rawNameEnd,
                            &raw mut toPtr as *mut *mut ::core::ffi::c_char,
                            ((*tag).bufEnd as *mut ICHAR)
                                .offset(-(1 as ::core::ffi::c_int as isize)),
                        ) as XML_Convert_Result;
                    convLen = toPtr.offset_from((*tag).buf.str_0) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                    if fromPtr >= rawNameEnd
                        || convert_res as ::core::ffi::c_uint
                            == XML_CONVERT_INPUT_INCOMPLETE as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                    {
                        (*tag).name.strLen = convLen;
                        break;
                    } else {
                        if (SIZE_MAX as size_t).wrapping_div(2 as size_t)
                            < (*tag).bufEnd.offset_from((*tag).buf.raw) as ::core::ffi::c_long
                                as size_t
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        let bufSize: size_t = ((*tag).bufEnd.offset_from((*tag).buf.raw)
                            as ::core::ffi::c_long
                            as size_t)
                            .wrapping_mul(2 as size_t);
                        let mut temp: *mut ::core::ffi::c_char = expat_realloc(
                            parser,
                            (*tag).buf.raw as *mut ::core::ffi::c_void,
                            bufSize,
                            3516 as ::core::ffi::c_int,
                        )
                            as *mut ::core::ffi::c_char;
                        if temp.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*tag).buf.raw = temp;
                        (*tag).bufEnd = temp.offset(bufSize as isize);
                        toPtr = (temp as *mut XML_Char).offset(convLen as isize);
                    }
                }
                (*tag).name.str_0 = (*tag).buf.str_0;
                *toPtr = '\0' as i32 as XML_Char;
                result_0 = storeAtts(
                    parser,
                    enc,
                    s,
                    &raw mut (*tag).name,
                    &raw mut (*tag).bindings,
                    account,
                );
                if result_0 as u64 != 0 {
                    return result_0;
                }
                if (*parser).m_startElementHandler.is_some() {
                    (*parser)
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*tag).name.str_0,
                        (*parser).m_atts as *mut *const XML_Char,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&raw mut (*parser).m_tempPool);
            }
            XML_TOK_EMPTY_ELEMENT_NO_ATTS | XML_TOK_EMPTY_ELEMENT_WITH_ATTS => {
                let mut rawName: *const ::core::ffi::c_char =
                    s.offset((*enc).minBytesPerChar as isize);
                let mut result_1: XML_Error = XML_ERROR_NONE;
                let mut bindings: *mut BINDING = ::core::ptr::null_mut::<BINDING>();
                let mut noElmHandlers: XML_Bool = XML_TRUE;
                let mut name_0: TAG_NAME = TAG_NAME {
                    str_0: ::core::ptr::null::<XML_Char>(),
                    localPart: ::core::ptr::null::<XML_Char>(),
                    prefix: ::core::ptr::null::<XML_Char>(),
                    strLen: 0,
                    uriLen: 0,
                    prefixLen: 0,
                };
                name_0.str_0 = poolStoreString(
                    &raw mut (*parser).m_tempPool,
                    enc,
                    rawName,
                    rawName.offset((*enc).nameLength.expect("non-null function pointer")(
                        enc, rawName,
                    ) as isize),
                );
                if name_0.str_0.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                result_1 = storeAtts(
                    parser,
                    enc,
                    s,
                    &raw mut name_0,
                    &raw mut bindings,
                    XML_ACCOUNT_NONE,
                );
                if result_1 as ::core::ffi::c_uint
                    != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    freeBindings(parser, bindings);
                    return result_1;
                }
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                if (*parser).m_startElementHandler.is_some() {
                    (*parser)
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        name_0.str_0,
                        (*parser).m_atts as *mut *const XML_Char,
                    );
                    noElmHandlers = XML_FALSE;
                }
                if (*parser).m_endElementHandler.is_some() {
                    if (*parser).m_startElementHandler.is_some() {
                        *eventPP = *eventEndPP;
                    }
                    (*parser)
                        .m_endElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg, name_0.str_0
                    );
                    noElmHandlers = XML_FALSE;
                }
                if noElmHandlers as ::core::ffi::c_int != 0 && (*parser).m_defaultHandler.is_some()
                {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&raw mut (*parser).m_tempPool);
                freeBindings(parser, bindings);
                if (*parser).m_tagLevel == 0 as ::core::ffi::c_int
                    && (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                        != XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                        == XML_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                            == XML_PARSING as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*parser).m_reenter as ::core::ffi::c_int != 0
                    {
                        (*parser).m_processor = Some(
                            epilogProcessor
                                as unsafe extern "C" fn(
                                    XML_Parser,
                                    *const ::core::ffi::c_char,
                                    *const ::core::ffi::c_char,
                                    *mut *const ::core::ffi::c_char,
                                )
                                    -> XML_Error,
                        );
                    } else {
                        return epilogProcessor(parser, next, end, nextPtr);
                    }
                }
            }
            XML_TOK_END_TAG => {
                if (*parser).m_tagLevel == startTagLevel {
                    return XML_ERROR_ASYNC_ENTITY;
                } else {
                    let mut len: ::core::ffi::c_int = 0;
                    let mut rawName_0: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut tag_0: *mut TAG = (*parser).m_tagStack;
                    rawName_0 =
                        s.offset(((*enc).minBytesPerChar * 2 as ::core::ffi::c_int) as isize);
                    len = (*enc).nameLength.expect("non-null function pointer")(enc, rawName_0);
                    if len != (*tag_0).rawNameLength
                        || memcmp(
                            (*tag_0).rawName as *const ::core::ffi::c_void,
                            rawName_0 as *const ::core::ffi::c_void,
                            len as size_t,
                        ) != 0 as ::core::ffi::c_int
                    {
                        *eventPP = rawName_0;
                        return XML_ERROR_TAG_MISMATCH;
                    }
                    (*parser).m_tagStack = (*tag_0).parent as *mut TAG;
                    (*tag_0).parent = (*parser).m_freeTagList as *mut tag;
                    (*parser).m_freeTagList = tag_0;
                    (*parser).m_tagLevel -= 1;
                    if (*parser).m_endElementHandler.is_some() {
                        let mut localPart: *const XML_Char = ::core::ptr::null::<XML_Char>();
                        let mut prefix: *const XML_Char = ::core::ptr::null::<XML_Char>();
                        let mut uri: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
                        localPart = (*tag_0).name.localPart;
                        if (*parser).m_ns as ::core::ffi::c_int != 0 && !localPart.is_null() {
                            uri = ((*tag_0).name.str_0 as *mut XML_Char)
                                .offset((*tag_0).name.uriLen as isize);
                            while *localPart != 0 {
                                let fresh22 = localPart;
                                localPart = localPart.offset(1);
                                let fresh23 = uri;
                                uri = uri.offset(1);
                                *fresh23 = *fresh22;
                            }
                            prefix = (*tag_0).name.prefix;
                            if (*parser).m_ns_triplets as ::core::ffi::c_int != 0
                                && !prefix.is_null()
                            {
                                let fresh24 = uri;
                                uri = uri.offset(1);
                                *fresh24 = (*parser).m_namespaceSeparator;
                                while *prefix != 0 {
                                    let fresh25 = prefix;
                                    prefix = prefix.offset(1);
                                    let fresh26 = uri;
                                    uri = uri.offset(1);
                                    *fresh26 = *fresh25;
                                }
                            }
                            *uri = '\0' as i32 as XML_Char;
                        }
                        (*parser)
                            .m_endElementHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*tag_0).name.str_0,
                        );
                    } else if (*parser).m_defaultHandler.is_some() {
                        reportDefault(parser, enc, s, next);
                    }
                    while !(*tag_0).bindings.is_null() {
                        let mut b: *mut BINDING = (*tag_0).bindings;
                        if (*parser).m_endNamespaceDeclHandler.is_some() {
                            (*parser)
                                .m_endNamespaceDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*b).prefix).name,
                            );
                        }
                        (*tag_0).bindings = (*(*tag_0).bindings).nextTagBinding as *mut BINDING;
                        (*b).nextTagBinding = (*parser).m_freeBindingList as *mut binding;
                        (*parser).m_freeBindingList = b;
                        (*(*b).prefix).binding = (*b).prevPrefixBinding as *mut BINDING;
                    }
                    if (*parser).m_tagLevel == 0 as ::core::ffi::c_int
                        && (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                            != XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                            == XML_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                            || (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                                == XML_PARSING as ::core::ffi::c_int as ::core::ffi::c_uint
                                && (*parser).m_reenter as ::core::ffi::c_int != 0
                        {
                            (*parser).m_processor = Some(
                                epilogProcessor
                                    as unsafe extern "C" fn(
                                        XML_Parser,
                                        *const ::core::ffi::c_char,
                                        *const ::core::ffi::c_char,
                                        *mut *const ::core::ffi::c_char,
                                    )
                                        -> XML_Error,
                            );
                        } else {
                            return epilogProcessor(parser, next, end, nextPtr);
                        }
                    }
                }
            }
            XML_TOK_CHAR_REF => {
                let mut n: ::core::ffi::c_int =
                    (*enc).charRefNumber.expect("non-null function pointer")(enc, s);
                if n < 0 as ::core::ffi::c_int {
                    return XML_ERROR_BAD_CHAR_REF;
                }
                if (*parser).m_characterDataHandler.is_some() {
                    let mut buf: [XML_Char; 4] = [0; 4];
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut buf as *mut XML_Char,
                        XmlUtf8Encode(n, &raw mut buf as *mut XML_Char as *mut ::core::ffi::c_char),
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
            XML_TOK_DATA_NEWLINE => {
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c_0: XML_Char = 0xa as XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c_0,
                        1 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_CDATA_SECT_OPEN => {
                let mut result_2: XML_Error = XML_ERROR_NONE;
                if (*parser).m_startCdataSectionHandler.is_some() {
                    (*parser)
                        .m_startCdataSectionHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                } else if 0 as ::core::ffi::c_int != 0 && (*parser).m_characterDataHandler.is_some()
                {
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_dataBuf,
                        0 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                result_2 =
                    doCdataSection(parser, enc, &raw mut next, end, nextPtr, haveMore, account);
                if result_2 as ::core::ffi::c_uint
                    != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return result_2;
                } else if next.is_null() {
                    (*parser).m_processor = Some(
                        cdataSectionProcessor
                            as unsafe extern "C" fn(
                                XML_Parser,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *mut *const ::core::ffi::c_char,
                            ) -> XML_Error,
                    );
                    return result_2;
                }
            }
            XML_TOK_TRAILING_RSQB => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                if (*parser).m_characterDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                        (*enc).utf8Convert.expect("non-null function pointer")(
                            enc,
                            &raw mut s,
                            end,
                            &raw mut dataPtr,
                            (*parser).m_dataBufEnd as *mut ICHAR,
                        );
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*parser).m_dataBuf,
                            dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR)
                                as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        );
                    } else {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *const XML_Char,
                            (end as *const XML_Char).offset_from(s as *const XML_Char)
                                as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                if startTagLevel == 0 as ::core::ffi::c_int {
                    *eventPP = end;
                    return XML_ERROR_NO_ELEMENTS;
                }
                if (*parser).m_tagLevel != startTagLevel {
                    *eventPP = end;
                    return XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return XML_ERROR_NONE;
            }
            XML_TOK_DATA_CHARS => {
                let mut charDataHandler: XML_CharacterDataHandler =
                    (*parser).m_characterDataHandler;
                if charDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr_0: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                            let convert_res_0: XML_Convert_Result =
                                (*enc).utf8Convert.expect("non-null function pointer")(
                                    enc,
                                    &raw mut s,
                                    next,
                                    &raw mut dataPtr_0,
                                    (*parser).m_dataBufEnd as *mut ICHAR,
                                ) as XML_Convert_Result;
                            *eventEndPP = s;
                            charDataHandler.expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr_0.offset_from((*parser).m_dataBuf as *mut ICHAR)
                                    as ::core::ffi::c_long
                                    as ::core::ffi::c_int,
                            );
                            if convert_res_0 as ::core::ffi::c_uint
                                == XML_CONVERT_COMPLETED as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                || convert_res_0 as ::core::ffi::c_uint
                                    == XML_CONVERT_INPUT_INCOMPLETE as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                            {
                                break;
                            }
                            *eventPP = s;
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *const XML_Char,
                            (next as *const XML_Char).offset_from(s as *const XML_Char)
                                as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_PI => {
                if reportProcessingInstruction(parser, enc, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            XML_TOK_COMMENT => {
                if reportComment(parser, enc, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            _ => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
        }
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                *eventPP = next;
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => {
                *eventPP = next;
                return XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    *nextPtr = next;
                    return XML_ERROR_NONE;
                }
            }
            _ => {}
        }
        s = next;
        *eventPP = s;
    }
}
unsafe extern "C" fn freeBindings(mut parser: XML_Parser, mut bindings: *mut BINDING) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        if (*parser).m_endNamespaceDeclHandler.is_some() {
            (*parser)
                .m_endNamespaceDeclHandler
                .expect("non-null function pointer")(
                (*parser).m_handlerArg, (*(*b).prefix).name
            );
        }
        bindings = (*bindings).nextTagBinding as *mut BINDING;
        (*b).nextTagBinding = (*parser).m_freeBindingList as *mut binding;
        (*parser).m_freeBindingList = b;
        (*(*b).prefix).binding = (*b).prevPrefixBinding as *mut BINDING;
    }
}
unsafe extern "C" fn storeAtts(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut attStr: *const ::core::ffi::c_char,
    mut tagNamePtr: *mut TAG_NAME,
    mut bindingsPtr: *mut *mut BINDING,
    mut account: XML_Account,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut elementType: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    let mut nDefaultAtts: ::core::ffi::c_int = 0;
    let mut appAtts: *mut *const XML_Char = ::core::ptr::null_mut::<*const XML_Char>();
    let mut attIndex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prefixLen: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut uri: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    let mut nPrefixes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut binding: *mut BINDING = ::core::ptr::null_mut::<BINDING>();
    let mut localPart: *const XML_Char = ::core::ptr::null::<XML_Char>();
    elementType = lookup(
        parser,
        &raw mut (*dtd).elementTypes,
        (*tagNamePtr).str_0 as KEY,
        0 as size_t,
    ) as *mut ELEMENT_TYPE;
    if elementType.is_null() {
        let mut name: *const XML_Char = poolCopyString(&raw mut (*dtd).pool, (*tagNamePtr).str_0);
        if name.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        elementType = lookup(
            parser,
            &raw mut (*dtd).elementTypes,
            name as KEY,
            ::core::mem::size_of::<ELEMENT_TYPE>() as size_t,
        ) as *mut ELEMENT_TYPE;
        if elementType.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        if (*parser).m_ns as ::core::ffi::c_int != 0
            && setElementTypePrefix(parser, elementType) == 0
        {
            return XML_ERROR_NO_MEMORY;
        }
    }
    nDefaultAtts = (*elementType).nDefaultAtts;
    n = (*enc).getAtts.expect("non-null function pointer")(
        enc,
        attStr,
        (*parser).m_attsSize,
        (*parser).m_atts,
    );
    if n > INT_MAX - nDefaultAtts {
        return XML_ERROR_NO_MEMORY;
    }
    if n + nDefaultAtts > (*parser).m_attsSize {
        let mut oldAttsSize: ::core::ffi::c_int = (*parser).m_attsSize;
        let mut temp: *mut ATTRIBUTE = ::core::ptr::null_mut::<ATTRIBUTE>();
        if nDefaultAtts > INT_MAX - INIT_ATTS_SIZE || n > INT_MAX - (nDefaultAtts + INIT_ATTS_SIZE)
        {
            return XML_ERROR_NO_MEMORY;
        }
        (*parser).m_attsSize = n + nDefaultAtts + INIT_ATTS_SIZE;
        temp = expat_realloc(
            parser,
            (*parser).m_atts as *mut ::core::ffi::c_void,
            ((*parser).m_attsSize as size_t)
                .wrapping_mul(::core::mem::size_of::<ATTRIBUTE>() as size_t),
            3896 as ::core::ffi::c_int,
        ) as *mut ATTRIBUTE;
        if temp.is_null() {
            (*parser).m_attsSize = oldAttsSize;
            return XML_ERROR_NO_MEMORY;
        }
        (*parser).m_atts = temp;
        if n > oldAttsSize {
            (*enc).getAtts.expect("non-null function pointer")(enc, attStr, n, (*parser).m_atts);
        }
    }
    appAtts = (*parser).m_atts as *mut *const XML_Char;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let mut currAtt: *mut ATTRIBUTE = (*parser).m_atts.offset(i as isize) as *mut ATTRIBUTE;
        let mut attId: *mut ATTRIBUTE_ID = getAttributeId(
            parser,
            enc,
            (*currAtt).name,
            (*currAtt)
                .name
                .offset(
                    (*enc).nameLength.expect("non-null function pointer")(enc, (*currAtt).name)
                        as isize,
                ),
        );
        if attId.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        if *(*attId).name.offset(-(1 as ::core::ffi::c_int) as isize) != 0 {
            if enc == (*parser).m_encoding {
                (*parser).m_eventPtr = (*(*parser).m_atts.offset(i as isize)).name;
            }
            return XML_ERROR_DUPLICATE_ATTRIBUTE;
        }
        *(*attId).name.offset(-(1 as ::core::ffi::c_int) as isize) = 1 as XML_Char;
        let fresh27 = attIndex;
        attIndex = attIndex + 1;
        let ref mut fresh28 = *appAtts.offset(fresh27 as isize);
        *fresh28 = (*attId).name;
        if (*(*parser).m_atts.offset(i as isize)).normalized == 0 {
            let mut result: XML_Error = XML_ERROR_NONE;
            let mut isCdata: XML_Bool = XML_TRUE;
            if (*attId).maybeTokenized != 0 {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < nDefaultAtts {
                    if attId
                        == (*(*elementType).defaultAtts.offset(j as isize)).id as *mut ATTRIBUTE_ID
                    {
                        isCdata = (*(*elementType).defaultAtts.offset(j as isize)).isCdata;
                        break;
                    } else {
                        j += 1;
                    }
                }
            }
            result = storeAttributeValue(
                parser,
                enc,
                isCdata,
                (*(*parser).m_atts.offset(i as isize)).valuePtr,
                (*(*parser).m_atts.offset(i as isize)).valueEnd,
                &raw mut (*parser).m_tempPool,
                account,
            );
            if result as u64 != 0 {
                return result;
            }
            let ref mut fresh29 = *appAtts.offset(attIndex as isize);
            *fresh29 = (*parser).m_tempPool.start;
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
        } else {
            let ref mut fresh30 = *appAtts.offset(attIndex as isize);
            *fresh30 = poolStoreString(
                &raw mut (*parser).m_tempPool,
                enc,
                (*(*parser).m_atts.offset(i as isize)).valuePtr,
                (*(*parser).m_atts.offset(i as isize)).valueEnd,
            );
            if (*appAtts.offset(attIndex as isize)).is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
        }
        if !(*attId).prefix.is_null() {
            if (*attId).xmlns != 0 {
                let mut result_0: XML_Error = addBinding(
                    parser,
                    (*attId).prefix,
                    attId,
                    *appAtts.offset(attIndex as isize),
                    bindingsPtr,
                );
                if result_0 as u64 != 0 {
                    return result_0;
                }
                attIndex -= 1;
            } else {
                attIndex += 1;
                nPrefixes += 1;
                *(*attId).name.offset(-(1 as ::core::ffi::c_int) as isize) = 2 as XML_Char;
            }
        } else {
            attIndex += 1;
        }
        i += 1;
    }
    (*parser).m_nSpecifiedAtts = attIndex;
    if !(*elementType).idAtt.is_null()
        && *(*(*elementType).idAtt)
            .name
            .offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < attIndex {
            if *appAtts.offset(i as isize) == (*(*elementType).idAtt).name as *const XML_Char {
                (*parser).m_idAttIndex = i;
                break;
            } else {
                i += 2 as ::core::ffi::c_int;
            }
        }
    } else {
        (*parser).m_idAttIndex = -(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < nDefaultAtts {
        let mut da: *const DEFAULT_ATTRIBUTE = (*elementType).defaultAtts.offset(i as isize);
        if *(*(*da).id).name.offset(-(1 as ::core::ffi::c_int) as isize) == 0
            && !(*da).value.is_null()
        {
            if !(*(*da).id).prefix.is_null() {
                if (*(*da).id).xmlns != 0 {
                    let mut result_1: XML_Error = addBinding(
                        parser,
                        (*(*da).id).prefix,
                        (*da).id,
                        (*da).value,
                        bindingsPtr,
                    );
                    if result_1 as u64 != 0 {
                        return result_1;
                    }
                } else {
                    *(*(*da).id).name.offset(-(1 as ::core::ffi::c_int) as isize) = 2 as XML_Char;
                    nPrefixes += 1;
                    let fresh31 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut fresh32 = *appAtts.offset(fresh31 as isize);
                    *fresh32 = (*(*da).id).name;
                    let fresh33 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut fresh34 = *appAtts.offset(fresh33 as isize);
                    *fresh34 = (*da).value;
                }
            } else {
                *(*(*da).id).name.offset(-(1 as ::core::ffi::c_int) as isize) = 1 as XML_Char;
                let fresh35 = attIndex;
                attIndex = attIndex + 1;
                let ref mut fresh36 = *appAtts.offset(fresh35 as isize);
                *fresh36 = (*(*da).id).name;
                let fresh37 = attIndex;
                attIndex = attIndex + 1;
                let ref mut fresh38 = *appAtts.offset(fresh37 as isize);
                *fresh38 = (*da).value;
            }
        }
        i += 1;
    }
    let ref mut fresh39 = *appAtts.offset(attIndex as isize);
    *fresh39 = ::core::ptr::null::<XML_Char>();
    i = 0 as ::core::ffi::c_int;
    if nPrefixes != 0 {
        let mut j_0: ::core::ffi::c_uint = 0;
        let mut version: ::core::ffi::c_ulong = (*parser).m_nsAttsVersion;
        if (*parser).m_nsAttsPower as usize
            >= (::core::mem::size_of::<::core::ffi::c_uint>() as usize).wrapping_mul(8 as usize)
        {
            return XML_ERROR_NO_MEMORY;
        }
        let mut nsAttsSize: ::core::ffi::c_uint =
            (1 as ::core::ffi::c_uint) << (*parser).m_nsAttsPower as ::core::ffi::c_int;
        let mut oldNsAttsPower: ::core::ffi::c_uchar = (*parser).m_nsAttsPower;
        if nPrefixes << 1 as ::core::ffi::c_int >> (*parser).m_nsAttsPower as ::core::ffi::c_int
            != 0
        {
            let mut temp_0: *mut NS_ATT = ::core::ptr::null_mut::<NS_ATT>();
            loop {
                let fresh40 = (*parser).m_nsAttsPower;
                (*parser).m_nsAttsPower = (*parser).m_nsAttsPower.wrapping_add(1);
                if !(nPrefixes >> fresh40 as ::core::ffi::c_int != 0) {
                    break;
                }
            }
            if ((*parser).m_nsAttsPower as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
                (*parser).m_nsAttsPower = 3 as ::core::ffi::c_uchar;
            }
            if (*parser).m_nsAttsPower as usize
                >= (::core::mem::size_of::<::core::ffi::c_uint>() as usize).wrapping_mul(8 as usize)
            {
                (*parser).m_nsAttsPower = oldNsAttsPower;
                return XML_ERROR_NO_MEMORY;
            }
            nsAttsSize =
                (1 as ::core::ffi::c_uint) << (*parser).m_nsAttsPower as ::core::ffi::c_int;
            temp_0 = expat_realloc(
                parser,
                (*parser).m_nsAtts as *mut ::core::ffi::c_void,
                (nsAttsSize as size_t).wrapping_mul(::core::mem::size_of::<NS_ATT>() as size_t),
                4091 as ::core::ffi::c_int,
            ) as *mut NS_ATT;
            if temp_0.is_null() {
                (*parser).m_nsAttsPower = oldNsAttsPower;
                return XML_ERROR_NO_MEMORY;
            }
            (*parser).m_nsAtts = temp_0;
            version = 0 as ::core::ffi::c_ulong;
        }
        if version == 0 {
            version = INIT_ATTS_VERSION as ::core::ffi::c_ulong;
            j_0 = nsAttsSize;
            while j_0 != 0 as ::core::ffi::c_uint {
                j_0 = j_0.wrapping_sub(1);
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version;
            }
        }
        version = version.wrapping_sub(1);
        (*parser).m_nsAttsVersion = version;
        while i < attIndex {
            let mut s: *const XML_Char = *appAtts.offset(i as isize);
            if *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
            {
                let mut id: *mut ATTRIBUTE_ID = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                let mut b: *const BINDING = ::core::ptr::null::<BINDING>();
                let mut uriHash: ::core::ffi::c_ulong = 0;
                let mut sip_state: siphash = siphash {
                    v0: 0,
                    v1: 0,
                    v2: 0,
                    v3: 0,
                    buf: [0; 8],
                    p: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                    c: 0,
                };
                let mut sip_key: sipkey = sipkey { k: [0; 2] };
                copy_salt_to_sipkey(parser, &raw mut sip_key);
                sip24_init(&raw mut sip_state, &raw mut sip_key);
                *(s as *mut XML_Char).offset(-(1 as ::core::ffi::c_int) as isize) = 0 as XML_Char;
                id = lookup(parser, &raw mut (*dtd).attributeIds, s as KEY, 0 as size_t)
                    as *mut ATTRIBUTE_ID;
                if id.is_null() || (*id).prefix.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                b = (*(*id).prefix).binding;
                if b.is_null() {
                    return XML_ERROR_UNBOUND_PREFIX;
                }
                j_0 = 0 as ::core::ffi::c_uint;
                while j_0 < (*b).uriLen as ::core::ffi::c_uint {
                    let c: XML_Char = *(*b).uri.offset(j_0 as isize);
                    if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                        && poolGrow(&raw mut (*parser).m_tempPool) == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        let fresh41 = (*parser).m_tempPool.ptr;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                        *fresh41 = c;
                        1 as ::core::ffi::c_int
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                sip24_update(
                    &raw mut sip_state,
                    (*b).uri as *const ::core::ffi::c_void,
                    ((*b).uriLen as size_t)
                        .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
                );
                loop {
                    let fresh42 = s;
                    s = s.offset(1);
                    if !(*fresh42 as ::core::ffi::c_int != 0x3a as ::core::ffi::c_int) {
                        break;
                    }
                }
                sip24_update(
                    &raw mut sip_state,
                    s as *const ::core::ffi::c_void,
                    keylen(s as KEY).wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
                );
                loop {
                    if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                        && poolGrow(&raw mut (*parser).m_tempPool) == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        let fresh43 = (*parser).m_tempPool.ptr;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                        *fresh43 = *s;
                        1 as ::core::ffi::c_int
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                    let fresh44 = s;
                    s = s.offset(1);
                    if !(*fresh44 != 0) {
                        break;
                    }
                }
                uriHash = sip24_final(&raw mut sip_state) as ::core::ffi::c_ulong;
                let mut step: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut mask: ::core::ffi::c_ulong =
                    nsAttsSize.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_ulong;
                j_0 = (uriHash & mask) as ::core::ffi::c_uint;
                while (*(*parser).m_nsAtts.offset(j_0 as isize)).version == version {
                    if uriHash == (*(*parser).m_nsAtts.offset(j_0 as isize)).hash {
                        let mut s1: *const XML_Char = (*parser).m_tempPool.start;
                        let mut s2: *const XML_Char =
                            (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
                        while *s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int
                            && *s1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                        {
                            s1 = s1.offset(1);
                            s2 = s2.offset(1);
                        }
                        if *s1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            return XML_ERROR_DUPLICATE_ATTRIBUTE;
                        }
                    }
                    if step == 0 {
                        step = ((uriHash & !mask)
                            >> (*parser).m_nsAttsPower as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                            & mask >> 2 as ::core::ffi::c_int
                            | 1 as ::core::ffi::c_ulong)
                            as ::core::ffi::c_uchar;
                    }
                    if j_0 < step as ::core::ffi::c_uint {
                        j_0 =
                            j_0.wrapping_add(nsAttsSize.wrapping_sub(step as ::core::ffi::c_uint));
                    } else {
                        j_0 = j_0.wrapping_sub(step as ::core::ffi::c_uint);
                    };
                }
                if (*parser).m_ns_triplets != 0 {
                    *(*parser)
                        .m_tempPool
                        .ptr
                        .offset(-(1 as ::core::ffi::c_int) as isize) =
                        (*parser).m_namespaceSeparator;
                    s = (*(*b).prefix).name;
                    loop {
                        if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                            && poolGrow(&raw mut (*parser).m_tempPool) == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            let fresh45 = (*parser).m_tempPool.ptr;
                            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                            *fresh45 = *s;
                            1 as ::core::ffi::c_int
                        } == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        let fresh46 = s;
                        s = s.offset(1);
                        if !(*fresh46 != 0) {
                            break;
                        }
                    }
                }
                s = (*parser).m_tempPool.start;
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                let ref mut fresh47 = *appAtts.offset(i as isize);
                *fresh47 = s;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).hash = uriHash;
                let ref mut fresh48 = (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
                *fresh48 = s;
                nPrefixes -= 1;
                if nPrefixes == 0 {
                    i += 2 as ::core::ffi::c_int;
                    break;
                }
            } else {
                *(s as *mut XML_Char).offset(-(1 as ::core::ffi::c_int) as isize) = 0 as XML_Char;
            }
            i += 2 as ::core::ffi::c_int;
        }
    }
    while i < attIndex {
        *(*appAtts.offset(i as isize) as *mut XML_Char)
            .offset(-(1 as ::core::ffi::c_int) as isize) = 0 as XML_Char;
        i += 2 as ::core::ffi::c_int;
    }
    binding = *bindingsPtr;
    while !binding.is_null() {
        *(*(*binding).attId)
            .name
            .offset(-(1 as ::core::ffi::c_int) as isize) = 0 as XML_Char;
        binding = (*binding).nextTagBinding as *mut BINDING;
    }
    if (*parser).m_ns == 0 {
        return XML_ERROR_NONE;
    }
    if !(*elementType).prefix.is_null() {
        binding = (*(*elementType).prefix).binding;
        if binding.is_null() {
            return XML_ERROR_UNBOUND_PREFIX;
        }
        localPart = (*tagNamePtr).str_0;
        loop {
            let fresh49 = localPart;
            localPart = localPart.offset(1);
            if !(*fresh49 as ::core::ffi::c_int != 0x3a as ::core::ffi::c_int) {
                break;
            }
        }
    } else if !(*dtd).defaultPrefix.binding.is_null() {
        binding = (*dtd).defaultPrefix.binding;
        localPart = (*tagNamePtr).str_0;
    } else {
        return XML_ERROR_NONE;
    }
    prefixLen = 0 as ::core::ffi::c_int;
    if (*parser).m_ns_triplets as ::core::ffi::c_int != 0 && !(*(*binding).prefix).name.is_null() {
        loop {
            let fresh50 = prefixLen;
            prefixLen = prefixLen + 1;
            if !(*(*(*binding).prefix).name.offset(fresh50 as isize) != 0) {
                break;
            }
        }
    }
    (*tagNamePtr).localPart = localPart;
    (*tagNamePtr).uriLen = (*binding).uriLen;
    (*tagNamePtr).prefix = (*(*binding).prefix).name;
    (*tagNamePtr).prefixLen = prefixLen;
    i = 0 as ::core::ffi::c_int;
    loop {
        let fresh51 = i;
        i = i + 1;
        if !(*localPart.offset(fresh51 as isize) != 0) {
            break;
        }
    }
    if (*binding).uriLen > INT_MAX - prefixLen || i > INT_MAX - ((*binding).uriLen + prefixLen) {
        return XML_ERROR_NO_MEMORY;
    }
    n = i + (*binding).uriLen + prefixLen;
    if n > (*binding).uriAlloc {
        let mut p: *mut TAG = ::core::ptr::null_mut::<TAG>();
        if n > INT_MAX - EXPAND_SPARE {
            return XML_ERROR_NO_MEMORY;
        }
        uri = expat_malloc(
            parser,
            ((n + 24 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
            4272 as ::core::ffi::c_int,
        ) as *mut XML_Char;
        if uri.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        (*binding).uriAlloc = n + EXPAND_SPARE;
        memcpy(
            uri as *mut ::core::ffi::c_void,
            (*binding).uri as *const ::core::ffi::c_void,
            ((*binding).uriLen as size_t)
                .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
        );
        p = (*parser).m_tagStack;
        while !p.is_null() {
            if (*p).name.str_0 == (*binding).uri as *const XML_Char {
                (*p).name.str_0 = uri;
            }
            p = (*p).parent as *mut TAG;
        }
        expat_free(
            parser,
            (*binding).uri as *mut ::core::ffi::c_void,
            4280 as ::core::ffi::c_int,
        );
        (*binding).uri = uri;
    }
    uri = (*binding).uri.offset((*binding).uriLen as isize);
    memcpy(
        uri as *mut ::core::ffi::c_void,
        localPart as *const ::core::ffi::c_void,
        (i as size_t).wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
    );
    if prefixLen != 0 {
        uri = uri.offset((i - 1 as ::core::ffi::c_int) as isize);
        *uri = (*parser).m_namespaceSeparator;
        memcpy(
            uri.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*(*binding).prefix).name as *const ::core::ffi::c_void,
            (prefixLen as size_t).wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
        );
    }
    (*tagNamePtr).str_0 = (*binding).uri;
    return XML_ERROR_NONE;
}
unsafe extern "C" fn is_rfc3986_uri_char(mut candidate: XML_Char) -> XML_Bool {
    match candidate as ::core::ffi::c_int {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104
        | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118
        | 119 | 120 | 121 | 122 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 37 | 45
        | 46 | 95 | 126 | 58 | 47 | 63 | 35 | 91 | 93 | 64 | 33 | 36 | 38 | 39 | 40 | 41 | 42
        | 43 | 44 | 59 | 61 => return XML_TRUE,
        _ => return XML_FALSE,
    };
}
unsafe extern "C" fn addBinding(
    mut parser: XML_Parser,
    mut prefix: *mut PREFIX,
    mut attId: *const ATTRIBUTE_ID,
    mut uri: *const XML_Char,
    mut bindingsPtr: *mut *mut BINDING,
) -> XML_Error {
    static mut xmlNamespace: [XML_Char; 37] = [
        ASCII_h as XML_Char,
        ASCII_t as XML_Char,
        ASCII_t as XML_Char,
        ASCII_p as XML_Char,
        ASCII_COLON as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_w as XML_Char,
        ASCII_3 as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_o as XML_Char,
        ASCII_r as XML_Char,
        ASCII_g as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_X as XML_Char,
        ASCII_M as XML_Char,
        ASCII_L as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_1 as XML_Char,
        ASCII_9 as XML_Char,
        ASCII_9 as XML_Char,
        ASCII_8 as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_n as XML_Char,
        ASCII_a as XML_Char,
        ASCII_m as XML_Char,
        ASCII_e as XML_Char,
        ASCII_s as XML_Char,
        ASCII_p as XML_Char,
        ASCII_a as XML_Char,
        ASCII_c as XML_Char,
        ASCII_e as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut xmlnsNamespace: [XML_Char; 30] = [
        ASCII_h as XML_Char,
        ASCII_t as XML_Char,
        ASCII_t as XML_Char,
        ASCII_p as XML_Char,
        ASCII_COLON as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_w as XML_Char,
        ASCII_3 as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_o as XML_Char,
        ASCII_r as XML_Char,
        ASCII_g as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_2 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_x as XML_Char,
        ASCII_m as XML_Char,
        ASCII_l as XML_Char,
        ASCII_n as XML_Char,
        ASCII_s as XML_Char,
        ASCII_SLASH as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    let mut mustBeXML: XML_Bool = XML_FALSE;
    let mut isXML: XML_Bool = XML_TRUE;
    let mut isXMLNS: XML_Bool = XML_TRUE;
    let mut b: *mut BINDING = ::core::ptr::null_mut::<BINDING>();
    let mut len: ::core::ffi::c_int = 0;
    if *uri as ::core::ffi::c_int == '\0' as i32 && !(*prefix).name.is_null() {
        return XML_ERROR_UNDECLARING_PREFIX;
    }
    if !(*prefix).name.is_null()
        && *(*prefix).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x78 as ::core::ffi::c_int
        && *(*prefix).name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x6d as ::core::ffi::c_int
        && *(*prefix).name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x6c as ::core::ffi::c_int
    {
        if *(*prefix).name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x6e as ::core::ffi::c_int
            && *(*prefix).name.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x73 as ::core::ffi::c_int
            && *(*prefix).name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
        {
            return XML_ERROR_RESERVED_PREFIX_XMLNS;
        }
        if *(*prefix).name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
        {
            mustBeXML = XML_TRUE;
        }
    }
    len = 0 as ::core::ffi::c_int;
    while *uri.offset(len as isize) != 0 {
        if isXML as ::core::ffi::c_int != 0
            && (len > xmlLen
                || *uri.offset(len as isize) as ::core::ffi::c_int
                    != xmlNamespace[len as usize] as ::core::ffi::c_int)
        {
            isXML = XML_FALSE;
        }
        if mustBeXML == 0
            && isXMLNS as ::core::ffi::c_int != 0
            && (len > xmlnsLen
                || *uri.offset(len as isize) as ::core::ffi::c_int
                    != xmlnsNamespace[len as usize] as ::core::ffi::c_int)
        {
            isXMLNS = XML_FALSE;
        }
        if (*parser).m_ns as ::core::ffi::c_int != 0
            && *uri.offset(len as isize) as ::core::ffi::c_int
                == (*parser).m_namespaceSeparator as ::core::ffi::c_int
            && is_rfc3986_uri_char(*uri.offset(len as isize)) == 0
        {
            return XML_ERROR_SYNTAX;
        }
        len += 1;
    }
    isXML = (isXML as ::core::ffi::c_int != 0 && len == xmlLen) as ::core::ffi::c_int as XML_Bool;
    isXMLNS =
        (isXMLNS as ::core::ffi::c_int != 0 && len == xmlnsLen) as ::core::ffi::c_int as XML_Bool;
    if mustBeXML as ::core::ffi::c_int != isXML as ::core::ffi::c_int {
        return (if mustBeXML as ::core::ffi::c_int != 0 {
            XML_ERROR_RESERVED_PREFIX_XML as ::core::ffi::c_int
        } else {
            XML_ERROR_RESERVED_NAMESPACE_URI as ::core::ffi::c_int
        }) as XML_Error;
    }
    if isXMLNS != 0 {
        return XML_ERROR_RESERVED_NAMESPACE_URI;
    }
    if (*parser).m_namespaceSeparator != 0 {
        len += 1;
    }
    if !(*parser).m_freeBindingList.is_null() {
        b = (*parser).m_freeBindingList;
        if len > (*b).uriAlloc {
            if len > INT_MAX - EXPAND_SPARE {
                return XML_ERROR_NO_MEMORY;
            }
            let mut temp: *mut XML_Char = expat_realloc(
                parser,
                (*b).uri as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<XML_Char>() as size_t)
                    .wrapping_mul((len + 24 as ::core::ffi::c_int) as size_t),
                4519 as ::core::ffi::c_int,
            ) as *mut XML_Char;
            if temp.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            (*b).uri = temp;
            (*b).uriAlloc = len + EXPAND_SPARE;
        }
        (*parser).m_freeBindingList = (*b).nextTagBinding as *mut BINDING;
    } else {
        b = expat_malloc(
            parser,
            ::core::mem::size_of::<BINDING>() as size_t,
            4527 as ::core::ffi::c_int,
        ) as *mut BINDING;
        if b.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        if len > INT_MAX - EXPAND_SPARE {
            return XML_ERROR_NO_MEMORY;
        }
        (*b).uri = expat_malloc(
            parser,
            (::core::mem::size_of::<XML_Char>() as size_t)
                .wrapping_mul((len + 24 as ::core::ffi::c_int) as size_t),
            4545 as ::core::ffi::c_int,
        ) as *mut XML_Char;
        if (*b).uri.is_null() {
            expat_free(
                parser,
                b as *mut ::core::ffi::c_void,
                4547 as ::core::ffi::c_int,
            );
            return XML_ERROR_NO_MEMORY;
        }
        (*b).uriAlloc = len + EXPAND_SPARE;
    }
    (*b).uriLen = len;
    memcpy(
        (*b).uri as *mut ::core::ffi::c_void,
        uri as *const ::core::ffi::c_void,
        (len as size_t).wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
    );
    if (*parser).m_namespaceSeparator != 0 {
        *(*b).uri.offset((len - 1 as ::core::ffi::c_int) as isize) = (*parser).m_namespaceSeparator;
    }
    (*b).prefix = prefix as *mut prefix;
    (*b).attId = attId as *const attribute_id;
    (*b).prevPrefixBinding = (*prefix).binding as *mut binding;
    if *uri as ::core::ffi::c_int == '\0' as i32
        && prefix == &raw mut (*(*parser).m_dtd).defaultPrefix
    {
        (*prefix).binding = ::core::ptr::null_mut::<BINDING>();
    } else {
        (*prefix).binding = b;
    }
    (*b).nextTagBinding = *bindingsPtr as *mut binding;
    *bindingsPtr = b;
    if !attId.is_null() && (*parser).m_startNamespaceDeclHandler.is_some() {
        (*parser)
            .m_startNamespaceDeclHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            (*prefix).name,
            if !(*prefix).binding.is_null() {
                uri
            } else {
                ::core::ptr::null::<XML_Char>()
            },
        );
    }
    return XML_ERROR_NONE;
}
unsafe extern "C" fn cdataSectionProcessor(
    mut parser: XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut result: XML_Error = doCdataSection(
        parser,
        (*parser).m_encoding,
        &raw mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int as XML_Bool,
        XML_ACCOUNT_DIRECT,
    );
    if result as ::core::ffi::c_uint != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    if !start.is_null() {
        if !(*parser).m_parentParser.is_null() {
            (*parser).m_processor = Some(
                externalEntityContentProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> XML_Error,
            );
            return externalEntityContentProcessor(parser, start, end, endPtr);
        } else {
            (*parser).m_processor = Some(
                contentProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> XML_Error,
            );
            return contentProcessor(parser, start, end, endPtr);
        }
    }
    return result;
}
unsafe extern "C" fn doCdataSection(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut startPtr: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
    mut haveMore: XML_Bool,
    mut account: XML_Account,
) -> XML_Error {
    let mut s: *const ::core::ffi::c_char = *startPtr;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    *startPtr = ::core::ptr::null::<::core::ffi::c_char>();
    loop {
        let mut next: *const ::core::ffi::c_char = s;
        let mut tok: ::core::ffi::c_int = (*enc).scanners[2 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            enc, s, end, &raw mut next
        );
        if accountingDiffTolerated(parser, tok, s, next, 4621 as ::core::ffi::c_int, account) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        *eventEndPP = next;
        match tok {
            XML_TOK_CDATA_SECT_CLOSE => {
                if (*parser).m_endCdataSectionHandler.is_some() {
                    (*parser)
                        .m_endCdataSectionHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                } else if 0 as ::core::ffi::c_int != 0 && (*parser).m_characterDataHandler.is_some()
                {
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_dataBuf,
                        0 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                *startPtr = next;
                *nextPtr = next;
                if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                    == XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return XML_ERROR_ABORTED;
                } else {
                    return XML_ERROR_NONE;
                }
            }
            XML_TOK_DATA_NEWLINE => {
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c: XML_Char = 0xa as XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c,
                        1 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_DATA_CHARS => {
                let mut charDataHandler: XML_CharacterDataHandler =
                    (*parser).m_characterDataHandler;
                if charDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                            let convert_res: XML_Convert_Result =
                                (*enc).utf8Convert.expect("non-null function pointer")(
                                    enc,
                                    &raw mut s,
                                    next,
                                    &raw mut dataPtr,
                                    (*parser).m_dataBufEnd as *mut ICHAR,
                                ) as XML_Convert_Result;
                            *eventEndPP = next;
                            charDataHandler.expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR)
                                    as ::core::ffi::c_long
                                    as ::core::ffi::c_int,
                            );
                            if convert_res as ::core::ffi::c_uint
                                == XML_CONVERT_COMPLETED as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                || convert_res as ::core::ffi::c_uint
                                    == XML_CONVERT_INPUT_INCOMPLETE as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                            {
                                break;
                            }
                            *eventPP = s;
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *const XML_Char,
                            (next as *const XML_Char).offset_from(s as *const XML_Char)
                                as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_INVALID => {
                *eventPP = next;
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            XML_TOK_PARTIAL | XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_CDATA_SECTION;
            }
            _ => {
                *eventPP = next;
                return XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                *eventPP = next;
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => {
                *eventPP = next;
                return XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            _ => {}
        }
        s = next;
        *eventPP = s;
    }
}
unsafe extern "C" fn ignoreSectionProcessor(
    mut parser: XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut result: XML_Error = doIgnoreSection(
        parser,
        (*parser).m_encoding,
        &raw mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int as XML_Bool,
    );
    if result as ::core::ffi::c_uint != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    if !start.is_null() {
        (*parser).m_processor = Some(
            prologProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> XML_Error,
        );
        return prologProcessor(parser, start, end, endPtr);
    }
    return result;
}
unsafe extern "C" fn doIgnoreSection(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut startPtr: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut next: *const ::core::ffi::c_char = *startPtr;
    let mut tok: ::core::ffi::c_int = 0;
    let mut s: *const ::core::ffi::c_char = *startPtr;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    *startPtr = ::core::ptr::null::<::core::ffi::c_char>();
    tok = (*enc).scanners[3 as ::core::ffi::c_int as usize].expect("non-null function pointer")(
        enc,
        s,
        end,
        &raw mut next,
    );
    if accountingDiffTolerated(
        parser,
        tok,
        s,
        next,
        4780 as ::core::ffi::c_int,
        XML_ACCOUNT_DIRECT,
    ) == 0
    {
        accountingOnAbort(parser);
        return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    *eventEndPP = next;
    match tok {
        XML_TOK_IGNORE_SECT => {
            if (*parser).m_defaultHandler.is_some() {
                reportDefault(parser, enc, s, next);
            }
            *startPtr = next;
            *nextPtr = next;
            if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                == XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return XML_ERROR_ABORTED;
            } else {
                return XML_ERROR_NONE;
            }
        }
        XML_TOK_INVALID => {
            *eventPP = next;
            return XML_ERROR_INVALID_TOKEN;
        }
        XML_TOK_PARTIAL_CHAR => {
            if haveMore != 0 {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_PARTIAL_CHAR;
        }
        XML_TOK_PARTIAL | XML_TOK_NONE => {
            if haveMore != 0 {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_SYNTAX;
        }
        _ => {
            *eventPP = next;
            return XML_ERROR_UNEXPECTED_STATE;
        }
    };
}
unsafe extern "C" fn initializeEncoding(mut parser: XML_Parser) -> XML_Error {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = (*parser).m_protocolEncodingName as *const ::core::ffi::c_char;
    if if (*parser).m_ns as ::core::ffi::c_int != 0 {
        Some(
            XmlInitEncodingNS
                as unsafe extern "C" fn(
                    *mut INIT_ENCODING,
                    *mut *const ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            XmlInitEncoding
                as unsafe extern "C" fn(
                    *mut INIT_ENCODING,
                    *mut *const ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
    }
    .expect("non-null function pointer")(
        &raw mut (*parser).m_initEncoding,
        &raw mut (*parser).m_encoding,
        s,
    ) != 0
    {
        return XML_ERROR_NONE;
    }
    return handleUnknownEncoding(parser, (*parser).m_protocolEncodingName);
}
unsafe extern "C" fn processXmlDecl(
    mut parser: XML_Parser,
    mut isGeneralTextEntity: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
    mut next: *const ::core::ffi::c_char,
) -> XML_Error {
    let mut encodingName: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut storedEncName: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut newEncoding: *const ENCODING = ::core::ptr::null::<ENCODING>();
    let mut version: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut versionend: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut storedversion: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut standalone: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if accountingDiffTolerated(
        parser,
        XML_TOK_XML_DECL,
        s,
        next,
        4872 as ::core::ffi::c_int,
        XML_ACCOUNT_DIRECT,
    ) == 0
    {
        accountingOnAbort(parser);
        return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    if if (*parser).m_ns as ::core::ffi::c_int != 0 {
        Some(
            XmlParseXmlDeclNS
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ENCODING,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            XmlParseXmlDecl
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ENCODING,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    }
    .expect("non-null function pointer")(
        isGeneralTextEntity,
        (*parser).m_encoding,
        s,
        next,
        &raw mut (*parser).m_eventPtr,
        &raw mut version,
        &raw mut versionend,
        &raw mut encodingName,
        &raw mut newEncoding,
        &raw mut standalone,
    ) == 0
    {
        if isGeneralTextEntity != 0 {
            return XML_ERROR_TEXT_DECL;
        } else {
            return XML_ERROR_XML_DECL;
        }
    }
    if isGeneralTextEntity == 0 && standalone == 1 as ::core::ffi::c_int {
        (*(*parser).m_dtd).standalone = XML_TRUE;
        if (*parser).m_paramEntityParsing as ::core::ffi::c_uint
            == XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            (*parser).m_paramEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
        }
    }
    if (*parser).m_xmlDeclHandler.is_some() {
        if !encodingName.is_null() {
            storedEncName = poolStoreString(
                &raw mut (*parser).m_temp2Pool,
                (*parser).m_encoding,
                encodingName,
                encodingName.offset((*(*parser).m_encoding)
                    .nameLength
                    .expect("non-null function pointer")(
                    (*parser).m_encoding, encodingName
                ) as isize),
            );
            if storedEncName.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            (*parser).m_temp2Pool.start = (*parser).m_temp2Pool.ptr;
        }
        if !version.is_null() {
            storedversion = poolStoreString(
                &raw mut (*parser).m_temp2Pool,
                (*parser).m_encoding,
                version,
                versionend.offset(-((*(*parser).m_encoding).minBytesPerChar as isize)),
            );
            if storedversion.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
        }
        (*parser)
            .m_xmlDeclHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            storedversion,
            storedEncName,
            standalone,
        );
    } else if (*parser).m_defaultHandler.is_some() {
        reportDefault(parser, (*parser).m_encoding, s, next);
    }
    if (*parser).m_protocolEncodingName.is_null() {
        if !newEncoding.is_null() {
            if (*newEncoding).minBytesPerChar != (*(*parser).m_encoding).minBytesPerChar
                || (*newEncoding).minBytesPerChar == 2 as ::core::ffi::c_int
                    && newEncoding != (*parser).m_encoding
            {
                (*parser).m_eventPtr = encodingName;
                return XML_ERROR_INCORRECT_ENCODING;
            }
            (*parser).m_encoding = newEncoding;
        } else if !encodingName.is_null() {
            let mut result: XML_Error = XML_ERROR_NONE;
            if storedEncName.is_null() {
                storedEncName = poolStoreString(
                    &raw mut (*parser).m_temp2Pool,
                    (*parser).m_encoding,
                    encodingName,
                    encodingName.offset((*(*parser).m_encoding)
                        .nameLength
                        .expect("non-null function pointer")(
                        (*parser).m_encoding, encodingName
                    ) as isize),
                );
                if storedEncName.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            result = handleUnknownEncoding(parser, storedEncName);
            poolClear(&raw mut (*parser).m_temp2Pool);
            if result as ::core::ffi::c_uint
                == XML_ERROR_UNKNOWN_ENCODING as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*parser).m_eventPtr = encodingName;
            }
            return result;
        }
    }
    if !storedEncName.is_null() || !storedversion.is_null() {
        poolClear(&raw mut (*parser).m_temp2Pool);
    }
    return XML_ERROR_NONE;
}
unsafe extern "C" fn handleUnknownEncoding(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Error {
    if (*parser).m_unknownEncodingHandler.is_some() {
        let mut info: XML_Encoding = XML_Encoding {
            map: [0; 256],
            data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            convert: None,
            release: None,
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            info.map[i as usize] = -(1 as ::core::ffi::c_int);
            i += 1;
        }
        info.convert = None;
        info.data = NULL;
        info.release = None;
        if (*parser)
            .m_unknownEncodingHandler
            .expect("non-null function pointer")(
            (*parser).m_unknownEncodingHandlerData,
            encodingName,
            &raw mut info,
        ) != 0
        {
            let mut enc: *mut ENCODING = ::core::ptr::null_mut::<ENCODING>();
            (*parser).m_unknownEncodingMem = expat_malloc(
                parser,
                XmlSizeOfUnknownEncoding() as size_t,
                4965 as ::core::ffi::c_int,
            );
            if (*parser).m_unknownEncodingMem.is_null() {
                if info.release.is_some() {
                    info.release.expect("non-null function pointer")(info.data);
                }
                return XML_ERROR_NO_MEMORY;
            }
            enc = if (*parser).m_ns as ::core::ffi::c_int != 0 {
                Some(
                    XmlInitUnknownEncodingNS
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_int,
                            CONVERTER,
                            *mut ::core::ffi::c_void,
                        ) -> *mut ENCODING,
                )
            } else {
                Some(
                    XmlInitUnknownEncoding
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_int,
                            CONVERTER,
                            *mut ::core::ffi::c_void,
                        ) -> *mut ENCODING,
                )
            }
            .expect("non-null function pointer")(
                (*parser).m_unknownEncodingMem,
                &raw mut info.map as *mut ::core::ffi::c_int,
                info.convert as CONVERTER,
                info.data,
            );
            if !enc.is_null() {
                (*parser).m_unknownEncodingData = info.data;
                (*parser).m_unknownEncodingRelease = info.release;
                (*parser).m_encoding = enc;
                return XML_ERROR_NONE;
            }
        }
        if info.release.is_some() {
            info.release.expect("non-null function pointer")(info.data);
        }
    }
    return XML_ERROR_UNKNOWN_ENCODING;
}
unsafe extern "C" fn prologInitProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut result: XML_Error = initializeEncoding(parser);
    if result as ::core::ffi::c_uint != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    (*parser).m_processor = Some(
        prologProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> XML_Error,
    );
    return prologProcessor(parser, s, end, nextPtr);
}
unsafe extern "C" fn externalParEntInitProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut result: XML_Error = initializeEncoding(parser);
    if result as ::core::ffi::c_uint != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    (*(*parser).m_dtd).paramEntityRead = XML_TRUE;
    if (*parser).m_prologState.inEntityValue != 0 {
        (*parser).m_processor = Some(
            entityValueInitProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> XML_Error,
        );
        return entityValueInitProcessor(parser, s, end, nextPtr);
    } else {
        (*parser).m_processor = Some(
            externalParEntProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> XML_Error,
        );
        return externalParEntProcessor(parser, s, end, nextPtr);
    };
}
unsafe extern "C" fn entityValueInitProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut tok: ::core::ffi::c_int = 0;
    let mut start: *const ::core::ffi::c_char = s;
    let mut next: *const ::core::ffi::c_char = start;
    (*parser).m_eventPtr = start;
    loop {
        tok = (*(*parser).m_encoding).scanners[0 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, start, end, &raw mut next
        );
        (*parser).m_eventEndPtr = next;
        if tok <= 0 as ::core::ffi::c_int {
            if (*parser).m_parsingStatus.finalBuffer == 0 && tok != XML_TOK_INVALID {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            match tok {
                XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
                XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
                XML_TOK_NONE | _ => {}
            }
            return storeEntityValue(
                parser,
                (*parser).m_encoding,
                s,
                end,
                XML_ACCOUNT_DIRECT,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
        } else if tok == XML_TOK_XML_DECL {
            let mut result: XML_Error = XML_ERROR_NONE;
            result = processXmlDecl(parser, 0 as ::core::ffi::c_int, start, next);
            if result as ::core::ffi::c_uint
                != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return result;
            }
            if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                == XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return XML_ERROR_ABORTED;
            }
            *nextPtr = next;
            (*parser).m_processor = Some(
                entityValueProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> XML_Error,
            );
            return entityValueProcessor(parser, next, end, nextPtr);
        } else if tok == XML_TOK_BOM {
            if accountingDiffTolerated(
                parser,
                tok,
                s,
                next,
                5079 as ::core::ffi::c_int,
                XML_ACCOUNT_DIRECT,
            ) == 0
            {
                accountingOnAbort(parser);
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            *nextPtr = next;
            s = next;
        } else if tok == XML_TOK_INSTANCE_START {
            *nextPtr = next;
            return XML_ERROR_SYNTAX;
        }
        start = next;
        (*parser).m_eventPtr = start;
    }
}
unsafe extern "C" fn externalParEntProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut next: *const ::core::ffi::c_char = s;
    let mut tok: ::core::ffi::c_int = 0;
    tok = (*(*parser).m_encoding).scanners[0 as ::core::ffi::c_int as usize]
        .expect("non-null function pointer")((*parser).m_encoding, s, end, &raw mut next);
    if tok <= 0 as ::core::ffi::c_int {
        if (*parser).m_parsingStatus.finalBuffer == 0 && tok != XML_TOK_INVALID {
            *nextPtr = s;
            return XML_ERROR_NONE;
        }
        match tok {
            XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
            XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
            XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
            XML_TOK_NONE | _ => {}
        }
    } else if tok == XML_TOK_BOM {
        if accountingDiffTolerated(
            parser,
            tok,
            s,
            next,
            5132 as ::core::ffi::c_int,
            XML_ACCOUNT_DIRECT,
        ) == 0
        {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        s = next;
        tok = (*(*parser).m_encoding).scanners[0 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, s, end, &raw mut next
        );
    }
    (*parser).m_processor = Some(
        prologProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> XML_Error,
    );
    return doProlog(
        parser,
        (*parser).m_encoding,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int as XML_Bool,
        XML_TRUE,
        XML_ACCOUNT_DIRECT,
    );
}
unsafe extern "C" fn entityValueProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut start: *const ::core::ffi::c_char = s;
    let mut next: *const ::core::ffi::c_char = s;
    let mut enc: *const ENCODING = (*parser).m_encoding;
    let mut tok: ::core::ffi::c_int = 0;
    loop {
        tok = (*enc).scanners[0 as ::core::ffi::c_int as usize].expect("non-null function pointer")(
            enc,
            start,
            end,
            &raw mut next,
        );
        if tok <= 0 as ::core::ffi::c_int {
            if (*parser).m_parsingStatus.finalBuffer == 0 && tok != XML_TOK_INVALID {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            match tok {
                XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
                XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
                XML_TOK_NONE | _ => {}
            }
            return storeEntityValue(
                parser,
                enc,
                s,
                end,
                XML_ACCOUNT_DIRECT,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
        } else if tok == XML_TOK_INSTANCE_START {
            *nextPtr = next;
            return XML_ERROR_SYNTAX;
        }
        start = next;
    }
}
unsafe extern "C" fn prologProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut next: *const ::core::ffi::c_char = s;
    let mut tok: ::core::ffi::c_int = (*(*parser).m_encoding).scanners
        [0 as ::core::ffi::c_int as usize]
        .expect("non-null function pointer")(
        (*parser).m_encoding, s, end, &raw mut next
    );
    return doProlog(
        parser,
        (*parser).m_encoding,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int as XML_Bool,
        XML_TRUE,
        XML_ACCOUNT_DIRECT,
    );
}
unsafe extern "C" fn doProlog(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut tok: ::core::ffi::c_int,
    mut next: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
    mut haveMore: XML_Bool,
    mut allowClosingDoctype: XML_Bool,
    mut account: XML_Account,
) -> XML_Error {
    let mut current_block: u64;
    static mut externalSubsetName: [XML_Char; 2] =
        [ASCII_HASH as XML_Char, '\0' as i32 as XML_Char];
    static mut atypeCDATA: [XML_Char; 6] = [
        ASCII_C as XML_Char,
        ASCII_D as XML_Char,
        ASCII_A as XML_Char,
        ASCII_T as XML_Char,
        ASCII_A as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut atypeID: [XML_Char; 3] = [
        ASCII_I as XML_Char,
        ASCII_D as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut atypeIDREF: [XML_Char; 6] = [
        ASCII_I as XML_Char,
        ASCII_D as XML_Char,
        ASCII_R as XML_Char,
        ASCII_E as XML_Char,
        ASCII_F as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut atypeIDREFS: [XML_Char; 7] = [
        ASCII_I as XML_Char,
        ASCII_D as XML_Char,
        ASCII_R as XML_Char,
        ASCII_E as XML_Char,
        ASCII_F as XML_Char,
        ASCII_S as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut atypeENTITY: [XML_Char; 7] = [
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_T as XML_Char,
        ASCII_Y as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut atypeENTITIES: [XML_Char; 9] = [
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_E as XML_Char,
        ASCII_S as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut atypeNMTOKEN: [XML_Char; 8] = [
        ASCII_N as XML_Char,
        ASCII_M as XML_Char,
        ASCII_T as XML_Char,
        ASCII_O as XML_Char,
        ASCII_K as XML_Char,
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut atypeNMTOKENS: [XML_Char; 9] = [
        ASCII_N as XML_Char,
        ASCII_M as XML_Char,
        ASCII_T as XML_Char,
        ASCII_O as XML_Char,
        ASCII_K as XML_Char,
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        ASCII_S as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut notationPrefix: [XML_Char; 10] = [
        ASCII_N as XML_Char,
        ASCII_O as XML_Char,
        ASCII_T as XML_Char,
        ASCII_A as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_O as XML_Char,
        ASCII_N as XML_Char,
        ASCII_LPAREN as XML_Char,
        '\0' as i32 as XML_Char,
    ];
    static mut enumValueSep: [XML_Char; 2] = [ASCII_PIPE as XML_Char, '\0' as i32 as XML_Char];
    static mut enumValueStart: [XML_Char; 2] = [ASCII_LPAREN as XML_Char, '\0' as i32 as XML_Char];
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut quant: XML_Content_Quant = XML_CQUANT_NONE;
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    loop {
        let mut role: ::core::ffi::c_int = 0;
        let mut handleDefault: XML_Bool = XML_TRUE;
        *eventPP = s;
        *eventEndPP = next;
        if tok <= 0 as ::core::ffi::c_int {
            if haveMore as ::core::ffi::c_int != 0 && tok != XML_TOK_INVALID {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            match tok {
                XML_TOK_INVALID => {
                    *eventPP = next;
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
                -15 => {
                    tok = -tok;
                }
                XML_TOK_NONE => {
                    if enc != (*parser).m_encoding
                        && (*(*parser).m_openInternalEntities).betweenDecl == 0
                    {
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    if (*parser).m_isParamEntity as ::core::ffi::c_int != 0
                        || enc != (*parser).m_encoding
                    {
                        if (*parser)
                            .m_prologState
                            .handler
                            .expect("non-null function pointer")(
                            &raw mut (*parser).m_prologState,
                            -(4 as ::core::ffi::c_int),
                            end,
                            end,
                            enc,
                        ) == XML_ROLE_ERROR as ::core::ffi::c_int
                        {
                            return XML_ERROR_INCOMPLETE_PE;
                        }
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_NO_ELEMENTS;
                }
                _ => {
                    tok = -tok;
                    next = end;
                }
            }
        }
        role = (*parser)
            .m_prologState
            .handler
            .expect("non-null function pointer")(
            &raw mut (*parser).m_prologState,
            tok,
            s,
            next,
            enc,
        );
        match role {
            2 | 1 | 57 => {}
            _ => {
                if accountingDiffTolerated(
                    parser,
                    tok,
                    s,
                    next,
                    5312 as ::core::ffi::c_int,
                    account,
                ) == 0
                {
                    accountingOnAbort(parser);
                    return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
                }
            }
        }
        match role {
            1 => {
                let mut result: XML_Error =
                    processXmlDecl(parser, 0 as ::core::ffi::c_int, s, next);
                if result as ::core::ffi::c_uint
                    != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return result;
                }
                enc = (*parser).m_encoding;
                handleDefault = XML_FALSE;
                current_block = 8258632986558375165;
            }
            4 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser).m_doctypeName =
                        poolStoreString(&raw mut (*parser).m_tempPool, enc, s, next);
                    if (*parser).m_doctypeName.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    (*parser).m_doctypePubid = ::core::ptr::null::<XML_Char>();
                    handleDefault = XML_FALSE;
                }
                (*parser).m_doctypeSysid = ::core::ptr::null::<XML_Char>();
                current_block = 8258632986558375165;
            }
            7 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser)
                        .m_startDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_doctypeName,
                        (*parser).m_doctypeSysid,
                        (*parser).m_doctypePubid,
                        1 as ::core::ffi::c_int,
                    );
                    (*parser).m_doctypeName = ::core::ptr::null::<XML_Char>();
                    poolClear(&raw mut (*parser).m_tempPool);
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            57 => {
                let mut result_0: XML_Error =
                    processXmlDecl(parser, 1 as ::core::ffi::c_int, s, next);
                if result_0 as ::core::ffi::c_uint
                    != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return result_0;
                }
                enc = (*parser).m_encoding;
                handleDefault = XML_FALSE;
                current_block = 8258632986558375165;
            }
            6 => {
                (*parser).m_useForeignDTD = XML_FALSE;
                (*parser).m_declEntity = lookup(
                    parser,
                    &raw mut (*dtd).paramEntities,
                    &raw const externalSubsetName as KEY,
                    ::core::mem::size_of::<ENTITY>() as size_t,
                ) as *mut ENTITY;
                if (*parser).m_declEntity.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                (*dtd).hasParamEntityRefs = XML_TRUE;
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    let mut pubId: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
                    if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP)
                        == 0
                    {
                        return XML_ERROR_PUBLICID;
                    }
                    pubId = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if pubId.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(pubId);
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    (*parser).m_doctypePubid = pubId;
                    handleDefault = XML_FALSE;
                    current_block = 14529472648802745661;
                } else {
                    current_block = 13984953578200811817;
                }
            }
            14 => {
                current_block = 13984953578200811817;
            }
            8 => {
                if allowClosingDoctype as ::core::ffi::c_int != XML_TRUE as ::core::ffi::c_int {
                    return XML_ERROR_INVALID_TOKEN;
                }
                if !(*parser).m_doctypeName.is_null() {
                    (*parser)
                        .m_startDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_doctypeName,
                        (*parser).m_doctypeSysid,
                        (*parser).m_doctypePubid,
                        0 as ::core::ffi::c_int,
                    );
                    poolClear(&raw mut (*parser).m_tempPool);
                    handleDefault = XML_FALSE;
                }
                if !(*parser).m_doctypeSysid.is_null()
                    || (*parser).m_useForeignDTD as ::core::ffi::c_int != 0
                {
                    let mut hadParamEntityRefs: XML_Bool = (*dtd).hasParamEntityRefs;
                    (*dtd).hasParamEntityRefs = XML_TRUE;
                    if (*parser).m_paramEntityParsing as ::core::ffi::c_uint != 0
                        && (*parser).m_externalEntityRefHandler.is_some()
                    {
                        let mut entity: *mut ENTITY = lookup(
                            parser,
                            &raw mut (*dtd).paramEntities,
                            &raw const externalSubsetName as KEY,
                            ::core::mem::size_of::<ENTITY>() as size_t,
                        ) as *mut ENTITY;
                        if entity.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        if (*parser).m_useForeignDTD != 0 {
                            (*entity).base = (*parser).m_curBase;
                        }
                        (*dtd).paramEntityRead = XML_FALSE;
                        if (*parser)
                            .m_externalEntityRefHandler
                            .expect("non-null function pointer")(
                            (*parser).m_externalEntityRefHandlerArg,
                            ::core::ptr::null::<XML_Char>(),
                            (*entity).base,
                            (*entity).systemId,
                            (*entity).publicId,
                        ) == 0
                        {
                            return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                        }
                        if (*dtd).paramEntityRead != 0 {
                            if (*dtd).standalone == 0
                                && (*parser).m_notStandaloneHandler.is_some()
                                && (*parser)
                                    .m_notStandaloneHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_handlerArg
                                ) == 0
                            {
                                return XML_ERROR_NOT_STANDALONE;
                            }
                        } else if (*parser).m_doctypeSysid.is_null() {
                            (*dtd).hasParamEntityRefs = hadParamEntityRefs;
                        }
                    }
                    (*parser).m_useForeignDTD = XML_FALSE;
                }
                if (*parser).m_endDoctypeDeclHandler.is_some() {
                    (*parser)
                        .m_endDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            2 => {
                if (*parser).m_useForeignDTD != 0 {
                    let mut hadParamEntityRefs_0: XML_Bool = (*dtd).hasParamEntityRefs;
                    (*dtd).hasParamEntityRefs = XML_TRUE;
                    if (*parser).m_paramEntityParsing as ::core::ffi::c_uint != 0
                        && (*parser).m_externalEntityRefHandler.is_some()
                    {
                        let mut entity_0: *mut ENTITY = lookup(
                            parser,
                            &raw mut (*dtd).paramEntities,
                            &raw const externalSubsetName as KEY,
                            ::core::mem::size_of::<ENTITY>() as size_t,
                        ) as *mut ENTITY;
                        if entity_0.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*entity_0).base = (*parser).m_curBase;
                        (*dtd).paramEntityRead = XML_FALSE;
                        if (*parser)
                            .m_externalEntityRefHandler
                            .expect("non-null function pointer")(
                            (*parser).m_externalEntityRefHandlerArg,
                            ::core::ptr::null::<XML_Char>(),
                            (*entity_0).base,
                            (*entity_0).systemId,
                            (*entity_0).publicId,
                        ) == 0
                        {
                            return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                        }
                        if (*dtd).paramEntityRead != 0 {
                            if (*dtd).standalone == 0
                                && (*parser).m_notStandaloneHandler.is_some()
                                && (*parser)
                                    .m_notStandaloneHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_handlerArg
                                ) == 0
                            {
                                return XML_ERROR_NOT_STANDALONE;
                            }
                        } else {
                            (*dtd).hasParamEntityRefs = hadParamEntityRefs_0;
                        }
                    }
                }
                (*parser).m_processor = Some(
                    contentProcessor
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> XML_Error,
                );
                return contentProcessor(parser, s, end, nextPtr);
            }
            34 => {
                (*parser).m_declElementType = getElementType(parser, enc, s, next);
                if (*parser).m_declElementType.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                current_block = 14544254959461281264;
            }
            22 => {
                (*parser).m_declAttributeId = getAttributeId(parser, enc, s, next);
                if (*parser).m_declAttributeId.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                (*parser).m_declAttributeIsCdata = XML_FALSE;
                (*parser).m_declAttributeType = ::core::ptr::null::<XML_Char>();
                (*parser).m_declAttributeIsId = XML_FALSE;
                current_block = 14544254959461281264;
            }
            23 => {
                (*parser).m_declAttributeIsCdata = XML_TRUE;
                (*parser).m_declAttributeType = &raw const atypeCDATA as *const XML_Char;
                current_block = 14544254959461281264;
            }
            24 => {
                (*parser).m_declAttributeIsId = XML_TRUE;
                (*parser).m_declAttributeType = &raw const atypeID as *const XML_Char;
                current_block = 14544254959461281264;
            }
            25 => {
                (*parser).m_declAttributeType = &raw const atypeIDREF as *const XML_Char;
                current_block = 14544254959461281264;
            }
            26 => {
                (*parser).m_declAttributeType = &raw const atypeIDREFS as *const XML_Char;
                current_block = 14544254959461281264;
            }
            27 => {
                (*parser).m_declAttributeType = &raw const atypeENTITY as *const XML_Char;
                current_block = 14544254959461281264;
            }
            28 => {
                (*parser).m_declAttributeType = &raw const atypeENTITIES as *const XML_Char;
                current_block = 14544254959461281264;
            }
            29 => {
                (*parser).m_declAttributeType = &raw const atypeNMTOKEN as *const XML_Char;
                current_block = 14544254959461281264;
            }
            30 => {
                (*parser).m_declAttributeType = &raw const atypeNMTOKENS as *const XML_Char;
                current_block = 14544254959461281264;
            }
            31 | 32 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    let mut prefix: *const XML_Char = ::core::ptr::null::<XML_Char>();
                    if !(*parser).m_declAttributeType.is_null() {
                        prefix = &raw const enumValueSep as *const XML_Char;
                    } else {
                        prefix = if role == XML_ROLE_ATTRIBUTE_NOTATION_VALUE as ::core::ffi::c_int
                        {
                            &raw const notationPrefix as *const XML_Char
                        } else {
                            &raw const enumValueStart as *const XML_Char
                        };
                    }
                    if poolAppendString(&raw mut (*parser).m_tempPool, prefix).is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if poolAppend(&raw mut (*parser).m_tempPool, enc, s, next).is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            35 | 36 => {
                if (*dtd).keepProcessing != 0 {
                    if defineAttribute(
                        (*parser).m_declElementType,
                        (*parser).m_declAttributeId,
                        (*parser).m_declAttributeIsCdata,
                        (*parser).m_declAttributeIsId,
                        ::core::ptr::null::<XML_Char>(),
                        parser,
                    ) == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*parser).m_attlistDeclHandler.is_some()
                        && !(*parser).m_declAttributeType.is_null()
                    {
                        if *(*parser).m_declAttributeType as ::core::ffi::c_int
                            == 0x28 as ::core::ffi::c_int
                            || *(*parser).m_declAttributeType as ::core::ffi::c_int
                                == 0x4e as ::core::ffi::c_int
                                && *(*parser)
                                    .m_declAttributeType
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0x4f as ::core::ffi::c_int
                        {
                            if (if (*parser).m_tempPool.ptr
                                == (*parser).m_tempPool.end as *mut XML_Char
                                && poolGrow(&raw mut (*parser).m_tempPool) == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                let fresh1 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *fresh1 = 0x29 as XML_Char;
                                1 as ::core::ffi::c_int
                            }) == 0
                                || (if (*parser).m_tempPool.ptr
                                    == (*parser).m_tempPool.end as *mut XML_Char
                                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                                {
                                    0 as ::core::ffi::c_int
                                } else {
                                    let fresh2 = (*parser).m_tempPool.ptr;
                                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                    *fresh2 = '\0' as i32 as XML_Char;
                                    1 as ::core::ffi::c_int
                                }) == 0
                            {
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                        }
                        *eventEndPP = s;
                        (*parser)
                            .m_attlistDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            (*(*parser).m_declAttributeId).name,
                            (*parser).m_declAttributeType,
                            ::core::ptr::null::<XML_Char>(),
                            (role == XML_ROLE_REQUIRED_ATTRIBUTE_VALUE as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                        );
                        handleDefault = XML_FALSE;
                    }
                }
                poolClear(&raw mut (*parser).m_tempPool);
                current_block = 8258632986558375165;
            }
            37 | 38 => {
                if (*dtd).keepProcessing != 0 {
                    let mut attVal: *const XML_Char = ::core::ptr::null::<XML_Char>();
                    let mut result_1: XML_Error = storeAttributeValue(
                        parser,
                        enc,
                        (*parser).m_declAttributeIsCdata,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                        &raw mut (*dtd).pool,
                        XML_ACCOUNT_NONE,
                    );
                    if result_1 as u64 != 0 {
                        return result_1;
                    }
                    attVal = (*dtd).pool.start;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if defineAttribute(
                        (*parser).m_declElementType,
                        (*parser).m_declAttributeId,
                        (*parser).m_declAttributeIsCdata,
                        XML_FALSE,
                        attVal,
                        parser,
                    ) == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*parser).m_attlistDeclHandler.is_some()
                        && !(*parser).m_declAttributeType.is_null()
                    {
                        if *(*parser).m_declAttributeType as ::core::ffi::c_int
                            == 0x28 as ::core::ffi::c_int
                            || *(*parser).m_declAttributeType as ::core::ffi::c_int
                                == 0x4e as ::core::ffi::c_int
                                && *(*parser)
                                    .m_declAttributeType
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0x4f as ::core::ffi::c_int
                        {
                            if (if (*parser).m_tempPool.ptr
                                == (*parser).m_tempPool.end as *mut XML_Char
                                && poolGrow(&raw mut (*parser).m_tempPool) == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                let fresh3 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *fresh3 = 0x29 as XML_Char;
                                1 as ::core::ffi::c_int
                            }) == 0
                                || (if (*parser).m_tempPool.ptr
                                    == (*parser).m_tempPool.end as *mut XML_Char
                                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                                {
                                    0 as ::core::ffi::c_int
                                } else {
                                    let fresh4 = (*parser).m_tempPool.ptr;
                                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                    *fresh4 = '\0' as i32 as XML_Char;
                                    1 as ::core::ffi::c_int
                                }) == 0
                            {
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                        }
                        *eventEndPP = s;
                        (*parser)
                            .m_attlistDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            (*(*parser).m_declAttributeId).name,
                            (*parser).m_declAttributeType,
                            attVal,
                            (role == XML_ROLE_FIXED_ATTRIBUTE_VALUE as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                        );
                        poolClear(&raw mut (*parser).m_tempPool);
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            12 => {
                if (*dtd).keepProcessing != 0 {
                    let mut result_2: XML_Error = callStoreEntityValue(
                        parser,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                        XML_ACCOUNT_NONE,
                    );
                    if !(*parser).m_declEntity.is_null() {
                        (*(*parser).m_declEntity).textPtr = (*dtd).entityValuePool.start;
                        (*(*parser).m_declEntity).textLen = (*dtd)
                            .entityValuePool
                            .ptr
                            .offset_from((*dtd).entityValuePool.start)
                            as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                        (*dtd).entityValuePool.start = (*dtd).entityValuePool.ptr;
                        if (*parser).m_entityDeclHandler.is_some() {
                            *eventEndPP = s;
                            (*parser)
                                .m_entityDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*parser).m_declEntity).name,
                                (*(*parser).m_declEntity).is_param as ::core::ffi::c_int,
                                (*(*parser).m_declEntity).textPtr,
                                (*(*parser).m_declEntity).textLen,
                                (*parser).m_curBase,
                                ::core::ptr::null::<XML_Char>(),
                                ::core::ptr::null::<XML_Char>(),
                                ::core::ptr::null::<XML_Char>(),
                            );
                            handleDefault = XML_FALSE;
                        }
                    } else {
                        (*dtd).entityValuePool.ptr = (*dtd).entityValuePool.start;
                    }
                    if result_2 as ::core::ffi::c_uint
                        != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        return result_2;
                    }
                }
                current_block = 8258632986558375165;
            }
            5 => {
                (*parser).m_useForeignDTD = XML_FALSE;
                (*dtd).hasParamEntityRefs = XML_TRUE;
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser).m_doctypeSysid = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if (*parser).m_doctypeSysid.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = XML_FALSE;
                } else {
                    (*parser).m_doctypeSysid = &raw const externalSubsetName as *const XML_Char;
                }
                if (*dtd).standalone == 0
                    && (*parser).m_paramEntityParsing as u64 == 0
                    && (*parser).m_notStandaloneHandler.is_some()
                    && (*parser)
                        .m_notStandaloneHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    ) == 0
                {
                    return XML_ERROR_NOT_STANDALONE;
                }
                if (*parser).m_declEntity.is_null() {
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).paramEntities,
                        &raw const externalSubsetName as KEY,
                        ::core::mem::size_of::<ENTITY>() as size_t,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*(*parser).m_declEntity).publicId = ::core::ptr::null::<XML_Char>();
                }
                current_block = 1224196737208379537;
            }
            13 => {
                current_block = 1224196737208379537;
            }
            15 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                    && (*parser).m_entityDeclHandler.is_some()
                {
                    *eventEndPP = s;
                    (*parser)
                        .m_entityDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*(*parser).m_declEntity).name,
                        (*(*parser).m_declEntity).is_param as ::core::ffi::c_int,
                        ::core::ptr::null::<XML_Char>(),
                        0 as ::core::ffi::c_int,
                        (*(*parser).m_declEntity).base,
                        (*(*parser).m_declEntity).systemId,
                        (*(*parser).m_declEntity).publicId,
                        ::core::ptr::null::<XML_Char>(),
                    );
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            16 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                {
                    (*(*parser).m_declEntity).notation =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if (*(*parser).m_declEntity).notation.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_unparsedEntityDeclHandler.is_some() {
                        *eventEndPP = s;
                        (*parser)
                            .m_unparsedEntityDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declEntity).name,
                            (*(*parser).m_declEntity).base,
                            (*(*parser).m_declEntity).systemId,
                            (*(*parser).m_declEntity).publicId,
                            (*(*parser).m_declEntity).notation,
                        );
                        handleDefault = XML_FALSE;
                    } else if (*parser).m_entityDeclHandler.is_some() {
                        *eventEndPP = s;
                        (*parser)
                            .m_entityDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declEntity).name,
                            0 as ::core::ffi::c_int,
                            ::core::ptr::null::<XML_Char>(),
                            0 as ::core::ffi::c_int,
                            (*(*parser).m_declEntity).base,
                            (*(*parser).m_declEntity).systemId,
                            (*(*parser).m_declEntity).publicId,
                            (*(*parser).m_declEntity).notation,
                        );
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            9 => {
                if (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(enc, s, next)
                    != 0
                {
                    (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                } else if (*dtd).keepProcessing != 0 {
                    let mut name: *const XML_Char =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if name.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).generalEntities,
                        name as KEY,
                        ::core::mem::size_of::<ENTITY>() as size_t,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*(*parser).m_declEntity).name != name {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                    } else {
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        (*(*parser).m_declEntity).publicId = ::core::ptr::null::<XML_Char>();
                        (*(*parser).m_declEntity).is_param = XML_FALSE;
                        (*(*parser).m_declEntity).is_internal =
                            !(!(*parser).m_parentParser.is_null()
                                || !(*parser).m_openInternalEntities.is_null())
                                as ::core::ffi::c_int as XML_Bool;
                        if (*parser).m_entityDeclHandler.is_some() {
                            handleDefault = XML_FALSE;
                        }
                    }
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                }
                current_block = 8258632986558375165;
            }
            10 => {
                if (*dtd).keepProcessing != 0 {
                    let mut name_0: *const XML_Char =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if name_0.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).paramEntities,
                        name_0 as KEY,
                        ::core::mem::size_of::<ENTITY>() as size_t,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*(*parser).m_declEntity).name != name_0 {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                    } else {
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        (*(*parser).m_declEntity).publicId = ::core::ptr::null::<XML_Char>();
                        (*(*parser).m_declEntity).is_param = XML_TRUE;
                        (*(*parser).m_declEntity).is_internal =
                            !(!(*parser).m_parentParser.is_null()
                                || !(*parser).m_openInternalEntities.is_null())
                                as ::core::ffi::c_int as XML_Bool;
                        if (*parser).m_entityDeclHandler.is_some() {
                            handleDefault = XML_FALSE;
                        }
                    }
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                }
                current_block = 8258632986558375165;
            }
            18 => {
                (*parser).m_declNotationPublicId = ::core::ptr::null::<XML_Char>();
                (*parser).m_declNotationName = ::core::ptr::null::<XML_Char>();
                if (*parser).m_notationDeclHandler.is_some() {
                    (*parser).m_declNotationName =
                        poolStoreString(&raw mut (*parser).m_tempPool, enc, s, next);
                    if (*parser).m_declNotationName.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            21 => {
                if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP) == 0
                {
                    return XML_ERROR_PUBLICID;
                }
                if !(*parser).m_declNotationName.is_null() {
                    let mut tem_0: *mut XML_Char = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if tem_0.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(tem_0);
                    (*parser).m_declNotationPublicId = tem_0;
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            19 => {
                if !(*parser).m_declNotationName.is_null()
                    && (*parser).m_notationDeclHandler.is_some()
                {
                    let mut systemId: *const XML_Char = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if systemId.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    *eventEndPP = s;
                    (*parser)
                        .m_notationDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_declNotationName,
                        (*parser).m_curBase,
                        systemId,
                        (*parser).m_declNotationPublicId,
                    );
                    handleDefault = XML_FALSE;
                }
                poolClear(&raw mut (*parser).m_tempPool);
                current_block = 8258632986558375165;
            }
            20 => {
                if !(*parser).m_declNotationPublicId.is_null()
                    && (*parser).m_notationDeclHandler.is_some()
                {
                    *eventEndPP = s;
                    (*parser)
                        .m_notationDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_declNotationName,
                        (*parser).m_curBase,
                        ::core::ptr::null::<XML_Char>(),
                        (*parser).m_declNotationPublicId,
                    );
                    handleDefault = XML_FALSE;
                }
                poolClear(&raw mut (*parser).m_tempPool);
                current_block = 8258632986558375165;
            }
            -1 => match tok {
                XML_TOK_PARAM_ENTITY_REF => return XML_ERROR_PARAM_ENTITY_REF,
                XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
                _ => return XML_ERROR_SYNTAX,
            },
            58 => {
                let mut result_3: XML_Error = XML_ERROR_NONE;
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                handleDefault = XML_FALSE;
                result_3 = doIgnoreSection(parser, enc, &raw mut next, end, nextPtr, haveMore);
                if result_3 as ::core::ffi::c_uint
                    != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return result_3;
                } else if next.is_null() {
                    (*parser).m_processor = Some(
                        ignoreSectionProcessor
                            as unsafe extern "C" fn(
                                XML_Parser,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *mut *const ::core::ffi::c_char,
                            ) -> XML_Error,
                    );
                    return result_3;
                }
                current_block = 8258632986558375165;
            }
            44 => {
                if (*parser).m_prologState.level >= (*parser).m_groupSize {
                    if (*parser).m_groupSize != 0 {
                        if (*parser).m_groupSize
                            > (-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                                .wrapping_div(2 as ::core::ffi::c_uint)
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*parser).m_groupSize =
                            (*parser).m_groupSize.wrapping_mul(2 as ::core::ffi::c_uint);
                        let new_connector: *mut ::core::ffi::c_char = expat_realloc(
                            parser,
                            (*parser).m_groupConnector as *mut ::core::ffi::c_void,
                            (*parser).m_groupSize as size_t,
                            5926 as ::core::ffi::c_int,
                        )
                            as *mut ::core::ffi::c_char;
                        if new_connector.is_null() {
                            (*parser).m_groupSize =
                                (*parser).m_groupSize.wrapping_div(2 as ::core::ffi::c_uint);
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*parser).m_groupConnector = new_connector;
                        if !(*dtd).scaffIndex.is_null() {
                            let new_scaff_index: *mut ::core::ffi::c_int = expat_realloc(
                                parser,
                                (*dtd).scaffIndex as *mut ::core::ffi::c_void,
                                ((*parser).m_groupSize as size_t).wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                                ),
                                5947 as ::core::ffi::c_int,
                            )
                                as *mut ::core::ffi::c_int;
                            if new_scaff_index.is_null() {
                                (*parser).m_groupSize =
                                    (*parser).m_groupSize.wrapping_div(2 as ::core::ffi::c_uint);
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*dtd).scaffIndex = new_scaff_index;
                        }
                    } else {
                        (*parser).m_groupSize = 32 as ::core::ffi::c_uint;
                        (*parser).m_groupConnector = expat_malloc(
                            parser,
                            (*parser).m_groupSize as size_t,
                            5955 as ::core::ffi::c_int,
                        )
                            as *mut ::core::ffi::c_char;
                        if (*parser).m_groupConnector.is_null() {
                            (*parser).m_groupSize = 0 as ::core::ffi::c_uint;
                            return XML_ERROR_NO_MEMORY;
                        }
                    }
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) = 0 as ::core::ffi::c_char;
                if (*dtd).in_eldecl != 0 {
                    let mut myindex: ::core::ffi::c_int = nextScaffoldPart(parser);
                    if myindex < 0 as ::core::ffi::c_int {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*dtd).scaffIndex.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    {
                        __assert_rtn(
                            b"doProlog\0" as *const u8 as *const ::core::ffi::c_char,
                            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                            5967 as ::core::ffi::c_int,
                            b"dtd->scaffIndex != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                    *(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) = myindex;
                    (*dtd).scaffLevel += 1;
                    (*(*dtd).scaffold.offset(myindex as isize)).type_0 = XML_CTYPE_SEQ;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            50 => {
                if *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize)
                    as ::core::ffi::c_int
                    == ASCII_PIPE
                {
                    return XML_ERROR_SYNTAX;
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) =
                    ASCII_COMMA as ::core::ffi::c_char;
                if (*dtd).in_eldecl as ::core::ffi::c_int != 0
                    && (*parser).m_elementDeclHandler.is_some()
                {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            49 => {
                if *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize)
                    as ::core::ffi::c_int
                    == ASCII_COMMA
                {
                    return XML_ERROR_SYNTAX;
                }
                if (*dtd).in_eldecl as ::core::ffi::c_int != 0
                    && *(*parser)
                        .m_groupConnector
                        .offset((*parser).m_prologState.level as isize)
                        == 0
                    && (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as ::core::ffi::c_int) as isize)
                            as isize,
                    ))
                    .type_0 as ::core::ffi::c_uint
                        != XML_CTYPE_MIXED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as ::core::ffi::c_int) as isize)
                            as isize,
                    ))
                    .type_0 = XML_CTYPE_CHOICE;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) =
                    ASCII_PIPE as ::core::ffi::c_char;
                current_block = 8258632986558375165;
            }
            60 | 59 => {
                (*dtd).hasParamEntityRefs = XML_TRUE;
                if (*parser).m_paramEntityParsing as u64 == 0 {
                    (*dtd).keepProcessing = (*dtd).standalone;
                    current_block = 16953886395775657100;
                } else {
                    let mut name_1: *const XML_Char = ::core::ptr::null::<XML_Char>();
                    let mut entity_1: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                    name_1 = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name_1.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    entity_1 = lookup(
                        parser,
                        &raw mut (*dtd).paramEntities,
                        name_1 as KEY,
                        0 as size_t,
                    ) as *mut ENTITY;
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    if (*parser).m_prologState.documentEntity != 0
                        && (if (*dtd).standalone as ::core::ffi::c_int != 0 {
                            (*parser).m_openInternalEntities.is_null() as ::core::ffi::c_int
                        } else {
                            ((*dtd).hasParamEntityRefs == 0) as ::core::ffi::c_int
                        }) != 0
                    {
                        if entity_1.is_null() {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity_1).is_internal == 0 {
                            return XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        current_block = 11938645146649090955;
                    } else if entity_1.is_null() {
                        (*dtd).keepProcessing = (*dtd).standalone;
                        if role == XML_ROLE_PARAM_ENTITY_REF as ::core::ffi::c_int
                            && (*parser).m_skippedEntityHandler.is_some()
                        {
                            (*parser)
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                name_1,
                                1 as ::core::ffi::c_int,
                            );
                            handleDefault = XML_FALSE;
                        }
                        current_block = 8258632986558375165;
                    } else {
                        current_block = 11938645146649090955;
                    }
                    match current_block {
                        8258632986558375165 => {}
                        _ => {
                            if (*entity_1).open != 0 {
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity_1).textPtr.is_null() {
                                let mut result_4: XML_Error = XML_ERROR_NONE;
                                let mut betweenDecl: XML_Bool =
                                    (if role == XML_ROLE_PARAM_ENTITY_REF as ::core::ffi::c_int {
                                        XML_TRUE as ::core::ffi::c_int
                                    } else {
                                        XML_FALSE as ::core::ffi::c_int
                                    }) as XML_Bool;
                                result_4 =
                                    processEntity(parser, entity_1, betweenDecl, ENTITY_INTERNAL);
                                if result_4 as ::core::ffi::c_uint
                                    != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    return result_4;
                                }
                                handleDefault = XML_FALSE;
                                current_block = 8258632986558375165;
                            } else if (*parser).m_externalEntityRefHandler.is_some() {
                                (*dtd).paramEntityRead = XML_FALSE;
                                (*entity_1).open = XML_TRUE;
                                entityTrackingOnOpen(parser, entity_1, 6068 as ::core::ffi::c_int);
                                if (*parser)
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_externalEntityRefHandlerArg,
                                    ::core::ptr::null::<XML_Char>(),
                                    (*entity_1).base,
                                    (*entity_1).systemId,
                                    (*entity_1).publicId,
                                ) == 0
                                {
                                    entityTrackingOnClose(
                                        parser,
                                        entity_1,
                                        6072 as ::core::ffi::c_int,
                                    );
                                    (*entity_1).open = XML_FALSE;
                                    return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                entityTrackingOnClose(parser, entity_1, 6076 as ::core::ffi::c_int);
                                (*entity_1).open = XML_FALSE;
                                handleDefault = XML_FALSE;
                                if (*dtd).paramEntityRead == 0 {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                    current_block = 8258632986558375165;
                                } else {
                                    current_block = 16953886395775657100;
                                }
                            } else {
                                (*dtd).keepProcessing = (*dtd).standalone;
                                current_block = 8258632986558375165;
                            }
                        }
                    }
                }
                match current_block {
                    8258632986558375165 => {}
                    _ => {
                        if (*dtd).standalone == 0
                            && (*parser).m_notStandaloneHandler.is_some()
                            && (*parser)
                                .m_notStandaloneHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg
                            ) == 0
                        {
                            return XML_ERROR_NOT_STANDALONE;
                        }
                        current_block = 8258632986558375165;
                    }
                }
            }
            40 => {
                if (*parser).m_elementDeclHandler.is_some() {
                    (*parser).m_declElementType = getElementType(parser, enc, s, next);
                    if (*parser).m_declElementType.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).scaffLevel = 0 as ::core::ffi::c_int;
                    (*dtd).scaffCount = 0 as ::core::ffi::c_uint;
                    (*dtd).in_eldecl = XML_TRUE;
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            41 | 42 => {
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler.is_some() {
                        let mut content: *mut XML_Content = (*parser)
                            .m_mem
                            .malloc_fcn
                            .expect("non-null function pointer")(
                            ::core::mem::size_of::<XML_Content>() as size_t,
                        )
                            as *mut XML_Content;
                        if content.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*content).quant = XML_CQUANT_NONE;
                        (*content).name = ::core::ptr::null_mut::<XML_Char>();
                        (*content).numchildren = 0 as ::core::ffi::c_uint;
                        (*content).children = ::core::ptr::null_mut::<XML_Content>();
                        (*content).type_0 = (if role == XML_ROLE_CONTENT_ANY as ::core::ffi::c_int {
                            XML_CTYPE_ANY as ::core::ffi::c_int
                        } else {
                            XML_CTYPE_EMPTY as ::core::ffi::c_int
                        }) as XML_Content_Type;
                        *eventEndPP = s;
                        (*parser)
                            .m_elementDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            content,
                        );
                        handleDefault = XML_FALSE;
                    }
                    (*dtd).in_eldecl = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            43 => {
                if (*dtd).in_eldecl != 0 {
                    (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as ::core::ffi::c_int) as isize)
                            as isize,
                    ))
                    .type_0 = XML_CTYPE_MIXED;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            51 => {
                quant = XML_CQUANT_NONE;
                current_block = 8842923054493027603;
            }
            53 => {
                quant = XML_CQUANT_OPT;
                current_block = 8842923054493027603;
            }
            52 => {
                quant = XML_CQUANT_REP;
                current_block = 8842923054493027603;
            }
            54 => {
                quant = XML_CQUANT_PLUS;
                current_block = 8842923054493027603;
            }
            45 => {
                quant = XML_CQUANT_NONE;
                current_block = 7905515844323611487;
            }
            47 => {
                quant = XML_CQUANT_OPT;
                current_block = 7905515844323611487;
            }
            46 => {
                quant = XML_CQUANT_REP;
                current_block = 7905515844323611487;
            }
            48 => {
                quant = XML_CQUANT_PLUS;
                current_block = 7905515844323611487;
            }
            55 => {
                if reportProcessingInstruction(parser, enc, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
                handleDefault = XML_FALSE;
                current_block = 8258632986558375165;
            }
            56 => {
                if reportComment(parser, enc, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
                handleDefault = XML_FALSE;
                current_block = 8258632986558375165;
            }
            0 => {
                match tok {
                    XML_TOK_BOM => {
                        handleDefault = XML_FALSE;
                    }
                    _ => {}
                }
                current_block = 8258632986558375165;
            }
            3 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            11 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && (*parser).m_entityDeclHandler.is_some()
                {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            17 => {
                if (*parser).m_notationDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            33 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            39 => {
                if (*parser).m_elementDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            _ => {
                current_block = 8258632986558375165;
            }
        }
        match current_block {
            13984953578200811817 => {
                if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP) == 0
                {
                    return XML_ERROR_PUBLICID;
                }
                current_block = 14529472648802745661;
            }
            1224196737208379537 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                {
                    (*(*parser).m_declEntity).systemId = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if (*(*parser).m_declEntity).systemId.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*(*parser).m_declEntity).base = (*parser).m_curBase;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_entityDeclHandler.is_some()
                        && role == XML_ROLE_ENTITY_SYSTEM_ID as ::core::ffi::c_int
                    {
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            14544254959461281264 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            8842923054493027603 => {
                if (*dtd).in_eldecl != 0 {
                    let mut el: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
                    let mut name_2: *const XML_Char = ::core::ptr::null::<XML_Char>();
                    let mut nameLen: size_t = 0;
                    let mut nxt: *const ::core::ffi::c_char = if quant as ::core::ffi::c_uint
                        == XML_CQUANT_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        next
                    } else {
                        next.offset(-((*enc).minBytesPerChar as isize))
                    };
                    let mut myindex_0: ::core::ffi::c_int = nextScaffoldPart(parser);
                    if myindex_0 < 0 as ::core::ffi::c_int {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*(*dtd).scaffold.offset(myindex_0 as isize)).type_0 = XML_CTYPE_NAME;
                    (*(*dtd).scaffold.offset(myindex_0 as isize)).quant = quant;
                    el = getElementType(parser, enc, s, nxt);
                    if el.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    name_2 = (*el).name;
                    let ref mut fresh5 = (*(*dtd).scaffold.offset(myindex_0 as isize)).name;
                    *fresh5 = name_2;
                    nameLen = 0 as size_t;
                    loop {
                        let fresh6 = nameLen;
                        nameLen = nameLen.wrapping_add(1);
                        if !(*name_2.offset(fresh6 as isize) != 0) {
                            break;
                        }
                    }
                    if nameLen > UINT_MAX.wrapping_sub((*dtd).contentStringLen) as size_t {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).contentStringLen = (*dtd)
                        .contentStringLen
                        .wrapping_add(nameLen as ::core::ffi::c_uint);
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            7905515844323611487 => {
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                    (*dtd).scaffLevel -= 1;
                    (*(*dtd)
                        .scaffold
                        .offset(*(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) as isize))
                    .quant = quant;
                    if (*dtd).scaffLevel == 0 as ::core::ffi::c_int {
                        if handleDefault == 0 {
                            let mut model: *mut XML_Content = build_model(parser);
                            if model.is_null() {
                                return XML_ERROR_NO_MEMORY;
                            }
                            *eventEndPP = s;
                            (*parser)
                                .m_elementDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*parser).m_declElementType).name,
                                model,
                            );
                        }
                        (*dtd).in_eldecl = XML_FALSE;
                        (*dtd).contentStringLen = 0 as ::core::ffi::c_uint;
                    }
                }
                current_block = 8258632986558375165;
            }
            _ => {}
        }
        match current_block {
            14529472648802745661 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                {
                    let mut tem: *mut XML_Char = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if tem.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(tem);
                    (*(*parser).m_declEntity).publicId = tem;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_entityDeclHandler.is_some()
                        && role == XML_ROLE_ENTITY_PUBLIC_ID as ::core::ffi::c_int
                    {
                        handleDefault = XML_FALSE;
                    }
                }
            }
            _ => {}
        }
        if handleDefault as ::core::ffi::c_int != 0 && (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, s, next);
        }
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => return XML_ERROR_ABORTED,
            1 => {
                if (*parser).m_reenter != 0 {
                    *nextPtr = next;
                    return XML_ERROR_NONE;
                }
            }
            _ => {}
        }
        s = next;
        tok = (*enc).scanners[0 as ::core::ffi::c_int as usize].expect("non-null function pointer")(
            enc,
            s,
            end,
            &raw mut next,
        );
    }
}
unsafe extern "C" fn epilogProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    (*parser).m_processor = Some(
        epilogProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> XML_Error,
    );
    (*parser).m_eventPtr = s;
    loop {
        let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut tok: ::core::ffi::c_int = (*(*parser).m_encoding).scanners
            [0 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, s, end, &raw mut next
        );
        if accountingDiffTolerated(
            parser,
            tok,
            s,
            next,
            6290 as ::core::ffi::c_int,
            XML_ACCOUNT_DIRECT,
        ) == 0
        {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        (*parser).m_eventEndPtr = next;
        match tok {
            -15 => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, (*parser).m_encoding, s, next);
                    if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                        == XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        return XML_ERROR_ABORTED;
                    }
                }
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            XML_TOK_NONE => {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            XML_TOK_PROLOG_S => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, (*parser).m_encoding, s, next);
                }
            }
            XML_TOK_PI => {
                if reportProcessingInstruction(parser, (*parser).m_encoding, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            XML_TOK_COMMENT => {
                if reportComment(parser, (*parser).m_encoding, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            XML_TOK_INVALID => {
                (*parser).m_eventPtr = next;
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_TOKEN;
            }
            XML_TOK_PARTIAL_CHAR => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            _ => return XML_ERROR_JUNK_AFTER_DOC_ELEMENT,
        }
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                (*parser).m_eventPtr = next;
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => {
                (*parser).m_eventPtr = next;
                return XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            _ => {}
        }
        s = next;
        (*parser).m_eventPtr = s;
    }
}
unsafe extern "C" fn processEntity(
    mut parser: XML_Parser,
    mut entity: *mut ENTITY,
    mut betweenDecl: XML_Bool,
    mut type_0: EntityType,
) -> XML_Error {
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    let mut openEntityList: *mut *mut OPEN_INTERNAL_ENTITY =
        ::core::ptr::null_mut::<*mut OPEN_INTERNAL_ENTITY>();
    let mut freeEntityList: *mut *mut OPEN_INTERNAL_ENTITY =
        ::core::ptr::null_mut::<*mut OPEN_INTERNAL_ENTITY>();
    match type_0 as ::core::ffi::c_uint {
        0 => {
            (*parser).m_processor = Some(
                internalEntityProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> XML_Error,
            );
            openEntityList = &raw mut (*parser).m_openInternalEntities;
            freeEntityList = &raw mut (*parser).m_freeInternalEntities;
        }
        1 => {
            openEntityList = &raw mut (*parser).m_openAttributeEntities;
            freeEntityList = &raw mut (*parser).m_freeAttributeEntities;
        }
        2 => {
            openEntityList = &raw mut (*parser).m_openValueEntities;
            freeEntityList = &raw mut (*parser).m_freeValueEntities;
        }
        _ => {
            if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"processEntity\0" as *const u8 as *const ::core::ffi::c_char,
                    b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                    6385 as ::core::ffi::c_int,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
    }
    if !(*freeEntityList).is_null() {
        openEntity = *freeEntityList;
        *freeEntityList = (*openEntity).next as *mut OPEN_INTERNAL_ENTITY;
    } else {
        openEntity = expat_malloc(
            parser,
            ::core::mem::size_of::<OPEN_INTERNAL_ENTITY>() as size_t,
            6393 as ::core::ffi::c_int,
        ) as *mut OPEN_INTERNAL_ENTITY;
        if openEntity.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
    }
    (*entity).open = XML_TRUE;
    (*entity).hasMore = XML_TRUE;
    entityTrackingOnOpen(parser, entity, 6400 as ::core::ffi::c_int);
    (*entity).processed = 0 as ::core::ffi::c_int;
    (*openEntity).next = *openEntityList as *mut open_internal_entity;
    *openEntityList = openEntity;
    (*openEntity).entity = entity;
    (*openEntity).type_0 = type_0;
    (*openEntity).startTagLevel = (*parser).m_tagLevel;
    (*openEntity).betweenDecl = betweenDecl;
    (*openEntity).internalEventPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*openEntity).internalEventEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
    if type_0 as ::core::ffi::c_uint == ENTITY_INTERNAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        triggerReenter(parser);
    }
    return XML_ERROR_NONE;
}
unsafe extern "C" fn internalEntityProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
    let mut textStart: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut textEnd: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut result: XML_Error = XML_ERROR_NONE;
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openInternalEntities;
    if openEntity.is_null() {
        return XML_ERROR_UNEXPECTED_STATE;
    }
    entity = (*openEntity).entity;
    if (*entity).hasMore != 0 {
        textStart =
            ((*entity).textPtr as *const ::core::ffi::c_char).offset((*entity).processed as isize);
        textEnd =
            (*entity).textPtr.offset((*entity).textLen as isize) as *const ::core::ffi::c_char;
        next = textStart;
        if (*entity).is_param != 0 {
            let mut tok: ::core::ffi::c_int = (*(*parser).m_internalEncoding).scanners
                [0 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*parser).m_internalEncoding,
                textStart,
                textEnd,
                &raw mut next,
            );
            result = doProlog(
                parser,
                (*parser).m_internalEncoding,
                textStart,
                textEnd,
                tok,
                next,
                &raw mut next,
                XML_FALSE,
                XML_FALSE,
                XML_ACCOUNT_ENTITY_EXPANSION,
            );
        } else {
            result = doContent(
                parser,
                (*openEntity).startTagLevel,
                (*parser).m_internalEncoding,
                textStart,
                textEnd,
                &raw mut next,
                XML_FALSE,
                XML_ACCOUNT_ENTITY_EXPANSION,
            );
        }
        if result as ::core::ffi::c_uint
            != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return result;
        }
        if textEnd != next
            && ((*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                == XML_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                    == XML_PARSING as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*parser).m_reenter as ::core::ffi::c_int != 0)
        {
            (*entity).processed = next.offset_from((*entity).textPtr as *const ::core::ffi::c_char)
                as ::core::ffi::c_long as ::core::ffi::c_int;
            return result;
        }
        (*entity).hasMore = XML_FALSE;
        if (*entity).is_param == 0 && (*openEntity).startTagLevel != (*parser).m_tagLevel {
            return XML_ERROR_ASYNC_ENTITY;
        }
        triggerReenter(parser);
        return result;
    }
    entityTrackingOnClose(parser, entity, 6481 as ::core::ffi::c_int);
    if !((*parser).m_openInternalEntities == openEntity) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"internalEntityProcessor\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            6487 as ::core::ffi::c_int,
            b"parser->m_openInternalEntities == openEntity\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*entity).open = XML_FALSE;
    (*parser).m_openInternalEntities =
        (*(*parser).m_openInternalEntities).next as *mut OPEN_INTERNAL_ENTITY;
    (*openEntity).next = (*parser).m_freeInternalEntities as *mut open_internal_entity;
    (*parser).m_freeInternalEntities = openEntity;
    if (*parser).m_openInternalEntities.is_null() {
        (*parser).m_processor = if (*entity).is_param as ::core::ffi::c_int != 0 {
            Some(
                prologProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> XML_Error,
            )
        } else {
            Some(
                contentProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> XML_Error,
            )
        };
    }
    triggerReenter(parser);
    return XML_ERROR_NONE;
}
unsafe extern "C" fn errorProcessor(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    return (*parser).m_errorCode;
}
unsafe extern "C" fn storeAttributeValue(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut isCdata: XML_Bool,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut pool: *mut STRING_POOL,
    mut account: XML_Account,
) -> XML_Error {
    let mut next: *const ::core::ffi::c_char = ptr;
    let mut result: XML_Error = XML_ERROR_NONE;
    loop {
        if (*parser).m_openAttributeEntities.is_null() {
            result = appendAttributeValue(
                parser,
                enc,
                isCdata,
                next,
                end,
                pool,
                account,
                &raw mut next,
            );
        } else {
            let openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openAttributeEntities;
            if openEntity.is_null() {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            let entity: *mut ENTITY = (*openEntity).entity;
            let textStart: *const ::core::ffi::c_char = ((*entity).textPtr
                as *const ::core::ffi::c_char)
                .offset((*entity).processed as isize);
            let textEnd: *const ::core::ffi::c_char =
                (*entity).textPtr.offset((*entity).textLen as isize) as *const ::core::ffi::c_char;
            let mut nextInEntity: *const ::core::ffi::c_char = textStart;
            if (*entity).hasMore != 0 {
                result = appendAttributeValue(
                    parser,
                    (*parser).m_internalEncoding,
                    isCdata,
                    textStart,
                    textEnd,
                    pool,
                    XML_ACCOUNT_ENTITY_EXPANSION,
                    &raw mut nextInEntity,
                );
                if result as ::core::ffi::c_uint
                    != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    break;
                }
                if textEnd != nextInEntity {
                    (*entity).processed =
                        nextInEntity.offset_from((*entity).textPtr as *const ::core::ffi::c_char)
                            as ::core::ffi::c_long as ::core::ffi::c_int;
                    continue;
                } else {
                    (*entity).hasMore = XML_FALSE;
                    continue;
                }
            } else {
                entityTrackingOnClose(parser, entity, 6558 as ::core::ffi::c_int);
                if !((*parser).m_openAttributeEntities == openEntity) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    __assert_rtn(
                        b"storeAttributeValue\0" as *const u8 as *const ::core::ffi::c_char,
                        b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                        6564 as ::core::ffi::c_int,
                        b"parser->m_openAttributeEntities == openEntity\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                } else {
                };
                (*entity).open = XML_FALSE;
                (*parser).m_openAttributeEntities =
                    (*(*parser).m_openAttributeEntities).next as *mut OPEN_INTERNAL_ENTITY;
                (*openEntity).next = (*parser).m_freeAttributeEntities as *mut open_internal_entity;
                (*parser).m_freeAttributeEntities = openEntity;
            }
        }
        if result as ::core::ffi::c_uint != 0
            || (*parser).m_openAttributeEntities.is_null() && end == next
        {
            break;
        }
    }
    if result as u64 != 0 {
        return result;
    }
    if isCdata == 0
        && (*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long != 0
        && *(*pool).ptr.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == 0x20 as ::core::ffi::c_int
    {
        (*pool).ptr = (*pool).ptr.offset(-1);
    }
    if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
        0 as ::core::ffi::c_int
    } else {
        let fresh55 = (*pool).ptr;
        (*pool).ptr = (*pool).ptr.offset(1);
        *fresh55 = '\0' as i32 as XML_Char;
        1 as ::core::ffi::c_int
    } == 0
    {
        return XML_ERROR_NO_MEMORY;
    }
    return XML_ERROR_NONE;
}
unsafe extern "C" fn appendAttributeValue(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut isCdata: XML_Bool,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut pool: *mut STRING_POOL,
    mut account: XML_Account,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    loop {
        let mut next: *const ::core::ffi::c_char = ptr;
        let mut tok: ::core::ffi::c_int =
            (*enc).literalScanners[0 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(enc, ptr, end, &raw mut next);
        if accountingDiffTolerated(parser, tok, ptr, next, 6602 as ::core::ffi::c_int, account) == 0
        {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        let mut current_block_70: u64;
        match tok {
            XML_TOK_NONE => {
                if !nextPtr.is_null() {
                    *nextPtr = next;
                }
                return XML_ERROR_NONE;
            }
            XML_TOK_INVALID => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = next;
                }
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = ptr;
                }
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_CHAR_REF => {
                let mut buf: [XML_Char; 4] = [0; 4];
                let mut i: ::core::ffi::c_int = 0;
                let mut n: ::core::ffi::c_int =
                    (*enc).charRefNumber.expect("non-null function pointer")(enc, ptr);
                if n < 0 as ::core::ffi::c_int {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = ptr;
                    }
                    return XML_ERROR_BAD_CHAR_REF;
                }
                if isCdata == 0
                    && n == 0x20 as ::core::ffi::c_int
                    && ((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long
                        == 0 as ::core::ffi::c_long
                        || *(*pool).ptr.offset(-(1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == 0x20 as ::core::ffi::c_int)
                {
                    current_block_70 = 18038362259723567392;
                } else {
                    n = XmlUtf8Encode(n, &raw mut buf as *mut XML_Char as *mut ::core::ffi::c_char);
                    i = 0 as ::core::ffi::c_int;
                    while i < n {
                        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
                            0 as ::core::ffi::c_int
                        } else {
                            let fresh56 = (*pool).ptr;
                            (*pool).ptr = (*pool).ptr.offset(1);
                            *fresh56 = buf[i as usize];
                            1 as ::core::ffi::c_int
                        } == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        i += 1;
                    }
                    current_block_70 = 18038362259723567392;
                }
            }
            XML_TOK_DATA_CHARS => {
                if poolAppend(pool, enc, ptr, next).is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                current_block_70 = 18038362259723567392;
            }
            XML_TOK_TRAILING_CR => {
                next = ptr.offset((*enc).minBytesPerChar as isize);
                current_block_70 = 14833121763727876827;
            }
            XML_TOK_ATTRIBUTE_VALUE_S | XML_TOK_DATA_NEWLINE => {
                current_block_70 = 14833121763727876827;
            }
            XML_TOK_ENTITY_REF => {
                let mut name: *const XML_Char = ::core::ptr::null::<XML_Char>();
                let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                let mut checkEntityDecl: bool = false;
                let mut ch: XML_Char = (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(
                    enc,
                    ptr.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                ) as XML_Char;
                if ch != 0 {
                    accountingDiffTolerated(
                        parser,
                        tok,
                        &raw mut ch as *mut ::core::ffi::c_char,
                        (&raw mut ch as *mut ::core::ffi::c_char).offset(::core::mem::size_of::<
                            XML_Char,
                        >()
                            as usize
                            as isize),
                        6674 as ::core::ffi::c_int,
                        XML_ACCOUNT_ENTITY_EXPANSION,
                    );
                    if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        let fresh58 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh58 = ch;
                        1 as ::core::ffi::c_int
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                } else {
                    name = poolStoreString(
                        &raw mut (*parser).m_temp2Pool,
                        enc,
                        ptr.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    entity = lookup(
                        parser,
                        &raw mut (*dtd).generalEntities,
                        name as KEY,
                        0 as size_t,
                    ) as *mut ENTITY;
                    (*parser).m_temp2Pool.ptr = (*parser).m_temp2Pool.start;
                    if pool == &raw mut (*dtd).pool {
                        checkEntityDecl = (*parser).m_prologState.documentEntity != 0
                            && (if (*dtd).standalone as ::core::ffi::c_int != 0 {
                                (*parser).m_openInternalEntities.is_null() as ::core::ffi::c_int
                            } else {
                                ((*dtd).hasParamEntityRefs == 0) as ::core::ffi::c_int
                            }) != 0;
                    } else {
                        checkEntityDecl = (*dtd).hasParamEntityRefs == 0
                            || (*dtd).standalone as ::core::ffi::c_int != 0;
                    }
                    if checkEntityDecl {
                        if entity.is_null() {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity).is_internal == 0 {
                            return XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        current_block_70 = 13678349939556791712;
                    } else if entity.is_null() {
                        current_block_70 = 18038362259723567392;
                    } else {
                        current_block_70 = 13678349939556791712;
                    }
                    match current_block_70 {
                        18038362259723567392 => {}
                        _ => {
                            if (*entity).open != 0 {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if (*entity).textPtr.is_null() {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
                            } else {
                                let mut result: XML_Error = XML_ERROR_NONE;
                                result = processEntity(parser, entity, XML_FALSE, ENTITY_ATTRIBUTE);
                                if result as ::core::ffi::c_uint
                                    == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                                    && !nextPtr.is_null()
                                {
                                    *nextPtr = next;
                                }
                                return result;
                            }
                        }
                    }
                }
                current_block_70 = 18038362259723567392;
            }
            _ => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = ptr;
                }
                return XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match current_block_70 {
            14833121763727876827 => {
                if !(isCdata == 0
                    && ((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long
                        == 0 as ::core::ffi::c_long
                        || *(*pool).ptr.offset(-(1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == 0x20 as ::core::ffi::c_int))
                {
                    if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        let fresh57 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh57 = 0x20 as XML_Char;
                        1 as ::core::ffi::c_int
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                }
            }
            _ => {}
        }
        ptr = next;
    }
}
unsafe extern "C" fn storeEntityValue(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut entityTextPtr: *const ::core::ffi::c_char,
    mut entityTextEnd: *const ::core::ffi::c_char,
    mut account: XML_Account,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> XML_Error {
    let mut current_block: u64;
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut pool: *mut STRING_POOL = &raw mut (*dtd).entityValuePool;
    let mut result: XML_Error = XML_ERROR_NONE;
    let mut oldInEntityValue: ::core::ffi::c_int = (*parser).m_prologState.inEntityValue;
    (*parser).m_prologState.inEntityValue = 1 as ::core::ffi::c_int;
    if (*pool).blocks.is_null() {
        if poolGrow(pool) == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    let mut next: *const ::core::ffi::c_char = entityTextPtr;
    if entityTextPtr >= entityTextEnd {
        result = XML_ERROR_NONE;
    } else {
        's_45: loop {
            next = entityTextPtr;
            let mut tok: ::core::ffi::c_int = (*enc).literalScanners
                [1 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                enc,
                entityTextPtr,
                entityTextEnd,
                &raw mut next,
            );
            if accountingDiffTolerated(
                parser,
                tok,
                entityTextPtr,
                next,
                6816 as ::core::ffi::c_int,
                account,
            ) == 0
            {
                accountingOnAbort(parser);
                result = XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
                break;
            } else {
                match tok {
                    XML_TOK_PARAM_ENTITY_REF => {
                        if (*parser).m_isParamEntity as ::core::ffi::c_int != 0
                            || enc != (*parser).m_encoding
                        {
                            let mut name: *const XML_Char = ::core::ptr::null::<XML_Char>();
                            let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                            name = poolStoreString(
                                &raw mut (*parser).m_tempPool,
                                enc,
                                entityTextPtr.offset((*enc).minBytesPerChar as isize),
                                next.offset(-((*enc).minBytesPerChar as isize)),
                            );
                            if name.is_null() {
                                result = XML_ERROR_NO_MEMORY;
                                break;
                            } else {
                                entity = lookup(
                                    parser,
                                    &raw mut (*dtd).paramEntities,
                                    name as KEY,
                                    0 as size_t,
                                ) as *mut ENTITY;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
                                if entity.is_null() {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                    break;
                                } else if (*entity).open as ::core::ffi::c_int != 0
                                    || entity == (*parser).m_declEntity
                                {
                                    if enc == (*parser).m_encoding {
                                        (*parser).m_eventPtr = entityTextPtr;
                                    }
                                    result = XML_ERROR_RECURSIVE_ENTITY_REF;
                                    break;
                                } else if !(*entity).systemId.is_null() {
                                    if (*parser).m_externalEntityRefHandler.is_some() {
                                        (*dtd).paramEntityRead = XML_FALSE;
                                        (*entity).open = XML_TRUE;
                                        entityTrackingOnOpen(
                                            parser,
                                            entity,
                                            6858 as ::core::ffi::c_int,
                                        );
                                        if (*parser)
                                            .m_externalEntityRefHandler
                                            .expect("non-null function pointer")(
                                            (*parser).m_externalEntityRefHandlerArg,
                                            ::core::ptr::null::<XML_Char>(),
                                            (*entity).base,
                                            (*entity).systemId,
                                            (*entity).publicId,
                                        ) == 0
                                        {
                                            entityTrackingOnClose(
                                                parser,
                                                entity,
                                                6862 as ::core::ffi::c_int,
                                            );
                                            (*entity).open = XML_FALSE;
                                            result = XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                            break;
                                        } else {
                                            entityTrackingOnClose(
                                                parser,
                                                entity,
                                                6867 as ::core::ffi::c_int,
                                            );
                                            (*entity).open = XML_FALSE;
                                            if (*dtd).paramEntityRead == 0 {
                                                (*dtd).keepProcessing = (*dtd).standalone;
                                            }
                                        }
                                    } else {
                                        (*dtd).keepProcessing = (*dtd).standalone;
                                    }
                                } else {
                                    result = processEntity(parser, entity, XML_FALSE, ENTITY_VALUE);
                                    break;
                                }
                            }
                        } else {
                            (*parser).m_eventPtr = entityTextPtr;
                            result = XML_ERROR_PARAM_ENTITY_REF;
                            break;
                        }
                        current_block = 18038362259723567392;
                    }
                    XML_TOK_NONE => {
                        result = XML_ERROR_NONE;
                        break;
                    }
                    XML_TOK_ENTITY_REF | XML_TOK_DATA_CHARS => {
                        if poolAppend(pool, enc, entityTextPtr, next).is_null() {
                            result = XML_ERROR_NO_MEMORY;
                            break;
                        } else {
                            current_block = 18038362259723567392;
                        }
                    }
                    XML_TOK_TRAILING_CR => {
                        next = entityTextPtr.offset((*enc).minBytesPerChar as isize);
                        current_block = 15972096214037155883;
                    }
                    XML_TOK_DATA_NEWLINE => {
                        current_block = 15972096214037155883;
                    }
                    XML_TOK_CHAR_REF => {
                        let mut buf: [XML_Char; 4] = [0; 4];
                        let mut i: ::core::ffi::c_int = 0;
                        let mut n: ::core::ffi::c_int = (*enc)
                            .charRefNumber
                            .expect("non-null function pointer")(
                            enc, entityTextPtr
                        );
                        if n < 0 as ::core::ffi::c_int {
                            if enc == (*parser).m_encoding {
                                (*parser).m_eventPtr = entityTextPtr;
                            }
                            result = XML_ERROR_BAD_CHAR_REF;
                            break;
                        } else {
                            n = XmlUtf8Encode(
                                n,
                                &raw mut buf as *mut XML_Char as *mut ::core::ffi::c_char,
                            );
                            i = 0 as ::core::ffi::c_int;
                            while i < n {
                                if (*pool).end == (*pool).ptr as *const XML_Char
                                    && poolGrow(pool) == 0
                                {
                                    result = XML_ERROR_NO_MEMORY;
                                    break 's_45;
                                } else {
                                    let fresh73 = (*pool).ptr;
                                    (*pool).ptr = (*pool).ptr.offset(1);
                                    *fresh73 = buf[i as usize];
                                    i += 1;
                                }
                            }
                        }
                        current_block = 18038362259723567392;
                    }
                    XML_TOK_PARTIAL => {
                        if enc == (*parser).m_encoding {
                            (*parser).m_eventPtr = entityTextPtr;
                        }
                        result = XML_ERROR_INVALID_TOKEN;
                        break;
                    }
                    XML_TOK_INVALID => {
                        if enc == (*parser).m_encoding {
                            (*parser).m_eventPtr = next;
                        }
                        result = XML_ERROR_INVALID_TOKEN;
                        break;
                    }
                    _ => {
                        if enc == (*parser).m_encoding {
                            (*parser).m_eventPtr = entityTextPtr;
                        }
                        result = XML_ERROR_UNEXPECTED_STATE;
                        break;
                    }
                }
                match current_block {
                    15972096214037155883 => {
                        if (*pool).end == (*pool).ptr as *const XML_Char && poolGrow(pool) == 0 {
                            result = XML_ERROR_NO_MEMORY;
                            break;
                        } else {
                            let fresh72 = (*pool).ptr;
                            (*pool).ptr = (*pool).ptr.offset(1);
                            *fresh72 = 0xa as XML_Char;
                        }
                    }
                    _ => {}
                }
                entityTextPtr = next;
            }
        }
    }
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    if !nextPtr.is_null() {
        *nextPtr = next;
    }
    return result;
}
unsafe extern "C" fn callStoreEntityValue(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut entityTextPtr: *const ::core::ffi::c_char,
    mut entityTextEnd: *const ::core::ffi::c_char,
    mut account: XML_Account,
) -> XML_Error {
    let mut next: *const ::core::ffi::c_char = entityTextPtr;
    let mut result: XML_Error = XML_ERROR_NONE;
    loop {
        if (*parser).m_openValueEntities.is_null() {
            result = storeEntityValue(parser, enc, next, entityTextEnd, account, &raw mut next);
        } else {
            let openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openValueEntities;
            if openEntity.is_null() {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            let entity: *mut ENTITY = (*openEntity).entity;
            let textStart: *const ::core::ffi::c_char = ((*entity).textPtr
                as *const ::core::ffi::c_char)
                .offset((*entity).processed as isize);
            let textEnd: *const ::core::ffi::c_char =
                (*entity).textPtr.offset((*entity).textLen as isize) as *const ::core::ffi::c_char;
            let mut nextInEntity: *const ::core::ffi::c_char = textStart;
            if (*entity).hasMore != 0 {
                result = storeEntityValue(
                    parser,
                    (*parser).m_internalEncoding,
                    textStart,
                    textEnd,
                    XML_ACCOUNT_ENTITY_EXPANSION,
                    &raw mut nextInEntity,
                );
                if result as ::core::ffi::c_uint
                    != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    break;
                }
                if textEnd != nextInEntity {
                    (*entity).processed =
                        nextInEntity.offset_from((*entity).textPtr as *const ::core::ffi::c_char)
                            as ::core::ffi::c_long as ::core::ffi::c_int;
                    continue;
                } else {
                    (*entity).hasMore = XML_FALSE;
                    continue;
                }
            } else {
                entityTrackingOnClose(parser, entity, 7016 as ::core::ffi::c_int);
                if !((*parser).m_openValueEntities == openEntity) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    __assert_rtn(
                        b"callStoreEntityValue\0" as *const u8 as *const ::core::ffi::c_char,
                        b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                        7022 as ::core::ffi::c_int,
                        b"parser->m_openValueEntities == openEntity\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                } else {
                };
                (*entity).open = XML_FALSE;
                (*parser).m_openValueEntities =
                    (*(*parser).m_openValueEntities).next as *mut OPEN_INTERNAL_ENTITY;
                (*openEntity).next = (*parser).m_freeValueEntities as *mut open_internal_entity;
                (*parser).m_freeValueEntities = openEntity;
            }
        }
        if result as ::core::ffi::c_uint != 0
            || (*parser).m_openValueEntities.is_null() && entityTextEnd == next
        {
            break;
        }
    }
    return result;
}
unsafe extern "C" fn normalizeLines(mut s: *mut XML_Char) {
    let mut p: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    loop {
        if *s as ::core::ffi::c_int == '\0' as i32 {
            return;
        }
        if *s as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
            break;
        }
        s = s.offset(1);
    }
    p = s;
    loop {
        if *s as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = 0xa as XML_Char;
            s = s.offset(1);
            if *s as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                s = s.offset(1);
            }
        } else {
            let fresh8 = s;
            s = s.offset(1);
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = *fresh8;
        }
        if !(*s != 0) {
            break;
        }
    }
    *p = '\0' as i32 as XML_Char;
}
unsafe extern "C" fn reportProcessingInstruction(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut target: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut data: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    let mut tem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if (*parser).m_processingInstructionHandler.is_none() {
        if (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, start, end);
        }
        return 1 as ::core::ffi::c_int;
    }
    start = start.offset(((*enc).minBytesPerChar * 2 as ::core::ffi::c_int) as isize);
    tem = start.offset((*enc).nameLength.expect("non-null function pointer")(enc, start) as isize);
    target = poolStoreString(&raw mut (*parser).m_tempPool, enc, start, tem);
    if target.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
    data = poolStoreString(
        &raw mut (*parser).m_tempPool,
        enc,
        (*enc).skipS.expect("non-null function pointer")(enc, tem),
        end.offset(-(((*enc).minBytesPerChar * 2 as ::core::ffi::c_int) as isize)),
    );
    if data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    normalizeLines(data);
    (*parser)
        .m_processingInstructionHandler
        .expect("non-null function pointer")((*parser).m_handlerArg, target, data);
    poolClear(&raw mut (*parser).m_tempPool);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn reportComment(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut data: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    if (*parser).m_commentHandler.is_none() {
        if (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, start, end);
        }
        return 1 as ::core::ffi::c_int;
    }
    data = poolStoreString(
        &raw mut (*parser).m_tempPool,
        enc,
        start.offset(((*enc).minBytesPerChar * 4 as ::core::ffi::c_int) as isize),
        end.offset(-(((*enc).minBytesPerChar * 3 as ::core::ffi::c_int) as isize)),
    );
    if data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    normalizeLines(data);
    (*parser)
        .m_commentHandler
        .expect("non-null function pointer")((*parser).m_handlerArg, data);
    poolClear(&raw mut (*parser).m_tempPool);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn reportDefault(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) {
    if (*enc).isUtf8 == 0 {
        let mut convert_res: XML_Convert_Result = XML_CONVERT_COMPLETED;
        let mut eventPP: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut eventEndPP: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        if enc == (*parser).m_encoding {
            eventPP = &raw mut (*parser).m_eventPtr;
            eventEndPP = &raw mut (*parser).m_eventEndPtr;
        } else {
            eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
            eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
        }
        loop {
            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
            convert_res = (*enc).utf8Convert.expect("non-null function pointer")(
                enc,
                &raw mut s,
                end,
                &raw mut dataPtr,
                (*parser).m_dataBufEnd as *mut ICHAR,
            );
            *eventEndPP = s;
            (*parser)
                .m_defaultHandler
                .expect("non-null function pointer")(
                (*parser).m_handlerArg,
                (*parser).m_dataBuf,
                dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            *eventPP = s;
            if !(convert_res as ::core::ffi::c_uint
                != XML_CONVERT_COMPLETED as ::core::ffi::c_int as ::core::ffi::c_uint
                && convert_res as ::core::ffi::c_uint
                    != XML_CONVERT_INPUT_INCOMPLETE as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                break;
            }
        }
    } else {
        (*parser)
            .m_defaultHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            s as *const XML_Char,
            (end as *const XML_Char).offset_from(s as *const XML_Char) as ::core::ffi::c_long
                as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn defineAttribute(
    mut type_0: *mut ELEMENT_TYPE,
    mut attId: *mut ATTRIBUTE_ID,
    mut isCdata: XML_Bool,
    mut isId: XML_Bool,
    mut value: *const XML_Char,
    mut parser: XML_Parser,
) -> ::core::ffi::c_int {
    let mut att: *mut DEFAULT_ATTRIBUTE = ::core::ptr::null_mut::<DEFAULT_ATTRIBUTE>();
    if !value.is_null() || isId as ::core::ffi::c_int != 0 {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*type_0).nDefaultAtts {
            if attId == (*(*type_0).defaultAtts.offset(i as isize)).id as *mut ATTRIBUTE_ID {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if isId as ::core::ffi::c_int != 0 && (*type_0).idAtt.is_null() && (*attId).xmlns == 0 {
            (*type_0).idAtt = attId;
        }
    }
    if (*type_0).nDefaultAtts == (*type_0).allocDefaultAtts {
        if (*type_0).allocDefaultAtts == 0 as ::core::ffi::c_int {
            (*type_0).allocDefaultAtts = 8 as ::core::ffi::c_int;
            (*type_0).defaultAtts = expat_malloc(
                parser,
                ((*type_0).allocDefaultAtts as size_t)
                    .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>() as size_t),
                7200 as ::core::ffi::c_int,
            ) as *mut DEFAULT_ATTRIBUTE;
            if (*type_0).defaultAtts.is_null() {
                (*type_0).allocDefaultAtts = 0 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
        } else {
            let mut temp: *mut DEFAULT_ATTRIBUTE = ::core::ptr::null_mut::<DEFAULT_ATTRIBUTE>();
            if (*type_0).allocDefaultAtts > INT_MAX / 2 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            let mut count: ::core::ffi::c_int =
                (*type_0).allocDefaultAtts * 2 as ::core::ffi::c_int;
            temp = expat_realloc(
                parser,
                (*type_0).defaultAtts as *mut ::core::ffi::c_void,
                (count as size_t)
                    .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>() as size_t),
                7226 as ::core::ffi::c_int,
            ) as *mut DEFAULT_ATTRIBUTE;
            if temp.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            (*type_0).allocDefaultAtts = count;
            (*type_0).defaultAtts = temp;
        }
    }
    att = (*type_0)
        .defaultAtts
        .offset((*type_0).nDefaultAtts as isize);
    (*att).id = attId;
    (*att).value = value;
    (*att).isCdata = isCdata;
    if isCdata == 0 {
        (*attId).maybeTokenized = XML_TRUE;
    }
    (*type_0).nDefaultAtts += 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn setElementTypePrefix(
    mut parser: XML_Parser,
    mut elementType: *mut ELEMENT_TYPE,
) -> ::core::ffi::c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const XML_Char = ::core::ptr::null::<XML_Char>();
    name = (*elementType).name;
    while *name != 0 {
        if *name as ::core::ffi::c_int == 0x3a as ::core::ffi::c_int {
            let mut prefix: *mut PREFIX = ::core::ptr::null_mut::<PREFIX>();
            let mut s: *const XML_Char = ::core::ptr::null::<XML_Char>();
            s = (*elementType).name;
            while s != name {
                if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                    && poolGrow(&raw mut (*dtd).pool) == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    let fresh15 = (*dtd).pool.ptr;
                    (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                    *fresh15 = *s;
                    1 as ::core::ffi::c_int
                } == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                s = s.offset(1);
            }
            if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                && poolGrow(&raw mut (*dtd).pool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh16 = (*dtd).pool.ptr;
                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                *fresh16 = '\0' as i32 as XML_Char;
                1 as ::core::ffi::c_int
            } == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            prefix = lookup(
                parser,
                &raw mut (*dtd).prefixes,
                (*dtd).pool.start as KEY,
                ::core::mem::size_of::<PREFIX>() as size_t,
            ) as *mut PREFIX;
            if prefix.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if (*prefix).name == (*dtd).pool.start as *const XML_Char {
                (*dtd).pool.start = (*dtd).pool.ptr;
            } else {
                (*dtd).pool.ptr = (*dtd).pool.start;
            }
            (*elementType).prefix = prefix;
            break;
        } else {
            name = name.offset(1);
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn getAttributeId(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ATTRIBUTE_ID {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut id: *mut ATTRIBUTE_ID = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    let mut name: *const XML_Char = ::core::ptr::null::<XML_Char>();
    if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char && poolGrow(&raw mut (*dtd).pool) == 0
    {
        0 as ::core::ffi::c_int
    } else {
        let fresh52 = (*dtd).pool.ptr;
        (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
        *fresh52 = '\0' as i32 as XML_Char;
        1 as ::core::ffi::c_int
    } == 0
    {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    name = poolStoreString(&raw mut (*dtd).pool, enc, start, end);
    if name.is_null() {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    name = name.offset(1);
    id = lookup(
        parser,
        &raw mut (*dtd).attributeIds,
        name as KEY,
        ::core::mem::size_of::<ATTRIBUTE_ID>() as size_t,
    ) as *mut ATTRIBUTE_ID;
    if id.is_null() {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    if (*id).name != name as *mut XML_Char {
        (*dtd).pool.ptr = (*dtd).pool.start;
    } else {
        (*dtd).pool.start = (*dtd).pool.ptr;
        if !((*parser).m_ns == 0) {
            if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x78 as ::core::ffi::c_int
                && *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x6d as ::core::ffi::c_int
                && *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x6c as ::core::ffi::c_int
                && *name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x6e as ::core::ffi::c_int
                && *name.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x73 as ::core::ffi::c_int
                && (*name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                    || *name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0x3a as ::core::ffi::c_int)
            {
                if *name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    (*id).prefix = &raw mut (*dtd).defaultPrefix;
                } else {
                    (*id).prefix = lookup(
                        parser,
                        &raw mut (*dtd).prefixes,
                        name.offset(6 as ::core::ffi::c_int as isize),
                        ::core::mem::size_of::<PREFIX>() as size_t,
                    ) as *mut PREFIX;
                }
                (*id).xmlns = XML_TRUE;
            } else {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while *name.offset(i as isize) != 0 {
                    if *name.offset(i as isize) as ::core::ffi::c_int == 0x3a as ::core::ffi::c_int
                    {
                        let mut j: ::core::ffi::c_int = 0;
                        j = 0 as ::core::ffi::c_int;
                        while j < i {
                            if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                                && poolGrow(&raw mut (*dtd).pool) == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                let fresh53 = (*dtd).pool.ptr;
                                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                                *fresh53 = *name.offset(j as isize);
                                1 as ::core::ffi::c_int
                            } == 0
                            {
                                return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                            }
                            j += 1;
                        }
                        if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                            && poolGrow(&raw mut (*dtd).pool) == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            let fresh54 = (*dtd).pool.ptr;
                            (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                            *fresh54 = '\0' as i32 as XML_Char;
                            1 as ::core::ffi::c_int
                        } == 0
                        {
                            return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                        }
                        (*id).prefix = lookup(
                            parser,
                            &raw mut (*dtd).prefixes,
                            (*dtd).pool.start as KEY,
                            ::core::mem::size_of::<PREFIX>() as size_t,
                        ) as *mut PREFIX;
                        if (*id).prefix.is_null() {
                            return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                        }
                        if (*(*id).prefix).name == (*dtd).pool.start as *const XML_Char {
                            (*dtd).pool.start = (*dtd).pool.ptr;
                        } else {
                            (*dtd).pool.ptr = (*dtd).pool.start;
                        }
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
    return id;
}
unsafe extern "C" fn getContext(mut parser: XML_Parser) -> *const XML_Char {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    let mut needSep: XML_Bool = XML_FALSE;
    if !(*dtd).defaultPrefix.binding.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut len: ::core::ffi::c_int = 0;
        if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
            && poolGrow(&raw mut (*parser).m_tempPool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            let fresh61 = (*parser).m_tempPool.ptr;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
            *fresh61 = 0x3d as XML_Char;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<XML_Char>();
        }
        len = (*(*dtd).defaultPrefix.binding).uriLen;
        if (*parser).m_namespaceSeparator != 0 {
            len -= 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < len {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh62 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh62 = *(*(*dtd).defaultPrefix.binding).uri.offset(i as isize);
                1 as ::core::ffi::c_int
            } == 0
            {
                return ::core::ptr::null::<XML_Char>();
            }
            i += 1;
        }
        needSep = XML_TRUE;
    }
    hashTableIterInit(&raw mut iter, &raw mut (*dtd).prefixes);
    loop {
        let mut i_0: ::core::ffi::c_int = 0;
        let mut len_0: ::core::ffi::c_int = 0;
        let mut s: *const XML_Char = ::core::ptr::null::<XML_Char>();
        let mut prefix: *mut PREFIX = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if prefix.is_null() {
            break;
        }
        if (*prefix).binding.is_null() {
            continue;
        }
        if needSep as ::core::ffi::c_int != 0
            && (if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh63 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh63 = 0xc as XML_Char;
                1 as ::core::ffi::c_int
            }) == 0
        {
            return ::core::ptr::null::<XML_Char>();
        }
        s = (*prefix).name;
        while *s != 0 {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh64 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh64 = *s;
                1 as ::core::ffi::c_int
            } == 0
            {
                return ::core::ptr::null::<XML_Char>();
            }
            s = s.offset(1);
        }
        if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
            && poolGrow(&raw mut (*parser).m_tempPool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            let fresh65 = (*parser).m_tempPool.ptr;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
            *fresh65 = 0x3d as XML_Char;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<XML_Char>();
        }
        len_0 = (*(*prefix).binding).uriLen;
        if (*parser).m_namespaceSeparator != 0 {
            len_0 -= 1;
        }
        i_0 = 0 as ::core::ffi::c_int;
        while i_0 < len_0 {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh66 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh66 = *(*(*prefix).binding).uri.offset(i_0 as isize);
                1 as ::core::ffi::c_int
            } == 0
            {
                return ::core::ptr::null::<XML_Char>();
            }
            i_0 += 1;
        }
        needSep = XML_TRUE;
    }
    hashTableIterInit(&raw mut iter, &raw mut (*dtd).generalEntities);
    loop {
        let mut s_0: *const XML_Char = ::core::ptr::null::<XML_Char>();
        let mut e: *mut ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if e.is_null() {
            break;
        }
        if (*e).open == 0 {
            continue;
        }
        if needSep as ::core::ffi::c_int != 0
            && (if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh67 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh67 = 0xc as XML_Char;
                1 as ::core::ffi::c_int
            }) == 0
        {
            return ::core::ptr::null::<XML_Char>();
        }
        s_0 = (*e).name;
        while *s_0 != 0 {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh68 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh68 = *s_0;
                1 as ::core::ffi::c_int
            } == 0
            {
                return ::core::ptr::null::<XML_Char>();
            }
            s_0 = s_0.offset(1);
        }
        needSep = XML_TRUE;
    }
    if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
        && poolGrow(&raw mut (*parser).m_tempPool) == 0
    {
        0 as ::core::ffi::c_int
    } else {
        let fresh69 = (*parser).m_tempPool.ptr;
        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
        *fresh69 = '\0' as i32 as XML_Char;
        1 as ::core::ffi::c_int
    } == 0
    {
        return ::core::ptr::null::<XML_Char>();
    }
    return (*parser).m_tempPool.start;
}
unsafe extern "C" fn setContext(mut parser: XML_Parser, mut context: *const XML_Char) -> XML_Bool {
    if context.is_null() {
        return XML_FALSE;
    }
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut s: *const XML_Char = context;
    while *context as ::core::ffi::c_int != '\0' as i32 {
        if *s as ::core::ffi::c_int == 0xc as ::core::ffi::c_int
            || *s as ::core::ffi::c_int == '\0' as i32
        {
            let mut e: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh76 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh76 = '\0' as i32 as XML_Char;
                1 as ::core::ffi::c_int
            } == 0
            {
                return XML_FALSE;
            }
            e = lookup(
                parser,
                &raw mut (*dtd).generalEntities,
                (*parser).m_tempPool.start as KEY,
                0 as size_t,
            ) as *mut ENTITY;
            if !e.is_null() {
                (*e).open = XML_TRUE;
            }
            if *s as ::core::ffi::c_int != '\0' as i32 {
                s = s.offset(1);
            }
            context = s;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
        } else if *s as ::core::ffi::c_int == 0x3d as ::core::ffi::c_int {
            let mut prefix: *mut PREFIX = ::core::ptr::null_mut::<PREFIX>();
            if (*parser)
                .m_tempPool
                .ptr
                .offset_from((*parser).m_tempPool.start) as ::core::ffi::c_long
                == 0 as ::core::ffi::c_long
            {
                prefix = &raw mut (*dtd).defaultPrefix;
            } else {
                if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    let fresh77 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh77 = '\0' as i32 as XML_Char;
                    1 as ::core::ffi::c_int
                } == 0
                {
                    return XML_FALSE;
                }
                let prefixName: *const XML_Char =
                    poolCopyStringNoFinish(&raw mut (*dtd).pool, (*parser).m_tempPool.start)
                        as *const XML_Char;
                if prefixName.is_null() {
                    return XML_FALSE;
                }
                prefix = lookup(
                    parser,
                    &raw mut (*dtd).prefixes,
                    prefixName as KEY,
                    ::core::mem::size_of::<PREFIX>() as size_t,
                ) as *mut PREFIX;
                let prefixNameUsed: bool = !prefix.is_null() && (*prefix).name == prefixName;
                if prefixNameUsed {
                    (*dtd).pool.start = (*dtd).pool.ptr;
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                }
                if prefix.is_null() {
                    return XML_FALSE;
                }
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
            }
            context = s.offset(1 as ::core::ffi::c_int as isize);
            while *context as ::core::ffi::c_int != 0xc as ::core::ffi::c_int
                && *context as ::core::ffi::c_int != '\0' as i32
            {
                if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    let fresh78 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh78 = *context;
                    1 as ::core::ffi::c_int
                } == 0
                {
                    return XML_FALSE;
                }
                context = context.offset(1);
            }
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh79 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh79 = '\0' as i32 as XML_Char;
                1 as ::core::ffi::c_int
            } == 0
            {
                return XML_FALSE;
            }
            if addBinding(
                parser,
                prefix,
                ::core::ptr::null::<ATTRIBUTE_ID>(),
                (*parser).m_tempPool.start,
                &raw mut (*parser).m_inheritedBindings,
            ) as ::core::ffi::c_uint
                != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return XML_FALSE;
            }
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
            if *context as ::core::ffi::c_int != '\0' as i32 {
                context = context.offset(1);
            }
            s = context;
        } else {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let fresh80 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh80 = *s;
                1 as ::core::ffi::c_int
            } == 0
            {
                return XML_FALSE;
            }
            s = s.offset(1);
        }
    }
    return XML_TRUE;
}
unsafe extern "C" fn normalizePublicId(mut publicId: *mut XML_Char) {
    let mut p: *mut XML_Char = publicId;
    let mut s: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    s = publicId;
    while *s != 0 {
        match *s as ::core::ffi::c_int {
            32 | 13 | 10 => {
                if p != publicId
                    && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        != 0x20 as ::core::ffi::c_int
                {
                    let fresh70 = p;
                    p = p.offset(1);
                    *fresh70 = 0x20 as XML_Char;
                }
            }
            _ => {
                let fresh71 = p;
                p = p.offset(1);
                *fresh71 = *s;
            }
        }
        s = s.offset(1);
    }
    if p != publicId
        && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == 0x20 as ::core::ffi::c_int
    {
        p = p.offset(-1);
    }
    *p = '\0' as i32 as XML_Char;
}
unsafe extern "C" fn dtdCreate(mut parser: XML_Parser) -> *mut DTD {
    let mut p: *mut DTD = expat_malloc(
        parser,
        ::core::mem::size_of::<DTD>() as size_t,
        7526 as ::core::ffi::c_int,
    ) as *mut DTD;
    if p.is_null() {
        return p;
    }
    poolInit(&raw mut (*p).pool, parser);
    poolInit(&raw mut (*p).entityValuePool, parser);
    hashTableInit(&raw mut (*p).generalEntities, parser);
    hashTableInit(&raw mut (*p).elementTypes, parser);
    hashTableInit(&raw mut (*p).attributeIds, parser);
    hashTableInit(&raw mut (*p).prefixes, parser);
    (*p).paramEntityRead = XML_FALSE;
    hashTableInit(&raw mut (*p).paramEntities, parser);
    (*p).defaultPrefix.name = ::core::ptr::null::<XML_Char>();
    (*p).defaultPrefix.binding = ::core::ptr::null_mut::<BINDING>();
    (*p).in_eldecl = XML_FALSE;
    (*p).scaffIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*p).scaffold = ::core::ptr::null_mut::<CONTENT_SCAFFOLD>();
    (*p).scaffLevel = 0 as ::core::ffi::c_int;
    (*p).scaffSize = 0 as ::core::ffi::c_uint;
    (*p).scaffCount = 0 as ::core::ffi::c_uint;
    (*p).contentStringLen = 0 as ::core::ffi::c_uint;
    (*p).keepProcessing = XML_TRUE;
    (*p).hasParamEntityRefs = XML_FALSE;
    (*p).standalone = XML_FALSE;
    return p;
}
unsafe extern "C" fn dtdReset(mut p: *mut DTD, mut parser: XML_Parser) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    hashTableIterInit(&raw mut iter, &raw mut (*p).elementTypes);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 as ::core::ffi::c_int {
            expat_free(
                parser,
                (*e).defaultAtts as *mut ::core::ffi::c_void,
                7565 as ::core::ffi::c_int,
            );
        }
    }
    hashTableClear(&raw mut (*p).generalEntities);
    (*p).paramEntityRead = XML_FALSE;
    hashTableClear(&raw mut (*p).paramEntities);
    hashTableClear(&raw mut (*p).elementTypes);
    hashTableClear(&raw mut (*p).attributeIds);
    hashTableClear(&raw mut (*p).prefixes);
    poolClear(&raw mut (*p).pool);
    poolClear(&raw mut (*p).entityValuePool);
    (*p).defaultPrefix.name = ::core::ptr::null::<XML_Char>();
    (*p).defaultPrefix.binding = ::core::ptr::null_mut::<BINDING>();
    (*p).in_eldecl = XML_FALSE;
    expat_free(
        parser,
        (*p).scaffIndex as *mut ::core::ffi::c_void,
        7582 as ::core::ffi::c_int,
    );
    (*p).scaffIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    expat_free(
        parser,
        (*p).scaffold as *mut ::core::ffi::c_void,
        7584 as ::core::ffi::c_int,
    );
    (*p).scaffold = ::core::ptr::null_mut::<CONTENT_SCAFFOLD>();
    (*p).scaffLevel = 0 as ::core::ffi::c_int;
    (*p).scaffSize = 0 as ::core::ffi::c_uint;
    (*p).scaffCount = 0 as ::core::ffi::c_uint;
    (*p).contentStringLen = 0 as ::core::ffi::c_uint;
    (*p).keepProcessing = XML_TRUE;
    (*p).hasParamEntityRefs = XML_FALSE;
    (*p).standalone = XML_FALSE;
}
unsafe extern "C" fn dtdDestroy(
    mut p: *mut DTD,
    mut isDocEntity: XML_Bool,
    mut parser: XML_Parser,
) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    hashTableIterInit(&raw mut iter, &raw mut (*p).elementTypes);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 as ::core::ffi::c_int {
            expat_free(
                parser,
                (*e).defaultAtts as *mut ::core::ffi::c_void,
                7606 as ::core::ffi::c_int,
            );
        }
    }
    hashTableDestroy(&raw mut (*p).generalEntities);
    hashTableDestroy(&raw mut (*p).paramEntities);
    hashTableDestroy(&raw mut (*p).elementTypes);
    hashTableDestroy(&raw mut (*p).attributeIds);
    hashTableDestroy(&raw mut (*p).prefixes);
    poolDestroy(&raw mut (*p).pool);
    poolDestroy(&raw mut (*p).entityValuePool);
    if isDocEntity != 0 {
        expat_free(
            parser,
            (*p).scaffIndex as *mut ::core::ffi::c_void,
            7618 as ::core::ffi::c_int,
        );
        expat_free(
            parser,
            (*p).scaffold as *mut ::core::ffi::c_void,
            7619 as ::core::ffi::c_int,
        );
    }
    expat_free(
        parser,
        p as *mut ::core::ffi::c_void,
        7621 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn dtdCopy(
    mut oldParser: XML_Parser,
    mut newDtd: *mut DTD,
    mut oldDtd: *const DTD,
    mut parser: XML_Parser,
) -> ::core::ffi::c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    hashTableIterInit(&raw mut iter, &raw const (*oldDtd).prefixes);
    loop {
        let mut name: *const XML_Char = ::core::ptr::null::<XML_Char>();
        let mut oldP: *const PREFIX = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if oldP.is_null() {
            break;
        }
        name = poolCopyString(&raw mut (*newDtd).pool, (*oldP).name);
        if name.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if lookup(
            oldParser,
            &raw mut (*newDtd).prefixes,
            name as KEY,
            ::core::mem::size_of::<PREFIX>() as size_t,
        )
        .is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    hashTableIterInit(&raw mut iter, &raw const (*oldDtd).attributeIds);
    loop {
        let mut newA: *mut ATTRIBUTE_ID = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
        let mut name_0: *const XML_Char = ::core::ptr::null::<XML_Char>();
        let mut oldA: *const ATTRIBUTE_ID = hashTableIterNext(&raw mut iter) as *mut ATTRIBUTE_ID;
        if oldA.is_null() {
            break;
        }
        if if (*newDtd).pool.ptr == (*newDtd).pool.end as *mut XML_Char
            && poolGrow(&raw mut (*newDtd).pool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            let fresh83 = (*newDtd).pool.ptr;
            (*newDtd).pool.ptr = (*newDtd).pool.ptr.offset(1);
            *fresh83 = '\0' as i32 as XML_Char;
            1 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        name_0 = poolCopyString(&raw mut (*newDtd).pool, (*oldA).name);
        if name_0.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        name_0 = name_0.offset(1);
        newA = lookup(
            oldParser,
            &raw mut (*newDtd).attributeIds,
            name_0 as KEY,
            ::core::mem::size_of::<ATTRIBUTE_ID>() as size_t,
        ) as *mut ATTRIBUTE_ID;
        if newA.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*newA).maybeTokenized = (*oldA).maybeTokenized;
        if !(*oldA).prefix.is_null() {
            (*newA).xmlns = (*oldA).xmlns;
            if (*oldA).prefix == &raw const (*oldDtd).defaultPrefix as *mut PREFIX {
                (*newA).prefix = &raw mut (*newDtd).defaultPrefix;
            } else {
                (*newA).prefix = lookup(
                    oldParser,
                    &raw mut (*newDtd).prefixes,
                    (*(*oldA).prefix).name as KEY,
                    0 as size_t,
                ) as *mut PREFIX;
            }
        }
    }
    hashTableIterInit(&raw mut iter, &raw const (*oldDtd).elementTypes);
    loop {
        let mut i: ::core::ffi::c_int = 0;
        let mut newE: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
        let mut name_1: *const XML_Char = ::core::ptr::null::<XML_Char>();
        let mut oldE: *const ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if oldE.is_null() {
            break;
        }
        name_1 = poolCopyString(&raw mut (*newDtd).pool, (*oldE).name);
        if name_1.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        newE = lookup(
            oldParser,
            &raw mut (*newDtd).elementTypes,
            name_1 as KEY,
            ::core::mem::size_of::<ELEMENT_TYPE>() as size_t,
        ) as *mut ELEMENT_TYPE;
        if newE.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*oldE).nDefaultAtts != 0 {
            (*newE).defaultAtts = expat_malloc(
                parser,
                ((*oldE).nDefaultAtts as size_t)
                    .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>() as size_t),
                7709 as ::core::ffi::c_int,
            ) as *mut DEFAULT_ATTRIBUTE;
            if (*newE).defaultAtts.is_null() {
                return 0 as ::core::ffi::c_int;
            }
        }
        if !(*oldE).idAtt.is_null() {
            (*newE).idAtt = lookup(
                oldParser,
                &raw mut (*newDtd).attributeIds,
                (*(*oldE).idAtt).name as KEY,
                0 as size_t,
            ) as *mut ATTRIBUTE_ID;
        }
        (*newE).nDefaultAtts = (*oldE).nDefaultAtts;
        (*newE).allocDefaultAtts = (*newE).nDefaultAtts;
        if !(*oldE).prefix.is_null() {
            (*newE).prefix = lookup(
                oldParser,
                &raw mut (*newDtd).prefixes,
                (*(*oldE).prefix).name as KEY,
                0 as size_t,
            ) as *mut PREFIX;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*newE).nDefaultAtts {
            let ref mut fresh84 = (*(*newE).defaultAtts.offset(i as isize)).id;
            *fresh84 = lookup(
                oldParser,
                &raw mut (*newDtd).attributeIds,
                (*(*(*oldE).defaultAtts.offset(i as isize)).id).name as KEY,
                0 as size_t,
            ) as *mut ATTRIBUTE_ID;
            (*(*newE).defaultAtts.offset(i as isize)).isCdata =
                (*(*oldE).defaultAtts.offset(i as isize)).isCdata;
            if !(*(*oldE).defaultAtts.offset(i as isize)).value.is_null() {
                let ref mut fresh85 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *fresh85 = poolCopyString(
                    &raw mut (*newDtd).pool,
                    (*(*oldE).defaultAtts.offset(i as isize)).value,
                );
                if (*(*newE).defaultAtts.offset(i as isize)).value.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
            } else {
                let ref mut fresh86 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *fresh86 = ::core::ptr::null::<XML_Char>();
            }
            i += 1;
        }
    }
    if copyEntityTable(
        oldParser,
        &raw mut (*newDtd).generalEntities,
        &raw mut (*newDtd).pool,
        &raw const (*oldDtd).generalEntities,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if copyEntityTable(
        oldParser,
        &raw mut (*newDtd).paramEntities,
        &raw mut (*newDtd).pool,
        &raw const (*oldDtd).paramEntities,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    (*newDtd).paramEntityRead = (*oldDtd).paramEntityRead;
    (*newDtd).keepProcessing = (*oldDtd).keepProcessing;
    (*newDtd).hasParamEntityRefs = (*oldDtd).hasParamEntityRefs;
    (*newDtd).standalone = (*oldDtd).standalone;
    (*newDtd).in_eldecl = (*oldDtd).in_eldecl;
    (*newDtd).scaffold = (*oldDtd).scaffold;
    (*newDtd).contentStringLen = (*oldDtd).contentStringLen;
    (*newDtd).scaffSize = (*oldDtd).scaffSize;
    (*newDtd).scaffLevel = (*oldDtd).scaffLevel;
    (*newDtd).scaffIndex = (*oldDtd).scaffIndex;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn copyEntityTable(
    mut oldParser: XML_Parser,
    mut newTable: *mut HASH_TABLE,
    mut newPool: *mut STRING_POOL,
    mut oldTable: *const HASH_TABLE,
) -> ::core::ffi::c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    let mut cachedOldBase: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut cachedNewBase: *const XML_Char = ::core::ptr::null::<XML_Char>();
    hashTableIterInit(&raw mut iter, oldTable);
    loop {
        let mut newE: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
        let mut name: *const XML_Char = ::core::ptr::null::<XML_Char>();
        let mut oldE: *const ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if oldE.is_null() {
            break;
        }
        name = poolCopyString(newPool, (*oldE).name);
        if name.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        newE = lookup(
            oldParser,
            newTable,
            name as KEY,
            ::core::mem::size_of::<ENTITY>() as size_t,
        ) as *mut ENTITY;
        if newE.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*oldE).systemId.is_null() {
            let mut tem: *const XML_Char = poolCopyString(newPool, (*oldE).systemId);
            if tem.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            (*newE).systemId = tem;
            if !(*oldE).base.is_null() {
                if (*oldE).base == cachedOldBase {
                    (*newE).base = cachedNewBase;
                } else {
                    cachedOldBase = (*oldE).base;
                    tem = poolCopyString(newPool, cachedOldBase);
                    if tem.is_null() {
                        return 0 as ::core::ffi::c_int;
                    }
                    (*newE).base = tem;
                    cachedNewBase = (*newE).base;
                }
            }
            if !(*oldE).publicId.is_null() {
                tem = poolCopyString(newPool, (*oldE).publicId);
                if tem.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
                (*newE).publicId = tem;
            }
        } else {
            let mut tem_0: *const XML_Char =
                poolCopyStringN(newPool, (*oldE).textPtr, (*oldE).textLen);
            if tem_0.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            (*newE).textPtr = tem_0;
            (*newE).textLen = (*oldE).textLen;
        }
        if !(*oldE).notation.is_null() {
            let mut tem_1: *const XML_Char = poolCopyString(newPool, (*oldE).notation);
            if tem_1.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            (*newE).notation = tem_1;
        }
        (*newE).is_param = (*oldE).is_param;
        (*newE).is_internal = (*oldE).is_internal;
    }
    return 1 as ::core::ffi::c_int;
}
pub const INIT_POWER: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
unsafe extern "C" fn keyeq(mut s1: KEY, mut s2: KEY) -> XML_Bool {
    while *s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int {
        if *s1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return XML_TRUE;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    return XML_FALSE;
}
unsafe extern "C" fn keylen(mut s: KEY) -> size_t {
    let mut len: size_t = 0 as size_t;
    while *s != 0 {
        s = s.offset(1);
        len = len.wrapping_add(1);
    }
    return len;
}
unsafe extern "C" fn copy_salt_to_sipkey(mut parser: XML_Parser, mut key: *mut sipkey) {
    (*key).k[0 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*key).k[1 as ::core::ffi::c_int as usize] = get_hash_secret_salt(parser) as uint64_t;
}
unsafe extern "C" fn hash(mut parser: XML_Parser, mut s: KEY) -> ::core::ffi::c_ulong {
    let mut state: siphash = siphash {
        v0: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        buf: [0; 8],
        p: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        c: 0,
    };
    let mut key: sipkey = sipkey { k: [0; 2] };
    copy_salt_to_sipkey(parser, &raw mut key);
    sip24_init(&raw mut state, &raw mut key);
    sip24_update(
        &raw mut state,
        s as *const ::core::ffi::c_void,
        keylen(s).wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
    );
    return sip24_final(&raw mut state) as ::core::ffi::c_ulong;
}
unsafe extern "C" fn lookup(
    mut parser: XML_Parser,
    mut table: *mut HASH_TABLE,
    mut name: KEY,
    mut createSize: size_t,
) -> *mut NAMED {
    let mut i: size_t = 0;
    if (*table).size == 0 as size_t {
        let mut tsize: size_t = 0;
        if createSize == 0 {
            return ::core::ptr::null_mut::<NAMED>();
        }
        (*table).power = INIT_POWER as ::core::ffi::c_uchar;
        (*table).size = (1 as ::core::ffi::c_int as size_t) << INIT_POWER;
        tsize = (*table)
            .size
            .wrapping_mul(::core::mem::size_of::<*mut NAMED>() as size_t);
        (*table).v =
            expat_malloc((*table).parser, tsize, 7871 as ::core::ffi::c_int) as *mut *mut NAMED;
        if (*table).v.is_null() {
            (*table).size = 0 as size_t;
            return ::core::ptr::null_mut::<NAMED>();
        }
        memset(
            (*table).v as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            tsize,
        );
        i = (hash(parser, name)
            & ((*table).size as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong))
            as size_t;
    } else {
        let mut h: ::core::ffi::c_ulong = hash(parser, name);
        let mut mask: ::core::ffi::c_ulong =
            ((*table).size as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong);
        let mut step: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
        i = (h & mask) as size_t;
        while !(*(*table).v.offset(i as isize)).is_null() {
            if keyeq(name, (**(*table).v.offset(i as isize)).name) != 0 {
                return *(*table).v.offset(i as isize);
            }
            if step == 0 {
                step = ((h & !mask)
                    >> (*table).power as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    & mask >> 2 as ::core::ffi::c_int
                    | 1 as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
            }
            if i < step as size_t {
                i = i.wrapping_add((*table).size.wrapping_sub(step as size_t));
            } else {
                i = i.wrapping_sub(step as size_t);
            };
        }
        if createSize == 0 {
            return ::core::ptr::null_mut::<NAMED>();
        }
        if (*table).used >> (*table).power as ::core::ffi::c_int - 1 as ::core::ffi::c_int != 0 {
            let mut newPower: ::core::ffi::c_uchar = ((*table).power as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            if newPower as usize
                >= (::core::mem::size_of::<::core::ffi::c_ulong>() as usize)
                    .wrapping_mul(8 as usize)
            {
                return ::core::ptr::null_mut::<NAMED>();
            }
            let mut newSize: size_t =
                (1 as ::core::ffi::c_int as size_t) << newPower as ::core::ffi::c_int;
            let mut newMask: ::core::ffi::c_ulong =
                (newSize as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong);
            if newSize
                > (SIZE_MAX as usize).wrapping_div(::core::mem::size_of::<*mut NAMED>() as usize)
            {
                return ::core::ptr::null_mut::<NAMED>();
            }
            let mut tsize_0: size_t =
                newSize.wrapping_mul(::core::mem::size_of::<*mut NAMED>() as size_t);
            let mut newV: *mut *mut NAMED =
                expat_malloc((*table).parser, tsize_0, 7911 as ::core::ffi::c_int)
                    as *mut *mut NAMED;
            if newV.is_null() {
                return ::core::ptr::null_mut::<NAMED>();
            }
            memset(
                newV as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                tsize_0,
            );
            i = 0 as size_t;
            while i < (*table).size {
                if !(*(*table).v.offset(i as isize)).is_null() {
                    let mut newHash: ::core::ffi::c_ulong =
                        hash(parser, (**(*table).v.offset(i as isize)).name);
                    let mut j: size_t = newHash as size_t & newMask as size_t;
                    step = 0 as ::core::ffi::c_uchar;
                    while !(*newV.offset(j as isize)).is_null() {
                        if step == 0 {
                            step = ((newHash & !newMask)
                                >> newPower as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                & newMask >> 2 as ::core::ffi::c_int
                                | 1 as ::core::ffi::c_ulong)
                                as ::core::ffi::c_uchar;
                        }
                        if j < step as size_t {
                            j = j.wrapping_add(newSize.wrapping_sub(step as size_t));
                        } else {
                            j = j.wrapping_sub(step as size_t);
                        };
                    }
                    let ref mut fresh17 = *newV.offset(j as isize);
                    *fresh17 = *(*table).v.offset(i as isize);
                }
                i = i.wrapping_add(1);
            }
            expat_free(
                (*table).parser,
                (*table).v as *mut ::core::ffi::c_void,
                7927 as ::core::ffi::c_int,
            );
            (*table).v = newV;
            (*table).power = newPower;
            (*table).size = newSize;
            i = (h & newMask) as size_t;
            step = 0 as ::core::ffi::c_uchar;
            while !(*(*table).v.offset(i as isize)).is_null() {
                if step == 0 {
                    step = ((h & !newMask)
                        >> newPower as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        & newMask >> 2 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_ulong)
                        as ::core::ffi::c_uchar;
                }
                if i < step as size_t {
                    i = i.wrapping_add(newSize.wrapping_sub(step as size_t));
                } else {
                    i = i.wrapping_sub(step as size_t);
                };
            }
        }
    }
    let ref mut fresh18 = *(*table).v.offset(i as isize);
    *fresh18 = expat_malloc((*table).parser, createSize, 7940 as ::core::ffi::c_int) as *mut NAMED;
    if (*(*table).v.offset(i as isize)).is_null() {
        return ::core::ptr::null_mut::<NAMED>();
    }
    memset(
        *(*table).v.offset(i as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        createSize,
    );
    let ref mut fresh19 = (**(*table).v.offset(i as isize)).name;
    *fresh19 = name;
    (*table).used = (*table).used.wrapping_add(1);
    return *(*table).v.offset(i as isize);
}
unsafe extern "C" fn hashTableClear(mut table: *mut HASH_TABLE) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*table).size {
        expat_free(
            (*table).parser,
            *(*table).v.offset(i as isize) as *mut ::core::ffi::c_void,
            7953 as ::core::ffi::c_int,
        );
        let ref mut fresh75 = *(*table).v.offset(i as isize);
        *fresh75 = ::core::ptr::null_mut::<NAMED>();
        i = i.wrapping_add(1);
    }
    (*table).used = 0 as size_t;
}
unsafe extern "C" fn hashTableDestroy(mut table: *mut HASH_TABLE) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*table).size {
        expat_free(
            (*table).parser,
            *(*table).v.offset(i as isize) as *mut ::core::ffi::c_void,
            7963 as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    expat_free(
        (*table).parser,
        (*table).v as *mut ::core::ffi::c_void,
        7964 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn hashTableInit(mut p: *mut HASH_TABLE, mut parser: XML_Parser) {
    (*p).power = 0 as ::core::ffi::c_uchar;
    (*p).size = 0 as size_t;
    (*p).used = 0 as size_t;
    (*p).v = ::core::ptr::null_mut::<*mut NAMED>();
    (*p).parser = parser;
}
unsafe extern "C" fn hashTableIterInit(
    mut iter: *mut HASH_TABLE_ITER,
    mut table: *const HASH_TABLE,
) {
    (*iter).p = (*table).v;
    (*iter).end = if !(*iter).p.is_null() {
        (*iter).p.offset((*table).size as isize)
    } else {
        ::core::ptr::null_mut::<*mut NAMED>()
    };
}
unsafe extern "C" fn hashTableIterNext(mut iter: *mut HASH_TABLE_ITER) -> *mut NAMED {
    while (*iter).p != (*iter).end {
        let fresh0 = (*iter).p;
        (*iter).p = (*iter).p.offset(1);
        let mut tem: *mut NAMED = *fresh0;
        if !tem.is_null() {
            return tem;
        }
    }
    return ::core::ptr::null_mut::<NAMED>();
}
unsafe extern "C" fn poolInit(mut pool: *mut STRING_POOL, mut parser: XML_Parser) {
    (*pool).blocks = ::core::ptr::null_mut::<BLOCK>();
    (*pool).freeBlocks = ::core::ptr::null_mut::<BLOCK>();
    (*pool).start = ::core::ptr::null_mut::<XML_Char>();
    (*pool).ptr = ::core::ptr::null_mut::<XML_Char>();
    (*pool).end = ::core::ptr::null::<XML_Char>();
    (*pool).parser = parser;
}
unsafe extern "C" fn poolClear(mut pool: *mut STRING_POOL) {
    if (*pool).freeBlocks.is_null() {
        (*pool).freeBlocks = (*pool).blocks;
    } else {
        let mut p: *mut BLOCK = (*pool).blocks;
        while !p.is_null() {
            let mut tem: *mut BLOCK = (*p).next as *mut BLOCK;
            (*p).next = (*pool).freeBlocks as *mut block;
            (*pool).freeBlocks = p;
            p = tem;
        }
    }
    (*pool).blocks = ::core::ptr::null_mut::<BLOCK>();
    (*pool).start = ::core::ptr::null_mut::<XML_Char>();
    (*pool).ptr = ::core::ptr::null_mut::<XML_Char>();
    (*pool).end = ::core::ptr::null::<XML_Char>();
}
unsafe extern "C" fn poolDestroy(mut pool: *mut STRING_POOL) {
    let mut p: *mut BLOCK = (*pool).blocks;
    while !p.is_null() {
        let mut tem: *mut BLOCK = (*p).next as *mut BLOCK;
        expat_free(
            (*pool).parser,
            p as *mut ::core::ffi::c_void,
            8026 as ::core::ffi::c_int,
        );
        p = tem;
    }
    p = (*pool).freeBlocks;
    while !p.is_null() {
        let mut tem_0: *mut BLOCK = (*p).next as *mut BLOCK;
        expat_free(
            (*pool).parser,
            p as *mut ::core::ffi::c_void,
            8032 as ::core::ffi::c_int,
        );
        p = tem_0;
    }
}
unsafe extern "C" fn poolAppend(
    mut pool: *mut STRING_POOL,
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut XML_Char {
    if (*pool).ptr.is_null() && poolGrow(pool) == 0 {
        return ::core::ptr::null_mut::<XML_Char>();
    }
    loop {
        let convert_res: XML_Convert_Result = (*enc).utf8Convert.expect("non-null function pointer")(
            enc,
            &raw mut ptr,
            end,
            &raw mut (*pool).ptr as *mut *mut ::core::ffi::c_char,
            (*pool).end as *const ::core::ffi::c_char,
        ) as XML_Convert_Result;
        if convert_res as ::core::ffi::c_uint
            == XML_CONVERT_COMPLETED as ::core::ffi::c_int as ::core::ffi::c_uint
            || convert_res as ::core::ffi::c_uint
                == XML_CONVERT_INPUT_INCOMPLETE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        if poolGrow(pool) == 0 {
            return ::core::ptr::null_mut::<XML_Char>();
        }
    }
    return (*pool).start;
}
unsafe extern "C" fn poolCopyString(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
) -> *const XML_Char {
    loop {
        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
            0 as ::core::ffi::c_int
        } else {
            let fresh59 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh59 = *s;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<XML_Char>();
        }
        let fresh60 = s;
        s = s.offset(1);
        if !(*fresh60 != 0) {
            break;
        }
    }
    s = (*pool).start;
    (*pool).start = (*pool).ptr;
    return s;
}
unsafe extern "C" fn poolCopyStringNoFinish(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
) -> *const XML_Char {
    let original: *const XML_Char = s;
    loop {
        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
            0 as ::core::ffi::c_int
        } else {
            let fresh81 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh81 = *s;
            1 as ::core::ffi::c_int
        } == 0
        {
            let advancedBy: ptrdiff_t = s.offset_from(original) as ptrdiff_t;
            if advancedBy > 0 as ptrdiff_t {
                (*pool).ptr = (*pool).ptr.offset(-(advancedBy as isize));
            }
            return ::core::ptr::null::<XML_Char>();
        }
        let fresh82 = s;
        s = s.offset(1);
        if !(*fresh82 != 0) {
            break;
        }
    }
    return (*pool).start;
}
unsafe extern "C" fn poolCopyStringN(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
    mut n: ::core::ffi::c_int,
) -> *const XML_Char {
    if (*pool).ptr.is_null() && poolGrow(pool) == 0 {
        return ::core::ptr::null::<XML_Char>();
    }
    while n > 0 as ::core::ffi::c_int {
        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
            0 as ::core::ffi::c_int
        } else {
            let fresh87 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh87 = *s;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<XML_Char>();
        }
        n -= 1;
        s = s.offset(1);
    }
    s = (*pool).start;
    (*pool).start = (*pool).ptr;
    return s;
}
unsafe extern "C" fn poolAppendString(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
) -> *const XML_Char {
    while *s != 0 {
        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
            0 as ::core::ffi::c_int
        } else {
            let fresh74 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh74 = *s;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<XML_Char>();
        }
        s = s.offset(1);
    }
    return (*pool).start;
}
unsafe extern "C" fn poolStoreString(
    mut pool: *mut STRING_POOL,
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut XML_Char {
    if poolAppend(pool, enc, ptr, end).is_null() {
        return ::core::ptr::null_mut::<XML_Char>();
    }
    if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
        return ::core::ptr::null_mut::<XML_Char>();
    }
    let fresh10 = (*pool).ptr;
    (*pool).ptr = (*pool).ptr.offset(1);
    *fresh10 = 0 as XML_Char;
    return (*pool).start;
}
unsafe extern "C" fn poolBytesToAllocateFor(mut blockSize: ::core::ffi::c_int) -> size_t {
    let stretch: size_t = ::core::mem::size_of::<XML_Char>() as size_t;
    if blockSize <= 0 as ::core::ffi::c_int {
        return 0 as size_t;
    }
    if blockSize > (INT_MAX as size_t).wrapping_div(stretch) as ::core::ffi::c_int {
        return 0 as size_t;
    }
    let stretchedBlockSize: ::core::ffi::c_int = blockSize * stretch as ::core::ffi::c_int;
    let bytesToAllocate: ::core::ffi::c_int = (12 as ::core::ffi::c_ulong)
        .wrapping_add(stretchedBlockSize as ::core::ffi::c_uint as ::core::ffi::c_ulong)
        as ::core::ffi::c_int;
    if bytesToAllocate < 0 as ::core::ffi::c_int {
        return 0 as size_t;
    }
    return bytesToAllocate as size_t;
}
unsafe extern "C" fn poolGrow(mut pool: *mut STRING_POOL) -> XML_Bool {
    if !(*pool).freeBlocks.is_null() {
        if (*pool).start.is_null() {
            (*pool).blocks = (*pool).freeBlocks;
            (*pool).freeBlocks = (*(*pool).freeBlocks).next as *mut BLOCK;
            (*(*pool).blocks).next = ::core::ptr::null_mut::<block>();
            (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char;
            (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize);
            (*pool).ptr = (*pool).start;
            return XML_TRUE;
        }
        if ((*pool).end.offset_from((*pool).start) as ::core::ffi::c_long)
            < (*(*pool).freeBlocks).size as ::core::ffi::c_long
        {
            let mut tem: *mut BLOCK = (*(*pool).freeBlocks).next as *mut BLOCK;
            (*(*pool).freeBlocks).next = (*pool).blocks as *mut block;
            (*pool).blocks = (*pool).freeBlocks;
            (*pool).freeBlocks = tem;
            memcpy(
                &raw mut (*(*pool).blocks).s as *mut XML_Char as *mut ::core::ffi::c_void,
                (*pool).start as *const ::core::ffi::c_void,
                ((*pool).end.offset_from((*pool).start) as ::core::ffi::c_long as size_t)
                    .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
            );
            (*pool).ptr = (&raw mut (*(*pool).blocks).s as *mut XML_Char)
                .offset((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long as isize);
            (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char;
            (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize);
            return XML_TRUE;
        }
    }
    if !(*pool).blocks.is_null() && (*pool).start == &raw mut (*(*pool).blocks).s as *mut XML_Char {
        let mut temp: *mut BLOCK = ::core::ptr::null_mut::<BLOCK>();
        let mut blockSize: ::core::ffi::c_int =
            ((*pool).end.offset_from((*pool).start) as ::core::ffi::c_long as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        let mut bytesToAllocate: size_t = 0;
        let offsetInsideBlock: ptrdiff_t = (*pool).ptr.offset_from((*pool).start) as ptrdiff_t;
        if blockSize < 0 as ::core::ffi::c_int {
            return XML_FALSE;
        }
        bytesToAllocate = poolBytesToAllocateFor(blockSize);
        if bytesToAllocate == 0 as size_t {
            return XML_FALSE;
        }
        temp = expat_realloc(
            (*pool).parser,
            (*pool).blocks as *mut ::core::ffi::c_void,
            bytesToAllocate,
            8204 as ::core::ffi::c_int,
        ) as *mut BLOCK;
        if temp.is_null() {
            return XML_FALSE;
        }
        (*pool).blocks = temp;
        (*(*pool).blocks).size = blockSize;
        (*pool).ptr =
            (&raw mut (*(*pool).blocks).s as *mut XML_Char).offset(offsetInsideBlock as isize);
        (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char;
        (*pool).end = (*pool).start.offset(blockSize as isize);
    } else {
        let mut tem_0: *mut BLOCK = ::core::ptr::null_mut::<BLOCK>();
        let mut blockSize_0: ::core::ffi::c_int =
            (*pool).end.offset_from((*pool).start) as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut bytesToAllocate_0: size_t = 0;
        if blockSize_0 < 0 as ::core::ffi::c_int {
            return XML_FALSE;
        }
        if blockSize_0 < INIT_BLOCK_SIZE {
            blockSize_0 = INIT_BLOCK_SIZE;
        } else {
            if ((blockSize_0 as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
                as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                return XML_FALSE;
            }
            blockSize_0 *= 2 as ::core::ffi::c_int;
        }
        bytesToAllocate_0 = poolBytesToAllocateFor(blockSize_0);
        if bytesToAllocate_0 == 0 as size_t {
            return XML_FALSE;
        }
        tem_0 = expat_malloc(
            (*pool).parser,
            bytesToAllocate_0,
            8244 as ::core::ffi::c_int,
        ) as *mut BLOCK;
        if tem_0.is_null() {
            return XML_FALSE;
        }
        (*tem_0).size = blockSize_0;
        (*tem_0).next = (*pool).blocks as *mut block;
        (*pool).blocks = tem_0;
        if (*pool).ptr != (*pool).start {
            memcpy(
                &raw mut (*tem_0).s as *mut XML_Char as *mut ::core::ffi::c_void,
                (*pool).start as *const ::core::ffi::c_void,
                ((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long as size_t)
                    .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
            );
        }
        (*pool).ptr = (&raw mut (*tem_0).s as *mut XML_Char)
            .offset((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long as isize);
        (*pool).start = &raw mut (*tem_0).s as *mut XML_Char;
        (*pool).end = (&raw mut (*tem_0).s as *mut XML_Char).offset(blockSize_0 as isize);
    }
    return XML_TRUE;
}
unsafe extern "C" fn nextScaffoldPart(mut parser: XML_Parser) -> ::core::ffi::c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut me: *mut CONTENT_SCAFFOLD = ::core::ptr::null_mut::<CONTENT_SCAFFOLD>();
    let mut next: ::core::ffi::c_int = 0;
    if (*dtd).scaffIndex.is_null() {
        (*dtd).scaffIndex = expat_malloc(
            parser,
            ((*parser).m_groupSize as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            8275 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
        if (*dtd).scaffIndex.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        *(*dtd).scaffIndex.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    }
    if (*dtd).scaffCount > INT_MAX as ::core::ffi::c_uint {
        return -(1 as ::core::ffi::c_int);
    }
    if (*dtd).scaffCount >= (*dtd).scaffSize {
        let mut temp: *mut CONTENT_SCAFFOLD = ::core::ptr::null_mut::<CONTENT_SCAFFOLD>();
        if !(*dtd).scaffold.is_null() {
            if (*dtd).scaffSize > UINT_MAX.wrapping_div(2 as ::core::ffi::c_uint) {
                return -(1 as ::core::ffi::c_int);
            }
            temp = expat_realloc(
                parser,
                (*dtd).scaffold as *mut ::core::ffi::c_void,
                ((*dtd).scaffSize.wrapping_mul(2 as ::core::ffi::c_uint) as size_t)
                    .wrapping_mul(::core::mem::size_of::<CONTENT_SCAFFOLD>() as size_t),
                8304 as ::core::ffi::c_int,
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
            (*dtd).scaffSize = (*dtd).scaffSize.wrapping_mul(2 as ::core::ffi::c_uint);
        } else {
            temp = expat_malloc(
                parser,
                (32 as size_t).wrapping_mul(::core::mem::size_of::<CONTENT_SCAFFOLD>() as size_t),
                8309 as ::core::ffi::c_int,
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
            (*dtd).scaffSize = INIT_SCAFFOLD_ELEMENTS as ::core::ffi::c_uint;
        }
        (*dtd).scaffold = temp;
    }
    let fresh14 = (*dtd).scaffCount;
    (*dtd).scaffCount = (*dtd).scaffCount.wrapping_add(1);
    next = fresh14 as ::core::ffi::c_int;
    me = (*dtd).scaffold.offset(next as isize) as *mut CONTENT_SCAFFOLD;
    if (*dtd).scaffLevel != 0 {
        let mut parent: *mut CONTENT_SCAFFOLD = (*dtd).scaffold.offset(
            *(*dtd)
                .scaffIndex
                .offset(((*dtd).scaffLevel - 1 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut CONTENT_SCAFFOLD;
        if (*parent).lastchild != 0 {
            (*(*dtd).scaffold.offset((*parent).lastchild as isize)).nextsib = next;
        }
        if (*parent).childcnt == 0 {
            (*parent).firstchild = next;
        }
        (*parent).lastchild = next;
        (*parent).childcnt += 1;
    }
    (*me).nextsib = 0 as ::core::ffi::c_int;
    (*me).childcnt = (*me).nextsib;
    (*me).lastchild = (*me).childcnt;
    (*me).firstchild = (*me).lastchild;
    return next;
}
unsafe extern "C" fn build_model(mut parser: XML_Parser) -> *mut XML_Content {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut ret: *mut XML_Content = ::core::ptr::null_mut::<XML_Content>();
    let mut str: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    if ((*dtd).scaffCount as usize).wrapping_mul(::core::mem::size_of::<XML_Content>() as usize)
        > (SIZE_MAX as usize).wrapping_sub(
            ((*dtd).contentStringLen as usize)
                .wrapping_mul(::core::mem::size_of::<XML_Char>() as usize),
        )
    {
        return ::core::ptr::null_mut::<XML_Content>();
    }
    let allocsize: size_t = ((*dtd).scaffCount as size_t)
        .wrapping_mul(::core::mem::size_of::<XML_Content>() as size_t)
        .wrapping_add(
            ((*dtd).contentStringLen as size_t)
                .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
        );
    ret = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(allocsize) as *mut XML_Content;
    if ret.is_null() {
        return ::core::ptr::null_mut::<XML_Content>();
    }
    let mut dest: *mut XML_Content = ret;
    let destLimit: *mut XML_Content = ret.offset((*dtd).scaffCount as isize) as *mut XML_Content;
    let mut jobDest: *mut XML_Content = ret;
    str = ret.offset((*dtd).scaffCount as isize) as *mut XML_Content as *mut XML_Char;
    let fresh11 = jobDest;
    jobDest = jobDest.offset(1);
    (*fresh11).numchildren = 0 as ::core::ffi::c_uint;
    while dest < destLimit {
        let src_node: ::core::ffi::c_int = (*dest).numchildren as ::core::ffi::c_int;
        (*dest).type_0 = (*(*dtd).scaffold.offset(src_node as isize)).type_0;
        (*dest).quant = (*(*dtd).scaffold.offset(src_node as isize)).quant;
        if (*dest).type_0 as ::core::ffi::c_uint
            == XML_CTYPE_NAME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut src: *const XML_Char = ::core::ptr::null::<XML_Char>();
            (*dest).name = str;
            src = (*(*dtd).scaffold.offset(src_node as isize)).name;
            loop {
                let fresh12 = str;
                str = str.offset(1);
                *fresh12 = *src;
                if *src == 0 {
                    break;
                }
                src = src.offset(1);
            }
            (*dest).numchildren = 0 as ::core::ffi::c_uint;
            (*dest).children = ::core::ptr::null_mut::<XML_Content>();
        } else {
            let mut i: ::core::ffi::c_uint = 0;
            let mut cn: ::core::ffi::c_int = 0;
            (*dest).name = ::core::ptr::null_mut::<XML_Char>();
            (*dest).numchildren =
                (*(*dtd).scaffold.offset(src_node as isize)).childcnt as ::core::ffi::c_uint;
            (*dest).children = jobDest;
            i = 0 as ::core::ffi::c_uint;
            cn = (*(*dtd).scaffold.offset(src_node as isize)).firstchild;
            while i < (*dest).numchildren {
                let fresh13 = jobDest;
                jobDest = jobDest.offset(1);
                (*fresh13).numchildren = cn as ::core::ffi::c_uint;
                i = i.wrapping_add(1);
                cn = (*(*dtd).scaffold.offset(cn as isize)).nextsib;
            }
        }
        dest = dest.offset(1);
    }
    return ret;
}
unsafe extern "C" fn getElementType(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ELEMENT_TYPE {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const XML_Char = poolStoreString(&raw mut (*dtd).pool, enc, ptr, end);
    let mut ret: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    if name.is_null() {
        return ::core::ptr::null_mut::<ELEMENT_TYPE>();
    }
    ret = lookup(
        parser,
        &raw mut (*dtd).elementTypes,
        name as KEY,
        ::core::mem::size_of::<ELEMENT_TYPE>() as size_t,
    ) as *mut ELEMENT_TYPE;
    if ret.is_null() {
        return ::core::ptr::null_mut::<ELEMENT_TYPE>();
    }
    if (*ret).name != name {
        (*dtd).pool.ptr = (*dtd).pool.start;
    } else {
        (*dtd).pool.start = (*dtd).pool.ptr;
        if setElementTypePrefix(parser, ret) == 0 {
            return ::core::ptr::null_mut::<ELEMENT_TYPE>();
        }
    }
    return ret;
}
unsafe extern "C" fn copyString(mut s: *const XML_Char, mut parser: XML_Parser) -> *mut XML_Char {
    let mut charsRequired: size_t = 0 as size_t;
    let mut result: *mut XML_Char = ::core::ptr::null_mut::<XML_Char>();
    while *s.offset(charsRequired as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        charsRequired = charsRequired.wrapping_add(1);
    }
    charsRequired = charsRequired.wrapping_add(1);
    result = expat_malloc(
        parser,
        charsRequired.wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
        8499 as ::core::ffi::c_int,
    ) as *mut XML_Char;
    if result.is_null() {
        return ::core::ptr::null_mut::<XML_Char>();
    }
    memcpy(
        result as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        charsRequired.wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
    );
    return result;
}
unsafe extern "C" fn accountingGetCurrentAmplification(
    mut rootParser: XML_Parser,
) -> ::core::ffi::c_float {
    let lenOfShortestInclude: size_t =
        (::core::mem::size_of::<[::core::ffi::c_char; 23]>() as size_t).wrapping_sub(1 as size_t);
    let countBytesOutput: XmlBigCount = (*rootParser)
        .m_accounting
        .countBytesDirect
        .wrapping_add((*rootParser).m_accounting.countBytesIndirect);
    let amplificationFactor: ::core::ffi::c_float =
        if (*rootParser).m_accounting.countBytesDirect != 0 {
            countBytesOutput as ::core::ffi::c_float
                / (*rootParser).m_accounting.countBytesDirect as ::core::ffi::c_float
        } else {
            (lenOfShortestInclude as XmlBigCount)
                .wrapping_add((*rootParser).m_accounting.countBytesIndirect)
                as ::core::ffi::c_float
                / lenOfShortestInclude as ::core::ffi::c_float
        };
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"accountingGetCurrentAmplification\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            8523 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return amplificationFactor;
}
unsafe extern "C" fn accountingReportStats(
    mut originParser: XML_Parser,
    mut epilog: *const ::core::ffi::c_char,
) {
    let rootParser: XML_Parser =
        getRootParserOf(originParser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"accountingReportStats\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            8530 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*rootParser).m_accounting.debugLevel == 0 as ::core::ffi::c_ulong {
        return;
    }
    let amplificationFactor: ::core::ffi::c_float =
        accountingGetCurrentAmplification(rootParser) as ::core::ffi::c_float;
    fprintf(
        __stderrp,
        b"expat: Accounting(%p): Direct %10llu, indirect %10llu, amplification %8.2f%s\0"
            as *const u8 as *const ::core::ffi::c_char,
        rootParser as *mut ::core::ffi::c_void,
        (*rootParser).m_accounting.countBytesDirect,
        (*rootParser).m_accounting.countBytesIndirect,
        amplificationFactor as ::core::ffi::c_double,
        epilog,
    );
}
unsafe extern "C" fn accountingOnAbort(mut originParser: XML_Parser) {
    accountingReportStats(
        originParser,
        b" ABORTING\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn accountingReportDiff(
    mut rootParser: XML_Parser,
    mut levelsAwayFromRootParser: ::core::ffi::c_uint,
    mut before: *const ::core::ffi::c_char,
    mut after: *const ::core::ffi::c_char,
    mut bytesMore: ptrdiff_t,
    mut source_line: ::core::ffi::c_int,
    mut account: XML_Account,
) {
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"accountingReportDiff\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            8556 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    fprintf(
        __stderrp,
        b" (+%6ld bytes %s|%u, xmlparse.c:%d) %*s\"\0" as *const u8 as *const ::core::ffi::c_char,
        bytesMore,
        if account as ::core::ffi::c_uint
            == XML_ACCOUNT_DIRECT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            b"DIR\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"EXP\0" as *const u8 as *const ::core::ffi::c_char
        },
        levelsAwayFromRootParser,
        source_line,
        10 as ::core::ffi::c_int,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let ellipis: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"[..]\0");
    let ellipsisLength: size_t =
        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1 as size_t);
    let contextLength: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
    let mut walker: *const ::core::ffi::c_char = before;
    if (*rootParser).m_accounting.debugLevel >= 3 as ::core::ffi::c_ulong
        || after.offset_from(before) as ptrdiff_t
            <= (contextLength as size_t)
                .wrapping_add(ellipsisLength)
                .wrapping_add(contextLength as size_t) as ptrdiff_t
    {
        while walker < after {
            fprintf(
                __stderrp,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                unsignedCharToPrintable(
                    *walker.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                ),
            );
            walker = walker.offset(1);
        }
    } else {
        while walker < before.offset(contextLength as isize) {
            fprintf(
                __stderrp,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                unsignedCharToPrintable(
                    *walker.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                ),
            );
            walker = walker.offset(1);
        }
        fprintf(__stderrp, &raw const ellipis as *const ::core::ffi::c_char);
        walker = after.offset(-(contextLength as isize));
        while walker < after {
            fprintf(
                __stderrp,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                unsignedCharToPrintable(
                    *walker.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                ),
            );
            walker = walker.offset(1);
        }
    }
    fprintf(
        __stderrp,
        b"\"\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn accountingDiffTolerated(
    mut originParser: XML_Parser,
    mut tok: ::core::ffi::c_int,
    mut before: *const ::core::ffi::c_char,
    mut after: *const ::core::ffi::c_char,
    mut source_line: ::core::ffi::c_int,
    mut account: XML_Account,
) -> XML_Bool {
    match tok {
        XML_TOK_INVALID | XML_TOK_PARTIAL | XML_TOK_PARTIAL_CHAR | XML_TOK_NONE => {
            return XML_TRUE;
        }
        _ => {}
    }
    if account as ::core::ffi::c_uint
        == XML_ACCOUNT_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return XML_TRUE;
    }
    let mut levelsAwayFromRootParser: ::core::ffi::c_uint = 0;
    let rootParser: XML_Parser =
        getRootParserOf(originParser, &raw mut levelsAwayFromRootParser) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"accountingDiffTolerated\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            8609 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    let isDirect: ::core::ffi::c_int = (account as ::core::ffi::c_uint
        == XML_ACCOUNT_DIRECT as ::core::ffi::c_int as ::core::ffi::c_uint
        && originParser == rootParser) as ::core::ffi::c_int;
    let bytesMore: ptrdiff_t = after.offset_from(before) as ptrdiff_t;
    let additionTarget: *mut XmlBigCount = if isDirect != 0 {
        &raw mut (*rootParser).m_accounting.countBytesDirect
    } else {
        &raw mut (*rootParser).m_accounting.countBytesIndirect
    };
    if *additionTarget
        > (-(1 as ::core::ffi::c_int) as XmlBigCount).wrapping_sub(bytesMore as XmlBigCount)
    {
        return XML_FALSE;
    }
    *additionTarget = (*additionTarget).wrapping_add(bytesMore as XmlBigCount);
    let countBytesOutput: XmlBigCount = (*rootParser)
        .m_accounting
        .countBytesDirect
        .wrapping_add((*rootParser).m_accounting.countBytesIndirect);
    let amplificationFactor: ::core::ffi::c_float =
        accountingGetCurrentAmplification(rootParser) as ::core::ffi::c_float;
    let tolerated: XML_Bool = (countBytesOutput
        < (*rootParser).m_accounting.activationThresholdBytes
        || amplificationFactor <= (*rootParser).m_accounting.maximumAmplificationFactor)
        as ::core::ffi::c_int as XML_Bool;
    if (*rootParser).m_accounting.debugLevel >= 2 as ::core::ffi::c_ulong {
        accountingReportStats(rootParser, b"\0" as *const u8 as *const ::core::ffi::c_char);
        accountingReportDiff(
            rootParser,
            levelsAwayFromRootParser,
            before,
            after,
            bytesMore,
            source_line,
            account,
        );
    }
    return tolerated;
}
#[no_mangle]
pub unsafe extern "C" fn testingAccountingGetCountBytesDirect(
    mut parser: XML_Parser,
) -> ::core::ffi::c_ulonglong {
    if parser.is_null() {
        return 0 as ::core::ffi::c_ulonglong;
    }
    return (*parser).m_accounting.countBytesDirect as ::core::ffi::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect(
    mut parser: XML_Parser,
) -> ::core::ffi::c_ulonglong {
    if parser.is_null() {
        return 0 as ::core::ffi::c_ulonglong;
    }
    return (*parser).m_accounting.countBytesIndirect as ::core::ffi::c_ulonglong;
}
unsafe extern "C" fn entityTrackingReportStats(
    mut rootParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut action: *const ::core::ffi::c_char,
    mut sourceLine: ::core::ffi::c_int,
) {
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"entityTrackingReportStats\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            8660 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if (*rootParser).m_entity_stats.debugLevel == 0 as ::core::ffi::c_ulong {
        return;
    }
    let entityName: *const ::core::ffi::c_char = (*entity).name as *const ::core::ffi::c_char;
    fprintf(
        __stderrp,
        b"expat: Entities(%p): Count %9u, depth %2u/%2u %*s%s%s; %s length %d (xmlparse.c:%d)\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        rootParser as *mut ::core::ffi::c_void,
        (*rootParser).m_entity_stats.countEverOpened,
        (*rootParser).m_entity_stats.currentDepth,
        (*rootParser).m_entity_stats.maximumDepthSeen,
        ((*rootParser).m_entity_stats.currentDepth as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        if (*entity).is_param as ::core::ffi::c_int != 0 {
            b"%\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"&\0" as *const u8 as *const ::core::ffi::c_char
        },
        entityName,
        action,
        (*entity).textLen,
        sourceLine,
    );
}
unsafe extern "C" fn entityTrackingOnOpen(
    mut originParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut sourceLine: ::core::ffi::c_int,
) {
    let rootParser: XML_Parser =
        getRootParserOf(originParser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"entityTrackingOnOpen\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            8684 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*rootParser).m_entity_stats.countEverOpened =
        (*rootParser).m_entity_stats.countEverOpened.wrapping_add(1);
    (*rootParser).m_entity_stats.currentDepth =
        (*rootParser).m_entity_stats.currentDepth.wrapping_add(1);
    if (*rootParser).m_entity_stats.currentDepth > (*rootParser).m_entity_stats.maximumDepthSeen {
        (*rootParser).m_entity_stats.maximumDepthSeen = (*rootParser)
            .m_entity_stats
            .maximumDepthSeen
            .wrapping_add(1);
    }
    entityTrackingReportStats(
        rootParser,
        entity,
        b"OPEN \0" as *const u8 as *const ::core::ffi::c_char,
        sourceLine,
    );
}
unsafe extern "C" fn entityTrackingOnClose(
    mut originParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut sourceLine: ::core::ffi::c_int,
) {
    let rootParser: XML_Parser =
        getRootParserOf(originParser, ::core::ptr::null_mut::<::core::ffi::c_uint>()) as XML_Parser;
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"entityTrackingOnClose\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            8699 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    entityTrackingReportStats(
        rootParser,
        entity,
        b"CLOSE\0" as *const u8 as *const ::core::ffi::c_char,
        sourceLine,
    );
    (*rootParser).m_entity_stats.currentDepth =
        (*rootParser).m_entity_stats.currentDepth.wrapping_sub(1);
}
unsafe extern "C" fn getRootParserOf(
    mut parser: XML_Parser,
    mut outLevelDiff: *mut ::core::ffi::c_uint,
) -> XML_Parser {
    let mut rootParser: XML_Parser = parser;
    let mut stepsTakenUpwards: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while !(*rootParser).m_parentParser.is_null() {
        rootParser = (*rootParser).m_parentParser;
        stepsTakenUpwards = stepsTakenUpwards.wrapping_add(1);
    }
    if !(*rootParser).m_parentParser.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"getRootParserOf\0" as *const u8 as *const ::core::ffi::c_char,
            b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
            8715 as ::core::ffi::c_int,
            b"! rootParser->m_parentParser\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !outLevelDiff.is_null() {
        *outLevelDiff = stepsTakenUpwards;
    }
    return rootParser;
}
#[no_mangle]
pub unsafe extern "C" fn unsignedCharToPrintable(
    mut c: ::core::ffi::c_uchar,
) -> *const ::core::ffi::c_char {
    match c as ::core::ffi::c_int {
        0 => return b"\\0\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"\\x1\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"\\x2\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"\\x3\0" as *const u8 as *const ::core::ffi::c_char,
        4 => return b"\\x4\0" as *const u8 as *const ::core::ffi::c_char,
        5 => return b"\\x5\0" as *const u8 as *const ::core::ffi::c_char,
        6 => return b"\\x6\0" as *const u8 as *const ::core::ffi::c_char,
        7 => return b"\\x7\0" as *const u8 as *const ::core::ffi::c_char,
        8 => return b"\\x8\0" as *const u8 as *const ::core::ffi::c_char,
        9 => return b"\\t\0" as *const u8 as *const ::core::ffi::c_char,
        10 => return b"\\n\0" as *const u8 as *const ::core::ffi::c_char,
        11 => return b"\\xB\0" as *const u8 as *const ::core::ffi::c_char,
        12 => return b"\\xC\0" as *const u8 as *const ::core::ffi::c_char,
        13 => return b"\\r\0" as *const u8 as *const ::core::ffi::c_char,
        14 => return b"\\xE\0" as *const u8 as *const ::core::ffi::c_char,
        15 => return b"\\xF\0" as *const u8 as *const ::core::ffi::c_char,
        16 => return b"\\x10\0" as *const u8 as *const ::core::ffi::c_char,
        17 => return b"\\x11\0" as *const u8 as *const ::core::ffi::c_char,
        18 => return b"\\x12\0" as *const u8 as *const ::core::ffi::c_char,
        19 => return b"\\x13\0" as *const u8 as *const ::core::ffi::c_char,
        20 => return b"\\x14\0" as *const u8 as *const ::core::ffi::c_char,
        21 => return b"\\x15\0" as *const u8 as *const ::core::ffi::c_char,
        22 => return b"\\x16\0" as *const u8 as *const ::core::ffi::c_char,
        23 => return b"\\x17\0" as *const u8 as *const ::core::ffi::c_char,
        24 => return b"\\x18\0" as *const u8 as *const ::core::ffi::c_char,
        25 => return b"\\x19\0" as *const u8 as *const ::core::ffi::c_char,
        26 => return b"\\x1A\0" as *const u8 as *const ::core::ffi::c_char,
        27 => return b"\\x1B\0" as *const u8 as *const ::core::ffi::c_char,
        28 => return b"\\x1C\0" as *const u8 as *const ::core::ffi::c_char,
        29 => return b"\\x1D\0" as *const u8 as *const ::core::ffi::c_char,
        30 => return b"\\x1E\0" as *const u8 as *const ::core::ffi::c_char,
        31 => return b"\\x1F\0" as *const u8 as *const ::core::ffi::c_char,
        32 => return b" \0" as *const u8 as *const ::core::ffi::c_char,
        33 => return b"!\0" as *const u8 as *const ::core::ffi::c_char,
        34 => return b"\\\"\0" as *const u8 as *const ::core::ffi::c_char,
        35 => return b"#\0" as *const u8 as *const ::core::ffi::c_char,
        36 => return b"$\0" as *const u8 as *const ::core::ffi::c_char,
        37 => return b"%\0" as *const u8 as *const ::core::ffi::c_char,
        38 => return b"&\0" as *const u8 as *const ::core::ffi::c_char,
        39 => return b"'\0" as *const u8 as *const ::core::ffi::c_char,
        40 => return b"(\0" as *const u8 as *const ::core::ffi::c_char,
        41 => return b")\0" as *const u8 as *const ::core::ffi::c_char,
        42 => return b"*\0" as *const u8 as *const ::core::ffi::c_char,
        43 => return b"+\0" as *const u8 as *const ::core::ffi::c_char,
        44 => return b",\0" as *const u8 as *const ::core::ffi::c_char,
        45 => return b"-\0" as *const u8 as *const ::core::ffi::c_char,
        46 => return b".\0" as *const u8 as *const ::core::ffi::c_char,
        47 => return b"/\0" as *const u8 as *const ::core::ffi::c_char,
        48 => return b"0\0" as *const u8 as *const ::core::ffi::c_char,
        49 => return b"1\0" as *const u8 as *const ::core::ffi::c_char,
        50 => return b"2\0" as *const u8 as *const ::core::ffi::c_char,
        51 => return b"3\0" as *const u8 as *const ::core::ffi::c_char,
        52 => return b"4\0" as *const u8 as *const ::core::ffi::c_char,
        53 => return b"5\0" as *const u8 as *const ::core::ffi::c_char,
        54 => return b"6\0" as *const u8 as *const ::core::ffi::c_char,
        55 => return b"7\0" as *const u8 as *const ::core::ffi::c_char,
        56 => return b"8\0" as *const u8 as *const ::core::ffi::c_char,
        57 => return b"9\0" as *const u8 as *const ::core::ffi::c_char,
        58 => return b":\0" as *const u8 as *const ::core::ffi::c_char,
        59 => return b";\0" as *const u8 as *const ::core::ffi::c_char,
        60 => return b"<\0" as *const u8 as *const ::core::ffi::c_char,
        61 => return b"=\0" as *const u8 as *const ::core::ffi::c_char,
        62 => return b">\0" as *const u8 as *const ::core::ffi::c_char,
        63 => return b"?\0" as *const u8 as *const ::core::ffi::c_char,
        64 => return b"@\0" as *const u8 as *const ::core::ffi::c_char,
        65 => return b"A\0" as *const u8 as *const ::core::ffi::c_char,
        66 => return b"B\0" as *const u8 as *const ::core::ffi::c_char,
        67 => return b"C\0" as *const u8 as *const ::core::ffi::c_char,
        68 => return b"D\0" as *const u8 as *const ::core::ffi::c_char,
        69 => return b"E\0" as *const u8 as *const ::core::ffi::c_char,
        70 => return b"F\0" as *const u8 as *const ::core::ffi::c_char,
        71 => return b"G\0" as *const u8 as *const ::core::ffi::c_char,
        72 => return b"H\0" as *const u8 as *const ::core::ffi::c_char,
        73 => return b"I\0" as *const u8 as *const ::core::ffi::c_char,
        74 => return b"J\0" as *const u8 as *const ::core::ffi::c_char,
        75 => return b"K\0" as *const u8 as *const ::core::ffi::c_char,
        76 => return b"L\0" as *const u8 as *const ::core::ffi::c_char,
        77 => return b"M\0" as *const u8 as *const ::core::ffi::c_char,
        78 => return b"N\0" as *const u8 as *const ::core::ffi::c_char,
        79 => return b"O\0" as *const u8 as *const ::core::ffi::c_char,
        80 => return b"P\0" as *const u8 as *const ::core::ffi::c_char,
        81 => return b"Q\0" as *const u8 as *const ::core::ffi::c_char,
        82 => return b"R\0" as *const u8 as *const ::core::ffi::c_char,
        83 => return b"S\0" as *const u8 as *const ::core::ffi::c_char,
        84 => return b"T\0" as *const u8 as *const ::core::ffi::c_char,
        85 => return b"U\0" as *const u8 as *const ::core::ffi::c_char,
        86 => return b"V\0" as *const u8 as *const ::core::ffi::c_char,
        87 => return b"W\0" as *const u8 as *const ::core::ffi::c_char,
        88 => return b"X\0" as *const u8 as *const ::core::ffi::c_char,
        89 => return b"Y\0" as *const u8 as *const ::core::ffi::c_char,
        90 => return b"Z\0" as *const u8 as *const ::core::ffi::c_char,
        91 => return b"[\0" as *const u8 as *const ::core::ffi::c_char,
        92 => return b"\\\\\0" as *const u8 as *const ::core::ffi::c_char,
        93 => return b"]\0" as *const u8 as *const ::core::ffi::c_char,
        94 => return b"^\0" as *const u8 as *const ::core::ffi::c_char,
        95 => return b"_\0" as *const u8 as *const ::core::ffi::c_char,
        96 => return b"`\0" as *const u8 as *const ::core::ffi::c_char,
        97 => return b"a\0" as *const u8 as *const ::core::ffi::c_char,
        98 => return b"b\0" as *const u8 as *const ::core::ffi::c_char,
        99 => return b"c\0" as *const u8 as *const ::core::ffi::c_char,
        100 => return b"d\0" as *const u8 as *const ::core::ffi::c_char,
        101 => return b"e\0" as *const u8 as *const ::core::ffi::c_char,
        102 => return b"f\0" as *const u8 as *const ::core::ffi::c_char,
        103 => return b"g\0" as *const u8 as *const ::core::ffi::c_char,
        104 => return b"h\0" as *const u8 as *const ::core::ffi::c_char,
        105 => return b"i\0" as *const u8 as *const ::core::ffi::c_char,
        106 => return b"j\0" as *const u8 as *const ::core::ffi::c_char,
        107 => return b"k\0" as *const u8 as *const ::core::ffi::c_char,
        108 => return b"l\0" as *const u8 as *const ::core::ffi::c_char,
        109 => return b"m\0" as *const u8 as *const ::core::ffi::c_char,
        110 => return b"n\0" as *const u8 as *const ::core::ffi::c_char,
        111 => return b"o\0" as *const u8 as *const ::core::ffi::c_char,
        112 => return b"p\0" as *const u8 as *const ::core::ffi::c_char,
        113 => return b"q\0" as *const u8 as *const ::core::ffi::c_char,
        114 => return b"r\0" as *const u8 as *const ::core::ffi::c_char,
        115 => return b"s\0" as *const u8 as *const ::core::ffi::c_char,
        116 => return b"t\0" as *const u8 as *const ::core::ffi::c_char,
        117 => return b"u\0" as *const u8 as *const ::core::ffi::c_char,
        118 => return b"v\0" as *const u8 as *const ::core::ffi::c_char,
        119 => return b"w\0" as *const u8 as *const ::core::ffi::c_char,
        120 => return b"x\0" as *const u8 as *const ::core::ffi::c_char,
        121 => return b"y\0" as *const u8 as *const ::core::ffi::c_char,
        122 => return b"z\0" as *const u8 as *const ::core::ffi::c_char,
        123 => return b"{\0" as *const u8 as *const ::core::ffi::c_char,
        124 => return b"|\0" as *const u8 as *const ::core::ffi::c_char,
        125 => return b"}\0" as *const u8 as *const ::core::ffi::c_char,
        126 => return b"~\0" as *const u8 as *const ::core::ffi::c_char,
        127 => return b"\\x7F\0" as *const u8 as *const ::core::ffi::c_char,
        128 => return b"\\x80\0" as *const u8 as *const ::core::ffi::c_char,
        129 => return b"\\x81\0" as *const u8 as *const ::core::ffi::c_char,
        130 => return b"\\x82\0" as *const u8 as *const ::core::ffi::c_char,
        131 => return b"\\x83\0" as *const u8 as *const ::core::ffi::c_char,
        132 => return b"\\x84\0" as *const u8 as *const ::core::ffi::c_char,
        133 => return b"\\x85\0" as *const u8 as *const ::core::ffi::c_char,
        134 => return b"\\x86\0" as *const u8 as *const ::core::ffi::c_char,
        135 => return b"\\x87\0" as *const u8 as *const ::core::ffi::c_char,
        136 => return b"\\x88\0" as *const u8 as *const ::core::ffi::c_char,
        137 => return b"\\x89\0" as *const u8 as *const ::core::ffi::c_char,
        138 => return b"\\x8A\0" as *const u8 as *const ::core::ffi::c_char,
        139 => return b"\\x8B\0" as *const u8 as *const ::core::ffi::c_char,
        140 => return b"\\x8C\0" as *const u8 as *const ::core::ffi::c_char,
        141 => return b"\\x8D\0" as *const u8 as *const ::core::ffi::c_char,
        142 => return b"\\x8E\0" as *const u8 as *const ::core::ffi::c_char,
        143 => return b"\\x8F\0" as *const u8 as *const ::core::ffi::c_char,
        144 => return b"\\x90\0" as *const u8 as *const ::core::ffi::c_char,
        145 => return b"\\x91\0" as *const u8 as *const ::core::ffi::c_char,
        146 => return b"\\x92\0" as *const u8 as *const ::core::ffi::c_char,
        147 => return b"\\x93\0" as *const u8 as *const ::core::ffi::c_char,
        148 => return b"\\x94\0" as *const u8 as *const ::core::ffi::c_char,
        149 => return b"\\x95\0" as *const u8 as *const ::core::ffi::c_char,
        150 => return b"\\x96\0" as *const u8 as *const ::core::ffi::c_char,
        151 => return b"\\x97\0" as *const u8 as *const ::core::ffi::c_char,
        152 => return b"\\x98\0" as *const u8 as *const ::core::ffi::c_char,
        153 => return b"\\x99\0" as *const u8 as *const ::core::ffi::c_char,
        154 => return b"\\x9A\0" as *const u8 as *const ::core::ffi::c_char,
        155 => return b"\\x9B\0" as *const u8 as *const ::core::ffi::c_char,
        156 => return b"\\x9C\0" as *const u8 as *const ::core::ffi::c_char,
        157 => return b"\\x9D\0" as *const u8 as *const ::core::ffi::c_char,
        158 => return b"\\x9E\0" as *const u8 as *const ::core::ffi::c_char,
        159 => return b"\\x9F\0" as *const u8 as *const ::core::ffi::c_char,
        160 => return b"\\xA0\0" as *const u8 as *const ::core::ffi::c_char,
        161 => return b"\\xA1\0" as *const u8 as *const ::core::ffi::c_char,
        162 => return b"\\xA2\0" as *const u8 as *const ::core::ffi::c_char,
        163 => return b"\\xA3\0" as *const u8 as *const ::core::ffi::c_char,
        164 => return b"\\xA4\0" as *const u8 as *const ::core::ffi::c_char,
        165 => return b"\\xA5\0" as *const u8 as *const ::core::ffi::c_char,
        166 => return b"\\xA6\0" as *const u8 as *const ::core::ffi::c_char,
        167 => return b"\\xA7\0" as *const u8 as *const ::core::ffi::c_char,
        168 => return b"\\xA8\0" as *const u8 as *const ::core::ffi::c_char,
        169 => return b"\\xA9\0" as *const u8 as *const ::core::ffi::c_char,
        170 => return b"\\xAA\0" as *const u8 as *const ::core::ffi::c_char,
        171 => return b"\\xAB\0" as *const u8 as *const ::core::ffi::c_char,
        172 => return b"\\xAC\0" as *const u8 as *const ::core::ffi::c_char,
        173 => return b"\\xAD\0" as *const u8 as *const ::core::ffi::c_char,
        174 => return b"\\xAE\0" as *const u8 as *const ::core::ffi::c_char,
        175 => return b"\\xAF\0" as *const u8 as *const ::core::ffi::c_char,
        176 => return b"\\xB0\0" as *const u8 as *const ::core::ffi::c_char,
        177 => return b"\\xB1\0" as *const u8 as *const ::core::ffi::c_char,
        178 => return b"\\xB2\0" as *const u8 as *const ::core::ffi::c_char,
        179 => return b"\\xB3\0" as *const u8 as *const ::core::ffi::c_char,
        180 => return b"\\xB4\0" as *const u8 as *const ::core::ffi::c_char,
        181 => return b"\\xB5\0" as *const u8 as *const ::core::ffi::c_char,
        182 => return b"\\xB6\0" as *const u8 as *const ::core::ffi::c_char,
        183 => return b"\\xB7\0" as *const u8 as *const ::core::ffi::c_char,
        184 => return b"\\xB8\0" as *const u8 as *const ::core::ffi::c_char,
        185 => return b"\\xB9\0" as *const u8 as *const ::core::ffi::c_char,
        186 => return b"\\xBA\0" as *const u8 as *const ::core::ffi::c_char,
        187 => return b"\\xBB\0" as *const u8 as *const ::core::ffi::c_char,
        188 => return b"\\xBC\0" as *const u8 as *const ::core::ffi::c_char,
        189 => return b"\\xBD\0" as *const u8 as *const ::core::ffi::c_char,
        190 => return b"\\xBE\0" as *const u8 as *const ::core::ffi::c_char,
        191 => return b"\\xBF\0" as *const u8 as *const ::core::ffi::c_char,
        192 => return b"\\xC0\0" as *const u8 as *const ::core::ffi::c_char,
        193 => return b"\\xC1\0" as *const u8 as *const ::core::ffi::c_char,
        194 => return b"\\xC2\0" as *const u8 as *const ::core::ffi::c_char,
        195 => return b"\\xC3\0" as *const u8 as *const ::core::ffi::c_char,
        196 => return b"\\xC4\0" as *const u8 as *const ::core::ffi::c_char,
        197 => return b"\\xC5\0" as *const u8 as *const ::core::ffi::c_char,
        198 => return b"\\xC6\0" as *const u8 as *const ::core::ffi::c_char,
        199 => return b"\\xC7\0" as *const u8 as *const ::core::ffi::c_char,
        200 => return b"\\xC8\0" as *const u8 as *const ::core::ffi::c_char,
        201 => return b"\\xC9\0" as *const u8 as *const ::core::ffi::c_char,
        202 => return b"\\xCA\0" as *const u8 as *const ::core::ffi::c_char,
        203 => return b"\\xCB\0" as *const u8 as *const ::core::ffi::c_char,
        204 => return b"\\xCC\0" as *const u8 as *const ::core::ffi::c_char,
        205 => return b"\\xCD\0" as *const u8 as *const ::core::ffi::c_char,
        206 => return b"\\xCE\0" as *const u8 as *const ::core::ffi::c_char,
        207 => return b"\\xCF\0" as *const u8 as *const ::core::ffi::c_char,
        208 => return b"\\xD0\0" as *const u8 as *const ::core::ffi::c_char,
        209 => return b"\\xD1\0" as *const u8 as *const ::core::ffi::c_char,
        210 => return b"\\xD2\0" as *const u8 as *const ::core::ffi::c_char,
        211 => return b"\\xD3\0" as *const u8 as *const ::core::ffi::c_char,
        212 => return b"\\xD4\0" as *const u8 as *const ::core::ffi::c_char,
        213 => return b"\\xD5\0" as *const u8 as *const ::core::ffi::c_char,
        214 => return b"\\xD6\0" as *const u8 as *const ::core::ffi::c_char,
        215 => return b"\\xD7\0" as *const u8 as *const ::core::ffi::c_char,
        216 => return b"\\xD8\0" as *const u8 as *const ::core::ffi::c_char,
        217 => return b"\\xD9\0" as *const u8 as *const ::core::ffi::c_char,
        218 => return b"\\xDA\0" as *const u8 as *const ::core::ffi::c_char,
        219 => return b"\\xDB\0" as *const u8 as *const ::core::ffi::c_char,
        220 => return b"\\xDC\0" as *const u8 as *const ::core::ffi::c_char,
        221 => return b"\\xDD\0" as *const u8 as *const ::core::ffi::c_char,
        222 => return b"\\xDE\0" as *const u8 as *const ::core::ffi::c_char,
        223 => return b"\\xDF\0" as *const u8 as *const ::core::ffi::c_char,
        224 => return b"\\xE0\0" as *const u8 as *const ::core::ffi::c_char,
        225 => return b"\\xE1\0" as *const u8 as *const ::core::ffi::c_char,
        226 => return b"\\xE2\0" as *const u8 as *const ::core::ffi::c_char,
        227 => return b"\\xE3\0" as *const u8 as *const ::core::ffi::c_char,
        228 => return b"\\xE4\0" as *const u8 as *const ::core::ffi::c_char,
        229 => return b"\\xE5\0" as *const u8 as *const ::core::ffi::c_char,
        230 => return b"\\xE6\0" as *const u8 as *const ::core::ffi::c_char,
        231 => return b"\\xE7\0" as *const u8 as *const ::core::ffi::c_char,
        232 => return b"\\xE8\0" as *const u8 as *const ::core::ffi::c_char,
        233 => return b"\\xE9\0" as *const u8 as *const ::core::ffi::c_char,
        234 => return b"\\xEA\0" as *const u8 as *const ::core::ffi::c_char,
        235 => return b"\\xEB\0" as *const u8 as *const ::core::ffi::c_char,
        236 => return b"\\xEC\0" as *const u8 as *const ::core::ffi::c_char,
        237 => return b"\\xED\0" as *const u8 as *const ::core::ffi::c_char,
        238 => return b"\\xEE\0" as *const u8 as *const ::core::ffi::c_char,
        239 => return b"\\xEF\0" as *const u8 as *const ::core::ffi::c_char,
        240 => return b"\\xF0\0" as *const u8 as *const ::core::ffi::c_char,
        241 => return b"\\xF1\0" as *const u8 as *const ::core::ffi::c_char,
        242 => return b"\\xF2\0" as *const u8 as *const ::core::ffi::c_char,
        243 => return b"\\xF3\0" as *const u8 as *const ::core::ffi::c_char,
        244 => return b"\\xF4\0" as *const u8 as *const ::core::ffi::c_char,
        245 => return b"\\xF5\0" as *const u8 as *const ::core::ffi::c_char,
        246 => return b"\\xF6\0" as *const u8 as *const ::core::ffi::c_char,
        247 => return b"\\xF7\0" as *const u8 as *const ::core::ffi::c_char,
        248 => return b"\\xF8\0" as *const u8 as *const ::core::ffi::c_char,
        249 => return b"\\xF9\0" as *const u8 as *const ::core::ffi::c_char,
        250 => return b"\\xFA\0" as *const u8 as *const ::core::ffi::c_char,
        251 => return b"\\xFB\0" as *const u8 as *const ::core::ffi::c_char,
        252 => return b"\\xFC\0" as *const u8 as *const ::core::ffi::c_char,
        253 => return b"\\xFD\0" as *const u8 as *const ::core::ffi::c_char,
        254 => return b"\\xFE\0" as *const u8 as *const ::core::ffi::c_char,
        255 => return b"\\xFF\0" as *const u8 as *const ::core::ffi::c_char,
        _ => {
            if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"unsignedCharToPrintable\0" as *const u8 as *const ::core::ffi::c_char,
                    b"xmlparse.c\0" as *const u8 as *const ::core::ffi::c_char,
                    9241 as ::core::ffi::c_int,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            return b"dead code\0" as *const u8 as *const ::core::ffi::c_char;
        }
    };
}
unsafe extern "C" fn getDebugLevel(
    mut variableName: *const ::core::ffi::c_char,
    mut defaultDebugLevel: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    let valueOrNull: *const ::core::ffi::c_char = getenv(variableName);
    if valueOrNull.is_null() {
        return defaultDebugLevel;
    }
    let value: *const ::core::ffi::c_char = valueOrNull;
    *__error() = 0 as ::core::ffi::c_int;
    let mut afterValue: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut debugLevel: ::core::ffi::c_ulong =
        strtoul(value, &raw mut afterValue, 10 as ::core::ffi::c_int);
    if *__error() != 0 as ::core::ffi::c_int
        || afterValue == value as *mut ::core::ffi::c_char
        || *afterValue.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        *__error() = 0 as ::core::ffi::c_int;
        return defaultDebugLevel;
    }
    return debugLevel;
}
pub const XML_CONTEXT_BYTES: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    xmlLen = (::core::mem::size_of::<[XML_Char; 37]>() as ::core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<XML_Char>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    xmlnsLen = (::core::mem::size_of::<[XML_Char; 30]>() as ::core::ffi::c_int as usize)
        .wrapping_div(::core::mem::size_of::<XML_Char>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
