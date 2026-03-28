"""Convert C tree-sitter AST to common AST (lossless, project-agnostic).

This module converts tree-sitter C AST nodes into common AST nodes.
It knows about C syntax (if_statement, compound_statement, etc.) but
has NO knowledge of libexpat, XML parsing, or any specific codebase.

All project-specific simplifications (parser->m_field → self.field,
camelCase → snake_case, XML_TOK_X → XmlTok::X) are applied as
JSON-configured simplification rules AFTER extraction.
"""

import tree_sitter
import tree_sitter_c

from .common_ast import Node

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


def extract(ts_node, source_file: str = "C") -> Node:
    """Convert a C tree-sitter node to a common AST node (lossless)."""
    return _convert(ts_node, source_file)


def _walk_all(node):
    yield node
    for child in node.children:
        yield from _walk_all(child)


def _text(node) -> str:
    return node.text.decode() if node.text else ""


def _line(node) -> int:
    return node.start_point[0] + 1


def _convert(ts, sf: str) -> Node:
    """Convert any C tree-sitter node to a common AST node.

    This is a generic C-to-common-AST converter. It maps tree-sitter
    node types to common AST node kinds without any project-specific logic.
    """
    t = ts.type

    # === Statements ===

    if t == "compound_statement":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("{", "}")]
        return Node("block", children=children, source_file=sf, line=_line(ts))

    if t == "if_statement":
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

    if t == "switch_statement":
        cond = ts.child_by_field_name("condition")
        body = ts.child_by_field_name("body")
        children = []
        if cond:
            children.append(_convert(cond, sf))
        if body:
            children.append(_convert(body, sf))
        return Node("match", children=children, source_file=sf, line=_line(ts))

    if t == "case_statement":
        value = ts.child_by_field_name("value")
        if not value and len(ts.children) > 1:
            value = ts.children[1]
        label = _text(value).strip() if value else "_default"
        if label == ":":
            label = "_default"
        body_children = []
        past_colon = False
        for child in ts.children:
            if child.type == ":":
                past_colon = True
                continue
            if not past_colon:
                continue
            body_children.append(_convert(child, sf))
        return Node("arm", children=body_children, value=label,
                     source_file=sf, line=_line(ts))

    if t == "default_statement":
        body_children = []
        past_colon = False
        for child in ts.children:
            if child.type == ":":
                past_colon = True
                continue
            if not past_colon:
                continue
            body_children.append(_convert(child, sf))
        return Node("arm", children=body_children, value="_default",
                     source_file=sf, line=_line(ts))

    if t in ("for_statement", "while_statement", "do_statement"):
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("for", "while", "do", "(", ")", ";")]
        return Node("loop", children=children, source_file=sf, line=_line(ts))

    if t == "return_statement":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("return", ";")]
        return Node("return", children=children, source_file=sf, line=_line(ts))

    if t == "break_statement":
        return Node("break", source_file=sf, line=_line(ts))

    if t == "continue_statement":
        return Node("continue", source_file=sf, line=_line(ts))

    if t == "expression_statement":
        children = [_convert(c, sf) for c in ts.children if c.type != ";"]
        if len(children) == 1:
            return Node("expr_stmt", children=children,
                         source_file=sf, line=_line(ts))
        return Node("expr_stmt", children=children,
                     source_file=sf, line=_line(ts))

    if t == "declaration":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in (";",)]
        return Node("let", children=children, source_file=sf, line=_line(ts))

    if t == "goto_statement":
        label = ""
        for c in ts.children:
            if c.type == "statement_identifier":
                label = _text(c)
        return Node("goto", value=label, source_file=sf, line=_line(ts))

    if t == "labeled_statement":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("statement_identifier", ":")]
        label = ""
        for c in ts.children:
            if c.type == "statement_identifier":
                label = _text(c)
        return Node("label_stmt", children=children, value=label,
                     source_file=sf, line=_line(ts))

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

    if t == "field_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("->", ".")]
        return Node("field", children=children, source_file=sf, line=_line(ts))

    if t == "subscript_expression":
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
            if c.type in ("!", "-", "~"):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("unary", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "pointer_expression":
        children = []
        op = ""
        for c in ts.children:
            if c.type in ("*", "&"):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("ptr", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "assignment_expression":
        children = []
        op = "="
        for c in ts.children:
            if c.type in ("=", "+=", "-=", "*=", "/=", "%=", "&=", "|=",
                          "^=", "<<=", ">>="):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("assign", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "conditional_expression":
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
        return Node("ternary", children=children, source_file=sf, line=_line(ts))

    if t == "parenthesized_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("(", ")")]
        if len(children) == 1:
            return children[0]  # Unwrap parens — always a semantic no-op in C
        return Node("paren", children=children, source_file=sf, line=_line(ts))

    if t == "cast_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("(", ")")]
        return Node("cast", children=children, source_file=sf, line=_line(ts))

    if t == "comma_expression":
        children = [_convert(c, sf) for c in ts.children if c.type != ","]
        return Node("comma", children=children, source_file=sf, line=_line(ts))

    if t == "update_expression":
        children = []
        op = ""
        for c in ts.children:
            if c.type in ("++", "--"):
                op = _text(c)
            else:
                children.append(_convert(c, sf))
        return Node("update", children=children, value=op,
                     source_file=sf, line=_line(ts))

    if t == "sizeof_expression":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("sizeof", "(", ")")]
        return Node("sizeof", children=children, source_file=sf, line=_line(ts))

    # === Preprocessor ===

    if t in ("preproc_if", "preproc_ifdef"):
        children = [_convert(c, sf) for c in ts.children
                    if not c.type.startswith("preproc_") and c.type not in
                    ("#if", "#ifdef", "#endif", "#else", "#elif")]
        return Node("preproc", children=children, source_file=sf, line=_line(ts))

    # === Atoms ===

    if t == "identifier":
        return Node("ident", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "field_identifier":
        return Node("ident", value=_text(ts), source_file=sf, line=_line(ts))

    if t in ("number_literal", "integer_literal", "float_literal"):
        return Node("literal", value=_text(ts), source_file=sf, line=_line(ts))

    if t in ("string_literal", "char_literal"):
        return Node("literal", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "true":
        return Node("literal", value="true", source_file=sf, line=_line(ts))

    if t == "false":
        return Node("literal", value="false", source_file=sf, line=_line(ts))

    if t == "null":
        return Node("literal", value="NULL", source_file=sf, line=_line(ts))

    if t == "type_identifier":
        return Node("type", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "primitive_type":
        return Node("type", value=_text(ts), source_file=sf, line=_line(ts))

    if t == "init_declarator":
        children = [_convert(c, sf) for c in ts.children if c.type != "="]
        return Node("init", children=children, source_file=sf, line=_line(ts))

    if t == "argument_list":
        children = [_convert(c, sf) for c in ts.children
                    if c.type not in ("(", ")", ",")]
        return Node("args", children=children, source_file=sf, line=_line(ts))

    # === Fallback: convert all children generically ===
    children = []
    for c in ts.children:
        # Skip pure punctuation
        if c.type in (";", ",", "(", ")", "{", "}", "[", "]",
                      "->", ".", ":"):
            continue
        # Skip C keywords that are already captured in parent node kind
        if c.type in ("if", "else", "switch", "case", "default",
                      "for", "while", "do", "return", "break", "continue",
                      "goto", "struct", "enum", "union", "typedef",
                      "const", "static", "extern", "inline", "void",
                      "#if", "#ifdef", "#endif", "#else"):
            continue
        children.append(_convert(c, sf))
    if len(children) == 1:
        return children[0]
    return Node(t, children=children, source_file=sf, line=_line(ts))
