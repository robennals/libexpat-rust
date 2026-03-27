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
_loaded_expr_rewrites = None
_loaded_sequence_rewrites = None


def _load_config():
    global _loaded_rules, _loaded_suppressions, _loaded_expr_rewrites, _loaded_sequence_rewrites
    if _loaded_rules is not None:
        return
    with open(_REWRITES_FILE) as f:
        config = json.load(f)
    _loaded_rules = config.get("rewrite_rules", [])
    _loaded_suppressions = config.get("per_function_suppressions", {})
    _loaded_expr_rewrites = config.get("expression_rewrites", [])
    # Load tree rewrite rules (text DSL format)
    raw_tree_rules = config.get("tree_rewrite_rules", [])
    from .tree_pattern import compile_text_rules
    _loaded_sequence_rewrites = compile_text_rules(raw_tree_rules)

    # Also load raw dict-format sequence rules (legacy)
    _loaded_sequence_rewrites.extend(config.get("sequence_rewrite_rules", []))

    # Also load temporary rules (marked with temporary=True)
    try:
        with open(_TEMP_REWRITES_FILE) as f:
            temp_config = json.load(f)
        for rule in temp_config.get("temporary_rules", []):
            rule["_temporary"] = True
            _loaded_rules.append(rule)
        for rule in temp_config.get("temporary_expression_rewrites", []):
            rule["_temporary"] = True
            _loaded_expr_rewrites.append(rule)
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

    # Children count match
    if "children_count" in pattern:
        if len(node.children) != pattern["children_count"]:
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

    # Apply sequence rewrite rules to children
    skeleton = _apply_sequence_rewrites(skeleton)

    # Flatten single-child sequences
    if skeleton.kind == "sequence" and len(skeleton.children) == 1:
        child = skeleton.children[0]
        if not child.source_start:
            child.source_start = skeleton.source_start
            child.source_end = skeleton.source_end
        return child

    return skeleton


# ========= Tree rewrite rules (multi-node pattern matching) =========

def _apply_sequence_rewrites(skeleton: SkeletonNode) -> SkeletonNode:
    """Apply tree rewrite rules that match and replace arbitrary subtrees.

    Tree rules match a pattern against the skeleton tree (at any depth)
    and replace the matched subtree with a new tree, substituting captured
    nodes. This handles structural patterns like:

      C pattern:     [call(*_tok), match(tok) { arms... }]
      Rust pattern:  [match(tok_result) { arm(Ok) { match(tok) { arms... } } }]

    Rules are applied bottom-up (leaves first), repeatedly until fixpoint.
    """
    _load_config()
    if not _loaded_sequence_rewrites:
        return skeleton

    # Apply repeatedly until no rule fires
    changed = True
    iterations = 0
    while changed and iterations < 10:
        changed = False
        iterations += 1
        for rule in _loaded_sequence_rewrites:
            result = _apply_tree_rule(skeleton, rule)
            if result is not skeleton:
                skeleton = result
                changed = True
                break  # Restart from first rule

    return skeleton


def _apply_tree_rule(skeleton: SkeletonNode,
                     rule: dict) -> SkeletonNode:
    """Try to apply a tree rewrite rule at every position in the skeleton.

    First recurses into children (bottom-up), then tries to match at
    the current node level. If a match is found, builds the output tree.
    """
    # First recurse into children
    new_children = []
    any_child_changed = False
    for child in skeleton.children:
        rewritten = _apply_tree_rule(child, rule)
        new_children.append(rewritten)
        if rewritten is not child:
            any_child_changed = True

    if any_child_changed:
        skeleton = SkeletonNode(
            kind=skeleton.kind, label=skeleton.label,
            args=skeleton.args, arg_exprs=skeleton.arg_exprs,
            expr=skeleton.expr, children=new_children,
            source_file=skeleton.source_file,
            source_start=skeleton.source_start,
            source_end=skeleton.source_end,
        )

    # Try to match the rule's input pattern against this node's children
    # (sequence-level matching: match a contiguous subsequence of children)
    input_pattern = rule.get("input")
    output_pattern = rule.get("output")
    if not input_pattern or output_pattern is None:
        return skeleton

    # Case 1: The input pattern matches the node itself (requires kind in pattern)
    if "kind" in input_pattern:
        captures = {}
        if _tree_matches(skeleton, input_pattern, captures):
            return _build_tree(output_pattern, captures)

    # Case 2: The input pattern has "children" — match against a contiguous
    # subsequence of this node's children
    if "children" in input_pattern and skeleton.children:
        child_patterns = input_pattern["children"]
        if not isinstance(child_patterns, list):
            return skeleton

        result = _match_and_replace_in_children(
            skeleton, child_patterns, output_pattern, rule
        )
        if result is not None:
            return result

    return skeleton


