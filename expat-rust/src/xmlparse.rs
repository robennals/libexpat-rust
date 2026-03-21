// AI-generated port of xmlparse.c — 1:1 function correspondence with C

use crate::xmltok;
use crate::xmltok_impl::{self, Encoding, TokenResult, XmlTok};
use crate::xmlrole::{self, Role, XmlRoleState};
use std::collections::HashMap;

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
type AttlistDeclHandler = Box<dyn FnMut(&str, &str, &str, Option<&str>, Option<&str>, bool) + 'static>;
type XmlDeclHandler = Box<dyn FnMut(Option<&str>, Option<&str>, Option<i32>) + 'static>;
type EntityDeclHandler = Box<dyn FnMut(&str, bool, Option<&str>, Option<&str>, Option<&str>) + 'static>;
type UnparsedEntityDeclHandler = Box<dyn FnMut(&str, Option<&str>, &str, Option<&str>) + 'static>;
type NotationDeclHandler = Box<dyn FnMut(&str, Option<&str>, &str, Option<&str>) + 'static>;
type NamespaceDeclHandler = Box<dyn FnMut(Option<&str>, &str) + 'static>;
type NamespaceDeclEndHandler = Box<dyn FnMut(Option<&str>) + 'static>;
type NotStandaloneHandler = Box<dyn FnMut() -> bool + 'static>;
type ExternalEntityRefHandler = Box<dyn FnMut(&str, Option<&str>, Option<&str>, Option<&str>) -> bool + 'static>;
type SkippedEntityHandler = Box<dyn FnMut(&str, bool) + 'static>;
type UnknownEncodingHandler = Box<dyn FnMut(&str) -> bool + 'static>;

/// XML parsing status result type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlStatus {
    Error = 0,
    Ok = 1,
    Suspended = 2,
}

/// XML error codes
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

/// Processor type enumeration — idiomatic Rust alternative to C function pointers
///
/// The parser uses a processor-based architecture similar to the C implementation:
/// 1. PrologInit: Initial processor that detects encoding (maps to prologInitProcessor in C)
/// 2. Prolog: Processes XML declaration, DOCTYPE, comments, PIs (maps to prologProcessor in C)
/// 3. Content: Processes element content (maps to contentProcessor in C)
/// 4. Epilog: Processes after root element closes (maps to epilogProcessor in C)
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

/// Opaque XML Parser structure with full state tracking
pub struct Parser {
    /// Parse buffer for incremental parsing
    buffer: Vec<u8>,
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
    /// Declared encoding name
    encoding_name: Option<String>,
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
    /// Whether we've seen the root element
    seen_root: bool,
    /// Whether the root element has been closed
    root_closed: bool,
    /// Whether we've seen an XML declaration
    seen_xml_decl: bool,
    /// Detected encoding from BOM/auto-detection
    detected_encoding: Option<String>,
    /// Total byte offset in input (for tracking position across parse calls)
    byte_offset: u64,
    /// Internal entity definitions — maps entity name to replacement text
    internal_entities: HashMap<String, String>,
    /// XML role state machine for prolog parsing
    prolog_state: XmlRoleState,

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
}

impl Parser {
    /// Create a new parser with the specified encoding
    ///
    /// Equivalent to XML_ParserCreate(encoding) in C
    pub fn new(encoding: Option<&str>) -> Option<Parser> {
        Some(Parser {
            buffer: Vec::new(),
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
            ns_enabled: false,
            ns_separator: '\0',
            tag_level: 0,
            hash_salt: 0,
            base_uri: None,
            param_entity_parsing: ParamEntityParsing::Never,
            reparse_deferral_enabled: false,
            tag_stack: Vec::new(),
            seen_root: false,
            root_closed: false,
            seen_xml_decl: false,
            detected_encoding: None,
            byte_offset: 0,
            internal_entities: HashMap::new(),
            prolog_state: XmlRoleState::new(),
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
        })
    }

    /// Create a new parser with namespace processing
    ///
    /// Equivalent to XML_ParserCreateNS(encoding, sep) in C
    pub fn new_ns(encoding: Option<&str>, separator: char) -> Option<Parser> {
        Some(Parser {
            buffer: Vec::new(),
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
            ns_enabled: true,
            ns_separator: separator,
            tag_level: 0,
            hash_salt: 0,
            base_uri: None,
            param_entity_parsing: ParamEntityParsing::Never,
            reparse_deferral_enabled: false,
            tag_stack: Vec::new(),
            seen_root: false,
            root_closed: false,
            seen_xml_decl: false,
            detected_encoding: None,
            byte_offset: 0,
            internal_entities: HashMap::new(),
            prolog_state: XmlRoleState::new(),
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
        })
    }

