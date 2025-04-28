# RF6: ECS Initialization Refactor Plan

## Objective
Refactor simulation initialization so that all ECS resource and component mutations (e.g., enqueuing agent/food spawns) happen inside ECS systems, not in imperative code. This will ensure idiomatic, robust, and maintainable ECS-driven initialization.

---

## 1. System Structure Overview

### New/Refactored Files
- `src/ecs/resources/init_config.rs` — ECS resource holding initialization data/config
- `src/ecs/systems/initial_spawn.rs` — Legion system(s) for initial agent/food spawn
- `src/ecs/schedules/init.rs` — Adds initialization systems to the schedule
- (Update) `src/sim_core.rs` — Only world/resource setup, no direct ECS resource mutation

### Directory Structure
```
/ecs
  /resources
    init_config.rs
  /systems
    initial_spawn.rs
  /schedules
    init.rs
sim_core.rs
```

---

## 2. Function/File Responsibilities & Signatures

### `src/ecs/resources/init_config.rs`
- **Purpose:** Defines `InitConfig` resource (holds agent/food spawn info, map, etc.)
- **Signature:**
  ```rust
  pub struct InitConfig {
      pub agent_types: Vec<AgentType>,
      pub num_agents: usize,
      pub food_spawns: Vec<(f32, f32)>,
      pub agent_spawns: Vec<(f32, f32, AgentType)>,
      // ...other fields as needed
  }
  ```
- **Responsibility:** Holds all data needed for initial ECS-driven spawning.

---

### `src/ecs/systems/initial_spawn.rs`
- **Purpose:** ECS system(s) that consume `InitConfig` and enqueue to `PendingAgentSpawns`, `PendingFoodSpawns`.
- **Signature:**
  ```rust
  pub fn initial_spawn_system() -> impl legion::systems::Runnable;
  ```
- **Responsibility:**
  - On first tick, reads `InitConfig` and pushes spawn requests into ECS resources.
  - Optionally, removes/flags `InitConfig` as 'done' after running.

---

### `src/ecs/schedules/init.rs`
- **Purpose:** Adds initialization systems to the ECS schedule.
- **Signature:**
  ```rust
  pub fn add_init_systems(builder: &mut legion::systems::Builder);
  ```
- **Responsibility:** Adds `initial_spawn_system` (and any future init systems) to the schedule, with appropriate flush.

---

### `src/sim_core.rs`
- **Purpose:** Only world/resource setup, no direct ECS resource mutation for spawns.
- **Signature:**
  - Inserts `InitConfig` as a resource.
  - Does NOT enqueue spawn requests directly.
- **Responsibility:**
  - Loads config/map/agent types from disk.
  - Inserts `InitConfig` and other baseline resources.
  - Leaves all ECS state changes to systems.

---

## 3. System Flow
1. `sim_core.rs` loads config/map/agent types, inserts `InitConfig` resource.
2. ECS schedule runs `initial_spawn_system` (via `add_init_systems`).
3. `initial_spawn_system` enqueues all agent/food spawns into ECS resources for the rest of the simulation.
4. `InitConfig` is dropped or flagged as complete.

---

## 4. Additional Notes
- This pattern can be extended to other initialization logic (e.g., items, money, etc.).
- All ECS state changes after resource insertion are managed by ECS systems.
- This approach is idiomatic, extensible, and minimizes borrow conflicts.

---

## 5. Next Steps
1. Implement `InitConfig` resource.
2. Implement `initial_spawn_system`.
3. Refactor `sim_core.rs` to use this pattern.
4. Update schedule construction to use `add_init_systems`.
5. Test and iterate.
