# Steps to Implement Unified Simulation Loop (RF4)

This table outlines the concrete steps necessary to scaffold and implement the unified simulation loop architecture. Each step should be scaffolded (traits, structs, functions created) before detailed logic is added.

| Step | Task Description | File/Module | Notes |
|------|------------------|-------------|-------|
| 1 | Create `SimulationRenderer`, `SimulationProfiler`, `SimulationInput` traits | `src/sim_loop.rs` | Define interfaces for rendering, profiling, input |
| 2 | Scaffold `run_simulation_loop` function | `src/sim_loop.rs` | Accepts world, resources, schedule, ticks, renderer, profiler, input |
| 3 | Define `SimulationStats` struct | `src/sim_summary.rs` or `src/sim_loop.rs` | Minimal stats for return value |
| 4 | Implement `AsciiRenderer` (headless) | `src/render_ascii.rs` | Prints map every 50 ticks |
| 5 | Implement `NoOpRenderer`, `NoOpInput`, `NoOpProfiler` | `src/sim_loop.rs` or submodule | For headless/no-op scenarios |
| 6 | Implement GUI renderer and input handler | `src/graphics/renderer.rs`, `src/graphics/input.rs` | For graphics mode |
| 7 | Update entry points to use unified loop | `src/main.rs` | Wire up correct types per mode |
| 8 | Remove legacy/duplicated simulation loops | Various | Clean up after migration |
| 9 | Add/update integration tests for both modes | `tests/`, `src/sim_loop.rs` | Ensure identical behavior |

---

**Reference:** Scaffold each step before implementing detailed logic. Use this checklist to track progress and maintain architectural clarity.
