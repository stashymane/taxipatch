#!/bin/bash
set -euo pipefail

if [ -z ${TARGET_DIR+x} ]; then
    echo "TARGET_DIR variable must be set to the plugin directory"
    exit 1
fi

if [ $# -lt 1 ]; then
    echo "Usage: $0 <command> [args...]"
    exit 1
fi

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