    /// Reset the parser for reuse
    ///
    /// Equivalent to XML_ParserReset(parser, encoding) in C
    pub fn reset(&mut self, encoding: Option<&str>) -> bool {
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
        self.tag_level = 0;
        self.tag_stack.clear();
        self.seen_root = false;
        self.root_closed = false;
        self.seen_xml_decl = false;
        self.detected_encoding = None;
        self.byte_offset = 0;
        self.internal_entities.clear();
        self.prolog_state = XmlRoleState::new();
        true
    }

    /// Run the current processor on the buffered data
    fn run_processor(&mut self) {
        let processor = self.processor;
        match processor {
            Processor::PrologInit => self.prolog_init_processor(),
            Processor::Prolog => self.prolog_processor(),
            Processor::Content => self.content_processor(),
            Processor::Epilog => self.epilog_processor(),
        }
    }

    /// Initial prolog processor — detects encoding and transitions to prolog processor
    fn prolog_init_processor(&mut self) {
        // For now, skip encoding detection and go straight to prolog
        // In a full implementation, this would call initializeEncoding()
        self.processor = Processor::Prolog;
        self.prolog_processor();
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
    fn prolog_processor(&mut self) {
        let data = std::mem::take(&mut self.buffer);
        if data.is_empty() {
            if self.is_final && !self.seen_root {
                self.error_code = XmlError::NoElements;
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
            self.buffer = data[next_pos..].to_vec();
            self.content_processor();
            return;
        }

        // Keep unprocessed data for next parse call
        if next_pos < data.len() {
            self.buffer = data[next_pos..].to_vec();
        }
    }

    /// Main prolog parsing loop — corresponds to C doProlog()
    /// Uses prolog_tok from xmltok_impl to tokenize, and xml_token_role from xmlrole to determine roles
    fn do_prolog(
        &mut self,
        enc: &xmltok::Utf8Encoding,
        data: &[u8],
        _start: usize,
        end: usize,
        have_more: bool,
    ) -> (XmlError, usize) {
        let mut pos = 0;

        loop {
            // Get the next token from the tokenizer
            let result = xmltok_impl::prolog_tok(enc, data, pos, end);
            let (tok, next) = match result {
                Ok(TokenResult { token, next_pos }) => (token, next_pos),
                Err(err_pos) => {
                    // Check if this is a partial UTF-8 character at the end
                    if have_more && Self::is_partial_utf8_sequence(data, err_pos) {
                        // Save the remaining data for next parse call
                        self.buffer = data[err_pos..].to_vec();
                        return (XmlError::None, end);
                    }
                    return (XmlError::InvalidToken, pos);
                }
            };

            match tok {
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
                    return (XmlError::UnclosedToken, pos);
                }
                XmlTok::Invalid => {
                    // Invalid token
                    return (XmlError::InvalidToken, pos);
                }
                _ => {
                    // Convert token type to role token type
                    let role_tok = Self::xmltok_to_role_token(tok);

                    // Extract token text for keyword matching
                    let tok_text = self.extract_token_text(tok, data, pos, next);

                    // Get the role for this token
                    let role = xmlrole::xml_token_role(&mut self.prolog_state, role_tok, &tok_text, &[]);

                    // Dispatch on role
                    let error = self.handle_prolog_role(role, tok, data, pos, next);
                    if error != XmlError::None {
                        return (error, pos);
                    }

                    // If processor changed to Content, break out — remaining data
                    // will be processed by content_processor
                    if self.processor == Processor::Content {
                        // InstanceStart: the token was the start tag, but
                        // content_tok needs to re-tokenize it, so return pos
                        // (not next) so the start tag is included in content data
                        return (XmlError::None, pos);
                    }

                    // Update position for next iteration
                    self.advance_pos_slice(&data[pos..next]);
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
                if pos + minbpc <= next && next >= pos + minbpc {
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
    /// Dispatches based on the role and calls the appropriate handler
    fn handle_prolog_role(
        &mut self,
        role: xmlrole::Role,
        tok: XmlTok,
        data: &[u8],
        pos: usize,
        next: usize,
    ) -> XmlError {
        match role {
            Role::XmlDecl => {
                // Process XML declaration
                if self.seen_xml_decl || self.byte_offset > 0 {
                    return XmlError::MisplacedXmlPi;
                }
                self.seen_xml_decl = true;
                // TODO: parse the actual declaration using xmltok::parse_xml_decl
                XmlError::None
            }
            Role::DoctypeName => {
                // Store DOCTYPE name for subsequent roles
                XmlError::None
            }
            Role::DoctypePublicId => {
                // Store public ID
                XmlError::None
            }
            Role::DoctypeSystemId => {
                // Store system ID
                XmlError::None
            }
            Role::DoctypeInternalSubset => {
                // Internal subset — call start_doctype_decl_handler
                if let Some(handler) = &mut self.start_doctype_decl_handler {
                    handler("", None, None, true);
                }
                XmlError::None
            }
            Role::DoctypeClose => {
                // End of DOCTYPE — call end_doctype_decl_handler
                if let Some(handler) = &mut self.end_doctype_decl_handler {
                    handler();
                }
                XmlError::None
            }
            Role::InstanceStart => {
                // Start of XML instance (root element)
                self.processor = Processor::Content;
                XmlError::None
            }
            Role::GeneralEntityName | Role::ParamEntityName => {
                // Entity declaration — track for subsequent entity roles
                XmlError::None
            }
            Role::EntityValue => {
                // Entity value
                XmlError::None
            }
            Role::EntityComplete => {
                // End of entity declaration — call entity_decl_handler
                XmlError::None
            }
            Role::NotationName => {
                // Notation declaration
                XmlError::None
            }
            Role::AttlistElementName => {
                // Start of ATTLIST declaration
                XmlError::None
            }
            Role::AttributeName => {
                // Attribute in ATTLIST
                XmlError::None
            }
            Role::ElementName => {
                // Element in ELEMENT declaration
                XmlError::None
            }
            Role::Pi => {
                // Processing instruction
                if tok == XmlTok::Pi {
                    self.report_processing_instruction(&xmltok::Utf8Encoding, data, pos, next);
                }
                XmlError::None
            }
            Role::Comment => {
                // Comment
                if tok == XmlTok::Comment {
                    self.report_comment(&xmltok::Utf8Encoding, data, pos, next);
                }
                XmlError::None
            }
            Role::Error => {
                // Syntax error from role state machine
                XmlError::Syntax
            }
            _ => {
                // Other roles — ignore for now
                XmlError::None
            }
        }
    }

    /// Content processor — corresponds to C contentProcessor()
    /// Uses do_content with the tokenizer for content parsing.
    fn content_processor(&mut self) {
        let data = std::mem::take(&mut self.buffer);
        if data.is_empty() {
            if self.is_final && !self.seen_root {
                self.error_code = XmlError::NoElements;
            }
            return;
        }
        let have_more = !self.is_final;
        let enc = xmltok::Utf8Encoding;

        let (error, next_pos) = self.do_content(0, &enc, &data, 0, data.len(), have_more);

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
        if byte_at_pos >= 0xc0 && byte_at_pos < 0xf8 {
            // This is a lead byte at err_pos
            let expected_bytes = if byte_at_pos >= 0xc0 && byte_at_pos < 0xe0 {
                2  // 2-byte UTF-8 character
            } else if byte_at_pos >= 0xe0 && byte_at_pos < 0xf0 {
                3  // 3-byte UTF-8 character
            } else {
                4  // 4-byte UTF-8 character (0xf0-0xf7)
            };

            let bytes_available = data.len() - err_pos;
            if bytes_available < expected_bytes && Self::all_bytes_valid(&data[err_pos..], expected_bytes) {
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
            if lead_byte >= 0xc0 && lead_byte < 0xf8 {
                // Determine expected byte count from lead byte
                let expected_bytes = if lead_byte >= 0xc0 && lead_byte < 0xe0 {
                    2  // 2-byte UTF-8 character
                } else if lead_byte >= 0xe0 && lead_byte < 0xf0 {
                    3  // 3-byte UTF-8 character
                } else {
                    4  // 4-byte UTF-8 character (0xf0-0xf7)
                };

                // Check if we have fewer bytes than expected from this lead byte to end of data
                let bytes_after_lead = data.len() - pos;
                if bytes_after_lead < expected_bytes && Self::all_bytes_valid(&data[pos..], expected_bytes) {
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
        for i in 1..sequence.len().min(expected_len) {
            if sequence[i] < 0x80 || sequence[i] >= 0xc0 {
                return false;
            }
        }
        true
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
                Ok(TokenResult { token, next_pos }) => {
                    match token {
                        XmlTok::PrologS => {
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
                            self.error_code = XmlError::JunkAfterDocElement;
                            return;
                        }
                        _ => {
                            self.error_code = XmlError::JunkAfterDocElement;
                            return;
                        }
                    }
                }
                Err(err_pos) => {
                    // Check if this is a partial UTF-8 character at the end
                    // Search backwards from err_pos to find the start of a potential UTF-8 lead byte
                    if have_more && Self::is_partial_utf8_sequence(&data, err_pos) {
                        self.buffer = data[err_pos..].to_vec();
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
                    if next_pos == pos && !matches!(token, XmlTok::None | XmlTok::Partial | XmlTok::PartialChar | XmlTok::TrailingCr | XmlTok::TrailingRsqb) {
                        return (XmlError::UnexpectedState, pos);
                    }
                    (token, next_pos)
                }
                Err(err_pos) => {
                    let _ = err_pos;
                    return (XmlError::InvalidToken, pos);
                }
            };

            match tok {
                XmlTok::TrailingCr => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&[b'\n']);
                    }
                    if start_tag_level == 0 && !self.seen_root {
                        return (XmlError::NoElements, next);
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
                    if !self.seen_root {
                        return (XmlError::NoElements, pos);
                    }
                    return (XmlError::None, pos);
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
                    let name_end = next - minbpc;  // skip ';'
                    let ch = xmltok_impl::predefined_entity_name(enc, data, name_start, name_end);
                    if ch != 0 {
                        if let Some(handler) = &mut self.character_data_handler {
                            let mut buf = [0u8; 4];
                            if let Some(c) = char::from_u32(ch as u32) {
                                let encoded = c.encode_utf8(&mut buf);
                                handler(encoded.as_bytes());
                            }
                        }
                    } else {
                        // General entity reference — check internal entities
                        let name = std::str::from_utf8(&data[name_start..name_end]).unwrap_or("");
                        if let Some(value) = self.internal_entities.get(name) {
                            // Internal entity found — substitute its value
                            if let Some(handler) = &mut self.character_data_handler {
                                handler(value.as_bytes());
                            }
                        } else if let Some(handler) = &mut self.skipped_entity_handler {
                            handler(name, false);
                        } else {
                            // No handler and no DTD — report undefined entity
                            // But only if standalone or no param entity refs
                            self.error_code = XmlError::UndefinedEntity;
                            return (XmlError::UndefinedEntity, pos);
                        }
                    }
                }

                XmlTok::StartTagNoAtts | XmlTok::StartTagWithAtts => {
                    let minbpc = enc.min_bytes_per_char();
                    let raw_name_start = pos + minbpc; // skip '<'
                    let raw_name_len = xmltok_impl::name_length(enc, data, raw_name_start);
                    let tag_name = std::str::from_utf8(&data[raw_name_start..raw_name_start + raw_name_len])
                        .unwrap_or("");

                    self.tag_level += 1;
                    self.seen_root = true;

                    // Extract attributes
                    let attrs = if tok == XmlTok::StartTagWithAtts {
                        self.extract_attrs(enc, data, pos, next)
                    } else {
                        Vec::new()
                    };

                    self.tag_stack.push(tag_name.to_string());

                    if let Some(handler) = &mut self.start_element_handler {
                        let attr_refs: Vec<(&str, &str)> =
                            attrs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                        handler(tag_name, &attr_refs);
                    }
                }

                XmlTok::EmptyElementNoAtts | XmlTok::EmptyElementWithAtts => {
                    let minbpc = enc.min_bytes_per_char();
                    let raw_name_start = pos + minbpc;
                    let raw_name_len = xmltok_impl::name_length(enc, data, raw_name_start);
                    let tag_name = std::str::from_utf8(&data[raw_name_start..raw_name_start + raw_name_len])
                        .unwrap_or("")
                        .to_string();

                    self.seen_root = true;

                    let attrs = if tok == XmlTok::EmptyElementWithAtts {
                        self.extract_attrs(enc, data, pos, next)
                    } else {
                        Vec::new()
                    };

                    if let Some(handler) = &mut self.start_element_handler {
                        let attr_refs: Vec<(&str, &str)> =
                            attrs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                        handler(&tag_name, &attr_refs);
                    }
                    if let Some(handler) = &mut self.end_element_handler {
                        handler(&tag_name);
                    }

                    // Check if root element closed (empty root element)
                    if self.tag_level == 0 {
                        self.root_closed = true;
                        self.processor = Processor::Epilog;
                        // Process epilog inline
                        if next < end {
                            let epilog_data = data[next..end].to_vec();
                            self.buffer = epilog_data;
                            self.epilog_processor();
                        }
                        return (self.error_code, end);
                    }
                }

                XmlTok::EndTag => {
                    let minbpc = enc.min_bytes_per_char();
                    let raw_name_start = pos + minbpc * 2; // skip '</'
                    let raw_name_len = xmltok_impl::name_length(enc, data, raw_name_start);
                    let tag_name = std::str::from_utf8(&data[raw_name_start..raw_name_start + raw_name_len])
                        .unwrap_or("");

                    // Check tag mismatch — set event position to rawName
                    // (matches C: *eventPP = rawName)
                    if let Some(expected) = self.tag_stack.last() {
                        if expected != tag_name {
                            self.event_pos = raw_name_start;
                            return (XmlError::TagMismatch, raw_name_start);
                        }
                    } else {
                        self.event_pos = raw_name_start;
                        return (XmlError::TagMismatch, raw_name_start);
                    }

                    self.tag_stack.pop();
                    self.tag_level = self.tag_level.saturating_sub(1);

                    if let Some(handler) = &mut self.end_element_handler {
                        handler(tag_name);
                    }

                    // Check if root element closed
                    if self.tag_level == 0 {
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
                    }
                }

                XmlTok::XmlDecl => {
                    return (XmlError::MisplacedXmlPi, pos);
                }

                XmlTok::DataNewline => {
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&[b'\n']);
                    }
                }

                XmlTok::CdataSectOpen => {
                    if let Some(handler) = &mut self.start_cdata_section_handler {
                        handler();
                    }
                    // Scan CDATA content
                    let (cdata_err, cdata_next) = self.do_cdata_section(enc, data, next, end, have_more);
                    if cdata_err != XmlError::None {
                        return (cdata_err, next);
                    }
                    pos = cdata_next;
                    continue; // don't update pos from next
                }

                XmlTok::TrailingRsqb => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&data[pos..end]);
                    }
                    if start_tag_level == 0 && !self.seen_root {
                        return (XmlError::NoElements, end);
                    }
                    return (XmlError::None, end);
                }

                XmlTok::DataChars => {
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&data[pos..next]);
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
        loop {
            let result = xmltok_impl::cdata_section_tok(enc, data, pos, end);
            let (tok, next) = match result {
                Ok(TokenResult { token, next_pos }) => (token, next_pos),
                Err(_) => return (XmlError::InvalidToken, pos),
            };

            match tok {
                XmlTok::CdataSectClose => {
                    if let Some(handler) = &mut self.end_cdata_section_handler {
                        handler();
                    }
                    return (XmlError::None, next);
                }
                XmlTok::DataNewline => {
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&[b'\n']);
                    }
                }
                XmlTok::DataChars => {
                    if let Some(handler) = &mut self.character_data_handler {
                        handler(&data[pos..next]);
                    }
                }
                XmlTok::None | XmlTok::Partial => {
                    if have_more {
                        return (XmlError::None, pos);
                    }
                    return (XmlError::UnclosedCdataSection, pos);
                }
                _ => {}
            }
            pos = next;
        }
    }

    /// Extract attributes from a start tag token span.
    /// Uses get_atts from xmltok_impl.
    fn extract_attrs<E: Encoding>(
        &self,
        enc: &E,
        data: &[u8],
        start: usize,
        end: usize,
    ) -> Vec<(String, String)> {
        let max_atts = 64; // reasonable upper bound
        let (_, atts) = xmltok_impl::get_atts(enc, data, start, max_atts);
        atts.iter()
            .map(|attr| {
                let name = std::str::from_utf8(&data[attr.name..attr.name_end])
                    .unwrap_or("")
                    .to_string();
                let value = std::str::from_utf8(&data[attr.value_ptr..attr.value_end])
                    .unwrap_or("")
                    .to_string();
                (name, value)
            })
            .collect()
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
        let target = std::str::from_utf8(&data[target_start..target_start + target_len])
            .unwrap_or("");

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
        }
    }

    /// Report a comment — corresponds to C reportComment()
    fn report_comment<E: Encoding>(
        &mut self,
        enc: &E,
        data: &[u8],
        start: usize,
        end: usize,
    ) {
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
        }
    }

