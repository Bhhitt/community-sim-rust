# ECS Systems Audit

## Purpose
Categorize all ECS systems currently registered in the simulation schedule, as a foundation for modularization.

---

## System List & Categorization (from `build_simulation_schedule_profiled` in `ecs_simulation.rs`)

| System | ccallAudit Status |
|--------|-------------------|
| collect_food_positions_system() | Done |
| collect_food_spawn_positions_system() | Done |
| food_spawn_apply_system() | Done |
| crate::agent::pause_system::agent_pause_system() | REMOVED |
| crate::ecs::systems::agent::agent_pausing_system() | Done |
| crate::ecs::systems::agent::agent_hunger_energy_system() | Done |
| crate::ecs::systems::agent::agent_path_movement_system() | Done |
| crate::ecs::systems::agent::agent_direct_movement_system() | Done |
| crate::ecs::systems::agent::agent_state_transition_system() | Done |

- `crate::agent::pause_system::agent_pause_system()` was removed as of 2025-04-26. All pausing logic is now handled by `agent_pausing_system` in `src/ecs/systems/agent.rs`.

### Food Systems
- `collect_food_positions_system()`
- `collect_food_spawn_positions_system()`
- `food_spawn_apply_system()`

### Agent Control & State Systems
- `crate::ecs::systems::agent::agent_pausing_system()`
- `crate::ecs::systems::agent::agent_hunger_energy_system()`
- `crate::ecs::systems::agent::agent_path_movement_system()`
- `crate::ecs::systems::agent::agent_direct_movement_system()`
- `crate::ecs::systems::agent::agent_state_transition_system()`

### Agent Spawning & Queue Systems
- `crate::ecs::systems::drain_agent_spawn_queue::drain_agent_spawn_queue_system()`
- `crate::ecs::systems::agent_spawn::agent_spawning_system()`

### Agent Logging Systems
- `crate::ecs::systems::agent_logging::agent_arrival_logging_system()`
- `crate::ecs::systems::agent_logging::agent_move_logging_system()`
- `crate::ecs::systems::agent_logging::agent_spawn_logging_system()`

### Entity & Agent Interaction Systems
- `entity_interaction_system()`
- `agent_death_system()`
- `agent::agent_event_log_to_gui_system()`

### (Commented/Legacy)
- `swimming_system()` (commented out)
- `passive_hunger_system` (unresolved/legacy)

---

## System Audit Table

| System Name                   | File                       | Reads                | Writes           | Description                                                                 | ECS Safety Notes               |
|-------------------------------|----------------------------|----------------------|------------------|-----------------------------------------------------------------------------|-------------------------------|
| collect_food_positions_system | src/food/systems.rs        | Position, Food       | FoodPositions    | Gathers all food entity positions and stores them in FoodPositions resource. | No conflicts; safe if FoodPositions is only read after. |
| collect_food_spawn_positions_system | src/food/systems.rs | Map      | PendingFoodSpawns  | Fills PendingFoodSpawns with new spawn positions based on map terrain.      | Safe: Only writes PendingFoodSpawns, flush after system prevents conflicts. |
| food_spawn_apply_system            | src/food/systems.rs | PendingFoodSpawns | PendingFoodSpawns, FoodStats | Drains PendingFoodSpawns and spawns food entities, updating FoodStats. | Safe: Exclusive access, ECS flushes prevent conflicts. |

---

## Notes
- Some systems are legacy or commented out and can be ignored for modularization unless reactivated.
- System names suggest clear domain groupings: food, agent control, agent spawning, agent logging, interactions.
- The next step is to propose modular schedule builder groupings based on these categories.

---

## Next Steps
1. Review these categories and system assignments.
2. Propose initial modular builder functions/files for each domain.
3. Begin extracting and implementing modular schedule builders.
