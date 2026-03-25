//! Main XML parser module — the public API of expat-rust.
//!
//! Ported from expat's `xmlparse.c` with 1:1 function correspondence. Create a
//! [`Parser`] with [`Parser::new`], register SAX-style callbacks (e.g.,
//! [`Parser::set_start_element_handler`]), then feed data incrementally with
//! [`Parser::parse`]. Supports namespaces, DTD processing, external entity
//! resolution, and parser suspension/resumption.

use crate::xmlrole::{self, Role, XmlRoleState};
use crate::xmltok;
use crate::xmltok_impl::{self, Encoding, TokenResult, XmlTok};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

/// DTD state — shared between parent and child DTD parsers via Rc<RefCell<>>.
/// Matches C's `DTD` struct which is shared via `m_dtd` pointer.
/// Parameter entity record
#[derive(Debug, Clone, Default)]
pub struct ParamEntity {
    pub system_id: Option<String>,
    pub public_id: Option<String>,
    pub value: Option<String>,
    pub is_internal: bool,
    pub open: bool,
}

/// For DTD child parsers (empty context), parent and child share the same DtdState.
/// For content child parsers (non-empty context), DtdState is cloned for isolation.
#[derive(Debug, Clone)]
pub struct DtdState {
    pub internal_entities: HashMap<String, String>,
    pub external_entities: HashMap<String, (Option<String>, Option<String>)>,
    pub attlist_defaults: HashMap<String, Vec<(String, String)>>,
    pub attlist_types: HashMap<String, HashMap<String, String>>,
    pub unparsed_entities: HashSet<String>,
    pub param_entities: HashMap<String, ParamEntity>,
    pub has_param_entity_refs: bool,
    pub standalone: bool,
    pub keep_processing: bool,
    pub param_entity_read: bool,
}

impl Default for DtdState {
    fn default() -> Self {
        DtdState {
            internal_entities: HashMap::new(),
            external_entities: HashMap::new(),
            attlist_defaults: HashMap::new(),
            attlist_types: HashMap::new(),
            unparsed_entities: HashSet::new(),
            param_entities: HashMap::new(),
            has_param_entity_refs: false,
            standalone: false,
            keep_processing: true,
            param_entity_read: false,
        }
    }
}

// Type aliases for handler function types
type StartElementHandler = Box<dyn FnMut(&str, &[(&str, &str)]) + 'static>;
type EndElementHandler = Box<dyn FnMut(&str) + 'static>;
type CharacterDataHandler = Box<dyn FnMut(&[u8]) + 'static>;
type ProcessingInstructionHandler = Box<dyn FnMut(&str, &str) + 'static>;
type CommentHandler = Box<dyn FnMut(&[u8]) + 'static>;
type CdataSectionHandler = Box<dyn FnMut() + 'static>;
type DefaultHandler = Box<dyn FnMut(&[u8]) + 'static>;
type StartDoctypeDeclHandler = Box<dyn FnMut(&str, Option<&str>, Option<&str>, bool) + 'static>;
type EndDoctypeDeclHandler = Box<dyn FnMut() + 'static>;
type ElementDeclHandler = Box<dyn FnMut(&str, &str) + 'static>;
type AttlistDeclHandler =
    Box<dyn FnMut(&str, &str, &str, Option<&str>, Option<&str>, bool) + 'static>;
type XmlDeclHandler = Box<dyn FnMut(Option<&str>, Option<&str>, Option<i32>) + 'static>;
type EntityDeclHandler =
    Box<dyn FnMut(&str, bool, Option<&str>, Option<&str>, Option<&str>) + 'static>;
type UnparsedEntityDeclHandler = Box<dyn FnMut(&str, Option<&str>, &str, Option<&str>) + 'static>;
type NotationDeclHandler = Box<dyn FnMut(&str, Option<&str>, &str, Option<&str>) + 'static>;
type NamespaceDeclHandler = Box<dyn FnMut(Option<&str>, &str) + 'static>;
type NamespaceDeclEndHandler = Box<dyn FnMut(Option<&str>) + 'static>;
type NotStandaloneHandler = Box<dyn FnMut() -> bool + 'static>;
type ExternalEntityRefHandler =
    Box<dyn FnMut(&str, Option<&str>, Option<&str>, Option<&str>) -> bool + 'static>;
type SkippedEntityHandler = Box<dyn FnMut(&str, bool) + 'static>;
type UnknownEncodingHandler = Box<dyn FnMut(&str) -> bool + 'static>;

/// Status returned by [`Parser::parse`] and [`Parser::parse_buffer`].
///
/// Corresponds to `XML_Status` in the C API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlStatus {
    /// A fatal error was encountered. Call [`Parser::error_code`] for details.
    Error = 0,
    /// Parsing completed successfully (or more data is expected).
    Ok = 1,
    /// Parsing was suspended by a handler calling [`Parser::stop`].
    Suspended = 2,
}

/// Error codes describing why parsing failed.
///
/// Corresponds to `XML_Error` in the C API. Retrieve the current error with
/// [`Parser::error_code`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlError {
    None = 0,
    NoMemory = 1,
    Syntax = 2,
    NoElements = 3,
    InvalidToken = 4,
    UnclosedToken = 5,
    PartialChar = 6,
    TagMismatch = 7,
    DuplicateAttribute = 8,
    JunkAfterDocElement = 9,
    ParamEntityRef = 10,
    UndefinedEntity = 11,
    RecursiveEntityRef = 12,
    AsyncEntity = 13,
    BadCharRef = 14,
    BinaryEntityRef = 15,
    AttributeExternalEntityRef = 16,
    MisplacedXmlPi = 17,
    UnknownEncoding = 18,
    IncorrectEncoding = 19,
    UnclosedCdataSection = 20,
    ExternalEntityHandling = 21,
    NotStandalone = 22,
    UnexpectedState = 23,
    EntityDeclaredInPe = 24,
    FeatureRequiresXmlDtd = 25,
    CantChangeFeatureOnceParsing = 26,
    UnboundPrefix = 27,
    UndeclaringPrefix = 28,
    IncompletePe = 29,
    XmlDecl = 30,
    TextDecl = 31,
    Publicid = 32,
    Suspended = 33,
    NotSuspended = 34,
    Aborted = 35,
    Finished = 36,
    SuspendPe = 37,
    ReservedPrefixXml = 38,
    ReservedPrefixXmlns = 39,
    ReservedNamespaceUri = 40,
    InvalidArgument = 41,
    NoBuffer = 42,
    AmplificationLimitBreach = 43,
    NotStarted = 44,
}

impl XmlError {
    /// Get a human-readable error message for this error code
    pub fn message(&self) -> &'static str {
        error_string(*self)
    }
}

/// Content type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Empty = 1,
    Any = 2,
    Mixed = 3,
    Name = 4,
    Choice = 5,
    Seq = 6,
}

/// Content quantifier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentQuant {
    None = 0,
    Opt = 1,
    Rep = 2,
    Plus = 3,
}

/// Content model node — represents a node in the element declaration tree
#[derive(Debug, Clone)]
pub struct ContentNode {
    pub content_type: ContentType,
    pub quant: ContentQuant,
    pub children: Vec<ContentNode>,
    pub name: Option<String>,
}

/// Parameter entity parsing mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamEntityParsing {
    Never = 0,
    UnlessStandalone = 1,
    Always = 2,
}

/// Parser state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingState {
    Initialized,
    Parsing,
    Finished,
    Suspended,
}

/// Internal entity record — represents an open internal entity being processed
/// Corresponds to OPEN_INTERNAL_ENTITY in C (xmlparse.c)
#[derive(Clone, Debug)]
struct OpenInternalEntity {
    /// Entity being processed
    entity_name: String,
    /// Entity text (replacement value)
    entity_text: Vec<u8>,
    /// Tag level when entity was opened (for async entity detection)
    start_tag_level: u32,
    /// Number of bytes already processed in entity text
    processed: usize,
    /// Whether this is a parameter entity (uses doProlog) vs general entity (uses doContent)
    is_param: bool,
    /// WFC: PE Between Declarations — entity occurred between declarations
    #[allow(dead_code)]
    between_decl: bool,
    /// Previous processor to restore when entity is closed
    saved_processor: Processor,
    /// Whether entity still has content to process (C: entity->hasMore)
    /// First pass (true): process entity text. Second pass (false): cleanup.
    has_more: bool,
}

/// Processor type enumeration — idiomatic Rust alternative to C function pointers
///
/// The parser uses a processor-based architecture similar to the C implementation:
/// 1. PrologInit: Initial processor that detects encoding (maps to prologInitProcessor in C)
/// 2. Prolog: Processes XML declaration, DOCTYPE, comments, PIs (maps to prologProcessor in C)
/// 3. Content: Processes element content (maps to contentProcessor in C)
/// 4. InternalEntity: Processes internal entity content (maps to internalEntityProcessor in C)
/// 5. Epilog: Processes after root element closes (maps to epilogProcessor in C)
///
/// This design allows clean separation of concerns and is called through run_processor()
/// in the main parse loop, which dispatches to the appropriate processor method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Processor {
    /// Initial processor — detects encoding then calls PrologProcessor
    PrologInit,
    /// Processes XML declaration, DOCTYPE, comments, PIs before root element
    Prolog,
    /// Processes element content
    Content,
    /// Processes CDATA section content (resumed from interrupted CDATA)
    CdataSection,
    /// External entity — accepts optional text declaration then processes content
    ExternalEntity,
    /// External parameter entity — one-shot bootstrap for PE content
    ExternalParamEntity,
    /// Internal entity processor — processes entity content from the stack
    InternalEntity,
    /// Processes after root element closes
    Epilog,
}

/// Parser status information
#[derive(Debug, Clone, Copy)]
pub struct ParsingStatus {
    pub state: ParsingState,
    pub final_buffer: bool,
}

/// Version information structure
#[derive(Debug, Clone, Copy)]
pub struct ExpatVersion {
    pub major: i32,
    pub minor: i32,
    pub micro: i32,
}

/// Feature enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    End = 0,
    Unicode = 1,
    UnicodeWcharT = 2,
    Dtd = 3,
    ContextBytes = 4,
    MinSize = 5,
    SizeofXmlChar = 6,
    SizeofXmlLchar = 7,
    Ns = 8,
    LargeSize = 9,
    AttrInfo = 10,
    BillionLaughsAttackProtectionMaximumAmplificationDefault = 11,
    BillionLaughsAttackProtectionActivationThresholdDefault = 12,
    Ge = 13,
    AllocTrackerMaximumAmplificationDefault = 14,
    AllocTrackerActivationThresholdDefault = 15,
}

/// Attribute information structure
#[derive(Debug, Clone, Copy)]
pub struct AttrInfo {
    pub name_start: i64,
    pub name_end: i64,
    pub value_start: i64,
    pub value_end: i64,
}

/// A streaming XML parser with SAX-style callback handlers.
///
/// Create with [`Parser::new`] (or [`Parser::new_ns`] for namespace support),
/// register handlers, then call [`Parser::parse`] one or more times.
/// Corresponds to `XML_Parser` in the C API.
pub struct Parser {
    /// Parse buffer for incremental parsing
    buffer: Vec<u8>,
    /// Buffer for XML_GetBuffer/XML_ParseBuffer two-phase API
    get_buffer_data: Vec<u8>,
    /// Data remaining when parser was suspended (for resume)
    suspended_data: Vec<u8>,
    /// Whether the suspended parse was final
    suspended_is_final: bool,
    /// Current error code
    error_code: XmlError,
    /// Parsing state machine
    parsing_state: ParsingState,
    /// Current processor — similar to m_processor in C
    processor: Processor,
    /// Accumulated position from previous parse calls (line count)
    line_number: u64,
    /// Accumulated position from previous parse calls (column count)
    column_number: u64,
    /// Raw data for current parse call — used for lazy position calculation
    /// Corresponds to C's m_positionPtr / m_eventPtr pattern
    parse_data: Vec<u8>,
    /// Position in parse_data up to which we've calculated line/column
    /// Corresponds to C's m_positionPtr
    position_pos: usize,
    /// Position in parse_data of the current event (for error reporting)
    /// Corresponds to C's m_eventPtr
    event_pos: usize,
    /// Is this the final buffer?
    is_final: bool,
    /// Declared encoding name (set via XML_SetEncoding or constructor)
    encoding_name: Option<String>,
    /// Whether encoding_name was set via XML_SetEncoding (protocol encoding)
    /// If true, conflicts with XML declaration encoding are ignored
    protocol_encoding_set: bool,
    /// Enable namespace processing
    #[allow(dead_code)]
    ns_enabled: bool,
    /// Namespace separator character
    #[allow(dead_code)]
    ns_separator: char,
    /// Element nesting depth
    tag_level: u32,
    /// Hash salt for DoS protection
    hash_salt: u64,
    /// Base URI for resolving relative URIs
    base_uri: Option<String>,
    /// Parameter entity parsing mode
    param_entity_parsing: ParamEntityParsing,
    /// Reparse deferral enabled flag
    reparse_deferral_enabled: bool,
    /// Stack of open tag names for mismatch detection
    tag_stack: Vec<String>,
    /// Stack of flags indicating whether each tag was opened with ns_triplets=true
    tag_triplet_flags: Vec<bool>,
    /// Whether we've seen the root element
    seen_root: bool,
    /// Whether the root element has been closed
    root_closed: bool,
    /// Whether we've seen an XML declaration
    seen_xml_decl: bool,
    /// Detected encoding from BOM/auto-detection
    detected_encoding: Option<String>,
    /// DTD state — shared between parent and child DTD parsers.
    /// Matches C's shared `m_dtd` pointer.
    pub dtd: Rc<RefCell<DtdState>>,
    /// Total original-encoding bytes consumed before the current parse() chunk.
    /// Incremented by data.len() at the start of each parse() call.
    original_bytes_before_chunk: u64,
    /// The original-encoding bytes of the current chunk (kept for byte-index re-scan)
    original_chunk: Vec<u8>,
    /// Length of current chunk's BOM that was stripped (for offset correction)
    original_chunk_bom_len: usize,
    /// Total byte offset in input (for tracking position across parse calls)
    byte_offset: u64,
    /// Number of bytes in the current event token (set during handler callbacks)
    event_cur_byte_count: i32,
    /// Raw bytes of the current event token (for XML_DefaultCurrent)
    event_cur_data: Vec<u8>,
    /// Pending byte from incomplete UTF-16 code unit across chunk boundaries
    utf16_pending_byte: Option<u8>,
    /// Pending bytes for custom encoding multi-byte sequences split across parse() calls
    custom_encoding_pending: Vec<u8>,
    /// Buffer for partial encoding detection (BOM bytes received across calls)
    encoding_detection_buf: Vec<u8>,
    /// Current ATTLIST element name being processed
    current_attlist_element: Option<String>,
    /// Current ATTLIST attribute name being processed
    current_attlist_attr: Option<String>,
    /// Set of currently open (being expanded) entities for recursion detection
    open_entities: std::collections::HashSet<String>,
    /// During entity expansion, the entity reference text (e.g., "&entity;") for XML_GetInputContext
    entity_reference_context: Option<Vec<u8>>,
    /// Current entity name being declared in DTD (for GeneralEntityName → EntityValue flow)
    current_entity_name: Option<String>,
    current_is_param_entity: bool,
    /// Current entity's system ID (for external entities)
    current_entity_system_id: Option<String>,
    /// Current entity's public ID (for external entities)
    current_entity_public_id: Option<String>,
    /// Current entity's notation (for unparsed entities with NDATA)
    current_entity_notation: Option<String>,
    /// DOCTYPE declaration state (accumulated across DoctypeName/SystemId/PublicId roles)
    doctype_name: Option<String>,
    doctype_system_id: Option<String>,
    doctype_public_id: Option<String>,
    /// Whether the start_doctype_decl_handler has been called for the current DOCTYPE
    doctype_handler_called: bool,
    /// Notation declaration tracking
    current_notation_name: Option<String>,
    current_notation_system_id: Option<String>,
    current_notation_public_id: Option<String>,
    /// Number of explicitly specified attributes in the most recent start element
    n_specified_atts: i32,
    /// Index of the ID-type attribute in the most recent start element (-1 if none)
    id_att_index: i32,
    /// Current ATTLIST attribute type being processed
    current_attlist_type: Option<String>,
    /// Whether to call external entity handler even without DOCTYPE
    foreign_dtd: bool,
    /// Whether this parser is parsing a foreign DTD subset (external entity with empty context)
    parsing_foreign_dtd: bool,
    /// Whether this parser is an external parameter entity parser (C: m_isParamEntity)
    is_param_entity: bool,
    /// Whether this parser is a subordinate (child) parser — cannot be reset
    pub is_subordinate: bool,
    /// Billion laughs: maximum amplification factor (0.0 = use default)
    billion_laughs_max_amplification: f32,
    /// Billion laughs: activation threshold in bytes (0 = use default)
    billion_laughs_activation_threshold: u64,
    /// XML role state machine for prolog parsing
    prolog_state: XmlRoleState,
    /// Current element declaration name being processed
    current_element_decl_name: Option<String>,
    /// Stack of content model groups being built (each is a ContentNode with children)
    content_model_stack: Vec<ContentNode>,
    /// Last serialized content model — a flat array representation for C FFI
    /// Format: Vec<(type_u32, quant_u32, name_opt, numchildren_u32)>
    #[allow(clippy::type_complexity)]
    last_content_model: Option<Vec<(u32, u32, Option<Vec<u8>>, u32)>>,
    /// Group connectors: 0=none, 1=comma/seq, 2=pipe/choice
    group_connectors: Vec<u8>,
    /// Namespace bindings stack: each entry is (element_level, prefix, uri, previous_uri)
    /// When an element closes, we pop all bindings at that level
    ns_bindings: Vec<(u32, String, String, Option<String>)>,
    /// Current namespace mapping: prefix → URI. "" key = default namespace.
    ns_map: HashMap<String, String>,
    /// Whether to return namespace triplets (uri + sep + localname + sep + prefix)
    ns_triplets: bool,
    /// For external entity parsers: content start_tag_level (1 for ext entities, 0 for main)
    /// This prevents do_content from returning NoElements for external entities
    content_start_tag_level: u32,
    /// Stack of open internal entities — corresponds to m_openInternalEntities in C
    open_internal_entities: Vec<OpenInternalEntity>,
    /// Reenter flag — signals run_processor to call processor again immediately.
    /// Matches C's m_reenter / triggerReenter mechanism.
    reenter: bool,

    // Handler fields
    start_element_handler: Option<StartElementHandler>,
    end_element_handler: Option<EndElementHandler>,
    character_data_handler: Option<CharacterDataHandler>,
    processing_instruction_handler: Option<ProcessingInstructionHandler>,
    comment_handler: Option<CommentHandler>,
    start_cdata_section_handler: Option<CdataSectionHandler>,
    end_cdata_section_handler: Option<CdataSectionHandler>,
    default_handler: Option<DefaultHandler>,
    default_handler_expand: Option<DefaultHandler>,
    start_doctype_decl_handler: Option<StartDoctypeDeclHandler>,
    end_doctype_decl_handler: Option<EndDoctypeDeclHandler>,
    element_decl_handler: Option<ElementDeclHandler>,
    attlist_decl_handler: Option<AttlistDeclHandler>,
    xml_decl_handler: Option<XmlDeclHandler>,
    entity_decl_handler: Option<EntityDeclHandler>,
    unparsed_entity_decl_handler: Option<UnparsedEntityDeclHandler>,
    notation_decl_handler: Option<NotationDeclHandler>,
    start_namespace_decl_handler: Option<NamespaceDeclHandler>,
    end_namespace_decl_handler: Option<NamespaceDeclEndHandler>,
    not_standalone_handler: Option<NotStandaloneHandler>,
    external_entity_ref_handler: Option<ExternalEntityRefHandler>,
    skipped_entity_handler: Option<SkippedEntityHandler>,
    unknown_encoding_handler: Option<UnknownEncodingHandler>,
    /// Custom encoding map from unknown encoding handler (for transcode of non-UTF-8 content)
    pub custom_encoding_map: Option<[i32; 256]>,
    /// Custom encoding converter function pointer
    pub custom_encoding_converter: Option<
        unsafe extern "C" fn(*mut std::ffi::c_void, *const std::ffi::c_char) -> std::ffi::c_int,
    >,
    /// User data for custom encoding converter
    pub custom_encoding_data: *mut std::ffi::c_void,
}

impl Parser {
    /// Create a new XML parser, optionally specifying the document encoding.
    ///
    /// Pass `None` to auto-detect encoding from the XML declaration or BOM.
    /// Returns `None` only if the encoding name is unsupported.
    /// Equivalent to `XML_ParserCreate` in the C API.
    pub fn new(encoding: Option<&str>) -> Option<Parser> {
        Some(Parser {
            buffer: Vec::new(),
            get_buffer_data: Vec::new(),
            suspended_data: Vec::new(),
            suspended_is_final: false,
            error_code: XmlError::None,
            parsing_state: ParsingState::Initialized,
            processor: Processor::PrologInit,
            line_number: 1,
            column_number: 0,
            parse_data: Vec::new(),
            position_pos: 0,
            event_pos: 0,
            is_final: false,
            encoding_name: encoding.map(|s| s.to_string()),
            protocol_encoding_set: false,
            ns_enabled: false,
            ns_separator: '\0',
            tag_level: 0,
            hash_salt: 0,
            base_uri: None,
            param_entity_parsing: ParamEntityParsing::Never,
            reparse_deferral_enabled: false,
            tag_stack: Vec::new(),
            tag_triplet_flags: Vec::new(),
            seen_root: false,
            root_closed: false,
            seen_xml_decl: false,
            detected_encoding: None,
            dtd: Rc::new(RefCell::new(DtdState::default())),
            original_bytes_before_chunk: 0,
            original_chunk: Vec::new(),
            original_chunk_bom_len: 0,
            byte_offset: 0,
            event_cur_byte_count: 0,
            event_cur_data: Vec::new(),
            utf16_pending_byte: None,
            custom_encoding_pending: Vec::new(),
            encoding_detection_buf: Vec::new(),
            current_attlist_element: None,
            current_attlist_attr: None,
            open_entities: std::collections::HashSet::new(),
            entity_reference_context: None,
            current_entity_name: None,
            current_is_param_entity: false,
            current_entity_system_id: None,
            current_entity_public_id: None,
            current_entity_notation: None,
            doctype_name: None,
            doctype_system_id: None,
            doctype_public_id: None,
            doctype_handler_called: false,
            current_notation_name: None,
            current_notation_system_id: None,
            current_notation_public_id: None,
            n_specified_atts: 0,
            id_att_index: -1,
            current_attlist_type: None,
            foreign_dtd: false,
            parsing_foreign_dtd: false,
            is_param_entity: false,
            is_subordinate: false,
            billion_laughs_max_amplification: 0.0,
            billion_laughs_activation_threshold: 0,
            prolog_state: XmlRoleState::new(),
            current_element_decl_name: None,
            content_model_stack: Vec::new(),
            last_content_model: None,
            group_connectors: Vec::new(),
            ns_bindings: Vec::new(),
            ns_map: HashMap::new(),
            ns_triplets: false,
            content_start_tag_level: 0,
            open_internal_entities: Vec::new(),
            reenter: false,
            start_element_handler: None,
            end_element_handler: None,
            character_data_handler: None,
            processing_instruction_handler: None,
            comment_handler: None,
            start_cdata_section_handler: None,
            end_cdata_section_handler: None,
            default_handler: None,
            default_handler_expand: None,
            start_doctype_decl_handler: None,
            end_doctype_decl_handler: None,
            element_decl_handler: None,
            attlist_decl_handler: None,
            xml_decl_handler: None,
            entity_decl_handler: None,
            unparsed_entity_decl_handler: None,
            notation_decl_handler: None,
            start_namespace_decl_handler: None,
            end_namespace_decl_handler: None,
            not_standalone_handler: None,
            external_entity_ref_handler: None,
            skipped_entity_handler: None,
            unknown_encoding_handler: None,
            custom_encoding_map: None,
            custom_encoding_converter: None,
            custom_encoding_data: std::ptr::null_mut(),
        })
    }

