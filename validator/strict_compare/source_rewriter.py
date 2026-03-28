"""Apply YAML-configured rewrite rules to source text.

Rewrites source code by finding pattern matches and applying
replacements or drops. Rules are applied repeatedly until fixpoint.
"""

import yaml
import os
from .source_pattern import tokenize, match_pattern, substitute, tokens_to_str


def load_rules(filepath: str) -> list[dict]:
    """Load rewrite rules from a YAML file."""
    with open(filepath) as f:
        config = yaml.safe_load(f)
    rules = []
    for section in ("c_rewrite_rules", "rust_rewrite_rules", "rewrite_rules"):
        for rule in config.get(section, []):
            rule["_before_tokens"] = tokenize(rule["before"])
            if "after" in rule and rule["after"] is not None:
                rule["_after_tokens"] = tokenize(rule["after"])
            rules.append(rule)
    return rules


def rewrite(source: str, rules: list[dict], max_iterations: int = 50) -> str:
    """Apply rewrite rules to source text until fixpoint.

    Each iteration scans the token stream for pattern matches and applies
    the first match found (replacing or dropping). Repeats until no rules
    fire or max_iterations reached.

    Returns the rewritten source text.
    """
    tokens = tokenize(source)

    for iteration in range(max_iterations):
        changed = False
        # Try each rule at each position
        for rule in rules:
            result = _apply_rule_once(tokens, rule)
            if result is not None:
                tokens = result
                changed = True
                break  # Restart from first rule after a change
        if not changed:
            break

    return tokens_to_str(tokens)


def _apply_rule_once(tokens: list[str], rule: dict) -> list[str] | None:
    """Try to apply a rule once anywhere in the token stream.

    Returns the new token list if applied, None if no match.
    """
    pat = rule["_before_tokens"]
    is_drop = rule.get("drop", False)
    after_tokens = rule.get("_after_tokens")

    # Scan for match at each position
    for start in range(len(tokens)):
        result = match_pattern(pat, tokens, start)
        if result is None:
            continue

        captures, end_pos = result

        if is_drop:
            # Remove the matched tokens
            return tokens[:start] + tokens[end_pos:]
        elif after_tokens is not None:
            # Substitute captures into template
            replacement = substitute(after_tokens, captures)
            return tokens[:start] + replacement + tokens[end_pos:]

    return None


def rewrite_function_body(source: str, rules: list[dict],
                          max_iterations: int = 50) -> str:
    """Rewrite a function body using rules.

    Same as rewrite() but returns just the rewritten text.
    """
    return rewrite(source, rules, max_iterations)
