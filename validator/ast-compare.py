#!/usr/bin/env python3
"""AST-based structural comparison of C and Rust function implementations.

Uses tree-sitter to parse both C and Rust to ASTs, then compares:
- Switch/match case coverage
- Error codes per case
- Handler calls per case
- Control flow structure (early returns, loops)
- Function calls

Every suppression is loaded from deliberate-divergences.json and must have a
justification. The tool refuses to suppress calls listed in NOT_SUPPRESSED.

Usage:
    python3 ast-compare.py <c_func> <rust_func>
    python3 ast-compare.py doContent do_content
    python3 ast-compare.py --all              # Compare all known pairs
    python3 ast-compare.py --ci               # CI mode: --all + exit 1 on divergences
    python3 ast-compare.py --audit            # Show all suppressions with justifications
    python3 ast-compare.py --list-cases <func> c|rust
    python3 ast-compare.py --prompt <c_func> <rust_func> [extra...]
    python3 ast-compare.py --prompt-all
    python3 ast-compare.py --missing-functions
"""

import sys
import os
import json
import re
import tree_sitter
import tree_sitter_c
import tree_sitter_rust

# ROOT is one level up from validator/
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
# Support both submodule layout (expat/expat/lib/) and flat layout (expat/lib/)
C_FILE = os.path.join(ROOT, "expat", "expat", "lib", "xmlparse.c")
if not os.path.exists(C_FILE):
    C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")
# Divergences file lives alongside this script in validator/
DIVERGENCES_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "deliberate-divergences.json")

# Initialize parsers
C_LANG = tree_sitter.Language(tree_sitter_c.language())
RUST_LANG = tree_sitter.Language(tree_sitter_rust.language())

# ========= Divergence config loading =========

_divergences_config = None

def load_divergences():
    global _divergences_config
    if _divergences_config is not None:
        return _divergences_config
    try:
        with open(DIVERGENCES_FILE) as f:
            _divergences_config = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"FATAL: Cannot load divergences file {DIVERGENCES_FILE}: {e}", file=sys.stderr)
        sys.exit(2)
    # Validate structure
    if "global_suppressions" not in _divergences_config:
        print(f"FATAL: divergences file missing 'global_suppressions'", file=sys.stderr)
        sys.exit(2)
    return _divergences_config


def build_suppressed_calls():
    """Build the set of suppressed C function calls from the divergences JSON.

    CRITICAL: This ONLY includes calls from 'calls' sections with status 'accepted'.
    Calls listed in NOT_SUPPRESSED are NEVER suppressed, even if accidentally
    added to a suppression category.
    """
    config = load_divergences()
    suppressed = set()
    calls_config = config.get("global_suppressions", {}).get("calls", {})

    for category, info in calls_config.items():
        if not isinstance(info, dict):
            continue
        if info.get("status") != "accepted":
            print(f"WARNING: Call category '{category}' has status '{info.get('status')}', not 'accepted' — skipping", file=sys.stderr)
            continue
        if "justification" not in info:
            print(f"FATAL: Call category '{category}' has no justification — refusing to suppress", file=sys.stderr)
            sys.exit(2)
        for call in info.get("calls", []):
            suppressed.add(call)

    # SAFETY CHECK: Never suppress calls listed in NOT_SUPPRESSED with status "must_port"
    not_suppressed_config = config.get("global_suppressions", {}).get("NOT_SUPPRESSED", {})
    never_suppress = set()
    for category, info in not_suppressed_config.items():
        if category.startswith("_"):
            continue
        if not isinstance(info, dict):
            continue
        if info.get("status") != "must_port":
            continue  # Only block must_port items; ported items can be per-function suppressed
        for call in info.get("calls", []):
            never_suppress.add(call)

    leaked = suppressed & never_suppress
    if leaked:
        print(f"FATAL: These calls appear in BOTH suppressed and NOT_SUPPRESSED: {sorted(leaked)}", file=sys.stderr)
        print(f"This is a configuration error. NOT_SUPPRESSED always wins.", file=sys.stderr)
        sys.exit(2)

    return suppressed


def build_never_suppress_errors():
    """Build set of error codes that must NOT be suppressed (only must_port items)."""
    config = load_divergences()
    never = set()
    not_suppressed = config.get("global_suppressions", {}).get("NOT_SUPPRESSED", {})
    for category, info in not_suppressed.items():
        if category.startswith("_"):
            continue
        if not isinstance(info, dict):
            continue
        if info.get("status") != "must_port":
            continue  # Only block must_port items
        if "error" in info:
            never.add(info["error"])
    return never


def get_suppressed_errors(rust_func_name):
    """Get error codes that should be suppressed for this function."""
    config = load_divergences()
    suppressed = set()
    for code, info in config.get("global_suppressions", {}).get("errors", {}).items():
        if isinstance(info, dict):
            if info.get("status") != "accepted":
                continue
            if "justification" not in info:
                print(f"FATAL: Global error suppression '{code}' has no justification", file=sys.stderr)
                sys.exit(2)
        suppressed.add(code)

    per_func = config.get("per_function_suppressions", {}).get(rust_func_name, {})
    suppressed |= set(per_func.get("suppressed_errors", []))

    # SAFETY: remove any errors that are in NOT_SUPPRESSED
    never = build_never_suppress_errors()
    leaked = suppressed & never
    if leaked:
        print(f"FATAL: Error codes in BOTH suppressed and NOT_SUPPRESSED: {sorted(leaked)}", file=sys.stderr)
        sys.exit(2)

    return suppressed


