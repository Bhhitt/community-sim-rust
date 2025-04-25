# Agent Systems Query Audit (2025)

| System | Status | Notes |
|--------|--------|-------|
| agent_movement_system | ✅ Done | TODO: Consider decoupling movement logic by agent type for future extensibility. |
| agent_movement_history_system | ✅ Done | Moved to ecs/systems/agent.rs for modular ECS organization. |
| agent_death_system | ✅ Done | Removes agents with zero hunger/energy. Minimal, single-responsibility. |
| swimming_system | ✅ Done | Moved to ecs/systems/swimming.rs for ECS modularity. |

## 1. agent_movement_system (legacy, refactored)

**File:** src/agent/systems.rs → now src/ecs/systems/agent.rs

**Legacy Query:**
```rust
.with_query::<(
    &mut crate::ecs_components::Position,
    &crate::agent::AgentType,
    &mut crate::agent::Hunger,
    Option<&mut Target>,
    Option<&mut Path>,
    &mut crate::agent::AgentState,
    &mut crate::agent::components::IdlePause,
)>()
```
- **Tuple Size:** 7
- **Responsibility:** (Legacy) Moved agents toward targets/waypoints, updated position, transitioned to Arrived state when destination reached, updated hunger, handled pause logic, and performed logging.
- **Notes:**
    - This query was close to the Legion 8-tuple limit.
    - System was responsible for multiple concerns: movement, state transitions, hunger updates, pause logic, and logging.
    - Refactored to split responsibilities into smaller, focused systems.

---

## 2. agent_movement_system (current)

**File:** src/ecs/systems/agent.rs

**Query:**
```rust
.with_query::<(
    &mut crate::ecs_components::Position,
    &crate::agent::AgentType,
    Option<&mut Target>,
    Option<&mut Path>,
    &mut crate::agent::AgentState,
    &mut crate::agent::components::IdlePause,
)>()
```
- **Tuple Size:** 6
- **Responsibility:** Only updates agent positions along path/waypoints or directly toward target. No state transitions, logging, or hunger logic.
- **Status:** ✅ Done
- **Notes:** Minimal, single-responsibility. TODO: Consider decoupling movement by agent type for further modularity.

---

## 3. agent_state_transition_system (new)

**File:** src/ecs/systems/agent.rs

**Query:**
```rust
.with_query::<(
    &mut crate::ecs_components::Position,
    Option<&Target>,
    &mut crate::agent::AgentState,
)>()
```
- **Tuple Size:** 3
- **Responsibility:** Sets `AgentState::Arrived` when agent position matches target. No logging or pausing handled here.

---

## 4. agent_movement_history_system

**File:** ecs/systems/agent.rs

**Query:**
```rust
.with_query::<(Entity, &crate::ecs_components::Position, &mut crate::agent::components::MovementHistory), ()>()
```
- **Tuple Size:** 3
- **Responsibility:** Records each agent’s recent positions for analytics, debugging, or visualization.
- **Status:** ✅ Done
- **Notes:** Minimal, single-responsibility. Moved from agent/systems.rs for modularity.

---

## 5. agent_death_system

**File:** ecs/systems/agent.rs

**Query:**
```rust
.with_query::<(Entity, &crate::agent::Hunger, &crate::agent::Energy), ()>()
```
- **Tuple Size:** 3
- **Responsibility:** Removes agents whose hunger or energy reaches zero, ensuring proper cleanup.
- **Status:** ✅ Done
- **Notes:** Minimal, single-responsibility. No refactor needed.

---

## 6. swimming_system

**File:** ecs/systems/swimming.rs

**Query:**
```rust
.with_query(<(Entity, &mut Position, &mut Hunger, &mut AgentState, &mut SwimmingProfile, &AgentType)>::query())
```
- **Tuple Size:** 6
- **Responsibility:** Handles swimming behavior for agents with a swimming profile, including movement through water and swim duration.
- **Status:** ✅ Done
- **Notes:** Minimal, single-responsibility. Moved from agent/swimming.rs for ECS modularity.

---

## 7. Removal of Legacy agent_arrival_system

**File:** src/agent/systems.rs (REMOVED)

- The legacy agent_arrival_system was removed. Its responsibilities (pausing, event logging, and state transitions on arrival) are now handled by:
    - agent_state_transition_system (state transitions)
    - agent_arrival_logging_system (event logging)
    - agent_pause_system (pausing, if applicable)
- The new agent_state_transition_system is registered in the ECS schedule after agent movement.
- This improves clarity, avoids duplication, and ensures each system has a single responsibility.

---

## 8. path_following_system (updated)

**File:** src/agent/systems.rs

**Query:**
```rust
.with_query::<(
    &mut crate::ecs_components::Position,
    &crate::agent::AgentType,
    &mut crate::agent::Hunger,
    &mut crate::agent::Energy,
    Option<&mut Target>,
    Option<&mut Path>,
    &mut crate::agent::AgentState,
    &mut crate::agent::components::IdlePause,
)>()
```
- **Tuple Size:** 8
- **Responsibility:** Path following and related logic. No state transitions or resets; only checks and logs.

---

**Summary:**
- All state transition logic is now handled by a dedicated system (`agent_state_transition_system`).
- Other systems are now focused on a single responsibility (movement, pausing, logging, etc.), reducing query size and improving maintainability.

*Continue auditing other systems one by one as per plan.*
