"""Compare scoping trees and content sets.

Compares two functions by:
1. Matching their scoping trees (control flow structure)
2. Comparing content sets within each matched scope
"""

import re
import json
import os
from .scoping import ScopeNode, ContentSet


class Mismatch:
    def __init__(self, category: str, detail: str, c_line: int = 0, r_line: int = 0, path: str = ""):
        self.category = category  # "missing_scope", "extra_scope", "missing_call", etc.
        self.detail = detail
        self.c_line = c_line
        self.r_line = r_line
        self.path = path

    def __repr__(self):
        loc = ""
        if self.c_line:
            loc += f" C@{self.c_line}"
        if self.r_line:
            loc += f" R@{self.r_line}"
        path = f" [{self.path}]" if self.path else ""
        return f"[{self.category}]{path}{loc}: {self.detail}"


# Rules loaded from YAML
_rules = None


def load_rules(filepath: str = None):
    global _rules
    import yaml
    if filepath is None:
        filepath = os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            "..", "scope-rules.yaml"
        )
    with open(filepath) as f:
        _rules = yaml.safe_load(f)
    return _rules


def compare_scopes(c_scope: ScopeNode, r_scope: ScopeNode,
                   path: str = "") -> list[Mismatch]:
    """Compare two scoping trees with content sets."""
    if _rules is None:
        load_rules()

    mismatches = []
    _compare_scope(c_scope, r_scope, path, mismatches)
    return mismatches


def _compare_scope(c: ScopeNode, r: ScopeNode, path: str,
                   mismatches: list[Mismatch]):
    """Compare a matched pair of scoping nodes."""

    # Compare content sets at this level
    _compare_content(c.content, r.content, path, c, r, mismatches)

    # Compare child scoping nodes
    _compare_scope_children(c.children, r.children, path, mismatches)


def _compare_content(c_content: ContentSet, r_content: ContentSet,
                     path: str, c_scope: ScopeNode, r_scope: ScopeNode,
                     mismatches: list[Mismatch]):
    """Compare content sets within a matched scope pair."""

    rules = _rules or {}
    skip_c_calls = set(rules.get("skip_c_calls", []))
    skip_c_idents = set(rules.get("skip_c_identifiers", []))
    skip_c_assigns = set(rules.get("skip_c_assigns_patterns", []))
    skip_r_calls = set(rules.get("skip_r_calls", []))
    c_to_r_call_map = rules.get("call_name_map", {})
    c_to_r_ident_map = rules.get("identifier_map", {})

    # --- Compare calls ---
    for c_call, c_args in c_content.calls.items():
        if c_call in skip_c_calls:
            continue
        # Map C call name to Rust equivalent
        r_call = c_to_r_call_map.get(c_call, c_call)
        # Check current scope AND child scopes for the call
        r_all_calls = _collect_all_calls(r_scope)
        if r_call in r_content.calls:
            # Call exists in Rust at same scope — compare arg identifiers
            r_args = r_content.calls[r_call]
            # Map C arg identifiers
            mapped_c_args = set()
            for arg in c_args:
                if arg in skip_c_idents:
                    continue
                mapped = c_to_r_ident_map.get(arg, arg)
                mapped_c_args.add(mapped)
            # Check C args are subset of Rust args (Rust may have extras)
            missing_args = mapped_c_args - r_args - skip_c_idents
            if missing_args:
                mismatches.append(Mismatch(
                    "missing_call_arg",
                    f"Call {c_call}(): C args {sorted(missing_args)} not in Rust",
                    c_scope.line, r_scope.line, path,
                ))
        elif r_call != c_call and c_call in r_content.calls:
            pass  # Original name found (mapping wasn't needed)
        elif r_call in r_all_calls or c_call in r_all_calls:
            pass  # Call exists in Rust but at a different scope level — OK
        else:
            mismatches.append(Mismatch(
                "missing_call",
                f"C calls {c_call}() but Rust doesn't (in any scope)",
                c_scope.line, r_scope.line, path,
            ))

    # Check for extra Rust calls not in C (informational, not error)
    for r_call in r_content.calls:
        if r_call in skip_r_calls:
            continue
        # Reverse-map: is this Rust call covered by any C call?
        found = False
        for c_call in c_content.calls:
            mapped = c_to_r_call_map.get(c_call, c_call)
            if mapped == r_call or c_call == r_call:
                found = True
                break
        if not found:
            mismatches.append(Mismatch(
                "extra_call",
                f"Rust calls {r_call}() but C doesn't (in this scope)",
                c_scope.line, r_scope.line, path,
            ))

    # --- Compare error returns ---
    skip_c_errors = set(rules.get("skip_c_errors", []))
    for c_error in c_content.error_returns:
        if c_error in skip_c_errors:
            continue
        if c_error not in r_content.error_returns:
            # Try with normalized name
            normalized = c_error.lower().replace("xml_error_", "xmlerror::")
            if not any(r.lower() == normalized for r in r_content.error_returns):
                mismatches.append(Mismatch(
                    "missing_error_return",
                    f"C returns {c_error} but Rust doesn't (in this scope)",
                    c_scope.line, r_scope.line, path,
                ))


