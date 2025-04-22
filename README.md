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

## Installation & Setup

### Prerequisites
- Rust (install from https://rustup.rs)
- SDL2 (required for GUI)

### Install SDL2
#### macOS (Homebrew)
```sh
brew install sdl2
```
#### Ubuntu/Debian (Apt)
```sh
sudo apt-get update
sudo apt-get install libsdl2-dev
```

### Build the Project
```sh
cargo build --release
```

## Usage

### Run a Simulation (Headless)
```sh
cargo run -- --headless --map-size 40 --agents 20 --ticks 20
```

### Run with Graphics (GUI)
```sh
cargo run --release -- --profile=med_run
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


## Controls (GUI Mode)

- **Arrow Keys:** Pan camera
- **A:** Add a single agent at a random location
- **S:** Add 100 random agents
- **Spacebar:** Pause/resume simulation
- **Period (.):** Advance one tick (when paused)
- **Esc:** Quit simulation

## Selecting Entities

- **Left Click:** Select agent/entity under cursor

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

What i run most of the time
```sh
cargo run --release -- --profile=med_run
```

## Additional CLI Options

- `--headless` : Run simulation without graphics (for benchmarking)
- `--scale` : scaling benchmark (takes like 2 mins on my machine)
- `--log-level <level>` : Set logging level (`error`, `warn`, `info`, `debug`, `trace`)

For a full list of options, run:
```sh
cargo run -- --help
```

## License
MIT
