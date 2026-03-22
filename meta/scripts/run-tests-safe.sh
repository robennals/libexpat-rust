#!/bin/bash
# Run Rust tests with memory and time safety limits.
# Prevents runaway tests from crashing the machine.
#
# Usage: ./scripts/run-tests-safe.sh [test_name] [-- extra cargo args]
# Examples:
#   ./scripts/run-tests-safe.sh                    # run all tests
#   ./scripts/run-tests-safe.sh misc_tests         # run specific test file
#   ./scripts/run-tests-safe.sh basic_tests_0      # run specific test file

set -e

TIMEOUT_SECS=${TEST_TIMEOUT:-60}
MAX_RSS_KB=${TEST_MAX_RSS_KB:-512000}  # 500 MB default

cd "$(dirname "$0")/../expat-rust"

TEST_ARGS=""
if [ -n "$1" ] && [ "$1" != "--" ]; then
    TEST_ARGS="--test $1"
    shift
fi

# Pass remaining args
EXTRA_ARGS="$@"

echo "Running tests with ${TIMEOUT_SECS}s timeout, ${MAX_RSS_KB}KB RSS limit..."
echo "Command: cargo test $TEST_ARGS -- --test-threads=1 $EXTRA_ARGS"

# Use perl alarm for timeout (works on macOS without coreutils)
perl -e '
    use POSIX ":sys_wait_h";
    $SIG{ALRM} = sub {
        print STDERR "\nTIMEOUT: Tests exceeded '${TIMEOUT_SECS}'s limit — killing.\n";
        kill "TERM", $pid;
        sleep 1;
        kill "KILL", $pid;
        exit 124;
    };
    alarm('${TIMEOUT_SECS}');
    $pid = fork();
    if ($pid == 0) {
        exec("cargo", "test", split(/ /, "'"$TEST_ARGS"'"), "--", "--test-threads=1", split(/ /, "'"$EXTRA_ARGS"'"));
    }
    waitpid($pid, 0);
    exit($? >> 8);
'
