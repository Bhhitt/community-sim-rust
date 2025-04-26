# Audit: agent_hunger_energy_system()

## Location
- File: `src/ecs/systems/agent.rs`
- Function: `agent_hunger_energy_system()`

## System Code (summary)
```rust
// --- ECS Agent Hunger/Energy System ---
pub fn agent_hunger_energy_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentHungerEnergySystem")
        .with_query(<(Entity, &AgentType, &mut crate::agent::Hunger, &mut crate::agent::Energy, &AgentState)>::query())
        .build(|_, world, _, query| {
            for (_entity, agent_type, hunger, energy, agent_state) in query.iter_mut(world) {
                // Hunger logic (mirrors previous passive_hunger_system)
                if *agent_state == AgentState::Idle || *agent_state == AgentState::Arrived {
                    hunger.value -= agent_type.hunger_rate * 0.1;
                    energy.value -= 0.1; // Example: slow energy drain when idle/arrived
                } else if *agent_state == AgentState::Moving {
                    hunger.value -= agent_type.hunger_rate;
                    energy.value -= 1.0; // Example: faster energy drain when moving
                }
            }
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ✅ Only updates hunger and energy values for agents based on their state. No unrelated logic.
- **Query Size & Tuple Limit:**
  - ✅ Queries 5 components: (Entity, &AgentType, &mut Hunger, &mut Energy, &AgentState). Under Legion’s tuple limit, but should be monitored if expanded.
- **Borrow Patterns:**
  - ✅ No resource borrows, only component mutability. No borrow conflicts.
- **Side Effects:**
  - ✅ Only modifies agent hunger and energy values.
- **Domain Appropriateness:**
  - ✅ Clearly an "Agent Control & State" system. Should live in agent schedule module.
- **Testability & Extensibility:**
  - ✅ Logic is straightforward and easily testable. Could be extended for more complex metabolism.
- **Code Quality:**
  - ✅ Clear, idiomatic ECS code. No extraneous logic.
- **Schedule/Order Dependencies:**
  - ⚠️ Should run before/with agent action decision and death systems, as hunger/energy may affect agent behavior or survival. Document dependency in schedule builder.

## Comments
- The system is focused and minimal.
- Only improvement: document (in schedule builder) its order relative to action/decision and death systems.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (pending modular schedule builder)
