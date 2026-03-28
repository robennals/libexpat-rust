"""Source-level pattern matching with $variable capture.

Matches patterns against source text (as token streams), where $variables
capture balanced substrings. Used for rewriting C and Rust source code
to a canonical form before AST comparison.

Pattern syntax:
    Literal tokens match exactly.
    $variable captures one or more tokens, consuming just enough for
    the rest of the pattern to match (non-greedy, with backtracking).
    Balanced delimiters: $var will not capture across unbalanced (){}[].

Example:
    pattern: "if let Some($h) = &mut self.$f { $h($args); }"
    source:  "if let Some(handler) = &mut self.comment_handler { handler(&normalized); }"
    captures: {"h": "handler", "f": "comment_handler", "args": "&normalized"}
"""

import re


# Delimiter pairs for balanced matching
_OPEN = {"(", "{", "["}
_CLOSE = {")", "}", "]"}
_MATCH = {"(": ")", "{": "}", "[": "]"}
_RMATCH = {")": "(", "}": "{", "]": "["}


def tokenize(text: str) -> list[str]:
    """Split source text into tokens.

    Handles: identifiers, numbers, strings, operators, single-char punctuation.
    Strips whitespace and comments.
    """
    tokens = []
    i = 0
    while i < len(text):
        c = text[i]

        # Skip whitespace
        if c in " \t\n\r":
            i += 1
            continue

        # Skip C-style comments
        if c == "/" and i + 1 < len(text):
            if text[i + 1] == "/":
                # Line comment
                end = text.find("\n", i)
                i = end + 1 if end >= 0 else len(text)
                continue
            if text[i + 1] == "*":
                # Block comment
                end = text.find("*/", i + 2)
                i = end + 2 if end >= 0 else len(text)
                continue

        # $variable (only in patterns)
        if c == "$" and i + 1 < len(text) and (text[i + 1].isalpha() or text[i + 1] == "_"):
            j = i + 1
            while j < len(text) and (text[j].isalnum() or text[j] == "_"):
                j += 1
            tokens.append(text[i:j])
            i = j
            continue

        # String literal
        if c in ('"', "'"):
            j = i + 1
            while j < len(text) and text[j] != c:
                if text[j] == "\\":
                    j += 1  # skip escaped char
                j += 1
            j += 1  # include closing quote
            tokens.append(text[i:j])
            i = j
            continue

        # Multi-char operators
        if i + 1 < len(text):
            two = text[i:i + 2]
            if two in ("==", "!=", "<=", ">=", "&&", "||", "->", "::", "..",
                        "+=", "-=", "*=", "/=", "=>", "|=", "&=", "^=",
                        "<<", ">>", "++", "--"):
                tokens.append(two)
                i += 2
                continue
            if i + 2 < len(text):
                three = text[i:i + 3]
                if three in ("..=", "<<=", ">>="):
                    tokens.append(three)
                    i += 3
                    continue

        # Identifier or keyword
        if c.isalpha() or c == "_":
            j = i + 1
            while j < len(text) and (text[j].isalnum() or text[j] == "_"):
                j += 1
            tokens.append(text[i:j])
            i = j
            continue

        # Number literal
        if c.isdigit():
            j = i + 1
            while j < len(text) and (text[j].isalnum() or text[j] in "._"):
                j += 1
            tokens.append(text[i:j])
            i = j
            continue

        # Single char (punctuation, delimiters)
        tokens.append(c)
        i += 1

    return tokens


def match_pattern(pattern_tokens: list[str], source_tokens: list[str],
                  start: int = 0,
                  require_full: bool = False) -> tuple[dict[str, list[str]], int] | None:
    """Try to match a pattern against source tokens starting at `start`.

    Returns (captures, end_pos) if matched, None otherwise.
    captures maps $variable names to lists of captured tokens.

    If require_full=True, all source tokens must be consumed.

    Key properties:
    - Non-greedy: $var captures minimum tokens needed for rest to match
    - Balanced: $var won't capture across unbalanced delimiters
    - Same-var: if $var appears twice, both must capture the same tokens
    """
    captures = {}
    end = len(source_tokens) if require_full else None
    result = _match(pattern_tokens, 0, source_tokens, start, captures, end)
    if result is not None:
        return captures, result
    return None


