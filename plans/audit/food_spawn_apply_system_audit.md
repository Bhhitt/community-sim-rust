# Audit: food_spawn_apply_system()

## Location
- File: `src/food/systems.rs`
- Function: `food_spawn_apply_system()`

## System Code (summary)
```rust
pub fn food_spawn_apply_system() -> impl systems::Runnable {
    SystemBuilder::new("FoodSpawnApplySystem")
        .write_resource::<PendingFoodSpawns>()
        .write_resource::<FoodStats>()
        .build(|cmd, _world, (pending, food_stats), _| {
            for (x, y) in pending.0.drain(..) {
                let pos = Position { x, y };
                let stats_opt = Some(&mut **food_stats);
                crate::ecs_components::spawn_food(cmd, pos, stats_opt);
            }
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Only applies (spawns) food entities at pending positions. No unrelated logic.
- **Query Size & Tuple Limit:**
  - ✅ No ECS queries; only uses resources and command buffer.
- **Borrow Patterns:**
  - ✅ Mutably borrows `PendingFoodSpawns` and `FoodStats`. No conflicts if scheduled after spawn position collection.
- **Side Effects:**
  - ✅ Only side effect is spawning new food entities.
- **Domain Appropriateness:**
  - ✅ Clearly a "Food System". Should reside in a food schedule module.
- **Testability & Extensibility:**
  - ✅ Logic is straightforward. Can be tested by checking entity creation and resource drain.
- **Code Quality:**
  - ✅ Clear, idiomatic ECS code. Uses command buffer for entity creation.
- **Schedule/Order Dependencies:**
  - ⚠️ Must run after `collect_food_spawn_positions_system()` to process new spawns. Document dependency in schedule builder.

## Comments
- The system is focused and clean.
- Only improvement: document (in schedule builder) that it must run after spawn position collection.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending modular schedule builder)
