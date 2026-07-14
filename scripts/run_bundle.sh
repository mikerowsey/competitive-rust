#!/usr/bin/env bash

set -euo pipefail

# Change to project root
cd "$(dirname "$0")/.."

for file in src/bin/*.rs; do
    problem="${file##*/}"
    problem="${problem%.rs}"

    echo "Bundling ${problem}..."
    python3 scripts/bundle.py "${problem}"
done

echo
echo "All bundles generated successfully."
