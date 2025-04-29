# ECS Schedule Refactor Plan (RF5)

## Objective
Refactor the Legion ECS schedule to use idiomatic system labels and dependencies, enabling robust ordering, parallelism, and extensibility. Ensure all system groups (food, spawning, agent core, logging, interaction, death, event log bridge, etc.) are included and documented. Remove legacy `.flush()` usage where possible.

---

## Step-by-Step Guide

### 1. **Audit and Reactivate System Groups**
- [x] Review all commented-out system groups in `build_main_schedule` (spawning, logging, interaction, death, event log bridge).
- [x] Remove duplicated agent systems from spawning group; keep them only in agent core.
- [x] Reactivate all food systems (collect, collection, spawn, apply).
- [x] Reactivate or refactor any remaining essential systems as needed.
    - [2025-04-28] All core system groups (spawning, food, agent core, logging, death, event log bridge) are now active in the schedule. Food and agent spawning/collection are fully functional in both headless and graphical modes. Any legacy/unused schedules have been deprecated per audit.

### 2. **Replace `.flush()` with System Labels/Dependencies**
- [ ] Assign a unique label to each system group (e.g., "food", "spawning", "agent_core", etc.).
- [ ] Use Legion's `before`/`after` dependencies to enforce correct ordering between groups.
- [ ] Remove `.flush()` calls except where absolutely necessary (e.g., for resource boundaries or side-effecting systems).

### 3. **Parallelize Where Possible**
- [ ] Group independent systems to run in parallel within their stage.
- [ ] Only enforce sequential execution where data dependencies exist.

### 4. **Standardize Adding New System Groups**
- [ ] Document the process for adding a new system group (label, dependencies, registration).
- [ ] Provide an example/template in both the codebase and this plan.

### 5. **Update Documentation and Comments**
- [ ] Clearly document the intended order and parallelism of all system groups in code and this plan.
- [ ] Mark any legacy or transitional code.
- [ ] Add a summary table of system group order and dependencies.

### 6. **Test and Validate**
- [ ] Run both headless and graphics modes to ensure all systems execute in the correct order and no panics or data races occur.
- [ ] Add debug logging to verify system execution order.

### 7. **Review and Iterate**
- [ ] Review the refactored schedule for clarity, robustness, and extensibility.
- [ ] Iterate based on testing and feedback.

---

## Rationale
- **System labels and dependencies** allow for more robust, declarative ordering and maximize parallelism.
- **Removing unnecessary `.flush()`** improves performance and reduces boilerplate.
- **Standardizing system group registration** makes the ECS schedule easier to extend and maintain.

---

## Example: System Group Labeling
```rust
builder.add_system(food_system().label("food"));
builder.add_system(spawning_system().label("spawning").after("food"));
builder.add_system(agent_core_system().label("agent_core").after("spawning"));
// ...
```

---

## Summary Table (to be updated during refactor)
| Group         | Label        | Depends On     |
|---------------|-------------|----------------|
| Food          | food        |                |
| Spawning      | spawning    | food           |
| Agent Core    | agent_core  | spawning       |
| Logging       | logging     | agent_core     |
| Interaction   | interaction | agent_core     |
| Death         | death       | interaction    |
| Event Bridge  | event_log   | death          |

---

**Status:** _Plan updated to reflect food system reactivation and removal of duplicated agent systems. Continue with label/dependency refactor next._
