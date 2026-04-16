"""Common AST representation for lossless cross-language comparison.

Both C and Rust tree-sitter ASTs are converted to this common form.
The common AST preserves ALL semantic information — nothing is dropped.
If two common ASTs are equal, the code is semantically identical.

Simplification rules normalize syntax differences (naming, punctuation)
but never drop nodes. Match rules define what C/Rust pattern pairs are
considered equivalent during top-down comparison.
"""

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class Node:
    """A node in the common AST.

    Every tree-sitter node maps to exactly one common AST node.
    No information is lost in the conversion.
    """
    kind: str  # Node type (see below)
    children: list['Node'] = field(default_factory=list)
    # For leaf nodes / labels:
    value: str = ""  # Identifier name, literal value, operator symbol
    # Source location:
    source_file: str = ""
    line: int = 0

    def __repr__(self):
        if self.value:
            return f"{self.kind}({self.value})"
        if self.children:
            return f"{self.kind}[{len(self.children)}]"
        return self.kind

    def dump(self, indent=0, max_depth=10) -> str:
        """Pretty-print the AST."""
        if indent > max_depth:
            return "  " * indent + "..."
        prefix = "  " * indent
        parts = [f"{prefix}{self.kind}"]
        if self.value:
            val = self.value[:60].replace('\n', '\\n')
            parts.append(f"({val})")
        if self.line:
            parts.append(f"  @{self.line}")
        line = "".join(parts)
        lines = [line]
        for child in self.children:
            lines.append(child.dump(indent + 1, max_depth))
        return "\n".join(lines)

    def walk(self):
        """Yield all nodes in pre-order."""
        yield self
        for child in self.children:
            yield from child.walk()


# ========= Common AST node kinds =========
#
# Statements:
#   block        — sequence of statements { ... }
#   if           — if (cond) { then } [else { else }]
#   match        — switch/match on expression
#   arm          — case/arm in match (pattern + body)
#   loop         — for/while/do/loop
#   return       — return expression
#   break        — break
#   continue     — continue
#   expr_stmt    — expression used as statement
#   let          — variable declaration/binding
#   label_stmt   — labeled statement (goto target)
#   goto         — goto label
#
# Expressions:
#   call         — function/method call
#   field        — field access (a.b or a->b)
#   index        — array/slice index (a[i] or a[i..j])
#   binary       — binary operation (a + b, a == b, a && b)
#   unary        — unary operation (!a, *a, &a, -a)
#   assign       — assignment (a = b, a += b)
#   ternary      — conditional expression (a ? b : c)
#   cast         — type cast ((int)x or x as i32)
#   range        — range expression (start..end)
#   tuple        — tuple expression (a, b, c)
#   closure      — lambda/closure expression
#   if_let       — Rust if-let pattern (if let Some(x) = expr)
#   match_expr   — match used as expression (Rust: let x = match ...)
#
# Atoms:
#   ident        — identifier (variable/function name)
#   literal      — numeric, string, char, bool literal
#   self         — self/this reference
#   type         — type name (in casts, sizeof, etc.)
#
# Special:
#   macro        — macro invocation
#   preproc      — preprocessor directive (#if, #ifdef)


@dataclass
class MatchRule:
    """A bidirectional match rule for cross-language comparison.

    Defines a C pattern and a Rust pattern that should be considered
    equivalent. When the comparator encounters the C pattern on the left
    and the Rust pattern on the right (or vice versa), it matches their
    captured variables recursively.

    Example:
        C:    call(ident(poolStoreString), $pool, $enc, $start, $end)
        Rust: index($data, range($start, $end))

    This means: C's poolStoreString(pool, enc, start, end) is equivalent
    to Rust's data[start..end]. The $start and $end must match recursively.
    $pool, $enc, $data are free (not required to match anything).
    """
    name: str
    c_pattern: 'Pattern'
    rust_pattern: 'Pattern'
    justification: str = ""
    # Whether this is a principled rule or temporary
    status: str = "verified"  # "verified", "temporary", "needs_fix"


@dataclass
class Pattern:
    """A pattern for matching against common AST nodes.

    Patterns can contain:
    - Exact matches: kind="call", value="foo"
    - Captures: kind="$var" (match anything, bind to $var)
    - Wildcards: kind="*" (match anything, don't bind)
    - Child patterns: children=[Pattern, Pattern, ...]
    - Rest captures: children=[Pattern, "$rest..."] (match remaining)
    """
    kind: str = ""  # Node kind to match, or "$var" for capture, "*" for wildcard
    value: str = ""  # Value to match, or "$var" for capture
    children: list['Pattern'] = field(default_factory=list)

    def is_capture(self) -> bool:
        return self.kind.startswith("$")

    def is_wildcard(self) -> bool:
        return self.kind == "*"

    def capture_name(self) -> str:
        if self.kind.startswith("$"):
            return self.kind[1:].rstrip(".")
        if self.value.startswith("$"):
            return self.value[1:].rstrip(".")
        return ""

    def is_rest(self) -> bool:
        """Is this a rest capture ($args...)?"""
        return self.kind.endswith("...") or self.value.endswith("...")