    /// Report default content — corresponds to C reportDefault()
    fn report_default<E: Encoding>(
        &mut self,
        _enc: &E,
        data: &[u8],
        start: usize,
        end: usize,
    ) {
        if let Some(handler) = &mut self.default_handler {
            handler(&data[start..end]);
        }
    }

    /// Parse a chunk of data
    ///
    /// Equivalent to XML_Parse(parser, s, len, isFinal) in C
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

        // Add data to buffer, handling encoding detection on first parse
        if self.buffer.is_empty() && !self.seen_root && !self.seen_xml_decl {
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
                        if has_bom { &data[2..] } else { data }
                    } else {
                        data
                    };
                    match self.transcode_utf16(input, is_be) {
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
                } else {
                    // Known encoding, detect BOM etc
                    match self.detect_and_transcode(data) {
                        Ok(transcoded) => self.buffer = transcoded,
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
                    Ok(transcoded) => self.buffer = transcoded,
                    Err(err) => {
                        self.error_code = err;
                        self.parsing_state = ParsingState::Finished;
                        return XmlStatus::Error;
                    }
                }
            }
        } else if self.detected_encoding.is_some() {
            // Subsequent chunk with UTF-16 encoding — transcode
            let is_be = self.detected_encoding.as_deref() == Some("UTF-16BE");
            match self.transcode_utf16(data, is_be) {
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
        // Save base position before processor modifies it via advance_pos
        let base_line = self.line_number;
        let base_column = self.column_number;

        // Run the current processor
        self.run_processor();

        // If error with event_pos set, override the incremental position
        // with lazy calculation from base position
        if self.error_code != XmlError::None && self.event_pos < self.parse_data.len() {
            let enc = xmltok::Utf8Encoding;
            let pos = xmltok_impl::update_position(
                &enc,
                &self.parse_data,
                self.position_pos,
                self.event_pos,
            );
            if pos.line_number > 0 {
                self.line_number = base_line + pos.line_number as u64;
                self.column_number = pos.column_number as u64;
            } else {
                self.line_number = base_line;
                self.column_number = base_column + pos.column_number as u64;
            }
        }

        // If an error occurred during processing, return error
        if self.error_code != XmlError::None {
            self.parsing_state = ParsingState::Finished;
            return XmlStatus::Error;
        }

        // If final, mark as finished
        if is_final {
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
        if data.len() >= 2 {
            // UTF-16 BE BOM: FE FF
            if data[0] == 0xFE && data[1] == 0xFF {
                // Check if user declared a conflicting encoding
                if let Some(ref enc) = self.encoding_name {
                    let enc_upper = enc.to_uppercase();
                    if enc_upper != "UTF-16" && enc_upper != "UTF-16BE" {
                        return Err(XmlError::IncorrectEncoding);
                    }
                }
                self.detected_encoding = Some("UTF-16BE".to_string());
                return self.transcode_utf16(&data[2..], true);
            }
            // UTF-16 LE BOM: FF FE
            if data[0] == 0xFF && data[1] == 0xFE {
                if let Some(ref enc) = self.encoding_name {
                    let enc_upper = enc.to_uppercase();
                    if enc_upper != "UTF-16" && enc_upper != "UTF-16LE" {
                        return Err(XmlError::IncorrectEncoding);
                    }
                }
                self.detected_encoding = Some("UTF-16LE".to_string());
                return self.transcode_utf16(&data[2..], false);
            }
            // Check for UTF-16 without BOM (NUL byte pattern)
            if data.len() >= 4 {
                if data[0] == 0 && data[1] == b'<' {
                    // UTF-16 BE without BOM
                    self.detected_encoding = Some("UTF-16BE".to_string());
                    return self.transcode_utf16(data, true);
                }
                if data[0] == b'<' && data[1] == 0 {
                    // UTF-16 LE without BOM
                    self.detected_encoding = Some("UTF-16LE".to_string());
                    return self.transcode_utf16(data, false);
                }
            }
        }
        // UTF-8 BOM: EF BB BF — skip it
        if data.len() >= 3 && data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF {
            return Ok(data[3..].to_vec());
        }
        Ok(data.to_vec())
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

    /// Process XML declaration attributes (version, encoding, standalone)
    fn process_xml_decl(&mut self, decl_data: &str) -> Result<(), ()> {
        let mut version: Option<String> = None;
        let mut encoding: Option<String> = None;
        let mut standalone: Option<i32> = None;

        let trimmed = decl_data.trim();

        // Parse pseudo-attributes
        let mut remaining = trimmed;
        let mut seen_version = false;
        let mut seen_encoding = false;
        let mut seen_standalone = false;
        let mut attr_count = 0;

        while !remaining.is_empty() {
            // Skip whitespace
            remaining = remaining.trim_start();
            if remaining.is_empty() {
                break;
            }

            // Parse attribute name
            let name_end = remaining
                .find(|c: char| c == '=' || c.is_whitespace())
                .unwrap_or(remaining.len());
            if name_end == 0 {
                self.error_code = XmlError::XmlDecl;
                return Err(());
            }
            let name = &remaining[..name_end];
            remaining = remaining[name_end..].trim_start();

            // Expect '='
            if !remaining.starts_with('=') {
                self.error_code = XmlError::XmlDecl;
                return Err(());
            }
            remaining = remaining[1..].trim_start();

            // Parse attribute value (quoted)
            if remaining.is_empty() {
                self.error_code = XmlError::XmlDecl;
                return Err(());
            }
            let quote = remaining.as_bytes()[0];
            if quote != b'"' && quote != b'\'' {
                self.error_code = XmlError::XmlDecl;
                return Err(());
            }
            remaining = &remaining[1..];
            let value_end = remaining.find(quote as char);
            match value_end {
                Some(end) => {
                    let value = &remaining[..end];
                    remaining = &remaining[end + 1..];

                    match name {
                        "version" => {
                            if attr_count != 0 {
                                // version must be first
                                self.error_code = XmlError::XmlDecl;
                                return Err(());
                            }
                            seen_version = true;
                            version = Some(value.to_string());
                        }
                        "encoding" => {
                            if !seen_version || seen_encoding {
                                self.error_code = XmlError::XmlDecl;
                                return Err(());
                            }
                            seen_encoding = true;
                            encoding = Some(value.to_string());

                            // Validate encoding
                            let enc_upper = value.to_uppercase();
                            if enc_upper == "UTF-16" {
                                // UTF-16 declared in what we're parsing as UTF-8
                                if self.detected_encoding.is_none() {
                                    self.error_code = XmlError::IncorrectEncoding;
                                    return Err(());
                                }
                            } else if !is_known_encoding(&enc_upper) {
                                // Check unknown encoding handler
                                let mut handled = false;
                                if let Some(handler) = &mut self.unknown_encoding_handler {
                                    handled = handler(value);
                                }
                                if !handled {
                                    self.error_code = XmlError::UnknownEncoding;
                                    return Err(());
                                }
                            }
                        }
                        "standalone" => {
                            if !seen_version || seen_standalone {
                                self.error_code = XmlError::XmlDecl;
                                return Err(());
                            }
                            seen_standalone = true;
                            standalone = match value {
                                "yes" => Some(1),
                                "no" => Some(0),
                                _ => {
                                    self.error_code = XmlError::XmlDecl;
                                    return Err(());
                                }
                            };
                        }
                        _ => {
                            self.error_code = XmlError::XmlDecl;
                            return Err(());
                        }
                    }
                    attr_count += 1;
                }
                None => {
                    self.error_code = XmlError::XmlDecl;
                    return Err(());
                }
            }
        }

        // version is required
        if !seen_version {
            self.error_code = XmlError::XmlDecl;
            return Err(());
        }

        // Call XML declaration handler
        if let Some(handler) = &mut self.xml_decl_handler {
            handler(
                version.as_deref(),
                encoding.as_deref(),
                standalone,
            );
        }

        Ok(())
    }

    /// Get a buffer for incremental parsing
    ///
    /// Equivalent to XML_GetBuffer(parser, len) in C
    pub fn get_buffer(&mut self, len: usize) -> Option<&mut [u8]> {
        // Ensure buffer has enough capacity
        if self.buffer.capacity() < len {
            self.buffer.reserve(len - self.buffer.capacity());
        }
        // Return a mutable slice for writing
        Some(&mut self.buffer)
    }

    /// Parse data from the internal buffer
    ///
    /// Equivalent to XML_ParseBuffer(parser, len, isFinal) in C
    pub fn parse_buffer(&mut self, _len: usize, _is_final: bool) -> XmlStatus {
        todo!("not yet implemented")
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

        if resumable {
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
        XmlStatus::Ok
    }

    /// Get the current error code
    ///
    /// Equivalent to XML_GetErrorCode(parser) in C
    pub fn error_code(&self) -> XmlError {
        self.error_code
    }

    /// Get the current line number in the parse
    ///
    /// Equivalent to XML_GetCurrentLineNumber(parser) in C
    pub fn current_line_number(&self) -> u64 {
        self.line_number
    }

    /// Get the current column number in the parse
    ///
    /// Equivalent to XML_GetCurrentColumnNumber(parser) in C
    pub fn current_column_number(&self) -> u64 {
        self.column_number
    }

    /// Get the current byte index in the parse
    ///
    /// Equivalent to XML_GetCurrentByteIndex(parser) in C
    pub fn current_byte_index(&self) -> i64 {
        self.byte_offset as i64
    }

    /// Get the number of bytes in the current event
    ///
    /// Equivalent to XML_GetCurrentByteCount(parser) in C
    pub fn current_byte_count(&self) -> i32 {
        0 // Placeholder
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
        self.param_entity_parsing = parsing;
        true
    }

    /// Use foreign DTD
    ///
    /// Equivalent to XML_UseForeignDTD(parser, useDTD) in C
    pub fn use_foreign_dtd(&mut self, _use_dtd: bool) -> XmlError {
        todo!("not yet implemented")
    }

    /// Set the encoding (before parsing starts)
    ///
    /// Equivalent to XML_SetEncoding(parser, encoding) in C
    pub fn set_encoding(&mut self, encoding: &str) -> XmlStatus {
        if self.parsing_state != ParsingState::Initialized {
            return XmlStatus::Error;
        }
        self.encoding_name = Some(encoding.to_string());
        XmlStatus::Ok
    }

    /// Get the number of specified attributes in the last element
    ///
    /// Equivalent to XML_GetSpecifiedAttributeCount(parser) in C
    pub fn specified_attribute_count(&self) -> i32 {
        todo!("not yet implemented")
    }

    /// Get the index of the ID attribute in the last element
    ///
    /// Equivalent to XML_GetIdAttributeIndex(parser) in C
    pub fn id_attribute_index(&self) -> i32 {
        todo!("not yet implemented")
    }

    /// Get attribute information for the last element
    ///
    /// Equivalent to XML_GetAttributeInfo(parser) in C
    pub fn attribute_info(&self) -> Option<&[AttrInfo]> {
        todo!("not yet implemented")
    }

    /// Set the start element handler
    ///
    /// Handler receives (element_name, attributes)
    /// where attributes is a slice of (name, value) pairs
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
    pub fn set_unparsed_entity_decl_handler(
        &mut self,
        handler: Option<UnparsedEntityDeclHandler>,
    ) {
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
    pub fn set_external_entity_ref_handler(
        &mut self,
        handler: Option<ExternalEntityRefHandler>,
    ) {
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
    pub fn set_return_ns_triplet(&mut self, _return_triplet: bool) {
        // Namespace triplet support requires full namespace processing.
        // For now, accept the call without error.
    }

    /// Make the parser call handlers with the parser as first argument
    pub fn use_parser_as_handler_arg(&mut self) {
        todo!("not yet implemented")
    }

    /// Default current markup to the default handler
    pub fn default_current(&mut self) {
        todo!("not yet implemented")
    }

    /// Set the billion laughs attack protection maximum amplification
    pub fn set_billion_laughs_attack_protection_maximum_amplification(
        &mut self,
        _factor: f32,
    ) -> bool {
        todo!("not yet implemented")
    }

    /// Set the billion laughs attack protection activation threshold
    pub fn set_billion_laughs_attack_protection_activation_threshold(
        &mut self,
        _threshold: u64,
    ) -> bool {
        todo!("not yet implemented")
    }

    /// Set the alloc tracker maximum amplification
    pub fn set_alloc_tracker_maximum_amplification(&mut self, _factor: f32) -> bool {
        todo!("not yet implemented")
    }

    /// Set the alloc tracker activation threshold
    pub fn set_alloc_tracker_activation_threshold(&mut self, _threshold: u64) -> bool {
        todo!("not yet implemented")
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
        _context: &str,
        encoding: Option<&str>,
    ) -> Option<Parser> {
        // Create a new parser with the specified encoding
        Parser::new(encoding)
    }

    /// Allocate memory using the parser's memory management
    pub fn mem_malloc(&mut self, _size: usize) -> Option<*mut u8> {
        todo!("not yet implemented")
    }

    /// Reallocate memory using the parser's memory management
    pub fn mem_realloc(&mut self, _ptr: *mut u8, _size: usize) -> Option<*mut u8> {
        todo!("not yet implemented")
    }

    /// Free memory using the parser's memory management
    pub fn mem_free(&mut self, _ptr: *mut u8) {
        todo!("not yet implemented")
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        // Rust's ownership model handles cleanup automatically
        // All Vec and String fields will be dropped naturally
    }
}

/// Check if an encoding name is known/supported
fn is_known_encoding(name: &str) -> bool {
    matches!(
        name,
        "UTF-8" | "US-ASCII" | "ASCII" | "ISO-8859-1" | "LATIN1"
            | "UTF-16" | "UTF-16BE" | "UTF-16LE"
            | "ISO-8859-2" | "ISO-8859-3" | "ISO-8859-4" | "ISO-8859-5"
            | "ISO-8859-6" | "ISO-8859-7" | "ISO-8859-8" | "ISO-8859-9"
            | "WINDOWS-1252"
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

/// Free a content model (for element declaration handlers)
pub fn free_content_model(_parser: &mut Parser, _model: *mut ()) {
    // Rust's ownership model handles cleanup automatically
}
