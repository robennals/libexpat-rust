"""Extract scoping tree + content sets from common ASTs.

Separates control flow structure (scoping nodes) from operations
(calls, identifiers, literals). This enables comparison at the
structural level while ignoring expression-level nesting differences.
"""

from dataclasses import dataclass, field
from .common_ast import Node


# Node kinds that define scope (control flow structure)
SCOPING_KINDS = {"block", "if", "if_let", "loop", "match", "arm", "closure"}


@dataclass
class ContentSet:
    """All operations found within a single scope level."""
    calls: dict[str, set[str]] = field(default_factory=dict)
    # call_name -> set of arg identifiers. E.g. {"report_default": {"enc", "data", "start", "end"}}

    identifiers: set[str] = field(default_factory=set)
    # All variable/field names referenced

    literals: set[str] = field(default_factory=set)
    # All literal values

    error_returns: set[str] = field(default_factory=set)
    # Error codes returned (e.g., "XmlError::InvalidToken")

    assigns_to: set[str] = field(default_factory=set)
    # Field/variable names assigned to

    operators: set[str] = field(default_factory=set)
    # Comparison/arithmetic operators used

    def is_empty(self) -> bool:
        return (not self.calls and not self.identifiers and not self.literals
                and not self.error_returns and not self.assigns_to)

    def dump(self, indent: int = 0) -> str:
        prefix = "  " * indent
        parts = []
        if self.calls:
            for name, args in sorted(self.calls.items()):
                parts.append(f"{prefix}call {name}({', '.join(sorted(args))})")
        if self.error_returns:
            parts.append(f"{prefix}returns: {sorted(self.error_returns)}")
        if self.assigns_to:
            parts.append(f"{prefix}assigns: {sorted(self.assigns_to)}")
        if self.literals:
            parts.append(f"{prefix}literals: {sorted(self.literals)}")
        return "\n".join(parts)


@dataclass
class ScopeNode:
    """A node in the scoping tree."""
    kind: str  # block, if, if_let, loop, match, arm
    label: str = ""  # arm label, condition summary
    children: list['ScopeNode'] = field(default_factory=list)
    content: ContentSet = field(default_factory=ContentSet)
    source_file: str = ""
    line: int = 0

    def dump(self, indent: int = 0) -> str:
        prefix = "  " * indent
        line = f"{prefix}{self.kind}"
        if self.label:
            label_short = self.label[:50].replace('\n', ' ')
            line += f"({label_short})"
        if self.line:
            line += f"  @{self.line}"
        lines = [line]
        content_str = self.content.dump(indent + 1)
        if content_str:
            lines.append(content_str)
        for child in self.children:
            lines.append(child.dump(indent + 1))
        return "\n".join(lines)


def extract_scoping_tree(ast: Node) -> ScopeNode:
    """Extract the scoping tree from a common AST.

    Walks the AST and:
    - Scoping nodes become ScopeNode children
    - Non-scoping nodes have their content collected into the
      enclosing scope's ContentSet
    """
    root = ScopeNode(kind="block", source_file=ast.source_file, line=ast.line)
    _extract_scope(ast, root)
    return root


def _extract_scope(ast: Node, current_scope: ScopeNode):
    """Recursively extract scoping structure and content."""
    for child in ast.children:
        if child.kind in SCOPING_KINDS:
                # Named/labeled scope — create a child scope
                child_scope = ScopeNode(
                    kind=child.kind,
                    label=_extract_scope_label(child),
                    source_file=child.source_file,
                    line=child.line,
                )
                current_scope.children.append(child_scope)
                _extract_scope(child, child_scope)
        else:
            # Non-scoping node — collect content, then check for nested scopes
            _collect_content(child, current_scope.content)
            _find_nested_scopes(child, current_scope)


def _find_nested_scopes(node: Node, parent_scope: ScopeNode):
    """Find scoping nodes nested within non-scoping expressions."""
    for child in node.children:
        if child.kind in SCOPING_KINDS:
            child_scope = ScopeNode(
                kind=child.kind,
                label=_extract_scope_label(child),
                source_file=child.source_file,
                line=child.line,
            )
            parent_scope.children.append(child_scope)
            _extract_scope(child, child_scope)
        else:
            _find_nested_scopes(child, parent_scope)


