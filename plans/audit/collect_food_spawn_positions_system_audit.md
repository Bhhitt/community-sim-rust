# Audit: collect_food_spawn_positions_system()

## Location
- File: `src/food/systems.rs`
- Function: `collect_food_spawn_positions_system()`

## System Code (summary)
```rust
pub fn collect_food_spawn_positions_system() -> impl systems::Runnable {
    SystemBuilder::new("CollectFoodSpawnPositionsSystem")
        .write_resource::<PendingFoodSpawns>()
        .read_resource::<crate::map::Map>()
        .build(|_, _world, (pending_food, map), _| {
            let num_to_spawn = (map.width * map.height / 20000).max(2);
            let mut rng = rand::thread_rng();
            let mut positions_to_spawn = Vec::new();
            for _ in 0..num_to_spawn {
                let mut x;
                let mut y;
                let mut tries = 0;
                loop {
                    x = rng.gen_range(0..map.width) as f32;
                    y = rng.gen_range(0..map.height) as f32;
                    if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                        break;
                    }
                    tries += 1;
                    if tries > 1000 {
                        break;
                    }
                }
                positions_to_spawn.push((x, y));
            }
            pending_food.0 = positions_to_spawn.into();
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Only determines valid spawn positions for food and updates `PendingFoodSpawns`. No unrelated logic.
- **Query Size & Tuple Limit:**
  - ✅ No ECS queries; only uses resources. No risk of tuple limit.
- **Borrow Patterns:**
  - ✅ Mutably borrows `PendingFoodSpawns`, reads `Map`. No conflicts if scheduled properly.
- **Side Effects:**
  - ✅ No side effects beyond updating the pending food spawns resource.
- **Domain Appropriateness:**
  - ✅ Clearly a "Food System". Should reside in a food schedule module.
- **Testability & Extensibility:**
  - ✅ Logic is straightforward and can be tested in isolation. Parameters (e.g., spawn density) could be made configurable.
- **Code Quality:**
  - ✅ Clear, idiomatic ECS code. Uses reasonable retry logic for valid positions.
- **Schedule/Order Dependencies:**
  - ⚠️ Should run before food spawning/apply systems. Dependency should be documented in schedule builder.

## Comments
- The system is focused and clean.
- Only improvement: document (in schedule builder) that it must run before food spawn application.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending modular schedule builder)
