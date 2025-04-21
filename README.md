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

## License
MIT
