"""Strict top-down AST comparison with match rules.

Compares two common ASTs (C and Rust) node-by-node from the top down.
At each pair of nodes:

1. If the nodes are identical (same kind, value, and children), they match.
2. If they differ, check if any match rule applies — a rule that says
   "this C pattern is equivalent to this Rust pattern". If a rule matches,
   recursively compare the captured variables.
3. If no rule applies, report a mismatch.

This guarantees: if the comparison passes, every C AST node has a
corresponding Rust AST node (and vice versa), modulo the match rules.
Extra nodes on either side are errors.
"""

import re
import json
import os
from .common_ast import Node, MatchRule, Pattern


class Mismatch:
    """A structural mismatch between C and Rust ASTs."""
    def __init__(self, c_node: Node, r_node: Node, reason: str, path: str = ""):
        self.c_node = c_node
        self.r_node = r_node
        self.reason = reason
        self.path = path

    def __repr__(self):
        loc = ""
        if self.c_node and self.c_node.line:
            loc += f" C@{self.c_node.line}"
        if self.r_node and self.r_node.line:
            loc += f" R@{self.r_node.line}"
        path = f" [{self.path}]" if self.path else ""
        return f"[MISMATCH]{path}{loc}: {self.reason}"


_match_rules: list[dict] = None


def load_match_rules(config_file: str = None):
    """Load match rules from YAML config."""
    global _match_rules
    from .parse_match_rules import parse_rules_file
    _match_rules = parse_rules_file(config_file)
    return _match_rules


def compare(c_tree: Node, r_tree: Node, path: str = "") -> list[Mismatch]:
    """Compare two common ASTs strictly, using match rules for allowed differences.

    Returns a list of mismatches. Empty list = trees are equivalent.
    """
    if _match_rules is None:
        load_match_rules()

    mismatches = []
    _compare_nodes(c_tree, r_tree, path, mismatches)
    return mismatches


def _compare_nodes(c: Node, r: Node, path: str,
                   mismatches: list[Mismatch]):
    """Compare two nodes strictly."""

    # Exact match: same kind, value
    if c.kind == r.kind and c.value == r.value:
        # For block/sequence nodes, use alignment-based child comparison
        if c.kind in ("block", "match") and c.children and r.children:
            _compare_children_aligned(c.children, r.children,
                                       f"{path}/{c.kind}", mismatches)
            return
        # For other nodes, require exact children count
        if len(c.children) == len(r.children):
            for i, (cc, rc) in enumerate(zip(c.children, r.children)):
                _compare_nodes(cc, rc, f"{path}/{c.kind}[{i}]", mismatches)
            return
        # Same kind/value but different children count — try alignment
        _compare_children_aligned(c.children, r.children,
                                   f"{path}/{c.kind}", mismatches)
        return

    # Try match rules
    for rule in _match_rules:
        captures = {}
        if _try_match_rule(c, r, rule, captures):
            # Rule matched — recursively compare captured variables
            for var_name, (c_captured, r_captured) in captures.items():
                _compare_nodes(c_captured, r_captured,
                               f"{path}/{rule.get('name', '?')}/${var_name}",
                               mismatches)
            return

    # No match — report mismatch
    _report_mismatch(c, r, path, mismatches)


