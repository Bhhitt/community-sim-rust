# Audit: entity_interaction_system()

## Location
- File: `src/ecs_components.rs`
- Function: `entity_interaction_system()`

## System Code (summary)
```rust
// --- ECS Interaction System (agent-agent, agent-food) ---
pub fn entity_interaction_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("EntityInteractionSystem")
        .write_resource::<InteractionStats>()
        .write_resource::<Arc<Mutex<crate::event_log::EventLog>>>()
        .write_resource::<FoodStats>()
        .write_resource::<AgentEventLog>()
        .with_query(<(legion::Entity, &Position, &InteractionState)>::query()) // agents
        .with_query(<(legion::Entity, &Position, &Food)>::query()) // food
        .with_query(<(legion::Entity, &mut Position)>::query())
        // ...
        .build(|cmd, world, (stats, event_log, food_stats, agent_event_log), (agent_query, food_query, pos_query)| {
            // Main interaction logic: agent-food, agent-agent, etc.
            // Handles food collection, updates stats, logs events, removes food, etc.
            // Updates multiple resources and modifies entities.
        })
}
```

## Audit Checklist
- **Single Responsibility:**
  - ⚠️ Handles multiple responsibilities: agent-food interaction, food collection, stats, event logging, and entity removal. Not minimal; could be split into smaller systems.
- **Query Size & Tuple Limit:**
  - ⚠️ Uses multiple queries and writes to several resources. At risk of exceeding Legion’s tuple limit and borrow conflicts as logic grows.
- **Borrow Patterns:**
  - ⚠️ Writes to multiple resources and mutates entities. Potential for borrow conflicts, especially as features expand.
- **Side Effects:**
  - ✅ Modifies resources (stats, logs), removes entities (food), updates world state.
- **Domain Appropriateness:**
  - ⚠️ Currently a catch-all for entity interactions. Would benefit from being split into focused systems (e.g., food collection, agent-agent interaction, event logging).
- **Testability & Extensibility:**
  - ⚠️ Complex logic and multiple responsibilities make it harder to test and extend. Refactor recommended.
- **Code Quality:**
  - ⚠️ Code is functional but not minimal or modular. Needs refactor for clarity and maintainability.
- **Schedule/Order Dependencies:**
  - ⚠️ Order is critical: must run after movement/state transitions, before death systems. Document dependencies in schedule builder.

## Comments
- This system is not minimal and violates single responsibility.
- **Refactor Recommendation:**
  - Split into separate systems:
    - Food collection (agent-food)
    - Agent-agent interaction (if needed)
    - Event logging
    - Stats update
  - Each should have clear, focused queries and minimal resource writes.
- Document schedule dependencies after refactor.

## Audit Status
- ✅ Audited (2025-04-26)
- ⬜ Refactored (refactor required for clarity, modularity, and ECS best practices)
