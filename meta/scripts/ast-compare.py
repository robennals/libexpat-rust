#!/usr/bin/env python3
"""AST-based structural comparison of C and Rust function implementations.

Uses tree-sitter to parse both C and Rust to ASTs, then compares:
- Switch/match case coverage
- Error codes per case
- Handler calls per case
- Control flow structure (early returns, loops)
- Function calls

Loads deliberate-divergences.json to suppress known intentional differences.

Usage:
    python3 scripts/ast-compare.py <c_func> <rust_func>
    python3 scripts/ast-compare.py doContent do_content
    python3 scripts/ast-compare.py --all              # Compare all known pairs
    python3 scripts/ast-compare.py --list-cases <func> c|rust  # List cases in a function
    python3 scripts/ast-compare.py --prompt <c_func> <rust_func> [extra...]  # Generate Haiku porting prompt
    python3 scripts/ast-compare.py --prompt-all       # Generate prompts for all divergent pairs
"""

import sys
import os
import json
import re
import tree_sitter
import tree_sitter_c
import tree_sitter_rust

# ROOT is two levels up from meta/scripts/
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
# Support both submodule layout (expat/expat/lib/) and flat layout (expat/lib/)
C_FILE = os.path.join(ROOT, "expat", "expat", "lib", "xmlparse.c")
if not os.path.exists(C_FILE):
    C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")
DIVERGENCES_FILE = os.path.join(ROOT, "scripts", "deliberate-divergences.json")
if not os.path.exists(DIVERGENCES_FILE):
    DIVERGENCES_FILE = os.path.join(ROOT, "meta", "scripts", "deliberate-divergences.json")

# Initialize parsers
C_LANG = tree_sitter.Language(tree_sitter_c.language())
RUST_LANG = tree_sitter.Language(tree_sitter_rust.language())

# Load deliberate divergences
_divergences_config = None

def load_divergences():
    global _divergences_config
    if _divergences_config is not None:
        return _divergences_config
    try:
        with open(DIVERGENCES_FILE) as f:
            _divergences_config = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError):
        _divergences_config = {"global_suppressions": {"errors": {}, "handlers": {}},
                               "function_pairs": [], "per_function_suppressions": {}}
    return _divergences_config


def get_suppressed_errors(rust_func_name):
    """Get error codes that should be suppressed for this function."""
    config = load_divergences()
    suppressed = set(config.get("global_suppressions", {}).get("errors", {}).keys())
    per_func = config.get("per_function_suppressions", {}).get(rust_func_name, {})
    suppressed |= set(per_func.get("suppressed_errors", []))
    return suppressed


