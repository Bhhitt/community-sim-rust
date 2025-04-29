# Implementation Plan: `Interacting` Component

## 1. Define the Component Struct
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Interacting {
    /// The other entity in the interaction.
    pub partner: legion::Entity,
    /// How many ticks remain before the interaction ends.
    pub ticks_remaining: u32,
}
```

## 2. Derive Traits for ECS Compatibility
- `Clone`, `Debug`, and `PartialEq` for ECS and testing.
- Add `Serialize`, `Deserialize` if you want to persist or debug state.

## 3. Document the Component
- The struct and fields are documented for clarity and future contributors.

## 4. Integrate with ECS
- Add this struct to your ECS components module (e.g., `src/ecs_components.rs`).
- Use it in systems to lock agents during interactions and manage interaction duration.

## 5. Testing/Usage Example
```rust
#[test]
fn test_interacting_basic() {
    let mut world = legion::World::default();
    let a = world.push(());
    let b = world.push(());
    let mut interacting = Interacting { partner: b, ticks_remaining: 5 };
    assert_eq!(interacting.partner, b);
    assert_eq!(interacting.ticks_remaining, 5);
    interacting.ticks_remaining -= 1;
    assert_eq!(interacting.ticks_remaining, 4);
}
```

## 6. Checklist for Completion
- [x] Struct defined and documented.
- [x] Traits derived for ECS.
- [ ] (Optional) Traits derived for serde.
- [ ] Component registered in ECS (if required).
- [x] Example/test written.
- [ ] Ready for use in systems.
