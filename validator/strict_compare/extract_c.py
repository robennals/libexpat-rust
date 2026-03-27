"""Extract semantic skeleton from C AST (tree-sitter)."""

import re
import tree_sitter
import tree_sitter_c

from .nodes import SkeletonNode
from . import normalize

C_LANG = tree_sitter.Language(tree_sitter_c.language())


def parse_c(src_bytes: bytes):
    parser = tree_sitter.Parser(C_LANG)
    return parser.parse(src_bytes)


def find_function(tree, name: str):
    """Find a C function definition by name."""
    def walk(node):
        if node.type == "function_definition":
            decl = node.child_by_field_name("declarator")
            if decl:
                for child in _walk_all(decl):
                    if child.type == "identifier" and child.text.decode() == name:
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
    """Extract a semantic skeleton from a C function AST node."""
    body = func_node.child_by_field_name("body")
    if not body:
        return SkeletonNode("sequence", source_file=source_file)

    children = _extract_compound(body, source_file)
    return SkeletonNode(
        "sequence",
        source_file=source_file,
        source_start=_start_line(func_node),
        source_end=_end_line(func_node),
        children=children,
    )


def _extract_compound(node, sf: str) -> list[SkeletonNode]:
    """Extract children from a compound_statement (block)."""
    result = []
    for child in node.children:
        if child.type in ("{", "}"):
            continue
        extracted = _extract_statement(child, sf)
        if extracted:
            if isinstance(extracted, list):
                result.extend(extracted)
            else:
                result.append(extracted)
    return result


def _extract_statement(node, sf: str):
    """Extract a skeleton node from a C statement."""
    t = node.type

    if t == "compound_statement":
        children = _extract_compound(node, sf)
        if len(children) == 1:
            return children[0]
        if not children:
            return None
        return SkeletonNode("sequence", children=children, source_file=sf,
                            source_start=_start_line(node), source_end=_end_line(node))

    if t == "switch_statement":
        return _extract_switch(node, sf)

    if t == "if_statement":
        return _extract_if(node, sf)

    if t in ("for_statement", "while_statement", "do_statement"):
        return _extract_loop(node, sf)

    if t == "return_statement":
        return _extract_return(node, sf)

    if t == "expression_statement":
        return _extract_expression_stmt(node, sf)

    if t == "declaration":
        return _extract_declaration(node, sf)

    if t == "break_statement":
        return SkeletonNode("break", source_file=sf,
                            source_start=_start_line(node), source_end=_end_line(node))

    if t == "continue_statement":
        return SkeletonNode("continue", source_file=sf,
                            source_start=_start_line(node), source_end=_end_line(node))

    if t == "labeled_statement":
        # goto labels -- extract the statement after the label
        for child in node.children:
            if child.type not in ("statement_identifier", ":"):
                return _extract_statement(child, sf)
        return None

    if t == "goto_statement":
        return SkeletonNode("goto", label=_node_text(node), source_file=sf,
                            source_start=_start_line(node), source_end=_end_line(node))

    # Preprocessor nodes
    if t in ("preproc_if", "preproc_ifdef"):
        return _extract_preproc(node, sf)

    # Fall through: skip comments, empty statements, etc.
    return None


