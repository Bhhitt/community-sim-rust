# Audit: agent_direct_movement_system()

## Location
- File: `src/ecs/systems/agent.rs`
- Function: `agent_direct_movement_system()`

## System Code (summary)
```rust
// --- ECS Agent Direct Movement System ---
/// Moves agent directly toward target if no path is present.
pub fn agent_direct_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDirectMovementSystem")
        .with_query(<(&mut Position, &AgentType, &mut Target, &mut AgentState)>::query())
        .build(|_, world, _, query| {
            for (pos, agent_type, target, agent_state) in query.iter_mut(world) {
                if *agent_state == AgentState::Idle || *agent_state == AgentState::Moving {
                    let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                    let step = agent_type.movement_profile.speed.min(dist);
                    if dist > 0.1 {
                        pos.x += (target.x - pos.x) / dist * step;
                        pos.y += (target.y - pos.y) / dist * step;
                    } else {
                        pos.x = target.x;
                        pos.y = target.y;
                        // State transition handled elsewhere
                    }
                }
            }
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Moves agent directly toward its target if no path is present. No unrelated logic.
- **Query Size & Tuple Limit:**
  - ✅ Queries 4 components: (&mut Position, &AgentType, &mut Target, &mut AgentState). Well under Legion’s tuple limit.
- **Borrow Patterns:**
  - ✅ No resource borrows, only component mutability. No borrow conflicts.
- **Side Effects:**
  - ✅ Only modifies agent position and state.
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
