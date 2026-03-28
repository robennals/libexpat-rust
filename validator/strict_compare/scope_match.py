"""Compare scoping trees and content sets.

Compares two functions by:
1. Matching their scoping trees (control flow structure) using identity-based
   pairing (arm labels, condition identifiers, scope equivalence rules)
2. Comparing content sets within each matched scope
3. Reporting unmatched scopes as mismatches (unless covered by drop rules)
"""

import re
import json
import os
from .scoping import ScopeNode, ContentSet


class Mismatch:
    def __init__(self, category: str, detail: str, c_line: int = 0, r_line: int = 0, path: str = ""):
        self.category = category
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
    if _rules is None:
        load_rules()
    mismatches = []
    _compare_scope(c_scope, r_scope, path, mismatches)
    return mismatches


def _compare_scope(c: ScopeNode, r: ScopeNode, path: str,
                   mismatches: list[Mismatch]):
    """Compare a matched pair of scoping nodes."""
    # Compare content
    _compare_content(c.content, r.content, path, c, r, mismatches)
    # Compare child scopes
    _compare_scope_children(c.children, r.children, path, mismatches)


def _compare_scope_children(c_children: list[ScopeNode], r_children: list[ScopeNode],
                            path: str, mismatches: list[Mismatch]):
    """Compare child scoping nodes using identity-based matching.

    1. Match arms by label
    2. Match other scopes by kind + label identity (or equivalence rules)
    3. Check drop rules for unmatched scopes
    4. Report remaining unmatched as mismatches
    """
    rules = _rules or {}
    scope_equivs = rules.get("scope_equivalences", [])
    c_drops = rules.get("c_only_scopes", [])
    r_drops = rules.get("r_only_scopes", [])

    c_matched = {}  # c_idx → r_idx
    r_matched = {}  # r_idx → c_idx

    # Phase 1: Match arms by label
    c_arms = [(i, c) for i, c in enumerate(c_children) if c.kind == "arm"]
    r_arms = [(i, r) for i, r in enumerate(r_children) if r.kind == "arm"]
    if c_arms and r_arms:
        _pair_arms(c_arms, r_arms, c_matched, r_matched)

    # Phase 2: Match non-arm scopes using maximum bipartite matching
    # Build compatibility matrix: which C scopes can match which R scopes
    c_remaining = [(i, c) for i, c in enumerate(c_children)
                   if i not in c_matched and c.kind != "arm"]
    r_remaining = [(i, r) for i, r in enumerate(r_children)
                   if i not in r_matched and r.kind != "arm"]

    def can_match(c_node, r_node):
        if _scopes_match(c_node, r_node):
            return True
        for rule in scope_equivs:
            if _equiv_rule_matches(c_node, r_node, rule):
                return True
        return False

    # Find maximum matching (augmenting paths algorithm)
    best_pairs = _find_max_matching(c_remaining, r_remaining, can_match)
    for ci, ri in best_pairs:
        c_matched[ci] = ri
        r_matched[ri] = ci

    # Phase 3: Recursively compare matched pairs
    for ci in sorted(c_matched.keys()):
        ri = c_matched[ci]
        c_node = c_children[ci]
        r_node = r_children[ri]
        _compare_scope(c_node, r_node,
                       f"{path}/{c_node.kind}({c_node.label[:20]})",
                       mismatches)

    # Phase 4: Report unmatched C scopes
    for ci, c_node in enumerate(c_children):
        if ci in c_matched:
            continue
        if _should_drop(c_node, c_drops):
            continue
        mismatches.append(Mismatch(
            "extra_c_scope",
            f"C has {c_node.kind}({c_node.label[:40]}) with no Rust equivalent",
            c_line=c_node.line, path=path,
        ))

    # Phase 5: Report unmatched Rust scopes
    for ri, r_node in enumerate(r_children):
        if ri in r_matched:
            continue
        if _should_drop(r_node, r_drops):
            continue
        mismatches.append(Mismatch(
            "extra_r_scope",
            f"Rust has {r_node.kind}({r_node.label[:40]}) with no C equivalent",
            r_line=r_node.line, path=path,
        ))


# ========= Maximum bipartite matching =========

