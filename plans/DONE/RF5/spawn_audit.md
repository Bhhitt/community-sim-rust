# Step 1: Audit of Current Spawning Logic

This file documents all locations in the codebase where entities (agents, food, etc.) are spawned directly into the Legion ECS world, as well as which entities use spawn queues and which do not.

---

## 1. Direct World Spawning (world.push)

### Food
- **File:** `src/sim_core.rs`
- **Context:** In `setup_simulation_world_and_resources`, food entities are spawned directly:
  ```rust
  world.push((Position { x, y }, Food { nutrition: ... }));
  ```
- **Status:** Direct world mutation. Does NOT use a spawn queue.

### Test/Legacy Code
- **File:** `src/legacy/ecs_hello.rs` (test)
- **Context:** Directly pushes positions to the world for testing.
- **File:** `src/legacy/ecs_sim.rs`
- **Context:** Test/legacy simulation pushes entities directly.

---

## 2. Spawn Queues (Queued/Deferred Spawning)

### Agents
- **File:** `src/ecs/systems/pending_agent_spawns.rs`
- **Resource:** `PendingAgentSpawns(VecDeque<AgentSpawnRequest>)`
- **Status:** Agents are intended to be spawned via this queue and a corresponding system, but in `setup_simulation_world_and_resources` the code to enqueue agent spawn requests is currently commented out (agents are NOT actually spawned at init).

### Food (Partial/Old)
- **File:** `src/food/components.rs`, `src/food/systems.rs`
- **Resource:** `PendingFoodSpawns(VecDeque<(f32, f32)>)`
- **System:** `food_spawn_apply_system` drains this queue and spawns food entities.
- **Status:** This mechanism exists but is NOT used in main initialization; food is still spawned directly in `sim_core.rs`.

---

## 3. Other Entities
- No evidence of other entity types (items, money, etc.) being spawned yet. No spawn queues or direct world mutation found for these.

---

## 4. Summary Table
| Entity Type | Direct world.push | Spawn Queue + System | Used in Main Init? |
|-------------|------------------|----------------------|-------------------|
| Agent       | No (intended, but not present) | Yes (PendingAgentSpawns) | No (queue not used in init) |
| Food        | Yes              | Yes (PendingFoodSpawns) | Direct only |
| Item/Money  | No               | No                   | N/A               |
| Legacy/Test | Yes              | No                   | Test only         |

---

## 5. Notes
- **Food**: Main code uses direct spawn; queue-based system exists but is not integrated.
- **Agents**: Queue system exists but is not used at initialization.
- **Legacy/Test**: Direct world mutation is used for simple ECS demos/tests.
- **Next Step**: Refactor all entity initialization to use per-type spawn queues and systems.
