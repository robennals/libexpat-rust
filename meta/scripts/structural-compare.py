#!/usr/bin/env python3
"""Structural comparison: verify Rust code structurally matches C code.

Compares key structural features between C xmlparse.c and Rust xmlparse.rs.
Outputs actionable divergences that can be fed to a haiku agent for fixing.

Usage:
    python3 scripts/structural-compare.py              # Full report
    python3 scripts/structural-compare.py --actionable # Just actionable items for agents

Checks:
1. Function correspondence — do all C functions have Rust equivalents?
2. Error code coverage — does each Rust function return the same errors as C?
3. Handler call coverage — does each Rust function invoke the same handlers?
4. Feature completeness — are major C features present in Rust?
"""

import re
import sys
import os

ROOT = os.path.join(os.path.dirname(__file__), "..")
C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")


def read(path):
    with open(path) as f:
        return f.read()


# ============================================================
# C extraction
# ============================================================

def extract_c_functions(src):
    """Extract function names and their bodies from C source."""
    funcs = {}
    # Match function definitions (not declarations)
    for m in re.finditer(r'^(?:static\s+)?(?:\w+\s+)+?(?:PTRCALL\s+)?(\w+)\s*\([^)]*\)\s*\{', src, re.MULTILINE):
        name = m.group(1)
        start = m.start()
        # Find matching closing brace
        depth = 0
        i = m.end() - 1  # at the opening brace
        while i < len(src):
            if src[i] == '{': depth += 1
            elif src[i] == '}': depth -= 1
            if depth == 0:
                funcs[name] = src[start:i+1]
                break
            i += 1
    return funcs


def extract_c_errors(body):
    """Extract XML_ERROR_* codes referenced in a function body."""
    return set(re.findall(r'XML_ERROR_(\w+)', body))


def extract_c_handlers(body):
    """Extract handler calls in a function body."""
    return set(re.findall(r'parser->m_(\w+Handler)', body))


# ============================================================
# Rust extraction
# ============================================================

def extract_rust_functions(src):
    """Extract function names and their bodies from Rust source."""
    funcs = {}
    for m in re.finditer(r'(?:pub\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\([^)]*\)', src):
        name = m.group(1)
        start = m.start()
        # Find the opening brace
        brace_pos = src.find('{', m.end())
        if brace_pos == -1:
            continue
        # Find matching closing brace
        depth = 0
        i = brace_pos
        while i < len(src):
            if src[i] == '{': depth += 1
            elif src[i] == '}': depth -= 1
            if depth == 0:
                funcs[name] = src[start:i+1]
                break
            i += 1
    return funcs


def extract_rust_errors(body):
    """Extract XmlError:: variants referenced in a function body."""
    return set(re.findall(r'XmlError::(\w+)', body))


def extract_rust_handlers(body):
    """Extract handler field accesses in a function body."""
    return set(re.findall(r'self\.(\w+_handler)', body))


# ============================================================
# Mapping tables
# ============================================================

C_TO_RUST_ERROR = {
    "NONE": "None", "NO_MEMORY": "NoMemory", "SYNTAX": "Syntax",
    "NO_ELEMENTS": "NoElements", "INVALID_TOKEN": "InvalidToken",
    "UNCLOSED_TOKEN": "UnclosedToken", "PARTIAL_CHAR": "PartialChar",
    "TAG_MISMATCH": "TagMismatch", "DUPLICATE_ATTRIBUTE": "DuplicateAttribute",
    "JUNK_AFTER_DOC_ELEMENT": "JunkAfterDocElement",
    "UNDEFINED_ENTITY": "UndefinedEntity", "NOT_STANDALONE": "NotStandalone",
    "EXTERNAL_ENTITY_HANDLING": "ExternalEntityHandling",
    "PUBLICID": "Publicid", "MISPLACED_XML_PI": "MisplacedXmlPi",
    "UNKNOWN_ENCODING": "UnknownEncoding", "INCORRECT_ENCODING": "IncorrectEncoding",
    "BAD_CHAR_REF": "BadCharRef", "XML_DECL": "XmlDecl",
    "ABORTED": "Aborted", "FINISHED": "Finished",
    "NOT_SUSPENDED": "NotSuspended", "SUSPENDED": "Suspended",
    "NOT_STARTED": "NotStarted", "UNCLOSED_CDATA_SECTION": "UnclosedCdataSection",
    "PARAM_ENTITY_REF": "ParamEntityRef", "ASYNC_ENTITY": "AsyncEntity",
    "RECURSIVE_ENTITY_REF": "RecursiveEntityRef",
    "BINARY_ENTITY_REF": "BinaryEntityRef",
    "ATTRIBUTE_EXTERNAL_ENTITY_REF": "AttributeExternalEntityRef",
    "TEXT_DECL": "TextDecl", "INCOMPLETE_PE": "IncompletePe",
    "UNBOUND_PREFIX": "UnboundPrefix", "UNDECLARING_PREFIX": "UndeclaringPrefix",
    "RESERVED_PREFIX_XML": "ReservedPrefixXml",
    "RESERVED_PREFIX_XMLNS": "ReservedPrefixXmlns",
    "RESERVED_NAMESPACE_URI": "ReservedNamespaceUri",
    "AMPLIFICATION_LIMIT_BREACH": "AmplificationLimitBreach",
    "ENTITY_DECLARED_IN_PE": "EntityDeclaredInPe",
    "SUSPEND_PE": "SuspendPe",
    "FEATURE_REQUIRES_XML_DTD": "FeatureRequiresXmlDtd",
    "UNEXPECTED_STATE": "UnexpectedState",
}

