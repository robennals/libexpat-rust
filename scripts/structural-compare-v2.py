#!/usr/bin/env python3
"""Reusable structural comparison: verify Rust code structurally matches C code.

Driven by a YAML/JSON config file that specifies:
- C and Rust source files to compare
- Function pairs (C name → Rust name)
- Error code mapping
- Handler mapping
- Feature checks

Usage:
    python3 scripts/structural-compare-v2.py config.json          # Full report
    python3 scripts/structural-compare-v2.py config.json --actionable  # Actionable items
    python3 scripts/structural-compare-v2.py config.json --json        # Machine-readable JSON

Config format (JSON):
{
  "c_file": "path/to/file.c",
  "rust_file": "path/to/file.rs",
  "function_pairs": [
    {"c": "doContent", "rust": "do_content", "desc": "main content parser loop"}
  ],
  "error_mapping": {"C_ERROR": "RustError", ...},
  "handler_mapping": {"cHandler": "rust_handler", ...},
  "c_error_pattern": "XML_ERROR_(\\w+)",
  "rust_error_pattern": "XmlError::(\\w+)",
  "c_handler_pattern": "parser->m_(\\w+Handler)",
  "rust_handler_pattern": "self\\.(\\w+_handler)",
  "acceptable_missing_errors": ["NoMemory"],
  "rust_functions_to_ignore": ["new", "drop"]
}
"""

import json
import re
import sys
import os


def read(path):
    with open(path) as f:
        return f.read()


def extract_functions(src, lang):
    """Extract function names and bodies from C or Rust source."""
    funcs = {}
    if lang == "c":
        pattern = r'^(?:static\s+)?(?:\w+\s+)+?(?:PTRCALL\s+)?(\w+)\s*\([^)]*\)\s*\{'
    else:
        pattern = r'(?:pub\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\([^)]*\)'

    for m in re.finditer(pattern, src, re.MULTILINE):
        name = m.group(1)
        start = m.start()
        # Find opening brace
        if lang == "rust":
            brace_pos = src.find('{', m.end())
            if brace_pos == -1:
                continue
        else:
            brace_pos = m.end() - 1

        # Find matching closing brace
        depth = 0
        i = brace_pos
        while i < len(src):
            if src[i] == '{':
                depth += 1
            elif src[i] == '}':
                depth -= 1
            if depth == 0:
                funcs[name] = src[start:i+1]
                break
            i += 1
    return funcs


def extract_patterns(body, pattern):
    """Extract all matches of a regex pattern from a function body."""
    return set(re.findall(pattern, body))


def camel_to_snake(name):
    """Convert camelCase to snake_case."""
    return re.sub(r'([A-Z])', r'_\1', name).lower().lstrip('_')


def snake_to_camel(name):
    """Convert snake_case to camelCase."""
    parts = name.split('_')
    return parts[0] + ''.join(p.capitalize() for p in parts[1:])


def load_config(path):
    """Load config from JSON file."""
    with open(path) as f:
        return json.load(f)


def find_unmatched_functions(c_funcs, rust_funcs, config):
    """Find C functions without Rust counterparts and vice versa."""
    ignore_rust = set(config.get("rust_functions_to_ignore", []))
    ignore_c = set(config.get("c_functions_to_ignore", []))

    # Build mapping from config pairs
    paired_c = set()
    paired_rust = set()
    for pair in config.get("function_pairs", []):
        paired_c.add(pair["c"])
        paired_rust.add(pair["rust"])

    # C functions without Rust counterpart
    missing_in_rust = []
    for name in sorted(c_funcs):
        if name in ignore_c or name in paired_c:
            continue
        snake = camel_to_snake(name)
        if snake not in rust_funcs and name not in rust_funcs:
            # Also try lowercased match
            if name.lower() not in {f.replace('_', '') for f in rust_funcs}:
                missing_in_rust.append(name)

    # Rust functions without C counterpart (ad-hoc functions to potentially remove)
    adhoc_rust = []
    for name in sorted(rust_funcs):
        if name in ignore_rust or name in paired_rust:
            continue
        camel = snake_to_camel(name)
        if camel not in c_funcs and name not in c_funcs:
            if name.replace('_', '') not in {f.lower() for f in c_funcs}:
                adhoc_rust.append(name)

    return missing_in_rust, adhoc_rust


