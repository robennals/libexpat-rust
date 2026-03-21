#!/usr/bin/env python3
"""
Call Tree Validator: Verify 1:1 correspondence between C and Rust function call trees.

For each C function in xmlparse.c, checks that the corresponding Rust function
in xmlparse.rs calls the same functions (translated to Rust naming conventions).

Usage:
    python3 scripts/validate-call-tree.py [--verbose] [--update-mapping]

Outputs:
    - Functions with matching call trees (OK)
    - Functions with divergent call trees (MISMATCH)
    - Functions missing from Rust (MISSING)
    - Override-documented divergences (OVERRIDE)
"""

import re
import sys
import json
import os
from pathlib import Path

# Root directory
ROOT = Path(__file__).parent.parent

# Source files
C_SOURCE = ROOT / "expat" / "lib" / "xmlparse.c"
RUST_SOURCE = ROOT / "expat-rust" / "src" / "xmlparse.rs"
MAPPING_FILE = ROOT / "scripts" / "call-tree-mapping.json"

# C-to-Rust function name mapping.
# Maps C function names to their Rust equivalents.
# Generated initially, then manually maintained.
C_TO_RUST_NAME = {
    # Public API
    "XML_ParserCreate": "Parser::new",
    "XML_ParserCreateNS": "Parser::new_ns",
    "XML_ParserReset": "Parser::reset",
    "XML_Parse": "Parser::parse",
    "XML_ParseBuffer": "Parser::parse_buffer",
    "XML_GetBuffer": "Parser::get_buffer",
    "XML_StopParser": "Parser::stop",
    "XML_ResumeParser": "Parser::resume",
    "XML_GetErrorCode": "Parser::error_code",
    "XML_GetCurrentLineNumber": "Parser::current_line_number",
    "XML_GetCurrentColumnNumber": "Parser::current_column_number",
    "XML_GetCurrentByteIndex": "Parser::current_byte_index",
    "XML_GetCurrentByteCount": "Parser::current_byte_count",
    "XML_GetParsingStatus": "Parser::parsing_status",
    "XML_SetHashSalt": "Parser::set_hash_salt",
    "XML_SetBase": "Parser::set_base",
    "XML_GetBase": "Parser::base",
    "XML_SetEncoding": "Parser::set_encoding",
    "XML_ErrorString": "error_string",
    "XML_ExpatVersion": "expat_version",
    "XML_ExpatVersionInfo": "expat_version_info",
    "XML_GetFeatureList": "get_feature_list",
    "XML_SetReparseDeferralEnabled": "Parser::set_reparse_deferral_enabled",
    "XML_ExternalEntityParserCreate": "Parser::create_external_entity_parser",
    "XML_FreeContentModel": "free_content_model",
    "XML_DefaultCurrent": "Parser::default_current",
    "XML_UseForeignDTD": "Parser::use_foreign_dtd",
    "XML_SetReturnNSTriplet": "Parser::set_return_ns_triplet",
    "XML_SetParamEntityParsing": "Parser::set_param_entity_parsing",
    "XML_UseParserAsHandlerArg": "Parser::use_parser_as_handler_arg",
    "XML_GetSpecifiedAttributeCount": "Parser::specified_attribute_count",
    "XML_GetIdAttributeIndex": "Parser::id_attribute_index",
    "XML_GetAttributeInfo": "Parser::attribute_info",

    # Handler setters (these are trivial, skip call-tree validation)
    # Listed for completeness

    # Internal functions - the important ones for call-tree matching
    "doContent": "Parser::do_content",
    "doProlog": "Parser::do_prolog",
    "doCdataSection": "Parser::do_cdata_section",
    "storeAtts": "Parser::store_atts",
    "storeAttributeValue": "Parser::store_attribute_value",
    "processXmlDecl": "Parser::process_xml_decl",
    "initializeEncoding": "Parser::initialize_encoding",
    "handleUnknownEncoding": "Parser::handle_unknown_encoding",
    "reportProcessingInstruction": "Parser::report_processing_instruction",
    "reportComment": "Parser::report_comment",
    "reportDefault": "Parser::report_default",
    "processEntity": "Parser::process_entity",
    "storeEntityValue": "Parser::store_entity_value",
    "normalizeLines": "normalize_lines",
    "normalizePublicId": "normalize_public_id",
    "setElementTypePrefix": "Parser::set_element_type_prefix",
    "getContext": "Parser::get_context",
    "setContext": "Parser::set_context",
    "parserInit": "Parser::parser_init",
    "startParsing": "Parser::start_parsing",
    "storeRawNames": "Parser::store_raw_names",
    "freeBindings": "Parser::free_bindings",
    "contentProcessor": "Parser::content_processor",
    "prologProcessor": "Parser::prolog_processor",
    "prologInitProcessor": "Parser::prolog_init_processor",
    "cdataSectionProcessor": "Parser::cdata_section_processor",
    "epilogProcessor": "Parser::epilog_processor",
    "errorProcessor": "Parser::error_processor",
    "externalEntityInitProcessor": "Parser::external_entity_init_processor",
    "externalEntityContentProcessor": "Parser::external_entity_content_processor",
    "internalEntityProcessor": "Parser::internal_entity_processor",

    # Tokenizer functions (mapped to xmltok_impl module)
    "XmlContentTok": "content_tok",
    "XmlPrologTok": "prolog_tok",
    "XmlCdataSectionTok": "cdata_section_tok",
    "XmlAttributeValueTok": "attribute_value_tok",
    "XmlEntityValueTok": "entity_value_tok",
    "XmlSameName": "name_matches_ascii",
    "XmlNameLength": "name_length",
    "XmlSkipS": "skip_s",
    "XmlGetAtts": "get_atts",
    "XmlCharRefNumber": "char_ref_number",
    "XmlPredefinedEntityName": "predefined_entity_name",
    "XmlUpdatePosition": "update_position",
    "XmlIsPublicId": "is_public_id",

    # Encoding functions
    "XmlInitEncoding": "detect_encoding_from_bom",
    "XmlInitUnknownEncoding": "select_encoding",
    "XmlParseXmlDecl": "parse_xml_decl",

    # Hash/string pool — Rust uses standard library equivalents
    "hash": "OVERRIDE:use_std_hash",
    "lookup": "OVERRIDE:use_std_hashmap",
    "poolInit": "OVERRIDE:use_std_vec",
    "poolClear": "OVERRIDE:use_std_vec",
    "poolDestroy": "OVERRIDE:use_std_drop",
    "poolCopyString": "OVERRIDE:use_std_string",
    "poolStoreString": "OVERRIDE:use_std_string",
    "poolAppend": "OVERRIDE:use_std_vec",
    "poolAppendString": "OVERRIDE:use_std_string",
    "poolGrow": "OVERRIDE:use_std_vec",
    "hashTableInit": "OVERRIDE:use_std_hashmap",
    "hashTableClear": "OVERRIDE:use_std_hashmap",
    "hashTableDestroy": "OVERRIDE:use_std_drop",
    "hashTableIterInit": "OVERRIDE:use_std_iter",
    "hashTableIterNext": "OVERRIDE:use_std_iter",

    # Memory management — Rust uses standard library
    "expat_malloc": "OVERRIDE:rust_allocator",
    "expat_free": "OVERRIDE:rust_allocator",
    "expat_realloc": "OVERRIDE:rust_allocator",
    "XML_MemMalloc": "OVERRIDE:rust_allocator",
    "XML_MemRealloc": "OVERRIDE:rust_allocator",
    "XML_MemFree": "OVERRIDE:rust_allocator",

    # DTD functions
    "dtdCreate": "OVERRIDE:rust_struct_init",
    "dtdReset": "OVERRIDE:rust_struct_init",
    "dtdDestroy": "OVERRIDE:rust_drop",
    "dtdCopy": "OVERRIDE:rust_clone",

    # Random/entropy — Rust uses standard library
    "generate_hash_secret_salt": "OVERRIDE:use_std_random",
    "writeRandomBytes_getrandom_nonblock": "OVERRIDE:use_std_random",
    "writeRandomBytes_dev_urandom": "OVERRIDE:use_std_random",
    "writeRandomBytes_arc4random": "OVERRIDE:use_std_random",
    "writeRandomBytes_rand_s": "OVERRIDE:use_std_random",
    "gather_time_entropy": "OVERRIDE:use_std_random",
}