    /// Create a new parser with namespace processing
    ///
    /// Equivalent to XML_ParserCreateNS(encoding, sep) in C
    pub fn new_ns(encoding: Option<&str>, separator: char) -> Option<Parser> {
        let mut ns_map = HashMap::new();
        // "xml" prefix is implicitly bound to the XML namespace
        ns_map.insert(
            "xml".to_string(),
            "http://www.w3.org/XML/1998/namespace".to_string(),
        );

        Some(Parser {
            buffer: Vec::new(),
            get_buffer_data: Vec::new(),
            suspended_data: Vec::new(),
            suspended_is_final: false,
            error_code: XmlError::None,
            parsing_state: ParsingState::Initialized,
            processor: Processor::PrologInit,
            line_number: 1,
            column_number: 0,
            parse_data: Vec::new(),
            position_pos: 0,
            event_pos: 0,
            is_final: false,
            encoding_name: encoding.map(|s| s.to_string()),
            protocol_encoding_set: false,
            ns_enabled: true,
            ns_separator: separator,
            tag_level: 0,
            hash_salt: 0,
            base_uri: None,
            param_entity_parsing: ParamEntityParsing::Never,
            reparse_deferral_enabled: false,
            tag_stack: Vec::new(),
            tag_triplet_flags: Vec::new(),
            seen_root: false,
            root_closed: false,
            seen_xml_decl: false,
            detected_encoding: None,
            dtd: Rc::new(RefCell::new(DtdState::default())),
            original_bytes_before_chunk: 0,
            original_chunk: Vec::new(),
            original_chunk_bom_len: 0,
            byte_offset: 0,
            event_cur_byte_count: 0,
            event_cur_data: Vec::new(),
            utf16_pending_byte: None,
            custom_encoding_pending: Vec::new(),
            encoding_detection_buf: Vec::new(),
            current_attlist_element: None,
            current_attlist_attr: None,
            open_entities: std::collections::HashSet::new(),
            entity_reference_context: None,
            current_entity_name: None,
            current_is_param_entity: false,
            current_entity_system_id: None,
            current_entity_public_id: None,
            current_entity_notation: None,
            doctype_name: None,
            doctype_system_id: None,
            doctype_public_id: None,
            doctype_handler_called: false,
            current_notation_name: None,
            current_notation_system_id: None,
            current_notation_public_id: None,
            n_specified_atts: 0,
            id_att_index: -1,
            current_attlist_type: None,
            foreign_dtd: false,
            parsing_foreign_dtd: false,
            is_param_entity: false,
            is_subordinate: false,
            billion_laughs_max_amplification: 0.0,
            billion_laughs_activation_threshold: 0,
            prolog_state: XmlRoleState::new(),
            current_element_decl_name: None,
            content_model_stack: Vec::new(),
            last_content_model: None,
            group_connectors: Vec::new(),
            ns_bindings: Vec::new(),
            ns_map,
            ns_triplets: false,
            content_start_tag_level: 0,
            open_internal_entities: Vec::new(),
            reenter: false,
            start_element_handler: None,
            end_element_handler: None,
            character_data_handler: None,
            processing_instruction_handler: None,
            comment_handler: None,
            start_cdata_section_handler: None,
            end_cdata_section_handler: None,
            default_handler: None,
            default_handler_expand: None,
            start_doctype_decl_handler: None,
            end_doctype_decl_handler: None,
            element_decl_handler: None,
            attlist_decl_handler: None,
            xml_decl_handler: None,
            entity_decl_handler: None,
            unparsed_entity_decl_handler: None,
            notation_decl_handler: None,
            start_namespace_decl_handler: None,
            end_namespace_decl_handler: None,
            not_standalone_handler: None,
            external_entity_ref_handler: None,
            skipped_entity_handler: None,
            unknown_encoding_handler: None,
            custom_encoding_map: None,
            custom_encoding_converter: None,
            custom_encoding_data: std::ptr::null_mut(),
        })
    }

    /// Reset the parser for reuse
    ///
    /// Equivalent to XML_ParserReset(parser, encoding) in C
    pub fn reset(&mut self, encoding: Option<&str>) -> bool {
        // C: Cannot reset a subordinate (child) parser
        if self.is_subordinate {
            return false;
        }
        self.buffer.clear();
        self.error_code = XmlError::None;
        self.parsing_state = ParsingState::Initialized;
        self.processor = Processor::PrologInit;
        self.line_number = 1;
        self.column_number = 0;
        self.parse_data.clear();
        self.position_pos = 0;
        self.event_pos = 0;
        self.is_final = false;
        self.encoding_name = encoding.map(|s| s.to_string());
        self.protocol_encoding_set = false;
        self.tag_level = 0;
        self.tag_stack.clear();
        self.seen_root = false;
        self.root_closed = false;
        self.seen_xml_decl = false;
        self.detected_encoding = None;
        self.original_bytes_before_chunk = 0;
        self.original_chunk.clear();
        self.original_chunk_bom_len = 0;
        self.encoding_detection_buf.clear();
        self.byte_offset = 0;
        self.event_cur_byte_count = 0;
        self.dtd = Rc::new(RefCell::new(DtdState::default()));
        self.open_entities.clear();
        self.get_buffer_data.clear();
        self.suspended_data.clear();
        self.suspended_is_final = false;
        self.parsing_foreign_dtd = false;
        self.prolog_state = XmlRoleState::new();
        self.current_element_decl_name = None;
        self.content_model_stack.clear();
        self.group_connectors.clear();
        // Reset namespace state
        self.ns_bindings.clear();
        self.ns_map.clear();
        if self.ns_enabled {
            // Re-initialize implicit "xml" binding for namespace-enabled parsers
            self.ns_map.insert(
                "xml".to_string(),
                "http://www.w3.org/XML/1998/namespace".to_string(),
            );
        }
        self.ns_triplets = false;
        self.content_start_tag_level = 0;
        // Clear all handlers (matches C parserInit behavior)
        self.start_element_handler = None;
        self.end_element_handler = None;
        self.character_data_handler = None;
        self.processing_instruction_handler = None;
        self.comment_handler = None;
        self.start_cdata_section_handler = None;
        self.end_cdata_section_handler = None;
        self.default_handler = None;
        self.default_handler_expand = None;
        self.start_doctype_decl_handler = None;
        self.end_doctype_decl_handler = None;
        self.element_decl_handler = None;
        self.attlist_decl_handler = None;
        self.xml_decl_handler = None;
        self.entity_decl_handler = None;
        self.unparsed_entity_decl_handler = None;
        self.notation_decl_handler = None;
        self.start_namespace_decl_handler = None;
        self.end_namespace_decl_handler = None;
        self.not_standalone_handler = None;
        self.external_entity_ref_handler = None;
        self.skipped_entity_handler = None;
        self.unknown_encoding_handler = None;
        self.custom_encoding_map = None;
        self.custom_encoding_converter = None;
        self.custom_encoding_data = std::ptr::null_mut();
        true
    }

    /// Run the current processor on the buffered data
    fn run_processor(&mut self) {
        // Take buffer once — matches C where parse() passes data to the processor
        let data = std::mem::take(&mut self.buffer);
        if data.is_empty() {
            if !self.is_final {
                return; // Non-final empty buffer — nothing to process
            }
            // Final empty buffer — processors need to detect unclosed constructs.
            // Only dispatch to processors that handle empty final buffers correctly.
            match self.processor {
                Processor::CdataSection => {
                    // CDATA needs to report UnclosedCdataSection
                    let have_more = false;
                    let enc = xmltok::Utf8Encoding;
                    let (error, _) = self.do_cdata_section(&enc, &data, 0, 0, have_more);
                    if error != XmlError::None {
                        self.error_code = error;
                    }
                }
                Processor::Content => {
                    if !self.seen_root && self.content_start_tag_level == 0 {
                        self.error_code = XmlError::NoElements;
                    }
                }
                _ => {}
            }
            return;
        }

        // Dispatch to processor — may loop if processor transitions
        let mut start = 0usize;
        let end = data.len();
        loop {
            let prev_processor = self.processor;
            let (error, next_pos) = match self.processor {
                Processor::PrologInit => {
                    self.processor = Processor::Prolog;
                    continue; // Just transition, don't consume data
                }
                Processor::Prolog => {
                    let have_more = !self.is_final;
                    let enc = xmltok::Utf8Encoding;
                    self.do_prolog(&enc, &data, start, end, have_more)
                }
                Processor::Content => {
                    let have_more = !self.is_final;
                    let enc = xmltok::Utf8Encoding;
                    let stl = self.content_start_tag_level;
                    self.do_content(stl, &enc, &data, start, end, have_more)
                }
                Processor::CdataSection => {
                    // CDATA section resumed — delegate to do_cdata_section
                    let have_more = !self.is_final;
                    let enc = xmltok::Utf8Encoding;
                    self.do_cdata_section(&enc, &data, start, end, have_more)
                }
                Processor::ExternalEntity => {
                    self.external_entity_init_processor3(&data, start, end)
                }
                Processor::ExternalParamEntity => {
                    self.external_par_ent_processor(&data, start, end)
                }
                Processor::InternalEntity => self.internal_entity_processor(&data, start, end),
                Processor::Epilog => {
                    // Epilog — process post-root content
                    // epilog_processor is still old-style, convert inline
                    self.buffer = data[start..end].to_vec();
                    self.epilog_processor();
                    return; // epilog handles buffer internally for now
                }
            };

            if error != XmlError::None {
                self.error_code = error;
                return;
            }

            // C: callProcessor (xmlparse.c:1299-1307)
            // Make parsing status (and in particular Suspended) take
            // precedence over re-enter flag when they disagree
            if self.parsing_state != ParsingState::Parsing {
                self.reenter = false;
            }

            // If reenter flag is set, clear it and loop to re-dispatch.
            if self.reenter {
                self.reenter = false;
                start = next_pos;
                continue;
            }

            // If processor changed, re-dispatch with remaining data
            if self.processor != prev_processor && next_pos < end {
                // If transitioning from Prolog and need to transcode, do it now
                let mut transcoded_data = None;

                // Check if we need to transcode custom encoding
                if prev_processor == Processor::Prolog && self.custom_encoding_map.is_some() {
                    let remaining = &data[next_pos..end];
                    match self.transcode_custom_encoding(remaining) {
                        Ok(t) => {
                            transcoded_data = Some(t);
                        }
                        Err(err) => {
                            self.error_code = err;
                            return;
                        }
                    }
                } else if prev_processor == Processor::Prolog
                    && is_latin1_encoding(self.detected_encoding.as_deref())
                {
                    let remaining = &data[next_pos..end];
                    transcoded_data = Some(transcode_latin1_to_utf8(remaining));
                }

                if let Some(new_data) = transcoded_data {
                    // Put transcoded data in buffer for next iteration
                    self.buffer = new_data;
                    // Re-take buffer with transcoded data
                    let new_data = std::mem::take(&mut self.buffer);
                    let (error2, next2) = match self.processor {
                        Processor::Content => {
                            let have_more = !self.is_final;
                            let enc = xmltok::Utf8Encoding;
                            let stl = self.content_start_tag_level;
                            self.do_content(stl, &enc, &new_data, 0, new_data.len(), have_more)
                        }
                        _ => (XmlError::None, 0),
                    };
                    if error2 != XmlError::None {
                        self.error_code = error2;
                    }
                    if next2 < new_data.len() && error2 == XmlError::None {
                        self.buffer = new_data[next2..].to_vec();
                    }
                    return;
                }

                start = next_pos;
                continue;
            }

            // Buffer remaining data
            if next_pos < end {
                self.buffer = data[next_pos..end].to_vec();
            }

            // Set event_pos for position tracking
            if error == XmlError::None {
                self.event_pos = next_pos;
            }

            // C's callProcessor returns the processor error code, and XML_Parse
            // only checks that return — not m_errorCode. In Rust, handlers may
            // set self.error_code for informational purposes (e.g., SuspendPe)
            // without causing a processor error. Clear stale handler-set errors
            // when the processor completed successfully.
            if error == XmlError::None && self.error_code != XmlError::None
                && self.parsing_state == ParsingState::Parsing {
                self.error_code = XmlError::None;
            }

            // Note: not_standalone check for external DTD subsets is done by
            // the PARENT's DoctypeClose after the handler returns, matching C.
            // The child's prolog processing sets has_param_entity_refs in the
            // shared DTD when it finds entity SYSTEM IDs, which the parent then checks.

            return;
        }
    }

    /// External entity init processor — new-style (data, start, end) version.
    /// Port of C externalEntityInitProcessor3: uses content tokenizer to detect
    /// text declaration, then transitions to content processing.
    fn external_entity_init_processor3(
        &mut self,
        data: &[u8],
        start: usize,
        end: usize,
    ) -> (XmlError, usize) {
        let enc = xmltok::Utf8Encoding;
        let tok_result = xmltok_impl::content_tok(&enc, data, start, end);
        match tok_result {
            Ok(TokenResult { token, next_pos }) => match token {
                XmlTok::XmlDecl => {
                    // Text declaration — process inline then transition to content
                    // Matches C: processXmlDecl(parser, 1, start, next) then
                    //            parser->m_processor = externalEntityContentProcessor
                    let decl_data = &data[start..next_pos];
                    let is_text_decl = true; // external entity text declaration
                    let parse_result = xmltok::parse_xml_decl(decl_data, is_text_decl);
                    let parse_result = if parse_result.is_err() && is_text_decl {
                        xmltok::parse_xml_decl(decl_data, false)
                    } else {
                        parse_result
                    };

                    match parse_result {
                        Ok(info) => {
                            // Handle encoding from text declaration
                            if !self.protocol_encoding_set
                                && info.encoding_end > info.encoding_start
                            {
                                if let Ok(enc_name) = std::str::from_utf8(
                                    &decl_data[info.encoding_start..info.encoding_end],
                                ) {
                                    let upper = enc_name.to_uppercase();
                                    if is_latin1_encoding(Some(&upper)) {
                                        self.detected_encoding = Some(upper);
                                    } else if !is_known_encoding(&upper) {
                                        let mut handled = false;
                                        if let Some(handler) = &mut self.unknown_encoding_handler {
                                            handled = handler(enc_name);
                                        }
                                        if !handled {
                                            return (XmlError::UnknownEncoding, start);
                                        }
                                    }
                                }
                            }

                            // Call xml_decl_handler if set
                            if let Some(handler) = &mut self.xml_decl_handler {
                                let version_str = if info.version_end > info.version_start {
                                    std::str::from_utf8(
                                        &decl_data[info.version_start..info.version_end],
                                    )
                                    .ok()
                                } else {
                                    None
                                };
                                let encoding_str = if info.encoding_end > info.encoding_start {
                                    std::str::from_utf8(
                                        &decl_data[info.encoding_start..info.encoding_end],
                                    )
                                    .ok()
                                } else {
                                    None
                                };
                                handler(
                                    version_str,
                                    encoding_str,
                                    info.standalone.map(|s| if s { 1 } else { 0 }),
                                );
                            }
                        }
                        Err(_err_pos) => {
                            return (XmlError::TextDecl, start);
                        }
                    }

                    // Transition to content processor — matches C
                    self.tag_level = self.content_start_tag_level;
                    self.processor = Processor::Content;
                    (XmlError::None, next_pos)
                }
                XmlTok::Partial | XmlTok::PartialChar => {
                    if !self.is_final {
                        (XmlError::None, start) // buffer from start, wait for more
                    } else {
                        let err = if token == XmlTok::Partial {
                            XmlError::UnclosedToken
                        } else {
                            XmlError::PartialChar
                        };
                        (err, start)
                    }
                }
                XmlTok::Bom => {
                    // Skip BOM, continue with init
                    if next_pos < end {
                        self.external_entity_init_processor3(data, next_pos, end)
                    } else {
                        (XmlError::None, next_pos)
                    }
                }
                _ => {
                    // Not a text declaration — transition to content
                    // C: parser->m_processor = externalEntityContentProcessor;
                    //    parser->m_tagLevel = 1;
                    //    return externalEntityContentProcessor(parser, start, end, endPtr);
                    self.tag_level = self.content_start_tag_level;
                    self.processor = Processor::Content;
                    // Return start (not next_pos) — content processor re-tokenizes same data
                    (XmlError::None, start)
                }
            },
            Err(_) => {
                if !self.is_final {
                    (XmlError::None, start)
                } else {
                    (XmlError::InvalidToken, start)
                }
            }
        }
    }

    /// External parameter entity processor — port of C externalParEntProcessor.
    ///
    /// This is a one-shot bootstrapping processor for external parameter entity content.
    /// It handles the initial token (which may be a BOM), then delegates to the prolog processor.
    ///
    /// C: externalParEntInitProcessor + externalParEntProcessor (xmlparse.c:4999-5146).
    /// Sets paramEntityRead, skips BOM if present, then delegates to do_prolog.
    fn external_par_ent_processor(
        &mut self,
        data: &[u8],
        start: usize,
        end: usize,
    ) -> (XmlError, usize) {
        // C: externalParEntInitProcessor sets paramEntityRead = true
        // "we know now that XML_Parse(Buffer) has been called,
        //  so we consider the external parameter entity read"
        self.dtd.borrow_mut().param_entity_read = true;

        let enc = xmltok::Utf8Encoding;
        let mut s = start;

        // Check for BOM — skip it if present (C: externalParEntProcessor lines 5124-5138)
        if let Ok(TokenResult { token, next_pos }) = xmltok_impl::prolog_tok(&enc, data, start, end)
        {
            if token == XmlTok::Bom {
                s = next_pos;
            }
        }

        // Switch to Prolog processor and delegate all parsing (including error handling)
        // to do_prolog, which correctly maps token errors to XML error codes.
        self.processor = Processor::Prolog;
        let have_more = !self.is_final;
        self.do_prolog(&enc, data, s, end, have_more)
    }

    /// Initial prolog processor — detects encoding and transitions to prolog processor
    #[allow(dead_code)]
    fn prolog_init_processor(&mut self) {
        // For now, skip encoding detection and go straight to prolog
        // In a full implementation, this would call initializeEncoding()
        self.processor = Processor::Prolog;
        self.prolog_processor();
    }

    /// External entity init processor — port of C externalEntityInitProcessor3.
    /// Uses content tokenizer to detect text declaration. On text decl, processes it
    /// via prolog. On any other token, sets tag_level=1 and delegates to content.
    /// On Partial/None, buffers and waits for more data.
    #[allow(dead_code)]
    fn external_entity_init_processor(&mut self) {
        let data = std::mem::take(&mut self.buffer);
        if data.is_empty() {
            return;
        }

        let enc = xmltok::Utf8Encoding;
        let tok_result = xmltok_impl::content_tok(&enc, &data, 0, data.len());

        match tok_result {
            Ok(TokenResult { token, next_pos }) => match token {
                XmlTok::XmlDecl => {
                    // Text declaration found — process via prolog then content
                    // C: processXmlDecl(parser, 1, start, next)
                    // Use prolog processor to handle the text declaration
                    self.processor = Processor::Prolog;
                    self.buffer = data;
                    self.prolog_processor();
                }
                XmlTok::Partial | XmlTok::PartialChar => {
                    if !self.is_final {
                        // Need more data — stay in init processor
                        self.buffer = data;
                    } else {
                        self.error_code = if token == XmlTok::Partial {
                            XmlError::UnclosedToken
                        } else {
                            XmlError::PartialChar
                        };
                    }
                }
                XmlTok::Bom => {
                    // Skip BOM, stay in init processor for next token
                    if next_pos < data.len() {
                        self.buffer = data[next_pos..].to_vec();
                    }
                    // Stay in ExternalEntity processor
                }
                _ => {
                    // Not a text declaration — transition to content mode
                    // C: parser->m_processor = externalEntityContentProcessor;
                    //    parser->m_tagLevel = 1;
                    //    return externalEntityContentProcessor(parser, start, end, endPtr);
                    self.processor = Processor::Content;
                    // tag_level already set at parser creation time
                    // Pass ALL data to content processor (it will re-tokenize from start)
                    self.buffer = data;
                    self.content_processor();
                }
            },
            Err(_err_pos) => {
                if !self.is_final {
                    self.buffer = data;
                } else {
                    self.error_code = XmlError::InvalidToken;
                }
            }
        }
    }

    /// Convert XmlTok to xmlrole::Token
    fn xmltok_to_role_token(tok: XmlTok) -> xmlrole::Token {
        match tok {
            XmlTok::PrologS => xmlrole::Token::PrologS,
            XmlTok::XmlDecl => xmlrole::Token::XmlDecl,
            XmlTok::Pi => xmlrole::Token::Pi,
            XmlTok::Comment => xmlrole::Token::Comment,
            XmlTok::Bom => xmlrole::Token::Bom,
            XmlTok::DeclOpen => xmlrole::Token::DeclOpen,
            XmlTok::DeclClose => xmlrole::Token::DeclarationClose,
            XmlTok::InstanceStart => xmlrole::Token::InstanceStart,
            XmlTok::Name => xmlrole::Token::Name,
            XmlTok::PrefixedName => xmlrole::Token::PrefixedName,
            XmlTok::OpenBracket => xmlrole::Token::OpenBracket,
            XmlTok::CloseBracket => xmlrole::Token::CloseBracket,
            XmlTok::Literal => xmlrole::Token::Literal,
            XmlTok::Nmtoken => xmlrole::Token::Nmtoken,
            XmlTok::PoundName => xmlrole::Token::PoundName,
            XmlTok::ParamEntityRef => xmlrole::Token::ParamEntityRef,
            XmlTok::OpenParen => xmlrole::Token::OpenParen,
            XmlTok::CloseParen => xmlrole::Token::CloseParen,
            XmlTok::Or => xmlrole::Token::Or,
            XmlTok::Comma => xmlrole::Token::Comma,
            XmlTok::Percent => xmlrole::Token::Percent,
            XmlTok::CondSectOpen => xmlrole::Token::CondSectOpen,
            XmlTok::CondSectClose => xmlrole::Token::CondSectClose,
            XmlTok::NameQuestion => xmlrole::Token::NameQuestion,
            XmlTok::NameAsterisk => xmlrole::Token::NameAsterix,
            XmlTok::NamePlus => xmlrole::Token::NamePlus,
            XmlTok::CloseParenQuestion => xmlrole::Token::CloseParenQuestion,
            XmlTok::CloseParenAsterisk => xmlrole::Token::CloseParenAsterix,
            XmlTok::CloseParenPlus => xmlrole::Token::CloseParenPlus,
            // All other tokens map to None
            _ => xmlrole::Token::None,
        }
    }

    /// Prolog processor — corresponds to C prologProcessor()
    /// Uses do_prolog with the tokenizer+role architecture to parse the XML prolog
    #[allow(dead_code)]
    fn prolog_processor(&mut self) {
        let data = std::mem::take(&mut self.buffer);
        if data.is_empty() {
            if self.is_final && !self.seen_root && !self.parsing_foreign_dtd {
                self.error_code = XmlError::NoElements;
            }
            // For foreign DTD parsing, check not_standalone when DTD parsing is complete
            // even if buffer is empty (already consumed) but is_final is true
            if self.is_final
                && self.parsing_foreign_dtd
                && self.dtd.borrow().has_param_entity_refs
                && !self.dtd.borrow().standalone
            {
                if let Some(handler) = &mut self.not_standalone_handler {
                    if !handler() {
                        self.error_code = XmlError::NotStandalone;
                    }
                }
            }
            return;
        }
        let have_more = !self.is_final;
        let enc = xmltok::Utf8Encoding;

        let (error, next_pos) = self.do_prolog(&enc, &data, 0, data.len(), have_more);

        if error != XmlError::None {
            self.error_code = error;
            return;
        }

        // If processor switched to Content, process remaining data as content
        if self.processor == Processor::Content && next_pos < data.len() {
            let remaining = &data[next_pos..];
            // If Latin-1 encoding was detected, transcode remaining bytes
            self.buffer = if is_latin1_encoding(self.detected_encoding.as_deref()) {
                transcode_latin1_to_utf8(remaining)
            } else {
                remaining.to_vec()
            };
            self.content_processor();
            return;
        }

        // Keep unprocessed data for next parse call
        if next_pos < data.len() {
            let remaining = &data[next_pos..];
            self.buffer = if is_latin1_encoding(self.detected_encoding.as_deref()) {
                transcode_latin1_to_utf8(remaining)
            } else {
                remaining.to_vec()
            };
        } else if self.is_final {
            if self.processor != Processor::Content && !self.seen_root && !self.parsing_foreign_dtd
            {
                // All prolog data consumed, is_final, but no root element seen
                self.error_code = XmlError::NoElements;
            }
            // For foreign DTD parsing, check not_standalone when DTD parsing is complete
            if self.parsing_foreign_dtd
                && self.dtd.borrow().has_param_entity_refs
                && !self.dtd.borrow().standalone
            {
                if let Some(handler) = &mut self.not_standalone_handler {
                    if !handler() {
                        self.error_code = XmlError::NotStandalone;
                    }
                }
            }
        }
    }