def _match_and_replace_in_children(
    parent: SkeletonNode,
    child_patterns: list[dict],
    output_pattern: dict,
    rule: dict,
) -> SkeletonNode | None:
    """Try to match child_patterns as a contiguous subsequence of parent's children.

    If matched, replace the subsequence with the output pattern.
    """
    pattern_len = len(child_patterns)
    children = parent.children
    if pattern_len > len(children):
        return None

    for start_idx in range(len(children) - pattern_len + 1):
        captures = {}
        matched = True
        for j, pattern in enumerate(child_patterns):
            child = children[start_idx + j]
            if not _tree_matches(child, pattern, captures):
                matched = False
                break

        if not matched:
            continue

        # Check parent-level constraints (kind, label) from the input pattern
        parent_kind = rule.get("input", {}).get("kind")
        parent_label_regex = rule.get("input", {}).get("label_regex")
        if parent_kind and parent_kind != parent.kind:
            continue
        if parent_label_regex and not re.search(parent_label_regex, parent.label):
            continue

        # Build output — the output_pattern may be:
        # 1. A capture reference {"capture_ref": "name"} → substitute the captured node
        # 2. A tree spec with kind/label/children → build new tree with captures
        if "capture_ref" in output_pattern:
            # Direct capture substitution — replace matched children with captured node
            captured = captures.get(output_pattern["capture_ref"])
            if captured:
                new_children = children[:start_idx] + [captured] + children[start_idx + pattern_len:]
            else:
                new_children = children[:start_idx] + children[start_idx + pattern_len:]
            return SkeletonNode(
                kind=parent.kind, label=parent.label,
                args=parent.args, arg_exprs=parent.arg_exprs,
                expr=parent.expr, children=new_children,
                source_file=parent.source_file,
                source_start=parent.source_start,
                source_end=parent.source_end,
            )

        # Build output nodes from tree spec
        output_children_spec = output_pattern.get("children", [])
        output_nodes = []
        for spec in output_children_spec:
            node = _build_tree(spec, captures)
            if node:
                output_nodes.append(node)

        # If output has no children spec but has kind, treat as a wrapper
        if not output_children_spec and "kind" in output_pattern:
            # The output is a single node wrapping captured content
            output_node = _build_tree(output_pattern, captures)
            if output_node:
                output_nodes = [output_node]

        # Replace the matched subsequence
        new_children = children[:start_idx] + output_nodes + children[start_idx + pattern_len:]

        return SkeletonNode(
            kind=parent.kind, label=parent.label,
            args=parent.args, arg_exprs=parent.arg_exprs,
            expr=parent.expr, children=new_children,
            source_file=parent.source_file,
            source_start=parent.source_start,
            source_end=parent.source_end,
        )

    return None


def _tree_matches(node: SkeletonNode, pattern: dict,
                  captures: dict[str, SkeletonNode]) -> bool:
    """Does a skeleton node match a tree pattern?

    Pattern fields:
      kind: exact kind match
      label: exact label match (or "*" for wildcard, "$X" for capture)
      label_regex: regex match on label
      children: list of child patterns (all must match in order)
      children_count: exact number of children
      capture: name to capture this node under

    Unspecified fields are wildcards (match anything).
    """
    # Kind match
    if "kind" in pattern:
        if pattern["kind"] != node.kind:
            return False

    # Label match
    if "label" in pattern:
        expected = pattern["label"]
        if expected == "*":
            pass
        elif expected.startswith("$"):
            pass  # Capture variable
        elif expected != node.label:
            return False

    # Regex label match
    if "label_regex" in pattern:
        if not re.search(pattern["label_regex"], node.label):
            return False

    # Children count
    if "children_count" in pattern:
        if len(node.children) != pattern["children_count"]:
            return False

    # Recursive children matching
    if "children" in pattern:
        child_patterns = pattern["children"]
        if isinstance(child_patterns, list):
            if len(child_patterns) > len(node.children):
                return False
            # Match child patterns as contiguous subsequence
            # For simplicity, require exact position matching
            for j, cpat in enumerate(child_patterns):
                if j >= len(node.children):
                    return False
                if not _tree_matches(node.children[j], cpat, captures):
                    return False

    # Capture
    if "capture" in pattern:
        captures[pattern["capture"]] = node

    return True


