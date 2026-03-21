#!/bin/bash
# List all currently failing tests grouped by test file
#
# Usage:
#   ./scripts/list-failures.sh              # List all failures
#   ./scripts/list-failures.sh basic_tests_0  # List failures in one file

set -e
cd "$(dirname "$0")/.."

if [ -n "$1" ]; then
    FILES="$1"
else
    FILES="basic_tests_0 basic_tests_1 basic_tests_2 basic_tests_3 basic_tests_4 basic_tests_missing ns_tests alloc_tests nsalloc_tests acc_tests misc_tests c_comparison_tests"
fi

TOTAL_PASS=0
TOTAL_FAIL=0
TOTAL_IGNORE=0

for test_file in $FILES; do
    OUTPUT=$(cargo test --manifest-path expat-rust/Cargo.toml --test "$test_file" 2>&1)
    FAILURES=$(echo "$OUTPUT" | grep "^test .* FAILED$" | sed 's/test //' | sed 's/ \.\.\. FAILED//')
    SUMMARY=$(echo "$OUTPUT" | grep "test result:" | head -1)

    PASS=$(echo "$SUMMARY" | grep -oP '\d+ passed' | awk '{print $1}')
    FAIL=$(echo "$SUMMARY" | grep -oP '\d+ failed' | awk '{print $1}')
    IGN=$(echo "$SUMMARY" | grep -oP '\d+ ignored' | awk '{print $1}')

    PASS=${PASS:-0}
    FAIL=${FAIL:-0}
    IGN=${IGN:-0}

    TOTAL_PASS=$((TOTAL_PASS + PASS))
    TOTAL_FAIL=$((TOTAL_FAIL + FAIL))
    TOTAL_IGNORE=$((TOTAL_IGNORE + IGN))

    if [ "$FAIL" -gt 0 ]; then
        echo "=== $test_file: $PASS passed, $FAIL FAILED, $IGN ignored ==="
        echo "$FAILURES" | while read -r name; do
            # Get the assertion message
            MSG=$(echo "$OUTPUT" | grep -A2 "thread '$name'" | grep "assertion" | head -1 | sed 's/.*assertion/assertion/')
            echo "  FAIL: $name"
            [ -n "$MSG" ] && echo "        $MSG"
        done
        echo ""
    fi
done

echo "========================================"
echo "TOTAL: $TOTAL_PASS passed, $TOTAL_FAIL failed, $TOTAL_IGNORE ignored"