    /// Main prolog parsing loop — corresponds to C doProlog()
    /// Uses prolog_tok from xmltok_impl to tokenize, and xml_token_role from xmlrole to determine roles
    fn do_prolog(
        &mut self,
        enc: &xmltok::Utf8Encoding,
        data: &[u8],
        start: usize,
        end: usize,
        have_more: bool,
    ) -> (XmlError, usize) {
        let mut pos = start;

        loop {
            // Get the next token from the tokenizer
            let result = xmltok_impl::prolog_tok(enc, data, pos, end);
            let (tok, next) = match result {
                Ok(TokenResult { token, next_pos }) => (token, next_pos),
                Err(err_pos) => {
                    // Check if this is a partial UTF-8 character at the end
                    if Self::is_partial_utf8_sequence(data, err_pos) {
                        if have_more {
                            // Save the remaining data for next parse call
                            self.buffer = data[err_pos..].to_vec();
                            return (XmlError::None, end);
                        }
                        // Final buffer with partial char → PartialChar error (C: XML_ERROR_PARTIAL_CHAR)
                        return (XmlError::PartialChar, pos);
                    }
                    return (XmlError::InvalidToken, pos);
                }
            };

            match tok {
                XmlTok::Bom => {
                    // BOM — skip it and continue
                    if next == end && have_more {
                        self.buffer = data[next..].to_vec();
                        return (XmlError::None, end);
                    }
                    self.advance_pos_slice(&data[pos..next]);
                    pos = next;
                    continue;
                }
                XmlTok::None => {
                    // End of buffer
                    break;
                }
                XmlTok::Partial => {
                    // Incomplete token — need more data
                    if have_more {
                        self.buffer = data[pos..].to_vec();
                        return (XmlError::None, end);
                    }
                    return (XmlError::UnclosedToken, pos);
                }
                XmlTok::PartialChar => {
                    // Incomplete UTF-8 character
                    if have_more {
                        self.buffer = data[pos..].to_vec();
                        return (XmlError::None, end);
                    }
                    return (XmlError::PartialChar, pos);
                }
                XmlTok::TrailingCr => {
                    // Trailing CR in prolog
                    if have_more {
                        self.buffer = data[pos..].to_vec();
                        return (XmlError::None, end);
                    }
                    // Final buffer with trailing CR: convert to newline and continue
                    // This handles DTD content that ends with \r followed by EOF
                    // In C doProlog, -3 (TRAILING_CR) falls through to default case
                    // which negates it and sets next=end. We handle it as whitespace.
                    pos = next; // skip the CR
                    continue;
                }
                XmlTok::Invalid => {
                    // Invalid token
                    return (XmlError::InvalidToken, pos);
                }
                _ => {
                    // C doProlog: if tok <= 0 && haveMore → buffer and wait
                    // In C, prologTok returns -tok for names at end of buffer.
                    // We detect this as: token consumed to end (next == end) and
                    // the token is a name/keyword type that might be incomplete.
                    if have_more
                        && next >= end
                        && matches!(
                            tok,
                            XmlTok::Name
                                | XmlTok::Nmtoken
                                | XmlTok::PrefixedName
                                | XmlTok::PoundName
                                | XmlTok::Literal
                                | XmlTok::CloseBracket
                        )
                    {
                        self.buffer = data[pos..].to_vec();
                        return (XmlError::None, end);
                    }

                    // Convert token type to role token type
                    let role_tok = Self::xmltok_to_role_token(tok);

                    // Extract token text for keyword matching
                    let tok_text = self.extract_token_text(tok, data, pos, next);

                    // Get the role for this token
                    let role =
                        xmlrole::xml_token_role(&mut self.prolog_state, role_tok, &tok_text, &[]);

                    // Handle IgnoreSect specially: scan past the closing ]]>
                    if role == xmlrole::Role::IgnoreSect {
                        let (error, ignore_end) = self.do_ignore_section(data, next, end);
                        if error != XmlError::None {
                            // If ignore section is incomplete and we expect more data, buffer and wait
                            if have_more {
                                self.buffer = data[pos..].to_vec();
                                return (XmlError::None, end);
                            }
                            return (error, pos);
                        }
                        // Report the entire ignore section to the default handler
                        // The section runs from 'pos' (start of <![) to 'ignore_end' (after ]]>)
                        if self.default_handler.is_some() || self.default_handler_expand.is_some() {
                            self.report_default(&xmltok::Utf8Encoding, data, pos, ignore_end);
                        }
                        self.advance_pos_slice(&data[pos..ignore_end]);
                        pos = ignore_end;
                        continue;
                    }

                    // Dispatch on role
                    let (error, suppress_default) =
                        self.handle_prolog_role(role, tok, data, pos, next, &tok_text);
                    if error != XmlError::None {
                        return (error, pos);
                    }

                    // If processor changed to Content, break out — remaining data
                    // will be processed by content_processor.
                    // Don't call report_default for the start tag here — let Content processor handle it
                    if self.processor == Processor::Content {
                        // InstanceStart: the token was the start tag, but
                        // content_tok needs to re-tokenize it, so return pos
                        // (not next) so the start tag is included in content data
                        return (XmlError::None, pos);
                    }

                    // Forward to default handler if suppress_default is false
                    // In C libexpat, reportDefault() is called for prolog tokens
                    // ONLY when no specific handler consumed them.
                    // Must happen BEFORE InternalEntity check (C does this before reenter return).
                    if (self.default_handler.is_some() || self.default_handler_expand.is_some())
                        && !suppress_default
                    {
                        self.report_default(&xmltok::Utf8Encoding, data, pos, next);
                    }

                    // Update position tracking for this token
                    self.advance_pos_slice(&data[pos..next]);

                    // C doProlog: check parsing state after handler calls
                    // Matches C xmlparse.c lines 6261-6271.
                    match self.parsing_state {
                        ParsingState::Suspended => {
                            return (XmlError::None, next);
                        }
                        ParsingState::Finished => {
                            return (XmlError::Aborted, next);
                        }
                        _ => {}
                    }

                    // C: if reenter flag is set (from processEntity → triggerReenter),
                    // return to let callProcessor/run_processor re-dispatch.
                    // Matches C xmlparse.c line 6268-6270.
                    if self.reenter {
                        return (XmlError::None, next);
                    }

                    pos = next;
                }
            }
        }

        (XmlError::None, pos)
    }

    /// Extract token text from data for the role state machine
    /// The role state machine needs text content for keyword matching (e.g., "DOCTYPE", "ENTITY")
    fn extract_token_text(&self, tok: XmlTok, data: &[u8], pos: usize, next: usize) -> Vec<u8> {
        let minbpc = 1; // UTF-8
        match tok {
            // For DeclOpen, skip the <!  prefix (2 bytes in UTF-8)
            XmlTok::DeclOpen => {
                if pos + minbpc * 2 <= next {
                    data[pos + minbpc * 2..next].to_vec()
                } else {
                    data[pos..next].to_vec()
                }
            }
            // For PoundName, skip the # prefix (1 byte)
            XmlTok::PoundName => {
                if pos + minbpc <= next {
                    data[pos + minbpc..next].to_vec()
                } else {
                    data[pos..next].to_vec()
                }
            }
            // For Literal, strip quotes
            XmlTok::Literal => {
                if pos + minbpc <= next {
                    data[pos + minbpc..next - minbpc].to_vec()
                } else {
                    data[pos..next].to_vec()
                }
            }
            // For all other tokens, return full text
            _ => data[pos..next].to_vec(),
        }
    }