# Standard divergence rules — a minimal set of categories where Rust
# intentionally diverges from the C call tree. These apply globally
# across ALL functions, so individual functions don't need per-call overrides.
#
# Each rule has:
#   - c_calls: C function names that are expected to be absent in Rust
#   - reason: Why this divergence is justified
STANDARD_DIVERGENCES = {
    "memory_management": {
        "c_calls": {"expat_malloc", "expat_free", "expat_realloc",
                     "XML_MemMalloc", "XML_MemRealloc", "XML_MemFree",
                     "MALLOC", "REALLOC", "FREE"},
        "reason": "Rust's ownership model handles allocation/deallocation via Vec, String, Box."
    },
    "string_pools": {
        "c_calls": {"poolInit", "poolClear", "poolDestroy", "poolCopyString",
                     "poolStoreString", "poolAppend", "poolAppendString", "poolGrow",
                     "poolCopyStringN", "poolCopyStringNoFinish", "poolBytesToAllocateFor",
                     "poolStart", "poolFinish", "poolDiscard", "poolLength",
                     "poolLastString", "poolChop"},
        "reason": "Rust uses String/Vec instead of C's arena-style string pool."
    },
    "hash_tables": {
        "c_calls": {"hash", "lookup", "keyeq", "keylen", "copy_salt_to_sipkey",
                     "hashTableInit", "hashTableClear", "hashTableDestroy",
                     "hashTableIterInit", "hashTableIterNext"},
        "reason": "Rust uses std::collections::HashMap with built-in SipHash."
    },
    "entropy": {
        "c_calls": {"generate_hash_secret_salt", "get_hash_secret_salt",
                     "writeRandomBytes_getrandom_nonblock", "writeRandomBytes_dev_urandom",
                     "writeRandomBytes_arc4random", "writeRandomBytes_rand_s",
                     "gather_time_entropy", "ENTROPY_DEBUG"},
        "reason": "Rust's HashMap has built-in randomized hashing for DoS protection."
    },
    "dtd_lifecycle": {
        "c_calls": {"dtdCreate", "dtdReset", "dtdDestroy", "dtdCopy"},
        "reason": "Rust uses struct Default/Clone/Drop traits for lifecycle management."
    },
    "error_handling_style": {
        "c_calls": {"parserBusy"},
        "reason": "Rust uses match on ParsingState enum instead of a separate check function."
    },
}