def _compare_children_aligned(c_children: list[Node], r_children: list[Node],
                               path: str, mismatches: list[Mismatch]):
    """Compare child lists using alignment (like a diff algorithm).

    Walks both lists trying to match corresponding children:
    1. If current C and R children match (or a match rule applies), recurse
    2. If C child can be skipped (a "c_only" rule says it has no Rust equivalent), skip it
    3. If R child can be skipped (a "r_only" rule says it has no C equivalent), skip it
    4. Otherwise, report mismatch and advance both

    This handles different numbers of children, extra nodes on either side,
    and reordered nodes (within limits).
    """
    c_idx = 0
    r_idx = 0

    while c_idx < len(c_children) and r_idx < len(r_children):
        c_node = c_children[c_idx]
        r_node = r_children[r_idx]

        # Try direct match
        test_mismatches = []
        _compare_nodes(c_node, r_node, f"{path}[{c_idx},{r_idx}]", test_mismatches)
        if not test_mismatches:
            c_idx += 1
            r_idx += 1
            continue

        # Try match rules
        rule_matched = False
        for rule in _match_rules:
            captures = {}
            if _try_match_rule(c_node, r_node, rule, captures):
                for var_name, (c_cap, r_cap) in captures.items():
                    _compare_nodes(c_cap, r_cap,
                                    f"{path}/{rule.get('name', '?')}/${var_name}",
                                    mismatches)
                c_idx += 1
                r_idx += 1
                rule_matched = True
                break
        if rule_matched:
            continue

        # Try skipping C child (C-only node)
        if _can_skip(c_node, "c"):
            c_idx += 1
            continue

        # Try skipping R child (Rust-only node)
        if _can_skip(r_node, "r"):
            r_idx += 1
            continue

        # Try lookahead: maybe C child matches a later R child
        found_later = False
        for r_look in range(r_idx + 1, min(r_idx + 5, len(r_children))):
            test = []
            _compare_nodes(c_node, r_children[r_look], "", test)
            if not test:
                # C matches later R — skip intervening R children
                for r_skip in range(r_idx, r_look):
                    if not _can_skip(r_children[r_skip], "r"):
                        mismatches.append(Mismatch(
                            None, r_children[r_skip],
                            f"Extra Rust node: {r_children[r_skip].kind}({r_children[r_skip].value})",
                            f"{path}[{r_skip}]",
                        ))
                _compare_nodes(c_node, r_children[r_look],
                                f"{path}[{c_idx},{r_look}]", mismatches)
                c_idx += 1
                r_idx = r_look + 1
                found_later = True
                break
        if found_later:
            continue

        # Try lookahead other direction: R child matches a later C child
        found_later = False
        for c_look in range(c_idx + 1, min(c_idx + 5, len(c_children))):
            test = []
            _compare_nodes(c_children[c_look], r_node, "", test)
            if not test:
                for c_skip in range(c_idx, c_look):
                    if not _can_skip(c_children[c_skip], "c"):
                        mismatches.append(Mismatch(
                            c_children[c_skip], None,
                            f"Extra C node: {c_children[c_skip].kind}({c_children[c_skip].value})",
                            f"{path}[{c_skip}]",
                        ))
                _compare_nodes(c_children[c_look], r_node,
                                f"{path}[{c_look},{r_idx}]", mismatches)
                c_idx = c_look + 1
                r_idx += 1
                found_later = True
                break
        if found_later:
            continue

        # No match possible — report and advance both
        mismatches.append(Mismatch(
            c_node, r_node,
            f"Unmatched pair: C {c_node.kind}({c_node.value}) vs R {r_node.kind}({r_node.value})",
            f"{path}[{c_idx},{r_idx}]",
        ))
        c_idx += 1
        r_idx += 1

    # Report remaining unmatched children
    while c_idx < len(c_children):
        c_node = c_children[c_idx]
        if not _can_skip(c_node, "c"):
            mismatches.append(Mismatch(
                c_node, None,
                f"Extra C node: {c_node.kind}({c_node.value})",
                f"{path}[{c_idx}]",
            ))
        c_idx += 1

    while r_idx < len(r_children):
        r_node = r_children[r_idx]
        if not _can_skip(r_node, "r"):
            mismatches.append(Mismatch(
                None, r_node,
                f"Extra Rust node: {r_node.kind}({r_node.value})",
                f"{path}[{r_idx}]",
            ))
        r_idx += 1


def _can_skip(node: Node, lang: str) -> bool:
    """Check if a node can be skipped (has a skip rule)."""
    for rule in _match_rules:
        skip_lang = rule.get("skip")
        if skip_lang and skip_lang == lang:
            pattern = rule.get("c_pattern" if lang == "c" else "r_pattern", {})
            if _pattern_matches(node, pattern, {}):
                return True
    return False