C_TO_RUST_HANDLER = {
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
    "elementDeclHandler": "element_decl_handler",
    "attlistDeclHandler": "attlist_decl_handler",
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
}

# Key C→Rust function pairs to compare structurally
# Format: (c_func, rust_func, description)
KEY_PAIRS = [
    ("doContent", "do_content", "main content parser loop"),
    ("epilogProcessor", "epilog_processor", "epilog processing"),
    ("contentProcessor", "content_processor", "content processor entry"),
    # Note: doProlog doesn't have a Rust equivalent because Rust uses scan_buffer/scan_prolog
    # That IS the structural divergence — Rust doesn't use xmlrole at all
]

# Errors that are OK to be missing (require features not yet ported)
ACCEPTABLE_MISSING_ERRORS = {
    "NoMemory",  # Rust doesn't OOM the same way
    "AmplificationLimitBreach",  # Billion laughs not yet ported
    "EntityDeclaredInPe",  # Parameter entity feature
    "SuspendPe",  # Parameter entity feature
}


# ============================================================
# Feature checks (structural, not just string matching)
# ============================================================

def check_namespace_support(rust_src):
    """Check if Rust code has namespace processing in start tag handling."""
    issues = []
    if "xmlns" not in rust_src:
        issues.append("No xmlns attribute detection in Rust code")
    if "UnboundPrefix" not in rust_src:
        issues.append("No UnboundPrefix error handling")
    if "namespace_decl_handler" not in rust_src or "start_namespace_decl_handler" not in rust_src:
        issues.append("Namespace declaration handlers not invoked during parsing")
    # Check if scan_start_tag handles namespace bindings
    rust_funcs = extract_rust_functions(rust_src)
    if "scan_start_tag" in rust_funcs:
        sst = rust_funcs["scan_start_tag"]
        if "xmlns" not in sst and "namespace" not in sst:
            issues.append("scan_start_tag does not process namespace bindings (C storeAtts does)")
    return issues


def check_entity_resolution(rust_src):
    """Check if Rust code resolves general entities from DTD."""
    issues = []
    rust_funcs = extract_rust_functions(rust_src)
    if "resolve_reference" in rust_funcs:
        rr = rust_funcs["resolve_reference"]
        if "general_entities" not in rr and "dtd" not in rr:
            issues.append("resolve_reference doesn't look up entities in DTD (only handles predefined)")
    return issues


def check_stop_resume(rust_src):
    """Check if stop/resume integrates with parse loop."""
    issues = []
    rust_funcs = extract_rust_functions(rust_src)
    # C's callProcessor checks m_parsingStatus.parsing for SUSPENDED/FINISHED
    if "parse" in rust_funcs:
        parse_body = rust_funcs["parse"]
        if "Suspended" not in parse_body and "suspended" not in parse_body.lower():
            issues.append("parse() doesn't check for suspended state before running processor")
    if "run_processor" in rust_funcs:
        rp = rust_funcs["run_processor"]
        if "Suspended" not in rp and "Aborted" not in rp:
            issues.append("run_processor doesn't check parsing state (C callProcessor does)")
    # Check if handlers can actually stop the parser
    if "do_content" in rust_funcs:
        dc = rust_funcs["do_content"]
        if "Suspended" not in dc and "Aborted" not in dc:
            issues.append("do_content doesn't check for suspend/abort after handler calls")
    return issues


def check_utf16(rust_src):
    """Check UTF-16 support completeness."""
    issues = []
    if "transcode_utf16" in rust_src:
        # Has basic transcoding, check if it's integrated into incremental parsing
        if "detect_and_transcode" not in rust_src:
            issues.append("UTF-16 transcoding exists but may not be integrated")
    else:
        issues.append("No UTF-16 transcoding function found")
    return issues


# ============================================================
# Main report
# ============================================================