def _compare_scope_children(c_children: list[ScopeNode], r_children: list[ScopeNode],
                            path: str, mismatches: list[Mismatch]):
    """Compare child scoping nodes between C and Rust."""

    rules = _rules or {}
    skip_c_scope_labels = set(rules.get("skip_c_scope_labels", []))
    skip_c_scope_kinds = set(rules.get("skip_c_scope_kinds", []))
    skip_r_scope_labels = set(rules.get("skip_r_scope_labels", []))

    # Filter out skippable scopes and flatten transparent scopes
    flatten_r = rules.get("flatten_r_scopes", [])
    flatten_both = rules.get("flatten_both_scopes", [])

    def should_flatten_r(scope):
        for pat in flatten_r:
            if scope.label.startswith(pat):
                return True
            if scope.kind == pat and not scope.label:
                return True
        return False

    c_effective = []
    for c in c_children:
        if c.kind in skip_c_scope_kinds:
            continue
        if any(pat in c.label for pat in skip_c_scope_labels):
            continue
        # Unwind bare C blocks: replace with their children.
        # C wraps switch/loop/if bodies in block nodes that Rust doesn't have.
        if c.kind == "block" and not c.label:
            for bc in c.children:
                if bc.kind in skip_c_scope_kinds:
                    continue
                if any(pat in bc.label for pat in skip_c_scope_labels):
                    continue
                c_effective.append(bc)
            continue
        c_effective.append(c)

    r_effective = []
    for r in r_children:
        if any(pat in r.label for pat in skip_r_scope_labels):
            continue
        if should_flatten_r(r):
            for child in r.children:
                if any(pat in child.label for pat in skip_r_scope_labels):
                    continue
                r_effective.append(child)
        else:
            r_effective.append(r)


    # Match by kind and label
    # For match/arm: pair by arm label
    # For if: pair by condition identifiers
    # For loop/block: pair by position

    if c_effective and c_effective[0].kind == "arm" and r_effective and r_effective[0].kind == "arm":
        # Match arms by label
        _match_arms(c_effective, r_effective, path, mismatches)
        return

    # General case: pair by kind, using greedy matching with lookahead
    c_idx = 0
    r_idx = 0
    while c_idx < len(c_effective) and r_idx < len(r_effective):
        c_node = c_effective[c_idx]
        r_node = r_effective[r_idx]

        if _scopes_match(c_node, r_node):
            _compare_scope(c_node, r_node,
                           f"{path}/{c_node.kind}({c_node.label[:20]})",
                           mismatches)
            c_idx += 1
            r_idx += 1
            continue

        # Try lookahead
        found = False
        for r_look in range(r_idx + 1, min(r_idx + 5, len(r_effective))):
            if _scopes_match(c_node, r_effective[r_look]):
                for r_skip in range(r_idx, r_look):
                    mismatches.append(Mismatch(
                        "extra_scope",
                        f"Extra Rust {r_effective[r_skip].kind}({r_effective[r_skip].label[:30]})",
                        r_line=r_effective[r_skip].line,
                        path=path,
                    ))
                _compare_scope(c_node, r_effective[r_look],
                               f"{path}/{c_node.kind}({c_node.label[:20]})",
                               mismatches)
                c_idx += 1
                r_idx = r_look + 1
                found = True
                break
        if found:
            continue

        for c_look in range(c_idx + 1, min(c_idx + 5, len(c_effective))):
            if _scopes_match(c_effective[c_look], r_node):
                for c_skip in range(c_idx, c_look):
                    mismatches.append(Mismatch(
                        "missing_scope",
                        f"Extra C {c_effective[c_skip].kind}({c_effective[c_skip].label[:30]})",
                        c_line=c_effective[c_skip].line,
                        path=path,
                    ))
                _compare_scope(c_effective[c_look], r_node,
                               f"{path}/{r_node.kind}({r_node.label[:20]})",
                               mismatches)
                c_idx = c_look + 1
                r_idx += 1
                found = True
                break
        if found:
            continue

        # No match — report both
        mismatches.append(Mismatch(
            "scope_mismatch",
            f"C {c_node.kind}({c_node.label[:30]}) vs R {r_node.kind}({r_node.label[:30]})",
            c_line=c_node.line, r_line=r_node.line, path=path,
        ))
        c_idx += 1
        r_idx += 1

    # Remaining
    while c_idx < len(c_effective):
        c_node = c_effective[c_idx]
        mismatches.append(Mismatch(
            "missing_scope",
            f"Extra C {c_node.kind}({c_node.label[:30]})",
            c_line=c_node.line, path=path,
        ))
        c_idx += 1

    while r_idx < len(r_effective):
        r_node = r_effective[r_idx]
        mismatches.append(Mismatch(
            "extra_scope",
            f"Extra Rust {r_node.kind}({r_node.label[:30]})",
            r_line=r_node.line, path=path,
        ))
        r_idx += 1


