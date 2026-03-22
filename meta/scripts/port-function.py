#!/usr/bin/env python3
"""Bottom-up C-to-Rust function porting tool.

Analyzes C call tree, extracts functions with context, and generates
agent-ready prompts for porting. Works bottom-up: port leaves first.

Usage:
    python3 scripts/port-function.py analyze              # Show call tree & port order
    python3 scripts/port-function.py ready                # Show functions ready to port
    python3 scripts/port-function.py extract <c_func>     # Extract C function + deps for porting
    python3 scripts/port-function.py prompt <c_func>      # Generate full agent prompt
    python3 scripts/port-function.py decompose <c_func>   # Suggest sub-function decomposition
"""

import re
import sys
import os
import json

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
# Support both submodule layout (expat/expat/lib/) and flat layout (expat/lib/)
C_FILE = os.path.join(ROOT, "expat", "expat", "lib", "xmlparse.c")
if not os.path.exists(C_FILE):
    C_FILE = os.path.join(ROOT, "expat", "lib", "xmlparse.c")
RUST_FILE = os.path.join(ROOT, "expat-rust", "src", "xmlparse.rs")


def read(path):
    with open(path) as f:
        return f.read()


def extract_c_functions(src):
    """Extract all C function names, bodies, and line counts."""
    funcs = {}
    for m in re.finditer(
        r'^(?:static\s+)?(?:\w+\s+)+?(?:PTRCALL\s+)?(\w+)\s*\([^)]*\)\s*\{',
        src, re.MULTILINE
    ):
        name = m.group(1)
        start = m.start()
        depth = 0
        i = m.end() - 1
        while i < len(src):
            if src[i] == '{': depth += 1
            elif src[i] == '}': depth -= 1
            if depth == 0:
                funcs[name] = src[start:i+1]
                break
            i += 1
    return funcs


def extract_rust_functions(src):
    """Extract all Rust function names."""
    funcs = {}
    for m in re.finditer(r'(?:pub\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\(', src):
        name = m.group(1)
        start = m.start()
        brace_pos = src.find('{', m.end())
        if brace_pos == -1:
            continue
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


def camel_to_snake(name):
    return re.sub(r'([A-Z])', r'_\1', name).lower().lstrip('_')


def build_call_graph(funcs):
    """Build call graph: {func_name: set of called func_names}."""
    all_names = set(funcs.keys())
    # Filter out C keywords that match function regex
    skip = {'if', 'while', 'for', 'switch', 'return', 'sizeof', 'memcpy', 'memset',
            'malloc', 'realloc', 'free', 'MALLOC', 'REALLOC', 'FREE',
            'XML_TRUE', 'XML_FALSE'}
    all_names -= skip

    calls = {}
    for name in all_names:
        body = funcs[name]
        called = set()
        for other in all_names:
            if other == name:
                continue
            if re.search(r'\b' + re.escape(other) + r'\s*\(', body):
                called.add(other)
        calls[name] = called
    return calls


def is_ported(c_name, rust_funcs):
    """Check if a C function has a Rust counterpart."""
    snake = camel_to_snake(c_name)
    return snake in rust_funcs or c_name in rust_funcs


def get_rust_name(c_name, rust_funcs):
    """Get the Rust function name for a C function."""
    snake = camel_to_snake(c_name)
    if snake in rust_funcs:
        return snake
    if c_name in rust_funcs:
        return c_name
    return snake  # default


