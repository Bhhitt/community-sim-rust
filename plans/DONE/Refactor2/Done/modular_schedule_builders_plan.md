# Modular ECS Schedule Builders Refactor Plan

## Objective

Modularize the ECS schedule construction by splitting system registration into domain-focused builder functions and composing the main schedule from these modular pieces. This will:
- Improve code clarity and maintainability
- Make system ordering and access patterns explicit
- Reduce runtime borrow conflicts
- Align with ECS and Legion best practices

---

## Steps

### 1. Directory & File Structure
Create a new directory for schedule builders:
```
src/ecs/schedules/
    food.rs
    agent.rs
    spawning.rs
    logging.rs
    interaction.rs
    death.rs
    event_log_bridge.rs
    mod.rs
```

### 2. Per-Domain Schedule Builder Functions
Each file contains a function that adds only the relevant systems for that domain. Example:
```rust
// src/ecs/schedules/food.rs
use legion::systems::Builder;

pub fn add_food_systems(builder: &mut Builder) {
    builder
        .add_system(crate::food::systems::collect_food_positions_system())
        .add_system(crate::food::systems::collect_food_spawn_positions_system())
        .add_system(crate::food::systems::food_spawn_apply_system());
}
```

### 3. Centralized Schedule Composition
In `src/ecs/schedules/mod.rs`:
```rust
mod food;
mod agent;
mod spawning;
mod logging;
mod interaction;
mod death;
mod event_log_bridge;

use legion::systems::Builder;

pub fn build_main_schedule() -> legion::Schedule {
    let mut builder = legion::Schedule::builder();

    food::add_food_systems(&mut builder);
    builder.flush();

    spawning::add_agent_spawning_systems(&mut builder);
    builder.flush();

    agent::add_agent_core_systems(&mut builder);
    builder.flush();

    logging::add_agent_logging_systems(&mut builder);
    builder.flush();

    interaction::add_interaction_systems(&mut builder);
    builder.flush();

    death::add_agent_death_systems(&mut builder);
    builder.flush();

    event_log_bridge::add_agent_event_log_bridge_system(&mut builder);

    builder.build()
}
```

- Use `.flush()` between groups that may have conflicting resource/component access.

### 4. Update Usage
Replace any direct schedule construction with:
```rust
use crate::ecs::schedules::build_main_schedule;
let schedule = build_main_schedule();
```

### 5. Testing & Validation
- After each refactor step, run the simulation to check for borrow conflicts.
- Update the audit table and adjust `.flush()` placement as needed.

---

## Benefits
- Prevents runtime panics due to borrow conflicts
- Makes system order explicit and easy to reason about
- Improves maintainability for future changes or additions
- Aligns with ECS and Legion best practices

---

## Next Steps
1. Implement per-domain schedule builder modules/files
2. Refactor main schedule builder to use the modular layout
3. Test and update documentation/audit tables as you go

DONE
