# Wild Changes Log

## 2025-04-25: ECS Agent System Refactor

### What changed?
- **Refactored agent movement logic:**
  - Moved the agent movement system to `src/ecs/systems/agent.rs`.
  - The new `agent_movement_system` now only updates agent positions (no state transitions, logging, or hunger logic).
- **New state transition system:**
  - Added `agent_state_transition_system` to `src/ecs/systems/agent.rs`.
  - This system sets `AgentState::Arrived` when an agent's position matches its target.
- **Schedule updated:**
  - Registered both new systems in the ECS schedule (see `build_simulation_schedule_profiled` in `src/ecs_simulation.rs`).
- **Removed legacy logic:**
  - The old, monolithic movement system was removed from `src/agent/systems.rs`.
- **Removed all remaining state transition logic from other systems:**
  - Now, only `agent_state_transition_system` sets or resets agent states. All other systems (arrival, path following, etc.) no longer perform state transitions, but only their core responsibilities (pausing, logging, etc.).
- **Centralized agent logging:**
  - Implemented dedicated logging systems for agent arrival, move, and spawn events in `src/ecs/systems/agent_logging.rs`.
  - Registered logging systems in the ECS schedule.
  - Removed all legacy logging from agent systems; logging is now modular and handled only by dedicated systems.

### Why?
- To align with ECS best practices: each system now has a single responsibility and smaller queries.
- To avoid Legion's tuple query/component borrow limits.
- To improve maintainability, testability, and clarity of agent systems.

### Next Steps
- Split out logging and pausing logic into their own systems if not already done.
- Continue auditing and refactoring other agent ECS systems as needed.

## 2025-04-25: Removal of legacy agent_arrival_system

- Removed the legacy `agent_arrival_system` from `src/agent/systems.rs`.
- All arrival logic (state transitions, logging, pausing) is now handled by dedicated systems:
    - `agent_state_transition_system` (state transitions)
    - `agent_arrival_logging_system` (event logging)
    - `agent_pause_system` (pausing)
- Registered `agent_state_transition_system` in the ECS schedule, immediately after agent movement.
- This further enforces single responsibility and modularity in the ECS architecture.

## 2025-04-25
- Extracted all pausing/IdlePause logic from agent_movement_system into a new dedicated agent_pausing_system in src/ecs/systems/agent.rs.
- agent_pausing_system now decrements ticks_remaining and handles all unpausing logic.
- Removed all pausing checks from agent_movement_system; movement now assumes pausing is handled elsewhere.
- Wired agent_pausing_system into the ECS schedule before agent_movement_system in src/ecs_simulation.rs.
- This completes the separation of pausing logic as per the path_following_system_refactor_plan.md.

- Extracted and consolidated all agent hunger and energy logic into a new agent_hunger_energy_system in src/ecs/systems/agent.rs.
- agent_hunger_energy_system now decrements both hunger and energy values per tick based on agent state, replacing the old passive_hunger_system.
- Removed passive_hunger_system from src/agent/systems.rs to avoid duplicate logic.
- Registered agent_hunger_energy_system in the ECS schedule after pausing and before movement/state transition systems.
- All hunger and energy updates are now handled by a single-responsibility system, in line with ECS best practices and refactor plan.

- Fully removed the obsolete path_following_system from src/agent/systems.rs after splitting all logic into dedicated systems (pausing, hunger/energy, movement, state transitions, logging).
- This completes the path_following_system refactor and audit as of 2025-04-25.

## 2025-04-25 — Agent Systems Refactor Complete
- Completed full modularization of all agent ECS systems:
    - Action selection, target assignment, path assignment, movement, pausing, hunger/energy, state transitions, and logging now handled by dedicated, single-responsibility systems.
    - Removed all legacy/monolithic systems and commented-out code (e.g., agent_action_selection_system).
    - Updated ECS schedule to use only new systems in correct order.
    - Confirmed all logic placement, file organization, and documentation matches ECS best practices.
- Next: Build and test the simulation, then proceed with MLP integration or further enhancements.
