#!/bin/bash
# macOS Flamegraph Script for Rust (filters out Rayon/Tokio frames)
# Usage: sudo ./gen_flamegraph_mac.sh <profile_name> [extra_cargo_args]
# Example: sudo ./gen_flamegraph_mac.sh med_run_quiet --release

set -e

PROFILE_NAME=${1:-med_run_quiet}
EXTRA_ARGS="${@:2}"
DATESTR=$(date +"%Y%m%d_%H%M%S")
OUTDIR="flames"
SVG_OUT="$OUTDIR/${PROFILE_NAME}_${DATESTR}_inferno.svg"
FOLDED_OUT="$OUTDIR/${PROFILE_NAME}_${DATESTR}.folded"

mkdir -p "$OUTDIR"

# 1. Run cargo flamegraph to collect and generate the initial flamegraph and stacks
sudo cargo flamegraph --root --output "$SVG_OUT" --flamechart -- --profile "$PROFILE_NAME" $EXTRA_ARGS

# 2. Use cargo-flamegraph.stacks as the folded input for filtering
FOLDED_SRC="cargo-flamegraph.stacks"
if [ ! -f "$FOLDED_SRC" ]; then
    echo "Could not find folded stack file (cargo-flamegraph.stacks)."
    exit 1
fi

# 3. Filter out Rayon and Tokio frames and regenerate SVG
cat "$FOLDED_SRC" | inferno-flamegraph --filter 'rayon|tokio|tokio_runtime|rayon_core' > "$SVG_OUT"

if [ $? -eq 0 ]; then
    echo "Filtered Inferno flamegraph saved to $SVG_OUT"
else
    echo "Inferno flamegraph generation failed"
    exit 1
fi
