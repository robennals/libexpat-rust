"""Structural comparison of C and Rust skeletons.

The core algorithm: given a rewritten-C skeleton and a Rust skeleton,
verify that every semantically meaningful C node has a corresponding
Rust node in the correct structural position.

Matching strategy:
- Match arms: pair by label, recursively compare bodies
- Sequences: C children must be an ordered subsequence of Rust children
- Calls: labels must match
- Branches: condition labels compared, then/else compared recursively
- Returns: return value labels must match
"""

import re
from .nodes import SkeletonNode, Mismatch


def compare_skeletons(c_skel: SkeletonNode, r_skel: SkeletonNode,
                      context: str = "") -> list[Mismatch]:
    """Compare C and Rust skeletons, returning all mismatches."""
    mismatches = []
    _compare(c_skel, r_skel, context, mismatches)
    return mismatches


def _compare(c: SkeletonNode, r: SkeletonNode, ctx: str,
             mismatches: list[Mismatch]):
    """Recursive comparison."""

    # Top-level kind mismatch
    if c.kind != r.kind:
        # Some kind mismatches are acceptable
        if c.kind == "sequence" and r.kind in ("sequence", "branch", "match"):
            pass  # Restructured
        elif c.kind in ("call", "handler_dispatch") and r.kind in ("call", "handler_dispatch"):
            pass  # Handler dispatch vs call is flexible
        elif c.kind in ("call", "handler_dispatch") and r.kind == "branch":
            if _branch_contains_call(r, c.label):
                return
            mismatches.append(Mismatch(
                c, r, f"Kind mismatch: C has {c.kind}, Rust has {r.kind}", ctx,
            ))
            return
        elif c.kind == "branch" and r.kind == "sequence":
            _compare_sequence(c.children, r.children, ctx, mismatches, c, r)
            return
        elif c.kind == "return" and r.kind == "sequence":
            return
        elif c.kind == "loop" and r.kind == "sequence":
            # C was flattened to just a loop; Rust has sequence > loop
            rust_loop = _find_loop_in_tree(r.children)
            if rust_loop:
                _compare(c, rust_loop, ctx, mismatches)
                return
            mismatches.append(Mismatch(
                c, r, f"Kind mismatch: C has {c.kind}, Rust has {r.kind}", ctx,
            ))
            return
        elif c.kind == "match" and r.kind == "sequence":
            # C was flattened to just a match; look for it in Rust
            rust_match = _find_in_children(r, "match")
            if rust_match:
                _compare(c, rust_match, ctx, mismatches)
                return
            mismatches.append(Mismatch(
                c, r, f"Kind mismatch: C has {c.kind}, Rust has {r.kind}", ctx,
            ))
            return
        else:
            mismatches.append(Mismatch(
                c, r, f"Kind mismatch: C has {c.kind}, Rust has {r.kind}", ctx,
            ))
            return

    kind = c.kind

    if kind == "match":
        _compare_match(c, r, ctx, mismatches)
    elif kind == "arm":
        _compare_sequence(c.children, r.children, f"{ctx}/{c.label}", mismatches, c, r)
    elif kind == "sequence":
        _compare_sequence(c.children, r.children, ctx, mismatches, c, r)
    elif kind == "branch":
        _compare_branch(c, r, ctx, mismatches)
    elif kind == "loop":
        _compare_sequence(c.children, r.children, f"{ctx}/loop", mismatches, c, r)
    elif kind in ("call", "handler_dispatch"):
        _compare_call(c, r, ctx, mismatches)
    elif kind == "return":
        _compare_return(c, r, ctx, mismatches)
    elif kind == "assign":
        pass  # Assignments are structural noise -- matched by position only
    elif kind in ("break", "continue", "goto"):
        pass  # Control flow keywords -- already handled by rewrite rules
    else:
        # Unknown kind -- compare children generically
        _compare_sequence(c.children, r.children, ctx, mismatches, c, r)