def get_suppressed_handlers(rust_func_name):
    """Get handler names that should be suppressed for this function."""
    config = load_divergences()
    suppressed = set(config.get("global_suppressions", {}).get("handlers", {}).keys())
    per_func = config.get("per_function_suppressions", {}).get(rust_func_name, {})
    suppressed |= set(per_func.get("suppressed_handlers", []))
    return suppressed


def get_per_function_suppressed_calls(rust_func_name):
    """Get function calls that should be suppressed for this specific function."""
    config = load_divergences()
    per_func = config.get("per_function_suppressions", {}).get(rust_func_name, {})
    # Convert C call names to their Rust equivalents for matching
    result = set()
    for call in per_func.get("suppressed_calls", []):
        result.add(call)
        result.add(c_to_rust_call_name(call))
    return result


# Build SUPPRESSED_CALLS from config (not hardcoded)
SUPPRESSED_CALLS = None  # Lazy-initialized

def get_suppressed_calls():
    global SUPPRESSED_CALLS
    if SUPPRESSED_CALLS is None:
        SUPPRESSED_CALLS = build_suppressed_calls()
    return SUPPRESSED_CALLS


# ========= AST helpers =========

def parse_c(src_bytes):
    parser = tree_sitter.Parser(C_LANG)
    return parser.parse(src_bytes)


def parse_rust(src_bytes):
    parser = tree_sitter.Parser(RUST_LANG)
    return parser.parse(src_bytes)


def find_function_node(tree, name, lang):
    """Find a function definition node by name in the AST."""
    root = tree.root_node

    def walk(node):
        if lang == "c" and node.type == "function_definition":
            decl = node.child_by_field_name("declarator")
            if decl:
                for child in walk_all(decl):
                    if child.type == "identifier" and child.text.decode() == name:
                        return node
        elif lang == "rust" and node.type == "function_item":
            name_node = node.child_by_field_name("name")
            if name_node and name_node.text.decode() == name:
                return node

        for child in node.children:
            result = walk(child)
            if result:
                return result
        return None

    def walk_all(node):
        yield node
        for child in node.children:
            yield from walk_all(child)

    return walk(root)


def walk_all(node):
    """Walk all nodes in the AST."""
    yield node
    for child in node.children:
        yield from walk_all(child)


def extract_switch_cases_ast(func_node, lang):
    """Extract switch/match cases from AST with their content summary."""
    cases = []

    for node in walk_all(func_node):
        if lang == "c" and node.type == "case_statement":
            value_node = node.child_by_field_name("value") or (node.children[1] if len(node.children) > 1 else None)
            if value_node:
                value = value_node.text.decode()
                body_text = node.text.decode()
                errors = set(re.findall(r'XML_ERROR_(\w+)', body_text))
                handlers = set(re.findall(r'parser->m_(\w+Handler)', body_text))
                raw_calls = set(re.findall(r'\b(\w+)\s*\(', body_text))
                # Remove C keywords (handled by is_plausible_c_function_name)
                # and handler field accesses (m_*Handler are struct fields, not calls)
                func_calls = {c for c in raw_calls
                              if c not in C_KEYWORDS
                              and not re.match(r'm_\w+Handler$', c)}
                has_return = 'return' in body_text
                cases.append({
                    "label": value,
                    "errors": sorted(errors),
                    "handlers": sorted(handlers),
                    "calls": sorted(func_calls),
                    "returns": has_return,
                    "lines": body_text.count('\n') + 1,
                })

        elif lang == "rust" and node.type == "match_arm":
            pattern_node = node.child_by_field_name("pattern")
            if pattern_node:
                pattern = pattern_node.text.decode()
                body_text = node.text.decode()
                errors = set(re.findall(r'XmlError::(\w+)', body_text))
                handlers = set(re.findall(r'self\.(\w+_handler)', body_text))
                if 'report_default' in body_text:
                    handlers.add('default_handler')
                if 'report_comment' in body_text:
                    handlers.add('comment_handler')
                    handlers.add('default_handler')
                if 'report_processing_instruction' in body_text:
                    handlers.add('processing_instruction_handler')
                    handlers.add('default_handler')
                func_calls = set(re.findall(r'(?:self\.|Self::)?(\w+)\s*\(', body_text))
                func_calls -= {'match', 'if', 'Some', 'None', 'Ok', 'Err', 'unwrap',
                               'unwrap_or', 'as_bytes', 'to_string', 'len', 'push',
                               'pop', 'is_empty', 'from_utf8', 'collect', 'iter',
                               'map', 'from_u32', 'encode_utf8', 'get', 'contains',
                               'saturating_sub', 'to_vec', 'clone', 'extend_from_slice',
                               'matches', 'as_str', 'as_deref'}
                has_return = 'return' in body_text
                cases.append({
                    "label": pattern,
                    "errors": sorted(errors),
                    "handlers": sorted(handlers),
                    "calls": sorted(func_calls),
                    "returns": has_return,
                    "lines": body_text.count('\n') + 1,
                })

    return cases


