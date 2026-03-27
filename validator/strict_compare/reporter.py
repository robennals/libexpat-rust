"""Format and report mismatches from structural comparison."""

import json
from .nodes import Mismatch, SkeletonNode


def format_text(mismatches: list[Mismatch], c_func: str, r_func: str) -> str:
    """Format mismatches as human-readable text."""
    if not mismatches:
        return f"  {c_func} <-> {r_func}: OK (structurally equivalent)"

    lines = [f"  {c_func} <-> {r_func}: {len(mismatches)} mismatch(es)"]
    errors = [m for m in mismatches if m.severity == "ERROR"]
    warnings = [m for m in mismatches if m.severity == "WARNING"]
    infos = [m for m in mismatches if m.severity == "INFO"]

    if errors:
        lines.append(f"    ERRORS ({len(errors)}):")
        for m in errors:
            lines.append(f"      {_format_mismatch(m)}")

    if warnings:
        lines.append(f"    WARNINGS ({len(warnings)}):")
        for m in warnings:
            lines.append(f"      {_format_mismatch(m)}")

    if infos:
        lines.append(f"    INFO ({len(infos)}):")
        for m in infos:
            lines.append(f"      {_format_mismatch(m)}")

    return "\n".join(lines)


def format_json(mismatches: list[Mismatch], c_func: str, r_func: str) -> dict:
    """Format mismatches as JSON-serializable dict."""
    return {
        "c_function": c_func,
        "rust_function": r_func,
        "status": "ok" if not mismatches else "mismatch",
        "error_count": sum(1 for m in mismatches if m.severity == "ERROR"),
        "warning_count": sum(1 for m in mismatches if m.severity == "WARNING"),
        "mismatches": [_mismatch_to_dict(m) for m in mismatches],
    }


def _format_mismatch(m: Mismatch) -> str:
    parts = []
    if m.context:
        parts.append(f"[{m.context}]")
    if m.c_node and m.c_node.source_start:
        parts.append(f"C@{m.c_node.source_start}")
    if m.r_node and m.r_node.source_start:
        parts.append(f"Rust@{m.r_node.source_start}")
    parts.append(m.reason)
    return " ".join(parts)


def _mismatch_to_dict(m: Mismatch) -> dict:
    d = {
        "severity": m.severity,
        "reason": m.reason,
        "context": m.context,
    }
    if m.c_node:
        d["c_location"] = {
            "kind": m.c_node.kind,
            "label": m.c_node.label,
            "line_start": m.c_node.source_start,
            "line_end": m.c_node.source_end,
        }
    if m.r_node:
        d["r_location"] = {
            "kind": m.r_node.kind,
            "label": m.r_node.label,
            "line_start": m.r_node.source_start,
            "line_end": m.r_node.source_end,
        }
    return d


def print_skeleton(skel: SkeletonNode, title: str = ""):
    """Print a skeleton tree for debugging."""
    if title:
        print(f"\n=== {title} ===")
    print(skel.dump())