def _compare_match(c: SkeletonNode, r: SkeletonNode, ctx: str,
                   mismatches: list[Mismatch]):
    """Compare match/switch: pair arms by label.

    If the Rust match has different arm labels (e.g., Ok/Err for Result unwrap),
    look for a nested match that has the right labels.
    """
    # Build label -> arm map for Rust
    r_arms = {}
    for arm in r.children:
        label = arm.label
        if label:
            for sub_label in label.split(" | "):
                sub_label = sub_label.strip()
                r_arms[sub_label] = arm

    # Check if any C arm matches any Rust arm
    c_labels = {arm.label for arm in c.children if arm.label}
    if c_labels and not any(_find_arm(r_arms, cl) for cl in c_labels):
        # No C arms match -- look for a nested match in Rust that does match
        nested_match = _find_matching_match_in_tree(r, c_labels)
        if nested_match:
            _compare_match(c, nested_match, ctx, mismatches)
            return

    for c_arm in c.children:
        c_label = c_arm.label
        if not c_label:
            continue

        # Find matching Rust arm
        r_arm = _find_arm(r_arms, c_label)

        if not r_arm:
            # Check if C arm is empty (fallthrough to next)
            if not c_arm.children:
                continue  # Empty case with fallthrough -- OK
            mismatches.append(Mismatch(
                c_arm, None,
                f"C arm {c_label} has no matching Rust arm",
                ctx, "ERROR",
            ))
            continue

        # Recursively compare arm bodies
        _compare(c_arm, r_arm, f"{ctx}/{c_label}", mismatches)

    # Check for Rust arms that have no C equivalent -- just informational
    c_labels = {arm.label for arm in c.children if arm.label}
    for r_arm in r.children:
        if r_arm.label and r_arm.label != "_default":
            # Check if any of its sub-labels are in C
            sub_labels = [l.strip() for l in r_arm.label.split("|")]
            if not any(l in c_labels for l in sub_labels):
                # This is just info, not an error -- Rust may handle extra cases
                pass


def _compare_sequence(c_children: list[SkeletonNode], r_children: list[SkeletonNode],
                      ctx: str, mismatches: list[Mismatch],
                      c_parent: SkeletonNode = None, r_parent: SkeletonNode = None):
    """Compare sequences: C children must be an ordered subsequence of Rust children.

    This is the key insight: Rust may have extra nodes (bounds checks, variable
    bindings, type conversions) but must contain all C nodes in order.
    """
    if not c_children:
        return  # Nothing in C to match

    # Filter out noise nodes from both sides
    c_meaningful = [c for c in c_children if _is_meaningful(c)]
    r_meaningful = [r for r in r_children if _is_meaningful(r)]

    if not c_meaningful:
        return

    # Find ordered subsequence match using greedy scan
    r_idx = 0
    for c_node in c_meaningful:
        matched = False
        start_r_idx = r_idx
        while r_idx < len(r_meaningful):
            r_node = r_meaningful[r_idx]
            if _nodes_match(c_node, r_node):
                # Found a match -- recursively compare their children
                _compare(c_node, r_node, ctx, mismatches)
                r_idx += 1
                matched = True
                break
            r_idx += 1

        if not matched:
            # Try deeper matching: if C has a loop, look for a loop
            # anywhere in the Rust subtree (may be nested differently)
            if c_node.kind == "loop":
                rust_loop = _find_loop_in_tree(r_children)
                if rust_loop:
                    _compare(c_node, rust_loop, ctx, mismatches)
                    matched = True

            # Try matching a C match inside Rust children's children
            # (when nesting differs by one level)
            if not matched and c_node.kind == "match":
                for r_child in r_meaningful[start_r_idx:]:
                    rust_match = _find_in_children(r_child, "match")
                    if rust_match:
                        _compare(c_node, rust_match, ctx, mismatches)
                        matched = True
                        break

        if not matched:
            mismatches.append(Mismatch(
                c_node, None,
                f"C {c_node.kind}({c_node.label}) not found in Rust sequence",
                ctx, "ERROR",
            ))


