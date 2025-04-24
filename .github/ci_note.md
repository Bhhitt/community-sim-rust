# Continuous Integration (CI) Note

This project includes a smoke test for the benchmarking script (`tests/benchmark_smoke_test.rs`).

- The test runs the benchmarking shell script and checks that a results CSV is generated.
- To avoid race conditions, CI should run tests serially:
  ```sh
  cargo test -- --test-threads=1
  ```
- Ensure the test environment allows shell script execution and file writing in `benchmark/results/`.

For best results, set up your CI to:
- Install Rust and dependencies (including SDL2 if needed)
- Grant execute permissions to `benchmark/run_benchmarks.sh`
- Run `cargo test -- --test-threads=1` as the test step
