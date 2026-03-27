"""Name normalization for C-to-Rust mapping.

All names are normalized to a common form so that the skeleton comparison
is language-agnostic. Maps are reused from the existing ast-compare.py.
"""

import re

# Error code mapping: C XML_ERROR_X suffix -> Rust XmlError::X variant
ERROR_MAP = {
    "NONE": "None", "NO_MEMORY": "NoMemory", "SYNTAX": "Syntax",
    "NO_ELEMENTS": "NoElements", "INVALID_TOKEN": "InvalidToken",
    "UNCLOSED_TOKEN": "UnclosedToken", "PARTIAL_CHAR": "PartialChar",
    "TAG_MISMATCH": "TagMismatch", "DUPLICATE_ATTRIBUTE": "DuplicateAttribute",
    "JUNK_AFTER_DOC_ELEMENT": "JunkAfterDocElement",
    "UNDEFINED_ENTITY": "UndefinedEntity", "NOT_STANDALONE": "NotStandalone",
    "EXTERNAL_ENTITY_HANDLING": "ExternalEntityHandling",
    "MISPLACED_XML_PI": "MisplacedXmlPi", "BAD_CHAR_REF": "BadCharRef",
    "ABORTED": "Aborted", "ASYNC_ENTITY": "AsyncEntity",
    "RECURSIVE_ENTITY_REF": "RecursiveEntityRef",
    "BINARY_ENTITY_REF": "BinaryEntityRef",
    "ATTRIBUTE_EXTERNAL_ENTITY_REF": "AttributeExternalEntityRef",
    "UNEXPECTED_STATE": "UnexpectedState",
    "AMPLIFICATION_LIMIT_BREACH": "AmplificationLimitBreach",
    "UNCLOSED_CDATA_SECTION": "UnclosedCdataSection",
    "XML_DECL": "XmlDecl", "TEXT_DECL": "TextDecl",
    "INCORRECT_ENCODING": "IncorrectEncoding",
    "UNKNOWN_ENCODING": "UnknownEncoding",
    "FINISHED": "Finished", "SUSPENDED": "Suspended",
    "ENTITY_DECLARED_IN_PE": "EntityDeclaredInPe",
    "PARAM_ENTITY_REF": "ParamEntityRef",
    "SUSPEND_PE": "SuspendPe",
}
# Build reverse map for Rust->normalized
_ERROR_REVERSE = {v: v for v in ERROR_MAP.values()}

HANDLER_MAP = {
    "startElementHandler": "start_element_handler",
    "endElementHandler": "end_element_handler",
    "characterDataHandler": "character_data_handler",
    "processingInstructionHandler": "processing_instruction_handler",
    "commentHandler": "comment_handler",
    "startCdataSectionHandler": "start_cdata_section_handler",
    "endCdataSectionHandler": "end_cdata_section_handler",
    "defaultHandler": "default_handler",
    "startNamespaceDeclHandler": "start_namespace_decl_handler",
    "endNamespaceDeclHandler": "end_namespace_decl_handler",
    "externalEntityRefHandler": "external_entity_ref_handler",
    "skippedEntityHandler": "skipped_entity_handler",
    "xmlDeclHandler": "xml_decl_handler",
    "startDoctypeDeclHandler": "start_doctype_decl_handler",
    "endDoctypeDeclHandler": "end_doctype_decl_handler",
    "notStandaloneHandler": "not_standalone_handler",
    "unknownEncodingHandler": "unknown_encoding_handler",
    "entityDeclHandler": "entity_decl_handler",
    "notationDeclHandler": "notation_decl_handler",
    "attlistDeclHandler": "attlist_decl_handler",
    "elementDeclHandler": "element_decl_handler",
    "unparsedEntityDeclHandler": "unparsed_entity_decl_handler",
}

