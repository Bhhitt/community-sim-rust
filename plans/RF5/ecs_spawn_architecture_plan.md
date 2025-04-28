# ECS Unified Spawn Architecture Refactor Plan (RF5)

This plan outlines a step-by-step approach to refactor entity spawning and initialization in the simulation, making all entity creation system-driven, extensible, and idiomatic for Legion ECS. The plan also covers deprecation of legacy code and documentation updates.

---

## **Architectural Choice for Massive Scale**
**For maximum scalability and parallelism, use a dedicated spawn queue/resource for each entity type (agent, food, item, money, etc.), with a dedicated spawn system for each.**
- This enables cache-friendly, parallel processing and avoids contention or filtering overhead.
- Each system only processes its own queue, and Legion can schedule them in parallel.
- This approach is idiomatic and best practice for high-performance ECS in Rust.

---

## **Step 1: Audit Current Spawning Logic**
- [ ] Identify all locations where entities (agents, food, etc.) are spawned directly into the world.
- [ ] Document which entities use spawn queues and which do not.

## **Step 2: Implement Per-Type Spawn Queues**
- [ ] For each entity type, create a dedicated ECS resource (e.g., `PendingAgentSpawns`, `PendingFoodSpawns`, `PendingItemSpawns`, etc.), each wrapping a `VecDeque<...SpawnRequest>`.
- [ ] Implement a dedicated spawn system for each entity type (e.g., `agent_spawning_system`, `food_spawning_system`, etc.) that drains its own queue and creates entities.
- [ ] Ensure all entity types are spawned via their respective queues and systems.

### Example:
```rust
// In ecs/systems/pending_agent_spawns.rs
pub struct PendingAgentSpawns(pub VecDeque<AgentSpawnRequest>);

// In ecs/systems/pending_food_spawns.rs
pub struct PendingFoodSpawns(pub VecDeque<FoodSpawnRequest>);

// In ecs/systems/pending_item_spawns.rs
pub struct PendingItemSpawns(pub VecDeque<ItemSpawnRequest>);

// Each system only processes its own queue:
pub fn agent_spawning_system() -> impl Runnable { /* ... */ }
pub fn food_spawning_system() -> impl Runnable { /* ... */ }
pub fn item_spawning_system() -> impl Runnable { /* ... */ }
```

---

## **Step 3: Refactor Initialization Logic**
- [ ] Update `setup_simulation_world_and_resources` to enqueue spawn requests for all initial entities (agents, food, etc.) into their respective queues.
- [ ] Remove all direct `world.push` calls for entities from initialization code.
- [ ] Ensure the first tick of the simulation processes all pending spawn requests.

## **Step 4: Separate Initialization Stages**
- [ ] Split setup into:
    - World/resource creation
    - Initial spawn request enqueuing
    - Simulation loop execution
- [ ] Provide hooks or log output to inspect world/resource state after each stage.

## **Step 5: Data-Driven Initialization (Optional)**
- [ ] Allow initial entities and properties to be loaded from config files (YAML/JSON) or scenario scripts.
- [ ] Implement logic to enqueue spawn requests based on these configs.

## **Step 6: System Scheduling Improvements**
- [ ] Use Legion system labels/dependencies (if needed) for explicit ordering (e.g., group all spawn systems in a dedicated stage).
- [ ] Replace manual `.flush()` chains with more robust scheduling if possible.

## **Step 7: Generalize and Document Extensibility**
- [ ] Document the per-type queue approach as the standard for new entity types.
- [ ] For each new entity type (e.g., Money), add:
    - A spawn request struct (e.g., `MoneySpawnRequest`)
    - A queue resource (e.g., `PendingMoneySpawns`)
    - A spawn system (e.g., `money_spawning_system`)
- [ ] Document how to add new entity types with minimal boilerplate.

### Example: Adding Money Entities
```rust
pub struct MoneySpawnRequest {
    pub pos: Position,
    pub amount: u32,
}

pub struct PendingMoneySpawns(pub VecDeque<MoneySpawnRequest>);

pub fn money_spawning_system() -> impl Runnable { /* ... */ }

// Enqueue a spawn request:
pending_money_spawns.push_back(MoneySpawnRequest { pos, amount: 100 });
```

## **Step 8: Deprecate and Mark Old Code**
- [ ] Mark all legacy/unused direct spawn code as `#[deprecated]` or with clear comments.
- [ ] Remove or isolate legacy code in a `legacy/` folder if needed.
- [ ] Update documentation and code comments to reference the new architecture.

## **Step 9: Testing and Debugging Hooks**
- [ ] Add hooks or ECS systems for test assertions/debug output (e.g., after spawn, after first tick).
- [ ] Write integration tests to verify correct entity spawning via queues.

---

## **Summary Table of Changes**

| Area                | Current        | Proposed (Per-Type Queues)  | Benefit                   |
|---------------------|---------------|-----------------------------|---------------------------|
| Agent Spawning      | Queue+System  | Per-type queue+system       | Scale, parallelism        |
| Food Spawning       | Direct        | Per-type queue+system       | Consistency, flexibility  |
| Initialization      | Mixed         | Separate clearly            | Modularity, clarity       |
| Config-driven       | No            | Support configs/scripts     | Flexibility, testability  |
| Scheduling          | Manual        | Use labels/deps             | Robustness, clarity       |
| Testing Hooks       | Minimal       | Add hooks/systems           | Debuggability, coverage   |
| Extensibility       | Manual        | Per-type queue+system       | Less boilerplate, flex.   |
| Deprecation         | None          | Mark/remove legacy code     | Clean, maintainable code  |

---

## **Notes**
- Prioritize agent and food spawning first; other entity types can follow the same pattern.
- Migration should be incremental; maintain a working build after each step.
- Document all changes and update READMEs as you go.
- For truly massive scale, preallocate queues and batch entity creation where possible.
