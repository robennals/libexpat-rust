#!/usr/bin/env python3
"""Deep structural comparison of C and Rust function implementations.

Given a C function and its Rust counterpart, produces a detailed diff of:
- Switch/match case coverage (which tokens/roles are handled)
- Error codes returned
- Handler calls made
- Function calls made
- Control flow patterns (loops, early returns)

Output is machine-readable and suitable for feeding to an AI agent.

Usage:
    python3 scripts/compare-functions.py <c_func> <rust_func> [config.json]
    python3 scripts/compare-functions.py doContent do_content
    python3 scripts/compare-functions.py --all config.json
"""

import re
import sys
import os
import json

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")


# ========= Source extraction =========

def read(path):
    with open(path) as f:
        return f.read()


def extract_function(src, name, lang):
    """Extract a single function body from source."""
    if lang == "c":
        pattern = r'^(?:static\s+)?(?:\w+\s+)+?(?:PTRCALL\s+)?' + re.escape(name) + r'\s*\([^)]*\)\s*\{'
    else:
        pattern = r'(?:pub\s+)?fn\s+' + re.escape(name) + r'\s*(?:<[^>]*>)?\s*\([^)]*\)'

    m = re.search(pattern, src, re.MULTILINE)
    if not m:
        return None

    if lang == "rust":
        brace_pos = src.find('{', m.end())
        if brace_pos == -1:
            return None
        start_idx = m.start()
    else:
        brace_pos = m.end() - 1
        start_idx = m.start()

    depth = 0
    i = brace_pos
    while i < len(src):
        if src[i] == '{': depth += 1
        elif src[i] == '}': depth -= 1
        if depth == 0:
            return src[start_idx:i+1]
        i += 1
    return None


# ========= Pattern extraction =========

def extract_switch_cases(body, lang):
    """Extract all switch/match case labels."""
    if lang == "c":
        return re.findall(r'case\s+([\w_]+)\s*:', body)
    else:
        # Rust match arms: XmlTok::Name | XmlTok::Other =>
        arms = re.findall(r'(\w+::\w+)\s*(?:\|[^=]*)?=>', body)
        # Also catch combined patterns
        combined = re.findall(r'(\w+::\w+)\s*\|', body)
        return list(set(arms + combined))


def extract_errors(body, lang):
    """Extract error codes referenced."""
    if lang == "c":
        return set(re.findall(r'XML_ERROR_(\w+)', body))
    else:
        return set(re.findall(r'XmlError::(\w+)', body))


def extract_handler_calls(body, lang):
    """Extract handler invocations."""
    if lang == "c":
        return set(re.findall(r'parser->m_(\w+Handler)', body))
    else:
        return set(re.findall(r'self\.(\w+_handler)', body))


def extract_function_calls(body, lang, known_funcs=None):
    """Extract function calls made."""
    if lang == "c":
        calls = set(re.findall(r'\b(\w+)\s*\(', body))
        # Filter to known functions if provided
        skip = {'if', 'while', 'for', 'switch', 'return', 'sizeof', 'memcpy',
                'memset', 'assert', 'NULL'}
        return calls - skip
    else:
        calls = set(re.findall(r'(?:self\.|Self::)?(\w+)\s*\(', body))
        skip = {'match', 'if', 'Some', 'None', 'Ok', 'Err', 'unwrap', 'unwrap_or',
                'as_bytes', 'to_string', 'as_str', 'len', 'push', 'pop', 'is_empty',
                'from_utf8', 'extend_from_slice', 'collect', 'iter', 'map', 'encode_utf8',
                'from_u32', 'get', 'contains', 'saturating_sub', 'to_vec', 'clone',
                'matches', 'to_uppercase'}
        return calls - skip


def extract_field_accesses(body, lang):
    """Extract struct field reads/writes."""
    if lang == "c":
        return set(re.findall(r'parser->m_(\w+)', body))
    else:
        return set(re.findall(r'self\.(\w+)', body))


# ========= Mapping tables =========