def _match(pat: list[str], pi: int, src: list[str], si: int,
           captures: dict[str, list[str]],
           required_end: int | None = None) -> int | None:
    """Recursive pattern matching with backtracking.

    Returns the source position after the match, or None if no match.
    If required_end is set, the match must end at exactly that position.
    """
    # Base case: pattern exhausted
    if pi >= len(pat):
        if required_end is not None and si != required_end:
            return None
        return si  # Match succeeds

    token = pat[pi]

    # Literal token: must match exactly
    if not token.startswith("$"):
        if si >= len(src) or src[si] != token:
            return None
        return _match(pat, pi + 1, src, si + 1, captures, required_end)

    # Variable token: try capturing tokens
    var_name = token[1:]  # strip $
    already_bound = var_name in captures

    max_capture = len(src) - si

    # If this is the LAST token in the pattern, capture greedily
    # (take as much balanced content as possible)
    is_last = (pi == len(pat) - 1)
    if is_last:
        capture_range = range(max_capture, -1, -1)  # greedy: try max first
    else:
        capture_range = range(0, max_capture + 1)   # non-greedy: try 0 first

    for capture_len in capture_range:
        captured = src[si:si + capture_len]

        # Check balanced delimiters
        if not _is_balanced(captured):
            continue

        # Same-variable constraint: if already bound, must match exactly
        if already_bound:
            if captured != captures[var_name]:
                continue
            result = _match(pat, pi + 1, src, si + capture_len,
                            captures, required_end)
            if result is not None:
                return result
        else:
            # New binding — tentatively bind and try rest
            captures[var_name] = captured
            result = _match(pat, pi + 1, src, si + capture_len,
                            captures, required_end)
            if result is not None:
                return result
            # Backtrack: remove the binding
            del captures[var_name]

    return None


def _is_balanced(tokens: list[str]) -> bool:
    """Check if a token list has balanced delimiters."""
    stack = []
    for t in tokens:
        if t in _OPEN:
            stack.append(t)
        elif t in _CLOSE:
            if not stack:
                return False
            expected = _RMATCH[t]
            if stack[-1] != expected:
                return False
            stack.pop()
    return len(stack) == 0


def substitute(template_tokens: list[str],
               captures: dict[str, list[str]]) -> list[str]:
    """Substitute captured variables into a template.

    Returns the resulting token list.
    """
    result = []
    for token in template_tokens:
        if token.startswith("$"):
            var_name = token[1:]
            if var_name in captures:
                result.extend(captures[var_name])
            else:
                result.append(token)  # Keep unresolved var as-is
        else:
            result.append(token)
    return result


def tokens_to_str(tokens: list[str]) -> str:
    """Join tokens back into a string with minimal spacing."""
    if not tokens:
        return ""
    parts = []
    for i, t in enumerate(tokens):
        if i > 0:
            prev = tokens[i - 1]
            # Add space between identifiers/numbers, but not around punctuation
            if (_is_word(prev) and _is_word(t)) or \
               (_is_word(prev) and t.startswith("$")) or \
               (prev.startswith("$") and _is_word(t)):
                parts.append(" ")
        parts.append(t)
    return "".join(parts)


def _is_word(token: str) -> bool:
    """Is this token a word (identifier, keyword, number)?"""
    return bool(token) and (token[0].isalnum() or token[0] == "_")


# ========= Convenience functions =========

def match_str(pattern: str, source: str, start: int = 0,
              full: bool = True):
    """Match a pattern string against a source string.

    Returns (captures, end_pos) where captures maps var names to strings,
    or None if no match.

    If full=True (default), requires that all source tokens are consumed.
    If full=False, allows prefix matching.
    """
    pat_tokens = tokenize(pattern)
    src_tokens = tokenize(source)
    result = match_pattern(pat_tokens, src_tokens, start, require_full=full)
    if result is None:
        return None
    captures_tokens, end_pos = result
    # Convert token lists to strings
    captures_str = {k: tokens_to_str(v) for k, v in captures_tokens.items()}
    return captures_str, end_pos


def substitute_str(template: str, captures: dict[str, str]) -> str:
    """Substitute captures into a template string."""
    tmpl_tokens = tokenize(template)
    # Convert string captures to token lists
    captures_tokens = {k: tokenize(v) for k, v in captures.items()}
    result_tokens = substitute(tmpl_tokens, captures_tokens)
    return tokens_to_str(result_tokens)
