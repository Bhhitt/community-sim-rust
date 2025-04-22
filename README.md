# Community Simulator (Rust)

A scalable community simulation written in Rust, featuring agents that interact on a procedurally generated terrain map. Designed for performance, extensibility, and experimentation with agent-based models.

## Features
- **Terrain Types:** Grass, Water, Forest, Mountain (randomly generated)
- **Agents:** Move across the map, interact with each other, and respect terrain passability
- **ASCII Rendering:** Visualize the map and agent positions in the terminal or as text files
- **Parallel Simulation:** Uses Rayon for efficient agent updates and interactions
- **Configurable:** Map size, agent count, and ticks are all settable via CLI
- **Performance Metrics:** Reports timings for movement, interaction, and total simulation
- **Testing:** Includes unit tests for agents and interactions

## Usage

### Build
```sh
cargo build
```

### Run a Simulation
```sh
cargo run -- --headless --map-size 40 --agents 20 --ticks 20
```

### Run Scaling Benchmarks
```sh
cargo run -- --headless --scale
```

### Run Tests
```sh
cargo test
```

## Project Structure
- `src/agent.rs` — Agent logic
- `src/map.rs` — Map and terrain generation
- `src/interaction.rs` — Agent interactions
- `src/simulation.rs` — Simulation loop and scaling
- `src/graphics.rs` — (Stub) for future graphical rendering
- `tests/` — Unit and integration tests

## Roadmap
- Add more terrain effects (altitude, slopes)
- Visual rendering (SDL2 or similar)
- More complex agent behaviors

## Running Tests

To run all unit and integration tests:
```sh
cargo test
```
Tests are located in the `tests/` directory and within modules using Rust's `#[cfg(test)]` attribute. These cover core agent logic, ECS systems, and simulation behaviors.

## Running with Different Profiles

Simulation profiles allow you to quickly launch scenarios with different map sizes, agent counts, and tick counts. Profiles are defined in `sim_profiles.yaml`.

To run with a specific profile (with graphics):
```sh
cargo run --release -- --profile=<profile_name>
```
Replace `<profile_name>` with the name of your desired profile (e.g., `small`, `med_run`, `large`).

To run all profiles in headless mode:
```sh
cargo run --release -- --headless --profile-system
```

You can also specify map size, agent count, and ticks directly:
```sh
cargo run --release -- --map-size 40 --agents 20 --ticks 20
```

## Controls (GUI Mode)

- **Arrow Keys / WASD:** Pan camera
- **Mouse Wheel / +/-:** Zoom in/out
- **Left Click:** Select agent/entity under cursor
- **Right Click:** Deselect or issue move command (if supported)
- **Spacebar:** Pause/resume simulation
- **Esc:** Quit simulation

## Selecting Entities

Click on an agent or entity to select it. When selected, its properties and stats will be displayed in the sidebar or info panel. You can select multiple entities by holding Shift (if supported).

## Additional CLI Options

- `--headless` : Run simulation without graphics (for benchmarking)
- `--scale` : Run scaling benchmarks
- `--profile-system` : Enable ECS system profiling (outputs CSV)
- `--profile-csv <file>` : Set CSV output file for profiling
- `--log-level <level>` : Set logging level (`error`, `warn`, `info`, `debug`, `trace`)

For a full list of options, run:
```sh
cargo run -- --help
```

## License
MIT