def load_mappings():
    """Load C→Rust name mappings."""
    error_map = {
        "NONE": "None", "NO_MEMORY": "NoMemory", "SYNTAX": "Syntax",
        "NO_ELEMENTS": "NoElements", "INVALID_TOKEN": "InvalidToken",
        "UNCLOSED_TOKEN": "UnclosedToken", "PARTIAL_CHAR": "PartialChar",
        "TAG_MISMATCH": "TagMismatch", "DUPLICATE_ATTRIBUTE": "DuplicateAttribute",
        "JUNK_AFTER_DOC_ELEMENT": "JunkAfterDocElement",
        "UNDEFINED_ENTITY": "UndefinedEntity", "NOT_STANDALONE": "NotStandalone",
        "EXTERNAL_ENTITY_HANDLING": "ExternalEntityHandling",
        "MISPLACED_XML_PI": "MisplacedXmlPi",
        "UNKNOWN_ENCODING": "UnknownEncoding", "INCORRECT_ENCODING": "IncorrectEncoding",
        "BAD_CHAR_REF": "BadCharRef", "XML_DECL": "XmlDecl",
        "ABORTED": "Aborted", "FINISHED": "Finished",
        "SUSPENDED": "Suspended", "ASYNC_ENTITY": "AsyncEntity",
        "RECURSIVE_ENTITY_REF": "RecursiveEntityRef",
        "BINARY_ENTITY_REF": "BinaryEntityRef",
        "ATTRIBUTE_EXTERNAL_ENTITY_REF": "AttributeExternalEntityRef",
        "TEXT_DECL": "TextDecl",
        "UNEXPECTED_STATE": "UnexpectedState",
        "AMPLIFICATION_LIMIT_BREACH": "AmplificationLimitBreach",
        "ENTITY_DECLARED_IN_PE": "EntityDeclaredInPe",
        "UNCLOSED_CDATA_SECTION": "UnclosedCdataSection",
        "PARAM_ENTITY_REF": "ParamEntityRef",
    }

    handler_map = {
        "startElementHandler": "start_element_handler",
        "endElementHandler": "end_element_handler",
        "characterDataHandler": "character_data_handler",
        "processingInstructionHandler": "processing_instruction_handler",
        "commentHandler": "comment_handler",
        "startCdataSectionHandler": "start_cdata_section_handler",
        "endCdataSectionHandler": "end_cdata_section_handler",
        "defaultHandler": "default_handler",
        "startDoctypeDeclHandler": "start_doctype_decl_handler",
        "endDoctypeDeclHandler": "end_doctype_decl_handler",
        "xmlDeclHandler": "xml_decl_handler",
        "entityDeclHandler": "entity_decl_handler",
        "unparsedEntityDeclHandler": "unparsed_entity_decl_handler",
        "notationDeclHandler": "notation_decl_handler",
        "startNamespaceDeclHandler": "start_namespace_decl_handler",
        "endNamespaceDeclHandler": "end_namespace_decl_handler",
        "notStandaloneHandler": "not_standalone_handler",
        "externalEntityRefHandler": "external_entity_ref_handler",
        "skippedEntityHandler": "skipped_entity_handler",
        "unknownEncodingHandler": "unknown_encoding_handler",
        "attlistDeclHandler": "attlist_decl_handler",
        "elementDeclHandler": "element_decl_handler",
    }

    token_map = {
        "XML_TOK_NONE": "XmlTok::None",
        "XML_TOK_INVALID": "XmlTok::Invalid",
        "XML_TOK_PARTIAL": "XmlTok::Partial",
        "XML_TOK_PARTIAL_CHAR": "XmlTok::PartialChar",
        "XML_TOK_TRAILING_CR": "XmlTok::TrailingCr",
        "XML_TOK_ENTITY_REF": "XmlTok::EntityRef",
        "XML_TOK_START_TAG_NO_ATTS": "XmlTok::StartTagNoAtts",
        "XML_TOK_START_TAG_WITH_ATTS": "XmlTok::StartTagWithAtts",
        "XML_TOK_EMPTY_ELEMENT_NO_ATTS": "XmlTok::EmptyElementNoAtts",
        "XML_TOK_EMPTY_ELEMENT_WITH_ATTS": "XmlTok::EmptyElementWithAtts",
        "XML_TOK_END_TAG": "XmlTok::EndTag",
        "XML_TOK_CHAR_REF": "XmlTok::CharRef",
        "XML_TOK_XML_DECL": "XmlTok::XmlDecl",
        "XML_TOK_DATA_NEWLINE": "XmlTok::DataNewline",
        "XML_TOK_CDATA_SECT_OPEN": "XmlTok::CdataSectOpen",
        "XML_TOK_TRAILING_RSQB": "XmlTok::TrailingRsqb",
        "XML_TOK_DATA_CHARS": "XmlTok::DataChars",
        "XML_TOK_PI": "XmlTok::Pi",
        "XML_TOK_COMMENT": "XmlTok::Comment",
        "XML_TOK_PROLOG_S": "XmlTok::PrologS",
        "XML_TOK_BOM": "XmlTok::Bom",
        "XML_TOK_DECL_OPEN": "XmlTok::DeclOpen",
        "XML_TOK_DECL_CLOSE": "XmlTok::DeclClose",
        "XML_TOK_INSTANCE_START": "XmlTok::InstanceStart",
        "XML_TOK_NAME": "XmlTok::Name",
    }

    return error_map, handler_map, token_map


