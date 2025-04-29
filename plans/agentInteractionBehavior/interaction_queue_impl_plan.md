# Implementation Plan: `InteractionQueue` Component

## 1. Define the Component Struct
```rust
use std::collections::VecDeque;
use legion::Entity;

#[derive(Clone, Debug, PartialEq)]
pub struct InteractionQueue {
    /// Queue of agents waiting to interact with this agent.
    pub queue: VecDeque<Entity>,
}
```

## 2. Derive Traits for ECS Compatibility
- `Clone`, `Debug`, and `PartialEq` for ECS and testing.
- Add `Serialize`, `Deserialize` if you want to persist or debug state.

## 3. Document the Component
- The struct and field are documented for clarity and future contributors.

## 4. Integrate with ECS
- Add this struct to your ECS components module (e.g., `src/ecs_components.rs`).
- Use it in systems to manage the queue of agents waiting to interact with a target agent.

## 5. Testing/Usage Example
```rust
#[test]
fn test_interaction_queue_basic() {
    let mut queue = InteractionQueue { queue: VecDeque::new() };
    let a = legion::Entity::from_bits(1);
    let b = legion::Entity::from_bits(2);
    queue.queue.push_back(a);
    queue.queue.push_back(b);
    assert_eq!(queue.queue.len(), 2);
    assert_eq!(queue.queue.pop_front(), Some(a));
    assert_eq!(queue.queue.pop_front(), Some(b));
    assert_eq!(queue.queue.pop_front(), None);
}
```

## 6. Checklist for Completion
- [x] Struct defined and documented.
- [x] Traits derived for ECS.
- [ ] (Optional) Traits derived for serde.
- [ ] Component registered in ECS (if required).
- [x] Example/test written.
- [ ] Ready for use in systems.