def compare_pair(c_funcs, rust_funcs, pair, config):
    """Compare a C/Rust function pair for error codes and handler calls."""
    divergences = []
    c_name = pair["c"]
    rust_name = pair["rust"]
    desc = pair.get("desc", "")

    if c_name not in c_funcs:
        return divergences
    if rust_name not in rust_funcs:
        divergences.append({
            "type": "missing_function",
            "severity": "HIGH",
            "message": f"Rust function '{rust_name}' not found (C has '{c_name}': {desc})",
            "fix": f"Port C function {c_name} to Rust as {rust_name}",
        })
        return divergences

    c_body = c_funcs[c_name]
    r_body = rust_funcs[rust_name]

    # Error code comparison
    error_map = config.get("error_mapping", {})
    c_err_pattern = config.get("c_error_pattern", r"XML_ERROR_(\w+)")
    r_err_pattern = config.get("rust_error_pattern", r"XmlError::(\w+)")
    acceptable = set(config.get("acceptable_missing_errors", []))

    c_errs = extract_patterns(c_body, c_err_pattern)
    r_errs = extract_patterns(r_body, r_err_pattern)
    c_mapped = {error_map.get(e, f"UNMAPPED:{e}") for e in c_errs}
    unmapped = {f"UNMAPPED:{e}" for e in c_errs if e not in error_map}

    missing_errs = c_mapped - r_errs - acceptable - unmapped
    if missing_errs:
        divergences.append({
            "type": "missing_errors",
            "severity": "MEDIUM",
            "message": f"{rust_name} missing error codes vs C {c_name}: {sorted(missing_errs)}",
            "fix": f"Add error handling for {sorted(missing_errs)} in {rust_name}, matching C {c_name} behavior",
        })

    # Handler comparison
    handler_map = config.get("handler_mapping", {})
    c_hdlr_pattern = config.get("c_handler_pattern", r"parser->m_(\w+Handler)")
    r_hdlr_pattern = config.get("rust_handler_pattern", r"self\.(\w+_handler)")

    c_hdlrs = extract_patterns(c_body, c_hdlr_pattern)
    r_hdlrs = extract_patterns(r_body, r_hdlr_pattern)
    c_mapped_h = {handler_map.get(h, f"UNMAPPED:{h}") for h in c_hdlrs}
    unmapped_h = {f"UNMAPPED:{h}" for h in c_hdlrs if h not in handler_map}

    missing_hdlrs = c_mapped_h - r_hdlrs - unmapped_h
    if missing_hdlrs:
        divergences.append({
            "type": "missing_handlers",
            "severity": "MEDIUM",
            "message": f"{rust_name} doesn't call handlers that C {c_name} calls: {sorted(missing_hdlrs)}",
            "fix": f"Add handler invocations for {sorted(missing_hdlrs)} in {rust_name}",
        })

    # Control flow comparison: count key patterns
    for pattern_name, c_pat, r_pat in config.get("control_flow_patterns", []):
        c_count = len(re.findall(c_pat, c_body))
        r_count = len(re.findall(r_pat, r_body))
        if c_count > 0 and r_count == 0:
            divergences.append({
                "type": "missing_control_flow",
                "severity": "MEDIUM",
                "message": f"{rust_name} missing '{pattern_name}' pattern (C {c_name} has {c_count} instances)",
                "fix": f"Add {pattern_name} handling in {rust_name}",
            })

    return divergences


def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} config.json [--actionable|--json|--adhoc]", file=sys.stderr)
        sys.exit(1)

    config_path = sys.argv[1]
    flags = set(sys.argv[2:])

    config = load_config(config_path)

    # Resolve paths relative to config file directory
    config_dir = os.path.dirname(os.path.abspath(config_path))
    c_file = os.path.join(config_dir, config["c_file"]) if not os.path.isabs(config["c_file"]) else config["c_file"]
    rust_file = os.path.join(config_dir, config["rust_file"]) if not os.path.isabs(config["rust_file"]) else config["rust_file"]

    c_src = read(c_file)
    rust_src = read(rust_file)

    c_funcs = extract_functions(c_src, "c")
    rust_funcs = extract_functions(rust_src, "rust")

    divergences = []

    # Function pair comparisons
    for pair in config.get("function_pairs", []):
        divergences.extend(compare_pair(c_funcs, rust_funcs, pair, config))

    # Find unmatched functions
    missing_in_rust, adhoc_rust = find_unmatched_functions(c_funcs, rust_funcs, config)

    if "--adhoc" in flags:
        # Just list ad-hoc Rust functions
        print("# Ad-hoc Rust functions (no C counterpart)")
        print(f"# Total: {len(adhoc_rust)}")
        for name in adhoc_rust:
            print(f"  {name}")
        print()
        print(f"# C functions not yet ported to Rust")
        print(f"# Total: {len(missing_in_rust)}")
        for name in missing_in_rust:
            print(f"  {name}")
        return

    # Add unmatched as divergences
    for name in missing_in_rust:
        divergences.append({
            "type": "missing_port",
            "severity": "LOW",
            "message": f"C function '{name}' has no Rust counterpart",
            "fix": f"Port {name} from C to Rust",
        })

    if "--json" in flags:
        result = {
            "c_functions": len(c_funcs),
            "rust_functions": len(rust_funcs),
            "divergences": divergences,
            "adhoc_rust_functions": adhoc_rust,
            "missing_c_ports": missing_in_rust,
        }
        print(json.dumps(result, indent=2))
        return

    if "--actionable" in flags:
        print("# Structural Divergences (actionable)")
        print(f"# Total: {len(divergences)}")
        print()
        severity_order = {"HIGH": 0, "MEDIUM": 1, "LOW": 2, "INFO": 3}
        for i, d in enumerate(sorted(divergences, key=lambda x: severity_order.get(x["severity"], 9)), 1):
            print(f"{i}. [{d['severity']}] {d['message']}")
            print(f"   FIX: {d['fix']}")
            print()
    else:
        print(f"=== Structural Comparison Report ===")
        print(f"C functions found: {len(c_funcs)}")
        print(f"Rust functions found: {len(rust_funcs)}")
        print(f"Ad-hoc Rust functions (no C counterpart): {len(adhoc_rust)}")
        print(f"C functions not yet ported: {len(missing_in_rust)}")
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
                    print(f"  [{d['type']}] {d['message']}")
                    print(f"    FIX: {d['fix']}")
                    print()


if __name__ == "__main__":
    main()
