# Audit: agent_death_system()

## Location
- File: `src/agent/systems.rs`
- Function: `agent_death_system()`

## System Code (summary)
```rust
// --- ECS Agent Death System ---
pub fn agent_death_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDeathSystem")
        .with_query(<(Entity, &crate::agent::Hunger, &crate::agent::Energy)>::query())
        .build(|cmd, world, _, query| {
            let mut to_remove = Vec::new();
            for (entity, hunger, energy) in query.iter(world) {
                if hunger.value <= 0.0 || energy.value <= 0.0 {
                    to_remove.push(entity);
                }
            }
            for entity in to_remove {
                cmd.remove(*entity);
            }
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Only removes agents with zero or negative hunger/energy. No unrelated logic.
- **Query Size & Tuple Limit:**
  - ✅ Queries 3 components: (Entity, &Hunger, &Energy). Well under Legion’s tuple limit.
- **Borrow Patterns:**
  - ✅ No resource borrows, only component reads. No borrow conflicts.
- **Side Effects:**
  - ✅ Removes entities from the ECS world.
- **Domain Appropriateness:**
  - ✅ Clearly an "Entity & Agent Interaction" system. Should run after hunger/energy update systems.
- **Testability & Extensibility:**
  - ✅ Logic is clear, can be tested by setting agent hunger/energy and checking world state.
- **Code Quality:**
  - ✅ Clear, idiomatic ECS code. No extraneous logic.
- **Schedule/Order Dependencies:**
  - ⚠️ Should run after hunger/energy systems. Document dependency in schedule builder.

## Comments
- The system is focused and minimal.
- Only improvement: document (in schedule builder) that it must run after hunger/energy systems.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending modular schedule builder)
