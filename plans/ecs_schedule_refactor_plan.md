# ECS Schedule & Tick Logic Refactor Plan

## Objective
Refactor the ECS schedule and tick logic in `ecs_simulation.rs` to split responsibilities into focused, modular ECS systems and schedule builders. This will improve maintainability, clarity, and scalability, and align with ECS best practices.

---

## Current Issues
- Monolithic schedule construction and tick logic in a single file.
- Difficult to reason about, extend, or test individual system groupings.
- Lacks clear separation of concerns (agent, food, rendering, etc.).

---

## Refactor Plan

### 1. **Audit and Categorize Existing Systems**
- List all systems currently registered in the ECS schedule.
- Categorize them by domain (e.g., agent, food, rendering, input, debug/logging).

### 2. **Create Modular Schedule Builder Functions**
- For each domain, create a function (e.g., `build_agent_systems()`, `build_food_systems()`) that returns a `Schedule` or a list of systems for that domain.
- Place these in appropriately named modules/files (e.g., `src/ecs/schedules/agent.rs`).

### 3. **Decouple Tick Logic from Schedule Construction**
- Move tick logic (e.g., profiling, timing, world stepping) into a dedicated function/module.
- Ensure the main simulation loop is only responsible for driving the tick, not for assembling the schedule.

### 4. **Compose the Main Schedule from Modular Pieces**
- In `ecs_simulation.rs`, assemble the overall schedule by combining the modular builders in the desired order (with `.flush()` calls as needed).
- This makes the schedule easy to read and modify.

### 5. **Update Imports and Module Structure**
- Remove legacy/unused imports.
- Update `mod.rs` files to expose new schedule builders.

### 6. **Testing & Validation**
- Ensure all systems are still registered and run in the correct order.
- Test for correctness, performance, and absence of borrow conflicts.

---

## Example Structure

```
src/ecs/schedules/
  agent.rs         // build_agent_systems()
  food.rs          // build_food_systems()
  rendering.rs     // build_rendering_systems()
  ...
```

`ecs_simulation.rs`:
```rust
let mut schedule = Schedule::builder()
    .add_systems(build_agent_systems())
    .flush()
    .add_systems(build_food_systems())
    .flush()
    .add_systems(build_rendering_systems())
    ...
    .build();
```

---

## Next Steps
1. Review and approve this plan.
2. Audit all current systems and propose the initial modular breakdown.
3. Implement modular schedule builders, one domain at a time.
4. Refactor tick logic and compose the new schedule.
5. Test and iterate.
