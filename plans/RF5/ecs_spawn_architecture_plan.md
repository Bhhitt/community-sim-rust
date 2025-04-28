# ECS Unified Spawn Architecture Refactor Plan (RF5)

This plan outlines a step-by-step approach to refactor entity spawning and initialization in the simulation, making all entity creation system-driven, extensible, and idiomatic for Legion ECS. The plan also covers deprecation of legacy code and documentation updates.

---

## **Step 1: Audit Current Spawning Logic**
- [ ] Identify all locations where entities (agents, food, etc.) are spawned directly into the world.
- [ ] Document which entities use spawn queues and which do not.

## **Step 2: Implement Unified Spawn Queues**
- [ ] Create a `PendingFoodSpawns` ECS resource, analogous to `PendingAgentSpawns`.
- [ ] Implement a `food_spawning_system` that consumes `PendingFoodSpawns` and creates food entities.
- [ ] Ensure all entity types (agents, food, etc.) are spawned via their respective queues and systems.

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

## **Step 7: Generalize Spawn Systems (Extensibility)**
- [ ] Refactor spawn systems to use traits/enums for extensibility (e.g., obstacles, power-ups).
- [ ] Document how to add new entity types with minimal boilerplate.

## **Step 8: Deprecate and Mark Old Code**
- [ ] Mark all legacy/unused direct spawn code as `#[deprecated]` or with clear comments.
- [ ] Remove or isolate legacy code in a `legacy/` folder if needed.
- [ ] Update documentation and code comments to reference the new architecture.

## **Step 9: Testing and Debugging Hooks**
- [ ] Add hooks or ECS systems for test assertions/debug output (e.g., after spawn, after first tick).
- [ ] Write integration tests to verify correct entity spawning via queues.

---

## **Summary Table of Changes**

| Area                | Current        | Proposed                  | Benefit                   |
|---------------------|---------------|---------------------------|---------------------------|
| Agent Spawning      | Queue+System  | Use for all entities      | Consistency, safety       |
| Food Spawning       | Direct        | Use queue+system          | Consistency, flexibility  |
| Initialization      | Mixed         | Separate clearly          | Modularity, clarity       |
| Config-driven       | No            | Support configs/scripts   | Flexibility, testability  |
| Scheduling          | Manual        | Use labels/deps           | Robustness, clarity       |
| Testing Hooks       | Minimal       | Add hooks/systems         | Debuggability, coverage   |
| Extensibility       | Manual        | Generalize spawn systems  | Less boilerplate, flex.   |
| Deprecation         | None          | Mark/remove legacy code   | Clean, maintainable code  |

---

## **Notes**
- Prioritize agent and food spawning first; other entity types can follow the same pattern.
- Migration should be incremental; maintain a working build after each step.
- Document all changes and update READMEs as you go.