# Collect all C calls that are covered by standard divergences
DIVERGENT_C_CALLS = set()
for rule in STANDARD_DIVERGENCES.values():
    DIVERGENT_C_CALLS.update(rule["c_calls"])


def extract_c_functions(source_path):
    """Extract function definitions and their callees from C source."""
    with open(source_path, 'r') as f:
        content = f.read()

    functions = {}

    # Find function definitions (simplified — looks for patterns like "name(...) {")
    # This regex finds function definitions at the start of a line
    func_pattern = re.compile(
        r'^(?:static\s+)?(?:enum\s+XML_Error\s+|int\s+|void\s+|XML_Parser\s+|'
        r'const\s+XML_Char\s*\*\s*|XML_Bool\s+|XML_Char\s*\*\s*|'
        r'unsigned\s+long\s+|XML_Status\s+|Processor\s+|'
        r'XML_Parsing\s+|XML_Size\s+|XML_Index\s+|'
        r'const\s+XML_Feature\s*\*\s*|const\s+XML_LChar\s*\*\s*|'
        r'NAMED\s*\*\s*|ELEMENT_TYPE\s*\*\s*|PREFIX\s*\*\s*|'
        r'XML_Content\s*\*\s*|DTD\s*\*\s*|HASH_TABLE_ITER\s*\*?\s*|'
        r'HASH_TABLE\s*\*?\s*|STRING_POOL\s*\*?\s*)'
        r'(?:XMLCALL\s+|PTRCALL\s+|FASTCALL\s+)?'
        r'(\w+)\s*\([^)]*\)\s*\{',
        re.MULTILINE
    )

    for match in func_pattern.finditer(content):
        func_name = match.group(1)
        func_start = match.end()

        # Find the matching closing brace (simplified — count braces)
        depth = 1
        pos = func_start
        while pos < len(content) and depth > 0:
            if content[pos] == '{':
                depth += 1
            elif content[pos] == '}':
                depth -= 1
            pos += 1

        func_body = content[func_start:pos]

        # Extract function calls from the body
        # Match identifiers followed by '(' that aren't keywords
        call_pattern = re.compile(r'\b([a-zA-Z_]\w*)\s*\(')
        keywords = {'if', 'else', 'while', 'for', 'switch', 'case', 'return',
                     'sizeof', 'typeof', 'do', 'break', 'continue', 'goto',
                     'default', 'struct', 'enum', 'union', 'typedef'}

        callees = set()
        for call_match in call_pattern.finditer(func_body):
            callee = call_match.group(1)
            if callee not in keywords and not callee.startswith('__'):
                callees.add(callee)

        functions[func_name] = sorted(callees)

    return functions


