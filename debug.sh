#!/bin/bash
set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <target_directory> <command> [args...]"
    exit 1
fi

TARGET_DIR="$1"
shift

if [ ! -d "$TARGET_DIR" ]; then
    echo "Error: Directory '$TARGET_DIR' does not exist."
    exit 1
fi

# --- run
cargo build
cp target/i686-pc-windows-gnu/debug/taxipatch.dll "$TARGET_DIR/taxipatch.asi"

if [ $# -gt 0 ]; then
    exec "$@"
fi