def _flatten_loop_blocks(children: list[ScopeNode]) -> list[ScopeNode]:
    """If a list has a single bare block child, promote its children."""
    if len(children) == 1 and children[0].kind == "block" and not children[0].label:
        return children[0].children
    return children


def _collect_all_calls(scope: ScopeNode) -> set[str]:
    """Collect all call names from a scope and all its descendants."""
    calls = set(scope.content.calls.keys())
    for child in scope.children:
        calls.update(_collect_all_calls(child))
    return calls


def _scopes_match(c: ScopeNode, r: ScopeNode) -> bool:
    """Do two scoping nodes correspond to each other?"""
    # Same kind
    if c.kind == r.kind:
        if c.kind == "arm":
            return _arm_labels_match(c.label, r.label)
        if c.kind in ("if", "if_let"):
            return True
        return True

    # Cross-kind matches
    if c.kind == "if" and r.kind == "if_let":
        return True
    if c.kind == "if_let" and r.kind == "if":
        return True


    return False


def _arm_labels_match(c_label: str, r_label: str) -> bool:
    """Do two arm labels match?"""
    if c_label == r_label:
        return True
    # Try short form
    c_short = c_label.split("::")[-1] if "::" in c_label else c_label
    r_short = r_label.split("::")[-1] if "::" in r_label else r_label
    if c_short == r_short:
        return True
    # Prefix match: "Ok" matches "Ok(TokenResult ...)"
    if r_label.startswith(f"{c_label}(") or c_label.startswith(f"{r_label}("):
        return True
    return False


def _match_arms(c_arms: list[ScopeNode], r_arms: list[ScopeNode],
                path: str, mismatches: list[Mismatch]):
    """Match switch/match arms by label."""
    r_arm_map = {}
    for arm in r_arms:
        if arm.label:
            for sub in arm.label.split("|"):
                r_arm_map[sub.strip()] = arm

    matched_r = set()
    for c_arm in c_arms:
        if not c_arm.label:
            continue
        # Find matching Rust arm
        r_arm = r_arm_map.get(c_arm.label)
        if not r_arm:
            # Try short label
            c_short = c_arm.label.split("::")[-1]
            for r_label, r_a in r_arm_map.items():
                r_short = r_label.split("::")[-1]
                if c_short == r_short:
                    r_arm = r_a
                    break
        if not r_arm:
            # Try prefix
            for r_label, r_a in r_arm_map.items():
                if r_label.startswith(f"{c_arm.label}("):
                    r_arm = r_a
                    break

        if r_arm:
            matched_r.add(id(r_arm))
            _compare_scope(c_arm, r_arm,
                           f"{path}/{c_arm.label[:30]}",
                           mismatches)
        else:
            if c_arm.label == "_default":
                continue  # Default arms often differ
            # Check if the arm is empty (fallthrough)
            if not c_arm.children and not c_arm.content.calls:
                continue
            mismatches.append(Mismatch(
                "missing_arm",
                f"C arm {c_arm.label} has no Rust match",
                c_line=c_arm.line, path=path,
            ))

    # Check for extra Rust arms
    for r_arm in r_arms:
        if id(r_arm) not in matched_r and r_arm.label and r_arm.label != "_default":
            # Informational — Rust may handle extra cases
            pass
