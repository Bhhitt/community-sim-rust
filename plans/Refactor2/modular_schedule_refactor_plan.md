# Modular ECS Schedule Builder Refactor Plan

## Objective
Refactor the ECS schedule and tick logic in `ecs_simulation.rs` to use modular, domain-focused schedule builder functions. This will improve maintainability, clarity, and scalability, and align with ECS best practices.

---

## Steps

### 1. Define Schedule Builder Functions by Domain
- Create a function for each major domain that returns a vector of systems or a partial Schedule:
    - `build_food_systems_schedule()`
    - `build_agent_core_systems_schedule()`
    - `build_agent_logging_systems_schedule()`
    - `build_interaction_systems_schedule()`
    - `build_agent_death_systems_schedule()`
    - `build_legacy_systems_schedule()` (for commented/legacy/optional systems)
- Each function should encapsulate only its relevant systems, in the correct order.

### 2. Compose the Main Schedule from Modular Builders
- In `build_simulation_schedule_profiled()`, call each modular builder and add their systems to the main schedule in the desired order.
- This keeps the top-level schedule readable and makes it easy to insert, remove, or reorder domains.

### 3. Update Imports and System Registration
- Move system imports to the top of their respective builder functions, or group them by domain at the top of the file.
- Remove any unused or legacy imports.

### 4. Document Schedule Dependencies
- In each builder, add comments about required order (e.g., “must run after hunger/energy systems”).
- Optionally, add assertions or checks for required resources.

### 5. Transition Tick Logic (Optional for Later)
- Consider modularizing tick logic as well, so that profiling, parallelization, or custom tick phases can be easily swapped.

---

## Example Skeleton

```rust
fn build_food_systems_schedule() -> Vec<Box<dyn Schedulable>> {
    vec![
        Box::new(collect_food_positions_system()),
        Box::new(collect_food_spawn_positions_system()),
        Box::new(food_spawn_apply_system()),
    ]
}

fn build_agent_core_systems_schedule() -> Vec<Box<dyn Schedulable>> {
    vec![
        Box::new(agent_pause_system()),
        Box::new(agent_pausing_system()),
        // ...etc
    ]
}

// ...repeat for other domains...

fn build_simulation_schedule_profiled() -> Schedule {
    let mut builder = Schedule::builder();
    for sys in build_food_systems_schedule() { builder.add_system(sys); }
    for sys in build_agent_core_systems_schedule() { builder.add_system(sys); }
    // ...etc
    builder.build()
}
```

---

## Next Steps

1. Draft the modular schedule builder functions for each domain.
2. Refactor `build_simulation_schedule_profiled()` to use these builders.
3. Test the new modular schedule to ensure system order and dependencies are preserved.
