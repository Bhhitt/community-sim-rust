# Benchmarking Community Simulator

This directory contains scripts and results for benchmarking the simulation.

## Usage

Run the benchmarking script from the project root or from within this directory:

```sh
./benchmark/run_benchmarks.sh           # Runs all profiles in headless mode
./benchmark/run_benchmarks.sh med_run   # Runs the 'med_run' profile only
```

Benchmark results (CSV files) will be saved in `benchmark/results/`.

You can pass extra arguments to the script and they will be forwarded to the simulation (e.g., log level, agent types):

```sh
./benchmark/run_benchmarks.sh med_run --log-level debug
```

## Requirements
- Bash shell
- Rust toolchain (cargo)

## Notes
- The script ensures all results are saved in a unified location for easy comparison.
- Edit the script if you want to customize the benchmarking workflow further.
