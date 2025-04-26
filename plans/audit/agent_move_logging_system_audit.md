# Audit: agent_move_logging_system()

## Location
- File: `src/ecs/systems/agent_logging.rs`
- Function: `agent_move_logging_system()`

## System Code (summary)
```rust
/// Logs agent movement events (position changes).
pub fn agent_move_logging_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMoveLoggingSystem")
        .with_query(<(Entity, &Position, &Target, &AgentState)>::query())
        .build(|_, world, _, query| {
            for (entity, pos, target, agent_state) in query.iter(world) {
                if *agent_state == AgentState::Moving {
                    log::info!("[MOVE] Agent {:?} at ({}, {}) moving toward ({}, {})", entity, pos.x, pos.y, target.x, target.y);
                }
            }
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Only logs agent movement events. No unrelated logic.
- **Query Size & Tuple Limit:**
  - ✅ Queries 4 components: (Entity, &Position, &Target, &AgentState). Well under Legion’s tuple limit.
- **Borrow Patterns:**
  - ✅ No resource borrows, only component reads. No borrow conflicts.
- **Side Effects:**
  - ✅ Only logs information; does not modify state.
- **Domain Appropriateness:**
  - ✅ Clearly an "Agent Logging" system. Should run after movement/state transitions.
- **Testability & Extensibility:**
  - ✅ Logic is clear, can be tested by checking log output.
- **Code Quality:**
  - ✅ Clear, idiomatic ECS code. No extraneous logic.
- **Schedule/Order Dependencies:**
  - ⚠️ Should run after agent movement and state transitions. Document dependency in schedule builder.

## Comments
- The system is focused and minimal.
- Only improvement: document (in schedule builder) that it must run after movement and state transitions.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending modular schedule builder)