    /// Handle the role returned by the role state machine
    /// Dispatches based on the role and calls the appropriate handler.
    /// Returns (error, suppress_default) where suppress_default indicates whether
    /// the default handler should be suppressed (a specific handler was called).
    fn handle_prolog_role(
        &mut self,
        role: xmlrole::Role,
        tok: XmlTok,
        data: &[u8],
        pos: usize,
        next: usize,
        tok_text: &[u8],
    ) -> (XmlError, bool) {
        match role {
            Role::XmlDecl => {
                // Process XML declaration — matches C processXmlDecl()
                if self.seen_xml_decl || self.seen_root {
                    return (XmlError::MisplacedXmlPi, false);
                }
                self.seen_xml_decl = true;

                let decl_data = &data[pos..next];
                // For external entities (non-document), parse as text declaration
                // For document entities, parse as full XML declaration
                let is_text_decl = !self.prolog_state.document_entity;

                // If parsing fails as a declaration, try accepting it for text declarations
                let parse_result = xmltok::parse_xml_decl(decl_data, is_text_decl);
                let parse_result = if parse_result.is_err() && is_text_decl {
                    // Text declarations are more lenient - try parsing as XML decl then ignore version requirement
                    xmltok::parse_xml_decl(decl_data, false)
                } else {
                    parse_result
                };

                match parse_result {
                    Ok(info) => {
                        // Extract version string
                        let version_str = if info.version_end > info.version_start {
                            Some(
                                std::str::from_utf8(
                                    &decl_data[info.version_start..info.version_end],
                                )
                                .unwrap_or("")
                                .to_string(),
                            )
                        } else {
                            None
                        };

                        // Extract encoding string
                        let encoding_str = if info.encoding_end > info.encoding_start {
                            Some(
                                std::str::from_utf8(
                                    &decl_data[info.encoding_start..info.encoding_end],
                                )
                                .unwrap_or("")
                                .to_string(),
                            )
                        } else {
                            None
                        };

                        // Handle standalone (C sets parser->m_dtd->standalone)
                        if info.standalone == Some(true) {
                            self.dtd.borrow_mut().standalone = true;
                        }

                        // Call xml_decl_handler if set — suppress default only if handler IS called
                        let handler_called = self.xml_decl_handler.is_some();
                        if let Some(handler) = &mut self.xml_decl_handler {
                            handler(
                                version_str.as_deref(),
                                encoding_str.as_deref(),
                                info.standalone.map(|s| if s { 1 } else { 0 }),
                            );
                        }

                        // Check encoding — matches C processXmlDecl logic
                        // If protocol_encoding_set (XML_SetEncoding was called), skip encoding conflict checks
                        // The C code skips these checks entirely if m_protocolEncodingName is set
                        if !self.protocol_encoding_set {
                            if let Some(ref enc_name) = encoding_str {
                                let upper = enc_name.to_uppercase();
                                if upper == "UTF-16" || upper == "UTF-16LE" || upper == "UTF-16BE" {
                                    // UTF-16 declared in what we're parsing as UTF-8 → error
                                    if self.detected_encoding.is_none() {
                                        self.event_pos = pos;
                                        return (XmlError::IncorrectEncoding, false);
                                    }
                                } else if upper == "ISO-8859-1"
                                    || upper == "LATIN1"
                                    || upper.starts_with("ISO-8859-")
                                    || upper == "WINDOWS-1252"
                                {
                                    // Latin-1 or similar single-byte encoding
                                    // Set detected_encoding so parse() transcodes subsequent data
                                    self.detected_encoding = Some(upper.clone());
                                } else if !is_known_encoding(&upper) {
                                    // Unknown encoding — try handler
                                    let mut handled = false;
                                    if let Some(handler) = &mut self.unknown_encoding_handler {
                                        handled = handler(enc_name);
                                    }
                                    if !handled {
                                        self.event_pos = pos;
                                        return (XmlError::UnknownEncoding, false);
                                    }
                                }
                            }
                        }
                        (XmlError::None, handler_called)
                    }
                    Err(_err_pos) => {
                        self.event_pos = pos;
                        (XmlError::XmlDecl, false)
                    }
                }
            }
            Role::DoctypeName => {
                // Store DOCTYPE name for subsequent roles
                let name = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.doctype_name = Some(name);
                self.doctype_system_id = None;
                self.doctype_public_id = None;
                self.doctype_handler_called = false;
                (XmlError::None, self.start_doctype_decl_handler.is_some())
            }
            Role::DoctypePublicId | Role::EntityPublicId | Role::NotationPublicId => {
                // Validate public ID characters (matches C normalizePublicId)
                // tok_text has quotes stripped
                if !is_valid_public_id(tok_text) {
                    self.event_pos = pos;
                    return (XmlError::Publicid, false);
                }
                let suppress = if matches!(role, Role::DoctypePublicId) {
                    self.dtd.borrow_mut().has_param_entity_refs = true;
                    let pubid = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                    self.doctype_public_id = Some(pubid);
                    self.start_doctype_decl_handler.is_some()
                } else if matches!(role, Role::EntityPublicId) {
                    let pubid = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                    self.current_entity_public_id = Some(pubid);
                    self.entity_decl_handler.is_some()
                } else {
                    let pubid = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                    self.current_notation_public_id = Some(pubid);
                    self.notation_decl_handler.is_some()
                };
                (XmlError::None, suppress)
            }
            Role::DoctypeSystemId => {
                // DOCTYPE SYSTEM — implies external subset
                self.dtd.borrow_mut().has_param_entity_refs = true;
                let sysid = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.doctype_system_id = Some(sysid);
                (XmlError::None, self.start_doctype_decl_handler.is_some())
            }
            Role::EntitySystemId => {
                // Entity SYSTEM ID — store for current entity
                let sys_id = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.current_entity_system_id = Some(sys_id);
                // Mark that we have external entity references (for not_standalone check)
                // This applies to both parameter entities and general entities with external references
                self.dtd.borrow_mut().has_param_entity_refs = true;
                (XmlError::None, self.entity_decl_handler.is_some())
            }
            Role::DoctypeInternalSubset => {
                // Internal subset — call start_doctype_decl_handler with has_internal=true
                let handler_called = self.start_doctype_decl_handler.is_some();
                if !self.doctype_handler_called {
                    if let Some(handler) = &mut self.start_doctype_decl_handler {
                        let name = self.doctype_name.clone().unwrap_or_default();
                        let sysid = self.doctype_system_id.clone();
                        let pubid = self.doctype_public_id.clone();
                        handler(&name, sysid.as_deref(), pubid.as_deref(), true);
                    }
                    self.doctype_handler_called = true;
                }
                (XmlError::None, handler_called)
            }
            Role::DoctypeClose => {
                // C issue #317: reject ]> inside entity expansion
                // Entity content cannot close the DOCTYPE declaration
                if !self.open_internal_entities.is_empty() {
                    return (XmlError::InvalidToken, false);
                }
                // Fire start handler if not already called (DOCTYPE without internal subset)
                let mut handler_called = false;
                if !self.doctype_handler_called {
                    if let Some(handler) = &mut self.start_doctype_decl_handler {
                        let name = self.doctype_name.clone().unwrap_or_default();
                        let sysid = self.doctype_system_id.clone();
                        let pubid = self.doctype_public_id.clone();
                        handler(&name, sysid.as_deref(), pubid.as_deref(), false);
                        handler_called = true;
                    }
                    self.doctype_handler_called = true;
                }

                // Load external DTD subset if system ID is present
                // Matches C: if (parser->m_doctypeSysid || parser->m_useForeignDTD)
                if self.doctype_system_id.is_some() || self.foreign_dtd {
                    let had_param_entity_refs = self.dtd.borrow().has_param_entity_refs;
                    self.dtd.borrow_mut().has_param_entity_refs = true;
                    // C: if (parser->m_paramEntityParsing && parser->m_externalEntityRefHandler)
                    // UnlessStandalone with standalone=yes → don't call handler
                    let should_parse = match self.param_entity_parsing {
                        ParamEntityParsing::Never => false,
                        ParamEntityParsing::Always => true,
                        ParamEntityParsing::UnlessStandalone => !self.dtd.borrow().standalone,
                    };
                    if should_parse {
                        if let Some(handler) = &mut self.external_entity_ref_handler {
                            let base = self.base_uri.clone();
                            let sys_id = self.doctype_system_id.clone();
                            let pub_id = self.doctype_public_id.clone();
                            self.dtd.borrow_mut().param_entity_read = false;
                            let ok =
                                handler("", base.as_deref(), sys_id.as_deref(), pub_id.as_deref());
                            if !ok {
                                return (XmlError::ExternalEntityHandling, false);
                            }
                            if self.dtd.borrow().param_entity_read {
                                if !self.dtd.borrow().standalone {
                                    if let Some(handler) = &mut self.not_standalone_handler {
                                        if !handler() {
                                            return (XmlError::NotStandalone, false);
                                        }
                                    }
                                }
                            } else if self.doctype_system_id.is_none() {
                                // Foreign DTD but nothing was read — restore
                                self.dtd.borrow_mut().has_param_entity_refs = had_param_entity_refs;
                            }
                        }
                    }
                    self.foreign_dtd = false;
                    // If we didn't load the external DTD (no handler or parsing disabled),
                    // still need to check not-standalone for docs with external subset
                    if (self.param_entity_parsing == ParamEntityParsing::Never
                        || self.external_entity_ref_handler.is_none())
                        && !self.dtd.borrow().standalone
                    {
                        if let Some(handler) = &mut self.not_standalone_handler {
                            if !handler() {
                                return (XmlError::NotStandalone, false);
                            }
                        }
                    }
                } else {
                    // No external subset — check not-standalone
                    if self.dtd.borrow().has_param_entity_refs && !self.dtd.borrow().standalone {
                        if let Some(handler) = &mut self.not_standalone_handler {
                            if !handler() {
                                return (XmlError::NotStandalone, false);
                            }
                        }
                    }
                }

                // End of DOCTYPE
                if let Some(handler) = &mut self.end_doctype_decl_handler {
                    handler();
                    handler_called = true;
                }
                // Clear DOCTYPE state
                self.doctype_name = None;
                self.doctype_system_id = None;
                self.doctype_public_id = None;
                let suppress = handler_called
                    || self.start_doctype_decl_handler.is_some()
                    || self.end_doctype_decl_handler.is_some();
                (XmlError::None, suppress)
            }
            Role::InstanceStart => {
                // If foreign DTD is enabled, call external entity ref handler
                // with empty context before processing the root element
                let mut handler_called = false;
                if self.foreign_dtd {
                    self.foreign_dtd = false; // Only trigger once
                    if let Some(handler) = &mut self.external_entity_ref_handler {
                        let base = self.base_uri.clone();
                        let ok = handler("", base.as_deref(), None, None);
                        handler_called = true;
                        if !ok {
                            return (XmlError::ExternalEntityHandling, false);
                        }
                    }
                    // C: saves hadParamEntityRefs, sets it true, calls handler.
                    // After handler, if paramEntityRead is true → keep hasParamEntityRefs.
                    // If paramEntityRead is false → restore original value.
                    // We track this via param_entity_read flag set by child parsers.
                    let had_param_entity_refs = self.dtd.borrow().has_param_entity_refs;
                    self.dtd.borrow_mut().has_param_entity_refs = true;
                    // Handler was already called above (line 1505) and may have set param_entity_read
                    // Check BEFORE clearing it
                    if self.dtd.borrow().param_entity_read {
                        // DTD was actually read — keep has_param_entity_refs = true
                    } else {
                        // DTD was not read — restore has_param_entity_refs
                        self.dtd.borrow_mut().has_param_entity_refs = had_param_entity_refs;
                    }
                    self.dtd.borrow_mut().param_entity_read = false;
                    // Check not-standalone after foreign DTD processing
                    if !self.dtd.borrow().standalone {
                        if let Some(handler) = &mut self.not_standalone_handler {
                            if !handler() {
                                return (XmlError::NotStandalone, false);
                            }
                        }
                    }
                }
                // Start of XML instance (root element)
                self.processor = Processor::Content;
                (XmlError::None, handler_called)
            }
            Role::GeneralEntityName => {
                // General entity declaration
                let name = std::str::from_utf8(&data[pos..next])
                    .unwrap_or("")
                    .to_string();
                self.current_entity_name = Some(name);
                self.current_is_param_entity = false;
                (XmlError::None, self.entity_decl_handler.is_some())
            }
            Role::ParamEntityName => {
                // PE declaration — store name and mark as param entity
                let name = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                // C: if entity already exists (duplicate declaration), m_declEntity=NULL
                let is_new = !self.dtd.borrow().param_entities.contains_key(&name);
                // Create entry in param_entities (will be updated with value/system_id later)
                self.dtd
                    .borrow_mut()
                    .param_entities
                    .entry(name.clone())
                    .or_insert_with(|| ParamEntity {
                        system_id: None,
                        public_id: None,
                        value: None,
                        is_internal: false,
                        open: false,
                    });
                if is_new {
                    self.current_entity_name = Some(name);
                } else {
                    // Duplicate declaration — set to None (C: m_declEntity=NULL)
                    self.current_entity_name = None;
                }
                self.current_is_param_entity = true;
                (XmlError::None, self.entity_decl_handler.is_some())
            }
            Role::EntityValue => {
                // Entity value — validate and store
                // C: doProlog case XML_ROLE_ENTITY_VALUE (xmlparse.c:5625-5670)
                let mut handler_called = false;
                // Clone name to avoid borrow conflict with &mut self in store_entity_value
                let name = self.current_entity_name.clone();
                let is_pe = self.current_is_param_entity;
                if let Some(name) = name {
                    if is_pe {
                        // PE value → store in param_entities (not internal_entities)
                        // C: callStoreEntityValue for param entities
                        match self.store_entity_value(tok_text) {
                            Ok(value) => {
                                if let Some(pe) =
                                    self.dtd.borrow_mut().param_entities.get_mut(&name)
                                {
                                    pe.value = Some(value);
                                }
                            }
                            Err(e) => {
                                // Propagate real errors (ExternalEntityHandling, RecursiveEntityRef)
                                // but not ParamEntityRef (shouldn't happen for PE in external subset)
                                if e != XmlError::None {
                                    return (e, false);
                                }
                            }
                        }
                        return (XmlError::None, self.entity_decl_handler.is_some());
                    }
                    // tok_text has quotes already stripped by extract_token_text
                    match self.store_entity_value(tok_text) {
                        Ok(value) => {
                            self.dtd
                                .borrow_mut()
                                .internal_entities
                                .insert(name.clone(), value.clone());
                            // Call entity declaration handler (matches C)
                            // Only if DTD processing hasn't been stopped by undefined PEs
                            if self.dtd.borrow().keep_processing {
                                if let Some(handler) = &mut self.entity_decl_handler {
                                    let base = self.base_uri.clone();
                                    handler(&name, false, Some(&value), base.as_deref(), None);
                                    handler_called = true;
                                }
                            }
                        }
                        Err(e) => {
                            self.event_pos = pos;
                            return (e, false);
                        }
                    }
                }
                (XmlError::None, handler_called)
            }
            Role::EntityComplete => {
                // End of entity declaration
                // If entity has a system ID, store as external entity
                let mut handler_called = false;
                if let (Some(ref name), Some(_)) =
                    (&self.current_entity_name, &self.current_entity_system_id)
                {
                    // Call entity declaration handler for external entity
                    // Only if DTD processing hasn't been stopped by undefined PEs
                    if self.dtd.borrow().keep_processing {
                        if let Some(handler) = &mut self.entity_decl_handler {
                            let base = self.base_uri.clone();
                            let sys_id = self.current_entity_system_id.clone();
                            handler(name, false, None, base.as_deref(), sys_id.as_deref());
                            handler_called = true;
                        }
                    }
                    if self.current_is_param_entity {
                        // Store system_id/public_id on the PE entry
                        if let Some(pe) = self.dtd.borrow_mut().param_entities.get_mut(name) {
                            pe.system_id = self.current_entity_system_id.take();
                            pe.public_id = self.current_entity_public_id.take();
                        }
                    } else {
                        self.dtd.borrow_mut().external_entities.insert(
                            name.clone(),
                            (
                                self.current_entity_system_id.take(),
                                self.current_entity_public_id.take(),
                            ),
                        );
                    }
                }
                self.current_entity_name = None;
                self.current_entity_system_id = None;
                self.current_entity_public_id = None;
                self.current_entity_notation = None;
                (XmlError::None, handler_called)
            }
            Role::NotationName => {
                // Notation declaration — save name
                let name = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.current_notation_name = Some(name);
                self.current_notation_system_id = None;
                self.current_notation_public_id = None;
                (XmlError::None, self.notation_decl_handler.is_some())
            }
            Role::NotationSystemId => {
                // Notation SYSTEM ID
                let sysid = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.current_notation_system_id = Some(sysid);
                // Call notation handler
                let handler_called = self.notation_decl_handler.is_some();
                if let Some(handler) = &mut self.notation_decl_handler {
                    let name = self.current_notation_name.clone().unwrap_or_default();
                    let base = self.base_uri.clone();
                    let sysid = self.current_notation_system_id.clone().unwrap_or_default();
                    let pubid = self.current_notation_public_id.clone();
                    handler(&name, base.as_deref(), &sysid, pubid.as_deref());
                }
                (XmlError::None, handler_called)
            }
            Role::NotationNoSystemId => {
                // Notation with PUBLIC but no SYSTEM — call handler
                let handler_called = self.notation_decl_handler.is_some();
                if let Some(handler) = &mut self.notation_decl_handler {
                    let name = self.current_notation_name.clone().unwrap_or_default();
                    let base = self.base_uri.clone();
                    let pubid = self.current_notation_public_id.clone();
                    handler(&name, base.as_deref(), "", pubid.as_deref());
                }
                (XmlError::None, handler_called)
            }
            Role::EntityNotationName => {
                // Entity NDATA notation name — store notation and call unparsed entity handler
                let notation = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.current_entity_notation = Some(notation);

                // Mark this entity as unparsed (has NDATA notation)
                if let Some(ref name) = self.current_entity_name {
                    self.dtd.borrow_mut().unparsed_entities.insert(name.clone());
                }

                // Call unparsed entity handler if set (matches C XML_ROLE_ENTITY_NOTATION_NAME)
                let mut handler_called = false;
                if let Some(ref name) = self.current_entity_name {
                    if self.dtd.borrow().keep_processing {
                        if let Some(handler) = &mut self.unparsed_entity_decl_handler {
                            let base = self.base_uri.clone();
                            let sys_id = self.current_entity_system_id.clone();
                            let pub_id = self.current_entity_public_id.clone();
                            handler(
                                name,
                                base.as_deref(),
                                sys_id.as_deref().unwrap_or(""),
                                pub_id.as_deref(),
                            );
                            handler_called = true;
                        } else if let Some(handler) = &mut self.entity_decl_handler {
                            // Fallback to entity_decl_handler if unparsed handler not set (matches C)
                            let base = self.base_uri.clone();
                            let sys_id = self.current_entity_system_id.clone();
                            handler(name, false, None, base.as_deref(), sys_id.as_deref());
                            handler_called = true;
                        }
                    }
                }
                (XmlError::None, handler_called)
            }
            Role::AttlistElementName => {
                // Start of ATTLIST declaration — remember element name
                let elem_name = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.current_attlist_element = Some(elem_name);
                self.current_attlist_attr = None;
                (XmlError::None, self.attlist_decl_handler.is_some())
            }
            Role::AttributeName => {
                // Attribute in ATTLIST — remember attribute name
                let attr_name = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.current_attlist_attr = Some(attr_name);
                self.current_attlist_type = None;
                (XmlError::None, self.attlist_decl_handler.is_some())
            }
            Role::AttributeTypeCdata
            | Role::AttributeTypeId
            | Role::AttributeTypeIdref
            | Role::AttributeTypeIdrefs
            | Role::AttributeTypeEntity
            | Role::AttributeTypeEntities
            | Role::AttributeTypeNmtoken
            | Role::AttributeTypeNmtokens => {
                // Store the attribute type for ID tracking
                let type_name = match role {
                    Role::AttributeTypeCdata => "CDATA",
                    Role::AttributeTypeId => "ID",
                    Role::AttributeTypeIdref => "IDREF",
                    Role::AttributeTypeIdrefs => "IDREFS",
                    Role::AttributeTypeEntity => "ENTITY",
                    Role::AttributeTypeEntities => "ENTITIES",
                    Role::AttributeTypeNmtoken => "NMTOKEN",
                    Role::AttributeTypeNmtokens => "NMTOKENS",
                    _ => "CDATA",
                };
                self.current_attlist_type = Some(type_name.to_string());
                if let (Some(ref elem), Some(ref attr)) =
                    (&self.current_attlist_element, &self.current_attlist_attr)
                {
                    self.dtd
                        .borrow_mut()
                        .attlist_types
                        .entry(elem.clone())
                        .or_default()
                        .insert(attr.clone(), type_name.to_string());
                }
                // Suppress default handler for all attribute type roles — they're part of ATTLIST handling
                let suppress = self.attlist_decl_handler.is_some();
                (XmlError::None, suppress)
            }
            Role::AttributeEnumValue => {
                // Enumeration value — append to type string like (one|two|three)
                let val = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                if let Some(ref mut type_str) = self.current_attlist_type {
                    if type_str.ends_with('(') {
                        type_str.push_str(&val);
                    } else {
                        type_str.push('|');
                        type_str.push_str(&val);
                    }
                } else {
                    self.current_attlist_type = Some(format!("({}", val));
                }
                (XmlError::None, self.attlist_decl_handler.is_some())
            }
            Role::AttributeNotationValue => {
                // NOTATION enum value — append to type string like NOTATION(foo|bar)
                let val = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                if let Some(ref mut type_str) = self.current_attlist_type {
                    if type_str.ends_with('(') {
                        type_str.push_str(&val);
                    } else {
                        type_str.push('|');
                        type_str.push_str(&val);
                    }
                } else {
                    self.current_attlist_type = Some(format!("NOTATION({}", val));
                }
                (XmlError::None, self.attlist_decl_handler.is_some())
            }
            Role::ImpliedAttributeValue => {
                // #IMPLIED — no default value, not required
                let handler_called = self.attlist_decl_handler.is_some();
                if let Some(handler) = &mut self.attlist_decl_handler {
                    let elem = self.current_attlist_element.clone().unwrap_or_default();
                    let attr = self.current_attlist_attr.clone().unwrap_or_default();
                    let mut type_str = self
                        .current_attlist_type
                        .clone()
                        .unwrap_or_else(|| "CDATA".to_string());
                    if type_str.contains('(') && !type_str.ends_with(')') {
                        type_str.push(')');
                    }
                    handler(&elem, &attr, &type_str, None, None, false);
                }
                (XmlError::None, handler_called)
            }
            Role::RequiredAttributeValue => {
                // #REQUIRED — no default value, is required
                let handler_called = self.attlist_decl_handler.is_some();
                if let Some(handler) = &mut self.attlist_decl_handler {
                    let elem = self.current_attlist_element.clone().unwrap_or_default();
                    let attr = self.current_attlist_attr.clone().unwrap_or_default();
                    let mut type_str = self
                        .current_attlist_type
                        .clone()
                        .unwrap_or_else(|| "CDATA".to_string());
                    if type_str.contains('(') && !type_str.ends_with(')') {
                        type_str.push(')');
                    }
                    handler(&elem, &attr, &type_str, None, None, true);
                }
                (XmlError::None, handler_called)
            }
            Role::ElementName => {
                // Start of ELEMENT declaration
                let name = std::str::from_utf8(tok_text).unwrap_or("").to_string();
                self.current_element_decl_name = Some(name);
                self.content_model_stack.clear();
                self.group_connectors.clear();
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::ContentEmpty | Role::ContentAny => {
                // ELEMENT name EMPTY or ANY — call handler immediately
                let handler_called = self.element_decl_handler.is_some();
                if let Some(ref name) = self.current_element_decl_name.clone() {
                    // For EMPTY/ANY, serialize an empty model
                    self.serialize_content_model();
                    if let Some(handler) = &mut self.element_decl_handler {
                        handler(name, "");
                    }
                }
                self.current_element_decl_name = None;
                (XmlError::None, handler_called)
            }
            Role::ContentPcdata => {
                // C: modifies existing scaffold entry's type to Mixed
                // (does NOT push a new node — just changes the current group's type)
                if let Some(node) = self.content_model_stack.last_mut() {
                    node.content_type = ContentType::Mixed;
                }
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::GroupOpen => {
                self.group_connectors.push(0);
                self.content_model_stack.push(ContentNode {
                    content_type: ContentType::Seq,
                    quant: ContentQuant::None,
                    children: Vec::new(),
                    name: None,
                });
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::GroupSequence => {
                if let Some(last) = self.group_connectors.last_mut() {
                    if *last == 2 {
                        return (XmlError::Syntax, false);
                    }
                    *last = 1;
                }
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::GroupChoice => {
                if let Some(last) = self.group_connectors.last_mut() {
                    if *last == 1 {
                        return (XmlError::Syntax, false);
                    }
                    *last = 2;
                }
                if let Some(node) = self.content_model_stack.last_mut() {
                    if node.content_type == ContentType::Seq {
                        node.content_type = ContentType::Choice;
                    }
                }
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::ContentElement => {
                self.add_content_element(tok_text, ContentQuant::None);
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::ContentElementOpt => {
                self.add_content_element(tok_text, ContentQuant::Opt);
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::ContentElementRep => {
                self.add_content_element(tok_text, ContentQuant::Rep);
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::ContentElementPlus => {
                self.add_content_element(tok_text, ContentQuant::Plus);
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::GroupClose => {
                self.close_content_group(ContentQuant::None);
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::GroupCloseOpt => {
                self.close_content_group(ContentQuant::Opt);
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::GroupCloseRep => {
                self.close_content_group(ContentQuant::Rep);
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::GroupClosePlus => {
                self.close_content_group(ContentQuant::Plus);
                (XmlError::None, self.element_decl_handler.is_some())
            }
            Role::Pi => {
                // Processing instruction — suppress default (report_processing_instruction handles it)
                if tok == XmlTok::Pi {
                    self.report_processing_instruction(&xmltok::Utf8Encoding, data, pos, next);
                }
                (XmlError::None, true)
            }
            Role::Comment => {
                // Comment — suppress default (report_comment handles it)
                if tok == XmlTok::Comment {
                    self.report_comment(&xmltok::Utf8Encoding, data, pos, next);
                }
                (XmlError::None, true)
            }
            Role::DefaultAttributeValue => {
                // ATTLIST default value — validate and store
                if let Err(e) = self.validate_attribute_value(tok_text) {
                    self.event_pos = pos;
                    return (e, false);
                }
                let handler_called = self.attlist_decl_handler.is_some();
                if let (Some(ref elem), Some(ref attr)) =
                    (&self.current_attlist_element, &self.current_attlist_attr)
                {
                    let entities = self.dtd.borrow().internal_entities.clone();
                    let value = Self::normalize_attribute_value(tok_text, &entities);
                    {
                        let mut dtd = self.dtd.borrow_mut();
                        let defaults = dtd.attlist_defaults.entry(elem.clone()).or_default();
                        if !defaults.iter().any(|(n, _)| n == attr) {
                            defaults.push((attr.clone(), value.clone()));
                        }
                    }
                    if let Some(handler) = &mut self.attlist_decl_handler {
                        let mut type_str = self
                            .current_attlist_type
                            .clone()
                            .unwrap_or_else(|| "CDATA".to_string());
                        if type_str.contains('(') && !type_str.ends_with(')') {
                            type_str.push(')');
                        }
                        handler(elem, attr, &type_str, Some(&value), None, false);
                    }
                }
                (XmlError::None, handler_called)
            }
            Role::FixedAttributeValue => {
                // ATTLIST #FIXED value — validate and store
                if let Err(e) = self.validate_attribute_value(tok_text) {
                    self.event_pos = pos;
                    return (e, false);
                }
                let handler_called = self.attlist_decl_handler.is_some();
                if let (Some(ref elem), Some(ref attr)) =
                    (&self.current_attlist_element, &self.current_attlist_attr)
                {
                    let entities = self.dtd.borrow().internal_entities.clone();
                    let value = Self::normalize_attribute_value(tok_text, &entities);
                    {
                        let mut dtd = self.dtd.borrow_mut();
                        let defaults = dtd.attlist_defaults.entry(elem.clone()).or_default();
                        if !defaults.iter().any(|(n, _)| n == attr) {
                            defaults.push((attr.clone(), value.clone()));
                        }
                    }
                    if let Some(handler) = &mut self.attlist_decl_handler {
                        let mut type_str = self
                            .current_attlist_type
                            .clone()
                            .unwrap_or_else(|| "CDATA".to_string());
                        if type_str.contains('(') && !type_str.ends_with(')') {
                            type_str.push(')');
                        }
                        handler(elem, attr, &type_str, Some(&value), None, false);
                    }
                }
                (XmlError::None, handler_called)
            }
            Role::Error => {
                // Syntax error from role state machine
                match tok {
                    XmlTok::XmlDecl => (XmlError::MisplacedXmlPi, false),
                    XmlTok::ParamEntityRef => (XmlError::ParamEntityRef, false),
                    _ => (XmlError::Syntax, false),
                }
            }
            Role::IgnoreSect => {
                // Ignore section: <![IGNORE[ ... ]]> — suppress default (already called internally)
                // NOTE: This handler is now unreachable since do_prolog handles IgnoreSect specially
                // before calling handle_prolog_role. Keeping for safety but should not be reached.
                if self.default_handler.is_some() || self.default_handler_expand.is_some() {
                    self.report_default(&xmltok::Utf8Encoding, data, pos, next);
                }
                let (result, _) = self.do_ignore_section(data, next, data.len());
                (result, true)
            }
            Role::ParamEntityRef | Role::InnerParamEntityRef => {
                // PE reference — shared logic matching C's fall-through at lines 5996-6092
                let is_between_decl = matches!(role, Role::ParamEntityRef);
                self.dtd.borrow_mut().has_param_entity_refs = true;
                let mut handler_called = false;
                // C uses break statements to skip the not_standalone check.
                // We use this flag to track whether to run it (only when falling through).
                let mut check_not_standalone = true;

                if self.param_entity_parsing == ParamEntityParsing::Never {
                    let standalone = self.dtd.borrow().standalone;
                    self.dtd.borrow_mut().keep_processing = standalone;
                    // C: falls through to not_standalone check
                } else {
                    // Extract entity name from %name; token
                    let pe_name = if data.len() > pos && data[pos] == b'%' {
                        data[pos + 1..]
                            .iter()
                            .position(|&b| b == b';')
                            .and_then(|semi| {
                                std::str::from_utf8(&data[pos + 1..pos + 1 + semi])
                                    .ok()
                                    .map(|s| s.to_string())
                            })
                    } else {
                        None
                    };

                    if let Some(name) = pe_name {
                        let pe = self.dtd.borrow().param_entities.get(&name).cloned();

                        if let Some(pe) = pe {
                            // C: entity->open check for recursion
                            if pe.open {
                                return (XmlError::RecursiveEntityRef, false);
                            }
                            if let Some(ref value) = pe.value {
                                // Internal PE — call processEntity (C line 6059)
                                let entity_bytes = value.as_bytes().to_vec();
                                if let Some(e) = self.dtd.borrow_mut().param_entities.get_mut(&name)
                                {
                                    e.open = true;
                                }
                                let result = self.process_entity(&name, &entity_bytes, true);
                                if result != XmlError::None {
                                    return (result, false);
                                }
                                handler_called = true;
                                check_not_standalone = false; // C: break at line 6063
                            } else if pe.system_id.is_some() {
                                // External PE — call handler
                                if let Some(handler) = self.external_entity_ref_handler.as_mut() {
                                    self.dtd.borrow_mut().param_entity_read = false;
                                    if let Some(e) =
                                        self.dtd.borrow_mut().param_entities.get_mut(&name)
                                    {
                                        e.open = true;
                                    }
                                    let base = self.base_uri.clone();
                                    // C passes 0 (NULL) as context for PE refs
                                    let ok = handler(
                                        "",
                                        base.as_deref(),
                                        pe.system_id.as_deref(),
                                        pe.public_id.as_deref(),
                                    );
                                    if let Some(e) =
                                        self.dtd.borrow_mut().param_entities.get_mut(&name)
                                    {
                                        e.open = false;
                                    }
                                    handler_called = true;
                                    if !ok {
                                        return (XmlError::ExternalEntityHandling, false);
                                    }
                                    if !self.dtd.borrow().param_entity_read {
                                        let standalone = self.dtd.borrow().standalone;
                                        self.dtd.borrow_mut().keep_processing = standalone;
                                        check_not_standalone = false; // C: break at line 6081
                                    }
                                    // paramEntityRead=true → falls through to not_standalone check
                                } else {
                                    let standalone = self.dtd.borrow().standalone;
                                    self.dtd.borrow_mut().keep_processing = standalone;
                                    check_not_standalone = false; // C: break at line 6085
                                }
                            }
                        } else {
                            // Entity not found
                            let standalone = self.dtd.borrow().standalone;
                            self.dtd.borrow_mut().keep_processing = standalone;
                            if is_between_decl {
                                if let Some(skipped_handler) = &mut self.skipped_entity_handler {
                                    skipped_handler(&name, true);
                                    handler_called = true;
                                }
                            }
                            check_not_standalone = false; // C: break at line 6051
                        }
                    }
                }

                // C line 6089-6091: not_standalone check only when falling through
                if check_not_standalone && !self.dtd.borrow().standalone {
                    if let Some(ns_handler) = &mut self.not_standalone_handler {
                        if !ns_handler() {
                            return (XmlError::NotStandalone, false);
                        }
                    }
                }

                (XmlError::None, handler_called)
            }
            Role::DoctypeNone => {
                // Suppress default only when startDoctypeDeclHandler is set (matches C)
                (XmlError::None, self.start_doctype_decl_handler.is_some())
            }
            Role::EntityNone => (
                XmlError::None,
                self.dtd.borrow().keep_processing && self.entity_decl_handler.is_some(),
            ),
            Role::NotationNone => (XmlError::None, self.notation_decl_handler.is_some()),
            Role::AttlistNone => (
                XmlError::None,
                self.dtd.borrow().keep_processing && self.attlist_decl_handler.is_some(),
            ),
            Role::ElementNone => (XmlError::None, self.element_decl_handler.is_some()),
            Role::TextDecl => {
                // Text declaration in external entity — C: processXmlDecl(parser, 1, s, next)
                // Parse and validate the text declaration (<?xml version='...' encoding='...'?>)
                let decl_data = &data[pos..next];
                match xmltok::parse_xml_decl(decl_data, true) {
                    Ok(info) => {
                        // Extract encoding
                        let encoding_str = if info.encoding_end > info.encoding_start {
                            Some(
                                std::str::from_utf8(
                                    &decl_data[info.encoding_start..info.encoding_end],
                                )
                                .unwrap_or("")
                                .to_string(),
                            )
                        } else {
                            None
                        };
                        // Call xml_decl_handler if set (C calls it for text declarations too)
                        let handler_called = self.xml_decl_handler.is_some();
                        if let Some(handler) = &mut self.xml_decl_handler {
                            let version_str = if info.version_end > info.version_start {
                                Some(
                                    std::str::from_utf8(
                                        &decl_data[info.version_start..info.version_end],
                                    )
                                    .unwrap_or("")
                                    .to_string(),
                                )
                            } else {
                                None
                            };
                            handler(version_str.as_deref(), encoding_str.as_deref(), None);
                        }
                        // Handle encoding switch if declared
                        if !self.protocol_encoding_set {
                            if let Some(ref enc_name) = encoding_str {
                                let upper = enc_name.to_uppercase();
                                if upper == "ISO-8859-1"
                                    || upper == "LATIN1"
                                    || upper.starts_with("ISO-8859-")
                                    || upper == "WINDOWS-1252"
                                {
                                    self.detected_encoding = Some(upper.clone());
                                }
                            }
                        }
                        (XmlError::None, handler_called)
                    }
                    Err(_) => {
                        self.event_pos = pos;
                        (XmlError::XmlDecl, false)
                    }
                }
            }
            _ => {
                // Other roles — ignore for now
                (XmlError::None, false)
            }
        }
    }

    /// CDATA section processor — resumes interrupted CDATA section parsing
    /// Corresponds to C cdataSectionProcessor()
    #[allow(dead_code)]
    fn cdata_section_processor(&mut self) {
        let data = std::mem::take(&mut self.buffer);
        if data.is_empty() {
            if self.is_final {
                self.error_code = XmlError::UnclosedCdataSection;
            }
            return;
        }
        let have_more = !self.is_final;
        let enc = xmltok::Utf8Encoding;

        let (error, next_pos) = self.do_cdata_section(&enc, &data, 0, data.len(), have_more);

        if error != XmlError::None {
            self.error_code = error;
            return;
        }

        // CDATA section completed — switch back to content processor
        self.processor = Processor::Content;

        // Process remaining data as content
        if next_pos < data.len() {
            self.buffer = data[next_pos..].to_vec();
            self.content_processor();
        } else if have_more {
            // All data consumed, more coming
        } else if self.is_final {
            // Final and no more data — check if we're properly closed
            // (The content processor will handle this)
        }
    }

    /// Content processor — corresponds to C contentProcessor()
    /// Uses do_content with the tokenizer for content parsing.
    #[allow(dead_code)]
    fn content_processor(&mut self) {
        let data = std::mem::take(&mut self.buffer);
        if data.is_empty() {
            if self.is_final && !self.seen_root && self.content_start_tag_level == 0 {
                self.error_code = XmlError::NoElements;
            }
            return;
        }
        let have_more = !self.is_final;
        let enc = xmltok::Utf8Encoding;
        let stl = self.content_start_tag_level;

        let (error, next_pos) = self.do_content(stl, &enc, &data, 0, data.len(), have_more);

        // Set event_pos for successful completion too (for position query after parse)
        if error == XmlError::None {
            self.event_pos = next_pos;
        }

        if error != XmlError::None {
            self.error_code = error;
        }

        // Keep unprocessed data for next parse call
        if next_pos < data.len() && error == XmlError::None {
            self.buffer = data[next_pos..].to_vec();
        }
    }

    /// Check if the data starting from err_pos is part of a partial UTF-8 sequence
    /// This checks if err_pos points to the start of a multi-byte UTF-8 lead byte that's incomplete,
    /// or if it's part of an incomplete sequence starting earlier
    fn is_partial_utf8_sequence(data: &[u8], err_pos: usize) -> bool {
        if err_pos >= data.len() {
            return false;
        }

        let byte_at_pos = data[err_pos];

        // First, check if err_pos itself points to a lead byte
        if (0xc0..0xf8).contains(&byte_at_pos) {
            // This is a lead byte at err_pos
            let expected_bytes = if (0xc0..0xe0).contains(&byte_at_pos) {
                2 // 2-byte UTF-8 character
            } else if (0xe0..0xf0).contains(&byte_at_pos) {
                3 // 3-byte UTF-8 character
            } else {
                4 // 4-byte UTF-8 character (0xf0-0xf7)
            };

            let bytes_available = data.len() - err_pos;
            if bytes_available < expected_bytes
                && Self::all_bytes_valid(&data[err_pos..], expected_bytes)
            {
                // Incomplete UTF-8 sequence starting at err_pos
                return true;
            }
        }

        // Otherwise, search backwards to find a lead byte that might have started before err_pos
        for lookback in 1..=3 {
            if err_pos < lookback {
                break;
            }
            let pos = err_pos - lookback;
            let lead_byte = data[pos];

            // Check if this looks like a lead byte
            if (0xc0..0xf8).contains(&lead_byte) {
                // Determine expected byte count from lead byte
                let expected_bytes = if (0xc0..0xe0).contains(&lead_byte) {
                    2 // 2-byte UTF-8 character
                } else if (0xe0..0xf0).contains(&lead_byte) {
                    3 // 3-byte UTF-8 character
                } else {
                    4 // 4-byte UTF-8 character (0xf0-0xf7)
                };

                // Check if we have fewer bytes than expected from this lead byte to end of data
                let bytes_after_lead = data.len() - pos;
                if bytes_after_lead < expected_bytes
                    && Self::all_bytes_valid(&data[pos..], expected_bytes)
                {
                    // This looks like an incomplete UTF-8 sequence
                    return true;
                }
            }
        }

        false
    }

    /// Helper to check if bytes form a valid (though possibly incomplete) UTF-8 sequence start
    fn all_bytes_valid(sequence: &[u8], expected_len: usize) -> bool {
        // First byte must be a lead byte
        if sequence.is_empty() || (sequence[0] < 0xc0 || sequence[0] >= 0xf8) {
            return false;
        }
        // Remaining bytes (if present) should be trail bytes (10xxxxxx)
        for byte in sequence
            .iter()
            .take(sequence.len().min(expected_len))
            .skip(1)
        {
            if *byte < 0x80 || *byte >= 0xc0 {
                return false;
            }
        }
        true
    }

    /// Internal entity processor — new-style (data, start, end) version
    /// Processes entity content from the open_internal_entities stack.
    /// Corresponds to internalEntityProcessor in C (xmlparse.c:6420).
    /// Uses C's two-pass approach:
    ///   Pass 1 (has_more=true): process entity text via doProlog/doContent
    ///   Pass 2 (has_more=false): cleanup — close entity, restore processor
    fn internal_entity_processor(
        &mut self,
        _data: &[u8],
        start: usize,
        _end: usize,
    ) -> (XmlError, usize) {
        if self.open_internal_entities.is_empty() {
            return (XmlError::UnexpectedState, start);
        }

        // Extract entity info (avoid borrowing self during do_prolog/do_content)
        let has_more = self.open_internal_entities.last().unwrap().has_more;

        if has_more {
            // Pass 1: process entity text
            let entity_text = self
                .open_internal_entities
                .last()
                .unwrap()
                .entity_text
                .clone();
            let start_pos = self.open_internal_entities.last().unwrap().processed;
            let is_param = self.open_internal_entities.last().unwrap().is_param;
            let start_tag_level = self.open_internal_entities.last().unwrap().start_tag_level;
            let entity_end = entity_text.len();

            let enc = xmltok::Utf8Encoding;
            let (error, next_pos) = if is_param {
                self.do_prolog(&enc, &entity_text, start_pos, entity_end, false)
            } else {
                self.do_content(
                    start_tag_level,
                    &enc,
                    &entity_text,
                    start_pos,
                    entity_end,
                    false,
                )
            };

            if error != XmlError::None {
                return (error, start);
            }

            // Check if entity is fully consumed
            if next_pos < entity_end {
                // Not done yet — save progress (suspended or needs reenter for inner entity)
                if let Some(open) = self.open_internal_entities.last_mut() {
                    open.processed = next_pos;
                }
                return (XmlError::None, start);
            }

            // Entity fully processed — mark has_more=false for cleanup on next call
            // Check for async entity first (tag level mismatch for general entities)
            if !is_param && start_tag_level != self.tag_level {
                return (XmlError::AsyncEntity, start);
            }

            if let Some(open) = self.open_internal_entities.last_mut() {
                open.has_more = false;
            }
            // Signal run_processor to call us again for cleanup (C: triggerReenter)
            self.reenter = true;
            return (XmlError::None, start);
        }

        // Pass 2: cleanup — entity is fully processed
        let closed = self.open_internal_entities.pop().unwrap();

        // Mark entity as closed
        if closed.is_param {
            if let Some(pe) = self
                .dtd
                .borrow_mut()
                .param_entities
                .get_mut(&closed.entity_name)
            {
                pe.open = false;
            }
        }

        // Restore processor
        if self.open_internal_entities.is_empty() {
            self.processor = closed.saved_processor;
        }
        // Signal run_processor to re-dispatch with restored processor (C: triggerReenter)
        self.reenter = true;

        (XmlError::None, start)
    }

    /// Process entity — opens an internal entity for processing
    /// Corresponds to processEntity in C (xmlparse.c).
    /// Called from EntityRef case to set up entity expansion through the processor loop.
    fn process_entity(
        &mut self,
        entity_name: &str,
        entity_text: &[u8],
        is_param: bool,
    ) -> XmlError {
        let open = OpenInternalEntity {
            entity_name: entity_name.to_string(),
            entity_text: entity_text.to_vec(),
            start_tag_level: self.tag_level,
            processed: 0,
            is_param,
            between_decl: false,
            saved_processor: self.processor,
            has_more: true,
        };

        self.open_internal_entities.push(open);
        self.processor = Processor::InternalEntity;
        self.reenter = true;
        XmlError::None
    }

    /// Epilog processor — corresponds to C epilogProcessor()
    /// After root element, only whitespace, comments, and PIs are allowed
    fn epilog_processor(&mut self) {
        let data = std::mem::take(&mut self.buffer);
        let have_more = !self.is_final;
        let enc = xmltok::Utf8Encoding;
        let len = data.len();
        let mut pos = 0;

        while pos < len {
            let result = xmltok_impl::prolog_tok(&enc, &data, pos, len);
            match result {
                Ok(TokenResult { token, next_pos }) => match token {
                    XmlTok::PrologS => {
                        self.report_default(&enc, &data, pos, next_pos);
                        pos = next_pos;
                    }
                    XmlTok::Comment => {
                        self.report_comment(&enc, &data, pos, next_pos);
                        pos = next_pos;
                    }
                    XmlTok::Pi => {
                        self.report_processing_instruction(&enc, &data, pos, next_pos);
                        pos = next_pos;
                    }
                    XmlTok::None => {
                        break;
                    }
                    XmlTok::Partial | XmlTok::TrailingCr => {
                        if have_more {
                            self.buffer = data[pos..].to_vec();
                            return;
                        }
                        self.error_code = XmlError::UnclosedToken;
                        return;
                    }
                    XmlTok::PartialChar => {
                        if have_more {
                            self.buffer = data[pos..].to_vec();
                            return;
                        }
                        self.error_code = XmlError::PartialChar;
                        return;
                    }
                    XmlTok::Invalid => {
                        self.error_code = XmlError::InvalidToken;
                        return;
                    }
                    _ => {
                        self.error_code = XmlError::JunkAfterDocElement;
                        return;
                    }
                },
                Err(err_pos) => {
                    // Check if this is a partial UTF-8 character at the end
                    if Self::is_partial_utf8_sequence(&data, err_pos) {
                        if have_more {
                            self.buffer = data[err_pos..].to_vec();
                            return;
                        }
                        // Final buffer with partial char — matches C epilogProcessor
                        self.error_code = XmlError::PartialChar;
                        return;
                    }
                    self.error_code = XmlError::JunkAfterDocElement;
                    return;
                }
            }
        }
    }

    /// Main content parsing loop — corresponds to C doContent()
    /// Uses content_tok from xmltok_impl to tokenize, then dispatches on token type
    fn do_content<E: Encoding>(
        &mut self,
        start_tag_level: u32,
        enc: &E,
        data: &[u8],
        start: usize,
        end: usize,
        have_more: bool,
    ) -> (XmlError, usize) {
        let mut pos = start;
        const MAX_ITERATIONS: usize = 10_000_000;
        let mut iterations = 0;

        loop {
            iterations += 1;
            if iterations > MAX_ITERATIONS {
                return (XmlError::UnexpectedState, pos);
            }

            let result = xmltok_impl::content_tok(enc, data, pos, end);
            let (tok, next) = match result {
                Ok(TokenResult { token, next_pos }) => {
                    // Safety: tokenizer must make progress
                    if next_pos == pos
                        && !matches!(
                            token,
                            XmlTok::None
                                | XmlTok::Partial
                                | XmlTok::PartialChar
                                | XmlTok::TrailingCr
                                | XmlTok::TrailingRsqb
                        )
                    {
                        return (XmlError::UnexpectedState, pos);
                    }
                    (token, next_pos)
                }
                Err(err_pos) => {
                    let _ = err_pos;
                    return (XmlError::InvalidToken, pos);
                }
            };

            // Track byte count and raw data of current token for XML_GetCurrentByteCount
            // and XML_DefaultCurrent
            self.event_cur_byte_count = (next - pos) as i32;
            self.event_cur_data = data[pos..next].to_vec();

            // Record the current token position for lazy line/column computation.
            // XML_GetCurrentLineNumber/ColumnNumber will scan parse_data on demand.
            self.event_pos = pos;

            match tok {
                XmlTok::TrailingCr => {
                    // C code: first call the handler, THEN check for async entity / no elements
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(b"\n");
                    } else {
                        self.report_default(enc, data, pos, end);
                    }
                    // Now check for error conditions AFTER calling handler
                    if start_tag_level == 0 {
                        return (XmlError::NoElements, next);
                    }
                    if start_tag_level > 0 && self.tag_level != start_tag_level {
                        return (XmlError::AsyncEntity, pos);
                    }
                    return (XmlError::None, end);
                }

                XmlTok::None => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    if start_tag_level > 0 {
                        if self.tag_level != start_tag_level {
                            return (XmlError::AsyncEntity, pos);
                        }
                        return (XmlError::None, pos);
                    }
                    // At top level, reaching end of data means no root element
                    // was fully parsed (either never opened, or still open).
                    // C always returns XML_ERROR_NO_ELEMENTS here.
                    return (XmlError::NoElements, pos);
                }

                XmlTok::Invalid => {
                    return (XmlError::InvalidToken, next);
                }

                XmlTok::Partial => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    return (XmlError::UnclosedToken, pos);
                }

                XmlTok::PartialChar => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    return (XmlError::PartialChar, pos);
                }

                XmlTok::EntityRef => {
                    // Check for predefined entities first
                    let minbpc = enc.min_bytes_per_char();
                    let name_start = pos + minbpc; // skip '&'
                    let name_end = next - minbpc; // skip ';'
                    let ch = xmltok_impl::predefined_entity_name(enc, data, name_start, name_end);
                    if ch != 0 {
                        if let Some(handler) = &mut self.character_data_handler {
                            let mut buf = [0u8; 4];
                            if let Some(c) = char::from_u32(ch as u32) {
                                let encoded = c.encode_utf8(&mut buf);
                                handler(encoded.as_bytes());
                            }
                        } else {
                            self.report_default(enc, data, pos, next);
                        }
                    } else {
                        // General entity reference — matches C doContent entity handling
                        let name = std::str::from_utf8(&data[name_start..name_end]).unwrap_or("");

                        // Check for unparsed entity (NDATA notation) — can't be referenced with &
                        if self.dtd.borrow().unparsed_entities.contains(name) {
                            return (XmlError::BinaryEntityRef, pos);
                        }

                        // 1. Check internal entities
                        let entity_value = self.dtd.borrow().internal_entities.get(name).cloned();
                        if let Some(value) = entity_value {
                            if self.open_entities.contains(name) {
                                return (XmlError::RecursiveEntityRef, pos);
                            }
                            // Check which handler takes precedence:
                            // 1. If skipped entity handler is set, call it (suppresses expansion)
                            // 2. Else if default_handler_expand is set, expand the entity (allows it through)
                            // 3. Else if default_handler is set, suppress expansion and report entity ref to default
                            // 4. Else expand the entity (default behavior)
                            if let Some(handler) = &mut self.skipped_entity_handler {
                                handler(name, false);
                            } else if self.default_handler_expand.is_some() {
                                // Expand through do_content (matches C processEntity → internalEntityProcessor)
                                // Save event_pos/data since entity expansion will modify it
                                let saved_event_pos = self.event_pos;
                                let _saved_event_cur_byte_count = self.event_cur_byte_count;
                                let saved_event_cur_data = self.event_cur_data.clone();
                                let saved_entity_ref_context =
                                    self.entity_reference_context.clone();

                                // Set event_pos to entity reference position BEFORE recursion so async errors
                                // report the correct column
                                self.event_pos = pos;

                                let entity_name = name.to_string();
                                self.open_entities.insert(entity_name.clone());
                                // Save the entity reference text for XML_GetInputContext
                                self.entity_reference_context = Some(data[pos..next].to_vec());
                                let entity_bytes = value.as_bytes().to_vec();
                                let (entity_err, _) = self.do_content(
                                    self.tag_level,
                                    &xmltok::Utf8Encoding,
                                    &entity_bytes,
                                    0,
                                    entity_bytes.len(),
                                    false,
                                );
                                self.open_entities.remove(&entity_name);

                                // Restore event context to point to the entity reference, not expanded content
                                self.event_cur_data = saved_event_cur_data;
                                self.entity_reference_context = saved_entity_ref_context;

                                if entity_err != XmlError::None {
                                    return (entity_err, pos);
                                }

                                // On success, restore the saved event position
                                self.event_pos = saved_event_pos;
                            } else if self.default_handler.is_some() {
                                // default_handler is set but not default_handler_expand
                                // Report the entity reference as-is instead of expanding
                                self.report_default(enc, data, pos, next);
                            } else {
                                // No handlers set, use default behavior: expand
                                // Expand through do_content (matches C processEntity → internalEntityProcessor)
                                // Save event_pos/data since entity expansion will modify it
                                let saved_event_pos = self.event_pos;
                                let _saved_event_cur_byte_count = self.event_cur_byte_count;
                                let saved_event_cur_data = self.event_cur_data.clone();
                                let saved_entity_ref_context =
                                    self.entity_reference_context.clone();

                                // Set event_pos to entity reference position BEFORE recursion so async errors
                                // report the correct column
                                self.event_pos = pos;

                                let entity_name = name.to_string();
                                self.open_entities.insert(entity_name.clone());
                                // Save the entity reference text for XML_GetInputContext
                                self.entity_reference_context = Some(data[pos..next].to_vec());
                                let entity_bytes = value.as_bytes().to_vec();
                                let (entity_err, _) = self.do_content(
                                    self.tag_level,
                                    &xmltok::Utf8Encoding,
                                    &entity_bytes,
                                    0,
                                    entity_bytes.len(),
                                    false,
                                );
                                self.open_entities.remove(&entity_name);

                                // Restore event context to point to the entity reference, not expanded content
                                self.event_cur_data = saved_event_cur_data;
                                self.entity_reference_context = saved_entity_ref_context;

                                if entity_err != XmlError::None {
                                    return (entity_err, pos);
                                }

                                // On success, restore the saved event position
                                self.event_pos = saved_event_pos;
                            }
                        }
                        // 2. Check external entities (have system ID)
                        else if self.dtd.borrow().external_entities.contains_key(name) {
                            let entity_name = name.to_string();
                            let (sys_id, pub_id) = self
                                .dtd
                                .borrow()
                                .external_entities
                                .get(&entity_name)
                                .cloned()
                                .unwrap_or((None, None));
                            // Call external entity ref handler if set
                            if let Some(handler) = &mut self.external_entity_ref_handler {
                                let base = self.base_uri.clone();
                                let ok = handler(
                                    &entity_name,
                                    base.as_deref(),
                                    sys_id.as_deref(),
                                    pub_id.as_deref(),
                                );
                                if !ok {
                                    return (XmlError::ExternalEntityHandling, pos);
                                }
                            } else if self.default_handler.is_some() {
                                self.report_default(enc, data, pos, next);
                            }
                            // If no handler, silently skip (entity can't be expanded)
                        }
                        // 3. Entity not found at all
                        else {
                            // WFC: Entity Declared
                            if !self.dtd.borrow().has_param_entity_refs
                                || self.dtd.borrow().standalone
                            {
                                return (XmlError::UndefinedEntity, pos);
                            }
                            // External subset might define it — skip
                            if let Some(handler) = &mut self.skipped_entity_handler {
                                handler(name, false);
                            } else if self.default_handler.is_some() {
                                self.report_default(enc, data, pos, next);
                            }
                        }
                    }
                }

                XmlTok::StartTagNoAtts | XmlTok::StartTagWithAtts => {
                    let minbpc = enc.min_bytes_per_char();
                    let raw_name_start = pos + minbpc; // skip '<'
                    let raw_name_len = if self.ns_enabled {
                        self.extract_qualified_name(enc, data, raw_name_start)
                    } else {
                        xmltok_impl::name_length(enc, data, raw_name_start)
                    };
                    let tag_name =
                        std::str::from_utf8(&data[raw_name_start..raw_name_start + raw_name_len])
                            .unwrap_or("");

                    self.tag_level += 1;
                    self.seen_root = true;

                    // Extract attributes (with duplicate detection)
                    let mut attrs = if tok == XmlTok::StartTagWithAtts {
                        match self.extract_attrs(enc, data, pos, next) {
                            Ok(a) => a,
                            Err(e) => return (e, pos),
                        }
                    } else {
                        Vec::new()
                    };

                    // Apply ATTLIST defaults BEFORE namespace processing
                    // (so xmlns:prefix defaults are picked up)
                    let specified_count = attrs.len() as i32;
                    if let Some(defaults) = self.dtd.borrow().attlist_defaults.get(tag_name) {
                        for (dname, dval) in defaults {
                            if !attrs.iter().any(|(n, _)| n == dname) {
                                attrs.push((dname.clone(), dval.clone()));
                            }
                        }
                    }

                    // Process namespaces if enabled
                    let effective_tag_name = if self.ns_enabled {
                        match self.process_namespaces(tag_name, &mut attrs) {
                            Ok(name) => name,
                            Err(e) => return (e, pos),
                        }
                    } else {
                        tag_name.to_string()
                    };

                    // Normalize tokenized attribute values per XML spec §3.3.3
                    // NMTOKENS, IDREFS, ENTITIES types get whitespace collapsed
                    if let Some(type_map) = self.dtd.borrow().attlist_types.get(tag_name) {
                        for (attr_name, attr_val) in attrs.iter_mut() {
                            if let Some(att_type) = type_map.get(attr_name.as_str()) {
                                if matches!(
                                    att_type.as_str(),
                                    "NMTOKENS"
                                        | "IDREFS"
                                        | "ENTITIES"
                                        | "NMTOKEN"
                                        | "IDREF"
                                        | "ENTITY"
                                        | "ID"
                                        | "NOTATION"
                                ) {
                                    // Collapse: strip leading/trailing whitespace, collapse internal runs
                                    let collapsed: String = attr_val
                                        .split_whitespace()
                                        .collect::<Vec<&str>>()
                                        .join(" ");
                                    *attr_val = collapsed;
                                }
                            }
                        }
                    }
                    self.n_specified_atts = specified_count * 2; // C counts name+value pairs
                                                                 // Find ID attribute index
                    self.id_att_index = -1;
                    if let Some(types) = self.dtd.borrow().attlist_types.get(tag_name) {
                        for (i, (name, _)) in attrs.iter().enumerate() {
                            if types.get(name.as_str()).map(|t| t == "ID").unwrap_or(false) {
                                self.id_att_index = (i * 2) as i32;
                                break;
                            }
                        }
                    }

                    self.tag_stack.push(effective_tag_name.clone());
                    self.tag_triplet_flags.push(self.ns_triplets);

                    if let Some(handler) = &mut self.start_element_handler {
                        let attr_refs: Vec<(&str, &str)> = attrs
                            .iter()
                            .map(|(k, v)| (k.as_str(), v.as_str()))
                            .collect();
                        handler(&effective_tag_name, &attr_refs);
                    } else if self.default_handler.is_some()
                        || self.default_handler_expand.is_some()
                    {
                        self.report_default(enc, data, pos, next);
                    }
                }

                XmlTok::EmptyElementNoAtts | XmlTok::EmptyElementWithAtts => {
                    let minbpc = enc.min_bytes_per_char();
                    let raw_name_start = pos + minbpc;
                    let raw_name_len = if self.ns_enabled {
                        self.extract_qualified_name(enc, data, raw_name_start)
                    } else {
                        xmltok_impl::name_length(enc, data, raw_name_start)
                    };
                    let tag_name =
                        std::str::from_utf8(&data[raw_name_start..raw_name_start + raw_name_len])
                            .unwrap_or("")
                            .to_string();

                    self.seen_root = true;

                    let mut attrs = if tok == XmlTok::EmptyElementWithAtts {
                        match self.extract_attrs(enc, data, pos, next) {
                            Ok(a) => a,
                            Err(e) => return (e, pos),
                        }
                    } else {
                        Vec::new()
                    };

                    // Apply ATTLIST defaults BEFORE namespace processing
                    let specified_count = attrs.len() as i32;
                    if let Some(defaults) = self.dtd.borrow().attlist_defaults.get(&tag_name) {
                        for (dname, dval) in defaults {
                            if !attrs.iter().any(|(n, _)| n == dname) {
                                attrs.push((dname.clone(), dval.clone()));
                            }
                        }
                    }

                    // For namespace processing, bump tag_level to track bindings
                    if self.ns_enabled {
                        self.tag_level += 1;
                    }

                    // Process namespaces if enabled
                    let effective_tag_name = if self.ns_enabled {
                        match self.process_namespaces(&tag_name, &mut attrs) {
                            Ok(name) => name,
                            Err(e) => return (e, pos),
                        }
                    } else {
                        tag_name.clone()
                    };
                    self.n_specified_atts = specified_count * 2;
                    self.id_att_index = -1;
                    if let Some(types) = self.dtd.borrow().attlist_types.get(&tag_name) {
                        for (i, (name, _)) in attrs.iter().enumerate() {
                            if types.get(name.as_str()).map(|t| t == "ID").unwrap_or(false) {
                                self.id_att_index = (i * 2) as i32;
                                break;
                            }
                        }
                    }

                    // For empty elements, call start and end handlers if they exist.
                    // If neither exists, report to default handler (matches C logic).
                    let mut no_elm_handlers = true;
                    if let Some(handler) = &mut self.start_element_handler {
                        let attr_refs: Vec<(&str, &str)> = attrs
                            .iter()
                            .map(|(k, v)| (k.as_str(), v.as_str()))
                            .collect();
                        handler(&effective_tag_name, &attr_refs);
                        no_elm_handlers = false;
                    }
                    if let Some(handler) = &mut self.end_element_handler {
                        // (matches C: eventPtr points to end of tag only if both start and end handlers exist)
                        if self.start_element_handler.is_some() {
                            self.event_pos = next;
                        }
                        handler(&effective_tag_name);
                        no_elm_handlers = false;
                    }
                    if no_elm_handlers
                        && (self.default_handler.is_some() || self.default_handler_expand.is_some())
                    {
                        self.report_default(enc, data, pos, next);
                    }

                    // Pop namespace bindings for empty element (tag_level still at binding level)
                    if self.ns_enabled {
                        self.pop_ns_bindings(self.tag_level);
                        self.tag_level = self.tag_level.saturating_sub(1);
                    }

                    // Check if root element closed (empty root element)
                    if self.tag_level == start_tag_level {
                        self.root_closed = true;
                        self.processor = Processor::Epilog;
                        if next < end {
                            let epilog_data = data[next..end].to_vec();
                            self.buffer = epilog_data;
                            self.epilog_processor();
                        }
                        return (self.error_code, end);
                    }
                }

                XmlTok::EndTag => {
                    // Check for async entity — tag level matches start means entity boundary crossed
                    // Matches C: if (parser->m_tagLevel == startTagLevel) return ASYNC_ENTITY
                    // But NOT for ext entity content completion (where start == content_start_tag_level)
                    if self.tag_level == start_tag_level
                        && start_tag_level != self.content_start_tag_level
                    {
                        return (XmlError::AsyncEntity, pos);
                    }

                    let minbpc = enc.min_bytes_per_char();
                    let raw_name_start = pos + minbpc * 2; // skip '</'
                    let raw_name_len = if self.ns_enabled {
                        self.extract_qualified_name(enc, data, raw_name_start)
                    } else {
                        xmltok_impl::name_length(enc, data, raw_name_start)
                    };
                    let tag_name =
                        std::str::from_utf8(&data[raw_name_start..raw_name_start + raw_name_len])
                            .unwrap_or("");

                    // Expand tag name if namespace processing is enabled
                    let effective_tag_name = if self.ns_enabled {
                        match self.expand_name(tag_name, true) {
                            Ok(name) => name,
                            Err(e) => return (e, raw_name_start),
                        }
                    } else {
                        tag_name.to_string()
                    };

                    // Check tag mismatch — set event position to rawName
                    // (matches C: *eventPP = rawName)
                    // Compare the canonical form (without triplet suffix) to handle ns_triplets changes
                    if let Some(expected) = self.tag_stack.last() {
                        let expected_canonical = self.strip_triplet(expected);
                        let actual_canonical = self.strip_triplet(&effective_tag_name);
                        if expected_canonical != actual_canonical {
                            self.event_pos = raw_name_start;
                            return (XmlError::TagMismatch, raw_name_start);
                        }
                    } else {
                        self.event_pos = raw_name_start;
                        return (XmlError::TagMismatch, raw_name_start);
                    }

                    self.tag_stack.pop();
                    let was_triplet = self.tag_triplet_flags.pop().unwrap_or(false);
                    self.tag_level = self.tag_level.saturating_sub(1);

                    // If the element was opened with ns_triplets=true, we should call the handler
                    // with the triplet format, even if ns_triplets is now false
                    let handler_name =
                        if was_triplet && !self.ns_triplets && self.ns_separator != '\0' {
                            // Need to add the prefix back
                            // The effective_tag_name is in format "uri + sep + local"
                            // We need to find the prefix from the raw tag name
                            if let Some(colon_pos) = tag_name.find(':') {
                                let prefix = &tag_name[..colon_pos];
                                format!("{}{}{}", effective_tag_name, self.ns_separator, prefix)
                            } else {
                                effective_tag_name.clone()
                            }
                        } else if !was_triplet && self.ns_triplets && self.ns_separator != '\0' {
                            // Need to remove the prefix
                            self.strip_triplet(&effective_tag_name)
                        } else {
                            effective_tag_name.clone()
                        };

                    if let Some(handler) = &mut self.end_element_handler {
                        handler(&handler_name);
                    } else if self.default_handler.is_some()
                        || self.default_handler_expand.is_some()
                    {
                        self.report_default(enc, data, pos, next);
                    }

                    // Pop namespace bindings (tag_level already decremented)
                    if self.ns_enabled {
                        self.pop_ns_bindings(self.tag_level + 1);
                    }

                    // Check if root element closed
                    if self.tag_level == start_tag_level {
                        self.root_closed = true;
                        self.processor = Processor::Epilog;
                        if next < end {
                            let epilog_data = data[next..end].to_vec();
                            self.buffer = epilog_data;
                            self.epilog_processor();
                        }
                        return (self.error_code, end);
                    }
                    // For external entities: return when tag_level returns to start level
                    // Matches C: if (parser->m_tagLevel == startTagLevel) return XML_ERROR_NONE;
                    if start_tag_level > 0 && self.tag_level == start_tag_level {
                        return (XmlError::None, next);
                    }
                }

                XmlTok::CharRef => {
                    let n = xmltok_impl::char_ref_number(enc, data, pos);
                    if n < 0 {
                        return (XmlError::BadCharRef, pos);
                    }
                    if let Some(handler) = &mut self.character_data_handler {
                        let mut buf = [0u8; 4];
                        if let Some(c) = char::from_u32(n as u32) {
                            let encoded = c.encode_utf8(&mut buf);
                            handler(encoded.as_bytes());
                        }
                    } else {
                        self.report_default(enc, data, pos, next);
                    }
                }

                XmlTok::XmlDecl => {
                    return (XmlError::MisplacedXmlPi, pos);
                }

                XmlTok::DataNewline => {
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(b"\n");
                    } else {
                        self.report_default(enc, data, pos, next);
                    }
                }

                XmlTok::CdataSectOpen => {
                    if let Some(handler) = &mut self.start_cdata_section_handler {
                        handler();
                    } else if self.default_handler.is_some()
                        || self.default_handler_expand.is_some()
                    {
                        self.report_default(enc, data, pos, next);
                    }
                    // Scan CDATA content
                    let saved_processor = self.processor;
                    self.processor = Processor::CdataSection; // Mark as in-CDATA
                    let (cdata_err, cdata_next) =
                        self.do_cdata_section(enc, data, next, end, have_more);
                    if cdata_err != XmlError::None {
                        self.processor = saved_processor;
                        return (cdata_err, next);
                    }
                    if self.processor == Processor::CdataSection {
                        // CDATA section didn't close yet — stay in CDATA processor
                        // so next parse call resumes in CDATA mode
                        return (XmlError::None, cdata_next);
                    }
                    // CDATA section closed — processor was restored by do_cdata_section
                    self.processor = saved_processor;
                    pos = cdata_next;
                    continue; // don't update pos from next
                }

                XmlTok::TrailingRsqb => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&data[pos..end]);
                    } else if self.default_handler.is_some()
                        || self.default_handler_expand.is_some()
                    {
                        self.report_default(enc, data, pos, end);
                    }
                    if start_tag_level == 0 {
                        return (XmlError::NoElements, end);
                    }
                    // Check for async entity — mismatched tag levels
                    if self.tag_level != start_tag_level {
                        return (XmlError::AsyncEntity, end);
                    }
                    return (XmlError::None, end);
                }

                XmlTok::DataChars => {
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&data[pos..next]);
                    } else {
                        self.report_default(enc, data, pos, next);
                    }
                }

                XmlTok::Pi => {
                    self.report_processing_instruction(enc, data, pos, next);
                }

                XmlTok::Comment => {
                    self.report_comment(enc, data, pos, next);
                }

                _ => {
                    // Unhandled token — skip
                }
            }

            // Check parsing state after handler calls
            match self.parsing_state {
                ParsingState::Suspended => {
                    return (XmlError::None, next);
                }
                ParsingState::Finished => {
                    return (XmlError::Aborted, next);
                }
                _ => {}
            }

            pos = next;
        }
    }

    /// Process a CDATA section — corresponds to C doCdataSection()
    fn do_cdata_section<E: Encoding>(
        &mut self,
        enc: &E,
        data: &[u8],
        start: usize,
        end: usize,
        have_more: bool,
    ) -> (XmlError, usize) {
        let mut pos = start;
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations > 10_000_000 {
                return (XmlError::UnexpectedState, pos);
            }
            let result = xmltok_impl::cdata_section_tok(enc, data, pos, end);
            let (tok, next) = match result {
                Ok(TokenResult { token, next_pos }) => (token, next_pos),
                Err(_) => return (XmlError::InvalidToken, pos),
            };

            // Track event position and byte count for handler callbacks
            self.event_pos = pos;
            self.event_cur_byte_count = (next - pos) as i32;
            self.event_cur_data = data[pos..next].to_vec();

            match tok {
                XmlTok::CdataSectClose => {
                    if let Some(handler) = &mut self.end_cdata_section_handler {
                        handler();
                    } else if self.default_handler.is_some()
                        || self.default_handler_expand.is_some()
                    {
                        self.report_default(enc, data, pos, next);
                    }
                    // Signal that CDATA section has closed
                    self.processor = Processor::Content;
                    return (XmlError::None, next);
                }
                XmlTok::DataNewline => {
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(b"\n");
                    } else {
                        self.report_default(enc, data, pos, next);
                    }
                }
                XmlTok::DataChars => {
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&data[pos..next]);
                    } else {
                        self.report_default(enc, data, pos, next);
                    }
                }
                XmlTok::Invalid => {
                    return (XmlError::InvalidToken, next);
                }
                XmlTok::PartialChar => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    return (XmlError::PartialChar, pos);
                }
                XmlTok::Partial => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    return (XmlError::UnclosedCdataSection, pos);
                }
                XmlTok::None => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    return (XmlError::UnclosedCdataSection, pos);
                }
                XmlTok::TrailingRsqb => {
                    // Trailing ]] — need more data to determine if ]]>
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    // Final buffer — deliver the ]] as character data
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&data[pos..next]);
                    }
                }
                _ => {}
            }

            // Check parsing state after handler calls
            match self.parsing_state {
                ParsingState::Suspended => {
                    return (XmlError::None, next);
                }
                ParsingState::Finished => {
                    return (XmlError::Aborted, next);
                }
                _ => {}
            }

            pos = next;
        }
    }

    /// Extract attributes from a start tag token span.
    /// Uses get_atts from xmltok_impl.
    /// Returns Err(XmlError::DuplicateAttribute) if any attribute name appears twice.
    /// Performs XML attribute value normalization per spec section 3.3.3:
    ///   - Expand character references (&#NN; &#xNN;)
    ///
    /// Process an ignore section: `<![IGNORE[ ... ]]>`
    /// Scans from start position for `]]>` while tracking nested `<![`
    /// Also validates characters inside the section (no invalid XML chars, no partial UTF-8)
    fn do_ignore_section(&self, data: &[u8], start: usize, end: usize) -> (XmlError, usize) {
        let mut level = 1; // Already inside one IGNORE section
        let mut i = start;

        while i < end {
            // Check for closing ]]>
            if i + 3 <= end && &data[i..i + 3] == b"]]>" {
                level -= 1;
                if level == 0 {
                    return (XmlError::None, i + 3);
                }
                i += 3;
            // Check for nested <![
            } else if i + 3 <= end && &data[i..i + 3] == b"<![" {
                level += 1;
                i += 3;
            // Check for invalid XML characters and partial UTF-8
            } else {
                let byte = data[i];

                // Check for invalid XML character (control chars except tab, LF, CR)
                if byte < 0x20 && byte != 0x09 && byte != 0x0A && byte != 0x0D {
                    // Invalid control character
                    return (XmlError::InvalidToken, start);
                }

                // Check for UTF-8 sequence validity
                if byte >= 0x80 {
                    // Multi-byte UTF-8 sequence
                    let bytes_needed = if byte & 0xE0 == 0xC0 {
                        2
                    } else if byte & 0xF0 == 0xE0 {
                        3
                    } else if byte & 0xF8 == 0xF0 {
                        4
                    } else {
                        // Invalid lead byte or continuation byte
                        return (XmlError::InvalidToken, start);
                    };

                    if i + bytes_needed > end {
                        // Incomplete UTF-8 sequence
                        return (XmlError::PartialChar, start);
                    }

                    // Validate continuation bytes
                    for j in 1..bytes_needed {
                        if data[i + j] & 0xC0 != 0x80 {
                            // Invalid continuation byte
                            return (XmlError::InvalidToken, start);
                        }
                    }

                    i += bytes_needed;
                } else {
                    i += 1;
                }
            }
        }

        // Incomplete IGNORE section (no closing ]]> found)
        (XmlError::Syntax, start)
    }

    /// Add a content element child to the current group in the content model stack
    fn add_content_element(&mut self, tok_text: &[u8], quant: ContentQuant) {
        // For quantified elements, the tokenizer includes the quantifier character at the end
        // The C code subtracts minBytesPerChar (1 for UTF-8) when extracting the name
        // So we need to strip the last character for Opt, Rep, Plus quantifiers
        let name_bytes = match quant {
            ContentQuant::None => tok_text,
            ContentQuant::Opt | ContentQuant::Rep | ContentQuant::Plus => {
                // Strip the last character (the quantifier)
                if tok_text.is_empty() {
                    tok_text
                } else {
                    &tok_text[..tok_text.len() - 1]
                }
            }
        };
        let name = std::str::from_utf8(name_bytes).unwrap_or("").to_string();
        let child = ContentNode {
            content_type: ContentType::Name,
            quant,
            children: Vec::new(),
            name: Some(name),
        };
        if let Some(parent) = self.content_model_stack.last_mut() {
            parent.children.push(child);
        }
    }

    /// Close a content group and either nest it in parent or call the handler
    fn close_content_group(&mut self, quant: ContentQuant) {
        self.group_connectors.pop();
        if self.content_model_stack.len() > 1 {
            let mut group = self.content_model_stack.pop().unwrap();
            group.quant = quant;
            if let Some(parent) = self.content_model_stack.last_mut() {
                parent.children.push(group);
            }
        } else if self.content_model_stack.len() == 1 {
            // Outermost group — set quant and call handler
            if let Some(group) = self.content_model_stack.last_mut() {
                group.quant = quant;
            }
            if let Some(ref name) = self.current_element_decl_name.clone() {
                // Serialize the model before calling handler
                self.serialize_content_model();
                if let Some(handler) = &mut self.element_decl_handler {
                    handler(name, "");
                }
            }
            self.current_element_decl_name = None;
            self.content_model_stack.clear();
        }
    }

    /// Serialize the content model tree into a flat array format
    /// Returns: Vec<(type_u32, quant_u32, name_bytes, numchildren_u32)>
    fn serialize_content_model(&mut self) {
        if self.content_model_stack.is_empty() {
            self.last_content_model = Some(Vec::new());
            return;
        }
        let root = &self.content_model_stack[0];
        let mut result = Vec::new();
        // Simple pre-order depth-first traversal
        Self::flatten_content_node(root, &mut result);
        self.last_content_model = Some(result);
    }

    /// Helper to recursively flatten a ContentNode tree into a flat array
    /// Order: node, all its immediate children, then descendants of those children (breadth-first ordering)
    fn flatten_content_node(
        node: &ContentNode,
        result: &mut Vec<(u32, u32, Option<Vec<u8>>, u32)>,
    ) {
        let name_bytes = node.name.as_ref().map(|s| {
            let mut bytes = s.as_bytes().to_vec();
            bytes.push(0); // null-terminate
            bytes
        });
        result.push((
            node.content_type as u32,
            node.quant as u32,
            name_bytes,
            node.children.len() as u32,
        ));

        // Add all immediate children first (breadth-first level 1)
        for child in &node.children {
            let name_bytes = child.name.as_ref().map(|s| {
                let mut bytes = s.as_bytes().to_vec();
                bytes.push(0);
                bytes
            });
            result.push((
                child.content_type as u32,
                child.quant as u32,
                name_bytes,
                child.children.len() as u32,
            ));
        }

        // Then process deeper levels - for each child, process its children but not the child itself
        for child in &node.children {
            // Recursively process the children of this child (grandchildren of original node)
            // but we've already added the child itself, so skip it in the recursion
            for grandchild in &child.children {
                // Process grandchild and its descendants
                Self::flatten_content_node(grandchild, result);
            }
        }
    }

    /// Get the last serialized content model for C FFI access
    #[allow(clippy::type_complexity)]
    pub fn last_content_model(&self) -> Option<&Vec<(u32, u32, Option<Vec<u8>>, u32)>> {
        self.last_content_model.as_ref()
    }

    /// - Expand predefined entity references (&amp; &lt; &gt; &apos; &quot;)
    /// - Expand internal general entity references
    /// - Normalize whitespace (\t, \n, \r, \r\n → space)
    fn extract_attrs<E: Encoding>(
        &self,
        enc: &E,
        data: &[u8],
        start: usize,
        _end: usize,
    ) -> Result<Vec<(String, String)>, XmlError> {
        let max_atts = 64; // reasonable upper bound
        let (_, atts) = xmltok_impl::get_atts(enc, data, start, max_atts);
        let mut result = Vec::with_capacity(atts.len());
        let mut seen = std::collections::HashSet::new();
        for attr in &atts {
            let name = std::str::from_utf8(&data[attr.name..attr.name_end])
                .unwrap_or("")
                .to_string();
            let raw_value = &data[attr.value_ptr..attr.value_end];
            // Always normalize: expand refs, normalize \t \n \r → space
            let value =
                Self::normalize_attribute_value(raw_value, &self.dtd.borrow().internal_entities);
            if value.contains('\x00') {
                return Err(XmlError::RecursiveEntityRef);
            }
            if !seen.insert(name.clone()) {
                return Err(XmlError::DuplicateAttribute);
            }
            result.push((name, value));
        }
        Ok(result)
    }

    /// Check if an entity value contains a cycle (recursive entity references)
    fn entity_value_contains_cycle(
        entity_name: &str,
        value: &[u8],
        entities: &std::collections::HashMap<String, String>,
        visited: &mut std::collections::HashSet<String>,
    ) -> bool {
        if !visited.insert(entity_name.to_string()) {
            return true; // Cycle detected
        }
        // Scan value for entity references
        let mut i = 0;
        while i < value.len() {
            if value[i] == b'&' {
                if let Some(semi) = value[i + 1..].iter().position(|&b| b == b';') {
                    let ref_name = &value[i + 1..i + 1 + semi];
                    if !ref_name.starts_with(b"#") {
                        if let Ok(name) = std::str::from_utf8(ref_name) {
                            if !matches!(name, "amp" | "lt" | "gt" | "apos" | "quot") {
                                if let Some(child_value) = entities.get(name) {
                                    if Self::entity_value_contains_cycle(
                                        name,
                                        child_value.as_bytes(),
                                        entities,
                                        visited,
                                    ) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                    i = i + 2 + semi;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        visited.remove(entity_name);
        false
    }

    /// Normalize an attribute value per XML spec section 3.3.3:
    /// - Replace \t, \n, \r with space; \r\n with single space
    /// - Expand character references (&#NN; &#xNN;)
    /// - Expand predefined entity references (&amp; &lt; &gt; &apos; &quot;)
    /// - Expand internal general entity references
    fn normalize_attribute_value(
        raw: &[u8],
        entities: &std::collections::HashMap<String, String>,
    ) -> String {
        let mut result = Vec::with_capacity(raw.len());
        let mut i = 0;
        while i < raw.len() {
            match raw[i] {
                b'&' => {
                    // Find the semicolon
                    if let Some(semi_offset) = raw[i + 1..].iter().position(|&b| b == b';') {
                        let ref_content = &raw[i + 1..i + 1 + semi_offset];
                        if ref_content.starts_with(b"#x") || ref_content.starts_with(b"#X") {
                            // Hex character reference
                            if let Ok(s) = std::str::from_utf8(&ref_content[2..]) {
                                if let Ok(n) = u32::from_str_radix(s, 16) {
                                    if let Some(c) = char::from_u32(n) {
                                        let mut buf = [0u8; 4];
                                        result
                                            .extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                                    }
                                }
                            }
                        } else if ref_content.starts_with(b"#") {
                            // Decimal character reference
                            if let Ok(s) = std::str::from_utf8(&ref_content[1..]) {
                                if let Ok(n) = s.parse::<u32>() {
                                    if let Some(c) = char::from_u32(n) {
                                        let mut buf = [0u8; 4];
                                        result
                                            .extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                                    }
                                }
                            }
                        } else if let Ok(name) = std::str::from_utf8(ref_content) {
                            // Named entity reference
                            match name {
                                "amp" => result.push(b'&'),
                                "lt" => result.push(b'<'),
                                "gt" => result.push(b'>'),
                                "apos" => result.push(b'\''),
                                "quot" => result.push(b'"'),
                                _ => {
                                    // Internal general entity — recursively expand
                                    if let Some(value) = entities.get(name) {
                                        // Check for recursive entity reference
                                        if Self::entity_value_contains_cycle(
                                            name,
                                            value.as_bytes(),
                                            entities,
                                            &mut std::collections::HashSet::new(),
                                        ) {
                                            // Return error marker — caller should detect
                                            return String::from("\x00RECURSIVE");
                                        }
                                        let expanded = Self::normalize_attribute_value(
                                            value.as_bytes(),
                                            entities,
                                        );
                                        result.extend_from_slice(expanded.as_bytes());
                                    } else {
                                        // Unknown entity — keep as-is
                                        result.extend_from_slice(&raw[i..i + 2 + semi_offset]);
                                    }
                                }
                            }
                        }
                        i = i + 2 + semi_offset;
                    } else {
                        result.push(raw[i]);
                        i += 1;
                    }
                }
                b'\r' => {
                    result.push(b' ');
                    i += 1;
                    // \r\n → single space
                    if i < raw.len() && raw[i] == b'\n' {
                        i += 1;
                    }
                }
                b'\n' | b'\t' => {
                    result.push(b' ');
                    i += 1;
                }
                _ => {
                    result.push(raw[i]);
                    i += 1;
                }
            }
        }
        String::from_utf8(result).unwrap_or_default()
    }

    /// Report a processing instruction — corresponds to C reportProcessingInstruction()
    fn report_processing_instruction<E: Encoding>(
        &mut self,
        enc: &E,
        data: &[u8],
        start: usize,
        end: usize,
    ) {
        let minbpc = enc.min_bytes_per_char();
        // PI format: <?target data?>
        let target_start = start + minbpc * 2; // skip '<?'
        let target_len = xmltok_impl::name_length(enc, data, target_start);
        let target =
            std::str::from_utf8(&data[target_start..target_start + target_len]).unwrap_or("");

        // Skip whitespace after target name
        let mut data_start = target_start + target_len;
        let pi_end = end - minbpc * 2; // before '?>'
        data_start = xmltok_impl::skip_s(enc, data, data_start);
        let pi_data = if data_start < pi_end {
            std::str::from_utf8(&data[data_start..pi_end]).unwrap_or("")
        } else {
            ""
        };

        if let Some(handler) = &mut self.processing_instruction_handler {
            handler(target, pi_data);
        } else if let Some(handler) = &mut self.default_handler {
            handler(&data[start..end]);
        }
    }

    /// Report a comment — corresponds to C reportComment()
    fn report_comment<E: Encoding>(&mut self, enc: &E, data: &[u8], start: usize, end: usize) {
        let minbpc = enc.min_bytes_per_char();
        // Comment format: <!--data-->
        let comment_start = start + minbpc * 4; // skip '<!--'
        let comment_end = end - minbpc * 3; // before '-->'
        let comment_data = if comment_start <= comment_end {
            &data[comment_start..comment_end]
        } else {
            &[]
        };
        if let Some(handler) = &mut self.comment_handler {
            handler(comment_data);
        } else if let Some(handler) = &mut self.default_handler {
            handler(&data[start..end]);
        }
    }

    /// Report default content — corresponds to C reportDefault()
    /// Validate and store an entity value — corresponds to C storeEntityValue()
    /// Tokenizes the entity value to validate char refs, detect entity refs, etc.
    /// In external subsets (is_param_entity), resolves parameter entity references
    /// by calling the external entity ref handler or inlining internal PE values.
    /// Returns the validated UTF-8 string or an error.
    fn store_entity_value(&mut self, value_data: &[u8]) -> Result<String, XmlError> {
        let enc = xmltok::Utf8Encoding;
        let end = value_data.len();
        let mut pos = 0;
        let mut result = Vec::new();

        loop {
            let tok_result = xmltok_impl::entity_value_tok(&enc, value_data, pos, end);
            let (tok, next) = match tok_result {
                Ok(TokenResult { token, next_pos }) => (token, next_pos),
                Err(_) => return Err(XmlError::InvalidToken),
            };

            match tok {
                XmlTok::None => break,
                XmlTok::EntityRef | XmlTok::DataChars => {
                    // Entity refs and data chars: append raw bytes
                    result.extend_from_slice(&value_data[pos..next]);
                }
                XmlTok::TrailingCr => {
                    result.push(b'\n');
                }
                XmlTok::DataNewline => {
                    result.push(b'\n');
                }
                XmlTok::CharRef => {
                    let n = xmltok_impl::char_ref_number(&enc, value_data, pos);
                    if n < 0 {
                        return Err(XmlError::BadCharRef);
                    }
                    if let Some(c) = char::from_u32(n as u32) {
                        let mut buf = [0u8; 4];
                        let encoded = c.encode_utf8(&mut buf);
                        result.extend_from_slice(encoded.as_bytes());
                    } else {
                        return Err(XmlError::BadCharRef);
                    }
                }
                XmlTok::Partial => {
                    return Err(XmlError::InvalidToken);
                }
                XmlTok::Invalid => {
                    return Err(XmlError::InvalidToken);
                }
                XmlTok::ParamEntityRef => {
                    // C: storeEntityValue case XML_TOK_PARAM_ENTITY_REF (xmlparse.c:6824-6884)
                    if self.is_param_entity {
                        // In external subset — resolve the PE reference
                        // Extract entity name from %name; token
                        let pe_name = if value_data.len() > pos && value_data[pos] == b'%' {
                            value_data[pos + 1..]
                                .iter()
                                .position(|&b| b == b';')
                                .and_then(|semi| {
                                    std::str::from_utf8(&value_data[pos + 1..pos + 1 + semi])
                                        .ok()
                                        .map(|s| s.to_string())
                                })
                        } else {
                            None
                        };

                        if let Some(name) = pe_name {
                            let pe = self.dtd.borrow().param_entities.get(&name).cloned();

                            if let Some(pe) = pe {
                                // C: entity->open || (entity == parser->m_declEntity)
                                let is_decl_entity =
                                    self.current_entity_name.as_ref() == Some(&name);
                                if pe.open || is_decl_entity {
                                    return Err(XmlError::RecursiveEntityRef);
                                }

                                if let Some(ref system_id) = pe.system_id {
                                    // External PE — call handler
                                    // C: xmlparse.c:6854-6872
                                    if let Some(handler) = self.external_entity_ref_handler.as_mut()
                                    {
                                        self.dtd.borrow_mut().param_entity_read = false;
                                        if let Some(e) =
                                            self.dtd.borrow_mut().param_entities.get_mut(&name)
                                        {
                                            e.open = true;
                                        }
                                        let base = self.base_uri.clone();
                                        let sys_id = system_id.clone();
                                        let pub_id = pe.public_id.clone();
                                        let ok = handler(
                                            "",
                                            base.as_deref(),
                                            Some(sys_id.as_str()),
                                            pub_id.as_deref(),
                                        );
                                        if let Some(e) =
                                            self.dtd.borrow_mut().param_entities.get_mut(&name)
                                        {
                                            e.open = false;
                                        }
                                        if !ok {
                                            return Err(XmlError::ExternalEntityHandling);
                                        }
                                        if !self.dtd.borrow().param_entity_read {
                                            let standalone = self.dtd.borrow().standalone;
                                            self.dtd.borrow_mut().keep_processing = standalone;
                                        }
                                    } else {
                                        // No handler — stop processing unless standalone
                                        let standalone = self.dtd.borrow().standalone;
                                        self.dtd.borrow_mut().keep_processing = standalone;
                                    }
                                } else if let Some(ref value) = pe.value {
                                    // Internal PE — inline the value
                                    // C: processEntity(parser, entity, XML_FALSE, ENTITY_VALUE)
                                    // For entity values, we can inline the text directly
                                    result.extend_from_slice(value.as_bytes());
                                }
                            } else {
                                // Entity not found — not a well-formedness error
                                // C: dtd->keepProcessing = dtd->standalone
                                let standalone = self.dtd.borrow().standalone;
                                self.dtd.borrow_mut().keep_processing = standalone;
                                break; // C: goto endEntityValue
                            }
                        }
                    } else {
                        // In internal subset, PE references in entity values are illegal
                        // C: xmlparse.c:6880-6883
                        return Err(XmlError::ParamEntityRef);
                    }
                }
                _ => {
                    result.extend_from_slice(&value_data[pos..next]);
                }
            }
            pos = next;
        }

        Ok(String::from_utf8(result).unwrap_or_default())
    }

    /// Validate an attribute default value — corresponds to C appendAttributeValue()
    /// Tokenizes the value to reject < and validate entity refs.
    fn validate_attribute_value(&self, value_data: &[u8]) -> Result<(), XmlError> {
        let enc = xmltok::Utf8Encoding;
        let end = value_data.len();
        let mut pos = 0;

        loop {
            let tok_result = xmltok_impl::attribute_value_tok(&enc, value_data, pos, end);
            let (tok, next) = match tok_result {
                Ok(TokenResult { token, next_pos }) => (token, next_pos),
                Err(_) => return Err(XmlError::InvalidToken),
            };

            match tok {
                XmlTok::None => return Ok(()),
                XmlTok::DataChars | XmlTok::DataNewline | XmlTok::TrailingCr => {}
                XmlTok::CharRef => {
                    let n = xmltok_impl::char_ref_number(&enc, value_data, pos);
                    if n < 0 {
                        return Err(XmlError::BadCharRef);
                    }
                }
                XmlTok::EntityRef => {
                    // Entity ref in attribute default — check it exists
                    let minbpc = enc.min_bytes_per_char();
                    let name_start = pos + minbpc;
                    let name_end = next - minbpc;
                    let _name =
                        std::str::from_utf8(&value_data[name_start..name_end]).unwrap_or("");
                    // In C, appendAttributeValue looks up the entity.
                    // We accept known predefined entities; others are stored as-is.
                }
                XmlTok::Partial | XmlTok::Invalid => {
                    return Err(XmlError::InvalidToken);
                }
                _ => {}
            }
            if next <= pos {
                break;
            }
            pos = next;
        }
        Ok(())
    }

    /// Process namespace declarations and expand element/attribute names
    fn process_namespaces(
        &mut self,
        tag_name: &str,
        attrs: &mut Vec<(String, String)>,
    ) -> Result<String, XmlError> {
        let tag_level = self.tag_level;

        // Phase 1: Extract xmlns declarations and create bindings
        let mut new_bindings = Vec::new();
        let mut i = 0;
        while i < attrs.len() {
            let (ref name, ref value) = attrs[i];
            if name == "xmlns" {
                // Default namespace declaration
                new_bindings.push(("".to_string(), value.clone()));
                attrs.remove(i);
                continue;
            } else if let Some(prefix) = name.strip_prefix("xmlns:") {
                // Prefixed namespace declaration
                // Empty URI is only valid for default namespace
                if value.is_empty() && !prefix.is_empty() {
                    return Err(XmlError::UndeclaringPrefix);
                }
                // Check reserved prefixes
                if prefix == "xmlns" {
                    return Err(XmlError::ReservedPrefixXmlns);
                }
                if prefix == "xml" {
                    // "xml" prefix must be bound to the XML namespace
                    if value != "http://www.w3.org/XML/1998/namespace" {
                        return Err(XmlError::ReservedPrefixXml);
                    }
                }
                // Check reserved namespace URIs
                if value == "http://www.w3.org/XML/1998/namespace" && prefix != "xml" {
                    return Err(XmlError::ReservedNamespaceUri);
                }
                if value == "http://www.w3.org/2000/xmlns/" {
                    return Err(XmlError::ReservedNamespaceUri);
                }
                // Check if namespace separator appears in URI (security check)
                if self.ns_separator != '\0' && !is_rfc3986_uri_char(self.ns_separator) {
                    for ch in value.chars() {
                        if ch == self.ns_separator {
                            return Err(XmlError::Syntax);
                        }
                    }
                }
                new_bindings.push((prefix.to_string(), value.clone()));
                attrs.remove(i);
                continue;
            }
            i += 1;
        }

        // Also check separator in default namespace URIs
        for (prefix, value) in &new_bindings {
            if prefix.is_empty()
                && !value.is_empty()
                && self.ns_separator != '\0'
                && !is_rfc3986_uri_char(self.ns_separator)
            {
                for ch in value.chars() {
                    if ch == self.ns_separator {
                        return Err(XmlError::Syntax);
                    }
                }
            }
        }

        // Apply bindings and call handler
        for (prefix, uri) in &new_bindings {
            let prev = self.ns_map.get(prefix).cloned();
            self.ns_bindings
                .push((tag_level, prefix.clone(), uri.clone(), prev.clone()));
            if uri.is_empty() && prefix.is_empty() {
                // Empty default namespace removes the binding
                self.ns_map.remove(prefix);
            } else {
                self.ns_map.insert(prefix.clone(), uri.clone());
            }
            if let Some(handler) = &mut self.start_namespace_decl_handler {
                let p = if prefix.is_empty() {
                    None
                } else {
                    Some(prefix.as_str())
                };
                handler(p, uri.as_str());
            }
        }

        // Phase 2: Expand element name
        let expanded_name = self.expand_name(tag_name, true)?;

        // Phase 3: Expand attribute names and check for duplicates
        let mut expanded_attrs = Vec::new();
        for (attr_name, attr_val) in attrs.iter_mut() {
            let expanded_attr_name = if attr_name.contains(':') {
                self.expand_name(attr_name, false)?
            } else {
                attr_name.clone()
            };

            // Check for duplicate expanded attribute names
            if expanded_attrs
                .iter()
                .any(|(n, _): &(String, String)| n == &expanded_attr_name)
            {
                return Err(XmlError::DuplicateAttribute);
            }
            expanded_attrs.push((expanded_attr_name, attr_val.clone()));
        }
        *attrs = expanded_attrs;

        Ok(expanded_name)
    }

    /// Expand a name by looking up its prefix in the namespace map
    fn expand_name(&self, name: &str, is_element: bool) -> Result<String, XmlError> {
        if let Some(colon_pos) = name.find(':') {
            let prefix = &name[..colon_pos];
            let local = &name[colon_pos + 1..];
            // Check for double colon or empty prefix/local
            if local.contains(':') || local.is_empty() || prefix.is_empty() {
                return Err(XmlError::InvalidToken);
            }
            if let Some(uri) = self.ns_map.get(prefix) {
                if self.ns_triplets {
                    Ok(format!(
                        "{}{}{}{}{}",
                        uri, self.ns_separator, local, self.ns_separator, prefix
                    ))
                } else {
                    Ok(format!("{}{}{}", uri, self.ns_separator, local))
                }
            } else {
                Err(XmlError::UnboundPrefix)
            }
        } else if is_element {
            // Elements use default namespace if available
            if let Some(uri) = self.ns_map.get("") {
                if !uri.is_empty() {
                    Ok(format!("{}{}{}", uri, self.ns_separator, name))
                } else {
                    Ok(name.to_string())
                }
            } else {
                Ok(name.to_string())
            }
        } else {
            // Attributes without prefix don't get default namespace
            Ok(name.to_string())
        }
    }

    /// Pop namespace bindings for a closing element
    /// `level` is the tag_level at which the bindings were created
    fn pop_ns_bindings(&mut self, level: u32) {
        while let Some(last) = self.ns_bindings.last() {
            if last.0 != level {
                break;
            }
            let (_, prefix, _uri, prev) = self.ns_bindings.pop().unwrap();
            // Restore previous binding
            if let Some(prev_uri) = prev {
                self.ns_map.insert(prefix.clone(), prev_uri);
            } else {
                self.ns_map.remove(&prefix);
            }
            // Call end namespace handler (in reverse order)
            if let Some(handler) = &mut self.end_namespace_decl_handler {
                let p = if prefix.is_empty() {
                    None
                } else {
                    Some(prefix.as_str())
                };
                handler(p);
            }
        }
    }

    fn report_default<E: Encoding>(&mut self, _enc: &E, data: &[u8], start: usize, end: usize) {
        // Call default_handler_expand if set, otherwise call default_handler
        let chunk = &data[start..end];
        if let Some(handler) = &mut self.default_handler_expand {
            handler(chunk);
        } else if let Some(handler) = &mut self.default_handler {
            handler(chunk);
        }
    }

    /// Strip the prefix from a namespace-expanded name
    /// If the name is in format "uri + sep + local + sep + prefix", returns "uri + sep + local"
    /// Otherwise returns the name as-is
    fn strip_triplet(&self, name: &str) -> String {
        // Count separator occurrences
        let sep_count = name.matches(self.ns_separator).count();

        // If we have exactly 2 separators, this is a triplet format
        // Format: "uri + sep + local + sep + prefix"
        // We want to return: "uri + sep + local"
        if sep_count == 2 && self.ns_separator != '\0' {
            // Find the last separator
            if let Some(last_sep_pos) = name.rfind(self.ns_separator) {
                return name[..last_sep_pos].to_string();
            }
        }

        name.to_string()
    }

    /// Extract a qualified name including colons for namespace-qualified names
    /// In non-namespace mode, uses standard name_length.
    /// In namespace mode, includes colons as part of the name.
    fn extract_qualified_name<E: Encoding>(&self, enc: &E, data: &[u8], pos: usize) -> usize {
        if !self.ns_enabled {
            return xmltok_impl::name_length(enc, data, pos);
        }

        // For namespace mode, extract name that may include colons
        let start = pos;
        let minbpc = enc.min_bytes_per_char();
        let mut ptr = pos;

        loop {
            match enc.byte_type(data, ptr) {
                crate::char_tables::ByteType::LEAD2
                | crate::char_tables::ByteType::LEAD3
                | crate::char_tables::ByteType::LEAD4 => {
                    let n = match enc.byte_type(data, ptr) {
                        crate::char_tables::ByteType::LEAD2 => 2,
                        crate::char_tables::ByteType::LEAD3 => 3,
                        crate::char_tables::ByteType::LEAD4 => 4,
                        _ => unreachable!(),
                    };
                    ptr += n;
                }
                crate::char_tables::ByteType::NMSTRT
                | crate::char_tables::ByteType::HEX
                | crate::char_tables::ByteType::DIGIT
                | crate::char_tables::ByteType::NAME
                | crate::char_tables::ByteType::MINUS
                | crate::char_tables::ByteType::COLON => {
                    ptr += minbpc;
                }
                _ => {
                    return ptr - start;
                }
            }
        }
    }

    /// Parse a chunk of XML data. Call repeatedly for incremental/streaming parsing.
    ///
    /// Set `is_final` to `true` on the last chunk to signal end-of-input.
    /// Returns [`XmlStatus::Ok`] on success, [`XmlStatus::Error`] on failure.
    /// Equivalent to `XML_Parse` in the C API.
    pub fn parse(&mut self, data: &[u8], is_final: bool) -> XmlStatus {
        // Check if we're already in an error state
        if self.error_code != XmlError::None {
            return XmlStatus::Error;
        }

        // If parsing was already suspended, reject further parsing unless resumed
        if self.parsing_state == ParsingState::Suspended {
            self.error_code = XmlError::Suspended;
            return XmlStatus::Error;
        }

        // If parsing already finished, reject further parsing
        if self.parsing_state == ParsingState::Finished {
            self.error_code = XmlError::Finished;
            return XmlStatus::Error;
        }

        // If not yet started, transition to parsing
        if self.parsing_state == ParsingState::Initialized {
            self.parsing_state = ParsingState::Parsing;
        }

        // Store the is_final flag
        self.is_final = is_final;

        // Track original-encoding bytes for XML_GetCurrentByteIndex
        self.original_bytes_before_chunk += self.original_chunk.len() as u64;
        self.original_chunk = data.to_vec();

        // Add data to buffer, handling encoding detection on first parse
        // Also allow re-entry if we have buffered data for encoding detection but haven't
        // detected the encoding yet (happens when parsing single bytes)
        if (self.buffer.is_empty() && !self.seen_root && !self.seen_xml_decl)
            || (!self.encoding_detection_buf.is_empty() && self.detected_encoding.is_none())
        {
            // First chunk — check for pre-set encoding validity
            if let Some(ref enc) = self.encoding_name {
                let enc_upper = enc.to_uppercase();
                if enc_upper == "UTF-16LE" || enc_upper == "UTF-16BE" {
                    // Explicit UTF-16 encoding — strip BOM if present, then transcode
                    let is_be = enc_upper == "UTF-16BE";
                    self.detected_encoding = Some(enc_upper);
                    let input = if data.len() >= 2 {
                        let has_bom = if is_be {
                            data[0] == 0xFE && data[1] == 0xFF
                        } else {
                            data[0] == 0xFF && data[1] == 0xFE
                        };
                        if has_bom {
                            &data[2..]
                        } else {
                            data
                        }
                    } else {
                        data
                    };
                    // Handle odd trailing byte
                    let (to_transcode, leftover) = if input.len() % 2 != 0 {
                        (&input[..input.len() - 1], Some(input[input.len() - 1]))
                    } else {
                        (input, None)
                    };
                    self.utf16_pending_byte = leftover;
                    match self.transcode_utf16(to_transcode, is_be) {
                        Ok(transcoded) => {
                            self.buffer = transcoded;
                        }
                        Err(err) => {
                            self.error_code = err;
                            self.parsing_state = ParsingState::Finished;
                            return XmlStatus::Error;
                        }
                    }
                } else if !is_known_encoding(&enc_upper) {
                    self.error_code = XmlError::UnknownEncoding;
                    self.parsing_state = ParsingState::Finished;
                    return XmlStatus::Error;
                } else if self.protocol_encoding_set && is_latin1_encoding(Some(&enc_upper)) {
                    // Explicit Latin-1/ISO-8859-x encoding set via XML_SetEncoding —
                    // bypass BOM detection entirely. Bytes like 0xFF 0xFE are Latin-1
                    // characters (ÿþ), not a UTF-16 BOM.
                    self.detected_encoding = Some(enc_upper);
                    self.buffer = transcode_latin1_to_utf8(data);
                } else if self.protocol_encoding_set
                    && (enc_upper == "UTF-8" || enc_upper == "US-ASCII" || enc_upper == "ASCII")
                {
                    // Explicit UTF-8/ASCII encoding — consume UTF-8 BOM if present,
                    // then treat as UTF-8
                    self.detected_encoding = Some(enc_upper);
                    if data.len() >= 3 && data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF {
                        self.original_chunk_bom_len = 3;
                        self.buffer = data[3..].to_vec();
                    } else {
                        self.buffer = data.to_vec();
                    }
                } else {
                    // Known encoding without protocol override — detect BOM etc
                    match self.detect_and_transcode(data) {
                        Ok(transcoded) => {
                            self.buffer = transcoded;
                            // If buffer is empty, we're waiting for more data for encoding detection
                            // Return early without processing
                            if self.buffer.is_empty() && !self.is_final {
                                // Update position tracking before returning
                                if self.error_code == XmlError::None {
                                    return XmlStatus::Ok;
                                }
                            }
                        }
                        Err(err) => {
                            self.error_code = err;
                            self.parsing_state = ParsingState::Finished;
                            return XmlStatus::Error;
                        }
                    }
                }
            } else {
                // No pre-set encoding — detect from BOM
                match self.detect_and_transcode(data) {
                    Ok(transcoded) => {
                        self.buffer = transcoded;
                        // If buffer is empty, we're waiting for more data for encoding detection
                        // Return early without processing
                        if self.buffer.is_empty() && !self.is_final {
                            // Update position tracking before returning
                            if self.error_code == XmlError::None {
                                return XmlStatus::Ok;
                            }
                        }
                    }
                    Err(err) => {
                        self.error_code = err;
                        self.parsing_state = ParsingState::Finished;
                        return XmlStatus::Error;
                    }
                }
            }
        } else if self.detected_encoding.is_some() {
            let enc_name = self.detected_encoding.as_deref().unwrap_or("");
            if enc_name == "UTF-16BE" || enc_name == "UTF-16LE" {
                // Subsequent chunk with UTF-16 encoding — transcode
                let is_be = enc_name == "UTF-16BE";
                let input = if let Some(pending) = self.utf16_pending_byte.take() {
                    let mut combined = vec![pending];
                    combined.extend_from_slice(data);
                    combined
                } else {
                    data.to_vec()
                };
                let (to_transcode, leftover) = if input.len() % 2 != 0 {
                    (&input[..input.len() - 1], Some(input[input.len() - 1]))
                } else {
                    (&input[..], None)
                };
                self.utf16_pending_byte = leftover;
                match self.transcode_utf16(to_transcode, is_be) {
                    Ok(transcoded) => self.buffer.extend_from_slice(&transcoded),
                    Err(err) => {
                        self.error_code = err;
                        self.parsing_state = ParsingState::Finished;
                        return XmlStatus::Error;
                    }
                }
            } else if is_latin1_encoding(Some(enc_name)) {
                // Latin-1 or similar single-byte encoding — transcode to UTF-8
                self.buffer.extend(transcode_latin1_to_utf8(data));
            } else {
                // UTF-8 or ASCII — append as-is (already UTF-8)
                self.buffer.extend_from_slice(data);
            }
        } else if self.custom_encoding_map.is_some() {
            // Custom encoding — transcode through the encoding map
            // Prepend any pending bytes from incomplete multi-byte sequences
            let input = if self.custom_encoding_pending.is_empty() {
                data.to_vec()
            } else {
                let mut combined = std::mem::take(&mut self.custom_encoding_pending);
                combined.extend_from_slice(data);
                combined
            };
            match self.transcode_custom_encoding(&input) {
                Ok(transcoded) => self.buffer.extend_from_slice(&transcoded),
                Err(err) => {
                    self.error_code = err;
                    self.parsing_state = ParsingState::Finished;
                    return XmlStatus::Error;
                }
            }
        } else {
            self.buffer.extend_from_slice(data);
        }

        // Store parse data and base position for lazy error position calculation
        // (corresponds to C's m_positionPtr / m_eventPtr pattern)
        self.parse_data = self.buffer.clone();
        self.position_pos = 0;
        self.event_pos = self.buffer.len(); // default: end of buffer

        // Run the current processor
        self.run_processor();

        // Reset event byte count and data (only valid during handler callbacks)
        self.event_cur_byte_count = 0;
        self.event_cur_data.clear();

        // Update final position tracking from processed data.
        // After this, line_number/column_number reflect the end of processed data.
        // On error: calculate position up to event_pos (error location)
        // On success: calculate position up to end of all processed data
        // Note: position_pos may have been advanced by inline updates in do_content
        {
            let calc_end =
                if self.error_code != XmlError::None && self.event_pos < self.parse_data.len() {
                    self.event_pos
                } else {
                    self.parse_data.len()
                };
            if self.position_pos < calc_end {
                let enc = xmltok::Utf8Encoding;
                let pos = xmltok_impl::update_position(
                    &enc,
                    &self.parse_data,
                    self.position_pos,
                    calc_end,
                );
                if pos.line_number > 0 {
                    self.line_number += pos.line_number as u64;
                    self.column_number = pos.column_number as u64;
                } else {
                    self.column_number += pos.column_number as u64;
                }
                self.position_pos = calc_end;
            }
            // Mark position as fully up-to-date so lazy getters just return stored values
            self.event_pos = self.position_pos;
        }

        // Track total byte offset (for XML_GetCurrentByteIndex)
        self.byte_offset += data.len() as u64;

        // If the parser was suspended during a handler callback, save remaining data and return Suspended
        if self.parsing_state == ParsingState::Suspended {
            // Save the buffer for resume — the buffer still has unprocessed data
            self.suspended_data = self.buffer.clone();
            self.suspended_is_final = is_final;
            return XmlStatus::Suspended;
        }

        // If an error occurred during processing, return error
        if self.error_code != XmlError::None {
            self.parsing_state = ParsingState::Finished;
            return XmlStatus::Error;
        }

        // If final, check for incomplete document and mark as finished
        if is_final {
            // If we never saw a root element, that's an error
            // BUT: external entities (tag_level > 0) don't require a complete root element
            // AND: foreign DTD subsets don't require a root element
            if !self.seen_root
                && self.tag_level == 0
                && !self.parsing_foreign_dtd
                && self.error_code == XmlError::None
            {
                self.error_code = XmlError::NoElements;
                self.parsing_state = ParsingState::Finished;
                return XmlStatus::Error;
            }
            // If root element was opened but not closed, that's unclosed token
            if self.seen_root
                && !self.root_closed
                && self.tag_level > 0
                && self.error_code == XmlError::None
            {
                self.error_code = XmlError::UnclosedToken;
                self.parsing_state = ParsingState::Finished;
                return XmlStatus::Error;
            }
            self.parsing_state = ParsingState::Finished;
        }

        XmlStatus::Ok
    }

    /// Advance position tracking for a single byte
    fn advance_pos(&mut self, b: u8) {
        if b == b'\n' {
            self.line_number += 1;
            self.column_number = 0;
        } else if b == b'\r' {
            // \r handled as newline (like \n); if \r\n, the \n will reset again
            self.line_number += 1;
            self.column_number = 0;
        } else {
            self.column_number += 1;
        }
    }

    /// Advance position tracking for a slice of bytes
    fn advance_pos_slice(&mut self, data: &[u8]) {
        for &b in data {
            self.advance_pos(b);
        }
    }

    /// Check if the input starts with a UTF-16 BOM and transcode if needed.
    /// Returns the (possibly transcoded) data and the detected encoding name.
    fn detect_and_transcode(&mut self, data: &[u8]) -> Result<Vec<u8>, XmlError> {
        // If we have a pending partial BOM/encoding detection, combine with new data
        if !self.encoding_detection_buf.is_empty() {
            let mut combined = std::mem::take(&mut self.encoding_detection_buf);
            combined.extend_from_slice(data);

            // Check if combined data could still be a partial BOM
            if !self.is_final && combined.len() < 4 {
                // Check if first bytes could be a BOM prefix that needs more bytes
                let could_be_bom = (combined[0] == 0xFF && combined[1] == 0xFE)
                    || (combined[0] == 0xFE && combined[1] == 0xFF)
                    || (combined[0] == 0xEF
                        && (combined.len() < 3 || (combined.len() >= 2 && combined[1] == 0xBB)));
                if could_be_bom {
                    // Still looks like a partial BOM, wait for more bytes
                    self.encoding_detection_buf = combined;
                    return Ok(Vec::new());
                }
                // If we have 2+ bytes and they match UTF-16 pattern (one 0, one not),
                // we have enough to detect—don't buffer
            }

            // Either we have a complete BOM signature or enough bytes to know there's no BOM
            return self.detect_and_transcode_impl(&combined);
        }

        // If non-final and too few bytes to determine encoding, buffer for later
        if !self.is_final && data.len() < 2 {
            // Need at least 2 bytes to detect UTF-16 without BOM
            // Buffer any single byte
            self.encoding_detection_buf = data.to_vec();
            return Ok(Vec::new());
        }

        // If non-final and 2-3 bytes, check if we might need more data for detection
        if !self.is_final && data.len() < 4 {
            // Check if first bytes could be a BOM prefix that needs more bytes
            let could_be_bom = (data[0] == 0xFF && data[1] == 0xFE)
                || (data[0] == 0xFE && data[1] == 0xFF)
                || (data[0] == 0xEF && (data.len() < 3 || data[1] == 0xBB));
            if could_be_bom {
                self.encoding_detection_buf = data.to_vec();
                return Ok(Vec::new());
            }
            // If we have 2+ bytes and they match UTF-16 pattern (one 0, one not),
            // we have enough to detect—don't buffer
        }

        self.detect_and_transcode_impl(data)
    }

    fn detect_and_transcode_impl(&mut self, data: &[u8]) -> Result<Vec<u8>, XmlError> {
        self.original_chunk_bom_len = 0;

        // Use init_scan to detect encoding, matching C libexpat logic exactly
        // is_content_state is false for initial parse (prolog), will be true for external entity content
        let is_content_state = matches!(self.processor, Processor::ExternalEntity);
        let scan_result = xmltok::init_scan(data, is_content_state);

        match scan_result {
            xmltok::InitScanResult::Bom(enc, bom_len) => {
                // BOM found — skip these bytes and set encoding
                self.original_chunk_bom_len = bom_len;
                let data_after_bom = &data[bom_len..];

                match enc {
                    xmltok::DetectedEncoding::Utf8 => {
                        // UTF-8 BOM — just skip it
                        Ok(data_after_bom.to_vec())
                    }
                    xmltok::DetectedEncoding::Utf16BE => {
                        // Check encoding conflict if protocol encoding not set
                        if !self.protocol_encoding_set {
                            if let Some(ref enc_name) = self.encoding_name {
                                let enc_upper = enc_name.to_uppercase();
                                if enc_upper != "UTF-16" && enc_upper != "UTF-16BE" {
                                    return Err(XmlError::IncorrectEncoding);
                                }
                            }
                        }
                        self.detected_encoding = Some("UTF-16BE".to_string());
                        self.transcode_utf16_with_pending(data_after_bom, true)
                    }
                    xmltok::DetectedEncoding::Utf16LE => {
                        // Check encoding conflict if protocol encoding not set
                        if !self.protocol_encoding_set {
                            if let Some(ref enc_name) = self.encoding_name {
                                let enc_upper = enc_name.to_uppercase();
                                if enc_upper != "UTF-16" && enc_upper != "UTF-16LE" {
                                    return Err(XmlError::IncorrectEncoding);
                                }
                            }
                        }
                        self.detected_encoding = Some("UTF-16LE".to_string());
                        self.transcode_utf16_with_pending(data_after_bom, false)
                    }
                }
            }
            xmltok::InitScanResult::Encoding(enc) => {
                // Encoding detected without BOM
                match enc {
                    xmltok::DetectedEncoding::Utf8 => {
                        // UTF-8 encoding, use data as-is
                        Ok(data.to_vec())
                    }
                    xmltok::DetectedEncoding::Utf16BE => {
                        self.detected_encoding = Some("UTF-16BE".to_string());
                        self.transcode_utf16_with_pending(data, true)
                    }
                    xmltok::DetectedEncoding::Utf16LE => {
                        self.detected_encoding = Some("UTF-16LE".to_string());
                        self.transcode_utf16_with_pending(data, false)
                    }
                }
            }
            xmltok::InitScanResult::Partial => {
                if self.is_final {
                    // Final buffer — can't wait for more bytes. Treat as UTF-8.
                    Ok(data.to_vec())
                } else {
                    // Need more bytes for reliable encoding detection
                    // Return empty to signal that we need to buffer and wait
                    Ok(Vec::new())
                }
            }
            xmltok::InitScanResult::None => {
                // No data
                Ok(Vec::new())
            }
        }
    }

    /// Transcode UTF-16 data to UTF-8, saving any odd trailing byte
    fn transcode_utf16_with_pending(
        &mut self,
        data: &[u8],
        big_endian: bool,
    ) -> Result<Vec<u8>, XmlError> {
        let (to_transcode, leftover) = if data.len() % 2 != 0 {
            (&data[..data.len() - 1], Some(data[data.len() - 1]))
        } else {
            (data, None)
        };
        self.utf16_pending_byte = leftover;
        self.transcode_utf16(to_transcode, big_endian)
    }

    /// Transcode UTF-16 data to UTF-8
    fn transcode_utf16(&self, data: &[u8], big_endian: bool) -> Result<Vec<u8>, XmlError> {
        let mut result = Vec::with_capacity(data.len());
        let mut i = 0;
        while i + 1 < data.len() {
            let code_unit = if big_endian {
                ((data[i] as u16) << 8) | (data[i + 1] as u16)
            } else {
                (data[i] as u16) | ((data[i + 1] as u16) << 8)
            };
            i += 2;

            let ch = if (0xD800..=0xDBFF).contains(&code_unit) {
                // High surrogate — need low surrogate
                if i + 1 >= data.len() {
                    return Err(XmlError::PartialChar);
                }
                let low = if big_endian {
                    ((data[i] as u16) << 8) | (data[i + 1] as u16)
                } else {
                    (data[i] as u16) | ((data[i + 1] as u16) << 8)
                };
                i += 2;
                if !(0xDC00..=0xDFFF).contains(&low) {
                    return Err(XmlError::InvalidToken);
                }
                let cp = 0x10000 + ((code_unit as u32 - 0xD800) << 10) + (low as u32 - 0xDC00);
                match char::from_u32(cp) {
                    Some(c) => c,
                    None => return Err(XmlError::InvalidToken),
                }
            } else if (0xDC00..=0xDFFF).contains(&code_unit) {
                return Err(XmlError::InvalidToken);
            } else {
                match char::from_u32(code_unit as u32) {
                    Some(c) => c,
                    None => return Err(XmlError::InvalidToken),
                }
            };

            let mut buf = [0u8; 4];
            let encoded = ch.encode_utf8(&mut buf);
            result.extend_from_slice(encoded.as_bytes());
        }
        Ok(result)
    }

    /// Transcode data from custom encoding to UTF-8
    fn transcode_custom_encoding(&mut self, data: &[u8]) -> Result<Vec<u8>, XmlError> {
        if let Some(ref map) = self.custom_encoding_map {
            let mut result = Vec::new();
            let mut i = 0;

            while i < data.len() {
                let byte = data[i];
                let map_val = map[byte as usize];

                if map_val == -1 {
                    // Malformed byte — error
                    return Err(XmlError::InvalidToken);
                } else if map_val >= 0 {
                    // Single-byte mapping to Unicode codepoint
                    let codepoint = map_val as u32;
                    // Validate codepoint
                    if (0xD800..=0xDFFF).contains(&codepoint) {
                        // Surrogate pair — invalid
                        return Err(XmlError::InvalidToken);
                    }
                    if codepoint > 0x10FFFF {
                        // Out of Unicode range
                        return Err(XmlError::InvalidToken);
                    }
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
                    return Err(XmlError::InvalidToken);
                } else {
                    // Multi-byte sequence: map_val in [-4, -2]
                    let n_bytes = (-map_val) as usize;
                    if i + n_bytes > data.len() {
                        // Not enough bytes — save remaining as pending
                        self.custom_encoding_pending = data[i..].to_vec();
                        return Ok(result);
                    }

                    if let Some(conv_fn) = self.custom_encoding_converter {
                        // Build a buffer for the converter
                        let mut conv_buf = [0u8; 4];
                        conv_buf[..n_bytes].copy_from_slice(&data[i..i + n_bytes]);

                        let codepoint = unsafe {
                            conv_fn(
                                self.custom_encoding_data,
                                conv_buf.as_ptr() as *const std::ffi::c_char,
                            )
                        };

                        if codepoint < 0 {
                            // Converter failed
                            return Err(XmlError::InvalidToken);
                        }

                        // Encode codepoint to UTF-8
                        let codepoint = codepoint as u32;
                        // Validate codepoint
                        if (0xD800..=0xDFFF).contains(&codepoint) {
                            // Surrogate pair — invalid
                            return Err(XmlError::InvalidToken);
                        }
                        if codepoint > 0x10FFFF {
                            // Out of Unicode range
                            return Err(XmlError::InvalidToken);
                        }
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
                        // No converter but multi-byte needed
                        return Err(XmlError::InvalidToken);
                    }
                }
            }

            Ok(result)
        } else {
            Ok(data.to_vec())
        }
    }

    /// Get a buffer for incremental parsing
    ///
    /// Equivalent to XML_GetBuffer(parser, len) in C
    pub fn get_buffer(&mut self, len: usize) -> Option<&mut [u8]> {
        // Resize the get_buffer storage to the requested length
        self.get_buffer_data.resize(len, 0);
        Some(&mut self.get_buffer_data)
    }

    /// Parse data from the internal buffer (populated by get_buffer)
    ///
    /// Equivalent to XML_ParseBuffer(parser, len, isFinal) in C
    pub fn parse_buffer(&mut self, len: usize, is_final: bool) -> XmlStatus {
        if len == 0 {
            return self.parse(&[], is_final);
        }
        if len > self.get_buffer_data.len() {
            self.error_code = XmlError::NoBuffer;
            return XmlStatus::Error;
        }
        let data = self.get_buffer_data[..len].to_vec();
        self.parse(&data, is_final)
    }

    /// Stop parsing (suspendable or abort)
    ///
    /// Equivalent to XML_StopParser(parser, resumable) in C
    pub fn stop(&mut self, resumable: bool) -> XmlStatus {
        if self.parsing_state == ParsingState::Initialized {
            self.error_code = XmlError::NotStarted;
            return XmlStatus::Error;
        }

        if self.parsing_state == ParsingState::Finished {
            self.error_code = XmlError::Finished;
            return XmlStatus::Error;
        }

        if self.parsing_state == ParsingState::Suspended {
            if resumable {
                // Can't suspend an already-suspended parser
                self.error_code = XmlError::Suspended;
                return XmlStatus::Error;
            } else {
                // Aborting a suspended parser is allowed — just finish it
                self.parsing_state = ParsingState::Finished;
                return XmlStatus::Ok;
            }
        }

        if resumable {
            // C: cannot suspend a param entity parser (subordinate parser for DTD)
            if self.parsing_foreign_dtd {
                self.error_code = XmlError::SuspendPe;
                return XmlStatus::Error;
            }
            self.parsing_state = ParsingState::Suspended;
            XmlStatus::Suspended
        } else {
            self.error_code = XmlError::Aborted;
            self.parsing_state = ParsingState::Finished;
            XmlStatus::Error
        }
    }

    /// Resume parsing after suspension
    ///
    /// Equivalent to XML_ResumeParser(parser) in C
    pub fn resume(&mut self) -> XmlStatus {
        if self.parsing_state != ParsingState::Suspended {
            self.error_code = XmlError::NotSuspended;
            return XmlStatus::Error;
        }
        self.parsing_state = ParsingState::Parsing;

        // Re-process the saved data from when we suspended
        if !self.suspended_data.is_empty() || self.suspended_is_final {
            let data = std::mem::take(&mut self.suspended_data);
            let is_final = self.suspended_is_final;
            self.suspended_is_final = false;
            // Clear the buffer since parse() will re-add data
            self.buffer.clear();
            return self.parse(&data, is_final);
        }

        XmlStatus::Ok
    }

    /// Get the current error code
    ///
    /// Equivalent to XML_GetErrorCode(parser) in C
    pub fn error_code(&self) -> XmlError {
        self.error_code
    }

    /// Set the error code directly (used by FFI layer for argument validation)
    pub fn set_error(&mut self, error: XmlError) {
        self.error_code = error;
    }

    /// Get the current line number in the parse
    ///
    /// Equivalent to XML_GetCurrentLineNumber(parser) in C
    pub fn current_line_number(&self) -> u64 {
        // Lazy computation: scan parse_data from position_pos to event_pos
        // to get the current line number during handler callbacks
        if self.event_pos > self.position_pos && !self.parse_data.is_empty() {
            let scan_end = self.event_pos.min(self.parse_data.len());
            let enc = xmltok::Utf8Encoding;
            let p =
                xmltok_impl::update_position(&enc, &self.parse_data, self.position_pos, scan_end);
            self.line_number + p.line_number as u64
        } else {
            self.line_number
        }
    }

    /// Get the current column number in the parse
    ///
    /// Equivalent to XML_GetCurrentColumnNumber(parser) in C
    pub fn current_column_number(&self) -> u64 {
        // Lazy computation: scan parse_data from position_pos to event_pos
        if self.event_pos > self.position_pos && !self.parse_data.is_empty() {
            let scan_end = self.event_pos.min(self.parse_data.len());
            let enc = xmltok::Utf8Encoding;
            let p =
                xmltok_impl::update_position(&enc, &self.parse_data, self.position_pos, scan_end);
            if p.line_number > 0 {
                p.column_number as u64
            } else {
                self.column_number + p.column_number as u64
            }
        } else {
            self.column_number
        }
    }

    /// Get the current byte index in the original input stream.
    ///
    /// Equivalent to XML_GetCurrentByteIndex(parser) in C.
    /// Returns -1 if no event is active.
    ///
    /// For non-UTF-8 encodings, this re-scans the current parse chunk
    /// to convert the internal UTF-8 position back to the original
    /// encoding's byte offset. This is O(chunk_size) but only for
    /// non-UTF-8 input and only when this function is called.
    pub fn current_byte_index(&self) -> i64 {
        if self.parsing_state == ParsingState::Initialized {
            return -1; // Before any parsing, C returns -1
        }
        if self.parse_data.is_empty() || self.event_pos > self.parse_data.len() {
            return -1;
        }

        let utf8_event_pos = self.event_pos;

        match self.detected_encoding.as_deref() {
            None | Some("UTF-8") => {
                // UTF-8: no transcoding, positions match directly
                // Add BOM length back if one was stripped
                self.original_bytes_before_chunk as i64
                    + self.original_chunk_bom_len as i64
                    + utf8_event_pos as i64
            }
            Some("UTF-16LE") | Some("UTF-16BE") => {
                // Re-scan: walk the original chunk's code units, counting
                // how many UTF-8 bytes each produces, until we reach utf8_event_pos
                let is_be = self.detected_encoding.as_deref() == Some("UTF-16BE");
                let orig = &self.original_chunk;
                let bom_len = self.original_chunk_bom_len;
                let mut orig_pos = bom_len; // skip BOM
                let mut utf8_pos = 0usize;

                while orig_pos + 1 < orig.len() && utf8_pos < utf8_event_pos {
                    let code_unit = if is_be {
                        ((orig[orig_pos] as u16) << 8) | (orig[orig_pos + 1] as u16)
                    } else {
                        (orig[orig_pos] as u16) | ((orig[orig_pos + 1] as u16) << 8)
                    };
                    orig_pos += 2;

                    let cp = if (0xD800..=0xDBFF).contains(&code_unit) {
                        // Surrogate pair
                        if orig_pos + 1 < orig.len() {
                            let low = if is_be {
                                ((orig[orig_pos] as u16) << 8) | (orig[orig_pos + 1] as u16)
                            } else {
                                (orig[orig_pos] as u16) | ((orig[orig_pos + 1] as u16) << 8)
                            };
                            orig_pos += 2;
                            0x10000 + ((code_unit as u32 - 0xD800) << 10) + (low as u32 - 0xDC00)
                        } else {
                            break;
                        }
                    } else {
                        code_unit as u32
                    };

                    // Count UTF-8 bytes this code point produces
                    let utf8_len = if cp < 0x80 {
                        1
                    } else if cp < 0x800 {
                        2
                    } else if cp < 0x10000 {
                        3
                    } else {
                        4
                    };
                    utf8_pos += utf8_len;
                }

                self.original_bytes_before_chunk as i64 + orig_pos as i64
            }
            Some(_) => {
                // Latin-1/ASCII: re-scan to count UTF-8 expansion
                let orig = &self.original_chunk;
                let mut orig_pos = 0usize;
                let mut utf8_pos = 0usize;

                while orig_pos < orig.len() && utf8_pos < utf8_event_pos {
                    let byte = orig[orig_pos];
                    orig_pos += 1;
                    // Latin-1 bytes 0x80-0xFF become 2 UTF-8 bytes; rest are 1
                    utf8_pos += if byte >= 0x80 { 2 } else { 1 };
                }

                self.original_bytes_before_chunk as i64 + orig_pos as i64
            }
        }
    }

    /// Get the number of bytes in the current event
    ///
    /// Equivalent to XML_GetCurrentByteCount(parser) in C
    pub fn current_byte_count(&self) -> i32 {
        // During entity expansion, return the entity reference size
        if let Some(entity_ref) = &self.entity_reference_context {
            return entity_ref.len() as i32;
        }
        self.event_cur_byte_count
    }

    /// Get the input context buffer and the offset of the current event within it.
    /// Returns (buffer_slice, event_offset). Empty slice if no context available.
    pub fn get_input_context(&self) -> (&[u8], usize) {
        // If we're in entity expansion, return the entity reference context
        if let Some(entity_ref) = &self.entity_reference_context {
            return (entity_ref, 0);
        }
        if self.parse_data.is_empty() {
            return (&[], 0);
        }
        (&self.parse_data, self.event_pos.min(self.parse_data.len()))
    }

    /// Get parsing status information
    ///
    /// Equivalent to XML_GetParsingStatus(parser, status) in C
    pub fn parsing_status(&self) -> ParsingStatus {
        ParsingStatus {
            state: self.parsing_state,
            final_buffer: self.is_final,
        }
    }

    /// Set the hash salt for DoS protection
    ///
    /// Equivalent to XML_SetHashSalt(parser, salt) in C
    pub fn set_hash_salt(&mut self, salt: u64) -> bool {
        if self.parsing_state != ParsingState::Initialized {
            return false;
        }
        self.hash_salt = salt;
        true
    }

    /// Set the base URI for resolving relative URIs
    ///
    /// Equivalent to XML_SetBase(parser, base) in C
    pub fn set_base(&mut self, base: &str) -> XmlStatus {
        self.base_uri = Some(base.to_string());
        XmlStatus::Ok
    }

    /// Get the base URI
    ///
    /// Equivalent to XML_GetBase(parser) in C
    pub fn base(&self) -> Option<&str> {
        self.base_uri.as_deref()
    }

    /// Set parameter entity parsing mode
    ///
    /// Equivalent to XML_SetParamEntityParsing(parser, parsing) in C
    pub fn set_param_entity_parsing(&mut self, parsing: ParamEntityParsing) -> bool {
        // Can't change once parsing has started
        if self.parsing_state != ParsingState::Initialized {
            return false;
        }
        self.param_entity_parsing = parsing;
        true
    }

    /// Use foreign DTD
    ///
    /// Equivalent to XML_UseForeignDTD(parser, useDTD) in C.
    /// When enabled, the parser will call the external entity ref handler
    /// at the start of parsing, even if the document has no DOCTYPE.
    pub fn use_foreign_dtd(&mut self, use_dtd: bool) -> XmlError {
        if self.parsing_state != ParsingState::Initialized {
            return XmlError::CantChangeFeatureOnceParsing;
        }
        self.foreign_dtd = use_dtd;
        XmlError::None
    }

    /// Set the encoding (before parsing starts)
    ///
    /// Equivalent to XML_SetEncoding(parser, encoding) in C
    pub fn set_encoding(&mut self, encoding: &str) -> XmlStatus {
        if self.parsing_state != ParsingState::Initialized {
            return XmlStatus::Error;
        }
        self.encoding_name = Some(encoding.to_string());
        self.protocol_encoding_set = true;
        XmlStatus::Ok
    }

    /// Clear the encoding setting (NULL encoding in C)
    /// Always succeeds — matches C behavior where NULL encoding is accepted in any state
    pub fn clear_encoding(&mut self) {
        self.encoding_name = None;
        self.protocol_encoding_set = false;
    }

    /// Get the number of specified attributes in the last element
    ///
    /// Equivalent to XML_GetSpecifiedAttributeCount(parser) in C.
    /// Returns the number of attributes that were explicitly specified
    /// (not defaulted from ATTLIST declarations).
    pub fn specified_attribute_count(&self) -> i32 {
        self.n_specified_atts
    }

    /// Get the index of the ID attribute in the last element
    ///
    /// Equivalent to XML_GetIdAttributeIndex(parser) in C.
    /// Returns -1 if there is no ID-type attribute.
    pub fn id_attribute_index(&self) -> i32 {
        self.id_att_index
    }

    /// Register a callback invoked at the start of each element.
    ///
    /// The handler receives `(element_name, attributes)` where `attributes` is
    /// a slice of `(name, value)` pairs. Pass `None` to remove the handler.
    pub fn set_start_element_handler(&mut self, handler: Option<StartElementHandler>) {
        self.start_element_handler = handler;
    }

    /// Set the end element handler
    ///
    /// Handler receives element_name
    pub fn set_end_element_handler(&mut self, handler: Option<EndElementHandler>) {
        self.end_element_handler = handler;
    }

    /// Set both start and end element handlers
    pub fn set_element_handlers(
        &mut self,
        start: Option<StartElementHandler>,
        end: Option<EndElementHandler>,
    ) {
        self.start_element_handler = start;
        self.end_element_handler = end;
    }

    /// Set the character data handler
    ///
    /// Handler receives character data as &[u8]
    pub fn set_character_data_handler(&mut self, handler: Option<CharacterDataHandler>) {
        self.character_data_handler = handler;
    }

    /// Set the processing instruction handler
    ///
    /// Handler receives (target, data)
    pub fn set_processing_instruction_handler(
        &mut self,
        handler: Option<ProcessingInstructionHandler>,
    ) {
        self.processing_instruction_handler = handler;
    }

    /// Set the comment handler
    ///
    /// Handler receives comment data
    pub fn set_comment_handler(&mut self, handler: Option<CommentHandler>) {
        self.comment_handler = handler;
    }

    /// Set the CDATA section handlers
    pub fn set_cdata_section_handlers(
        &mut self,
        start: Option<CdataSectionHandler>,
        end: Option<CdataSectionHandler>,
    ) {
        self.start_cdata_section_handler = start;
        self.end_cdata_section_handler = end;
    }

    /// Set the start CDATA section handler
    pub fn set_start_cdata_section_handler(&mut self, handler: Option<CdataSectionHandler>) {
        self.start_cdata_section_handler = handler;
    }

    /// Set the end CDATA section handler
    pub fn set_end_cdata_section_handler(&mut self, handler: Option<CdataSectionHandler>) {
        self.end_cdata_section_handler = handler;
    }

    /// Set the default handler
    ///
    /// Handler receives raw data as &[u8]
    pub fn set_default_handler(&mut self, handler: Option<DefaultHandler>) {
        self.default_handler = handler;
    }

    /// Set the default handler (without inhibiting internal entity expansion)
    pub fn set_default_handler_expand(&mut self, handler: Option<DefaultHandler>) {
        self.default_handler_expand = handler;
    }

    /// Set the DOCTYPE declaration handlers
    pub fn set_doctype_decl_handlers(
        &mut self,
        start: Option<StartDoctypeDeclHandler>,
        end: Option<EndDoctypeDeclHandler>,
    ) {
        self.start_doctype_decl_handler = start;
        self.end_doctype_decl_handler = end;
    }

    /// Set the start DOCTYPE declaration handler
    pub fn set_start_doctype_decl_handler(&mut self, handler: Option<StartDoctypeDeclHandler>) {
        self.start_doctype_decl_handler = handler;
    }

    /// Set the end DOCTYPE declaration handler
    pub fn set_end_doctype_decl_handler(&mut self, handler: Option<EndDoctypeDeclHandler>) {
        self.end_doctype_decl_handler = handler;
    }

    /// Set the element declaration handler
    ///
    /// Handler receives (element_name, content_model)
    pub fn set_element_decl_handler(&mut self, handler: Option<ElementDeclHandler>) {
        self.element_decl_handler = handler;
    }

    /// Set the attribute list declaration handler
    pub fn set_attlist_decl_handler(&mut self, handler: Option<AttlistDeclHandler>) {
        self.attlist_decl_handler = handler;
    }

    /// Set the XML declaration handler
    pub fn set_xml_decl_handler(&mut self, handler: Option<XmlDeclHandler>) {
        self.xml_decl_handler = handler;
    }

    /// Set the entity declaration handler
    pub fn set_entity_decl_handler(&mut self, handler: Option<EntityDeclHandler>) {
        self.entity_decl_handler = handler;
    }

    /// Set the unparsed entity declaration handler
    pub fn set_unparsed_entity_decl_handler(&mut self, handler: Option<UnparsedEntityDeclHandler>) {
        self.unparsed_entity_decl_handler = handler;
    }

    /// Set the notation declaration handler
    pub fn set_notation_decl_handler(&mut self, handler: Option<NotationDeclHandler>) {
        self.notation_decl_handler = handler;
    }

    /// Set the namespace declaration handlers
    pub fn set_namespace_decl_handlers(
        &mut self,
        start: Option<NamespaceDeclHandler>,
        end: Option<NamespaceDeclEndHandler>,
    ) {
        self.start_namespace_decl_handler = start;
        self.end_namespace_decl_handler = end;
    }

    /// Set the start namespace declaration handler
    pub fn set_start_namespace_decl_handler(&mut self, handler: Option<NamespaceDeclHandler>) {
        self.start_namespace_decl_handler = handler;
    }

    /// Set the end namespace declaration handler
    pub fn set_end_namespace_decl_handler(&mut self, handler: Option<NamespaceDeclEndHandler>) {
        self.end_namespace_decl_handler = handler;
    }

    /// Set the "not standalone" handler
    pub fn set_not_standalone_handler(&mut self, handler: Option<NotStandaloneHandler>) {
        self.not_standalone_handler = handler;
    }

    /// Set the external entity reference handler
    pub fn set_external_entity_ref_handler(&mut self, handler: Option<ExternalEntityRefHandler>) {
        self.external_entity_ref_handler = handler;
    }

    /// Set the skipped entity handler
    pub fn set_skipped_entity_handler(&mut self, handler: Option<SkippedEntityHandler>) {
        self.skipped_entity_handler = handler;
    }

    /// Set the unknown encoding handler
    pub fn set_unknown_encoding_handler(&mut self, handler: Option<UnknownEncodingHandler>) {
        self.unknown_encoding_handler = handler;
    }

    /// Set whether to return namespace triplets
    pub fn set_return_ns_triplet(&mut self, return_triplet: bool) {
        // Namespace triplet support requires full namespace processing.
        // When enabled, element and attribute names are expanded to:
        // uri + separator + localname + separator + prefix
        self.ns_triplets = return_triplet;
    }

    /// Make the parser call handlers with the parser as first argument
    pub fn use_parser_as_handler_arg(&mut self) {
        // Handled in C ABI shim layer
    }

    /// Default current markup to the default handler
    pub fn default_current(&mut self) {
        // Forward the current event's raw bytes to the default handler
        if !self.event_cur_data.is_empty() {
            let data = self.event_cur_data.clone();
            if let Some(handler) = &mut self.default_handler_expand {
                handler(&data);
            } else if let Some(handler) = &mut self.default_handler {
                handler(&data);
            }
        }
    }

    /// Set the billion laughs attack protection maximum amplification
    ///
    /// Equivalent to XML_SetBillionLaughsAttackProtectionMaximumAmplification in C.
    /// Controls the maximum ratio of output text to input text during entity expansion.
    pub fn set_billion_laughs_attack_protection_maximum_amplification(
        &mut self,
        factor: f32,
    ) -> bool {
        if factor < 1.0 && factor != 0.0 {
            return false;
        }
        self.billion_laughs_max_amplification = factor;
        true
    }

    /// Set the billion laughs attack protection activation threshold
    ///
    /// Equivalent to XML_SetBillionLaughsAttackProtectionActivationThreshold in C.
    /// Entity expansion limits only activate after this many bytes of input.
    pub fn set_billion_laughs_attack_protection_activation_threshold(
        &mut self,
        threshold: u64,
    ) -> bool {
        self.billion_laughs_activation_threshold = threshold;
        true
    }

    /// Set the alloc tracker maximum amplification
    pub fn set_alloc_tracker_maximum_amplification(&mut self, _factor: f32) -> bool {
        true // Accept but don't enforce
    }

    /// Set the alloc tracker activation threshold
    pub fn set_alloc_tracker_activation_threshold(&mut self, _threshold: u64) -> bool {
        true // Accept but don't enforce
    }

    /// Set reparse deferral enabled
    pub fn set_reparse_deferral_enabled(&mut self, enabled: bool) -> bool {
        self.reparse_deferral_enabled = enabled;
        true
    }

    /// Create an external entity parser
    ///
    /// Equivalent to XML_ExternalEntityParserCreate(parser, context, encoding) in C
    pub fn create_external_entity_parser(
        &self,
        context: &str,
        encoding: Option<&str>,
    ) -> Option<Parser> {
        let mut child = if self.ns_enabled {
            Parser::new_ns(encoding, self.ns_separator)?
        } else {
            Parser::new(encoding)?
        };
        // If an encoding was passed to XML_ExternalEntityParserCreate, mark it as protocol encoding
        // This matches C behavior where the encoding parameter sets m_protocolEncodingName
        if encoding.is_some() {
            child.protocol_encoding_set = true;
        }
        // Inherit DTD state from parent
        // Matches C: parent and child share m_dtd for DTD subsets, clone for content entities
        if context.is_empty() {
            // DTD child — share DTD state (modifications visible to parent)
            child.dtd = Rc::clone(&self.dtd);
        } else {
            // Content child — clone DTD state (isolated snapshot)
            child.dtd = Rc::new(RefCell::new(self.dtd.borrow().clone()));
        }
        child.param_entity_parsing = self.param_entity_parsing;
        // Foreign DTD subset (empty context) is treated as document entity
        // General entities (non-empty context) are not
        child.prolog_state.document_entity = context.is_empty();
        // For non-empty context (general entity), use ExternalEntity processor
        // which accepts optional text declaration then content
        // For empty context (DTD external subset), start in internal subset mode
        // to allow parsing DTD declarations directly
        if !context.is_empty() {
            child.processor = Processor::ExternalEntity;
            child.content_start_tag_level = 1;
            // C sets m_tagLevel=1 in externalEntityInitProcessor3 after detecting
            // the first token. Since our init processor delegates to content_processor
            // which needs tag_level == content_start_tag_level, set it at creation.
            child.tag_level = 1;
            child.is_subordinate = true;
        } else {
            // For DTD external subset (empty context), set up as parameter entity parser
            // C: m_isParamEntity = XML_TRUE; m_processor = externalParEntInitProcessor;
            child.is_param_entity = true;
            child.prolog_state.init_external_entity();
            child.processor = Processor::ExternalParamEntity;
            child.parsing_foreign_dtd = true;
            child.is_subordinate = true;
        }
        Some(child)
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        // Rust's ownership model handles cleanup automatically
        // All Vec and String fields will be dropped naturally
    }
}

/// Check if an encoding name is known/supported
/// Check if an encoding is a Latin-1 variant that needs transcoding
fn is_latin1_encoding(name: Option<&str>) -> bool {
    match name {
        Some(n) => matches!(
            n,
            "ISO-8859-1"
                | "LATIN1"
                | "WINDOWS-1252"
                | "ISO-8859-2"
                | "ISO-8859-3"
                | "ISO-8859-4"
                | "ISO-8859-5"
                | "ISO-8859-6"
                | "ISO-8859-7"
                | "ISO-8859-8"
                | "ISO-8859-9"
        ),
        None => false,
    }
}

/// Transcode Latin-1 (ISO-8859-1) bytes to UTF-8
fn transcode_latin1_to_utf8(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() * 2);
    for &b in data {
        if b < 0x80 {
            result.push(b);
        } else {
            // Latin-1 byte value = Unicode code point
            // Encode as 2-byte UTF-8: 110xxxxx 10xxxxxx
            result.push(0xC0 | (b >> 6));
            result.push(0x80 | (b & 0x3F));
        }
    }
    result
}

/// Validate public ID characters per XML spec
/// PubidChar ::= #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
fn is_valid_public_id(data: &[u8]) -> bool {
    for &b in data {
        match b {
            0x20 | 0x0D | 0x0A => {}                      // whitespace
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {} // alphanumeric
            b'-' | b'\'' | b'(' | b')' | b'+' | b',' | b'.' | b'/' | b':' | b'=' | b'?' | b';'
            | b'!' | b'*' | b'#' | b'@' | b'$' | b'_' | b'%' => {} // special chars
            _ => return false,
        }
    }
    true
}

fn is_known_encoding(name: &str) -> bool {
    matches!(
        name,
        "UTF-8"
            | "US-ASCII"
            | "ASCII"
            | "ISO-8859-1"
            | "LATIN1"
            | "UTF-16"
            | "UTF-16BE"
            | "UTF-16LE"
            | "ISO-8859-2"
            | "ISO-8859-3"
            | "ISO-8859-4"
            | "ISO-8859-5"
            | "ISO-8859-6"
            | "ISO-8859-7"
            | "ISO-8859-8"
            | "ISO-8859-9"
            | "WINDOWS-1252"
    )
}

/// Check if a character is a valid RFC 3986 URI character
fn is_rfc3986_uri_char(c: char) -> bool {
    matches!(c,
        'A'..='Z' | 'a'..='z' | '0'..='9' |  // unreserved
        '-' | '.' | '_' | '~' |               // unreserved
        ':' | '@' | '!' | '$' | '&' | '\'' |  // sub-delims + reserved
        '(' | ')' | '*' | '+' | ',' | ';' |
        '=' | '/' | '?' | '#' | '[' | ']' |
        '%'                                     // pct-encoded
    )
}

// Free functions (not tied to a parser instance)

/// Get the version string
pub fn expat_version() -> &'static str {
    "expat_2.7.5"
}

/// Get version information as a structure
pub fn expat_version_info() -> ExpatVersion {
    ExpatVersion {
        major: 2,
        minor: 7,
        micro: 5,
    }
}

/// Get the error string for an error code
pub fn error_string(code: XmlError) -> &'static str {
    match code {
        XmlError::None => "no error",
        XmlError::NoMemory => "out of memory",
        XmlError::Syntax => "syntax error",
        XmlError::NoElements => "no elements",
        XmlError::InvalidToken => "invalid token",
        XmlError::UnclosedToken => "unclosed token",
        XmlError::PartialChar => "partial character",
        XmlError::TagMismatch => "tag mismatch",
        XmlError::DuplicateAttribute => "duplicate attribute",
        XmlError::JunkAfterDocElement => "junk after doc element",
        XmlError::ParamEntityRef => "param entity ref",
        XmlError::UndefinedEntity => "undefined entity",
        XmlError::RecursiveEntityRef => "recursive entity ref",
        XmlError::AsyncEntity => "async entity",
        XmlError::BadCharRef => "bad char ref",
        XmlError::BinaryEntityRef => "binary entity ref",
        XmlError::AttributeExternalEntityRef => "attribute external entity ref",
        XmlError::MisplacedXmlPi => "misplaced xml pi",
        XmlError::UnknownEncoding => "unknown encoding",
        XmlError::IncorrectEncoding => "incorrect encoding",
        XmlError::UnclosedCdataSection => "unclosed cdata section",
        XmlError::ExternalEntityHandling => "external entity handling",
        XmlError::NotStandalone => "not standalone",
        XmlError::UnexpectedState => "unexpected state",
        XmlError::EntityDeclaredInPe => "entity declared in pe",
        XmlError::FeatureRequiresXmlDtd => "feature requires xml dtd",
        XmlError::CantChangeFeatureOnceParsing => "can't change feature once parsing",
        XmlError::UnboundPrefix => "unbound prefix",
        XmlError::UndeclaringPrefix => "undeclaring prefix",
        XmlError::IncompletePe => "incomplete pe",
        XmlError::XmlDecl => "xml decl",
        XmlError::TextDecl => "text decl",
        XmlError::Publicid => "publicid",
        XmlError::Suspended => "suspended",
        XmlError::NotSuspended => "not suspended",
        XmlError::Aborted => "aborted",
        XmlError::Finished => "finished",
        XmlError::SuspendPe => "suspend pe",
        XmlError::ReservedPrefixXml => "reserved prefix xml",
        XmlError::ReservedPrefixXmlns => "reserved prefix xmlns",
        XmlError::ReservedNamespaceUri => "reserved namespace uri",
        XmlError::InvalidArgument => "invalid argument",
        XmlError::NoBuffer => "no buffer",
        XmlError::AmplificationLimitBreach => "amplification limit breach",
        XmlError::NotStarted => "not started",
    }
}

// Static feature list
static FEATURE_LIST: &[Feature] = &[
    Feature::Unicode,
    Feature::Dtd,
    Feature::ContextBytes,
    Feature::MinSize,
    Feature::SizeofXmlChar,
    Feature::SizeofXmlLchar,
    Feature::Ns,
    Feature::LargeSize,
    Feature::AttrInfo,
    Feature::BillionLaughsAttackProtectionMaximumAmplificationDefault,
    Feature::BillionLaughsAttackProtectionActivationThresholdDefault,
    Feature::Ge,
    Feature::AllocTrackerMaximumAmplificationDefault,
    Feature::AllocTrackerActivationThresholdDefault,
    Feature::End,
];

/// Get the list of features
pub fn get_feature_list() -> &'static [Feature] {
    FEATURE_LIST
}
