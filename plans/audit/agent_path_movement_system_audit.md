# Audit: agent_path_movement_system()

## Location
- File: `src/ecs/systems/agent.rs`
- Function: `agent_path_movement_system()`

## System Code (summary)
```rust
// --- ECS Agent Path Movement System ---
/// Moves agent along waypoints if Path is present and not empty.
pub fn agent_path_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentPathMovementSystem")
        .with_query(<(&mut Position, &AgentType, &mut Path, &mut Target, &mut AgentState)>::query())
        .build(|_, world, _, query| {
            for (pos, agent_type, path, maybe_target, agent_state) in query.iter_mut(world) {
                if *agent_state == AgentState::Idle || *agent_state == AgentState::Moving {
                    if !path.waypoints.is_empty() {
                        let (tx, ty) = path.waypoints.front().unwrap();
                        let dx = *tx as f32 - pos.x;
                        let dy = *ty as f32 - pos.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        let step = agent_type.movement_profile.speed.min(dist);
                        pos.x += dx / dist * step;
                        pos.y += dy / dist * step;
                        path.waypoints.pop_front();
                    } else {
                        pos.x = maybe_target.x;
                        pos.y = maybe_target.y;
                        // State transition handled elsewhere
                    }
                }
            }
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Moves agent along its path if present. No unrelated logic.
- **Query Size & Tuple Limit:**
  - ✅ Queries 5 components: (&mut Position, &AgentType, &mut Path, &mut Target, &mut AgentState). Under Legion’s tuple limit, but should be monitored if expanded.
- **Borrow Patterns:**
  - ✅ No resource borrows, only component mutability. No borrow conflicts.
- **Side Effects:**
  - ✅ Only modifies agent position, path, target, and state.
- **Domain Appropriateness:**
  - ✅ Clearly an "Agent Control & State" system. Should live in agent schedule module.
- **Testability & Extensibility:**
  - ✅ Logic is clear and can be tested for correct movement. Can be extended for more movement logic.
- **Code Quality:**
  - ✅ Clear, idiomatic ECS code. No extraneous logic.
- **Schedule/Order Dependencies:**
  - ⚠️ Should run before/with state transition and movement history systems. Document dependency in schedule builder.

## Comments
- The system is focused and minimal.
- Only improvement: document (in schedule builder) its order relative to state transition/history systems.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending modular schedule builder)
