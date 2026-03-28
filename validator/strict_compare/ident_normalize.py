"""Normalize identifiers for comparison.

Strips prefixes, converts case, and produces a canonical identifier
form. Two identifiers that refer to the same thing should normalize
to the same string.

The comparison ignores case, underscores, and :: when matching.
"""

import re


def normalize_ident(token: str) -> str:
    """Normalize an identifier for comparison.

    Strips:
    - m_ prefix (C member prefix)
    - self. prefix (Rust/canonical)
    - Module:: prefixes (Rust)
    - XML_TOK_ / XML_ERROR_ / XML_ROLE_ prefixes → XmlTok/XmlError/Role

    Then lowercases and strips underscores for comparison.
    """
    s = token

    # Strip common prefixes
    if s.startswith("m_"):
        s = s[2:]
    if s.startswith("self."):
        s = s[5:]

    # Enum normalization: strip enum prefixes
    # XML_TOK_FOO_BAR → FOO_BAR
    # XmlTok::FooBar → FooBar
    if s.startswith("XML_TOK_"):
        s = s[8:]
    elif s.startswith("XML_ERROR_"):
        s = s[10:]
    elif s.startswith("XML_ROLE_"):
        s = s[9:]

    # Strip :: module paths (XmlTok::X → X, xmltok_impl::foo → foo)
    if "::" in s:
        s = s.split("::")[-1]

    # Strip common type prefixes
    if s.startswith("XmlTok"):
        s = s[6:] or s  # Don't produce empty
    if s.startswith("XmlError"):
        s = s[8:] or s
    if s.startswith("Role"):
        s = s[4:] or s

    # Known variable name mappings (C name → canonical)
    _VAR_MAP = {
        "s": "pos",       # C current-position pointer = Rust pos/start offset
        "next": "next",   # Same name
        "end": "end",     # Same name
        "start": "pos",   # Rust start = C s = canonical pos
        "tok": "tok",     # Same name
    }
    if s.lower() in _VAR_MAP:
        return _VAR_MAP[s.lower()]

    # Canonical form: lowercase, no underscores
    return s.lower().replace("_", "")


def tokens_match(c_token: str, r_token: str) -> bool:
    """Do two tokens match (identifier-insensitive)?

    For identifiers: normalize and compare.
    For everything else: exact match.
    """
    if c_token == r_token:
        return True

    # Both must be identifiers (start with letter/underscore)
    if _is_ident(c_token) and _is_ident(r_token):
        return normalize_ident(c_token) == normalize_ident(r_token)

    return False


def _is_ident(token: str) -> bool:
    """Is this token an identifier?"""
    return bool(token) and (token[0].isalpha() or token[0] == "_")
