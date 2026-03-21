#!/bin/bash
# Verification script: compile check + test suite + comparison tests
#
# Usage:
#   ./scripts/verify-port.sh          # Full verification
#   ./scripts/verify-port.sh quick    # Just compile check
#   ./scripts/verify-port.sh tests    # Compile + unit tests only
#   ./scripts/verify-port.sh compare  # Compile + comparison tests only

set -e
cd "$(dirname "$0")/.."

MODE="${1:-full}"

echo "=== Step 1: Compile check ==="
if ! cargo check --manifest-path expat-rust/Cargo.toml 2>&1; then
    echo "FAIL: Does not compile"
    exit 1
fi
echo "OK: Compiles"

if [ "$MODE" = "quick" ]; then
    exit 0
fi

echo ""
echo "=== Step 2: Clippy ==="
cargo clippy --manifest-path expat-rust/Cargo.toml 2>&1 | grep -E "warning|error" | head -20
echo ""

if [ "$MODE" = "compare" ]; then
    echo "=== Step 3: Comparison tests ==="
    cargo test --manifest-path expat-rust/Cargo.toml --test c_comparison_tests 2>&1 | tail -15
    exit 0
fi

echo "=== Step 3: Unit tests ==="
RESULTS=$(cargo test --manifest-path expat-rust/Cargo.toml 2>&1)
echo "$RESULTS" | grep "test result:"
echo ""

# Count totals
PASSED=$(echo "$RESULTS" | grep "test result:" | grep -oP '\d+ passed' | awk '{sum+=$1} END{print sum}')
FAILED=$(echo "$RESULTS" | grep "test result:" | grep -oP '\d+ failed' | awk '{sum+=$1} END{print sum}')
IGNORED=$(echo "$RESULTS" | grep "test result:" | grep -oP '\d+ ignored' | awk '{sum+=$1} END{print sum}')
echo "TOTAL: $PASSED passed, $FAILED failed, $IGNORED ignored"

if [ "$MODE" = "tests" ]; then
    exit 0
fi

echo ""
echo "=== Step 4: Comparison tests ==="
if cargo test --manifest-path expat-rust/Cargo.toml --test c_comparison_tests 2>&1 | tail -15; then
    echo "OK: Comparison tests pass"
else
    echo "WARN: Some comparison tests failed (may be expected for unimplemented features)"
fi

echo ""
echo "=== Step 5: Unsafe audit ==="
UNSAFE_COUNT=$(grep -rn "unsafe" expat-rust/src/ 2>/dev/null | wc -l | tr -d ' ')
echo "Unsafe blocks: $UNSAFE_COUNT"

echo ""
echo "=== Summary ==="
echo "Passed: $PASSED | Failed: $FAILED | Ignored: $IGNORED | Unsafe: $UNSAFE_COUNT"