# ========= Comparison =========

def compare_functions(c_body, r_body, c_name, r_name):
    """Compare C and Rust function implementations."""
    error_map, handler_map, token_map = load_mappings()

    results = {
        "c_function": c_name,
        "rust_function": r_name,
        "c_lines": c_body.count('\n') + 1,
        "rust_lines": r_body.count('\n') + 1,
        "divergences": [],
    }

    # 1. Switch/match case coverage
    c_cases = extract_switch_cases(c_body, "c")
    r_cases = extract_switch_cases(r_body, "rust")

    c_tokens = set(c_cases)
    r_tokens = set(r_cases)

    # Map C tokens to Rust
    c_mapped_tokens = {}
    for ct in c_tokens:
        rt = token_map.get(ct, ct)
        c_mapped_tokens[ct] = rt

    missing_cases = []
    for ct, rt in c_mapped_tokens.items():
        if rt not in r_tokens:
            missing_cases.append(f"{ct} -> {rt}")

    if missing_cases:
        results["divergences"].append({
            "type": "missing_cases",
            "severity": "HIGH",
            "details": missing_cases,
            "fix": f"Add match arms for: {', '.join(missing_cases)}",
        })

    # 2. Error codes
    c_errors = extract_errors(c_body, "c")
    r_errors = extract_errors(r_body, "rust")

    skip_errors = {"NoMemory", "AmplificationLimitBreach"}
    c_mapped_errs = {error_map.get(e, f"UNMAPPED:{e}") for e in c_errors}
    unmapped = {f"UNMAPPED:{e}" for e in c_errors if e not in error_map}

    missing_errs = sorted(c_mapped_errs - r_errors - skip_errors - unmapped)
    extra_errs = sorted(r_errors - c_mapped_errs - {"UnexpectedState"})

    if missing_errs:
        results["divergences"].append({
            "type": "missing_errors",
            "severity": "MEDIUM",
            "details": missing_errs,
            "fix": f"Add error handling for: {', '.join(missing_errs)}",
        })

    if extra_errs:
        results["divergences"].append({
            "type": "extra_errors",
            "severity": "LOW",
            "details": extra_errs,
            "fix": f"Verify these extra error codes are correct: {', '.join(extra_errs)}",
        })

    # 3. Handler calls
    c_handlers = extract_handler_calls(c_body, "c")
    r_handlers = extract_handler_calls(r_body, "rust")

    c_mapped_hdlrs = {handler_map.get(h, f"UNMAPPED:{h}") for h in c_handlers}
    unmapped_h = {f"UNMAPPED:{h}" for h in c_handlers if h not in handler_map}

    missing_hdlrs = sorted(c_mapped_hdlrs - r_handlers - unmapped_h)
    if missing_hdlrs:
        results["divergences"].append({
            "type": "missing_handlers",
            "severity": "MEDIUM",
            "details": missing_hdlrs,
            "fix": f"Add handler calls for: {', '.join(missing_hdlrs)}",
        })

    # 4. Key function calls
    c_calls = extract_function_calls(c_body, "c")
    r_calls = extract_function_calls(r_body, "rust")

    # Map known C functions to Rust
    key_c_funcs = {
        'storeAtts': 'extract_attrs',
        'reportDefault': 'report_default',
        'reportComment': 'report_comment',
        'reportProcessingInstruction': 'report_processing_instruction',
        'freeBindings': 'free_bindings',
        'processEntity': 'process_entity',
        'doCdataSection': 'do_cdata_section',
        'epilogProcessor': 'epilog_processor',
        'accountingDiffTolerated': None,  # skip
        'accountingOnAbort': None,  # skip
        'poolClear': None,  # skip
    }

    missing_calls = []
    for c_fn, r_fn in key_c_funcs.items():
        if r_fn is None:
            continue
        if c_fn in c_calls and r_fn not in r_calls:
            missing_calls.append(f"{c_fn} -> {r_fn}")

    if missing_calls:
        results["divergences"].append({
            "type": "missing_function_calls",
            "severity": "MEDIUM",
            "details": missing_calls,
            "fix": f"Add function calls: {', '.join(missing_calls)}",
        })

    return results