def _report_mismatch(c: Node, r: Node, path: str, mismatches: list[Mismatch]):
    """Report a mismatch with a descriptive message."""
    if c.kind != r.kind:
        mismatches.append(Mismatch(
            c, r,
            f"Kind mismatch: C has {c.kind}({c.value}), Rust has {r.kind}({r.value})",
            path,
        ))
    elif c.value != r.value:
        mismatches.append(Mismatch(
            c, r,
            f"Value mismatch: C has {c.kind}({c.value}), Rust has {r.kind}({r.value})",
            path,
        ))
    else:
        mismatches.append(Mismatch(
            c, r,
            f"Structural mismatch at {c.kind}({c.value}): "
            f"C has {len(c.children)} children, Rust has {len(r.children)}",
            path,
        ))


def _try_match_rule(c: Node, r: Node, rule: dict,
                    captures: dict[str, tuple[Node, Node]]) -> bool:
    """Try to apply a match rule to a C/Rust node pair.

    A match rule has:
      c_pattern: pattern to match against the C node
      r_pattern: pattern to match against the Rust node
      captures: variable names that must match recursively

    Returns True if the rule applies and all captured variables can be paired.
    """
    c_pattern = rule.get("c_pattern", {})
    r_pattern = rule.get("r_pattern", {})

    c_captures = {}
    r_captures = {}

    if not _pattern_matches(c, c_pattern, c_captures):
        return False
    if not _pattern_matches(r, r_pattern, r_captures):
        return False

    # Pair up captured variables by name
    all_vars = set(c_captures.keys()) | set(r_captures.keys())
    for var in all_vars:
        c_val = c_captures.get(var)
        r_val = r_captures.get(var)
        if c_val is not None and r_val is not None:
            captures[var] = (c_val, r_val)
        # Variables captured on only one side are free (don't need to match)

    return True


def _pattern_matches(node: Node, pattern: dict,
                     captures: dict[str, Node]) -> bool:
    """Check if a common AST node matches a pattern.

    Pattern format (JSON):
    {
      "kind": "call",              — exact kind match
      "value": "foo",              — exact value match
      "value_regex": ".*_tok$",    — regex value match
      "children": [                — child patterns (positional)
        {"kind": "ident", "capture": "func"},
        {"capture": "args", "rest": true}
      ],
      "capture": "var_name"        — capture this node as $var_name
    }
    """
    if not pattern:
        return True  # Empty pattern matches anything

    # Kind match
    if "kind" in pattern:
        if pattern["kind"] != node.kind:
            return False

    # Value match
    if "value" in pattern:
        if pattern["value"] != node.value:
            return False

    # Value regex match
    if "value_regex" in pattern:
        if not re.search(pattern["value_regex"], node.value):
            return False

    # Children match
    if "children" in pattern:
        child_patterns = pattern["children"]
        if not _match_children(node.children, child_patterns, captures):
            return False

    # Capture this node
    if "capture" in pattern:
        captures[pattern["capture"]] = node

    return True


def _match_children(children: list[Node], patterns: list[dict],
                    captures: dict[str, Node]) -> bool:
    """Match a list of children against a list of patterns.

    Handles:
    - Positional matching: patterns match children in order
    - Rest captures: {"capture": "rest", "rest": true} matches remaining
    - Partial matching: if patterns don't cover all children, the
      remaining children are ignored (patterns are a prefix match)
    """
    c_idx = 0
    p_idx = 0

    while p_idx < len(patterns):
        pattern = patterns[p_idx]

        if pattern.get("rest"):
            # Rest capture: match all remaining children
            capture_name = pattern.get("capture", "")
            remaining = children[c_idx:]
            if capture_name:
                captures[capture_name] = Node("block", children=remaining)
            return True

        if c_idx >= len(children):
            return False  # Ran out of children

        if not _pattern_matches(children[c_idx], pattern, captures):
            return False

        c_idx += 1
        p_idx += 1

    # All patterns matched — remaining children are OK (prefix match)
    return True
