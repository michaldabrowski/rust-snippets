#!/bin/bash

# Run all Advent of Code 2025 solutions

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

for day_dir in "$SCRIPT_DIR"/day*/; do
    if [ -d "$day_dir" ]; then
        echo "Running $(basename "$day_dir")..."
        (cd "$day_dir" && cargo run --quiet)
        echo ""
    fi
done