def format_report(results):
    """Format comparison results as human-readable report."""
    lines = []
    lines.append(f"=== Structural Comparison: {results['c_function']} ({results['c_lines']} lines) vs {results['rust_function']} ({results['rust_lines']} lines) ===")
    lines.append("")

    if not results["divergences"]:
        lines.append("No structural divergences found!")
        return "\n".join(lines)

    lines.append(f"Divergences: {len(results['divergences'])}")
    lines.append("")

    for d in sorted(results["divergences"], key=lambda x: {"HIGH": 0, "MEDIUM": 1, "LOW": 2}[x["severity"]]):
        lines.append(f"[{d['severity']}] {d['type']}")
        if isinstance(d["details"], list):
            for item in d["details"]:
                lines.append(f"  - {item}")
        else:
            lines.append(f"  {d['details']}")
        lines.append(f"  FIX: {d['fix']}")
        lines.append("")

    return "\n".join(lines)


def main():
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(1)

    if sys.argv[1] == "--all":
        # Compare all pairs from config
        config_path = sys.argv[2]
        with open(config_path) as f:
            config = json.load(f)

        config_dir = os.path.dirname(os.path.abspath(config_path))
        c_path = os.path.join(config_dir, config["c_file"]) if not os.path.isabs(config["c_file"]) else config["c_file"]
        r_path = os.path.join(config_dir, config["rust_file"]) if not os.path.isabs(config["rust_file"]) else config["rust_file"]

        c_src = read(c_path)
        r_src = read(r_path)

        for pair in config.get("function_pairs", []):
            c_body = extract_function(c_src, pair["c"], "c")
            r_body = extract_function(r_src, pair["rust"], "rust")
            if c_body and r_body:
                results = compare_functions(c_body, r_body, pair["c"], pair["rust"])
                print(format_report(results))
                print()
        return

    c_name = sys.argv[1]
    r_name = sys.argv[2]

    c_file = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
    r_file = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")

    c_src = read(c_file)
    r_src = read(r_file)

    c_body = extract_function(c_src, c_name, "c")
    r_body = extract_function(r_src, r_name, "rust")

    if not c_body:
        print(f"Error: C function '{c_name}' not found", file=sys.stderr)
        sys.exit(1)
    if not r_body:
        print(f"Error: Rust function '{r_name}' not found", file=sys.stderr)
        sys.exit(1)

    if "--json" in sys.argv:
        results = compare_functions(c_body, r_body, c_name, r_name)
        print(json.dumps(results, indent=2))
    else:
        results = compare_functions(c_body, r_body, c_name, r_name)
        print(format_report(results))


if __name__ == "__main__":
    main()
