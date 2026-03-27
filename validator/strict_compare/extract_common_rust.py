"""Convert Rust tree-sitter AST to common AST (lossless, project-agnostic).

This module converts tree-sitter Rust AST nodes into common AST nodes.
It knows about Rust syntax (if_expression, match_expression, etc.) but
has NO knowledge of libexpat, XML parsing, or any specific codebase.

All project-specific simplifications are applied as JSON-configured
simplification rules AFTER extraction.
"""

import tree_sitter
import tree_sitter_rust

from .common_ast import Node

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


def extract(ts_node, source_file: str = "Rust") -> Node:
    """Convert a Rust tree-sitter node to a common AST node (lossless)."""
    return _convert(ts_node, source_file)


def _text(node) -> str:
    return node.text.decode() if node.text else ""


def _line(node) -> int:
    return node.start_point[0] + 1


def _convert(ts, sf: str) -> Node:
    """Convert any Rust tree-sitter node to a common AST node."""
    t = ts.type

    # === Statements ===

    if t == "block":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("{", "}")]
        return Node("block", children=children, source_file=sf, line=_line(ts))

    if t == "if_expression":
        cond = ts.child_by_field_name("condition")
        cons = ts.child_by_field_name("consequence")
        alt = ts.child_by_field_name("alternative")
        children = []
        if cond:
            children.append(_convert(cond, sf))
        if cons:
            children.append(_convert(cons, sf))
        if alt:
            children.append(_convert(alt, sf))
        return Node("if", children=children, source_file=sf, line=_line(ts))

    if t == "if_let_expression":
        pattern = ts.child_by_field_name("pattern")
        value = ts.child_by_field_name("value")
        cons = ts.child_by_field_name("consequence")
        alt = ts.child_by_field_name("alternative")
        children = []
        if pattern:
            children.append(_convert(pattern, sf))
        if value:
            children.append(_convert(value, sf))
        if cons:
            children.append(_convert(cons, sf))
        if alt:
            children.append(_convert(alt, sf))
        return Node("if_let", children=children, source_file=sf, line=_line(ts))

    if t == "match_expression":
        value = ts.child_by_field_name("value")
        children = []
        if value:
            children.append(_convert(value, sf))
        for c in ts.children:
            if c.type == "match_block":
                for arm in c.children:
                    if arm.type == "match_arm":
                        children.append(_convert(arm, sf))
        return Node("match", children=children, source_file=sf, line=_line(ts))

    if t == "match_arm":
        pattern = ts.child_by_field_name("pattern")
        value = ts.child_by_field_name("value")
        children = []
        label = _text(pattern).strip() if pattern else ""
        if value:
            children.append(_convert(value, sf))
        return Node("arm", children=children, value=label,
                     source_file=sf, line=_line(ts))

    if t == "loop_expression":
        body = ts.child_by_field_name("body")
        children = []
        if body:
            children.append(_convert(body, sf))
        return Node("loop", children=children, source_file=sf, line=_line(ts))

    if t in ("for_expression", "while_expression", "while_let_expression"):
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("for", "while", "in", "let")]
        return Node("loop", children=children, source_file=sf, line=_line(ts))

    if t == "return_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type != "return"]
        return Node("return", children=children, source_file=sf, line=_line(ts))

    if t == "break_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type != "break"]
        return Node("break", children=children, source_file=sf, line=_line(ts))

    if t == "continue_expression":
        return Node("continue", source_file=sf, line=_line(ts))

    if t == "expression_statement":
        children = [_convert(c, sf) for c in ts.children if c.type != ";"]
        if len(children) == 1:
            return Node("expr_stmt", children=children,
                         source_file=sf, line=_line(ts))
        return Node("expr_stmt", children=children,
                     source_file=sf, line=_line(ts))

    if t == "let_declaration":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("let", "=", ";", "mut", ":")]
        return Node("let", children=children, source_file=sf, line=_line(ts))

    # === Expressions ===

    if t == "call_expression":
        func = ts.child_by_field_name("function")
        args = ts.child_by_field_name("arguments")
        children = []
        if func:
            children.append(_convert(func, sf))
        if args:
            for c in args.children:
                if c.type not in ("(", ")", ","):
                    children.append(_convert(c, sf))
        return Node("call", children=children, source_file=sf, line=_line(ts))

    if t == "method_call_expression":
        name_node = ts.child_by_field_name("name")
        args_node = ts.child_by_field_name("arguments")
        # First child (before the dot) is the receiver
        receiver = None
        for c in ts.children:
            if c == name_node:
                break
            if c.type == ".":
                continue
            receiver = c
        children = []
        if receiver:
            children.append(_convert(receiver, sf))
        if name_node:
            children.append(Node("ident", value=_text(name_node),
                                  source_file=sf, line=_line(name_node)))
        if args_node:
            for c in args_node.children:
                if c.type not in ("(", ")", ","):
                    children.append(_convert(c, sf))
        return Node("method_call", children=children,
                     source_file=sf, line=_line(ts))

    if t == "field_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type != "."]
        return Node("field", children=children, source_file=sf, line=_line(ts))

    if t == "index_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("[", "]")]
        return Node("index", children=children, source_file=sf, line=_line(ts))

    if t == "binary_expression":
        children = []
        op = ""
        for c in ts.children:
            if c.type in ("+", "-", "*", "/", "%", "==", "!=", "<", ">",
                          "<=", ">=", "&&", "||", "&", "|", "^", "<<", ">>"):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("binary", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "unary_expression":
        children = []
        op = ""
        for c in ts.children:
            if c.type in ("!", "-"):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("unary", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "reference_expression":
        children = []
        mutable = False
        for c in ts.children:
            if c.type == "&":
                continue
            if c.type == "mutable_specifier":
                mutable = True
                continue
            children.append(_convert(c, sf))
        return Node("ref", children=children, value="&mut" if mutable else "&",
                     source_file=sf, line=_line(ts))

    if t == "dereference_expression":
        children = [_convert(c, sf) for c in ts.children if c.type != "*"]
        return Node("deref", children=children, source_file=sf, line=_line(ts))

    if t == "assignment_expression":
        children = []
        op = "="
        for c in ts.children:
            if c.type in ("=", "+=", "-=", "*=", "/=", "%="):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("assign", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "compound_assignment_expr":
        children = []
        op = ""
        for c in ts.children:
            if c.type in ("+=", "-=", "*=", "/=", "%=", "&=", "|=", "^="):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("assign", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "range_expression":
        children = []
        op = ""
        for c in ts.children:
            if c.type in ("..", "..="):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("range", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "tuple_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("(", ")", ",")]
        return Node("tuple", children=children, source_file=sf, line=_line(ts))

    if t == "closure_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("|", "move")]
        return Node("closure", children=children, source_file=sf, line=_line(ts))

    if t == "type_cast_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type != "as"]
        return Node("cast", children=children, source_file=sf, line=_line(ts))

    if t == "parenthesized_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("(", ")")]
        if len(children) == 1:
            return children[0]  # Unwrap — semantic no-op
        return Node("paren", children=children, source_file=sf, line=_line(ts))

    if t == "else_clause":
        children = [_convert(c, sf) for c in ts.children
                    if c.type != "else"]
        if len(children) == 1:
            return children[0]
        return Node("else", children=children, source_file=sf, line=_line(ts))

    if t == "macro_invocation":
        macro = ts.child_by_field_name("macro")
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("!", "(", ")", "{", "}", "[", "]")]
        name = _text(macro).rstrip("!") if macro else ""
        return Node("macro", children=children, value=name,
                     source_file=sf, line=_line(ts))

    # === Atoms ===

    if t == "identifier":
        return Node("ident", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "field_identifier":
        return Node("ident", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "self":
        return Node("self", source_file=sf, line=_line(ts))

    if t in ("integer_literal", "float_literal"):
        return Node("literal", value=_text(ts), source_file=sf, line=_line(ts))

    if t in ("string_literal", "char_literal", "raw_string_literal"):
        return Node("literal", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "boolean_literal":
        return Node("literal", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "scoped_identifier":
        return Node("ident", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "type_identifier":
        return Node("type", value=_text(ts), source_file=sf, line=_line(ts))

    if t in ("primitive_type", "generic_type"):
        return Node("type", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "mutable_specifier":
        return Node("mut", source_file=sf, line=_line(ts))

    if t == "token_tree":
        # Macro arguments — convert children
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("(", ")", "{", "}", "[", "]", ",")]
        return Node("token_tree", children=children,
                     source_file=sf, line=_line(ts))

    # === Patterns (for match arms, if-let) ===

    if t in ("tuple_struct_pattern", "struct_pattern"):
        return Node("pattern", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "or_pattern":
        children = [_convert(c, sf) for c in ts.children if c.type != "|"]
        return Node("or_pattern", children=children,
                     source_file=sf, line=_line(ts))

    if t == "_":
        return Node("wildcard", source_file=sf, line=_line(ts))

    # === Fallback ===
    children = []
    for c in ts.children:
        if c.type in (";", ",", "(", ")", "{", "}", "[", "]", ".", "::",
                      "=>", ":", "if", "else", "match", "for", "while",
                      "loop", "return", "break", "continue", "let", "mut",
                      "fn", "pub", "use", "mod", "struct", "enum", "impl",
                      "trait", "where", "async", "await", "move", "ref",
                      "as", "in"):
            continue
        children.append(_convert(c, sf))
    if len(children) == 1:
        return children[0]
    return Node(t, children=children, source_file=sf, line=_line(ts))
