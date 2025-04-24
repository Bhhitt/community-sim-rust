//! Smoke test for the benchmarking script (run_benchmarks.sh)
//! Ensures the script runs and produces some output CSV file.

use std::process::Command;
use std::fs;

#[test]
fn benchmark_script_smoke_test() {
    // Path to the benchmarking script
    let script_path = "./benchmark/run_benchmarks.sh";
    // Remove previous results if they exist
    let results_path = "benchmark/results/benchmark_profiles.csv";
    let _ = fs::remove_file(results_path);

    // Run the script with --benchmark-profiles (should be quick for a smoke test)
    let output = Command::new(script_path)
        .arg("--benchmark-profiles")
        .output()
        .expect("Failed to execute benchmark script");

    assert!(output.status.success(), "Benchmark script did not exit successfully. Stdout: {}\nStderr: {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));

    // Check that the results file was created
    assert!(fs::metadata(results_path).is_ok(), "Benchmark results file was not created");

    // Optionally, check that the file is non-empty
    let contents = fs::read_to_string(results_path).expect("Could not read benchmark results file");
    assert!(!contents.trim().is_empty(), "Benchmark results file is empty");
}
