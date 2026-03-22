#!/bin/bash
# C2Rust Pipeline: Full workflow for converting C2Rust output to idiomatic Rust
#
# Usage:
#   ./scripts/c2rust-pipeline.sh regenerate   # Re-run c2rust transpilation
#   ./scripts/c2rust-pipeline.sh cleanup      # Run mechanical cleanup
#   ./scripts/c2rust-pipeline.sh compare      # Run comparison tests
#   ./scripts/c2rust-pipeline.sh extract FUNC # Extract a function for manual transformation
#   ./scripts/c2rust-pipeline.sh analyze      # Analyze patterns in c2rust output
#   ./scripts/c2rust-pipeline.sh all          # Run full pipeline

set -e
cd "$(dirname "$0")/.."

case "${1:-help}" in
    regenerate)
        echo "=== Regenerating C2Rust output ==="
        # Ensure cmake build dir exists with compile_commands.json
        if [ ! -f expat/build/compile_commands.json ]; then
            mkdir -p expat/build
            cd expat/build
            CC=/opt/homebrew/opt/llvm@17/bin/clang cmake .. \
                -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
                -DEXPAT_BUILD_TESTS=OFF \
                -DEXPAT_BUILD_EXAMPLES=OFF \
                -DEXPAT_BUILD_TOOLS=OFF
            # Strip -arch flags that c2rust doesn't support
            python3 -c "
import json
with open('compile_commands.json') as f:
    data = json.load(f)
for entry in data:
    cmd = entry['command']
    parts = cmd.split()
    new_parts = []
    skip_next = False
    for p in parts:
        if skip_next:
            skip_next = False
            continue
        if p in ['-arch', '-target']:
            skip_next = True
            continue
        new_parts.append(p)
    entry['command'] = ' '.join(new_parts)
with open('compile_commands.json', 'w') as f:
    json.dump(data, f, indent=2)
"
            cd ../..
        fi
        mkdir -p c2rust-output
        LLVM_LIB_DIR=/opt/homebrew/opt/llvm@17/lib \
            c2rust transpile expat/build/compile_commands.json \
            --output-dir c2rust-output
        echo "Done. Output in c2rust-output/"
        ;;

    cleanup)
        echo "=== Running mechanical cleanup ==="
        for f in c2rust-output/src/xmlparse.rs c2rust-output/src/xmlrole.rs c2rust-output/src/xmltok.rs; do
            if [ -f "$f" ]; then
                python3 scripts/c2rust-cleanup.py "$f" -i
                echo "Cleaned $f"
            fi
        done
        ;;

    compare)
        echo "=== Running comparison tests ==="
        cd expat-rust
        cargo test --test c_comparison_tests 2>&1 || true
        ;;

    extract)
        if [ -z "$2" ]; then
            echo "Usage: $0 extract FUNCTION_NAME [--prompt]"
            exit 1
        fi
        python3 scripts/transform-function.py \
            c2rust-output/src/xmlparse.rs "$2" \
            --c-source expat/lib/xmlparse.c \
            --existing-port expat-rust/src/xmlparse.rs \
            ${3:+$3}
        ;;

    analyze)
        echo "=== Analyzing C2Rust output patterns ==="
        for f in c2rust-output/src/xmlparse.rs c2rust-output/src/xmlrole.rs c2rust-output/src/xmltok.rs; do
            if [ -f "$f" ]; then
                python3 scripts/c2rust-analyze-patterns.py "$f"
                echo
            fi
        done
        ;;

    functions)
        echo "=== Function comparison ==="
        python3 scripts/extract-c2rust-functions.py \
            c2rust-output/src/xmlparse.rs \
            --compare expat-rust/src/xmlparse.rs
        ;;

    all)
        "$0" regenerate
        "$0" cleanup
        "$0" analyze
        "$0" functions
        "$0" compare
        ;;

    help|*)
        echo "C2Rust Pipeline for libexpat Rust port"
        echo ""
        echo "Commands:"
        echo "  regenerate  - Re-run c2rust transpilation"
        echo "  cleanup     - Run mechanical cleanup on c2rust output"
        echo "  compare     - Run comparison tests (Rust port vs C library)"
        echo "  extract FN  - Extract function FN for manual transformation"
        echo "  analyze     - Analyze patterns in c2rust output"
        echo "  functions   - Compare function lists between c2rust and existing port"
        echo "  all         - Run full pipeline"
        ;;
esac