def extract_rust_functions(source_path):
    """Extract function definitions and their callees from Rust source."""
    with open(source_path, 'r') as f:
        content = f.read()

    functions = {}

    # Find function definitions
    func_pattern = re.compile(
        r'(?:pub\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\([^)]*\)',
        re.MULTILINE
    )

    for match in func_pattern.finditer(content):
        func_name = match.group(1)
        func_start = content.find('{', match.end())
        if func_start == -1:
            continue

        # Find matching closing brace
        depth = 1
        pos = func_start + 1
        while pos < len(content) and depth > 0:
            if content[pos] == '{':
                depth += 1
            elif content[pos] == '}':
                depth -= 1
            pos += 1

        func_body = content[func_start:pos]

        # Extract function/method calls
        # Match: identifier( or self.identifier( or module::identifier(
        call_pattern = re.compile(r'(?:self\.|\w+::)?(\w+)\s*[(<]')
        keywords = {'if', 'else', 'while', 'for', 'match', 'return', 'let', 'mut',
                     'fn', 'pub', 'struct', 'enum', 'impl', 'use', 'mod', 'type',
                     'where', 'trait', 'loop', 'break', 'continue', 'as', 'Some',
                     'None', 'Ok', 'Err', 'Box', 'Vec', 'String', 'new', 'unwrap',
                     'expect', 'clone', 'to_string', 'push', 'pop', 'len', 'is_empty',
                     'extend_from_slice', 'clear', 'into', 'from', 'map', 'and_then',
                     'unwrap_or', 'unwrap_or_default', 'as_str', 'as_deref',
                     'saturating_sub', 'contains', 'trim', 'trim_start', 'starts_with',
                     'find', 'parse', 'encode_utf8', 'from_u32', 'from_utf8',
                     'from_str_radix', 'to_uppercase', 'eq_ignore_ascii_case',
                     'matches', 'take', 'iter', 'collect', 'to_vec', 'is_ascii_whitespace'}

        callees = set()
        for call_match in call_pattern.finditer(func_body):
            callee = call_match.group(1)
            if callee not in keywords:
                callees.add(callee)

        functions[func_name] = sorted(callees)

    return functions


def build_reverse_mapping():
    """Build Rust-name-to-C-name reverse mapping."""
    reverse = {}
    for c_name, rust_name in C_TO_RUST_NAME.items():
        if not rust_name.startswith("OVERRIDE:"):
            # Strip "Parser::" prefix for matching
            simple_name = rust_name.split("::")[-1] if "::" in rust_name else rust_name
            reverse[simple_name] = c_name
    return reverse


def validate_call_tree(c_functions, rust_functions, verbose=False):
    """Compare call trees between C and Rust."""
    results = {
        'ok': [],
        'mismatch': [],
        'missing_rust': [],
        'missing_c': [],
        'overridden': [],
        'trivial': [],  # setter functions etc.
    }

    # Build mappings
    rust_to_c = build_reverse_mapping()
    c_to_rust_simple = {}
    for c_name, rust_name in C_TO_RUST_NAME.items():
        if not rust_name.startswith("OVERRIDE:"):
            simple = rust_name.split("::")[-1] if "::" in rust_name else rust_name
            c_to_rust_simple[c_name] = simple

    # Check each C function that has a Rust mapping
    for c_name, rust_name in C_TO_RUST_NAME.items():
        if rust_name.startswith("OVERRIDE:"):
            results['overridden'].append({
                'c_name': c_name,
                'override': rust_name,
            })
            continue

        simple_rust = rust_name.split("::")[-1] if "::" in rust_name else rust_name

        if c_name not in c_functions:
            continue  # C function not found (might be in another file)

        if simple_rust not in rust_functions:
            results['missing_rust'].append({
                'c_name': c_name,
                'expected_rust': rust_name,
            })
            continue

        # Compare call trees
        c_callees = set(c_functions[c_name])
        rust_callees = set(rust_functions[simple_rust])

        # Translate C callee names to Rust names for comparison
        translated_c_callees = set()
        untranslatable = set()
        for callee in c_callees:
            # Skip calls covered by standard divergences
            if callee in DIVERGENT_C_CALLS:
                continue
            if callee in c_to_rust_simple:
                translated_c_callees.add(c_to_rust_simple[callee])
            elif callee in C_TO_RUST_NAME and C_TO_RUST_NAME[callee].startswith("OVERRIDE:"):
                continue  # Overridden, skip
            else:
                untranslatable.add(callee)

        # Find differences
        missing_in_rust = translated_c_callees - rust_callees
        extra_in_rust = rust_callees - translated_c_callees

        if not missing_in_rust and not extra_in_rust:
            results['ok'].append({
                'c_name': c_name,
                'rust_name': rust_name,
                'callees': len(c_callees),
            })
        else:
            results['mismatch'].append({
                'c_name': c_name,
                'rust_name': rust_name,
                'missing_in_rust': sorted(missing_in_rust),
                'extra_in_rust': sorted(extra_in_rust),
                'untranslatable': sorted(untranslatable),
            })

    return results


