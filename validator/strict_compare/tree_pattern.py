"""Tree pattern DSL for skeleton rewrite rules.

Patterns use a simple syntax close to the skeleton dump format:

    call($fn where /.*_tok$/)       — match call with label matching regex, capture as $fn
    match($m) { $arms }             — match a match node, capture it, capture children
    branch($cond) { $body }         — match branch, capture condition label and body
    sequence { call(foo); $rest }   — match sequence starting with call(foo)
    $captured_name                  — reference a captured node (in output)

Pattern syntax:
    pattern     := kind '(' label ')' ['{' children '}']
    kind        := identifier
    label       := literal | '$' name | '$' name 'where' '/' regex '/' | '*'
    children    := pattern [';' pattern]* | '$' name
    literal     := quoted_string | identifier

Output syntax is the same, with $captures substituted.
"""

import re
from .nodes import SkeletonNode


def parse_pattern(text: str) -> dict:
    """Parse a pattern string into a pattern dict.

    Examples:
        "call($fn where /.*_tok$/)"
        "match($m) { $arms }"
        "sequence { call($fn where /.*_tok$/); match($m) }"
        "$captured"
    """
    text = text.strip()

    # Variable reference: $name
    if text.startswith("$") and " " not in text and "{" not in text:
        return {"capture_ref": text[1:]}

    # Ellipsis: ...
    if text == "...":
        return {"ellipsis": True}

    # kind(label) { children }
    m = re.match(r'^(\w+)\s*\(\s*(.*?)\s*\)\s*(\{.*\})?\s*$', text, re.DOTALL)
    if not m:
        # Try kind { children } (no label)
        m = re.match(r'^(\w+)\s*(\{.*\})?\s*$', text, re.DOTALL)
        if m:
            kind = m.group(1)
            children_block = m.group(2)
            result = {"kind": kind}
            if children_block:
                result["children"] = _parse_children_block(children_block)
            return result
        raise ValueError(f"Cannot parse pattern: {text!r}")

    kind = m.group(1)
    label_text = m.group(2)
    children_block = m.group(3)

    result = {"kind": kind}

    # Parse label
    if label_text:
        label_info = _parse_label(label_text)
        result.update(label_info)

    # Parse children block
    if children_block:
        result["children"] = _parse_children_block(children_block)

    return result


def _parse_label(text: str) -> dict:
    """Parse a label expression.

    Formats:
        "literal_text"       → {"label": "literal_text"}
        $name                → {"capture": "name"}
        $name where /regex/  → {"capture": "name", "label_regex": "regex"}
        *                    → {"label": "*"}
    """
    text = text.strip()

    if text == "*":
        return {"label": "*"}

    # $name where /regex/
    m = re.match(r'^\$(\w+)\s+where\s+/(.+)/$', text)
    if m:
        return {"capture": m.group(1), "label_regex": m.group(2)}

    # $name
    if text.startswith("$"):
        return {"capture": text[1:]}

    # Literal (possibly quoted)
    if text.startswith('"') and text.endswith('"'):
        return {"label": text[1:-1]}

    return {"label": text}


def _parse_children_block(text: str) -> list[dict]:
    """Parse a { child1; child2; ... } block."""
    text = text.strip()
    if text.startswith("{"):
        text = text[1:]
    if text.endswith("}"):
        text = text[:-1]
    text = text.strip()

    if not text:
        return []

    # Split on semicolons, respecting nested braces
    parts = _split_preserving_braces(text, ";")
    return [parse_pattern(p.strip()) for p in parts if p.strip()]


def _split_preserving_braces(text: str, delimiter: str) -> list[str]:
    """Split text by delimiter, but don't split inside { }."""
    parts = []
    current = []
    depth = 0
    for char in text:
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
        elif char == delimiter[0] and depth == 0:
            parts.append("".join(current))
            current = []
            continue
        current.append(char)
    if current:
        parts.append("".join(current))
    return parts


def parse_rule(input_text: str, output_text: str) -> dict:
    """Parse input and output pattern strings into a rule dict.

    Returns a dict compatible with the sequence rewrite rule format.
    """
    input_pat = parse_pattern(input_text)
    output_pat = parse_pattern(output_text) if output_text else None
    return {
        "input": input_pat,
        "output": output_pat,
    }


# ========= Convenience: compile rules from text format =========

def compile_text_rules(rules: list[dict]) -> list[dict]:
    """Compile text-format rules into tree pattern dicts.

    Each rule has:
        "input_pattern": "call($fn where /.*_tok$/); match($m)"
        "output_pattern": "match(tok_result) { arm(Ok) { $m } }"
    """
    compiled = []
    for rule in rules:
        input_text = rule.get("input_pattern", "")
        output_text = rule.get("output_pattern")

        if not input_text:
            continue

        # Multiple patterns separated by ; at top level = children of a sequence
        parts = _split_preserving_braces(input_text, ";")
        if len(parts) > 1:
            # Multiple top-level patterns → match as children of parent
            input_pat = {
                "children": [parse_pattern(p.strip()) for p in parts if p.strip()]
            }
        else:
            input_pat = parse_pattern(input_text)

        output_pat = None
        if output_text:
            out_parts = _split_preserving_braces(output_text, ";")
            if len(out_parts) > 1:
                output_pat = {
                    "kind": "sequence",
                    "children": [parse_pattern(p.strip()) for p in out_parts if p.strip()]
                }
            else:
                output_pat = parse_pattern(output_text)

        compiled.append({
            "name": rule.get("name", ""),
            "justification": rule.get("justification", ""),
            "input": input_pat,
            "output": output_pat,
        })

    return compiled
