"""Skeleton IR nodes for language-agnostic structural comparison."""

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class SkeletonNode:
    """A node in the semantic skeleton tree.

    Represents a language-agnostic semantic operation extracted from
    either C or Rust AST. The skeleton strips syntactic differences
    and retains only semantically meaningful operations.
    """
    kind: str  # sequence, match, arm, branch, loop, call, return, assign, handler_dispatch, break
    label: str = ""  # normalized name
    args: list[str] = field(default_factory=list)  # normalized arguments
    expr: Optional['ExprInfo'] = None  # parsed expression details (operators, literals)
    children: list['SkeletonNode'] = field(default_factory=list)
    source_file: str = ""
    source_start: int = 0  # line number
    source_end: int = 0

    def __repr__(self):
        parts = [f"{self.kind}"]
        if self.label:
            parts.append(f"({self.label})")
        if self.args:
            parts.append(f"[{', '.join(self.args)}]")
        return "".join(parts)

    def dump(self, indent=0) -> str:
        """Pretty-print the skeleton tree."""
        prefix = "  " * indent
        line = f"{prefix}{self!r}"
        if self.source_start:
            line += f"  @{self.source_start}-{self.source_end}"
        lines = [line]
        for child in self.children:
            lines.append(child.dump(indent + 1))
        return "\n".join(lines)

    def leaf_calls(self) -> list[str]:
        """Collect all call labels in tree order (for quick comparison)."""
        result = []
        if self.kind == "call":
            result.append(self.label)
        for child in self.children:
            result.extend(child.leaf_calls())
        return result


@dataclass
class ExprInfo:
    """Parsed expression details for deep comparison.

    Captures the structure of an expression — operators, identifiers,
    and literals — in a language-agnostic form.
    """
    operator: str = ""  # ==, !=, <, >, <=, >=, &&, ||, !, or "" for simple exprs
    identifiers: list[str] = field(default_factory=list)  # normalized variable names
    literals: list[str] = field(default_factory=list)  # numeric/string literal values
    negated: bool = False  # ! prefix
    sub_exprs: list['ExprInfo'] = field(default_factory=list)  # for compound expressions (a && b)

    def __repr__(self):
        parts = []
        if self.negated:
            parts.append("!")
        if self.operator:
            parts.append(f"op:{self.operator}")
        if self.identifiers:
            parts.append(f"ids:{self.identifiers}")
        if self.literals:
            parts.append(f"lits:{self.literals}")
        if self.sub_exprs:
            parts.append(f"sub:[{', '.join(repr(s) for s in self.sub_exprs)}]")
        return f"Expr({' '.join(parts)})"


@dataclass
class Mismatch:
    """A structural mismatch between C and Rust skeletons."""
    c_node: Optional[SkeletonNode]
    r_node: Optional[SkeletonNode]
    reason: str
    context: str = ""  # e.g., "in arm XmlTok::EntityRef"
    severity: str = "ERROR"  # ERROR, WARNING, INFO

    def __repr__(self):
        loc = ""
        if self.c_node and self.c_node.source_start:
            loc += f" C@{self.c_node.source_start}"
        if self.r_node and self.r_node.source_start:
            loc += f" Rust@{self.r_node.source_start}"
        ctx = f" [{self.context}]" if self.context else ""
        return f"[{self.severity}]{ctx}{loc}: {self.reason}"