# Function call name exceptions (C name -> Rust name)
CALL_MAP = {
    "storeAtts": "process_namespaces",
    "XmlNameLength": "name_length",
    "XmlGetAttributes": "get_atts",
    "XmlContentTok": "content_tok",
    "XmlPrologTok": "prolog_tok",
    "XmlCdataSectionTok": "cdata_section_tok",
    "XmlCharRefNumber": "char_ref_number",
    "XmlIgnoreSectionTok": "ignore_section_tok",
}

TOKEN_MAP = {
    "XML_TOK_NONE": "XmlTok::None", "XML_TOK_INVALID": "XmlTok::Invalid",
    "XML_TOK_PARTIAL": "XmlTok::Partial", "XML_TOK_PARTIAL_CHAR": "XmlTok::PartialChar",
    "XML_TOK_TRAILING_CR": "XmlTok::TrailingCr", "XML_TOK_ENTITY_REF": "XmlTok::EntityRef",
    "XML_TOK_START_TAG_NO_ATTS": "XmlTok::StartTagNoAtts",
    "XML_TOK_START_TAG_WITH_ATTS": "XmlTok::StartTagWithAtts",
    "XML_TOK_EMPTY_ELEMENT_NO_ATTS": "XmlTok::EmptyElementNoAtts",
    "XML_TOK_EMPTY_ELEMENT_WITH_ATTS": "XmlTok::EmptyElementWithAtts",
    "XML_TOK_END_TAG": "XmlTok::EndTag", "XML_TOK_CHAR_REF": "XmlTok::CharRef",
    "XML_TOK_XML_DECL": "XmlTok::XmlDecl", "XML_TOK_DATA_NEWLINE": "XmlTok::DataNewline",
    "XML_TOK_CDATA_SECT_OPEN": "XmlTok::CdataSectOpen",
    "XML_TOK_TRAILING_RSQB": "XmlTok::TrailingRsqb",
    "XML_TOK_DATA_CHARS": "XmlTok::DataChars", "XML_TOK_PI": "XmlTok::Pi",
    "XML_TOK_COMMENT": "XmlTok::Comment",
    "XML_TOK_PROLOG_S": "XmlTok::PrologS",
    "XML_TOK_BOM": "XmlTok::Bom",
}

ROLE_MAP = {
    "XML_ROLE_XML_DECL": "Role::XmlDecl",
    "XML_ROLE_INSTANCE_START": "Role::InstanceStart",
    "XML_ROLE_DOCTYPE_NAME": "Role::DoctypeName",
    "XML_ROLE_DOCTYPE_SYSTEM_ID": "Role::DoctypeSystemId",
    "XML_ROLE_DOCTYPE_PUBLIC_ID": "Role::DoctypePublicId",
    "XML_ROLE_DOCTYPE_INTERNAL_SUBSET": "Role::DoctypeInternalSubset",
    "XML_ROLE_DOCTYPE_CLOSE": "Role::DoctypeClose",
    "XML_ROLE_GENERAL_ENTITY_NAME": "Role::GeneralEntityName",
    "XML_ROLE_PARAM_ENTITY_NAME": "Role::ParamEntityName",
    "XML_ROLE_ENTITY_VALUE": "Role::EntityValue",
    "XML_ROLE_ENTITY_COMPLETE": "Role::EntityComplete",
    "XML_ROLE_PI": "Role::Pi",
    "XML_ROLE_COMMENT": "Role::Comment",
    "XML_ROLE_ERROR": "Role::Error",
    "XML_ROLE_NONE": "Role::None",
    "XML_ROLE_NOTATION_NAME": "Role::NotationName",
    "XML_ROLE_ATTLIST_ELEMENT_NAME": "Role::AttlistElementName",
    "XML_ROLE_ATTRIBUTE_NAME": "Role::AttributeName",
    "XML_ROLE_ELEMENT_NAME": "Role::ElementName",
}

C_KEYWORDS = {
    'if', 'while', 'for', 'switch', 'return', 'sizeof', 'assert', 'case',
    'break', 'continue', 'goto', 'do', 'else', 'default', 'typedef',
    'struct', 'enum', 'union', 'const', 'static', 'extern', 'inline',
    'void', 'int', 'char', 'long', 'short', 'unsigned', 'signed',
    'float', 'double', 'NULL',
}