def extract_all_errors(func_node, lang):
    """Extract all error codes from the function."""
    text = func_node.text.decode()
    if lang == "c":
        return set(re.findall(r'XML_ERROR_(\w+)', text))
    else:
        return set(re.findall(r'XmlError::(\w+)', text))


def extract_all_handlers(func_node, lang):
    """Extract all handler references from the function."""
    text = func_node.text.decode()
    if lang == "c":
        return set(re.findall(r'parser->m_(\w+Handler)', text))
    else:
        handlers = set(re.findall(r'self\.(\w+_handler)', text))
        if 'report_default' in text:
            handlers.add('default_handler')
        if 'report_comment' in text:
            handlers.add('comment_handler')
            handlers.add('default_handler')
        if 'report_processing_instruction' in text:
            handlers.add('processing_instruction_handler')
            handlers.add('default_handler')
        return handlers


# ========= Mapping tables =========

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

CALL_MAP = {
    # Only exceptions to the standard camelCase->snake_case rule
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


# C language keywords and control-flow that regex can extract as "calls"
C_KEYWORDS = {
    'if', 'while', 'for', 'switch', 'return', 'sizeof', 'assert', 'case',
    'break', 'continue', 'goto', 'do', 'else', 'default', 'typedef',
    'struct', 'enum', 'union', 'const', 'static', 'extern', 'inline',
    'void', 'int', 'char', 'long', 'short', 'unsigned', 'signed',
    'float', 'double', 'NULL',
}


def is_plausible_c_function_name(name):
    """Determine if a regex-extracted name is plausibly a C function call.

    Returns False for:
    - C keywords and control flow
    - Numeric literals (regex artifacts)
    - Names shorter than 3 chars that aren't known functions
    - Names that are pure hex digits (e.g., '2f', '36')

    Returns True for everything else. ALL_CAPS names are NOT filtered —
    if they are C macros that should be suppressed, they must be listed
    in deliberate-divergences.json under a suppression category.
    """
    if name in C_KEYWORDS:
        return False
    # Numeric or hex-like strings (regex noise from integer literals)
    if re.match(r'^[0-9a-f]+$', name):
        return False
    # Very short names that aren't known functions (single char loop vars, etc.)
    # But allow e.g. "lx" if it's in the suppression list — the JSON decides
    if len(name) <= 2 and name.islower():
        return False
    return True


def c_to_rust_call_name(c_name):
    """Convert a C function name to its expected Rust equivalent.
    Uses CALL_MAP for exceptions, otherwise converts camelCase to snake_case."""
    if c_name in CALL_MAP:
        return CALL_MAP[c_name]
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', c_name)
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    return s.lower()


# ========= Main comparison =========

def compare(c_func_name, r_func_name, extra_rust_funcs=None, c_file=None, r_file=None):
    """Main comparison function."""
    suppressed_calls = get_suppressed_calls()

    c_path = c_file or C_FILE
    r_path = r_file or RUST_FILE

    if not os.path.exists(c_path):
        print(f"Error: C source file not found: {c_path}", file=sys.stderr)
        print(f"  Hint: Initialize the expat submodule with: git submodule update --init", file=sys.stderr)
        return None
    if not os.path.exists(r_path):
        print(f"Error: Rust source file not found: {r_path}", file=sys.stderr)
        return None

    c_src = open(c_path, 'rb').read()
    r_src = open(r_path, 'rb').read()

    c_tree = parse_c(c_src)
    r_tree = parse_rust(r_src)

    c_node = find_function_node(c_tree, c_func_name, "c")
    r_node = find_function_node(r_tree, r_func_name, "rust")

    if not c_node:
        print(f"Error: C function '{c_func_name}' not found in {c_path}", file=sys.stderr)
        return None
    if not r_node:
        print(f"Error: Rust function '{r_func_name}' not found in {r_path}", file=sys.stderr)
        return None

    c_text = c_node.text.decode()
    r_text = r_node.text.decode()

    # Collect from Rust function and any split functions
    r_errors = extract_all_errors(r_node, "rust")
    r_handlers = extract_all_handlers(r_node, "rust")
    r_cases = extract_switch_cases_ast(r_node, "rust")
    extra_lines = 0

    if extra_rust_funcs:
        for efn in extra_rust_funcs:
            enode = find_function_node(r_tree, efn, "rust")
            if enode:
                r_errors |= extract_all_errors(enode, "rust")
                r_handlers |= extract_all_handlers(enode, "rust")
                r_cases.extend(extract_switch_cases_ast(enode, "rust"))
                extra_lines += enode.text.decode().count('\n') + 1

    # Load suppressions
    suppressed_errors = get_suppressed_errors(r_func_name)
    suppressed_handlers = get_suppressed_handlers(r_func_name)
    per_func_suppressed_calls = get_per_function_suppressed_calls(r_func_name)

    divergences = []

    # 1. Overall errors
    c_errors = extract_all_errors(c_node, "c")
    c_mapped = {ERROR_MAP.get(e, f"?{e}") for e in c_errors}
    unmapped = {f"?{e}" for e in c_errors if e not in ERROR_MAP}
    missing_errs = sorted(c_mapped - r_errors - suppressed_errors - unmapped)
    if missing_errs:
        divergences.append(("MEDIUM", "missing_errors", missing_errs))

    # 2. Overall handlers
    c_handlers = extract_all_handlers(c_node, "c")
    c_mapped_h = {HANDLER_MAP.get(h, f"?{h}") for h in c_handlers}
    unmapped_h = {f"?{h}" for h in c_handlers if h not in HANDLER_MAP}
    missing_h = sorted(c_mapped_h - r_handlers - suppressed_handlers - unmapped_h)
    if missing_h:
        divergences.append(("MEDIUM", "missing_handlers", missing_h))

    # 3. Case-by-case comparison
    c_cases = extract_switch_cases_ast(c_node, "c")

    c_case_map = {}
    for case in c_cases:
        rust_label = TOKEN_MAP.get(case["label"])
        if rust_label:
            c_case_map[rust_label] = case
        rust_role = ROLE_MAP.get(case["label"])
        if rust_role:
            c_case_map[rust_role] = case

    r_case_map = {}
    for case in r_cases:
        for part in case["label"].split("|"):
            part = part.strip()
            r_case_map[part] = case
            # Also store with module prefix stripped (e.g., "xmlrole::Role::None" → "Role::None")
            if "::" in part:
                # Strip leading module paths: "xmlrole::Role::None" → "Role::None"
                segments = part.split("::")
                if len(segments) >= 2:
                    short = "::".join(segments[-2:])
                    if short not in r_case_map:
                        r_case_map[short] = case

    # Check for wildcard/default patterns that handle unmatched cases
    has_rust_wildcard = "_" in r_case_map or any(
        p.strip() in ("_", "..") for p in r_case_map.keys()
    )
    missing_cases = sorted(set(c_case_map.keys()) - set(r_case_map.keys()))
    if missing_cases and not has_rust_wildcard:
        divergences.append(("HIGH", "missing_match_arms", missing_cases))
    elif missing_cases and has_rust_wildcard:
        # Cases are handled by wildcard — note but don't flag as HIGH
        divergences.append(("LOW", "match_arms_handled_by_wildcard", missing_cases))

    # Per-case divergences
    for rust_label, c_case in c_case_map.items():
        if rust_label not in r_case_map:
            continue
        r_case = r_case_map[rust_label]

        # Compare errors in this case
        c_case_errs = {ERROR_MAP.get(e, f"?{e}") for e in c_case["errors"]}
        r_case_errs = set(r_case["errors"])
        case_missing_errs = c_case_errs - r_case_errs - suppressed_errors - {f"?{e}" for e in c_case["errors"] if e not in ERROR_MAP}
        if case_missing_errs:
            divergences.append(("LOW", f"case {rust_label}: missing errors", sorted(case_missing_errs)))

        # Compare handlers in this case
        c_case_h = {HANDLER_MAP.get(h, f"?{h}") for h in c_case["handlers"]}
        r_case_h = set(r_case["handlers"])
        case_missing_h = c_case_h - r_case_h - suppressed_handlers - {f"?{h}" for h in c_case["handlers"] if h not in HANDLER_MAP}
        if case_missing_h:
            divergences.append(("LOW", f"case {rust_label}: missing handlers", sorted(case_missing_h)))

        # Compare function calls in this case
        c_calls = set(c_case.get("calls", []))
        r_calls = set(r_case.get("calls", []))
        c_mapped_calls = set()
        for c in c_calls:
            if c in suppressed_calls:
                continue
            if not is_plausible_c_function_name(c):
                continue
            c_mapped_calls.add(c_to_rust_call_name(c))
        missing_calls = sorted(c_mapped_calls - r_calls - per_func_suppressed_calls)
        if missing_calls:
            divergences.append(("LOW", f"case {rust_label}: missing calls", missing_calls))

    # 4. Overall function call comparison
    c_all_calls = set()
    for case in c_cases:
        c_all_calls |= set(case.get("calls", []))
    r_all_calls = set()
    for case in r_cases:
        r_all_calls |= set(case.get("calls", []))
    c_mapped_all = set()
    for c in c_all_calls:
        if c in suppressed_calls:
            continue
        if not is_plausible_c_function_name(c):
            continue
        c_mapped_all.add(c_to_rust_call_name(c))
    missing_all_calls = sorted(c_mapped_all - r_all_calls - per_func_suppressed_calls)
    if missing_all_calls:
        divergences.append(("MEDIUM", "missing_function_calls", missing_all_calls))

    # 5. Function call argument comparison
    c_full_calls = re.findall(r'\b(\w+)\s*\(([^)]*)\)', c_text)
    r_full_calls = re.findall(r'\b(\w+)\s*\(([^)]*)\)', r_text)
    c_call_args = {}
    for name, args in c_full_calls:
        if name in suppressed_calls or not is_plausible_c_function_name(name):
            continue
        rust_name = c_to_rust_call_name(name)
        if rust_name not in c_call_args:
            c_call_args[rust_name] = set()
        arg_parts = [a.strip() for a in args.split(',')][:3]
        c_call_args[rust_name].add(tuple(arg_parts))

    r_call_args = {}
    for name, args in r_full_calls:
        if name not in r_call_args:
            r_call_args[name] = set()
        arg_parts = [a.strip() for a in args.split(',')][:3]
        r_call_args[name].add(tuple(arg_parts))

    for func_name in sorted(set(c_call_args) & set(r_call_args)):
        c_args = c_call_args[func_name]
        r_args = r_call_args[func_name]
        if c_args != r_args:
            c_nums = {a for arg_tuple in c_args for a in arg_tuple if a.isdigit()}
            r_nums = {a for arg_tuple in r_args for a in arg_tuple if a.isdigit()}
            if c_nums != r_nums and c_nums:
                divergences.append(("LOW", f"call {func_name}: C args include {c_nums}, Rust args include {r_nums}", []))

    rust_total_lines = r_text.count('\n') + 1 + extra_lines
    return {
        "c_function": c_func_name,
        "rust_function": r_func_name,
        "extra_rust_funcs": extra_rust_funcs or [],
        "c_lines": c_text.count('\n') + 1,
        "rust_lines": rust_total_lines,
        "c_cases": len(c_cases),
        "rust_cases": len(r_cases),
        "divergences": divergences,
    }


def format_report(results):
    """Format as human-readable report."""
    if results is None:
        return "Comparison failed."

    lines = []
    func_desc = results['rust_function']
    if results.get('extra_rust_funcs'):
        func_desc += " + " + ", ".join(results['extra_rust_funcs'])
    lines.append(f"=== {results['c_function']} ({results['c_lines']}L, {results['c_cases']} cases) vs {func_desc} ({results['rust_lines']}L, {results['rust_cases']} cases) ===")

    if not results["divergences"]:
        lines.append("  ✓ No divergences found")
        return "\n".join(lines)

    for sev, desc, details in sorted(results["divergences"]):
        marker = "✗" if sev == "HIGH" else "⚠" if sev == "MEDIUM" else "·"
        lines.append(f"  {marker} [{sev}] {desc}: {details}")

    return "\n".join(lines)


def get_pairs():
    """Get function pairs from deliberate-divergences.json."""
    config = load_divergences()
    pairs = []
    for fp in config.get("function_pairs", []):
        c_func = fp["c_function"]
        r_func = fp["rust_function"]
        extra = fp.get("split_functions", [])
        pairs.append((c_func, r_func, extra))
    if not pairs:
        pairs = [
            ("doContent", "do_content", []),
            ("epilogProcessor", "epilog_processor", []),
            ("doCdataSection", "do_cdata_section", []),
            ("doProlog", "do_prolog", ["handle_prolog_role"]),
            ("contentProcessor", "content_processor", []),
            ("prologProcessor", "prolog_processor", []),
            ("processXmlDecl", "handle_prolog_role", []),
            ("reportComment", "report_comment", []),
            ("reportProcessingInstruction", "report_processing_instruction", []),
            ("reportDefault", "report_default", []),
        ]
    return pairs


def extract_c_case_source(c_func_name, case_label):
    """Extract the C source code for a specific switch case."""
    c_src = open(C_FILE, 'rb').read()
    c_tree = parse_c(c_src)
    c_node = find_function_node(c_tree, c_func_name, "c")
    if not c_node:
        return None
    for node in walk_all(c_node):
        if node.type == "case_statement":
            value_node = node.child_by_field_name("value") or (node.children[1] if len(node.children) > 1 else None)
            if value_node and value_node.text.decode().strip() == case_label.strip():
                return node.text.decode()
    c_text = c_node.text.decode()
    marker = f"case {case_label}:"
    idx = c_text.find(marker)
    if idx >= 0:
        end = c_text.find("\n    case ", idx + 1)
        if end < 0:
            end = min(idx + 500, len(c_text))
        return c_text[idx:end].strip()
    return None


def extract_rust_function_source(func_name):
    """Extract the full Rust function source."""
    r_src = open(RUST_FILE, 'rb').read()
    r_tree = parse_rust(r_src)
    r_node = find_function_node(r_tree, func_name, "rust")
    if not r_node:
        return None
    return r_node.text.decode()


def generate_prompt(c_func_name, r_func_name, extra_rust_funcs=None):
    """Generate a prompt for fixing divergences between C and Rust functions."""
    results = compare(c_func_name, r_func_name, extra_rust_funcs)
    if not results or not results["divergences"]:
        return "No divergences found — nothing to fix."

    c_src = open(C_FILE, 'rb').read()
    c_tree = parse_c(c_src)
    c_node = find_function_node(c_tree, c_func_name, "c")

    c_cases = extract_switch_cases_ast(c_node, "c")
    c_case_by_label = {c["label"]: c for c in c_cases}

    missing_arms = []
    case_fixes = []
    missing_errors_overall = []
    missing_handlers_overall = []

    for sev, desc, details in results["divergences"]:
        if "missing_match_arms" in desc:
            missing_arms = details
        elif desc.startswith("case ") and ("missing errors" in desc or "missing handlers" in desc or "missing calls" in desc):
            case_fixes.append((desc.split(": ")[1].split()[1] if ": " in desc else "fix", desc, details))
        elif desc == "missing_errors":
            missing_errors_overall = details
        elif desc == "missing_handlers":
            missing_handlers_overall = details

    lines = []
    lines.append(f"# Task: Fix divergences in {r_func_name} (Rust) to match {c_func_name} (C)")
    lines.append("")
    lines.append("You are porting C libexpat to Rust. Fix the Rust implementation to match C behavior.")
    lines.append("")
    lines.append("## File to modify")
    lines.append(f"- `expat-rust/src/xmlparse.rs` — function `{r_func_name}`")
    if extra_rust_funcs:
        for ef in extra_rust_funcs:
            lines.append(f"- `expat-rust/src/xmlparse.rs` — function `{ef}`")
    lines.append("")

    if missing_arms:
        lines.append("## Missing match arms (cases C handles but Rust doesn't)")
        lines.append("")
        for arm in missing_arms:
            c_label = None
            for cl in c_case_by_label:
                mapped = TOKEN_MAP.get(cl) or ROLE_MAP.get(cl)
                if mapped == arm:
                    c_label = cl
                    break
            if c_label:
                c_source = extract_c_case_source(c_func_name, c_label)
                if c_source:
                    lines.append(f"### {arm} (C: {c_label})")
                    lines.append("```c")
                    lines.append(c_source.strip())
                    lines.append("```")
                    lines.append("")
                    c_case = c_case_by_label.get(c_label, {})
                    if c_case.get("errors"):
                        lines.append(f"Errors used: {c_case['errors']}")
                    if c_case.get("handlers"):
                        lines.append(f"Handlers called: {c_case['handlers']}")
                    if c_case.get("calls"):
                        lines.append(f"Functions called: {c_case['calls']}")
                    lines.append("")
            else:
                lines.append(f"### {arm}")
                lines.append("(Could not find C source for this case)")
                lines.append("")

    if case_fixes:
        lines.append("## Existing cases that need fixes")
        lines.append("")
        for fix_type, desc, details in case_fixes:
            m = re.match(r'case ([\w:]+): missing', desc)
            rust_label = m.group(1) if m else desc.split(":")[0].replace("case ", "")
            c_label = None
            for cl in c_case_by_label:
                mapped = TOKEN_MAP.get(cl) or ROLE_MAP.get(cl)
                if mapped == rust_label:
                    c_label = cl
                    break
            lines.append(f"### {desc}")
            if c_label:
                c_source = extract_c_case_source(c_func_name, c_label)
                if c_source:
                    lines.append(f"C source ({c_label}):")
                    lines.append("```c")
                    lines.append(c_source.strip())
                    lines.append("```")
            lines.append(f"Missing: {details}")
            lines.append("")

    if missing_errors_overall:
        lines.append(f"## Overall missing error codes: {missing_errors_overall}")
        lines.append("These errors appear in C but not in Rust. Add them to the appropriate cases.")
        lines.append("")
    if missing_handlers_overall:
        lines.append(f"## Overall missing handlers: {missing_handlers_overall}")
        lines.append("These handler calls appear in C but not in Rust.")
        lines.append("")

    all_missing_calls = []
    for s, d, det in results["divergences"]:
        if d == "missing_function_calls":
            all_missing_calls = det
    if all_missing_calls:
        lines.append(f"## Overall missing function calls: {all_missing_calls}")
        lines.append("These C functions are called but have no Rust equivalent call.")
        lines.append("Each needs to be implemented or the logic inlined.")
        lines.append("")

    lines.append("## Rules")
    lines.append("- No unsafe code")
    lines.append("- Match C behavior exactly")
    lines.append("- Keep changes minimal and focused")
    lines.append("- Use Rust std types (String, Vec, HashMap) instead of C pools/hash tables")
    lines.append("- Read the existing Rust code before making changes")
    lines.append("")
    lines.append("## Verification")
    lines.append("```bash")
    lines.append("cargo build -p expat-rust 2>&1 | tail -5")
    lines.append("```")

    return "\n".join(lines)


def cmd_audit():
    """Show all suppressions with their justifications and status."""
    config = load_divergences()

    print("=" * 80)
    print("AST COMPARE SUPPRESSION AUDIT")
    print(f"Divergences file: {DIVERGENCES_FILE}")
    print(f"Last audited: {config.get('last_audited', 'unknown')}")
    print("=" * 80)
    print()

    # Global error suppressions
    print("── Global Error Suppressions ──")
    for code, info in config.get("global_suppressions", {}).get("errors", {}).items():
        if isinstance(info, dict):
            status = info.get("status", "unknown")
            justification = info.get("justification", "NONE")
            category = info.get("category", "uncategorized")
        else:
            status = "accepted"
            justification = info
            category = "legacy"
        print(f"  {code}: [{status}] ({category})")
        print(f"    {justification}")
    print()

    # Global handler suppressions
    print("── Global Handler Suppressions ──")
    handlers = config.get("global_suppressions", {}).get("handlers", {})
    if not handlers:
        print("  (none)")
    for name, info in handlers.items():
        print(f"  {name}: {info}")
    print()

    # Call suppressions
    print("── Suppressed Call Categories ──")
    calls_config = config.get("global_suppressions", {}).get("calls", {})
    total_suppressed = 0
    for category, info in calls_config.items():
        if not isinstance(info, dict):
            continue
        calls = info.get("calls", [])
        status = info.get("status", "unknown")
        justification = info.get("justification", "NONE")
        total_suppressed += len(calls)
        print(f"  {category} ({len(calls)} calls) [{status}]")
        print(f"    Justification: {justification}")
        print(f"    Calls: {', '.join(sorted(calls))}")
        print()
    print(f"  Total suppressed calls: {total_suppressed}")
    print()

    # NOT_SUPPRESSED (must-port)
    print("── NOT SUPPRESSED (Must Port) ──")
    not_suppressed = config.get("global_suppressions", {}).get("NOT_SUPPRESSED", {})
    for category, info in not_suppressed.items():
        if category.startswith("_"):
            continue
        if not isinstance(info, dict):
            continue
        calls = info.get("calls", [])
        error = info.get("error", "")
        status = info.get("status", "unknown")
        reason = info.get("why_not_suppressed", "no reason given")
        print(f"  ✗ {category} [{status}]")
        if calls:
            print(f"    Calls: {', '.join(sorted(calls))}")
        if error:
            print(f"    Error: {error}")
        print(f"    Why: {reason}")
        print()

    # Per-function suppressions
    print("── Per-Function Suppressions ──")
    per_func = config.get("per_function_suppressions", {})
    for func, info in per_func.items():
        errs = info.get("suppressed_errors", [])
        handlers = info.get("suppressed_handlers", [])
        justification = info.get("justification", info.get("notes", "NONE"))
        if errs or handlers:
            print(f"  {func}:")
            if errs:
                print(f"    Suppressed errors: {errs}")
            if handlers:
                print(f"    Suppressed handlers: {handlers}")
            print(f"    Justification: {justification}")
        else:
            print(f"  {func}: (no suppressions)")
    print()

    # Function pairs
    print("── Tracked Function Pairs ──")
    for fp in config.get("function_pairs", []):
        c = fp["c_function"]
        r = fp["rust_function"]
        extra = fp.get("split_functions", [])
        notes = fp.get("notes", "")
        desc = f"{c} → {r}"
        if extra:
            desc += f" (+ {', '.join(extra)})"
        print(f"  {desc}")
        if notes:
            print(f"    {notes}")
    print()


def cmd_ci():
    """CI mode: compare all pairs, exit 1 if any divergences found."""
    pairs = get_pairs()
    total_divergences = 0
    failed_pairs = []
    passed_pairs = []
    errors = []

    for item in pairs:
        c_name, r_name = item[0], item[1]
        extra = item[2] if len(item) > 2 else []
        results = compare(c_name, r_name, extra)
        if results is None:
            errors.append(f"{c_name} → {r_name}: comparison failed (missing source?)")
            continue
        report = format_report(results)
        print(report)
        print()
        if results["divergences"]:
            total_divergences += len(results["divergences"])
            failed_pairs.append((c_name, r_name, len(results["divergences"])))
        else:
            passed_pairs.append((c_name, r_name))

    # Summary
    print("=" * 80)
    print("CI SUMMARY")
    print("=" * 80)
    print(f"Pairs compared: {len(pairs)}")
    print(f"Passed (no divergences): {len(passed_pairs)}")
    print(f"Failed (has divergences): {len(failed_pairs)}")
    print(f"Errors (could not compare): {len(errors)}")
    print(f"Total divergences: {total_divergences}")
    print()

    if errors:
        print("ERRORS:")
        for e in errors:
            print(f"  ✗ {e}")
        print()

    if failed_pairs:
        print("FAILED PAIRS:")
        for c, r, n in failed_pairs:
            print(f"  ✗ {c} → {r}: {n} divergence(s)")
        print()

    if passed_pairs:
        print("PASSED PAIRS:")
        for c, r in passed_pairs:
            print(f"  ✓ {c} → {r}")
        print()

    # Show suppression stats for transparency
    suppressed = get_suppressed_calls()
    never = set()
    config = load_divergences()
    not_suppressed = config.get("global_suppressions", {}).get("NOT_SUPPRESSED", {})
    for cat, info in not_suppressed.items():
        if cat.startswith("_") or not isinstance(info, dict):
            continue
        for call in info.get("calls", []):
            never.add(call)

    print(f"Suppressed calls: {len(suppressed)} (from deliberate-divergences.json)")
    print(f"Never-suppress calls: {len(never)} (must be ported)")
    print()

    if total_divergences > 0 or errors:
        print("RESULT: FAIL")
        return 1
    else:
        print("RESULT: PASS")
        return 0


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    # Parse --files flag to override C/Rust source files
    global C_FILE, RUST_FILE
    if sys.argv[1] == "--files":
        if len(sys.argv) < 5:
            print("Usage: ast-compare.py --files <c_file> <rust_file> <command> ...")
            sys.exit(1)
        C_FILE = os.path.join(ROOT, sys.argv[2])
        RUST_FILE = os.path.join(ROOT, sys.argv[3])
        sys.argv = [sys.argv[0]] + sys.argv[4:]

    if sys.argv[1] == "--tok":
        tok_c = os.path.join(ROOT, "expat", "expat", "lib", "xmltok.c")
        if not os.path.exists(tok_c):
            tok_c = os.path.join(ROOT, "expat", "lib", "xmltok.c")
        C_FILE = tok_c
        RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmltok.rs")
        sys.argv = [sys.argv[0]] + sys.argv[2:]

    if sys.argv[1] == "--tok-impl":
        tok_c = os.path.join(ROOT, "expat", "expat", "lib", "xmltok_impl.c")
        if not os.path.exists(tok_c):
            tok_c = os.path.join(ROOT, "expat", "lib", "xmltok_impl.c")
        C_FILE = tok_c
        RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmltok_impl.rs")
        sys.argv = [sys.argv[0]] + sys.argv[2:]

    if sys.argv[1] == "--audit":
        cmd_audit()
        return

    if sys.argv[1] == "--ci":
        exit_code = cmd_ci()
        sys.exit(exit_code)

    if sys.argv[1] == "--all":
        pairs = get_pairs()
        for item in pairs:
            c_name, r_name = item[0], item[1]
            extra = item[2] if len(item) > 2 else []
            results = compare(c_name, r_name, extra)
            if results:
                print(format_report(results))
                print()
        return

    if sys.argv[1] == "--prompt":
        if len(sys.argv) < 4:
            print("Usage: ast-compare.py --prompt <c_func> <rust_func> [extra_rust_funcs...]")
            sys.exit(1)
        c_name = sys.argv[2]
        r_name = sys.argv[3]
        extra = sys.argv[4:] if len(sys.argv) > 4 else []
        prompt = generate_prompt(c_name, r_name, extra if extra else None)
        print(prompt)
        return

    if sys.argv[1] == "--missing-functions":
        c_src = open(C_FILE, 'rb').read()
        r_src = open(RUST_FILE, 'rb').read()
        c_tree = parse_c(c_src)
        r_tree = parse_rust(r_src)
        suppressed_calls = get_suppressed_calls()
        c_funcs = {}
        for node in walk_all(c_tree.root_node):
            if node.type == "function_definition":
                decl = node.child_by_field_name("declarator")
                if decl:
                    for child in walk_all(decl):
                        if child.type == "identifier":
                            name = child.text.decode()
                            lines = node.text.decode().count('\n') + 1
                            c_funcs[name] = lines
                            break
        r_funcs = set()
        for node in walk_all(r_tree.root_node):
            if node.type == "function_item":
                name_node = node.child_by_field_name("name")
                if name_node:
                    r_funcs.add(name_node.text.decode())
        missing = []
        for c_name, lines in sorted(c_funcs.items()):
            rust_name = c_to_rust_call_name(c_name)
            if rust_name not in r_funcs and c_name not in suppressed_calls:
                skip = False
                if c_name.startswith("pool") or c_name.startswith("hash"):
                    skip = True
                if not skip:
                    missing.append((c_name, rust_name, lines))
        print(f"C functions in xmlparse.c with no Rust equivalent ({len(missing)} of {len(c_funcs)}):")
        print(f"{'C Function':<40} {'Expected Rust Name':<35} {'Lines':>5}")
        print("-" * 82)
        for c_name, r_name, lines in sorted(missing, key=lambda x: -x[2]):
            print(f"{c_name:<40} {r_name:<35} {lines:>5}")
        return

    if sys.argv[1] == "--prompt-all":
        pairs = get_pairs()
        for item in pairs:
            c_name, r_name = item[0], item[1]
            extra = item[2] if len(item) > 2 else []
            results = compare(c_name, r_name, extra)
            if results and results["divergences"]:
                prompt = generate_prompt(c_name, r_name, extra if extra else None)
                print(f"{'='*80}")
                print(f"PROMPT FOR: {c_name} -> {r_name}")
                print(f"{'='*80}")
                print(prompt)
                print()
        return

    if sys.argv[1] == "--list-cases":
        func_name = sys.argv[2]
        lang = sys.argv[3] if len(sys.argv) > 3 else "c"
        src_file = C_FILE if lang == "c" else RUST_FILE
        src = open(src_file, 'rb').read()
        tree = parse_c(src) if lang == "c" else parse_rust(src)
        node = find_function_node(tree, func_name, lang)
        if not node:
            print(f"Function '{func_name}' not found")
            sys.exit(1)
        cases = extract_switch_cases_ast(node, lang)
        for case in cases:
            print(f"{case['label']} ({case['lines']}L) errors={case['errors']} handlers={case['handlers']}")
        return

    c_name = sys.argv[1]
    r_name = sys.argv[2]
    extra = sys.argv[3:] if len(sys.argv) > 3 and sys.argv[3] != "--json" else []
    extra = [e for e in extra if e != "--json"]

    if "--json" in sys.argv:
        results = compare(c_name, r_name, extra if extra else None)
        print(json.dumps(results, indent=2, default=list))
    else:
        results = compare(c_name, r_name, extra if extra else None)
        print(format_report(results))


if __name__ == "__main__":
    main()