def get_suppressed_handlers(rust_func_name):
    """Get handler names that should be suppressed for this function."""
    config = load_divergences()
    suppressed = set(config.get("global_suppressions", {}).get("handlers", {}).keys())
    per_func = config.get("per_function_suppressions", {}).get(rust_func_name, {})
    suppressed |= set(per_func.get("suppressed_handlers", []))
    return suppressed


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
            # Find the function declarator
            decl = node.child_by_field_name("declarator")
            if decl:
                # Walk into declarator to find identifier
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
            # Get the case value
            value_node = node.child_by_field_name("value") or (node.children[1] if len(node.children) > 1 else None)
            if value_node:
                value = value_node.text.decode()
                # Extract what this case does
                body_text = node.text.decode()
                errors = set(re.findall(r'XML_ERROR_(\w+)', body_text))
                handlers = set(re.findall(r'parser->m_(\w+Handler)', body_text))
                raw_calls = set(re.findall(r'\b(\w+)\s*\(', body_text))
                # Filter out: keywords, handler field accesses, C macros, type casts
                func_calls = raw_calls - {
                    'if', 'while', 'for', 'switch', 'return', 'sizeof', 'assert',
                    'XML_T', 'XML_L', 'XML_TRUE', 'XML_FALSE', 'CHAR_HASH',
                    'XCS', 'INT_MAX', 'UINT_MAX', 'SIZE_MAX',
                }
                # Remove handler field accesses (m_*Handler)
                func_calls = {c for c in func_calls if not re.match(r'm_\w+Handler$', c)}
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
            # Get the pattern
            pattern_node = node.child_by_field_name("pattern")
            if pattern_node:
                pattern = pattern_node.text.decode()
                body_text = node.text.decode()
                errors = set(re.findall(r'XmlError::(\w+)', body_text))
                handlers = set(re.findall(r'self\.(\w+_handler)', body_text))
                # report_default() is equivalent to calling default_handler
                if 'report_default' in body_text:
                    handlers.add('default_handler')
                # report_comment() is equivalent to calling comment_handler + default_handler
                if 'report_comment' in body_text:
                    handlers.add('comment_handler')
                    handlers.add('default_handler')
                # report_processing_instruction() is equivalent to calling processing_instruction_handler + default_handler
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
        # Recognize helper functions that internally call handlers
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
    # Only exceptions to the standard camelCase→snake_case rule
    # Standard rule: "doContent" → "do_content" is derived automatically
    "storeAtts": "process_namespaces",  # Rust replaces C's storeAtts with namespace processing
    "XmlNameLength": "name_length",     # Xml prefix stripped
    "XmlGetAttributes": "get_atts",     # different abbreviation
    "XmlContentTok": "content_tok",     # Xml prefix stripped
    "XmlPrologTok": "prolog_tok",       # Xml prefix stripped
    "XmlCdataSectionTok": "cdata_section_tok",  # Xml prefix stripped
    "XmlCharRefNumber": "char_ref_number",      # Xml prefix stripped
    "XmlIgnoreSectionTok": "ignore_section_tok", # Xml prefix stripped
}


def c_to_rust_call_name(c_name):
    """Convert a C function name to its expected Rust equivalent.
    Uses CALL_MAP for exceptions, otherwise converts camelCase to snake_case."""
    if c_name in CALL_MAP:
        return CALL_MAP[c_name]
    # Convert camelCase/PascalCase to snake_case
    import re
    # Insert _ before uppercase letters that follow lowercase
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', c_name)
    # Insert _ before uppercase letters that are followed by lowercase (for sequences like "XMLDecl")
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    return s.lower()

# C calls that are intentionally not in Rust (memory management, pools, etc.)
SUPPRESSED_CALLS = {
    # Memory management
    "poolStoreString", "poolAppend", "poolAppendString", "poolCopyString",
    "poolCopyStringN", "poolFinish", "poolDiscard", "poolStart", "poolGrow",
    "poolClear", "poolDestroy", "poolInit", "poolLastString", "poolLength",
    "poolBytesToAllocateFor", "poolChop", "poolCopyStringNoFinish",
    "REALLOC", "MALLOC", "FREE",
    # Hash tables
    "lookup", "hash", "hashTableInit", "hashTableClear", "hashTableDestroy",
    # Entropy/randomization
    "generate_hash_secret_salt", "gather_time_entropy",
    # C string/conversion ops that are handled inline in Rust
    "memcpy", "memcmp", "strcmp", "strlen", "XML_T", "XmlConvert",
    "mustConvert", "XmlEncode", "getContext",
    "CHAR_HASH", "dtdCopy", "dtdReset",
    # Type system / casting
    "ENTITY", "ELEMENT_TYPE", "ATTRIBUTE_ID", "PREFIX", "TAG",
    # Macro-like patterns
    "charDataHandler", "XML_Char",
    # C-only internal functions/macros with no Rust equivalent
    "mustConvert", "MUST_CONVERT", "lines",
    "poolAppendChar", "POOL_APPEND_CHAR",
    "malloc_fcn", "free_fcn", "free",
    "XmlConvert", "XmlEncode",
    "XmlPredefinedEntityName", "XmlIsPublicId",
    # Accounting (Rust doesn't do C-style accounting)
    "accountingOnAbort", "accountingDiffTolerated",
    # Content model building helpers (Rust uses different approach)
    "buildModel", "nextScaffoldPart",
    # Accounting
    "accountingDiffTolerated", "entityTrackingOnOpen", "entityTrackingOnClose",
    "entityTrackingReportStats",
    # String ops that have Rust equivalents inline
    "memcpy", "memcmp", "strcmp", "strlen",
    # C-specific
    "parserBusy", "parserInit",
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

# C roles map to Rust Role:: variants
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
    "XML_ROLE_NOTATION_NAME": "Role::NotationName",
    "XML_ROLE_ATTLIST_ELEMENT_NAME": "Role::AttlistElementName",
    "XML_ROLE_ATTRIBUTE_NAME": "Role::AttributeName",
    "XML_ROLE_ELEMENT_NAME": "Role::ElementName",
}


