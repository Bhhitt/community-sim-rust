# Refactor Plan: entity_interaction_system

## Objective
Refactor the `entity_interaction_system` in `src/ecs_components.rs` to follow ECS best practices by splitting it into smaller, single-responsibility systems. This will improve maintainability, reduce borrow conflicts, and align with Legion ECS guidelines.

---

## Background
- The current system handles multiple concerns: agent-food interaction, food collection, stats updates, event logging, and entity removal.
- This monolithic approach increases the risk of borrow conflicts, makes the code harder to maintain, and violates the single responsibility principle.

---

## Refactor Steps

### 1. Audit and Document Current Logic
- [x] Review the current implementation and document all responsibilities handled by `entity_interaction_system`.
- [x] List all queries and resources it reads/writes.

#### **Responsibilities handled by `entity_interaction_system`:**
- Logs the current agent and food counts each tick.
- Tracks agent-agent interactions (agents within 1.5 units of each other).
- Tracks agent-food interactions (agents within 1.0 units of food; randomizes which food is eaten if multiple are in range).
- Collects and removes food entities that are eaten.
- Logs interaction events (agent-agent and agent-food) to the event log.
- Updates `InteractionStats` (total and active interactions, and a history buffer).
- Updates `FoodStats` (collected per tick).
- Updates `AgentEventLog` with agent-food events.

#### **Queries:**
- Query 1: `(Entity, &Position, &InteractionState)` — all agents
- Query 2: `(Entity, &Position, &Food)` — all food entities
- Query 3: `(Entity, &mut Position)` — agents (for mutation)

#### **Resources read/written:**
- `InteractionStats` (write)
- `Arc<Mutex<EventLog>>` (write)
- `FoodStats` (write)
- `AgentEventLog` (write)

### 2. Identify Logical Subsystems
- [x] Food Collection System: Detects and processes agent-food interactions, removes food, updates stats.
- [x] Agent-Agent Interaction System: Detects and logs agent-agent interactions.
- [x] Interaction Stats Update System: Updates `InteractionStats` and history.
- [x] Interaction Event Logging System: Handles event log updates for interactions.

### 3. Design System Interfaces
- [x] Define clear queries and resource access patterns for each new system.
- [x] Ensure each system writes to as few resources as possible.
- [x] Document required execution order (e.g., food collection must precede stats update).

### 4. Implement Modular Systems
- [x] Create new system functions in `src/ecs_components.rs` or a new module (e.g., `src/ecs/systems/interaction.rs`).
- [x] Move/refactor logic from the monolithic system into these new systems.
- [x] Update the interaction schedule builder to add the new systems in the correct order, with `flush()` where needed.

### 5. Update Tests and Documentation
- [x] Update or add tests for each new system.
- [x] Update documentation and audit files to reflect the refactor.

### 6. Remove Monolithic System
- [x] Remove the original `entity_interaction_system` once all new systems are validated.

---

## Additional Notes
- Document all schedule dependencies and order in the builder function.
- Consider adding assertions or checks for required resources.
- Refactor incrementally to ensure simulation correctness at each step.

---

## Additional Task: Identify and Consolidate ECS Schedules

### Background
- Multiple ECS schedule definitions exist in the codebase, including legacy and modular schedules.
- Some are not used in the main simulation and may cause confusion or maintenance burden.

### Steps
1. **Enumerate All Schedules**
    - [x] List all files and functions related to ECS schedule construction:
      - `src/ecs/schedules/mod.rs` — Main modular schedule builder (`build_main_schedule`).
      - `src/ecs/schedules/*.rs` — Modular domain-specific schedule builders (food, agent, spawning, logging, interaction, death, event_log_bridge).
      - `src/legacy/schedule.rs` — Legacy schedule setup, including `run_simulation`, `run_profiles_from_yaml`, etc. (explicitly marked as not used in main simulation).
      - `src/legacy/ecs_sim.rs` — Minimal ECS simulation loop for testing only.

2. **Identify the Active/Main Schedule**
    - [x] The main simulation uses `build_main_schedule` from `src/ecs/schedules/mod.rs`.
    - [x] Modular domain builders (food, agent, etc.) are composed here.
    - [x] Legacy schedules in `legacy/schedule.rs` and `legacy/ecs_sim.rs` are not used in production.

3. **Plan for Consolidation**
    - [x] Clearly document in the refactor plan and code comments which schedule is authoritative.
    - [x] Remove or archive legacy schedules after confirming no dependencies remain.
    - [x] Ensure all tests and documentation reference only the main modular schedule.

### Checklist
- [x] Audit and document all schedule definitions
- [x] Remove or archive unused/legacy schedules
- [x] Update documentation to reference only the main modular schedule

---

## Checklist
- [x] Audit current system and document responsibilities
- [x] Design and implement focused systems
- [x] Update schedule builder and documentation
- [x] Remove legacy monolithic system
- [x] Validate with tests and simulation runs
