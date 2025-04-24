#!/bin/bash
# Unified benchmarking script for Community Sim
# Usage: ./run_benchmarks.sh [profile_name|--scale|--benchmark-profiles] [extra_args]
#
# If no profile_name is given, runs all profiles in headless mode (default benchmarking)
# If --scale is given as the first argument, runs the scaling benchmark.
# If --benchmark-profiles is given as the first argument, runs YAML-driven benchmarks (benchmark: true).
# Otherwise, runs the specified profile in headless mode.

set -e

cd "$(dirname "$0")/.."

BENCHMARK_DIR="benchmark/results"
mkdir -p "$BENCHMARK_DIR"

ARG1=${1:-}
EXTRA_ARGS="${@:2}"

if [ "$ARG1" = "--scale" ]; then
    echo "[Benchmark] Running SCALING benchmark in headless mode..."
    cargo run --release -- --headless --scale --profile-systems --profile-csv "$BENCHMARK_DIR/scale.csv" $EXTRA_ARGS
elif [ "$ARG1" = "--benchmark-profiles" ]; then
    echo "[Benchmark] Running YAML benchmark profiles (benchmark: true) in headless mode..."
    cargo run --release -- --headless --benchmark-profiles --profile-systems --profile-csv "$BENCHMARK_DIR/benchmark_profiles.csv" $EXTRA_ARGS
elif [ -z "$ARG1" ]; then
    echo "[Benchmark] Running ALL profiles in headless mode..."
    cargo run --release -- --headless --profile-systems --profile-csv "$BENCHMARK_DIR/all_profiles.csv" $EXTRA_ARGS
else
    echo "[Benchmark] Running profile: $ARG1 in headless mode..."
    cargo run --release -- --headless --profile "$ARG1" --profile-systems --profile-csv "$BENCHMARK_DIR/${ARG1}.csv" $EXTRA_ARGS
fi

echo "[Benchmark] Done. Results in $BENCHMARK_DIR/"