def compare(c_func_name, r_func_name, extra_rust_funcs=None):
    """Main comparison function.

    extra_rust_funcs: list of additional Rust function names whose errors/handlers/cases
    should be merged into the Rust side (for split functions like handle_prolog_role).
    """
    c_src = open(C_FILE, 'rb').read()
    r_src = open(RUST_FILE, 'rb').read()

    c_tree = parse_c(c_src)
    r_tree = parse_rust(r_src)

    c_node = find_function_node(c_tree, c_func_name, "c")
    r_node = find_function_node(r_tree, r_func_name, "rust")

    if not c_node:
        print(f"Error: C function '{c_func_name}' not found in AST", file=sys.stderr)
        return None
    if not r_node:
        print(f"Error: Rust function '{r_func_name}' not found in AST", file=sys.stderr)
        return None

    c_text = c_node.text.decode()
    r_text = r_node.text.decode()

    # Collect errors/handlers/cases from the Rust function and any extra split functions
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

    # Map C case labels to Rust
    c_case_map = {}
    for case in c_cases:
        rust_label = TOKEN_MAP.get(case["label"])
        if rust_label:
            c_case_map[rust_label] = case
        # Also check role labels for doProlog
        rust_role = ROLE_MAP.get(case["label"])
        if rust_role:
            c_case_map[rust_role] = case

    r_case_map = {}
    for case in r_cases:
        # Normalize label (handle combined patterns like "XmlTok::A | XmlTok::B")
        for part in case["label"].split("|"):
            part = part.strip()
            r_case_map[part] = case

    missing_cases = sorted(set(c_case_map.keys()) - set(r_case_map.keys()))
    if missing_cases:
        divergences.append(("HIGH", "missing_match_arms", missing_cases))

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
        # Map C calls to Rust equivalents using auto snake_case + exceptions
        c_mapped_calls = set()
        for c in c_calls:
            if c in SUPPRESSED_CALLS:
                continue
            # Skip ALL_CAPS names (C macros)
            if c.isupper() or (c.replace('_', '').isupper() and '_' in c):
                continue
            # Skip single-letter names and very short names (loop vars, etc.)
            if len(c) <= 2:
                continue
            c_mapped_calls.add(c_to_rust_call_name(c))
        missing_calls = sorted(c_mapped_calls - r_calls)
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
        if c in SUPPRESSED_CALLS:
            continue
        if c.isupper() or (c.replace('_', '').isupper() and '_' in c):
            continue
        if len(c) <= 2:
            continue
        c_mapped_all.add(c_to_rust_call_name(c))
    missing_all_calls = sorted(c_mapped_all - r_all_calls)
    if missing_all_calls:
        divergences.append(("MEDIUM", "missing_function_calls", missing_all_calls))

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
        lines.append("  No divergences found!")
        return "\n".join(lines)

    for sev, desc, details in sorted(results["divergences"]):
        lines.append(f"  [{sev}] {desc}: {details}")

    return "\n".join(lines)


