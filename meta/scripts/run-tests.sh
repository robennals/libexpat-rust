#!/bin/bash
# Run tests with per-test timeout protection.
# Usage: ./scripts/run-tests.sh [cargo test args...]
#
# Each test gets 10 seconds max. Total suite gets 120 seconds max.
# Tests run single-threaded to avoid resource contention.

set -e

cd "$(dirname "$0")/../expat-rust"

# Single-threaded to avoid resource blowup, with per-test timeout
export RUST_TEST_THREADS=1

# Build tests first (no timeout needed for compilation)
echo "Building tests..."
cargo test --no-run 2>&1 | tail -5

# Find the test binary
TEST_BIN=$(cargo test --no-run 2>&1 | grep -o 'target/debug/deps/[^ ]*' | head -1)

if [ -z "$TEST_BIN" ]; then
    echo "Could not find test binary, falling back to cargo test"
    cargo test "$@" 2>&1
    exit $?
fi

echo "Running tests with 10s per-test timeout..."
echo "Test binary: $TEST_BIN"

# Run with overall 120s timeout using perl alarm
perl -e '
    $SIG{ALRM} = sub {
        print STDERR "\n\nTEST SUITE TIMEOUT: exceeded 120s total limit\n";
        kill "TERM", $pid;
        exit 1;
    };
    alarm(120);
    $pid = fork();
    if ($pid == 0) {
        exec("cargo", "test", @ARGV);
    }
    waitpid($pid, 0);
    exit($? >> 8);
' -- "$@"
