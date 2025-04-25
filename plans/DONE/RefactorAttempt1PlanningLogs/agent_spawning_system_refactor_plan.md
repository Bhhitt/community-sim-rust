# Agent Spawning System Refactor Plan

## Goal
Move agent spawning logic from the standalone `spawn_agent` function to a dedicated ECS system (`agent_spawning_system`) in a new module (`ecs/systems/agent_spawn.rs`). This enables ECS-driven spawning, modularity, and future extensibility (e.g., batch spawns, scripted spawns).

## Step-by-Step Plan

### 1. Design the Trigger Mechanism
- Decide how agent spawning is triggered:
  - **Resource-driven:** Use a `PendingAgentSpawns` resource (queue of spawn requests).
  - **Event-driven:** Use an event queue for spawn requests.

### 2. Create a PendingAgentSpawns Resource
- Define a struct to hold pending spawn requests (positions, agent types, etc.).
- Add as a Legion resource.

### 3. Move/Refactor spawn_agent Logic
- Move logic from `spawn_agent` to a new ECS system (`agent_spawning_system`) in `ecs/systems/agent_spawn.rs`.
- The system:
  - Iterates over pending spawn requests.
  - For each request, creates a new agent entity with all required components.
  - Logs the spawn event.
  - Removes processed requests from the queue.

### 4. Update All Callers
- Replace direct calls to `spawn_agent` with logic that adds requests to `PendingAgentSpawns`.
- Update UI, tests, and scripts to use the queue-based pattern.

### 5. Register the New System
- Add `agent_spawning_system` to the ECS schedule.

### 6. Testing & Validation
- Test agent spawning in all scenarios.
- Ensure logs/analytics are correct.
- Remove or deprecate the old `spawn_agent` function.

### 7. Documentation
- Update refactor plan and audit files to reflect the new system and rationale.

## Optional Enhancements
- Generalize spawn system for other entity types (food, objects, etc.).
- Add support for spawn parameters (randomization, effects, etc.).
- Provide unit tests for the new resource and system.