def _compare_branch(c: SkeletonNode, r: SkeletonNode, ctx: str,
                    mismatches: list[Mismatch]):
    """Compare if/else branches."""
    # Condition comparison -- relaxed (C and Rust express conditions very differently)
    # We check that handler checks match and error condition checks match
    c_cond = c.label
    r_cond = r.label

    # Check for handler null check pattern
    c_handler = _extract_handler_from_condition(c_cond)
    r_handler = _extract_handler_from_condition(r_cond)
    if c_handler and r_handler:
        if c_handler != r_handler:
            mismatches.append(Mismatch(
                c, r,
                f"Handler check mismatch: C checks {c_handler}, Rust checks {r_handler}",
                ctx, "WARNING",
            ))

    # Compare then-branches
    c_then = c.children[0] if c.children else SkeletonNode("sequence")
    r_then = r.children[0] if r.children else SkeletonNode("sequence")

    if c_then.kind == "sequence" and r_then.kind == "sequence":
        _compare_sequence(c_then.children, r_then.children, f"{ctx}/then", mismatches, c_then, r_then)
    else:
        _compare(c_then, r_then, f"{ctx}/then", mismatches)

    # Compare else-branches (if both have them)
    if len(c.children) > 1 and len(r.children) > 1:
        c_else = c.children[1]
        r_else = r.children[1]
        if c_else.kind == "sequence" and r_else.kind == "sequence":
            _compare_sequence(c_else.children, r_else.children, f"{ctx}/else", mismatches, c_else, r_else)
        else:
            _compare(c_else, r_else, f"{ctx}/else", mismatches)
    elif len(c.children) > 1:
        # C has else but Rust doesn't
        c_else = c.children[1]
        if _has_meaningful_content(c_else):
            mismatches.append(Mismatch(
                c_else, None,
                "C has else-branch but Rust doesn't",
                ctx, "WARNING",
            ))


def _compare_call(c: SkeletonNode, r: SkeletonNode, ctx: str,
                  mismatches: list[Mismatch]):
    """Compare function calls."""
    if c.label != r.label:
        mismatches.append(Mismatch(
            c, r,
            f"Call name mismatch: C={c.label}, Rust={r.label}",
            ctx, "ERROR",
        ))


def _compare_return(c: SkeletonNode, r: SkeletonNode, ctx: str,
                    mismatches: list[Mismatch]):
    """Compare return statements."""
    if c.label and r.label and c.label != r.label:
        # Check if they're equivalent error codes
        if c.label.startswith("XmlError::") and r.label.startswith("XmlError::"):
            if c.label != r.label:
                mismatches.append(Mismatch(
                    c, r,
                    f"Return value mismatch: C returns {c.label}, Rust returns {r.label}",
                    ctx, "ERROR",
                ))
        # For non-error returns, be more relaxed (C returns 0/1, Rust returns differently)


# ========= Helper functions =========

def _is_meaningful(node: SkeletonNode) -> bool:
    """Is this node semantically meaningful for comparison?"""
    if node.kind == "assign":
        return False
    if node.kind in ("break", "continue"):
        return False
    if node.kind == "sequence" and not node.children:
        return False
    return True


def _nodes_match(c: SkeletonNode, r: SkeletonNode) -> bool:
    """Do these two nodes correspond to each other (shallow match)?"""
    # Same kind
    if c.kind == r.kind:
        if c.kind in ("call", "handler_dispatch"):
            return _calls_match(c.label, r.label)
        if c.kind == "return":
            return c.label == r.label or not c.label or not r.label
        if c.kind == "match":
            # Only match if at least one arm label overlaps
            c_labels = {arm.label for arm in c.children if arm.label and arm.label != "_default"}
            r_labels = set()
            for arm in r.children:
                if arm.label:
                    for sub in arm.label.split(" | "):
                        r_labels.add(sub.strip())
            if c_labels and r_labels:
                return any(_label_matches(cl, r_labels) for cl in c_labels)
            return True  # If no labels to compare, assume match
        if c.kind == "branch":
            return _conditions_correspond(c.label, r.label)
        if c.kind == "loop":
            return True
        if c.kind == "sequence":
            return True
        return True

    # Cross-kind matches
    if c.kind in ("call", "handler_dispatch") and r.kind in ("call", "handler_dispatch"):
        return _calls_match(c.label, r.label)

    # A C call might match a Rust branch that wraps the call (handler dispatch pattern)
    if c.kind == "call" and r.kind == "branch":
        # Check if the branch contains the call
        return _branch_contains_call(r, c.label)

    return False


