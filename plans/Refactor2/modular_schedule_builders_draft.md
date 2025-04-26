# Modular ECS Schedule Builder Functions: Draft

This document drafts the modular builder functions for each domain, as planned.

---

## Food Systems
```rust
pub fn build_food_systems() -> Vec<Box<dyn legion::systems::Schedulable>> {
    vec![
        Box::new(crate::food::systems::collect_food_positions_system()),
        Box::new(crate::food::systems::collect_food_spawn_positions_system()),
        Box::new(crate::food::systems::food_spawn_apply_system()),
    ]
}
```

## Agent Core Systems
```rust
pub fn build_agent_core_systems() -> Vec<Box<dyn legion::systems::Schedulable>> {
    vec![
        Box::new(crate::agent::pause_system::agent_pause_system()),
        Box::new(crate::ecs::systems::agent::agent_pausing_system()),
        Box::new(crate::ecs::systems::agent::agent_hunger_energy_system()),
        Box::new(crate::ecs::systems::agent::agent_path_movement_system()),
        Box::new(crate::ecs::systems::agent::agent_direct_movement_system()),
        Box::new(crate::ecs::systems::agent::agent_state_transition_system()),
    ]
}
```

## Agent Spawning Systems
```rust
pub fn build_agent_spawning_systems() -> Vec<Box<dyn legion::systems::Schedulable>> {
    vec![
        Box::new(crate::ecs::systems::drain_agent_spawn_queue::drain_agent_spawn_queue_system()),
        Box::new(crate::ecs::systems::agent_spawn::agent_spawning_system()),
    ]
}
```

## Agent Logging Systems
```rust
pub fn build_agent_logging_systems() -> Vec<Box<dyn legion::systems::Schedulable>> {
    vec![
        Box::new(crate::ecs::systems::agent_logging::agent_arrival_logging_system()),
        Box::new(crate::ecs::systems::agent_logging::agent_move_logging_system()),
        Box::new(crate::ecs::systems::agent_logging::agent_spawn_logging_system()),
    ]
}
```

## Interaction Systems
```rust
pub fn build_interaction_systems() -> Vec<Box<dyn legion::systems::Schedulable>> {
    vec![
        Box::new(crate::ecs_components::entity_interaction_system()),
    ]
}
```

## Agent Death Systems
```rust
pub fn build_agent_death_systems() -> Vec<Box<dyn legion::systems::Schedulable>> {
    vec![
        Box::new(crate::agent::agent_death_system()),
    ]
}
```

## Agent Event Log Bridge System
```rust
pub fn build_agent_event_log_bridge_system() -> Vec<Box<dyn legion::systems::Schedulable>> {
    vec![
        Box::new(crate::agent::event_log_bridge::agent_event_log_to_gui_system()),
    ]
}
```

## Legacy/Optional Systems
```rust
pub fn build_legacy_systems() -> Vec<Box<dyn legion::systems::Schedulable>> {
    vec![
        // Box::new(crate::ecs::systems::swimming::swimming_system()),
    ]
}
```

---

## Usage Example
In your main schedule builder, call each of these and add their systems in order:

```rust
let mut builder = legion::Schedule::builder();
for sys in build_food_systems() { builder.add_system(sys); }
for sys in build_agent_core_systems() { builder.add_system(sys); }
for sys in build_agent_spawning_systems() { builder.add_system(sys); }
for sys in build_agent_logging_systems() { builder.add_system(sys); }
for sys in build_interaction_systems() { builder.add_system(sys); }
for sys in build_agent_death_systems() { builder.add_system(sys); }
for sys in build_agent_event_log_bridge_system() { builder.add_system(sys); }
for sys in build_legacy_systems() { builder.add_system(sys); }
let schedule = builder.build();
```

---

**Review these drafts and adjust system membership/order as needed before implementation.**
