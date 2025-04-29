# Implementation Plan: `InteractionIntent` Component

## 1. Define the Component Struct
```rust
use legion::Entity;

#[derive(Clone, Debug, PartialEq)]
pub struct InteractionIntent {
    /// The entity being pursued for interaction.
    pub target: Entity,
    /// How many ticks the agent has spent pursuing.
    pub ticks_pursued: u32,
    /// Maximum ticks to pursue before giving up.
    pub max_pursue_ticks: u32, // e.g., 50
}
```

## 2. Derive Traits for ECS Compatibility
- `Clone`, `Debug`, and `PartialEq` are derived for ECS and testing.
- Add `Serialize`, `Deserialize` if you want to persist or debug state.

## 3. Document the Component
- The struct and fields are documented for clarity and future contributors.

## 4. Integrate with ECS
- Add this struct to your ECS components module (e.g., `src/ecs_components.rs`).
- Use it in systems when an agent decides to pursue another agent for interaction.

## 5. Testing/Usage Example
```rust
#[test]
fn test_interaction_intent_creation() {
    let target = legion::Entity::from_bits(42);
    let intent = InteractionIntent { target, ticks_pursued: 0, max_pursue_ticks: 50 };
    assert_eq!(intent.target, target);
    assert_eq!(intent.ticks_pursued, 0);
    assert_eq!(intent.max_pursue_ticks, 50);
}
```

## 6. Checklist for Completion
- [x] Struct defined and documented.
- [x] Traits derived for ECS.
- [ ] (Optional) Traits derived for serde.
- [ ] Component registered in ECS (if required).
- [x] Example/test written.
- [ ] Ready for use in systems.