def _calls_match(c_label: str, r_label: str) -> bool:
    """Do two call labels refer to the same function?"""
    if c_label == r_label:
        return True
    # Rust method calls: self.foo becomes just foo in C normalization
    if r_label.endswith(f".{c_label}"):
        return True
    # Module-qualified Rust calls
    if r_label.endswith(f"::{c_label}"):
        return True
    # The Self:: prefix removal
    r_stripped = re.sub(r'^(?:self\.|Self::)', '', r_label)
    if c_label == r_stripped:
        return True
    return False


def _branch_contains_call(branch: SkeletonNode, call_label: str) -> bool:
    """Check if a branch node contains a call with the given label."""
    for child in branch.children:
        if child.kind in ("call", "handler_dispatch"):
            if _calls_match(call_label, child.label):
                return True
        if child.kind == "sequence":
            for subchild in child.children:
                if subchild.kind in ("call", "handler_dispatch"):
                    if _calls_match(call_label, subchild.label):
                        return True
    return False


def _conditions_correspond(c_cond: str, r_cond: str) -> bool:
    """Do two condition expressions correspond?"""
    if not c_cond or not r_cond:
        return True  # If either is unknown, assume match

    # Extract the core variable/field being checked
    c_core = _extract_condition_core(c_cond)
    r_core = _extract_condition_core(r_cond)

    if c_core and r_core:
        return c_core == r_core

    # Fallback: check for common substrings
    # This is intentionally loose -- conditions vary a lot between C and Rust
    return True


def _extract_condition_core(cond: str) -> str:
    """Extract the core variable from a condition."""
    # Handler check: !comment_handler or let Some(handler) = comment_handler
    m = re.search(r'(\w+_handler)', cond)
    if m:
        return m.group(1)
    # Variable check: have_more, tag_level == start_tag_level, etc.
    m = re.match(r'!?\s*(\w+)', cond)
    if m:
        return m.group(1)
    return ""


def _extract_handler_from_condition(cond: str) -> str:
    """Extract handler name from a condition like '!comment_handler'."""
    m = re.search(r'(\w+_handler)', cond)
    return m.group(1) if m else ""


def _find_arm(r_arms: dict, c_label: str) -> SkeletonNode | None:
    """Find a Rust arm matching a C arm label."""
    r_arm = r_arms.get(c_label)
    if r_arm:
        return r_arm
    # Try without module prefix
    short_label = c_label.split("::")[-1] if "::" in c_label else c_label
    for r_label, r_a in r_arms.items():
        if r_label.endswith(f"::{short_label}") or r_label == short_label:
            return r_a
    return None


def _find_matching_match_in_tree(node: SkeletonNode, c_labels: set) -> SkeletonNode | None:
    """Find a match node in the tree whose arms match the given labels."""
    for child in node.children:
        if child.kind == "match":
            child_labels = set()
            for arm in child.children:
                if arm.label:
                    for sub in arm.label.split(" | "):
                        child_labels.add(sub.strip())
            if any(_label_matches(cl, child_labels) for cl in c_labels):
                return child
        # Recurse
        result = _find_matching_match_in_tree(child, c_labels)
        if result:
            return result
    return None


def _label_matches(c_label: str, r_labels: set) -> bool:
    """Does a C label match any Rust label?"""
    if c_label in r_labels:
        return True
    short = c_label.split("::")[-1] if "::" in c_label else c_label
    for rl in r_labels:
        if rl.endswith(f"::{short}") or rl == short:
            return True
    return False


def _find_loop_in_tree(nodes: list[SkeletonNode]) -> SkeletonNode | None:
    """Find a loop node anywhere in the tree."""
    for node in nodes:
        if node.kind == "loop":
            return node
        result = _find_loop_in_tree(node.children)
        if result:
            return result
    return None


def _find_in_children(node: SkeletonNode, kind: str) -> SkeletonNode | None:
    """Find a node of given kind in immediate or nested children."""
    for child in node.children:
        if child.kind == kind:
            return child
        # One level deeper
        for grandchild in child.children:
            if grandchild.kind == kind:
                return grandchild
    return None


def _has_meaningful_content(node: SkeletonNode) -> bool:
    """Does this node have any semantically meaningful content?"""
    if node.kind in ("call", "handler_dispatch", "return"):
        return True
    return any(_has_meaningful_content(c) for c in node.children)
