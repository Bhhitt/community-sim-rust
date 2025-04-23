# High-Level Project Structure (Text)

Top-Level Modules:
- agent: Agent components, systems, and neural net logic (MLP)
- map: Map structure and logic
- interaction: Handles agent interactions
- simulation: Core simulation logic (legacy and ECS)
- graphics: Rendering, input, overlays, and simulation loop
- util: Utility functions
- ecs_hello, ecs_components, ecs_sim, ecs_simulation: ECS-specific simulation modules and components
- terrain: Terrain generation and types
- navigation: Pathfinding and navigation logic
- food: Food components and systems
- sim_summary: Simulation summary output
- event_log: Event logging
- log_config: Logging configuration
- render_ascii: ASCII rendering

Key Relationships:
- main.rs: Entry point, sets up and runs the simulation via modules in lib.rs
- lib.rs: Central module, re-exports and organizes all core modules
- agent: Defines agent data, behavior (systems), and neural net logic (MLP)
- food: Defines food components and systems for spawning/collecting food
- graphics: Handles all rendering, input, overlays, and the main simulation loop (including ECS rendering)
- navigation: Pathfinding and agent movement logic
- terrain: Map and terrain generation/types
- simulation/ecs_simulation: Core ECS simulation logic and systems
- event_log/sim_summary: Logging and simulation summary output

ECS Structure:
- Components are defined in agent/components.rs, food/components.rs, navigation/components.rs, etc.
- Systems are in agent/systems.rs, food/systems.rs, graphics/input_systems.rs, etc.
- The ECS world is set up and managed in ecs_simulation.rs and related modules.

---

# Mermaid Diagram

```mermaid
graph TD
    A[main.rs] -->|calls| B[lib.rs]
    B --> C[agent]
    B --> D[map]
    B --> E[interaction]
    B --> F[simulation]
    B --> G[graphics]
    B --> H[util]
    B --> I[ecs_hello]
    B --> J[ecs_components]
    B --> K[ecs_sim]
    B --> L[ecs_simulation]
    B --> M[terrain]
    B --> N[navigation]
    B --> O[food]
    B --> P[sim_summary]
    B --> Q[event_log]
    B --> R[log_config]
    B --> S[render_ascii]

    %% Submodules
    C --> C1[components]
    C --> C2[mlp]
    C --> C3[systems]
    O --> O1[components]
    O --> O2[systems]
    N --> N1[components]
    N --> N2[pathfinding]
    N --> N3[random_target]
    G --> G1[render]
    G --> G2[input]
    G --> G3[overlays]
    G --> G4[sim_loop]
    G --> G5[sim_render]
    G --> G6[stats]

    %% ECS relationships
    L -->|manages| J
    L -->|runs| C3
    L -->|runs| O2
    L -->|runs| N2
    L -->|runs| G4

    %% Data flow
    C3 -->|Agent actions| L
    O2 -->|Food logic| L
    N2 -->|Pathfinding| L
    G4 -->|Render loop| L

    %% Logging and summary
    L --> Q
    L --> P
```
