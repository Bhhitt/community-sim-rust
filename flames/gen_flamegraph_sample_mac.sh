#!/bin/bash
# macOS Flamegraph Script using 'sample' and 'inferno' (filters out Rayon/Tokio frames)
# Usage: ./gen_flamegraph_sample_mac.sh <profile_name> [extra_cargo_args]
# Example: ./gen_flamegraph_sample_mac.sh med_run_quiet --release

set -e

PROFILE_NAME=${1:-med_run_quiet}
EXTRA_ARGS="${@:2}"
DATESTR=$(date +"%Y%m%d_%H%M%S")
OUTDIR="flames"
SVG_OUT="$OUTDIR/${PROFILE_NAME}_${DATESTR}_inferno.svg"
SAMPLE_OUT="$OUTDIR/${PROFILE_NAME}_${DATESTR}_sample.txt"
FOLDED_OUT="$OUTDIR/${PROFILE_NAME}_${DATESTR}.folded"

mkdir -p "$OUTDIR"

# 1. Build the binary (release mode for realistic perf)
cargo build --release

# 2. Run the binary in the background
./target/release/community_sim --profile "$PROFILE_NAME" $EXTRA_ARGS &
PID=$!

# 3. Sample the process for 10 seconds (adjust duration as needed)
echo "Sampling PID $PID for 10 seconds..."
sample $PID 10 -file "$SAMPLE_OUT"

# 4. Kill the process after sampling (if still running)
kill $PID || true

# 5. Collapse the sample output
test -s "$SAMPLE_OUT" || { echo "Sample output is empty!"; exit 1; }
inferno-collapse-sample "$SAMPLE_OUT" > "$FOLDED_OUT"

# 6. Generate filtered SVG flamegraph
inferno-flamegraph --filter 'rayon|tokio|tokio_runtime|rayon_core' "$FOLDED_OUT" > "$SVG_OUT"

if [ $? -eq 0 ]; then
    echo "Filtered Inferno flamegraph saved to $SVG_OUT"
else
    echo "Inferno flamegraph generation failed"
    exit 1
fi
