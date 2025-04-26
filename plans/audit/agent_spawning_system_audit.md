# Audit: agent_spawning_system()

## Location
- File: `src/ecs/systems/agent_spawn.rs`
- Function: `agent_spawning_system()`

## System Code (summary)
```rust
/// Spawns new agents from pending requests in PendingAgentSpawns resource.
pub fn agent_spawning_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentSpawningSystem")
        .write_resource::<PendingAgentSpawns>()
        .build(|cmd, _world, pending_spawns, _| {
            while let Some(spawn_request) = pending_spawns.0.pop() {
                crate::ecs_components::spawn_agent(cmd, spawn_request);
            }
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Only spawns new agents from PendingAgentSpawns. No unrelated logic.
- **Query Size & Tuple Limit:**
  - ✅ No query, only resource write. No risk of tuple limit.
- **Borrow Patterns:**
  - ✅ Only borrows PendingAgentSpawns mutably. No borrow conflicts.
- **Side Effects:**
  - ✅ Spawns new entities in the ECS world.
- **Domain Appropriateness:**
  - ✅ Clearly an "Agent Spawning & Queue" system. Should run early in agent schedule.
- **Testability & Extensibility:**
  - ✅ Logic is clear, can be tested by checking spawn queue and world state.
- **Code Quality:**
  - ✅ Clear, idiomatic ECS code. No extraneous logic.
- **Schedule/Order Dependencies:**
  - ⚠️ Should run before agent systems that operate on new agents. Document dependency in schedule builder.

## Comments
- The system is focused and minimal.
- Only improvement: document (in schedule builder) that it must run before agent control/state systems.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending modular schedule builder)