# Load function pairs from config, falling back to defaults
def get_pairs():
    """Get function pairs from deliberate-divergences.json or use defaults."""
    config = load_divergences()
    pairs = []
    for fp in config.get("function_pairs", []):
        c_func = fp["c_function"]
        r_func = fp["rust_function"]
        extra = fp.get("split_functions", [])
        pairs.append((c_func, r_func, extra))
    if not pairs:
        # Fallback to hardcoded pairs
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
    # Fallback: search by substring in case AST label differs
    c_text = c_node.text.decode()
    marker = f"case {case_label}:"
    idx = c_text.find(marker)
    if idx >= 0:
        # Extract until next "case " or "break;" or "}" at same indent
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
    """Generate a Haiku-ready prompt for fixing divergences between C and Rust functions."""
    results = compare(c_func_name, r_func_name, extra_rust_funcs)
    if not results or not results["divergences"]:
        return "No divergences found — nothing to fix."

    c_src = open(C_FILE, 'rb').read()
    r_src = open(RUST_FILE, 'rb').read()
    c_tree = parse_c(c_src)
    r_tree = parse_rust(r_src)
    c_node = find_function_node(c_tree, c_func_name, "c")

    # Collect C cases for missing match arms
    c_cases = extract_switch_cases_ast(c_node, "c")
    c_case_by_label = {c["label"]: c for c in c_cases}

    # Find missing match arms and per-case divergences
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
        elif desc == "missing_function_calls":
            pass  # Will include in overall section

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

    # Missing match arms — extract C source for each
    if missing_arms:
        lines.append("## Missing match arms (cases C handles but Rust doesn't)")
        lines.append("")
        for arm in missing_arms:
            # Find the original C label
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
                    # Show case metadata
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

    # Per-case divergences
    if case_fixes:
        lines.append("## Existing cases that need fixes")
        lines.append("")
        for fix_type, desc, details in case_fixes:
            # Extract Rust label from "case XmlTok::Foo: missing ..." or "case Role::Bar: missing ..."
            # Can't split on ":" because labels contain "::"
            m = re.match(r'case ([\w:]+): missing', desc)
            rust_label = m.group(1) if m else desc.split(":")[0].replace("case ", "")
            # Find C label via reverse mapping
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

    # Overall missing errors/handlers/calls
    if missing_errors_overall:
        lines.append(f"## Overall missing error codes: {missing_errors_overall}")
        lines.append("These errors appear in C but not in Rust. Add them to the appropriate cases.")
        lines.append("")
    if missing_handlers_overall:
        lines.append(f"## Overall missing handlers: {missing_handlers_overall}")
        lines.append("These handler calls appear in C but not in Rust.")
        lines.append("")

    # Missing function calls
    missing_calls = [d for s, d, det in results["divergences"] if d == "missing_function_calls"]
    if missing_calls:
        for det_list in [d for s, d, det in results["divergences"] if d == "missing_function_calls"]:
            pass
        all_missing_calls = []
        for s, d, det in results["divergences"]:
            if d == "missing_function_calls":
                all_missing_calls = det
        if all_missing_calls:
            lines.append(f"## Overall missing function calls: {all_missing_calls}")
            lines.append("These C functions are called but have no Rust equivalent call.")
            lines.append("Each needs to be implemented or the logic inlined.")
            lines.append("")

    # Rules
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


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

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
        # Generate a Haiku-ready prompt for fixing divergences
        # Usage: ast-compare.py --prompt <c_func> <rust_func> [extra_rust_funcs...]
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
        # List C functions that have no Rust equivalent
        c_src = open(C_FILE, 'rb').read()
        r_src = open(RUST_FILE, 'rb').read()
        c_tree = parse_c(c_src)
        r_tree = parse_rust(r_src)
        # Extract all C function names
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
        # Extract all Rust function names
        r_funcs = set()
        for node in walk_all(r_tree.root_node):
            if node.type == "function_item":
                name_node = node.child_by_field_name("name")
                if name_node:
                    r_funcs.add(name_node.text.decode())
        # Check which C functions have no Rust equivalent
        missing = []
        for c_name, lines in sorted(c_funcs.items()):
            rust_name = c_to_rust_call_name(c_name)
            if rust_name not in r_funcs and c_name not in SUPPRESSED_CALLS:
                # Also check if it's a suppressed category
                skip = False
                for sup in SUPPRESSED_CALLS:
                    if c_name.startswith(sup) or c_name == sup:
                        skip = True
                        break
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
        # Generate prompts for all function pairs that have divergences
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
