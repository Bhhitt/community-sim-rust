# Agent Systems Query Split Refactor Plan (2025)

## Context
- Recent refactoring moved and modularized agent ECS systems, but some queries may still be too large or complex.
- Legion limits tuple queries to 8 components, and smaller queries/systems align better with ECS best practices.
- Some splitting may already be done in code, but this plan is to ensure all systems are as focused as possible and to document the intended architecture.

## Goals
- **Reduce query size**: Ensure no system exceeds the Legion tuple query limit.
- **Improve separation of concerns**: Each system should do one thing (movement, hunger, pausing, etc.).
- **Increase maintainability**: Smaller, focused systems are easier to test, debug, and extend.
- **Document current and target state**: Track which systems have been split and which need further work.

## Current (Post-Refactor) Systems
- `spawn_agent` (spawning/init)
- `agent_pause_system` (IdlePause decrement/unpause)
- `agent_action_selection_system` (action choice, target/path assignment)
- `path_following_system` (waypoint/path management)
- `agent_movement_system` (position updates, state transitions)
- `agent_arrival_system` (detect arrival, set IdlePause)
- `passive_hunger_system` (hunger updates)
- `agent_movement_history_system` (record positions)
- `agent_death_system` (removal/cleanup)
- `swimming_system` (swimming logic)
- `agent_event_log_to_gui_system` (GUI event log)

## Refactor Plan

### 1. Audit All System Queries
- Review each system's `.with_query` to document current query size and components.
- Identify any queries at/near the 8-tuple limit.
- Mark systems that are already minimal and those that need splitting.

### 2. Split Overly Large or Mixed-Concern Systems
- For any system with a large tuple or multiple responsibilities, split into:
    - **Movement**: Only position/path updates.
    - **State**: Only state transitions.
    - **Pause**: Only IdlePause logic.
    - **Hunger**: Only hunger/energy updates.
    - **History**: Only movement history.
    - **Arrival**: Only arrival detection/processing.
    - **Event Logging**: Only event log bridging.
- Use helper functions for shared logic (in `logic.rs` or similar).

### 3. Update System Registration
- Ensure new/split systems are registered in the ECS schedule in the correct order.
- Remove any legacy or now-redundant systems.

### 4. Test and Validate
- After each split, test for correct agent behavior (no logic loss/regression).
- Add/expand unit tests for new, smaller systems.
- Use debug logging to verify system boundaries and transitions.

### 5. Document Architecture
- Update this plan and `agent_systems_refactor_plan.md` with the new system list and responsibilities.
- Add comments in code about why/when systems are split (e.g., Legion query limit).

## Checklist Table (fill as you go)
| System Name | Query Size | Single Responsibility? | Split Needed? | Notes |
|-------------|------------|-----------------------|--------------|-------|
| spawn_agent |            | Yes                   | No           |       |
| agent_pause_system |      | Yes                   | No           | Split due to query limit |
| agent_action_selection_system |    |                   |              |       |
| path_following_system |     |                      |              |       |
| agent_movement_system |     |                      |              |       |
| agent_arrival_system |      |                      |              |       |
| passive_hunger_system |     |                      |              |       |
| agent_movement_history_system | |                    |              |       |
| agent_death_system |        |                      |              |       |
| swimming_system |           |                      |              |       |
| agent_event_log_to_gui_system | |                   |              |       |

## Next Steps
- [ ] Audit all system queries in code
- [ ] Fill in checklist table
- [ ] Split any large/mixed systems
- [ ] Update docs and code comments
- [ ] Test and validate

---

*This plan should be updated as progress is made and as the codebase evolves. If systems are already split due to recent refactors, update the checklist and notes accordingly.*