def _find_max_matching(c_items: list, r_items: list, can_match) -> list:
    """Find maximum matching between C and R scope lists.

    Uses augmenting paths (Hopcroft-Karp simplified for small sets).
    Returns list of (c_idx, r_idx) pairs.
    """
    # Build adjacency: for each c_item, which r_items can it match?
    adj = {}
    for ci, c_node in c_items:
        adj[ci] = []
        for ri, r_node in r_items:
            if can_match(c_node, r_node):
                adj[ci].append(ri)

    # Hungarian algorithm (simplified): try to find augmenting path for each C node
    r_to_c = {}  # r_idx → c_idx (current matching)

    def augment(ci, visited):
        for ri in adj.get(ci, []):
            if ri in visited:
                continue
            visited.add(ri)
            # If r is unmatched, or we can find an augmenting path
            if ri not in r_to_c or augment(r_to_c[ri], visited):
                r_to_c[ri] = ci
                return True
        return False

    for ci, _ in c_items:
        augment(ci, set())

    # Convert to (c_idx, r_idx) pairs
    return [(c_idx, r_idx) for r_idx, c_idx in r_to_c.items()]


# ========= Scope matching =========

def _scopes_match(c: ScopeNode, r: ScopeNode) -> bool:
    """Do two scoping nodes correspond by identity?"""
    if c.kind == r.kind:
        if c.kind == "arm":
            return _arm_labels_match(c.label, r.label)
        if c.kind in ("if", "if_let"):
            return _condition_labels_overlap(c.label, r.label)
        if c.kind == "match":
            return True  # Matches pair by position within parent
        if c.kind == "loop":
            return True
        return True

    # Cross-kind: C if = Rust if_let (handler dispatch pattern)
    if c.kind == "if" and r.kind == "if_let":
        return _condition_labels_overlap(c.label, r.label)
    if c.kind == "if_let" and r.kind == "if":
        return _condition_labels_overlap(c.label, r.label)

    return False


def _condition_labels_overlap(c_label: str, r_label: str) -> bool:
    """Do two condition labels share any meaningful identifiers?"""
    if not c_label or not r_label:
        return True  # Can't compare — assume match
    c_ids = set(c_label.replace(",", " ").split())
    r_ids = set(r_label.replace(",", " ").split())
    # Strip m_ prefix from C identifiers
    c_normalized = {i.lstrip("m_") if i.startswith("m_") else i for i in c_ids}
    r_normalized = set(r_ids)
    # Check overlap
    if c_normalized & r_normalized:
        return True
    # Case-insensitive
    c_lower = {i.lower().replace("_", "") for i in c_normalized}
    r_lower = {i.lower().replace("_", "") for i in r_normalized}
    return bool(c_lower & r_lower)


def _arm_labels_match(c_label: str, r_label: str) -> bool:
    """Do two arm labels match?"""
    if c_label == r_label:
        return True
    # Default/wildcard arms
    if c_label in ("_default", "default") and r_label in ("_", "_default", "default"):
        return True
    if r_label in ("_default", "default") and c_label in ("_", "_default", "default"):
        return True
    c_short = c_label.split("::")[-1] if "::" in c_label else c_label
    r_short = r_label.split("::")[-1] if "::" in r_label else r_label
    if c_short == r_short:
        return True
    # Case-insensitive (C ALL_CAPS vs Rust PascalCase)
    if c_short.replace("_", "").lower() == r_short.replace("_", "").lower():
        return True
    # Prefix match: "Ok" matches "Ok(TokenResult ...)"
    if r_label.startswith(f"{c_label}(") or c_label.startswith(f"{r_label}("):
        return True
    return False


def _pair_arms(c_arms: list, r_arms: list,
               c_matched: dict, r_matched: dict):
    """Pair match arms by label."""
    r_arm_map = {}
    for ri, r_node in r_arms:
        if r_node.label:
            for sub in r_node.label.split("|"):
                r_arm_map[sub.strip()] = (ri, r_node)

    for ci, c_node in c_arms:
        if not c_node.label:
            continue
        # Find matching Rust arm
        for r_label, (ri, r_node) in r_arm_map.items():
            if ri in r_matched:
                continue
            if _arm_labels_match(c_node.label, r_label):
                c_matched[ci] = ri
                r_matched[ri] = ci
                break


# ========= Equivalence rules =========