# Only true Rust language keywords and constructors that appear as
# syntactic elements, NOT method calls. Method calls like .get(),
# .insert(), .push() are semantically meaningful and must NOT be filtered.
RUST_SYNTAX_ONLY = {
    'match', 'if', 'let', 'mut', 'fn', 'return', 'loop', 'while',
    'for', 'break', 'continue', 'self', 'Self', 'true', 'false',
    'Some', 'None', 'Ok', 'Err',  # enum constructors, not calls
}

# Calls that are suppressed globally because they have no Rust equivalent
# (memory management, pool ops, C macros, etc.)
# Loaded from deliberate-divergences.json at runtime
_suppressed_calls: set[str] | None = None


def camel_to_snake(name: str) -> str:
    """Convert camelCase/PascalCase to snake_case."""
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    return s.lower()


def normalize_c_call(name: str) -> str:
    """Normalize a C function call name to its Rust equivalent."""
    if name in CALL_MAP:
        return CALL_MAP[name]
    return camel_to_snake(name)


def normalize_c_error(text: str) -> str:
    """Normalize C XML_ERROR_X to XmlError::X."""
    m = re.match(r'XML_ERROR_(\w+)', text)
    if m:
        suffix = m.group(1)
        if suffix in ERROR_MAP:
            return f"XmlError::{ERROR_MAP[suffix]}"
    return text


def normalize_rust_error(text: str) -> str:
    """Normalize Rust XmlError::X (already in normal form)."""
    if text.startswith("XmlError::"):
        return text
    return text


def normalize_c_token(text: str) -> str:
    """Normalize C XML_TOK_X to XmlTok::X."""
    if text in TOKEN_MAP:
        return TOKEN_MAP[text]
    # Try to auto-convert
    m = re.match(r'XML_TOK_(\w+)', text)
    if m:
        parts = m.group(1).split('_')
        camel = ''.join(p.capitalize() for p in parts)
        return f"XmlTok::{camel}"
    return text


def normalize_rust_token(text: str) -> str:
    """Normalize Rust token pattern to standard form."""
    # Strip module prefix: xmltok::XmlTok::X -> XmlTok::X
    text = re.sub(r'^(?:self::)?(?:xmltok(?:_impl)?::)?', '', text)
    # Strip leading self:: or crate:: etc.
    text = re.sub(r'^(?:crate::)?', '', text)
    return text


def normalize_c_role(text: str) -> str:
    """Normalize C XML_ROLE_X to Role::X."""
    if text in ROLE_MAP:
        return ROLE_MAP[text]
    m = re.match(r'XML_ROLE_(\w+)', text)
    if m:
        parts = m.group(1).split('_')
        camel = ''.join(p.capitalize() for p in parts)
        return f"Role::{camel}"
    return text


def normalize_rust_role(text: str) -> str:
    """Normalize Rust role pattern to standard form."""
    text = re.sub(r'^(?:self::)?(?:xmlrole::)?', '', text)
    return text


def normalize_c_handler(text: str) -> str:
    """Normalize C handler name (e.g., m_commentHandler -> comment_handler)."""
    # Strip m_ prefix
    name = re.sub(r'^m_', '', text)
    if name in HANDLER_MAP:
        return HANDLER_MAP[name]
    return camel_to_snake(name)


def normalize_c_field_access(text: str) -> str:
    """Normalize C parser->m_field to field name."""
    m = re.match(r'parser->m_(\w+)', text)
    if m:
        return camel_to_snake(m.group(1))
    return text


def is_c_keyword(name: str) -> bool:
    return name in C_KEYWORDS


def is_rust_noise(name: str) -> bool:
    """Is this a Rust language keyword / constructor that's not a real call?

    Only filters true syntax elements. Method calls like .get(), .push()
    are semantically meaningful and are NOT filtered.
    """
    return name in RUST_SYNTAX_ONLY
