#!/usr/bin/env bash
# Run the W3C XML Test Suite against expat-rust's xmlwf binary.
#
# Usage:
#   ./xmlwf/run-w3c-tests.sh
#
# The W3C test suite (~1.5MB) is downloaded automatically on first run.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

XMLWF="$REPO_DIR/target/debug/xmlwf"
if [ ! -x "$XMLWF" ]; then
    echo "Building xmlwf..."
    (cd "$REPO_DIR" && cargo build -p xmlwf)
fi

XMLCONF_DIR="$REPO_DIR/tests/XML-Test-Suite/xmlconf"
if [ ! -d "$XMLCONF_DIR" ]; then
    echo "Downloading W3C XML Test Suite..."
    curl -sL -o /tmp/xmlts20020606.zip "https://www.w3.org/XML/Test/xmlts20020606.zip"
    mkdir -p "$REPO_DIR/tests"
    unzip -q -o /tmp/xmlts20020606.zip -d "$REPO_DIR/tests/"
fi

# xmltest.sh expects $PWD/tests/xmlconf/
SYMLINK="$REPO_DIR/tests/xmlconf"
if [ ! -L "$SYMLINK" ]; then
    rmdir "$SYMLINK" 2>/dev/null || true
    ln -sf "$XMLCONF_DIR" "$SYMLINK"
fi

rm -rf "$REPO_DIR/tests/out"
mkdir -p "$REPO_DIR/tests/out"

echo "Running W3C XML Test Suite..."
echo "  xmlwf:  $XMLWF"
echo ""

cd "$REPO_DIR"
bash "$REPO_DIR/expat/expat/tests/xmltest.sh" "$XMLWF" 2>&1