def _collect_content(node: Node, content: ContentSet):
    """Collect content items from a non-scoping node into a ContentSet.

    Recurses into children but stops at scoping boundaries.
    """
    kind = node.kind

    if kind == "call" or kind == "method_call":
        # Extract function name and argument identifiers
        func_name = _extract_call_name(node)
        if func_name:
            arg_idents = set()
            # Collect identifiers from all children except the function name
            for i, child in enumerate(node.children):
                if i == 0 and kind == "call":
                    continue  # Skip function name child
                if i <= 1 and kind == "method_call":
                    continue  # Skip receiver and method name
                _collect_identifiers(child, arg_idents)
            content.calls.setdefault(func_name, set()).update(arg_idents)

    elif kind == "return":
        # Check for error return
        for child in node.children:
            error = _extract_error_code(child)
            if error:
                content.error_returns.add(error)

    elif kind == "assign":
        # Track what's being assigned to
        if node.children:
            target = _extract_assign_target(node.children[0])
            if target:
                content.assigns_to.add(target)

    elif kind == "ident":
        content.identifiers.add(node.value)

    elif kind == "literal":
        content.literals.add(node.value)

    elif kind == "binary" or kind == "unary":
        if node.value:
            content.operators.add(node.value)

    # Recurse into children (but not into scoping nodes)
    for child in node.children:
        if child.kind not in SCOPING_KINDS:
            _collect_content(child, content)


def _extract_call_name(node: Node) -> str:
    """Extract the function name from a call node."""
    if not node.children:
        return ""
    first = node.children[0]
    if first.kind == "ident":
        return first.value
    if first.kind == "field":
        # method call: obj.method — return the method name
        if len(first.children) >= 2:
            return first.children[-1].value if first.children[-1].kind == "ident" else ""
        return first.value if first.value else ""
    # Scoped identifier: Self::foo
    if first.kind == "ident" and "::" in first.value:
        return first.value.split("::")[-1]
    return first.value if first.value else ""


def _extract_error_code(node: Node) -> str:
    """Extract an error code from a return value node."""
    if node.kind == "ident":
        val = node.value
        if "error" in val.lower() or "XmlError" in val:
            return val
    # Check for scoped like XmlError::InvalidToken
    for child in node.children:
        result = _extract_error_code(child)
        if result:
            return result
    return ""


def _extract_assign_target(node: Node) -> str:
    """Extract the name of what's being assigned to."""
    if node.kind == "ident":
        return node.value
    if node.kind == "field":
        parts = []
        for child in node.children:
            if child.kind == "ident":
                parts.append(child.value)
            elif child.kind == "self":
                parts.append("self")
        return ".".join(parts)
    if node.kind == "ptr":
        # *ptr = value — deref assignment
        if node.children:
            return f"*{_extract_assign_target(node.children[0])}"
    return ""


def _collect_identifiers(node: Node, idents: set[str]):
    """Collect all identifiers from a subtree."""
    if node.kind == "ident":
        idents.add(node.value)
    for child in node.children:
        if child.kind not in SCOPING_KINDS:
            _collect_identifiers(child, idents)


def _extract_scope_label(node: Node) -> str:
    """Extract a label for a scoping node (for matching purposes).

    For arms: the match arm label
    For if: the condition's core identifiers
    For loop/block: empty
    """
    if node.kind == "arm":
        return node.value  # arm label from common AST

    if node.kind in ("if", "if_let"):
        # Extract condition identifiers
        if node.children:
            cond = node.children[0]
            idents = set()
            _collect_identifiers(cond, idents)
            # Filter noise
            noise = {"self", "parser", "enc", "let", "mut", "some", "none"}
            meaningful = sorted(i for i in idents if i.lower() not in noise)
            return ", ".join(meaningful[:3]) if meaningful else ""
    return ""
