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
    """Load match rules from JSON config."""
    global _match_rules
    if config_file is None:
        config_file = os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            "..", "match-rules.json"
        )
    with open(config_file) as f:
        config = json.load(f)
    _match_rules = config.get("match_rules", [])
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

    # Exact match: same kind, value, and same number of children
    if c.kind == r.kind and c.value == r.value and len(c.children) == len(r.children):
        # Recursively compare children
        for i, (cc, rc) in enumerate(zip(c.children, r.children)):
            _compare_nodes(cc, rc, f"{path}/{c.kind}[{i}]", mismatches)
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
    elif len(c.children) != len(r.children):
        mismatches.append(Mismatch(
            c, r,
            f"Children count mismatch: C {c.kind} has {len(c.children)} children, "
            f"Rust has {len(r.children)} children",
            path,
        ))
    else:
        mismatches.append(Mismatch(
            c, r,
            f"Structural mismatch at {c.kind}({c.value})",
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

    Handles rest captures ({"capture": "rest", "rest": true}) which
    match zero or more remaining children.
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
                # Capture as a block node containing the remaining children
                captures[capture_name] = Node("block", children=remaining)
            return True  # Rest always succeeds

        if c_idx >= len(children):
            return False  # Ran out of children

        if not _pattern_matches(children[c_idx], pattern, captures):
            return False

        c_idx += 1
        p_idx += 1

    # All patterns matched — check for leftover children
    return c_idx == len(children)
