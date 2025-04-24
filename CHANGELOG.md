# 🚀 Community Simulator Release v0.0.1

This release brings the first release of this project to light. No telling if there will be more. This is a pet project to learn rust, build with ai, and push hardware around. This tool simulates agents interacting in an open ended setting. see what emerges

## ✨ Features

- **Configurable Simulation Settings**
  - Define simulation profiles in `config/sim_profiles.yaml` for flexible map size, agent count, tick count, and more.
  - Easily scale up to thousands of agents for performance and scaling studies.
  - Switch between profiles instantly via CLI:
    ```
    cargo run --release -- --profile=<profile_name>
    ```

- **Selectable Entities with Information**
  - In GUI mode, left-click to select any agent or entity on the map.
  - View detailed information about the selected entity for analysis and debugging.

- **YAML-Driven Benchmarking**
  - Mark any profile in `sim_profiles.yaml` with `benchmark: true`.
  - Run all benchmark profiles in headless mode with:
    ```
    ./benchmark/run_benchmarks.sh --benchmark-profiles
    ```
  - Results are saved to `benchmark/results/benchmark_profiles.csv`.

- **Scaling Benchmarks**
  - Built-in scaling profiles for quick performance comparisons.
  - Run with:
    ```
    ./benchmark/run_benchmarks.sh --scale
    ```

- **Parallel Simulation**
  - Leveraging Rayon for efficient updates, supporting large-scale simulations.

- **ASCII and GUI Rendering**
  - Visualize your simulation in the terminal or with a graphical interface (SDL2).

- **Extensive CLI Options**
  - Fine-tune simulation parameters, logging, and profiling from the command line.

- **Testing and Quality**
  - Comprehensive unit and integration tests for core logic and systems.

---

## 🛠️ Installation & Usage

See the [README.md](./README.md) for setup, usage, and benchmarking instructions.