def print_results(results, verbose=False):
    """Print validation results."""
    total = (len(results['ok']) + len(results['mismatch']) +
             len(results['missing_rust']) + len(results['overridden']))

    print(f"\n{'='*60}")
    print(f"Call Tree Validation Report")
    print(f"{'='*60}\n")

    # Summary
    print(f"Total mapped functions: {total}")
    print(f"  OK (matching):     {len(results['ok'])}")
    print(f"  MISMATCH:          {len(results['mismatch'])}")
    print(f"  MISSING in Rust:   {len(results['missing_rust'])}")
    print(f"  OVERRIDDEN:        {len(results['overridden'])}")
    print()

    # Mismatches (most important)
    if results['mismatch']:
        print(f"\n--- MISMATCHES ---\n")
        for m in results['mismatch']:
            print(f"  {m['c_name']} -> {m['rust_name']}")
            if m['missing_in_rust']:
                print(f"    Missing calls: {', '.join(m['missing_in_rust'])}")
            if m['extra_in_rust']:
                print(f"    Extra calls:   {', '.join(m['extra_in_rust'])}")
            if m['untranslatable'] and verbose:
                print(f"    Unmapped C calls: {', '.join(m['untranslatable'])}")
            print()

    # Missing
    if results['missing_rust']:
        print(f"\n--- MISSING IN RUST ---\n")
        for m in results['missing_rust']:
            print(f"  {m['c_name']} -> {m['expected_rust']} (not found)")

    # Overrides
    if verbose and results['overridden']:
        print(f"\n--- OVERRIDDEN (documented divergences) ---\n")
        for m in results['overridden']:
            print(f"  {m['c_name']} -> {m['override']}")

    # OK
    if verbose and results['ok']:
        print(f"\n--- OK (matching call trees) ---\n")
        for m in results['ok']:
            print(f"  {m['c_name']} -> {m['rust_name']} ({m['callees']} callees)")

    print(f"\n{'='*60}")

    # Return exit code
    if results['mismatch'] or results['missing_rust']:
        return 1
    return 0


def main():
    verbose = '--verbose' in sys.argv or '-v' in sys.argv

    if not C_SOURCE.exists():
        print(f"Error: C source not found: {C_SOURCE}")
        sys.exit(1)
    if not RUST_SOURCE.exists():
        print(f"Error: Rust source not found: {RUST_SOURCE}")
        sys.exit(1)

    print(f"Extracting C functions from {C_SOURCE}...")
    c_functions = extract_c_functions(C_SOURCE)
    print(f"  Found {len(c_functions)} functions")

    print(f"Extracting Rust functions from {RUST_SOURCE}...")
    rust_functions = extract_rust_functions(RUST_SOURCE)
    print(f"  Found {len(rust_functions)} functions")

    results = validate_call_tree(c_functions, rust_functions, verbose)
    exit_code = print_results(results, verbose)

    # Also save divergence documentation
    override_doc = ROOT / "plans" / "call-tree-overrides.md"
    with open(override_doc, 'w') as f:
        f.write("# Standard Call Tree Divergences\n\n")
        f.write("These are the allowed categories where Rust intentionally diverges from\n")
        f.write("the C call tree. Each is a global rule that applies to ALL functions.\n\n")
        for name, info in STANDARD_DIVERGENCES.items():
            f.write(f"## {name}\n\n")
            f.write(f"**C calls omitted:** {', '.join(sorted(info['c_calls']))}\n\n")
            f.write(f"**Reason:** {info['reason']}\n\n")

    sys.exit(exit_code)


if __name__ == '__main__':
    main()
