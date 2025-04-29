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
- [x] Identify all locations where entities (agents, food, etc.) are spawned directly into the world.
- [x] Document which entities use spawn queues and which do not.

## **Step 2: Implement Per-Type Spawn Queues**
- [x] For each entity type, create a dedicated ECS resource (e.g., `PendingAgentSpawns`, `PendingFoodSpawns`, etc.), each wrapping a `VecDeque<...SpawnRequest>`.
- [x] Implement a dedicated spawn system for each entity type (e.g., `agent_spawning_system`, `food_spawning_system`, etc.) that drains its own queue and creates entities.
- [x] Ensure all entity types are spawned via their respective queues and systems. (Agents and food complete; other types pending if needed)

## **Step 3: Refactor Initialization Logic**
- [x] Update `setup_simulation_world_and_resources` to enqueue spawn requests for all initial entities (agents, food, etc.) into their respective queues.
- [x] Remove all direct `world.push` calls for entities from initialization code.
- [x] Ensure the first tick of the simulation processes all pending spawn requests.

## **Step 4: Separate Initialization Stages**
- [x] Split setup into:
    - World/resource creation
    - Initial spawn request enqueuing
    - Simulation loop execution
- [x] Provide hooks or log output to inspect world/resource state after each stage. (Logging added)

## **Step 5: Data-Driven Initialization (Optional)**
- [x] Allow initial entities and properties to be loaded from config files (YAML/JSON) or scenario scripts.
- [x] Implement logic to enqueue spawn requests based on these configs.
- [x] **Profiles remain the entry point for scenario selection.**
- [x] **Profiles can optionally reference a `spawn_config` for explicit spawns.**

### **Integration Strategy: Profiles + Spawn Config**

- Each profile in `sim_profiles.yaml` may include a `spawn_config` field (filename/path).
- If `spawn_config` is present, load and apply it for explicit entity spawns.
- If not present, retain current procedural/random spawn logic based on profile fields (e.g., `num_agents`).
- Loader merges both: config-driven spawns take priority, procedural logic fills any gaps.
- This approach is fully backwards compatible—existing profiles work as before.
- Document this workflow in both the plan and codebase.
- **Status:** Implemented and tested. The simulation now loads YAML-based spawn configs and enqueues initial spawns from them. See `demo_spawn.yaml` and the `custom_demo` profile for working examples.

#### **Example Profile**
```yaml
- name: custom_demo
  map_width: 100
  map_height: 100
  num_agents: 10
  ticks: 50
  spawn_config: "demo_spawn.yaml"
```

### **Design: Data-Driven Spawn Configuration**

#### **Config File Structure (YAML Example)**
```yaml
map:
  width: 200
  height: 200

agents:
  - type: worker
    pos: { x: 10, y: 20 }
    count: 5
  - type: scout
    pos: { x: 30, y: 40 }
    count: 2

food:
  - pos: { x: 15, y: 25 }
    count: 10

items:
  - type: medkit
    pos: { x: 50, y: 60 }
    count: 1

money:
  - pos: { x: 100, y: 100 }
    amount: 500
    count: 3
```

#### **Design Notes**
- Each entity type (agents, food, items, money, etc.) is specified as a list.
- Each entry can specify a `count` for batch spawning, or a single instance.
- Positions are explicit (`x`, `y`).
- Entity-specific fields (e.g., `type`, `amount`) are included as needed.
- The config can be extended for new entity types with minimal changes.

#### **Integration Plan**
- Add a loader function to parse this config (YAML/JSON).
- During initialization, for each entry, enqueue the appropriate spawn requests into the ECS resource queues.
- If both config and procedural/random spawning are desired, merge or prioritize as needed.
- Document the schema in code and in this plan.

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
- [x] Mark all legacy/unused direct spawn code as `#[deprecated]` or with clear comments. (Most legacy code removed; comment remaining if found)
- [x] Remove or isolate legacy code in a `legacy/` folder if needed. (No major legacy code remains)
- [x] Update documentation and code comments to reference the new architecture.

## **Step 9: Testing and Debugging Hooks**
- [x] Add hooks or ECS systems for test assertions/debug output (e.g., after spawn, after first tick). (Logging and debug output added)
- [ ] Write integration tests to verify correct entity spawning via queues.

---

## **Summary Table of Changes**

| Area                | Current        | Proposed (Per-Type Queues)  | Benefit                   |
|---------------------|---------------|-----------------------------|---------------------------|
| Spawning            | Mixed direct/queue | Dedicated per-type queues  | Parallel, idiomatic ECS   |
| Initialization      | Monolithic    | Split, staged               | Testable, modular         |
| Extensibility       | Manual        | Standardized per-type queue | Easy to add new types     |
| Deprecation         | None          | Mark/remove legacy code     | Clean, maintainable code  |

---

## **Notes**
- As of 2025-04-27, all core simulation runners use the split-stage ECS initialization pipeline.
- All initial agent/food spawns are now enqueued and processed by ECS systems.
- Legacy direct spawn logic has been removed from both headless and graphics runners.
- Logging is present after each initialization stage for debugging and profiling.
- Next steps: implement data-driven initialization and add integration tests for spawn correctness.
