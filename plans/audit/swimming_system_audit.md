# Audit: swimming_system() (Legacy/Commented)

## Location
- File: `src/ecs/systems/swimming.rs` (all code commented out)
- Function: `swimming_system()` (legacy, disabled)

## System Code (summary)
```rust
// pub fn swimming_system() -> impl legion::systems::Runnable {
//     SystemBuilder::new("SwimmingSystem")
//         .with_query(<(Entity, &mut Position, &mut Hunger, &mut AgentState, &mut SwimmingProfile, &AgentType)>::query())
//         .read_resource::<Map>()
//         .write_resource::<Arc<Mutex<EventLog>>>()
//         .read_resource::<LogConfig>()
//         .build(|_cmd, world, (map, event_log, log_config), query| {
//             let mut rng = rand::thread_rng();
//             for (entity, pos, hunger, agent_state, swimming_profile, agent_type) in query.iter_mut(world) {
//                 if swimming_profile.swim_ticks_remaining > 0 {
//                     // Move to a random water neighbor, decrement swim_ticks_remaining, update hunger, log event
//                 }
//             }
//         })
// }
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Focused: handles agent swimming logic only (when enabled).
- **Query Size & Tuple Limit:**
  - ⚠️ Large query tuple: 6 components. Acceptable, but further expansion could hit Legion’s limit.
- **Borrow Patterns:**
  - ⚠️ Reads/writes multiple mutable agent components and resources. Could risk borrow conflicts if expanded.
- **Side Effects:**
  - ✅ Moves agent, updates hunger, logs swim events, decrements swim_ticks_remaining.
- **Domain Appropriateness:**
  - ✅ Swimming logic is separate from main movement systems (good modularity).
- **Testability & Extensibility:**
  - ⚠️ Disabled/commented. If re-enabled, ensure logic is testable in isolation and schedule dependencies are clear.
- **Code Quality:**
  - ⚠️ Code is commented out; unclear if up-to-date with current ECS patterns. Needs review if re-enabled.
- **Schedule/Order Dependencies:**
  - ⚠️ If re-enabled, must run after agent state and before movement/death systems. Document dependencies.

## Comments
- This system is currently **disabled/commented out** and not part of the active simulation.
- If swimming is to be re-enabled, review and update logic to match current ECS best practices and modularity.
- Consider splitting into smaller systems if logic expands (e.g., separate movement, hunger, event logging).

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending decision to re-enable and update logic)
