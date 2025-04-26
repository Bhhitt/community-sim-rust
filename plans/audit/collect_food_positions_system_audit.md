# Audit: collect_food_positions_system()

## Location
- File: `src/food/systems.rs`
- Function: `collect_food_positions_system()`

## System Code
```rust
pub fn collect_food_positions_system() -> impl systems::Runnable {
    SystemBuilder::new("CollectFoodPositionsSystem")
        .write_resource::<FoodPositions>()
        .with_query(<(&Position, &Food)>::query())
        .build(|_, world, food_positions, query| {
            let mut positions = Vec::new();
            for (pos, _food) in query.iter(world) {
                positions.push((pos.x, pos.y));
            }
            food_positions.0 = positions;
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Only collects all food entity positions and stores them in the `FoodPositions` resource. No extra logic or side effects.
- **Query Size & Tuple Limit:**
  - ✅ Only queries `(&Position, &Food)`. Very minimal and focused.
- **Borrow Patterns:**
  - ✅ Only mutably borrows the `FoodPositions` resource. No conflicts expected if scheduled properly.
- **Side Effects:**
  - ✅ No side effects. Only updates the `FoodPositions` resource.
- **Domain Appropriateness:**
  - ✅ Clearly a "Food System". Should live in a food schedule module.
- **Testability & Extensibility:**
  - ✅ Very easy to test and extend (e.g., could add filtering if needed).
- **Code Quality:**
  - ✅ Clear, idiomatic Legion ECS code. Well-structured.
- **Schedule/Order Dependencies:**
  - ⚠️ Should run before any system that needs up-to-date food positions (e.g., agent targeting). Dependency should be documented in schedule builder.

## Comments
- This system is already minimal and well-designed.
- Only improvement: document (in schedule builder) that it must run before agent targeting systems.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending modular schedule builder)