# Functions that don't need porting (memory management, testing, etc.)
SKIP_FUNCTIONS = {
    'if', 'while', 'for', 'switch', 'return', 'sizeof',
    'MALLOC', 'REALLOC', 'FREE', 'XML_TRUE', 'XML_FALSE',
    # Memory pool functions (Rust uses String/Vec)
    'poolInit', 'poolClear', 'poolDestroy', 'poolGrow', 'poolBytesToAllocateFor',
    # Hash table (Rust uses HashMap)
    'hashTableInit', 'hashTableClear', 'hashTableDestroy', 'hashTableIterInit',
    'hash', 'keyeq', 'keylen', 'copy_salt_to_sipkey',
    # Accounting (optional, can be added later)
    'accountingDiffTolerated', 'accountingOnAbort', 'accountingReportDiff',
    'accountingReportStats', 'accountingGetCurrentAmplification',
    'entityTrackingOnOpen', 'entityTrackingOnClose', 'entityTrackingReportStats',
    'expat_heap_increase_tolerable', 'expat_heap_stat',
    # Random/entropy (Rust uses rand or OsRng)
    'generate_hash_secret_salt', 'get_hash_secret_salt', 'writeRandomBytes_arc4random',
    'writeRandomBytes_dev_urandom', 'writeRandomBytes_getrandom_nonblock',
    'writeRandomBytes_rand_s', 'gather_time_entropy', 'ENTROPY_DEBUG',
    # Testing functions
    'testingAccountingGetCountBytesDirect', 'testingAccountingGetCountBytesIndirect',
    'triggerReenter',
    # Parser lifecycle (already handled by Rust new/drop)
    'XML_ParserCreate', 'XML_ParserCreateNS', 'XML_ParserCreate_MM',
    'XML_ParserFree', 'XML_ParserReset', 'parserInit',
    # Simple API wrappers (already in Rust)
    'XML_Parse', 'XML_ParseBuffer', 'XML_GetErrorCode', 'XML_ErrorString',
    'XML_GetCurrentLineNumber', 'XML_GetCurrentColumnNumber',
    'XML_GetCurrentByteIndex', 'XML_GetCurrentByteCount',
    'XML_GetIdAttributeIndex', 'XML_GetSpecifiedAttributeCount',
    'XML_GetParsingStatus', 'XML_DefaultCurrent', 'XML_ExpatVersion',
    'XML_ExpatVersionInfo', 'XML_GetFeatureList', 'XML_GetBase',
    'XML_FreeContentModel', 'XML_MemMalloc', 'XML_MemRealloc', 'XML_MemFree',
    # Setter functions (already in Rust)
    'XML_SetEncoding', 'XML_SetBase', 'XML_SetHashSalt',
    'XML_SetUserData', 'XML_SetReturnNSTriplet', 'XML_UseForeignDTD',
    'XML_SetParamEntityParsing', 'XML_SetReparseDeferralEnabled',
    'XML_SetBillionLaughsAttackProtectionActivationThreshold',
    'XML_SetBillionLaughsAttackProtectionMaximumAmplification',
    'XML_SetAllocTrackerActivationThreshold', 'XML_SetAllocTrackerMaximumAmplification',
    'XML_StopParser', 'XML_ResumeParser', 'XML_UseParserAsHandlerArg',
    'XML_SetExternalEntityRefHandlerArg',
    # All Set*Handler functions
    'XML_SetElementHandler', 'XML_SetStartElementHandler', 'XML_SetEndElementHandler',
    'XML_SetCharacterDataHandler', 'XML_SetProcessingInstructionHandler',
    'XML_SetCommentHandler', 'XML_SetCdataSectionHandler',
    'XML_SetStartCdataSectionHandler', 'XML_SetEndCdataSectionHandler',
    'XML_SetDefaultHandler', 'XML_SetDefaultHandlerExpand',
    'XML_SetDoctypeDeclHandler', 'XML_SetStartDoctypeDeclHandler',
    'XML_SetEndDoctypeDeclHandler', 'XML_SetElementDeclHandler',
    'XML_SetAttlistDeclHandler', 'XML_SetXmlDeclHandler',
    'XML_SetEntityDeclHandler', 'XML_SetUnparsedEntityDeclHandler',
    'XML_SetNotationDeclHandler', 'XML_SetNamespaceDeclHandler',
    'XML_SetStartNamespaceDeclHandler', 'XML_SetEndNamespaceDeclHandler',
    'XML_SetNotStandaloneHandler', 'XML_SetExternalEntityRefHandler',
    'XML_SetSkippedEntityHandler', 'XML_SetUnknownEncodingHandler',
    # Utility
    'parserBusy', 'getRootParserOf', 'getDebugLevel',
}


def cmd_analyze(c_funcs, rust_funcs, calls):
    """Show call tree and port order."""
    # Core functions we care about
    core = ['doContent', 'doProlog', 'storeAtts', 'epilogProcessor',
            'doCdataSection', 'contentProcessor', 'prologProcessor',
            'processXmlDecl', 'addBinding', 'processEntity',
            'appendAttributeValue', 'storeEntityValue', 'callStoreEntityValue']

    print("=== Port status of core functions and their dependencies ===\n")

    for fn in core:
        if fn not in c_funcs:
            continue
        lines = c_funcs[fn].count('\n') + 1
        ported = "PORTED" if is_ported(fn, rust_funcs) else "NOT PORTED"
        deps = sorted(calls.get(fn, set()) - SKIP_FUNCTIONS)
        unported_deps = [d for d in deps if not is_ported(d, rust_funcs) and d not in SKIP_FUNCTIONS]

        status = "READY" if not unported_deps and not is_ported(fn, rust_funcs) else ported
        print(f"{fn} ({lines} lines) [{status}]")
        for d in deps:
            d_ported = "✓" if is_ported(d, rust_funcs) else "✗"
            d_lines = c_funcs[d].count('\n') + 1 if d in c_funcs else 0
            print(f"  {d_ported} {d} ({d_lines} lines)")
        print()


def cmd_ready(c_funcs, rust_funcs, calls):
    """Show functions that are ready to port (all deps satisfied)."""
    print("=== Functions ready to port (all callees already ported or skippable) ===\n")

    ready = []
    for name in sorted(c_funcs):
        if name in SKIP_FUNCTIONS or is_ported(name, rust_funcs):
            continue
        deps = calls.get(name, set()) - SKIP_FUNCTIONS
        unported = [d for d in deps if not is_ported(d, rust_funcs) and d not in SKIP_FUNCTIONS]
        if not unported:
            lines = c_funcs[name].count('\n') + 1
            ready.append((lines, name))

    ready.sort()
    for lines, name in ready:
        print(f"  {lines:5d} lines  {name}")
    print(f"\nTotal: {len(ready)} functions ready to port")


