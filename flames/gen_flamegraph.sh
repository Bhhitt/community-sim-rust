#!/bin/bash
# Usage: ./gen_flamegraph.sh <profile_name> [width] [height] [extra_cargo_args]
# Example: ./gen_flamegraph.sh med_run_quiet 2400 48 --release

set -e

PROFILE_NAME=${1:-med_run_quiet}
WIDTH=${2:-2400}
HEIGHT=${3:-48}
EXTRA_ARGS="${@:4}"

if [ -z "$PROFILE_NAME" ]; then
  echo "Usage: $0 <profile_name> [width] [height] [extra_cargo_args]"
  exit 1
fi

DATESTR=$(date +"%Y%m%d_%H%M%S")
OUTFILE="flames/${PROFILE_NAME}_${DATESTR}.svg"

mkdir -p flames

# Run cargo flamegraph with environment variables for width/height
FLAMEGRAPH_WIDTH="$WIDTH" FLAMEGRAPH_HEIGHT="$HEIGHT" \
cargo flamegraph --root $EXTRA_ARGS --output "$OUTFILE" -- --profile "$PROFILE_NAME"

if [ $? -eq 0 ]; then
  echo "Flamegraph saved to $OUTFILE"
else
  echo "Flamegraph generation failed"
  exit 1
fi
