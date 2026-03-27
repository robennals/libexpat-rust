#!/usr/bin/env bash
# Download OSS-Fuzz public corpora for expat fuzzers.
# These are raw-input corpora (not protobuf) so we skip xml_lpm_fuzzer.
set -euo pipefail

CORPUS_DIR="${1:-corpus}"

FUZZERS=(
    xml_parse_fuzzer_UTF-8
    xml_parsebuffer_fuzzer_UTF-16LE
)

mkdir -p "${CORPUS_DIR}"

for fuzzer in "${FUZZERS[@]}"; do
    dest="${CORPUS_DIR}/${fuzzer}"
    if [ -d "${dest}" ] && [ "$(find "${dest}" -type f | head -1)" ]; then
        echo "Corpus already exists: ${dest} ($(find "${dest}" -type f | wc -l | tr -d ' ') files)"
        continue
    fi
    echo "Downloading corpus for ${fuzzer}..."
    url="https://storage.googleapis.com/expat-backup.clusterfuzz-external.appspot.com/corpus/libFuzzer/expat_${fuzzer}/public.zip"
    tmp=$(mktemp)
    if curl -fsSL -o "${tmp}" "${url}"; then
        mkdir -p "${dest}"
        unzip -q -o -d "${dest}" "${tmp}"
        rm -f "${tmp}"
        echo "  Extracted $(find "${dest}" -type f | wc -l | tr -d ' ') files to ${dest}"
    else
        echo "  WARNING: Failed to download corpus for ${fuzzer} (URL may be unavailable)"
        rm -f "${tmp}"
    fi
done
