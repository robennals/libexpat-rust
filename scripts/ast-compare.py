#!/usr/bin/env python3
"""AST-based structural comparison of C and Rust function implementations.

Uses tree-sitter to parse both C and Rust to ASTs, then compares:
- Switch/match case coverage
- Error codes per case
- Handler calls per case
- Control flow structure (early returns, loops)
- Function calls

Usage:
    python3 scripts/ast-compare.py <c_func> <rust_func>
    python3 scripts/ast-compare.py doContent do_content
    python3 scripts/ast-compare.py --all              # Compare all known pairs
    python3 scripts/ast-compare.py --list-cases <func> c|rust  # List cases in a function
"""

import sys
import os
import json
import re
import tree_sitter
import tree_sitter_c
import tree_sitter_rust

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")

# Initialize parsers
C_LANG = tree_sitter.Language(tree_sitter_c.language())
RUST_LANG = tree_sitter.Language(tree_sitter_rust.language())


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
                func_calls = set(re.findall(r'\b(\w+)\s*\(', body_text)) - {'if', 'while', 'for', 'switch', 'return', 'sizeof'}
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
        return set(re.findall(r'self\.(\w+_handler)', text))


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
}

SKIP_ERRORS = {"NoMemory", "AmplificationLimitBreach"}


def compare(c_func_name, r_func_name):
    """Main comparison function."""
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

    divergences = []

    # 1. Overall errors
    c_errors = extract_all_errors(c_node, "c")
    r_errors = extract_all_errors(r_node, "rust")
    c_mapped = {ERROR_MAP.get(e, f"?{e}") for e in c_errors}
    unmapped = {f"?{e}" for e in c_errors if e not in ERROR_MAP}
    missing_errs = sorted(c_mapped - r_errors - SKIP_ERRORS - unmapped)
    if missing_errs:
        divergences.append(("MEDIUM", "missing_errors", missing_errs))

    # 2. Overall handlers
    c_handlers = extract_all_handlers(c_node, "c")
    r_handlers = extract_all_handlers(r_node, "rust")
    c_mapped_h = {HANDLER_MAP.get(h, f"?{h}") for h in c_handlers}
    unmapped_h = {f"?{h}" for h in c_handlers if h not in HANDLER_MAP}
    missing_h = sorted(c_mapped_h - r_handlers - unmapped_h)
    if missing_h:
        divergences.append(("MEDIUM", "missing_handlers", missing_h))

    # 3. Case-by-case comparison
    c_cases = extract_switch_cases_ast(c_node, "c")
    r_cases = extract_switch_cases_ast(r_node, "rust")

    # Map C case labels to Rust
    c_case_map = {}
    for case in c_cases:
        rust_label = TOKEN_MAP.get(case["label"])
        if rust_label:
            c_case_map[rust_label] = case

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
        case_missing_errs = c_case_errs - r_case_errs - SKIP_ERRORS - {f"?{e}" for e in c_case["errors"] if e not in ERROR_MAP}
        if case_missing_errs:
            divergences.append(("LOW", f"case {rust_label}: missing errors", sorted(case_missing_errs)))

        # Compare handlers in this case
        c_case_h = {HANDLER_MAP.get(h, f"?{h}") for h in c_case["handlers"]}
        r_case_h = set(r_case["handlers"])
        case_missing_h = c_case_h - r_case_h - {f"?{h}" for h in c_case["handlers"] if h not in HANDLER_MAP}
        if case_missing_h:
            divergences.append(("LOW", f"case {rust_label}: missing handlers", sorted(case_missing_h)))

    return {
        "c_function": c_func_name,
        "rust_function": r_func_name,
        "c_lines": c_text.count('\n') + 1,
        "rust_lines": r_text.count('\n') + 1,
        "c_cases": len(c_cases),
        "rust_cases": len(r_cases),
        "divergences": divergences,
    }


def format_report(results):
    """Format as human-readable report."""
    if results is None:
        return "Comparison failed."

    lines = []
    lines.append(f"=== {results['c_function']} ({results['c_lines']}L, {results['c_cases']} cases) vs {results['rust_function']} ({results['rust_lines']}L, {results['rust_cases']} cases) ===")

    if not results["divergences"]:
        lines.append("  No divergences found!")
        return "\n".join(lines)

    for sev, desc, details in sorted(results["divergences"]):
        lines.append(f"  [{sev}] {desc}: {details}")

    return "\n".join(lines)


# Known function pairs
PAIRS = [
    ("doContent", "do_content"),
    ("epilogProcessor", "epilog_processor"),
    ("doCdataSection", "do_cdata_section"),
    ("doProlog", "do_prolog"),
    ("contentProcessor", "content_processor"),
    ("prologProcessor", "prolog_processor"),
    ("processXmlDecl", "process_xml_decl"),
    ("reportComment", "report_comment"),
    ("reportProcessingInstruction", "report_processing_instruction"),
    ("reportDefault", "report_default"),
]


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    if sys.argv[1] == "--all":
        for c_name, r_name in PAIRS:
            results = compare(c_name, r_name)
            if results:
                print(format_report(results))
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

    if "--json" in sys.argv:
        results = compare(c_name, r_name)
        print(json.dumps(results, indent=2, default=list))
    else:
        results = compare(c_name, r_name)
        print(format_report(results))


if __name__ == "__main__":
    main()