def _extract_switch(node, sf: str) -> SkeletonNode:
    """Extract switch -> match skeleton."""
    condition = node.child_by_field_name("condition")
    cond_text = _normalize_expr(_node_text(condition)) if condition else ""

    body = node.child_by_field_name("body")
    arms = []
    if body:
        arms = _extract_switch_cases(body, sf)

    return SkeletonNode(
        "match", label=cond_text, children=arms,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_switch_cases(body_node, sf: str) -> list[SkeletonNode]:
    """Extract case arms from a switch body."""
    arms = []
    for child in body_node.children:
        if child.type == "case_statement":
            arm = _extract_case(child, sf)
            if arm:
                arms.append(arm)
        elif child.type == "default_statement":
            arm = _extract_default_case(child, sf)
            if arm:
                arms.append(arm)
        elif child.type == "compound_statement":
            # Nested compound within switch -- recurse
            arms.extend(_extract_switch_cases(child, sf))
    return arms


def _extract_case(node, sf: str) -> SkeletonNode:
    """Extract a single case arm."""
    value_node = node.child_by_field_name("value")
    if not value_node and len(node.children) > 1:
        value_node = node.children[1]

    label = ""
    if value_node:
        raw_label = _node_text(value_node).strip()
        label = _normalize_case_label(raw_label)

    # Extract body statements (everything after the colon)
    body_children = []
    past_colon = False
    for child in node.children:
        if child.type == ":":
            past_colon = True
            continue
        if not past_colon:
            continue
        # Handle fallthrough: a case_statement child that is another case_statement
        if child.type == "case_statement":
            # This is a fallthrough pattern -- the current case has no body
            # and falls into the next case
            break
        extracted = _extract_statement(child, sf)
        if extracted:
            if isinstance(extracted, list):
                body_children.extend(extracted)
            else:
                body_children.append(extracted)

    return SkeletonNode(
        "arm", label=label, children=body_children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_default_case(node, sf: str) -> SkeletonNode:
    """Extract default case arm."""
    body_children = []
    past_colon = False
    for child in node.children:
        if child.type == ":":
            past_colon = True
            continue
        if not past_colon:
            continue
        extracted = _extract_statement(child, sf)
        if extracted:
            if isinstance(extracted, list):
                body_children.extend(extracted)
            else:
                body_children.append(extracted)

    return SkeletonNode(
        "arm", label="_default", children=body_children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_if(node, sf: str) -> SkeletonNode:
    """Extract if/else -> branch skeleton."""
    condition = node.child_by_field_name("condition")
    cond_text = _normalize_condition(_node_text(condition)) if condition else ""

    consequence = node.child_by_field_name("consequence")
    alternative = node.child_by_field_name("alternative")

    children = []
    if consequence:
        then_skel = _extract_statement(consequence, sf)
        if then_skel:
            if then_skel.kind == "sequence":
                children.append(then_skel)
            else:
                children.append(SkeletonNode("sequence", children=[then_skel],
                                             source_file=sf))
        else:
            children.append(SkeletonNode("sequence", source_file=sf))
    else:
        children.append(SkeletonNode("sequence", source_file=sf))

    if alternative:
        else_skel = _extract_statement(alternative, sf)
        if else_skel:
            if else_skel.kind == "sequence":
                children.append(else_skel)
            elif else_skel.kind == "branch":
                # else-if chain: wrap the nested branch
                children.append(else_skel)
            else:
                children.append(SkeletonNode("sequence", children=[else_skel],
                                             source_file=sf))
        else:
            children.append(SkeletonNode("sequence", source_file=sf))

    return SkeletonNode(
        "branch", label=cond_text, children=children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_loop(node, sf: str) -> SkeletonNode:
    """Extract loop skeleton."""
    body = node.child_by_field_name("body")
    children = []
    if body:
        extracted = _extract_statement(body, sf)
        if extracted:
            if extracted.kind == "sequence":
                children = extracted.children
            else:
                children = [extracted]

    return SkeletonNode(
        "loop", children=children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_return(node, sf: str) -> SkeletonNode:
    """Extract return statement."""
    # Get return value
    label = ""
    for child in node.children:
        if child.type not in ("return", ";"):
            raw = _node_text(child).strip()
            label = _normalize_return_value(raw)
            break

    return SkeletonNode(
        "return", label=label,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_expression_stmt(node, sf: str):
    """Extract an expression statement -- usually a function call or assignment."""
    for child in node.children:
        if child.type == ";":
            continue
        return _extract_expression(child, sf)
    return None


def _extract_expression(node, sf: str):
    """Extract a skeleton from an expression node."""
    t = node.type

    if t == "call_expression":
        return _extract_call(node, sf)

    if t == "assignment_expression":
        return _extract_assignment(node, sf)

    if t == "conditional_expression":
        return _extract_ternary(node, sf)

    if t == "comma_expression":
        # Multiple expressions -- extract each
        results = []
        for child in node.children:
            if child.type == ",":
                continue
            extracted = _extract_expression(child, sf)
            if extracted:
                results.append(extracted)
        return results if results else None

    if t == "update_expression":
        # i++, --j, etc.
        text = _node_text(node)
        return SkeletonNode("assign", label=_normalize_expr(text),
                            source_file=sf, source_start=_start_line(node),
                            source_end=_end_line(node))

    if t == "parenthesized_expression":
        for child in node.children:
            if child.type not in ("(", ")"):
                return _extract_expression(child, sf)

    # For pointer dereference assignments like *eventPP = next
    if t == "pointer_expression":
        return None  # These are position tracking, handled by return normalization

    return None


def _extract_call(node, sf: str) -> SkeletonNode:
    """Extract a function call."""
    func_node = node.child_by_field_name("function")
    args_node = node.child_by_field_name("arguments")

    raw_name = _node_text(func_node) if func_node else ""
    name = _normalize_call_name(raw_name)

    args = []
    if args_node:
        for child in args_node.children:
            if child.type in ("(", ")", ","):
                continue
            args.append(_normalize_expr(_node_text(child)))

    # Check if this is a handler dispatch: parser->m_*Handler(...)
    if re.match(r'parser->m_\w+Handler', raw_name):
        handler_name = normalize.normalize_c_handler(
            re.search(r'm_(\w+)', raw_name).group(0)
        )
        return SkeletonNode(
            "handler_dispatch", label=handler_name, args=args[1:],  # skip handlerArg
            source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
        )

    return SkeletonNode(
        "call", label=name, args=args,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_assignment(node, sf: str):
    """Extract an assignment expression."""
    left = node.child_by_field_name("left")
    right = node.child_by_field_name("right")

    left_text = _node_text(left) if left else ""
    # Check if RHS is a function call -- these are semantically important
    if right and right.type == "call_expression":
        call_skel = _extract_call(right, sf)
        if call_skel:
            call_skel.args.insert(0, f"-> {_normalize_expr(left_text)}")
            return call_skel

    # Position tracking assignments like *eventPP = next, *nextPtr = s
    if re.match(r'\*\w+PP\b|\*nextPtr\b|\*eventEndPP\b|\*eventPP\b', left_text):
        return None  # Position tracking -- Rust handles via return tuples

    return SkeletonNode(
        "assign", label=_normalize_expr(left_text),
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_ternary(node, sf: str) -> SkeletonNode:
    """Extract ternary expression as branch."""
    condition = node.child_by_field_name("condition")
    consequence = node.child_by_field_name("consequence")
    alternative = node.child_by_field_name("alternative")

    cond_text = _normalize_condition(_node_text(condition)) if condition else ""
    children = []
    if consequence:
        c = _extract_expression(consequence, sf)
        children.append(c if c else SkeletonNode("sequence", source_file=sf))
    if alternative:
        a = _extract_expression(alternative, sf)
        children.append(a if a else SkeletonNode("sequence", source_file=sf))

    return SkeletonNode(
        "branch", label=cond_text, children=children,
        source_file=sf, source_start=_start_line(node), source_end=_end_line(node),
    )


def _extract_declaration(node, sf: str):
    """Extract variable declarations (only if they have meaningful initializers)."""
    # Look for declarators with initializers that are function calls
    for child in _walk_all(node):
        if child.type == "init_declarator":
            value = child.child_by_field_name("value")
            if value and value.type == "call_expression":
                return _extract_call(value, sf)
    return None


def _extract_preproc(node, sf: str):
    """Extract from preprocessor conditionals.

    For #if XML_GE (the active config), extract the 'then' branch.
    For other conditions, extract both branches.
    """
    # For now, extract all children that are statements
    result = []
    for child in node.children:
        if child.type.startswith("preproc_") or child.type in ("#if", "#ifdef", "#endif", "#else"):
            continue
        extracted = _extract_statement(child, sf)
        if extracted:
            if isinstance(extracted, list):
                result.extend(extracted)
            else:
                result.append(extracted)
    return result if result else None


# ========= Normalization helpers =========

def _normalize_call_name(raw: str) -> str:
    """Normalize a C function call name."""
    # Strip parser-> prefix for method-like calls
    raw = raw.strip()
    if raw.startswith("parser->"):
        raw = raw[len("parser->"):]
    # Strip m_ prefix
    raw = re.sub(r'^m_', '', raw)
    return normalize.normalize_c_call(raw)


def _normalize_case_label(raw: str) -> str:
    """Normalize a switch case label."""
    raw = raw.strip()
    if raw.startswith("XML_TOK_"):
        return normalize.normalize_c_token(raw)
    if raw.startswith("XML_ROLE_"):
        return normalize.normalize_c_role(raw)
    return raw


def _normalize_return_value(raw: str) -> str:
    """Normalize a return value."""
    raw = raw.strip()
    if raw.startswith("XML_ERROR_"):
        return normalize.normalize_c_error(raw)
    if raw == "0":
        return "error"  # C return 0 = error in reportComment-like functions
    if raw == "1":
        return "ok"  # C return 1 = success
    return _normalize_expr(raw)


def _normalize_condition(raw: str) -> str:
    """Normalize a condition expression for structural matching."""
    raw = raw.strip()
    if raw.startswith("(") and raw.endswith(")"):
        raw = raw[1:-1].strip()

    # Normalize handler null checks: !parser->m_*Handler -> !handler
    raw = re.sub(r'!\s*parser->m_(\w+)', lambda m: f"!{normalize.camel_to_snake(m.group(1))}", raw)
    raw = re.sub(r'parser->m_(\w+)', lambda m: normalize.camel_to_snake(m.group(1)), raw)
    # Normalize camelCase local variables to snake_case
    raw = re.sub(r'\b([a-z][a-zA-Z]+)\b', lambda m: normalize.camel_to_snake(m.group(1)), raw)

    return raw


def _normalize_expr(raw: str) -> str:
    """Normalize a general expression."""
    raw = raw.strip()
    # Strip wrapping parentheses
    while raw.startswith("(") and raw.endswith(")"):
        raw = raw[1:-1].strip()
    # Normalize parser->m_field to field
    raw = re.sub(r'parser->m_(\w+)', lambda m: normalize.camel_to_snake(m.group(1)), raw)
    # Normalize enc->minBytesPerChar to min_bytes_per_char
    raw = re.sub(r'enc->(\w+)', lambda m: normalize.camel_to_snake(m.group(1)), raw)
    return raw
