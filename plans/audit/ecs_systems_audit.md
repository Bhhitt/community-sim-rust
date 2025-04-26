# ECS Systems Audit

## Purpose
Categorize all ECS systems currently registered in the simulation schedule, as a foundation for modularization.

---

## System List & Categorization (from `build_simulation_schedule_profiled` in `ecs_simulation.rs`)

### Food Systems
- `collect_food_positions_system()`
- `collect_food_spawn_positions_system()`
- `food_spawn_apply_system()`

### Agent Control & State Systems
- `crate::agent::pause_system::agent_pause_system()`
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

## Notes
- Some systems are legacy or commented out and can be ignored for modularization unless reactivated.
- System names suggest clear domain groupings: food, agent control, agent spawning, agent logging, interactions.
- The next step is to propose modular schedule builder groupings based on these categories.

---

## Next Steps
1. Review these categories and system assignments.
2. Propose initial modular builder functions/files for each domain.
3. Begin extracting and implementing modular schedule builders.