def main():
    actionable_only = "--actionable" in sys.argv

    c_src = read(C_FILE)
    rust_src = read(RUST_FILE)

    c_funcs = extract_c_functions(c_src)
    rust_funcs = extract_rust_functions(rust_src)

    divergences = []

    # ---- Error code comparison ----
    for c_func, rust_func, desc in KEY_PAIRS:
        if c_func not in c_funcs:
            continue
        if rust_func not in rust_funcs:
            divergences.append({
                "type": "missing_function",
                "severity": "HIGH",
                "message": f"Rust function '{rust_func}' not found (C has '{c_func}': {desc})",
                "fix": f"Port C function {c_func} to Rust as {rust_func}",
            })
            continue

        c_errs = extract_c_errors(c_funcs[c_func])
        r_errs = extract_rust_errors(rust_funcs[rust_func])
        c_mapped = {C_TO_RUST_ERROR.get(e, f"UNMAPPED:{e}") for e in c_errs}

        missing = c_mapped - r_errs - ACCEPTABLE_MISSING_ERRORS - {f"UNMAPPED:{e}" for e in c_errs if e not in C_TO_RUST_ERROR}
        if missing:
            divergences.append({
                "type": "missing_errors",
                "severity": "MEDIUM",
                "message": f"{rust_func} missing error codes vs C {c_func}: {sorted(missing)}",
                "fix": f"Add error handling for {sorted(missing)} in {rust_func}, matching C {c_func} behavior",
            })

    # ---- Handler comparison ----
    for c_func, rust_func, desc in KEY_PAIRS:
        if c_func not in c_funcs or rust_func not in rust_funcs:
            continue
        c_hdlrs = extract_c_handlers(c_funcs[c_func])
        r_hdlrs = extract_rust_handlers(rust_funcs[rust_func])
        c_mapped = {C_TO_RUST_HANDLER.get(h, f"UNMAPPED:{h}") for h in c_hdlrs}

        missing = c_mapped - r_hdlrs - {f"UNMAPPED:{h}" for h in c_hdlrs if h not in C_TO_RUST_HANDLER}
        if missing:
            divergences.append({
                "type": "missing_handlers",
                "severity": "MEDIUM",
                "message": f"{rust_func} doesn't call handlers that C {c_func} calls: {sorted(missing)}",
                "fix": f"Add handler invocations for {sorted(missing)} in {rust_func}",
            })

    # ---- Feature checks ----
    ns_issues = check_namespace_support(rust_src)
    for issue in ns_issues:
        divergences.append({
            "type": "missing_feature",
            "severity": "HIGH",
            "feature": "namespace_processing",
            "message": issue,
            "fix": f"Implement namespace processing: {issue}",
            "tests_blocked": 11,
        })

    entity_issues = check_entity_resolution(rust_src)
    for issue in entity_issues:
        divergences.append({
            "type": "missing_feature",
            "severity": "HIGH",
            "feature": "entity_resolution",
            "message": issue,
            "fix": f"Implement DTD entity resolution: {issue}",
            "tests_blocked": 3,
        })

    stop_issues = check_stop_resume(rust_src)
    for issue in stop_issues:
        divergences.append({
            "type": "missing_feature",
            "severity": "MEDIUM",
            "feature": "stop_resume",
            "message": issue,
            "fix": f"Implement stop/resume: {issue}",
            "tests_blocked": 2,
        })

    utf16_issues = check_utf16(rust_src)
    for issue in utf16_issues:
        divergences.append({
            "type": "missing_feature",
            "severity": "LOW",
            "feature": "utf16",
            "message": issue,
            "fix": f"Fix UTF-16 support: {issue}",
            "tests_blocked": 1,
        })

    # ---- Architectural check: role state machine ----
    if "xml_token_role" not in rust_src and "xmlrole" not in rust_src:
        divergences.append({
            "type": "architecture",
            "severity": "INFO",
            "message": "Rust prolog does not use xmlrole state machine (uses hand-rolled scan_prolog instead). "
                       "This works for basic cases but diverges architecturally from C.",
            "fix": "Replace scan_prolog/scan_doctype/skip_internal_subset with do_prolog using "
                   "prolog_tok + xml_token_role (matching C's prologProcessor + doProlog)",
        })

    # ---- Output ----
    if actionable_only:
        print("# Structural Divergences (actionable)")
        print(f"# Total: {len(divergences)}")
        print()
        for i, d in enumerate(sorted(divergences, key=lambda x: {"HIGH": 0, "MEDIUM": 1, "LOW": 2, "INFO": 3}[x["severity"]]), 1):
            blocked = f" ({d['tests_blocked']} tests blocked)" if "tests_blocked" in d else ""
            print(f"{i}. [{d['severity']}] {d['message']}{blocked}")
            print(f"   FIX: {d['fix']}")
            print()
    else:
        print(f"=== Structural Comparison Report ===")
        print(f"C functions found: {len(c_funcs)}")
        print(f"Rust functions found: {len(rust_funcs)}")
        print(f"Divergences found: {len(divergences)}")
        print()

        by_severity = {}
        for d in divergences:
            by_severity.setdefault(d["severity"], []).append(d)

        for sev in ["HIGH", "MEDIUM", "LOW", "INFO"]:
            items = by_severity.get(sev, [])
            if items:
                print(f"--- {sev} ({len(items)}) ---")
                for d in items:
                    blocked = f" [{d['tests_blocked']} tests]" if "tests_blocked" in d else ""
                    print(f"  [{d['type']}] {d['message']}{blocked}")
                    print(f"    FIX: {d['fix']}")
                    print()


if __name__ == "__main__":
    main()
