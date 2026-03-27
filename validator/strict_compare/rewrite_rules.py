"""Rewrite rules loaded from structural-rewrites.json.

Each rule has:
- input: what to match in the C skeleton
- output: what it becomes (null = delete)
- justification: why this rewrite is correct

Rules are applied bottom-up (leaves first) to the C skeleton.
"""

import re
import json
import os
from .nodes import SkeletonNode

_REWRITES_FILE = os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..", "structural-rewrites.json"
)
_TEMP_REWRITES_FILE = os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..", "temporary-rewrites.json"
)

_loaded_rules = None
_loaded_suppressions = None


def _load_config():
    global _loaded_rules, _loaded_suppressions
    if _loaded_rules is not None:
        return
    with open(_REWRITES_FILE) as f:
        config = json.load(f)
    _loaded_rules = config.get("rewrite_rules", [])
    _loaded_suppressions = config.get("per_function_suppressions", {})

    # Also load temporary rules (marked with temporary=True)
    try:
        with open(_TEMP_REWRITES_FILE) as f:
            temp_config = json.load(f)
        for rule in temp_config.get("temporary_rules", []):
            rule["_temporary"] = True
            _loaded_rules.append(rule)
    except (FileNotFoundError, json.JSONDecodeError):
        pass


def get_per_function_suppressions(func_name: str) -> dict:
    """Get per-function suppression config."""
    _load_config()
    return _loaded_suppressions.get(func_name, {})


def _pattern_matches(node: SkeletonNode, pattern: dict) -> bool:
    """Check if a skeleton node matches a JSON rule pattern."""
    # Kind match
    if "kind" in pattern:
        if pattern["kind"] != node.kind:
            return False

    # Exact label match
    if "label" in pattern:
        expected = pattern["label"]
        if expected == "*":
            pass  # wildcard, matches anything
        elif expected.startswith("$"):
            pass  # capture variable, matches anything
        elif expected.startswith("XmlError::$"):
            if not node.label.startswith("XmlError::"):
                return False
        elif expected != node.label:
            return False

    # Regex label match
    if "label_regex" in pattern:
        if not re.search(pattern["label_regex"], node.label):
            return False

    return True


def _apply_rule(node: SkeletonNode, rule: dict) -> SkeletonNode | None:
    """Apply a single rule to a node. Returns None if node should be deleted,
    returns modified node if transformed, returns original if rule doesn't match."""
    input_pattern = rule.get("input", {})
    output_pattern = rule.get("output")

    if not _pattern_matches(node, input_pattern):
        return node  # No match, return unchanged

    # Check additional conditions
    condition = rule.get("condition", "")
    if condition == "children contain only return(error) or return(0)":
        if not _children_are_error_returns(node):
            return node

    # Rule matched
    if output_pattern is None:
        return None  # Delete this node

    # Transform: apply output pattern
    new_node = SkeletonNode(
        kind=output_pattern.get("kind", node.kind),
        label=_apply_label_template(output_pattern.get("label", node.label), node),
        args=node.args,
        children=node.children,
        source_file=node.source_file,
        source_start=node.source_start,
        source_end=node.source_end,
    )
    return new_node


def _apply_label_template(template: str, node: SkeletonNode) -> str:
    """Apply a label template with variable substitution."""
    if template.startswith("$"):
        return node.label  # Capture variable, keep original
    if "$" in template:
        # Replace $error etc with actual value from node.label
        return node.label  # For now, preserve original
    return template


def _children_are_error_returns(node: SkeletonNode) -> bool:
    """Check if all children of a branch are error returns."""
    for child in node.children:
        if child.kind == "return":
            if child.label in ("error", "0", "ok", "1"):
                continue
            if child.label.startswith("XmlError::"):
                continue
            return False
        if child.kind == "sequence":
            if not _children_are_error_returns(child):
                return False
            continue
        return False
    return True


def apply_all_rules(skeleton: SkeletonNode) -> SkeletonNode | None:
    """Apply all rewrite rules bottom-up to a C skeleton."""
    _load_config()

    # First apply to children recursively
    new_children = []
    for child in skeleton.children:
        rewritten = apply_all_rules(child)
        if rewritten is not None:
            new_children.append(rewritten)

    skeleton = SkeletonNode(
        kind=skeleton.kind,
        label=skeleton.label,
        args=skeleton.args,
        children=new_children,
        source_file=skeleton.source_file,
        source_start=skeleton.source_start,
        source_end=skeleton.source_end,
    )

    # Apply each rule in order
    for rule in _loaded_rules:
        result = _apply_rule(skeleton, rule)
        if result is None:
            return None  # Node deleted
        skeleton = result

    # Flatten single-child sequences
    if skeleton.kind == "sequence" and len(skeleton.children) == 1:
        child = skeleton.children[0]
        if not child.source_start:
            child.source_start = skeleton.source_start
            child.source_end = skeleton.source_end
        return child

    return skeleton
