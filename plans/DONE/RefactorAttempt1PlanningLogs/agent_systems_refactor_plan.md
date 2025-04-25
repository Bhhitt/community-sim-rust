# Agent Systems Refactor Plan (2025 Update)

This plan reflects the current set of agent ECS systems in the simulation, with clear responsibilities and a modular organization for maintainability, extensibility, and debugging.

## 1. Core Agent ECS Systems

1. **spawn_agent**
   - Handles agent creation and initialization, setting up all required components and logging spawn events.
   - **Status:** Done (now handled by ECS agent_spawning_system and PendingAgentSpawns resource)

2. **agent_pause_system**
   - Decrements IdlePause for agents, unpausing them when the timer reaches zero. Ensures agents only act when not paused.
   - **Status:** Done (now agent_pausing_system)

3. **agent_action_selection_system**
   - Decides the next action for each agent (seek food, wander, etc.), assigns targets/paths, and sets state to Moving. Logs all decisions.
   - **Status:** Done (now agent_action_decision_system)

4. **path_following_system**
   - Advances agents along their assigned paths, manages waypoints, and resets state to Idle after arrival.
   - **Status:** Removed/split (all responsibilities now handled by dedicated systems)

5. **agent_movement_system**
   - Physically moves agents toward their targets or along waypoints, updates position, and transitions to Arrived state when appropriate. Skips movement if paused.
   - **Status:** Done (minimal, single-responsibility)

6. **agent_arrival_system**
   - Detects when agents reach their destination, sets IdlePause, and transitions them to Idle. Simulates realistic pauses between actions.
   - **Status:** Removed/split (responsibilities now handled by agent_state_transition_system, agent_pausing_system, and logging systems)

7. **passive_hunger_system**
   - Updates hunger based on agent activity (Idle, Moving, Arrived), with slower hunger loss when idle or paused.
   - **Status:** Removed/replaced (now agent_hunger_energy_system)

8. **agent_movement_history_system**
   - Records each agent’s recent positions for analytics, debugging, or visualization.
   - **Status:** Done (now in ecs/systems/agent.rs)

9. **agent_death_system**
   - Removes agents whose hunger or energy reaches zero, ensuring proper cleanup.
   - **Status:** Done

10. **swimming_system**
     - Handles swimming behavior for agents with a swimming profile, including movement through water and swim duration.
     - **Status:** Done (now in ecs/systems/swimming.rs)

11. **agent_event_log_to_gui_system**
     - Bridges agent event logs to the GUI event log for display, transferring recent agent events for visualization.
     - **Status:** Done (minimal, single-responsibility, appropriate location in agent/event_log_bridge.rs)

## 2. Refactor Goals and Guidelines

- **Separation of Concerns:** Each system should have a clear, single responsibility.
- **Helpers/Logic Modules:** Move complex logic (e.g., movement, action selection) to `logic.rs` or dedicated helpers.
- **Logging:** Add debug logs for key transitions and decisions in all systems.
- **Assertions:** Add assertions for invariants (e.g., agents must have a target if moving).
- **Testing:** After each refactor, run and test the simulation. Add unit tests for helpers where possible.
- **File Organization:**
    - `components.rs`: Component definitions only
    - `systems.rs`: System registration and short system definitions
    - `logic.rs`: Movement, decision, and pause helpers
    - `spawn.rs`: (Optional) Agent spawning logic

## 3. System Feature Checklists

Below is a table for each agent ECS system. Use these tables to systematically verify that each system implements all required and intended features. Fill in the "Present?" and "Notes" columns as you review or update each system.

| System Name | Feature/Responsibility | Present? | Notes |
|-------------|-----------------------|----------|-------|
| spawn_agent | Initializes agent components (state, hunger, etc.) |  |  |
| spawn_agent | Logs agent spawn events |  |  |
| spawn_agent | Sets up default values for all agent fields |  |  |
| agent_pause_system | Decrements IdlePause each tick |  |  |
| agent_pause_system | Unpauses agent when timer reaches zero |  |  |
| agent_pause_system | Ensures other systems skip paused agents |  |  |
| agent_pause_system | Logs pause/unpause events |  |  |
| agent_action_selection_system | Chooses next action (seek food, wander, etc.) |  |  |
| agent_action_selection_system | Assigns targets and/or paths |  |  |
| agent_action_selection_system | Logs action selection decisions |  |  |
| path_following_system | Advances agents along assigned paths |  |  |
| path_following_system | Handles waypoint progression |  |  |
| path_following_system | Resets agent state to Idle after arrival |  |  |
| agent_movement_system | Moves agents toward targets/waypoints |  |  |
| agent_movement_system | Updates agent position |  |  |
| agent_movement_system | Transitions to Arrived state when destination reached |  |  |
| agent_movement_system | Skips movement if paused |  |  |
| agent_arrival_system | Detects arrival at destination |  |  |
| agent_arrival_system | Sets IdlePause and transitions to Idle |  |  |
| agent_arrival_system | Logs arrival events |  |  |
| passive_hunger_system | Decrements hunger based on activity |  |  |
| passive_hunger_system | Triggers state changes on hunger thresholds |  |  |
| passive_hunger_system | Logs hunger changes |  |  |
| agent_movement_history_system | Records agent position history |  |  |
| agent_movement_history_system | Supports analytics/debugging/visualization |  |  |
| agent_death_system | Removes agents with zero hunger/energy |  |  |
| agent_death_system | Cleans up associated resources |  |  |
| swimming_system | Handles movement through water |  |  |
| swimming_system | Decrements swim duration/ticks |  |  |
| swimming_system | Applies swimming-specific state changes |  |  |
| agent_event_log_to_gui_system | Transfers agent events to GUI log |  |  |
| agent_event_log_to_gui_system | Ensures recent events are visible in GUI |  |  |

## ✅ Refactor Completion: Agent Systems (2025-04-25)

All core agent ECS systems have now been fully refactored and modularized. Each system has a clear, single responsibility, and legacy/monolithic systems have been removed or replaced. The structure and logic placement for the agent system is complete and matches ECS best practices.

- [x] All legacy systems removed or split (see individual statuses above)
- [x] Modular, single-responsibility systems in place for action, target, path, movement, pausing, hunger/energy, state transitions, logging, and more
- [x] ECS schedule updated to use only new systems
- [x] File organization and documentation updated

**Next:** Build and test the simulation, then proceed with MLP integration or further enhancements as needed.

## 4. Example File Structure

```
src/
  agent/
    components.rs
    systems.rs
    logic.rs
    spawn.rs
```

## 5. System Flow Diagram

See `someDocs/agent_ecs_systems_flowchart.mmd` for a visual overview of system execution order and data flow.

This plan ensures the agent systems remain modular, testable, and easy to extend as the simulation grows.
