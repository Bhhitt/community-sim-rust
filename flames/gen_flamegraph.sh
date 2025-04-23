#!/bin/bash
# Usage: ./gen_flamegraph.sh <profile_name> [extra_cargo_args]
# Example: ./gen_flamegraph.sh med_run_quiet --release

set -e

PROFILE_NAME=${1:-med_run_quiet}
EXTRA_ARGS="${@:2}"
DATESTR=$(date +"%Y%m%d_%H%M%S")
OUTDIR="flames"
SVG_OUT="$OUTDIR/${PROFILE_NAME}_${DATESTR}_inferno.svg"
FOLDED_OUT="$OUTDIR/${PROFILE_NAME}_${DATESTR}.folded"

mkdir -p "$OUTDIR"

# Run the simulation under perf, collapse stacks, and generate SVG with inferno
perf record -F 99 -g -- cargo run --release -- --profile "$PROFILE_NAME" $EXTRA_ARGS
perf script | inferno-collapse-perf > "$FOLDED_OUT"
inferno-flamegraph "$FOLDED_OUT" > "$SVG_OUT"

if [ $? -eq 0 ]; then
    echo "Inferno flamegraph saved to $SVG_OUT"
else
    echo "Inferno flamegraph generation failed"
    exit 1
fi
