#!/bin/bash
# Check for C test regressions.
# Compares actual test results against known-failures.txt.
# Exits 0 if no regressions. Exits 1 if a previously passing test now fails.
#
# Usage: ./c-tests-runner/check-regressions.sh [path-to-runner]

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
KNOWN_FAILURES="$SCRIPT_DIR/known-failures.txt"
RUNNER="${1:-./target/debug/c-tests-runner}"

if [ ! -f "$KNOWN_FAILURES" ]; then
    echo "ERROR: known-failures.txt not found at $KNOWN_FAILURES"
    exit 1
fi

if [ ! -x "$RUNNER" ]; then
    echo "ERROR: test runner not found at $RUNNER"
    echo "Build with: cargo build -p c-tests-runner"
    exit 1
fi

# Extract known failure test names (strip comments and blank lines)
KNOWN=$(grep -v '^\s*#' "$KNOWN_FAILURES" | grep -v '^\s*$' | awk '{print $1}' | sort)

# Run tests and capture results (runner may exit non-zero due to C assert failures)
echo "Running C test suite..."
RESULTS=$("$RUNNER" 2>/dev/null || true)

PASS_TESTS=$(echo "$RESULTS" | grep "^PASS:" | awk '{print $2}' | sort)
FAIL_TESTS=$(echo "$RESULTS" | grep "^FAIL " | awk '{print $3}' | sort)

PASS_COUNT=$(echo "$PASS_TESTS" | wc -l | tr -d ' ')
FAIL_COUNT=$(echo "$FAIL_TESTS" | wc -l | tr -d ' ')
TOTAL=$((PASS_COUNT + FAIL_COUNT))

echo ""
echo "=== C Test Suite Results ==="
echo "Pass: $PASS_COUNT / $TOTAL"
echo "Fail: $FAIL_COUNT / $TOTAL"
echo ""

# Check for regressions: tests that FAIL but are NOT in known-failures
REGRESSIONS=""
while IFS= read -r test; do
    [ -z "$test" ] && continue
    if ! echo "$KNOWN" | grep -q "^${test}$"; then
        REGRESSIONS="$REGRESSIONS $test"
    fi
done <<< "$FAIL_TESTS"

# Check for newly passing: tests in known-failures that now PASS
NEWLY_PASSING=""
while IFS= read -r test; do
    [ -z "$test" ] && continue
    if echo "$PASS_TESTS" | grep -q "^${test}$"; then
        NEWLY_PASSING="$NEWLY_PASSING $test"
    fi
done <<< "$KNOWN"

# Report
if [ -n "$NEWLY_PASSING" ]; then
    echo "=== Newly Passing (remove from known-failures.txt) ==="
    for test in $NEWLY_PASSING; do
        echo "  FIXED: $test"
    done
    echo ""
fi

if [ -n "$REGRESSIONS" ]; then
    echo "=== REGRESSIONS (previously passing tests now failing) ==="
    for test in $REGRESSIONS; do
        echo "  REGRESSION: $test"
    done
    echo ""
    echo "ERROR: $(($(echo $REGRESSIONS | wc -w))) regression(s) detected!"
    echo "If these failures are expected, add them to known-failures.txt"
    exit 1
fi

echo "=== No regressions detected ==="

if [ -n "$NEWLY_PASSING" ]; then
    echo ""
    echo "NOTE: $(echo $NEWLY_PASSING | wc -w | tr -d ' ') test(s) are now passing."
    echo "Please remove them from known-failures.txt to lock in the improvement."
fi

exit 0
