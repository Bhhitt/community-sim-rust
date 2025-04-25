# Agent Logging Audit

## 1. Logging in Agent Spawning (`src/agent/systems.rs`, `spawn_agent`)
- Uses `log::debug!` with tags like `[SPAWN_INFO]`, `[SPAWN]`, `[SPAWN_DEBUG]` to record spawning events, positions, and states.
- Pushes an `AgentEvent::Spawned` event to `agent_event_log`.

## 2. Logging in Agent Arrival System (`src/agent/systems.rs`, `agent_arrival_system`)
- When an agent arrives, logs with:
  - `resources.0.lock().unwrap().push(format!('[ARRIVAL] Agent {{:?}} arrived and is pausing for 30 ticks', entity));`
  - `log::debug!('[DEBUG] AgentArrivalSystem processed {{}} arrivals this tick', arrivals);`
- This is currently still in the same system as pausing logic.

## 3. Logging in Path Following System (`src/agent/systems.rs`, `path_following_system`)
- Logs debug info for agent resets:
  - `log::debug!('[PATH_FOLLOWING] Agent state reset to Idle after arrival');`
  - `log::debug!('[DEBUG] PathFollowingSystem reset {{}} agents (matched {{}} total) this tick', reset, agent_count);`
- After refactor, only logs checks, not state transitions.

## 4. Logging in Movement System (`src/ecs/systems/agent.rs`)
- No longer contains logging after refactor. All `[MOVE]` and `[ARRIVE]` log events have been removed.

## 5. General Logging Patterns
- Logging is performed via both `log::debug!` (Rust logging macros) and by pushing formatted strings to an `EventLog` resource (for UI or persistent logs).
- Log messages are tagged: `[SPAWN]`, `[ARRIVAL]`, `[PATH_FOLLOWING]`, `[MOVE]`, etc.

---

## Audit Findings
- **Logging is still mixed with other logic** in some systems (e.g., arrival and path following systems handle both pausing and logging).
- **Event logging is not yet fully separated** into a dedicated system. Logging and side effects (like pausing) are still handled together.
- **Movement system is now clean**—it only handles movement, no logging.
- **Logging is performed both via the standard logger and via pushing to an `EventLog` resource**.

## Recommendations
- **Split logging into a dedicated system** for each event type (e.g., arrivals, moves, spawns). This will further separate concerns and make testing/logging more consistent.
- **Keep only the core logic in each ECS system** (e.g., arrival system should only handle pausing, not logging).
- **Standardize logging approach** (decide when to use the logger vs. the event log resource).

---

## Integration Update (2025-04-25)

- Dedicated logging systems for agent arrival, move, and spawn events are now implemented in `src/ecs/systems/agent_logging.rs`.
- These systems are registered in the ECS schedule in `build_simulation_schedule_profiled`.
- All legacy logging calls have been removed from agent systems (`src/agent/systems.rs`), ensuring logging is now fully modular and centralized.
- Logging for these events now appears in both the `EventLog` resource and the Rust logger (`log::debug!`).

---

## Next Steps
- Test and verify logging output in both destinations.
- Continue ECS audit and refactor for other agent systems.

---
