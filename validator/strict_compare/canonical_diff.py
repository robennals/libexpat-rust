"""Diff canonical forms and generate fix prompts.

Compares two canonical token streams (C and Rust after rewriting)
and produces a list of differences with source line references.
"""

from .source_pattern import tokenize
from .ident_normalize import tokens_match, normalize_ident


def diff_canonical(c_canonical: str, r_canonical: str,
                   c_body: str = "", r_body: str = "",
                   c_func: str = "", r_func: str = "") -> list[dict]:
    """Diff two canonical forms.

    Returns list of difference dicts:
      {"type": "c_extra"|"r_extra"|"mismatch",
       "c_token": str, "r_token": str,
       "c_line": int, "r_line": int,
       "context": str}
    """
    c_tokens = tokenize(c_canonical)
    r_tokens = tokenize(r_canonical)

    # Build line maps from original bodies
    c_line_map = _build_line_map(c_body) if c_body else {}
    r_line_map = _build_line_map(r_body) if r_body else {}

    # LCS-based diff with identifier-insensitive matching
    diffs = _lcs_diff(c_tokens, r_tokens, c_line_map, r_line_map,
                       c_body, r_body)
    return diffs


def _build_line_map(body: str) -> dict[str, list[int]]:
    """Map tokens to their line numbers in the original body."""
    result = {}
    line = 1
    tokens_with_lines = []
    i = 0
    while i < len(body):
        c = body[i]
        if c == '\n':
            line += 1
            i += 1
            continue
        if c in ' \t\r':
            i += 1
            continue
        # Extract token at this position
        if c.isalpha() or c == '_':
            j = i + 1
            while j < len(body) and (body[j].isalnum() or body[j] == '_'):
                j += 1
            token = body[i:j]
            result.setdefault(token, []).append(line)
            i = j
        elif c.isdigit():
            j = i + 1
            while j < len(body) and (body[j].isalnum() or body[j] in '._'):
                j += 1
            token = body[i:j]
            result.setdefault(token, []).append(line)
            i = j
        else:
            i += 1
    return result


def _find_line(token: str, line_map: dict, used_lines: dict) -> int:
    """Find the source line for a token, tracking which lines were used."""
    lines = line_map.get(token, [])
    # Use the first unused line
    key = token
    used = used_lines.get(key, 0)
    if used < len(lines):
        used_lines[key] = used + 1
        return lines[used]
    # Fallback: try normalized form
    norm = normalize_ident(token)
    for orig_token, orig_lines in line_map.items():
        if normalize_ident(orig_token) == norm:
            used2 = used_lines.get(orig_token, 0)
            if used2 < len(orig_lines):
                used_lines[orig_token] = used2 + 1
                return orig_lines[used2]
    return 0


def _lcs_diff(c_tokens: list[str], r_tokens: list[str],
              c_line_map: dict, r_line_map: dict,
              c_body: str, r_body: str) -> list[dict]:
    """Diff using LCS with identifier-insensitive matching."""
    n = len(c_tokens)
    m = len(r_tokens)

    # Build LCS table
    dp = [[0] * (m + 1) for _ in range(n + 1)]
    for i in range(1, n + 1):
        for j in range(1, m + 1):
            if tokens_match(c_tokens[i - 1], r_tokens[j - 1]):
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])

    # Trace back to find diffs
    diffs = []
    c_used_lines = {}
    r_used_lines = {}
    i, j = n, m
    aligned = []

    while i > 0 or j > 0:
        if i > 0 and j > 0 and tokens_match(c_tokens[i - 1], r_tokens[j - 1]):
            aligned.append(("match", i - 1, j - 1))
            i -= 1
            j -= 1
        elif j > 0 and (i == 0 or dp[i][j - 1] >= dp[i - 1][j]):
            aligned.append(("r_extra", -1, j - 1))
            j -= 1
        else:
            aligned.append(("c_extra", i - 1, -1))
            i -= 1

    aligned.reverse()

    # Convert to diff list
    for kind, ci, ri in aligned:
        if kind == "match":
            continue
        if kind == "c_extra":
            c_tok = c_tokens[ci]
            c_line = _find_line(c_tok, c_line_map, c_used_lines)
            diffs.append({
                "type": "c_extra",
                "c_token": c_tok,
                "r_token": "",
                "c_line": c_line,
                "r_line": 0,
            })
        elif kind == "r_extra":
            r_tok = r_tokens[ri]
            r_line = _find_line(r_tok, r_line_map, r_used_lines)
            diffs.append({
                "type": "r_extra",
                "c_token": "",
                "r_token": r_tok,
                "c_line": 0,
                "r_line": r_line,
            })

    return diffs


