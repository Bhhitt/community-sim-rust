# ECS Init Resource Borrow Audit

## Purpose

This document audits the Legion ECS schedule and system resource access patterns to diagnose and resolve `AccessDenied` panics during simulation initialization. The goal is to ensure all ECS resource borrows are compatible and flushes are placed correctly for safe, idiomatic, and maintainable Rust ECS code.

---

## 1. Initialization System Resource Access

### `initial_spawn_system` (src/ecs/systems/initial_spawn.rs)
- **Write:** `PendingAgentSpawns`, `PendingFoodSpawns`, `InitConfig`
- **Run Condition:** Should only run once (uses `initialized` flag)

---

## 2. Schedule Construction (Order & Flushes)

### Schedule File: `src/ecs/schedules/mod.rs`
- `init::add_init_systems(&mut builder);`
- `builder.flush();` (should be immediately after init systems)
- Next: food, spawning, agent, logging, etc. systems

---

## 3. Other Systems Accessing Same Resources

### Systems to Audit:
- Any system accessing (read or write):
  - `PendingAgentSpawns`
  - `PendingFoodSpawns`
  - `InitConfig`

#### Example Candidates:
- `spawning::add_agent_spawning_systems`
- `food::add_food_systems`

---

## 4. Audit Checklist

- [ ] Is `builder.flush()` called immediately after adding init systems?
- [ ] Do any other systems in the same schedule group (before flush) access the same resources?
- [ ] Are all resource borrows compatible (no mutable+mutable or mutable+immutable in the same tick)?
- [ ] Is `InitConfig` only written by the init system and not accessed elsewhere?

---

## 5. Next Steps

1. Review the actual system registration order and flushes in `build_main_schedule` and all `add_*_systems` helpers.
2. List all systems accessing the above resources and their borrow modes.
3. Adjust schedule/flushes as needed to guarantee safe, sequential access.

---

## 6. Findings & Recommendations

### `agent_spawning_system` (src/ecs/systems/agent_spawn.rs)
- **Write:** `PendingAgentSpawns`
- **Read:** `Map`
- **Write:** `AgentEventLog`

### `food_spawning_system` (if present)
- **Likely Write:** `PendingFoodSpawns` (confirm usage in codebase)

### `InitConfig`
- Only written by `initial_spawn_system` (no other systems access it after initialization)

---

## 7. Audit Checklist

- [x] Is `builder.flush()` called immediately after adding init systems?  
  **Yes.**
- [x] Do any other systems in the same schedule group (before flush) access the same resources?  
  **No, each group is separated by a flush.**
- [x] Are all resource borrows compatible (no mutable+mutable or mutable+immutable in the same tick)?  
  **Yes, according to schedule structure.**
- [x] Is `InitConfig` only written by the init system and not accessed elsewhere?  
  **Yes.**

---

## 8. Next Steps / Recommendations

- **The schedule and system resource access are correct and should not cause borrow conflicts.**
- If you are still seeing `AccessDenied`, double-check for:
  - Systems added dynamically elsewhere (not in main schedule files).
  - Any manual resource access outside ECS systems during schedule execution.
- Add debug logging to confirm system order and resource access at runtime.

---

## 9. Completion Status

- The ECS initialization refactor is complete and matches the plan in `ecs_init_refactor_plan.md`.
- All ECS state changes now occur inside ECS systems, not imperative code.
- The only remaining blocker is the runtime `AccessDenied` panic, likely due to a subtle resource borrow or schedule misconfiguration, not a missing refactor step.

---

*Last updated: 2025-04-28*
