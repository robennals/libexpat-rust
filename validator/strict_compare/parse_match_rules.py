"""Parse match rules from YAML format into dicts used by strict_match.py.

Rules are in match-rules.yaml with a readable syntax:

    principled:
      - name: c_break
        skip: c
        c: "break"
        why: C switch needs explicit break.

      - name: return_1_is_void
        c: "return(literal(1))"
        r: "return"
        why: C returns 1 for success.

    temporary:
      - name: handler_dispatch_if_let
        c: "if(field(self, $handler), $body)"
        r: "if_let($pattern, $handler_expr, $body)"
        why: C if(handler) vs Rust if let Some(h).
        risk: Body details may differ.

Pattern syntax uses common AST node names:
    kind(child1, child2, ...)     — match node with children
    $var                          — capture variable
    $var...                       — capture rest of children
    ident(name)                   — identifier with exact name
    literal(value)                — literal with exact value
"""

import re
import os


def parse_rules_file(filepath: str = None) -> list[dict]:
    """Parse a match-rules.yaml file into a list of rule dicts."""
    import yaml

    if filepath is None:
        filepath = os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            "..", "match-rules.yaml"
        )

    with open(filepath) as f:
        config = yaml.safe_load(f)

    rules = []

    for rule_def in config.get("principled", []):
        rule = _convert_yaml_rule(rule_def, "principled")
        if rule:
            rules.append(rule)

    for rule_def in config.get("temporary", []):
        rule = _convert_yaml_rule(rule_def, "temporary")
        if rule:
            rules.append(rule)

    return rules


def _convert_yaml_rule(rule_def: dict, status: str) -> dict | None:
    """Convert a YAML rule definition to the dict format."""
    rule = {
        "name": rule_def.get("name", ""),
        "status": status,
        "justification": rule_def.get("why", ""),
    }

    if "risk" in rule_def:
        rule["risk"] = rule_def["risk"]

    # Skip rule
    if "skip" in rule_def:
        rule["skip"] = rule_def["skip"]

    # C pattern
    c_text = rule_def.get("c", "")
    if c_text:
        rule["c_pattern"] = parse_pattern(c_text)

    # Rust pattern
    r_text = rule_def.get("r", "")
    if r_text:
        rule["r_pattern"] = parse_pattern(r_text)

    return rule


def parse_pattern(text: str) -> dict:
    """Parse a pattern string into a pattern dict.

    Examples:
        "call(ident(foo), $arg)"
        "if(unary(!, field(self, $handler)), $body)"
        "return(literal(1))"
        "$captured"
        "break"
    """
    text = text.strip()

    # Variable: $name or $name...
    if text.startswith("$"):
        name = text[1:]
        if name.endswith("..."):
            return {"capture": name[:-3], "rest": True}
        return {"capture": name}

    # Wildcard
    if text == "*":
        return {}

    # kind(children...) or kind(value) or just kind
    m = re.match(r'^(\w+)\s*\((.*)\)\s*$', text, re.DOTALL)
    if m:
        kind = m.group(1)
        inner = m.group(2).strip()

        # Check if it's a leaf with a value: ident(foo), literal(42)
        if kind in ("ident", "literal", "type") and not _has_nested_parens(inner):
            # Could be a value or a capture
            if inner.startswith("$"):
                return {"kind": kind, "capture": inner[1:]}
            return {"kind": kind, "value": inner}

        # Otherwise parse as children
        children = _split_top_level(inner, ",")
        child_patterns = [parse_pattern(c.strip()) for c in children if c.strip()]
        return {"kind": kind, "children": child_patterns}

    # Plain kind with no parens: "break", "continue", "self"
    if re.match(r'^\w+$', text):
        return {"kind": text}

    # Operator or special value
    return {"kind": text}


def _has_nested_parens(text: str) -> bool:
    """Check if text contains nested parentheses (indicating it's children, not a value)."""
    depth = 0
    for ch in text:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if depth > 0:
            return True
    return False


def _split_top_level(text: str, delimiter: str) -> list[str]:
    """Split text by delimiter, respecting nested parentheses."""
    parts = []
    current = []
    depth = 0
    for ch in text:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif ch == delimiter and depth == 0:
            parts.append("".join(current))
            current = []
            continue
        current.append(ch)
    if current:
        parts.append("".join(current))
    return parts


if __name__ == "__main__":
    import json
    rules = parse_rules_file()
    print(json.dumps(rules, indent=2))