def _build_tree(spec, captures: dict[str, SkeletonNode]) -> SkeletonNode | None:
    """Build an output tree from a specification, substituting captured nodes.

    spec can be:
      - A string "$name" to substitute a captured node
      - A dict with capture_ref to substitute a captured node (from DSL parser)
      - A dict with kind/label/children to build a new node
      - A list (treated as children of a sequence)
    """
    if isinstance(spec, str):
        if spec.startswith("$"):
            return captures.get(spec[1:])
        return None

    if isinstance(spec, dict) and "capture_ref" in spec:
        return captures.get(spec["capture_ref"])

    if isinstance(spec, list):
        children = [_build_tree(s, captures) for s in spec]
        children = [c for c in children if c is not None]
        return SkeletonNode("sequence", children=children)

    if not isinstance(spec, dict):
        return None

    kind = spec.get("kind", "sequence")
    label = spec.get("label", "")

    # Build children
    child_specs = spec.get("children", [])
    children = []
    for child_spec in child_specs:
        child = _build_tree(child_spec, captures)
        if child:
            children.append(child)

    # Get source info from captures
    source_file = ""
    source_start = 0
    source_end = 0
    for cap in captures.values():
        source_file = source_file or cap.source_file
        source_start = source_start or cap.source_start
        source_end = max(source_end, cap.source_end)

    return SkeletonNode(
        kind=kind, label=label, children=children,
        source_file=source_file,
        source_start=source_start,
        source_end=source_end,
    )


# ========= Expression rewrite rules =========

def apply_expression_rewrites(args: list[str], func_name: str, lang: str) -> list[str]:
    """Apply expression rewrite rules to a list of argument strings.

    Args:
        args: List of normalized argument strings.
        func_name: The call's function name (for function-specific rules).
        lang: "c" or "r" — which side these args come from.

    Returns:
        Rewritten argument list. Some entries may be None (deleted by rules).

    Expression rewrite rules in JSON have the form:
        {
            "name": "rule_name",
            "side": "c" | "r" | "both",
            "function": "optional_func_name_or_*",
            "match": "regex_or_literal",
            "replace": "replacement_or_null",
            "justification": "why"
        }

    - "side": which arg list the rule applies to
    - "function": function name filter ("*" or omitted = all functions)
    - "match": regex to match against arg string
    - "replace": replacement string (null = delete the arg)
    """
    _load_config()

    result = list(args)
    for rule in _loaded_expr_rewrites:
        # Check side filter
        rule_side = rule.get("side", "both")
        if rule_side != "both" and rule_side != lang:
            continue

        # Check function filter
        rule_func = rule.get("function", "*")
        if rule_func != "*" and rule_func != func_name:
            continue

        # Apply match/replace to each arg
        match_pattern = rule.get("match", "")
        replacement = rule.get("replace")

        for i, arg in enumerate(result):
            if arg is None:
                continue
            # Handle list of args (alternatives from previous rules)
            args_to_check = arg if isinstance(arg, list) else [arg]
            new_alternatives = []
            any_matched = False

            for alt in args_to_check:
                full_match = re.fullmatch(match_pattern, alt)
                if full_match:
                    any_matched = True
                    if replacement is None:
                        pass  # Delete — don't add any alternative
                    elif isinstance(replacement, list):
                        # Multiple possible replacements
                        for rep in replacement:
                            new_alternatives.append(full_match.expand(rep))
                    else:
                        new_alternatives.append(full_match.expand(replacement))
                elif replacement is not None:
                    partial_match = re.search(match_pattern, alt)
                    if partial_match:
                        any_matched = True
                        if isinstance(replacement, list):
                            for rep in replacement:
                                new_alternatives.append(re.sub(match_pattern, rep, alt))
                        else:
                            new_alternatives.append(re.sub(match_pattern, replacement, alt))
                    else:
                        new_alternatives.append(alt)  # No match, keep original
                else:
                    new_alternatives.append(alt)  # No match, keep original

            if any_matched:
                if not new_alternatives:
                    result[i] = None  # All alternatives deleted
                elif len(new_alternatives) == 1:
                    result[i] = new_alternatives[0]
                else:
                    result[i] = new_alternatives  # Multiple alternatives

    return result
