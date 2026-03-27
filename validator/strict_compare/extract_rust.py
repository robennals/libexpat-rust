"""Extract semantic skeleton from Rust AST (tree-sitter)."""

import re
import tree_sitter
import tree_sitter_rust

from .nodes import SkeletonNode
from . import normalize

RUST_LANG = tree_sitter.Language(tree_sitter_rust.language())


def parse_rust(src_bytes: bytes):
    parser = tree_sitter.Parser(RUST_LANG)
    return parser.parse(src_bytes)


def find_function(tree, name: str):
    """Find a Rust function definition by name."""
    def walk(node):
        if node.type == "function_item":
            name_node = node.child_by_field_name("name")
            if name_node and name_node.text.decode() == name:
                return node
        for child in node.children:
            result = walk(child)
            if result:
                return result
        return None
    return walk(tree.root_node)


def _walk_all(node):
    yield node
    for child in node.children:
        yield from _walk_all(child)


def _node_text(node) -> str:
    return node.text.decode()


def _start_line(node) -> int:
    return node.start_point[0] + 1


def _end_line(node) -> int:
    return node.end_point[0] + 1


def extract_skeleton(func_node, source_file: str = "") -> SkeletonNode:
    """Extract a semantic skeleton from a Rust function AST node."""
    body = func_node.child_by_field_name("body")
    if not body:
        return SkeletonNode("sequence", source_file=source_file)

    children = _extract_block(body, source_file)
    return SkeletonNode(
        "sequence",
        source_file=source_file,
        source_start=_start_line(func_node),
        source_end=_end_line(func_node),
        children=children,
    )


def _extract_block(node, sf: str) -> list[SkeletonNode]:
    """Extract children from a block."""
    result = []
    for child in node.children:
        if child.type in ("{", "}"):
            continue
        extracted = _extract_node(child, sf)
        if extracted:
            if isinstance(extracted, list):
                result.extend(extracted)
            else:
                result.append(extracted)
    return result


def _extract_node(node, sf: str):
    """Extract a skeleton from any Rust AST node."""
    t = node.type

    if t == "block":
        children = _extract_block(node, sf)
        if len(children) == 1:
            return children[0]
        if not children:
            return None
        return SkeletonNode("sequence", children=children, source_file=sf,
                            source_start=_start_line(node), source_end=_end_line(node))

    if t == "expression_statement":
        return _extract_expression_stmt(node, sf)

    if t == "let_declaration":
        return _extract_let(node, sf)

    if t == "match_expression":
        return _extract_match(node, sf)

    if t in ("if_expression", "if_let_expression"):
        return _extract_if(node, sf)

    if t == "loop_expression":
        return _extract_loop(node, sf)

    if t in ("for_expression", "while_expression", "while_let_expression"):
        return _extract_while_loop(node, sf)

    if t == "return_expression":
        return _extract_return(node, sf)

    if t == "call_expression":
        return _extract_call(node, sf)

    if t == "method_call_expression":
        return _extract_method_call(node, sf)

    if t == "macro_invocation":
        return _extract_macro(node, sf)

    if t == "break_expression":
        return SkeletonNode("break", source_file=sf,
                            source_start=_start_line(node), source_end=_end_line(node))

    if t == "continue_expression":
        return SkeletonNode("continue", source_file=sf,
                            source_start=_start_line(node), source_end=_end_line(node))

    if t == "assignment_expression":
        return _extract_assignment(node, sf)

    if t == "compound_assignment_expr":
        return _extract_assignment(node, sf)

    # Tuple expression (commonly used for return values)
    if t == "tuple_expression":
        return _extract_tuple_return(node, sf)

    # Try to extract from children for wrapper nodes
    if t in ("parenthesized_expression", "type_cast_expression"):
        for child in node.children:
            if child.type not in ("(", ")", "as", "type_identifier"):
                result = _extract_node(child, sf)
                if result:
                    return result

    return None


def _extract_expression_stmt(node, sf: str):
    """Extract from expression statement (expression followed by ;)."""
    for child in node.children:
        if child.type == ";":
            continue
        return _extract_node(child, sf)
    return None