def generate_prompt(c_func: str, r_func: str,
                    diffs: list[dict],
                    c_body: str, r_body: str,
                    c_file: str = "", r_file: str = "") -> str:
    """Generate a fix prompt from diffs.

    Returns a markdown prompt directing an agent to fix the Rust function.
    """
    if not diffs:
        return f"# {c_func} <-> {r_func}: MATCH\nNo differences found."

    c_lines = c_body.split('\n') if c_body else []
    r_lines = r_body.split('\n') if r_body else []

    # Group diffs by type
    c_extras = [d for d in diffs if d["type"] == "c_extra"]
    r_extras = [d for d in diffs if d["type"] == "r_extra"]

    # Summarize
    c_extra_tokens = [d["c_token"] for d in c_extras]
    r_extra_tokens = [d["r_token"] for d in r_extras]

    # Filter noise tokens (punctuation-only diffs)
    c_meaningful = [t for t in c_extra_tokens if len(t) > 1 or t.isalpha()]
    r_meaningful = [t for t in r_extra_tokens if len(t) > 1 or t.isalpha()]

    parts = []
    parts.append(f"# Fix `{r_func}` to match C `{c_func}`")
    parts.append(f"")
    parts.append(f"After applying rewrite rules, {len(diffs)} token differences remain.")
    parts.append(f"- {len(c_extras)} tokens in C not in Rust")
    parts.append(f"- {len(r_extras)} tokens in Rust not in C")
    parts.append(f"")

    if c_meaningful:
        parts.append(f"## C code not found in Rust")
        parts.append(f"")
        # Group by line
        by_line = {}
        for d in c_extras:
            if d["c_line"]:
                by_line.setdefault(d["c_line"], []).append(d["c_token"])
        for line_no in sorted(by_line.keys()):
            tokens = by_line[line_no]
            if line_no > 0 and line_no <= len(c_lines):
                src = c_lines[line_no - 1].strip()[:70]
                parts.append(f"- C line {line_no}: `{src}`")
                parts.append(f"  Missing tokens: {', '.join(tokens[:10])}")
        parts.append(f"")

    if r_meaningful:
        parts.append(f"## Rust code not in C")
        parts.append(f"")
        by_line = {}
        for d in r_extras:
            if d["r_line"]:
                by_line.setdefault(d["r_line"], []).append(d["r_token"])
        for line_no in sorted(by_line.keys()):
            tokens = by_line[line_no]
            if line_no > 0 and line_no <= len(r_lines):
                src = r_lines[line_no - 1].strip()[:70]
                parts.append(f"- Rust line {line_no}: `{src}`")
                parts.append(f"  Extra tokens: {', '.join(tokens[:10])}")
        parts.append(f"")

    parts.append(f"## Action")
    parts.append(f"")
    if c_meaningful and not r_meaningful:
        parts.append(f"Rust is missing logic from C. Add the equivalent Rust code.")
    elif r_meaningful and not c_meaningful:
        parts.append(f"Rust has extra logic not in C. Remove it unless there's a")
        parts.append(f"principled reason (borrow checker, memory safety, etc.).")
    else:
        parts.append(f"Both sides have differences. Restructure `{r_func}` to")
        parts.append(f"match C's `{c_func}` structure more closely.")
    parts.append(f"")
    parts.append(f"After changes, verify with:")
    parts.append(f"```bash")
    parts.append(f"cargo check -p expat-rust && cargo test -p expat-rust")
    parts.append(f"python3 validator/canonical-compare.py --dump {c_func} {r_func}")
    parts.append(f"```")

    return "\n".join(parts)
