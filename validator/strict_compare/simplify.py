"""Apply simplification rules to common AST trees.

Simplification rules are always applied to both C and Rust trees.
They normalize syntax differences without losing semantic information.

Rules are loaded from JSON and applied bottom-up (leaves first).
Each rule matches a tree pattern and replaces it with a simpler form.

Example rules:
- C field_expression(parser, m_commentHandler) → field(self, comment_handler)
- camelCase identifiers → snake_case
- XML_TOK_X → XmlTok::X
- C break in switch → deleted (Rust match arms don't need break)
"""

import re
import json
import os
from .common_ast import Node


_rules = None


def load_rules(config_file: str = None):
    """Load simplification rules from JSON config."""
    global _rules
    if config_file is None:
        config_file = os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            "..", "simplification-rules.json"
        )
    with open(config_file) as f:
        config = json.load(f)
    _rules = config.get("simplification_rules", [])
    return _rules


def apply_simplifications(tree: Node, lang: str) -> Node:
    """Apply all simplification rules to a common AST tree.

    Args:
        tree: Common AST root node
        lang: "c" or "rust" — rules can be language-specific

    Returns:
        Simplified tree (new tree, original is not modified)
    """
    if _rules is None:
        load_rules()

    # Apply bottom-up
    return _simplify(tree, lang)


def _simplify(node: Node, lang: str) -> Node:
    """Recursively simplify a node and its children."""
    # First simplify children
    new_children = []
    for child in node.children:
        simplified = _simplify(child, lang)
        if simplified is not None:  # Rules can delete nodes
            new_children.append(simplified)

    # Rebuild if children changed
    if new_children != node.children:
        node = Node(kind=node.kind, children=new_children,
                     value=node.value, source_file=node.source_file,
                     line=node.line)

    # Apply rules to this node
    for rule in _rules:
        # Check language filter
        rule_lang = rule.get("lang", "both")
        if rule_lang != "both" and rule_lang != lang:
            continue

        result = _apply_rule(node, rule)
        if result is None:
            return None  # Node deleted
        if result is not node:
            node = result

    return node


def _apply_rule(node: Node, rule: dict) -> Node | None:
    """Apply a single simplification rule to a node.

    Returns None if the node should be deleted, the modified node if
    transformed, or the original node if the rule doesn't match.
    """
    action = rule.get("action", "")
    match = rule.get("match", {})

    if not _matches(node, match):
        return node

    if action == "delete":
        return None

    if action == "rename_value":
        new_value = _apply_rename(node.value, rule)
        if new_value != node.value:
            return Node(kind=node.kind, children=node.children,
                         value=new_value, source_file=node.source_file,
                         line=node.line)
        return node

    if action == "replace_kind":
        new_kind = rule.get("new_kind", node.kind)
        return Node(kind=new_kind, children=node.children,
                     value=node.value, source_file=node.source_file,
                     line=node.line)

    if action == "unwrap":
        # Replace this node with its first child
        if node.children:
            return node.children[0]
        return node

    if action == "replace_tree":
        # Replace this node with a specified tree structure
        return _build_replacement(rule.get("replacement", {}), node)

    return node


def _matches(node: Node, match: dict) -> bool:
    """Check if a node matches a rule's match criteria."""
    if "kind" in match:
        if match["kind"] != node.kind:
            return False

    if "value" in match:
        if match["value"] != node.value:
            return False

    if "value_regex" in match:
        if not re.search(match["value_regex"], node.value):
            return False

    if "has_child_kind" in match:
        if not any(c.kind == match["has_child_kind"] for c in node.children):
            return False

    if "child_count" in match:
        if len(node.children) != match["child_count"]:
            return False

    if "child_value" in match:
        # Check if any direct child has this value
        idx = match.get("child_index", 0)
        if idx < len(node.children):
            if node.children[idx].value != match["child_value"]:
                return False
        else:
            return False

    return True


def _apply_rename(value: str, rule: dict) -> str:
    """Apply a rename transformation to a node value."""
    rename_map = rule.get("rename_map", {})
    if value in rename_map:
        return rename_map[value]

    rename_regex = rule.get("rename_regex")
    rename_replace = rule.get("rename_replace")
    if rename_regex and rename_replace:
        return re.sub(rename_regex, rename_replace, value)

    transform = rule.get("transform", "")
    if transform == "camel_to_snake":
        return _camel_to_snake(value)

    if transform == "strip_prefix":
        prefix = rule.get("prefix", "")
        if value.startswith(prefix):
            return value[len(prefix):]

    return value


def _build_replacement(spec: dict, original: Node) -> Node:
    """Build a replacement node from a spec, preserving source info."""
    kind = spec.get("kind", original.kind)
    value = spec.get("value", original.value)
    children = []
    for child_spec in spec.get("children", []):
        if child_spec == "$children":
            children.extend(original.children)
        elif child_spec == "$self":
            children.append(Node("self"))
        elif isinstance(child_spec, dict):
            children.append(_build_replacement(child_spec, original))
        elif isinstance(child_spec, str) and child_spec.startswith("$child_"):
            idx = int(child_spec[7:])
            if idx < len(original.children):
                children.append(original.children[idx])
    return Node(kind=kind, children=children, value=value,
                 source_file=original.source_file, line=original.line)


def _camel_to_snake(name: str) -> str:
    """Convert camelCase to snake_case."""
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    return s.lower()