def _extract_match(node, sf: str) -> SkeletonNode:
    """Extract match expression -> match skeleton."""
    value = node.child_by_field_name("value")
    value_text = _normalize_expr(_node_text(value)) if value else ""

    # Find the match_body
    arms = []
    for child in node.children:
        if child.type == "match_block":
            for arm_node in child.children:
                if arm_node.type == "match_arm":
                    arm = _extract_match_arm(arm_node, sf)
                    if arm:
                        arms.append(arm)

    return SkeletonNode(
        "match", label=value_text, children=arms,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_match_arm(node, sf: str) -> SkeletonNode:
    """Extract a single match arm."""
    pattern_node = node.child_by_field_name("pattern")
    label = ""
    if pattern_node:
        raw = _node_text(pattern_node).strip()
        label = _normalize_arm_label(raw)

    # Extract body -- the value field of the match arm
    value_node = node.child_by_field_name("value")
    body_children = []
    if value_node:
        extracted = _extract_node(value_node, sf)
        if extracted:
            if isinstance(extracted, SkeletonNode) and extracted.kind == "sequence":
                body_children = extracted.children
            elif isinstance(extracted, list):
                body_children = extracted
            else:
                body_children = [extracted]

    return SkeletonNode(
        "arm", label=label, children=body_children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_if(node, sf: str) -> SkeletonNode:
    """Extract if / if-let -> branch skeleton."""
    # Get condition text and parsed expression
    cond_text = ""
    cond_expr = None
    if node.type == "if_let_expression":
        # if let Some(x) = expr { ... }
        pattern = node.child_by_field_name("pattern")
        value = node.child_by_field_name("value")
        if pattern and value:
            pat_text = _node_text(pattern)
            val_text = _normalize_expr(_node_text(value))
            cond_text = f"let {pat_text} = {val_text}"
            cond_expr = normalize.extract_expr_info(value, "rust")
    else:
        condition = node.child_by_field_name("condition")
        if condition:
            cond_text = _normalize_condition(_node_text(condition))
            cond_expr = normalize.extract_expr_info(condition, "rust")

    consequence = node.child_by_field_name("consequence")
    alternative = node.child_by_field_name("alternative")

    children = []
    if consequence:
        then_skel = _extract_node(consequence, sf)
        if then_skel and then_skel.kind == "sequence":
            children.append(then_skel)
        elif then_skel:
            children.append(SkeletonNode("sequence", children=[then_skel], source_file=sf))
        else:
            children.append(SkeletonNode("sequence", source_file=sf))
    else:
        children.append(SkeletonNode("sequence", source_file=sf))

    if alternative:
        # The alternative might be an else_clause containing another if or block
        else_body = None
        for child in alternative.children if alternative.type == "else_clause" else [alternative]:
            if child.type in ("block", "if_expression", "if_let_expression"):
                else_body = child
                break
        if else_body:
            else_skel = _extract_node(else_body, sf)
            if else_skel and else_skel.kind in ("sequence", "branch"):
                children.append(else_skel)
            elif else_skel:
                children.append(SkeletonNode("sequence", children=[else_skel], source_file=sf))

    return SkeletonNode(
        "branch", label=cond_text, expr=cond_expr, children=children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_loop(node, sf: str) -> SkeletonNode:
    """Extract loop expression."""
    body = node.child_by_field_name("body")
    children = []
    if body:
        extracted = _extract_node(body, sf)
        if extracted and extracted.kind == "sequence":
            children = extracted.children
        elif extracted:
            children = [extracted]

    return SkeletonNode(
        "loop", children=children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_while_loop(node, sf: str) -> SkeletonNode:
    """Extract while/for loop."""
    body = node.child_by_field_name("body")
    children = []
    if body:
        extracted = _extract_node(body, sf)
        if extracted and extracted.kind == "sequence":
            children = extracted.children
        elif extracted:
            children = [extracted]

    return SkeletonNode(
        "loop", children=children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_return(node, sf: str) -> SkeletonNode:
    """Extract return expression."""
    # Get the return value
    label = ""
    for child in node.children:
        if child.type == "return":
            continue
        raw = _node_text(child).strip()
        label = _normalize_return_value(raw)
        break

    return SkeletonNode(
        "return", label=label,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_tuple_return(node, sf: str):
    """Extract tuple expression -- often a return value like (XmlError::X, pos)."""
    text = _node_text(node).strip()
    # Check if it looks like an error return tuple
    if re.search(r'XmlError::', text):
        return SkeletonNode(
            "return", label=_normalize_return_value(text),
            source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
        )
    return None


def _extract_call(node, sf: str) -> SkeletonNode:
    """Extract a function call."""
    func_node = node.child_by_field_name("function")
    args_node = node.child_by_field_name("arguments")

    raw_name = _node_text(func_node) if func_node else ""
    name = _normalize_call_name(raw_name)

    if normalize.is_rust_noise(name):
        return None

    args = []
    arg_exprs = []
    if args_node:
        for child in args_node.children:
            if child.type in ("(", ")", ","):
                continue
            args.append(_normalize_expr(_node_text(child)))
            arg_exprs.append(normalize.extract_expr_info(child, "rust"))

    return SkeletonNode(
        "call", label=name, args=args, arg_exprs=arg_exprs,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_method_call(node, sf: str):
    """Extract a method call like self.foo(...) or obj.method(...)."""
    # Get the method name
    name_node = node.child_by_field_name("name")
    args_node = node.child_by_field_name("arguments")

    method_name = _node_text(name_node) if name_node else ""

    # Get the receiver (self, self.field, etc.)
    # First child is typically the receiver expression
    receiver = ""
    for child in node.children:
        if child == name_node:
            break
        if child.type == ".":
            continue
        receiver = _node_text(child)

    full_name = f"{receiver}.{method_name}" if receiver else method_name

    # Check for handler dispatch: self.*_handler call via if-let
    if re.match(r'self\.\w+_handler', full_name):
        # This is not a direct call -- it's a field access
        # Handler dispatch is captured by the if-let pattern in _extract_if
        return None

    name = _normalize_call_name(full_name)

    if normalize.is_rust_noise(name) or normalize.is_rust_noise(method_name):
        return None

    args = []
    arg_exprs = []
    if args_node:
        for child in args_node.children:
            if child.type in ("(", ")", ","):
                continue
            args.append(_normalize_expr(_node_text(child)))
            arg_exprs.append(normalize.extract_expr_info(child, "rust"))

    # Detect handler dispatch pattern: handler(data) where handler was bound by if-let
    # These show up as simple call expressions with "handler" as the function
    if method_name in ("handler",) or (not receiver and method_name == "handler"):
        pass  # Let it through as a regular call -- will be matched via handler_dispatch

    return SkeletonNode(
        "call", label=name, args=args, arg_exprs=arg_exprs,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_macro(node, sf: str):
    """Extract macro invocation (e.g., vec![], format!(), etc.)."""
    macro_node = node.child_by_field_name("macro")
    if not macro_node:
        # Try first child
        if node.children:
            macro_node = node.children[0]
    name = _node_text(macro_node) if macro_node else ""
    name = name.rstrip("!")

    if name in ("vec", "format", "println", "eprintln", "debug", "trace",
                "unreachable", "panic", "todo", "unimplemented"):
        return None

    return SkeletonNode(
        "call", label=name,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_let(node, sf: str):
    """Extract let declaration -- only if RHS has a meaningful call."""
    value = node.child_by_field_name("value")
    if not value:
        return None

    # If the value is a function call, extract it
    if value.type in ("call_expression", "method_call_expression"):
        call = _extract_node(value, sf)
        if call and call.kind == "call":
            # Attach the binding name
            pattern = node.child_by_field_name("pattern")
            if pattern:
                call.args.insert(0, f"-> {_node_text(pattern)}")
            return call

    # If the value is a match expression, extract it
    if value.type == "match_expression":
        return _extract_match(value, sf)

    # If the value is an if expression, extract it
    if value.type in ("if_expression", "if_let_expression"):
        return _extract_if(value, sf)

    # Skip pure data bindings (let x = some_value) -- not semantically interesting
    return None


def _extract_assignment(node, sf: str):
    """Extract assignment expression."""
    left = node.child_by_field_name("left")
    right = node.child_by_field_name("right")

    if right and right.type in ("call_expression", "method_call_expression"):
        call = _extract_node(right, sf)
        if call and call.kind == "call":
            left_text = _node_text(left) if left else ""
            call.args.insert(0, f"-> {_normalize_expr(left_text)}")
            return call

    left_text = _node_text(left) if left else ""
    return SkeletonNode(
        "assign", label=_normalize_expr(left_text),
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


# ========= Normalization helpers =========

def _normalize_call_name(raw: str) -> str:
    """Normalize a Rust call name."""
    raw = raw.strip()
    # self.method -> method
    raw = re.sub(r'^self\.', '', raw)
    # Self::method -> method
    raw = re.sub(r'^Self::', '', raw)
    # Module paths: xmltok_impl::content_tok -> content_tok
    raw = re.sub(r'^(?:xmltok(?:_impl)?|xmlrole|crate)::', '', raw)
    return raw


def _normalize_arm_label(raw: str) -> str:
    """Normalize a match arm pattern."""
    raw = raw.strip()
    # Handle OR patterns: XmlTok::A | XmlTok::B
    if "|" in raw:
        parts = [_normalize_arm_label(p.strip()) for p in raw.split("|")]
        return " | ".join(parts)
    # Wildcard
    if raw == "_":
        return "_default"
    # Strip binding patterns: tok @ XmlTok::X -> XmlTok::X
    if "@" in raw:
        raw = raw.split("@")[-1].strip()
    return normalize.normalize_rust_token(raw)


def _normalize_return_value(raw: str) -> str:
    """Normalize a return value."""
    raw = raw.strip()
    # Tuple returns: (XmlError::X, pos) -> XmlError::X
    m = re.match(r'\(\s*(XmlError::\w+)\s*,', raw)
    if m:
        return m.group(1)
    # Direct error: XmlError::X
    if raw.startswith("XmlError::"):
        return raw
    return _normalize_expr(raw)


def _normalize_condition(raw: str) -> str:
    """Normalize a condition expression."""
    raw = raw.strip()
    # Normalize self.field to field
    raw = re.sub(r'self\.(\w+)', r'\1', raw)
    return raw


def _normalize_expr(raw: str) -> str:
    """Normalize a general expression."""
    raw = raw.strip()
    raw = re.sub(r'self\.(\w+)', r'\1', raw)
    raw = re.sub(r'(?:xmltok(?:_impl)?|xmlrole)::', '', raw)
    return raw
