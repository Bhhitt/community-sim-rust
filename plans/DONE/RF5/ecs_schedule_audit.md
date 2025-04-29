# ECS Schedule System Audit (2025-04-28)

This audit lists every system group and system referenced in the ECS schedule, and notes whether each is currently active ("ON") or commented out/disabled ("OFF").

---

## System Groups and Systems

### 1. Food Systems (food.rs)
- `collect_food_positions_system` — ON (reactivated)
- `food_collection_system` — ON (reactivated)
- `collect_food_spawn_positions_system` — ON (reactivated)
- `food_spawn_apply_system` — ON (reactivated)

### 2. Agent Core Systems (agent.rs)
- `agent_pausing_system` — ON
- `agent_hunger_energy_system` — ON
- `agent_path_movement_system` — ON
- `agent_direct_movement_system` — ON
- `agent_spawning_system` — ON
- `agent_state_transition_system` — ON

### 3. Spawning Systems (spawning.rs)
- `agent_spawning_system` — ON
- `agent_spawn_log_system` — ON (reactivated 2025-04-28)
- `food_spawning_system` — ON
- `agent_spawn_logging_system` — ON
- `agent_arrival_logging_system` — ON
- `agent_movement_history_system` — ON

### 4. Logging Systems (logging.rs)
- `agent_arrival_logging_system` — ON
- `agent_move_logging_system` — ON
- `agent_spawn_logging_system` — ON
- `interaction_event_logging_system` — ON

### 5. Interaction Systems (interaction.rs)
- `agent_agent_interaction_system` — ON
- `interaction_stats_update_system` — ON

### 6. Death Systems (death.rs)
- `agent_death_system` — ON

### 7. Event Log Bridge System (event_log_bridge.rs)
- `agent_event_log_to_gui_system` — ON

---

## Notes
- All food systems are now ON (reactivated).
- All duplicated agent systems have been removed from the spawning group; they now exist only in agent core.
- The agent spawn log system is now ON (reactivated 2025-04-28).
- System group activation in the main schedule (`build_main_schedule`) may differ from their status in the group modules. This audit reflects the group module status as of 2025-04-28.

---

**Next Steps:**
- Continue with label/dependency refactor.
- Use this audit as a checklist during the schedule refactor.