def _equiv_rule_matches(c: ScopeNode, r: ScopeNode, rule: dict) -> bool:
    """Does a scope equivalence rule match this C/R pair?"""
    c_pat = rule.get("c_scope", {})
    r_pat = rule.get("r_scope", {})
    if not c_pat or not r_pat:
        return False
    return _scope_matches_pattern(c, c_pat) and _scope_matches_pattern(r, r_pat)


def _scope_matches_pattern(scope: ScopeNode, pattern: dict) -> bool:
    """Does a scope node match a pattern from a rule?"""
    if "kind" in pattern:
        if pattern["kind"] != scope.kind:
            return False
    if "label_contains" in pattern:
        if pattern["label_contains"] not in scope.label:
            return False
    if "label_regex" in pattern:
        if not re.search(pattern["label_regex"], scope.label):
            return False
    if "has_child_label" in pattern:
        # Match if ANY child has a label containing the pattern
        if not any(pattern["has_child_label"] in c.label for c in scope.children):
            return False
    return True


# ========= Drop rules =========

def _should_drop(scope: ScopeNode, drop_rules: list) -> bool:
    """Should this unmatched scope be dropped (not reported)?"""
    for rule in drop_rules:
        if _scope_matches_pattern(scope, rule):
            return True
    return False


# ========= Content comparison =========

def _compare_content(c_content: ContentSet, r_content: ContentSet,
                     path: str, c_scope: ScopeNode, r_scope: ScopeNode,
                     mismatches: list[Mismatch]):
    """Compare content sets within a matched scope pair."""
    rules = _rules or {}
    skip_c_calls = set(rules.get("skip_c_calls", []))
    skip_c_idents = set(rules.get("skip_c_identifiers", []))
    skip_r_calls = set(rules.get("skip_r_calls", []))
    c_to_r_call_map = rules.get("call_name_map", {})
    c_to_r_ident_map = rules.get("identifier_map", {})
    skip_c_errors = set(rules.get("skip_c_errors", []))

    # Normalize Rust calls through mapping
    r_calls_normalized = {}
    for r_call, r_args in r_content.calls.items():
        normalized = c_to_r_call_map.get(r_call, r_call)
        r_calls_normalized.setdefault(normalized, set()).update(r_args)

    # Also collect all calls from child scopes (for cross-level matching)
    r_all_calls = set(r_calls_normalized.keys())
    for call in _collect_all_calls_normalized(r_scope, c_to_r_call_map):
        r_all_calls.add(call)

    # --- Compare calls ---
    for c_call, c_args in c_content.calls.items():
        if c_call in skip_c_calls:
            continue
        r_call = c_to_r_call_map.get(c_call, c_call)
        if r_call in r_calls_normalized:
            # Call found — compare args
            r_args = r_calls_normalized[r_call]
            mapped_c_args = set()
            for arg in c_args:
                if arg in skip_c_idents:
                    continue
                mapped = c_to_r_ident_map.get(arg, arg)
                mapped_c_args.add(mapped)
            missing_args = mapped_c_args - r_args - skip_c_idents
            if missing_args:
                mismatches.append(Mismatch(
                    "missing_call_arg",
                    f"Call {c_call}(): C args {sorted(missing_args)} not in Rust",
                    c_scope.line, r_scope.line, path,
                ))
        elif r_call in r_all_calls:
            pass  # Call exists at a different scope level
        else:
            mismatches.append(Mismatch(
                "missing_call",
                f"C calls {c_call}() but Rust doesn't",
                c_scope.line, r_scope.line, path,
            ))

    # --- Compare error returns ---
    for c_error in c_content.error_returns:
        if c_error in skip_c_errors:
            continue
        if c_error not in r_content.error_returns:
            # Try normalized
            c_lower = c_error.lower()
            if not any(r.lower() == c_lower for r in r_content.error_returns):
                mismatches.append(Mismatch(
                    "missing_error_return",
                    f"C returns {c_error} but Rust doesn't",
                    c_scope.line, r_scope.line, path,
                ))


def _collect_all_calls_normalized(scope: ScopeNode, call_map: dict) -> set[str]:
    """Collect all call names from a scope and descendants, normalized."""
    calls = set()
    for call in scope.content.calls:
        calls.add(call_map.get(call, call))
    for child in scope.children:
        calls.update(_collect_all_calls_normalized(child, call_map))
    return calls