def cmd_extract(c_funcs, func_name):
    """Extract a C function and its dependencies."""
    if func_name not in c_funcs:
        print(f"Error: function '{func_name}' not found in C source", file=sys.stderr)
        sys.exit(1)

    print(f"// C function: {func_name}")
    print(f"// Lines: {c_funcs[func_name].count(chr(10)) + 1}")
    print()
    print(c_funcs[func_name])


def cmd_decompose(c_funcs, calls, func_name):
    """Suggest how to decompose a large C function into sub-functions."""
    if func_name not in c_funcs:
        print(f"Error: function '{func_name}' not found", file=sys.stderr)
        sys.exit(1)

    body = c_funcs[func_name]
    lines = body.count('\n') + 1

    print(f"=== Decomposition analysis for {func_name} ({lines} lines) ===\n")

    # Find switch/case blocks (common in doProlog/doContent)
    cases = re.findall(r'case\s+(\w+)\s*:', body)
    if cases:
        print(f"Switch cases ({len(cases)}):")
        # Group by prefix
        groups = {}
        for case in cases:
            prefix = case.split('_')[0] if '_' in case else case[:10]
            groups.setdefault(prefix, []).append(case)
        for prefix, group_cases in sorted(groups.items()):
            print(f"  {prefix}: {len(group_cases)} cases")
        print()
        print("Suggested decomposition: extract each case group into a handler function")
        print("Pattern: handle_{prefix}_role(role, tok, data, pos, next) -> XmlError")
        print()

    # Find repeated patterns (error handling, handler calls)
    handler_calls = re.findall(r'parser->m_(\w+Handler)', body)
    if handler_calls:
        unique = sorted(set(handler_calls))
        print(f"Handler calls ({len(handler_calls)} total, {len(unique)} unique):")
        for h in unique:
            count = handler_calls.count(h)
            print(f"  {h}: {count}x")
        print()

    # Estimate complexity
    if lines > 200:
        n_chunks = (lines + 99) // 100
        print(f"Recommendation: split into ~{n_chunks} sub-functions of ~100 lines each")
        print(f"Use switch case groups or logical sections as natural boundaries")


def cmd_prompt(c_funcs, rust_funcs, calls, func_name):
    """Generate a complete agent prompt for porting a function."""
    if func_name not in c_funcs:
        print(f"Error: function '{func_name}' not found", file=sys.stderr)
        sys.exit(1)

    c_body = c_funcs[func_name]
    lines = c_body.count('\n') + 1
    rust_name = get_rust_name(func_name, rust_funcs)

    # Get existing Rust version if any
    rust_body = rust_funcs.get(rust_name, None)

    # Get callees that are already ported
    deps = calls.get(func_name, set()) - SKIP_FUNCTIONS
    ported_deps = [(d, get_rust_name(d, rust_funcs)) for d in deps if is_ported(d, rust_funcs)]

    print(f"# Port {func_name} → {rust_name}")
    print()
    print(f"Port the C function `{func_name}` ({lines} lines) to Rust as `{rust_name}`.")
    print()
    print("## C source")
    print("```c")
    print(c_body)
    print("```")
    print()

    if rust_body:
        print("## Existing Rust version (to update/replace)")
        print("```rust")
        print(rust_body)
        print("```")
        print()

    if ported_deps:
        print("## Already-ported callees (use these)")
        for c_dep, r_dep in ported_deps:
            print(f"- `{c_dep}` → `{r_dep}`")
        print()

    print("## Rules")
    print("1. Match C behavior exactly — use the C code as ground truth")
    print("2. Use idiomatic Rust (no unsafe, use enums/Result/Option)")
    print("3. C's pool/hash → Rust's String/Vec/HashMap")
    print("4. Maintain the same error codes and handler calls as C")
    print("5. If the function is >200 lines, split into sub-functions")


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    cmd = sys.argv[1]

    c_src = read(C_FILE)
    rust_src = read(RUST_FILE)
    c_funcs = extract_c_functions(c_src)
    rust_funcs = extract_rust_functions(rust_src)
    calls = build_call_graph(c_funcs)

    if cmd == "analyze":
        cmd_analyze(c_funcs, rust_funcs, calls)
    elif cmd == "ready":
        cmd_ready(c_funcs, rust_funcs, calls)
    elif cmd == "extract" and len(sys.argv) > 2:
        cmd_extract(c_funcs, sys.argv[2])
    elif cmd == "decompose" and len(sys.argv) > 2:
        cmd_decompose(c_funcs, calls, sys.argv[2])
    elif cmd == "prompt" and len(sys.argv) > 2:
        cmd_prompt(c_funcs, rust_funcs, calls, sys.argv[2])
    else:
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
