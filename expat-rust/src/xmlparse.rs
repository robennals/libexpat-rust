// AI-generated API facade from expat.h — stubs with todo!()

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
    /// Current line number
    line_number: u64,
    /// Current column number
    column_number: u64,
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
        self.is_final = false;
        self.encoding_name = encoding.map(|s| s.to_string());
        self.tag_level = 0;
        self.tag_stack.clear();
        self.seen_root = false;
        self.root_closed = false;
        self.seen_xml_decl = false;
        self.detected_encoding = None;
        self.byte_offset = 0;
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

    /// Prolog processor — processes XML declaration, DOCTYPE, comments, PIs
    fn prolog_processor(&mut self) {
        // Use existing scan_buffer which contains prolog logic
        let _ = self.scan_buffer();
        // scan_buffer updates error_code if needed
    }

    /// Content processor — processes element content
    fn content_processor(&mut self) {
        // Use existing scan_buffer which contains content logic
        let _ = self.scan_buffer();
        // scan_buffer updates error_code if needed
    }

    /// Epilog processor — processes after root element closes
    fn epilog_processor(&mut self) {
        let data = std::mem::take(&mut self.buffer);
        // In epilog, we expect no content, just EOF or whitespace
        if !data.is_empty() && data.iter().any(|&b| b != b' ' && b != b'\t' && b != b'\n' && b != b'\r') {
            self.error_code = XmlError::JunkAfterDocElement;
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
                    // Explicit UTF-16 encoding — transcode
                    let is_be = enc_upper == "UTF-16BE";
                    self.detected_encoding = Some(enc_upper);
                    match self.transcode_utf16(data, is_be) {
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

        // Run the current processor
        self.run_processor();

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

    /// Scan the buffer for XML tokens and call handlers
    ///
    /// This implements the main parsing loop, using the ported xmltok tokenizer functions
    /// (xmltok_impl::prolog_tok and xmltok_impl::content_tok) for tokenization. The current
    /// implementation delegates to helper methods that manage token handling and callbacks,
    /// which could be consolidated in a future refactoring to use the tokenizer more directly.
    ///
    /// Note: The xmltok module provides:
    /// - prolog_tok() - tokenizes prolog content (XML decl, DOCTYPE, PIs, comments)
    /// - content_tok() - tokenizes element content (tags, char data, entity/char refs, etc)
    /// - Supporting functions for scanning attributes, entity references, etc.
    fn scan_buffer(&mut self) -> bool {
        let data = std::mem::take(&mut self.buffer);
        let len = data.len();
        let mut pos = 0;

        // Handle prolog (XML declaration, DOCTYPE, comments, PIs before root element)
        // This section uses scan_prolog which dispatches based on byte patterns.
        // Could be refactored to use crate::xmltok_impl::prolog_tok directly.
        if !self.seen_root && self.processor == Processor::Prolog {
            pos = match self.scan_prolog(&data, pos) {
                Ok(p) => p,
                Err(_) => return false,
            };
            // If we see the root element, transition to content processor
            if self.seen_root {
                self.processor = Processor::Content;
            }
        }

        // Handle content
        while pos < len {
            if data[pos] == b'<' {
                pos += 1;
                self.column_number += 1;

                if pos >= len {
                    if self.is_final {
                        self.error_code = XmlError::UnclosedToken;
                        return false;
                    }
                    break;
                }

                match data[pos] {
                    // Comment <!-- ... -->
                    b'!' if pos + 2 < len && data[pos + 1] == b'-' && data[pos + 2] == b'-' => {
                        self.column_number += 2; // for !-
                        pos += 2;
                        let comment_start = pos + 1; // after the second -
                        self.column_number += 1;
                        pos += 1;
                        match self.find_comment_end(&data, pos) {
                            Some(end_pos) => {
                                let comment_data = &data[comment_start..end_pos];
                                if let Some(handler) = &mut self.comment_handler {
                                    handler(comment_data);
                                }
                                // advance past -->
                                self.advance_pos_slice(&data[pos..end_pos]);
                                self.column_number += 3; // for -->
                                pos = end_pos + 3;
                            }
                            None => {
                                if self.is_final {
                                    self.error_code = XmlError::UnclosedToken;
                                    return false;
                                }
                                break;
                            }
                        }
                    }
                    // CDATA section <![CDATA[...]]>
                    b'!' if pos + 7 < len && &data[pos..pos + 7] == b"[CDATA[" => {
                        self.advance_pos_slice(&data[pos..pos + 7]);
                        pos += 7;
                        match self.find_cdata_end(&data, pos) {
                            Some(end_pos) => {
                                if let Some(handler) = &mut self.start_cdata_section_handler {
                                    handler();
                                }
                                let cdata = &data[pos..end_pos];
                                if let Some(handler) = &mut self.character_data_handler {
                                    handler(cdata);
                                }
                                if let Some(handler) = &mut self.end_cdata_section_handler {
                                    handler();
                                }
                                self.advance_pos_slice(&data[pos..end_pos]);
                                self.column_number += 3; // for ]]>
                                pos = end_pos + 3;
                            }
                            None => {
                                if self.is_final {
                                    self.error_code = XmlError::UnclosedCdataSection;
                                    return false;
                                }
                                break;
                            }
                        }
                    }
                    // DOCTYPE <!DOCTYPE ...>
                    b'!' => {
                        // Should not see DOCTYPE after root element
                        if self.seen_root {
                            self.error_code = XmlError::Syntax;
                            return false;
                        }
                        // Handle as prolog item
                        pos -= 1; // back to '<'
                        self.column_number -= 1;
                        pos = match self.scan_prolog(&data, pos) {
                            Ok(p) => p,
                            Err(_) => return false,
                        };
                    }
                    // Processing instruction <?...?>
                    b'?' => {
                        self.column_number += 1;
                        pos += 1;
                        match self.scan_pi(&data, pos) {
                            Ok((target, pi_data, new_pos)) => {
                                // Check for misplaced XML declaration
                                if target.eq_ignore_ascii_case("xml") {
                                    self.error_code = XmlError::MisplacedXmlPi;
                                    return false;
                                }
                                if let Some(handler) = &mut self.processing_instruction_handler {
                                    handler(&target, &pi_data);
                                }
                                pos = new_pos;
                            }
                            Err(_) => return false,
                        }
                    }
                    // End tag </...>
                    b'/' => {
                        self.column_number += 1;
                        pos += 1;
                        let (tag_name, new_pos) = self.scan_end_tag(&data, pos);
                        if new_pos == 0 {
                            return false;
                        }

                        if let Some(expected) = self.tag_stack.last() {
                            if expected != &tag_name {
                                self.error_code = XmlError::TagMismatch;
                                return false;
                            }
                        } else {
                            self.error_code = XmlError::TagMismatch;
                            return false;
                        }

                        self.tag_stack.pop();
                        self.tag_level = self.tag_level.saturating_sub(1);

                        if let Some(handler) = &mut self.end_element_handler {
                            handler(&tag_name);
                        }

                        // After root element closes, check for junk
                        if self.tag_stack.is_empty() {
                            self.root_closed = true;
                        }

                        pos = new_pos;
                    }
                    // Start tag <...>
                    _ => {
                        let (tag_name, attrs, is_empty, new_pos) = self.scan_start_tag(&data, pos);
                        if new_pos == 0 {
                            return false;
                        }

                        if self.root_closed {
                            self.error_code = XmlError::JunkAfterDocElement;
                            return false;
                        }

                        self.seen_root = true;

                        if let Some(handler) = &mut self.start_element_handler {
                            let attr_refs: Vec<(&str, &str)> =
                                attrs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                            handler(&tag_name, &attr_refs);
                        }

                        if is_empty {
                            if let Some(handler) = &mut self.end_element_handler {
                                handler(&tag_name);
                            }
                            if self.tag_stack.is_empty() {
                                self.root_closed = true;
                            }
                        } else {
                            self.tag_stack.push(tag_name);
                            self.tag_level += 1;
                        }

                        pos = new_pos;
                    }
                }
            } else if self.root_closed {
                // After root element, transition to epilog processor and only whitespace is allowed
                if self.processor == Processor::Content {
                    self.processor = Processor::Epilog;
                }
                if (data[pos] as char).is_ascii_whitespace() {
                    self.advance_pos(data[pos]);
                    pos += 1;
                } else {
                    self.error_code = XmlError::JunkAfterDocElement;
                    return false;
                }
            } else if !self.seen_root {
                // Before root element, only whitespace is allowed (comments/PIs handled above)
                if (data[pos] as char).is_ascii_whitespace() {
                    self.advance_pos(data[pos]);
                    pos += 1;
                } else {
                    self.error_code = XmlError::Syntax;
                    return false;
                }
            } else {
                // Character data inside elements
                let result = self.scan_char_data(&data, pos);
                match result {
                    Ok((char_data, new_pos)) => {
                        if new_pos == pos {
                            // No progress — skip byte to avoid infinite loop
                            pos += 1;
                            continue;
                        }
                        if !char_data.is_empty() {
                            if let Some(handler) = &mut self.character_data_handler {
                                handler(&char_data);
                            }
                        }
                        pos = new_pos;
                    }
                    Err(_) => return false,
                }
            }
        }

        // Check for missing root element on final parse (only if we had data)
        if self.is_final && !self.seen_root && !data.is_empty() {
            self.error_code = XmlError::NoElements;
            return false;
        }

        true
    }

    /// Scan prolog content (XML declaration, DOCTYPE, comments, PIs)
    fn scan_prolog(&mut self, data: &[u8], mut pos: usize) -> Result<usize, ()> {
        let len = data.len();

        while pos < len {
            // Skip whitespace — but track byte offset for XML declaration check
            while pos < len && (data[pos] as char).is_ascii_whitespace() {
                self.advance_pos(data[pos]);
                self.byte_offset += 1;
                pos += 1;
            }
            if pos >= len {
                break;
            }

            if data[pos] != b'<' {
                // Non-whitespace, non-tag content in prolog — start of content
                break;
            }

            pos += 1;
            self.column_number += 1;
            if pos >= len {
                break;
            }

            match data[pos] {
                b'?' => {
                    // Processing instruction or XML declaration
                    self.column_number += 1;
                    pos += 1;
                    match self.scan_pi(&data, pos) {
                        Ok((target, pi_data, new_pos)) => {
                            if target.eq_ignore_ascii_case("xml") {
                                // XML declaration must be at the very start (byte_offset == 0)
                                // The '<' was at byte_offset, and we skipped whitespace before it
                                if self.seen_xml_decl || self.byte_offset > 0 {
                                    self.error_code = XmlError::MisplacedXmlPi;
                                    return Err(());
                                }
                                self.seen_xml_decl = true;
                                self.process_xml_decl(&pi_data)?;
                            } else {
                                if let Some(handler) = &mut self.processing_instruction_handler {
                                    handler(&target, &pi_data);
                                }
                            }
                            pos = new_pos;
                        }
                        Err(_) => return Err(()),
                    }
                }
                b'!' if pos + 2 < len && data[pos + 1] == b'-' && data[pos + 2] == b'-' => {
                    // Comment
                    self.column_number += 2;
                    pos += 2;
                    let comment_start = pos + 1;
                    self.column_number += 1;
                    pos += 1;
                    match self.find_comment_end(data, pos) {
                        Some(end_pos) => {
                            let comment_data = &data[comment_start..end_pos];
                            if let Some(handler) = &mut self.comment_handler {
                                handler(comment_data);
                            }
                            self.advance_pos_slice(&data[pos..end_pos]);
                            self.column_number += 3;
                            pos = end_pos + 3;
                        }
                        None => {
                            if self.is_final {
                                self.error_code = XmlError::UnclosedToken;
                                return Err(());
                            }
                            break;
                        }
                    }
                }
                b'!' => {
                    // DOCTYPE
                    if pos + 7 <= len && &data[pos..pos + 7] == b"DOCTYPE" {
                        self.advance_pos_slice(&data[pos..pos + 7]);
                        pos += 7;
                        pos = self.scan_doctype(data, pos)?;
                    } else {
                        self.error_code = XmlError::Syntax;
                        return Err(());
                    }
                }
                _ => {
                    // Start tag — back up and return to content scanning
                    pos -= 1; // back to '<'
                    self.column_number -= 1;
                    break;
                }
            }
        }
        Ok(pos)
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

    /// Scan a DOCTYPE declaration (simplified — skips internal subset)
    fn scan_doctype(&mut self, data: &[u8], mut pos: usize) -> Result<usize, ()> {
        let len = data.len();

        // Skip whitespace
        while pos < len && (data[pos] as char).is_ascii_whitespace() {
            self.advance_pos(data[pos]);
            pos += 1;
        }

        // Parse root element name
        let name_start = pos;
        while pos < len && is_name_char(data[pos]) {
            pos += 1;
        }
        if pos == name_start {
            self.error_code = XmlError::Syntax;
            return Err(());
        }
        let root_name = std::str::from_utf8(&data[name_start..pos]).unwrap_or("");
        self.advance_pos_slice(&data[name_start..pos]);

        let mut system_id: Option<String> = None;
        let mut public_id: Option<String> = None;
        let mut has_internal_subset = false;

        // Skip whitespace
        while pos < len && (data[pos] as char).is_ascii_whitespace() {
            self.advance_pos(data[pos]);
            pos += 1;
        }

        // Check for SYSTEM or PUBLIC
        if pos + 6 <= len && &data[pos..pos + 6] == b"SYSTEM" {
            self.advance_pos_slice(&data[pos..pos + 6]);
            pos += 6;
            while pos < len && (data[pos] as char).is_ascii_whitespace() {
                self.advance_pos(data[pos]);
                pos += 1;
            }
            let (sid, new_pos) = self.scan_quoted_string(data, pos)?;
            system_id = Some(sid);
            pos = new_pos;
        } else if pos + 6 <= len && &data[pos..pos + 6] == b"PUBLIC" {
            self.advance_pos_slice(&data[pos..pos + 6]);
            pos += 6;
            while pos < len && (data[pos] as char).is_ascii_whitespace() {
                self.advance_pos(data[pos]);
                pos += 1;
            }
            let (pid, new_pos) = self.scan_quoted_string(data, pos)?;
            public_id = Some(pid);
            pos = new_pos;
            while pos < len && (data[pos] as char).is_ascii_whitespace() {
                self.advance_pos(data[pos]);
                pos += 1;
            }
            let (sid, new_pos) = self.scan_quoted_string(data, pos)?;
            system_id = Some(sid);
            pos = new_pos;
        }

        // Skip whitespace
        while pos < len && (data[pos] as char).is_ascii_whitespace() {
            self.advance_pos(data[pos]);
            pos += 1;
        }

        // Call start doctype handler
        if let Some(handler) = &mut self.start_doctype_decl_handler {
            handler(
                root_name,
                system_id.as_deref(),
                public_id.as_deref(),
                pos < len && data[pos] == b'[',
            );
        }

        // Handle internal subset [...]
        if pos < len && data[pos] == b'[' {
            has_internal_subset = true;
            self.column_number += 1;
            pos += 1;
            pos = self.skip_internal_subset(data, pos)?;
        }

        // Skip whitespace
        while pos < len && (data[pos] as char).is_ascii_whitespace() {
            self.advance_pos(data[pos]);
            pos += 1;
        }

        if pos >= len || data[pos] != b'>' {
            self.error_code = XmlError::Syntax;
            return Err(());
        }
        self.column_number += 1;
        pos += 1;

        let _ = has_internal_subset;

        // Call end doctype handler
        if let Some(handler) = &mut self.end_doctype_decl_handler {
            handler();
        }

        Ok(pos)
    }

    /// Scan a quoted string (for DOCTYPE SYSTEM/PUBLIC identifiers)
    fn scan_quoted_string(&mut self, data: &[u8], mut pos: usize) -> Result<(String, usize), ()> {
        if pos >= data.len() || (data[pos] != b'"' && data[pos] != b'\'') {
            self.error_code = XmlError::Syntax;
            return Err(());
        }
        let quote = data[pos];
        self.column_number += 1;
        pos += 1;
        let start = pos;
        while pos < data.len() && data[pos] != quote {
            self.advance_pos(data[pos]);
            pos += 1;
        }
        if pos >= data.len() {
            self.error_code = XmlError::UnclosedToken;
            return Err(());
        }
        let value = std::str::from_utf8(&data[start..pos]).unwrap_or("").to_string();
        self.column_number += 1;
        pos += 1;
        Ok((value, pos))
    }

    /// Skip internal subset in DOCTYPE [...] — simplified, handles nested brackets
    fn skip_internal_subset(&mut self, data: &[u8], mut pos: usize) -> Result<usize, ()> {
        let len = data.len();
        let mut depth = 0; // for nested <!...> declarations
        // Safety: limit iterations to prevent infinite loops
        let max_iter = len * 2;
        let mut iter = 0;
        while pos < len {
            iter += 1;
            if iter > max_iter {
                self.error_code = XmlError::Syntax;
                return Err(());
            }
            match data[pos] {
                b']' if depth == 0 => {
                    self.column_number += 1;
                    pos += 1;
                    return Ok(pos);
                }
                b'<' if pos + 1 < len && data[pos + 1] == b'!' => {
                    depth += 1;
                    self.advance_pos(data[pos]);
                    pos += 1;
                }
                b'>' if depth > 0 => {
                    depth -= 1;
                    self.column_number += 1;
                    pos += 1;
                }
                b'\'' | b'"' => {
                    // Skip quoted strings inside declarations
                    let quote = data[pos];
                    self.column_number += 1;
                    pos += 1;
                    while pos < len && data[pos] != quote {
                        self.advance_pos(data[pos]);
                        pos += 1;
                    }
                    if pos < len {
                        self.column_number += 1;
                        pos += 1;
                    }
                }
                _ => {
                    self.advance_pos(data[pos]);
                    pos += 1;
                }
            }
        }
        self.error_code = XmlError::Syntax;
        Err(())
    }

    /// Find the end of a comment (position of first '-' in '-->'), returns None if not found
    fn find_comment_end(&self, data: &[u8], pos: usize) -> Option<usize> {
        let mut i = pos;
        while i + 2 < data.len() {
            if data[i] == b'-' && data[i + 1] == b'-' && data[i + 2] == b'>' {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// Find the end of a CDATA section (position of first ']' in ']]>'), returns None if not found
    fn find_cdata_end(&self, data: &[u8], pos: usize) -> Option<usize> {
        let mut i = pos;
        while i + 2 < data.len() {
            if data[i] == b']' && data[i + 1] == b']' && data[i + 2] == b'>' {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// Scan a processing instruction: target data ?>
    /// pos should be right after '<?'
    fn scan_pi(&mut self, data: &[u8], mut pos: usize) -> Result<(String, String, usize), ()> {
        let len = data.len();

        // Parse target name
        let target_start = pos;
        while pos < len && is_name_char(data[pos]) {
            pos += 1;
        }
        if pos == target_start {
            self.error_code = XmlError::Syntax;
            return Err(());
        }
        let target = std::str::from_utf8(&data[target_start..pos])
            .unwrap_or("")
            .to_string();
        self.advance_pos_slice(&data[target_start..pos]);

        // For XML declaration, check for immediate ?>
        if pos + 1 < len && data[pos] == b'?' && data[pos + 1] == b'>' {
            self.column_number += 2;
            return Ok((target, String::new(), pos + 2));
        }

        // Skip whitespace before data
        if pos < len && (data[pos] as char).is_ascii_whitespace() {
            self.advance_pos(data[pos]);
            pos += 1;
        }

        // Find ?>
        let data_start = pos;
        while pos + 1 < len {
            if data[pos] == b'?' && data[pos + 1] == b'>' {
                let pi_data = std::str::from_utf8(&data[data_start..pos])
                    .unwrap_or("")
                    .to_string();
                self.advance_pos_slice(&data[data_start..pos]);
                self.column_number += 2;
                return Ok((target, pi_data, pos + 2));
            }
            pos += 1;
        }

        if self.is_final {
            self.error_code = XmlError::UnclosedToken;
        }
        Err(())
    }

    /// Scan a start tag <tagname attrs...>
    fn scan_start_tag(
        &mut self,
        data: &[u8],
        mut pos: usize,
    ) -> (String, Vec<(String, String)>, bool, usize) {
        let len = data.len();

        // Parse tag name
        let tag_start = pos;
        while pos < len && is_name_char(data[pos]) {
            pos += 1;
        }

        if pos == tag_start {
            self.error_code = XmlError::Syntax;
            return (String::new(), vec![], false, 0);
        }

        let tag_name = match std::str::from_utf8(&data[tag_start..pos]) {
            Ok(s) => s.to_string(),
            Err(_) => {
                self.error_code = XmlError::InvalidToken;
                return (String::new(), vec![], false, 0);
            }
        };

        self.advance_pos_slice(&data[tag_start..pos]);

        // Check for invalid bytes immediately following the name (e.g., 4-byte UTF-8)
        if pos < len && data[pos] >= 0xF0 && data[pos] <= 0xF7 {
            self.error_code = XmlError::InvalidToken;
            return (String::new(), vec![], false, 0);
        }

        let mut attrs: Vec<(String, String)> = Vec::new();
        let mut attr_names: Vec<String> = Vec::new();
        let mut is_empty = false;

        loop {
            // Skip whitespace
            while pos < len && (data[pos] as char).is_ascii_whitespace() {
                self.advance_pos(data[pos]);
                pos += 1;
            }

            if pos >= len {
                if self.is_final {
                    self.error_code = XmlError::UnclosedToken;
                }
                return (String::new(), vec![], false, 0);
            }

            // Check for end of tag
            if data[pos] == b'>' {
                self.column_number += 1;
                return (tag_name, attrs, is_empty, pos + 1);
            }

            if data[pos] == b'/' {
                self.column_number += 1;
                pos += 1;
                if pos < len && data[pos] == b'>' {
                    self.column_number += 1;
                    is_empty = true;
                    return (tag_name, attrs, is_empty, pos + 1);
                } else {
                    self.error_code = XmlError::Syntax;
                    return (String::new(), vec![], false, 0);
                }
            }

            // Parse attribute name
            let attr_name_start = pos;
            while pos < len
                && data[pos] != b'='
                && data[pos] != b'>'
                && data[pos] != b'/'
                && !(data[pos] as char).is_ascii_whitespace()
            {
                pos += 1;
            }
            if pos == attr_name_start {
                self.error_code = XmlError::Syntax;
                return (String::new(), vec![], false, 0);
            }
            let attr_name = std::str::from_utf8(&data[attr_name_start..pos])
                .unwrap_or("")
                .to_string();
            self.advance_pos_slice(&data[attr_name_start..pos]);

            // Check for duplicate attribute
            if attr_names.contains(&attr_name) {
                self.error_code = XmlError::DuplicateAttribute;
                return (String::new(), vec![], false, 0);
            }
            attr_names.push(attr_name.clone());

            // Skip whitespace
            while pos < len && (data[pos] as char).is_ascii_whitespace() {
                self.advance_pos(data[pos]);
                pos += 1;
            }

            // Expect '='
            if pos >= len || data[pos] != b'=' {
                self.error_code = XmlError::Syntax;
                return (String::new(), vec![], false, 0);
            }
            self.column_number += 1;
            pos += 1;

            // Skip whitespace
            while pos < len && (data[pos] as char).is_ascii_whitespace() {
                self.advance_pos(data[pos]);
                pos += 1;
            }

            // Parse attribute value (quoted)
            if pos >= len || (data[pos] != b'"' && data[pos] != b'\'') {
                self.error_code = XmlError::Syntax;
                return (String::new(), vec![], false, 0);
            }
            let quote = data[pos];
            self.column_number += 1;
            pos += 1;

            let mut attr_value = Vec::new();
            while pos < len && data[pos] != quote {
                if data[pos] == b'&' {
                    // Entity/character reference in attribute value
                    match self.resolve_reference(&data, pos) {
                        Ok((replacement, new_pos)) => {
                            attr_value.extend_from_slice(&replacement);
                            self.advance_pos_slice(&data[pos..new_pos]);
                            pos = new_pos;
                        }
                        Err(_) => {
                            return (String::new(), vec![], false, 0);
                        }
                    }
                } else {
                    if data[pos] == b'\n' || data[pos] == b'\r' || data[pos] == b'\t' {
                        attr_value.push(b' '); // normalize whitespace in attrs
                    } else {
                        attr_value.push(data[pos]);
                    }
                    self.advance_pos(data[pos]);
                    pos += 1;
                }
            }
            if pos >= len {
                self.error_code = XmlError::UnclosedToken;
                return (String::new(), vec![], false, 0);
            }
            self.column_number += 1; // closing quote
            pos += 1;

            let attr_value_str = String::from_utf8(attr_value).unwrap_or_default();
            attrs.push((attr_name, attr_value_str));
        }
    }

    /// Scan an end tag </tagname>
    fn scan_end_tag(&mut self, data: &[u8], mut pos: usize) -> (String, usize) {
        let len = data.len();

        // Parse tag name
        let tag_start = pos;
        while pos < len && is_name_char(data[pos]) {
            pos += 1;
        }

        if pos == tag_start {
            self.error_code = XmlError::Syntax;
            return (String::new(), 0);
        }

        let tag_name = match std::str::from_utf8(&data[tag_start..pos]) {
            Ok(s) => s.to_string(),
            Err(_) => {
                self.error_code = XmlError::InvalidToken;
                return (String::new(), 0);
            }
        };

        self.advance_pos_slice(&data[tag_start..pos]);

        // Skip whitespace
        while pos < len && (data[pos] as char).is_ascii_whitespace() {
            self.advance_pos(data[pos]);
            pos += 1;
        }

        if pos >= len || data[pos] != b'>' {
            self.error_code = XmlError::Syntax;
            return (String::new(), 0);
        }

        self.column_number += 1;
        (tag_name, pos + 1)
    }

    /// Scan character data, handling entity/character references.
    /// Returns the expanded text and the new position.
    fn scan_char_data(&mut self, data: &[u8], mut pos: usize) -> Result<(Vec<u8>, usize), ()> {
        let len = data.len();
        let mut result = Vec::new();

        while pos < len && data[pos] != b'<' {
            if data[pos] == 0 {
                self.error_code = XmlError::InvalidToken;
                return Err(());
            }

            if data[pos] == b'&' {
                match self.resolve_reference(data, pos) {
                    Ok((replacement, new_pos)) => {
                        result.extend_from_slice(&replacement);
                        self.advance_pos_slice(&data[pos..new_pos]);
                        pos = new_pos;
                    }
                    Err(_) => return Err(()),
                }
            } else {
                self.advance_pos(data[pos]);
                result.push(data[pos]);
                pos += 1;
            }
        }

        Ok((result, pos))
    }

    /// Resolve an entity reference (&...; or &#...; or &#x...;)
    /// Returns the replacement bytes and the position after the ';'
    fn resolve_reference(&mut self, data: &[u8], pos: usize) -> Result<(Vec<u8>, usize), ()> {
        let len = data.len();
        debug_assert!(data[pos] == b'&');
        let start = pos;
        let mut p = pos + 1;

        if p >= len {
            self.error_code = XmlError::UnclosedToken;
            return Err(());
        }

        if data[p] == b'#' {
            // Character reference
            p += 1;
            if p >= len {
                self.error_code = XmlError::UnclosedToken;
                return Err(());
            }

            let hex = data[p] == b'x' || data[p] == b'X';
            if hex {
                p += 1;
            }

            let num_start = p;
            while p < len && data[p] != b';' {
                p += 1;
            }
            if p >= len {
                self.error_code = XmlError::UnclosedToken;
                return Err(());
            }

            let num_str = std::str::from_utf8(&data[num_start..p]).unwrap_or("");
            let codepoint = if hex {
                u32::from_str_radix(num_str, 16).unwrap_or(0xFFFFFFFF)
            } else {
                num_str.parse::<u32>().unwrap_or(0xFFFFFFFF)
            };

            // Validate codepoint — XML 1.0 allows specific ranges
            if !is_valid_xml_char(codepoint) {
                self.error_code = XmlError::BadCharRef;
                return Err(());
            }

            let ch = match char::from_u32(codepoint) {
                Some(c) => c,
                None => {
                    self.error_code = XmlError::BadCharRef;
                    return Err(());
                }
            };

            let mut buf = [0u8; 4];
            let encoded = ch.encode_utf8(&mut buf);
            let _ = start;
            Ok((encoded.as_bytes().to_vec(), p + 1))
        } else {
            // Named entity reference
            let name_start = p;
            while p < len && data[p] != b';' {
                if !is_name_char(data[p]) {
                    self.error_code = XmlError::Syntax;
                    return Err(());
                }
                p += 1;
            }
            if p >= len {
                self.error_code = XmlError::UnclosedToken;
                return Err(());
            }

            let name = std::str::from_utf8(&data[name_start..p]).unwrap_or("");
            p += 1; // skip ';'

            match name {
                "amp" => Ok((b"&".to_vec(), p)),
                "lt" => Ok((b"<".to_vec(), p)),
                "gt" => Ok((b">".to_vec(), p)),
                "quot" => Ok((b"\"".to_vec(), p)),
                "apos" => Ok((b"'".to_vec(), p)),
                _ => {
                    // General entity — call skipped entity handler or report error
                    if let Some(handler) = &mut self.skipped_entity_handler {
                        handler(name, false);
                        Ok((vec![], p))
                    } else {
                        self.error_code = XmlError::UndefinedEntity;
                        Err(())
                    }
                }
            }
        }
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
        todo!("not yet implemented")
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

/// Check if a byte is a valid XML name character (or start of a valid multi-byte name char).
/// This handles ASCII name chars directly. For multi-byte UTF-8:
/// - 2-byte (0xC0..0xDF) and 3-byte (0xE0..0xEF) sequences are accepted (covers most valid XML name chars)
/// - 4-byte (0xF0..0xF7) sequences are rejected (U+10000+ not valid in XML 1.0 names)
/// - Continuation bytes (0x80..0xBF) are accepted (part of multi-byte sequences)
fn is_name_char(b: u8) -> bool {
    matches!(
        b,
        b'a'..=b'z'
            | b'A'..=b'Z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b':'
            | b'.'
            | 0x80..=0xBF // UTF-8 continuation bytes
            | 0xC0..=0xDF // 2-byte UTF-8 start
            | 0xE0..=0xEF // 3-byte UTF-8 start
    )
}

/// Check if a codepoint is a valid XML 1.0 character
fn is_valid_xml_char(cp: u32) -> bool {
    matches!(cp, 0x9 | 0xA | 0xD | 0x20..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x10FFFF)
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
