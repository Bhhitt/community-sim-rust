# Agent Logging Systems Integration Plan

This plan outlines the step-by-step process for integrating the new dedicated agent logging systems into the ECS schedule and cleaning up legacy logging from other systems.

---

| Step | Task Description                                                                 | Checklist |
|------|----------------------------------------------------------------------------------|-----------|
| 1    | Register `agent_arrival_logging_system` in ECS schedule                          | [ ]       |
| 2    | Register `agent_move_logging_system` in ECS schedule                             | [ ]       |
| 3    | Register `agent_spawn_logging_system` in ECS schedule (if used)                  | [ ]       |
| 4    | Remove arrival logging from `agent_arrival_system`                               | [ ]       |
| 5    | Remove move logging from any movement/path systems                               | [ ]       |
| 6    | Remove spawn logging from `spawn_agent` or related systems                       | [ ]       |
| 7    | Ensure all logging for agent events is handled only by dedicated logging systems | [ ]       |
| 8    | Test and verify that all log events appear in both `EventLog` and Rust logger    | [ ]       |
| 9    | Update documentation and audit files to reflect changes                          | [ ]       |

---

## Step-by-Step Instructions

1. **Register Logging Systems**
   - Open your ECS schedule setup (e.g., `build_simulation_schedule_profiled` in `src/ecs_simulation.rs`).
   - Add calls to `agent_arrival_logging_system`, `agent_move_logging_system`, and `agent_spawn_logging_system` as appropriate.

2. **Remove Legacy Logging**
   - In each agent system (arrival, movement, spawn, etc.), delete or comment out any direct logging calls related to agent events.
   - Ensure these systems only handle their core logic (no logging side effects).

3. **Centralize Logging**
   - Confirm that all agent event logging now occurs only in the new dedicated logging systems.

4. **Test Logging**
   - Run the simulation and verify that arrival, move, and spawn events are logged both to the `EventLog` resource and via the Rust logger (`log::debug!`).

5. **Update Documentation**
   - Update your audit, plan, and change log files to document the new logging architecture and system responsibilities.

---

**Review this plan and check off each step as you complete it. Adjust as needed for your project structure and workflow.**
