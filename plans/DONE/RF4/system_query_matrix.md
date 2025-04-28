# ECS System Query Matrix

This document lists all ECS systems and their query types, indicating what query syntax is required for each (tuple/simple, optional, or advanced/builder).

---

## agent.rs

### agent_path_movement_system
- **Query:** `<(&mut Position, &AgentType, &mut Path, &mut Target, &mut AgentState)>::query()`
- **Query Type:** Simple tuple (all required, Legion API expects passing a query object)

### agent_direct_movement_system
- **Query:** `<(&mut Position, &AgentType, &mut Path, &mut Target, &mut AgentState)>::query()`
- **Query Type:** Simple tuple (all required, Legion API expects passing a query object)

### agent_state_transition_system
- **Query:** `<(&mut Position, &Target, &mut AgentState, &Path)>::query()`
- **Query Type:** Simple tuple (all required, Legion API expects passing a query object)

### agent_pausing_system
- **Query:** `<(Entity, &mut IdlePause)>::query()`
- **Query Type:** Simple tuple (all required, Legion API expects passing a query object)

### agent_movement_history_system
- **Query:** `<(Entity, &Position, &mut MovementHistory)>::query()`
- **Query Type:** Simple tuple (all required, Legion API expects passing a query object)

### agent_hunger_energy_system
- **Query:** `<(Entity, &AgentType, &mut Hunger, &mut Energy, &AgentState)>::query()`
- **Query Type:** Simple tuple (all required, Legion API expects passing a query object)

---

## agent_action_decision.rs

### agent_action_decision_system
- **Query:** `(Entity, &AgentType, &Hunger, &AgentState)`
- **Query Type:** Simple tuple (all required)

---

## agent_agent_interaction.rs

### agent_agent_interaction_system
- **Query:** `(Entity, &Position, &InteractionState)`
- **Query Type:** Simple tuple (all required)

---

## agent_logging.rs

### agent_arrival_logging_system
- **Query:** `(Entity, &Position, &AgentState)`
- **Query Type:** Simple tuple (all required)

### agent_move_logging_system
- **Query:** `(Entity, &Position, &AgentState)`
- **Query Type:** Simple tuple (all required)

### agent_spawn_logging_system
- **Query:** `(Entity, &Position, &AgentType)`
- **Query Type:** Simple tuple (all required)

---

## agent_path_assignment.rs

### agent_path_assignment_system
- **Query:** `(Entity, &Position, &Target, &AgentType, &AgentState, &mut Option<Path>)`
- **Query Type:** Tuple with optional (`Option<Path>`) component

---

## agent_state_transition.rs

### agent_state_transition_system
- **Query:** `<(&mut Position, Option<&Target>, &mut AgentState)>::query()`
- **Query Type:** Tuple with optional (`Option<&Target>`) component

---

## agent_target_assignment.rs

### agent_target_assignment_system
- **Query:** `(Entity, &IntendedAction, &Position, &AgentType, &mut Option<Target>, &mut AgentState)`
- **Query Type:** Tuple with optional (`Option<Target>`) component

---

## food.rs

### food_interaction_system
- **Query 1:** `(Entity, &Position, &InteractionState)` (for agents)
- **Query 2:** `(Entity, &Position, &Food)` (for food)
- **Query Type:** Simple tuple (all required)

---

## swimming.rs

### swimming_system
- **Query:** `(Entity, &mut Position, &mut Hunger, &mut AgentState, &mut SwimmingProfile, &AgentType)`
- **Query Type:** Simple tuple (all required) *(commented out in code, but included for completeness)*

---

## Summary
- **Simple tuple queries** (all required components): Can use `.with_query::<(...)>().query()`
- **Tuple with Option<>**: Use `.with_query::<(...)>().query()`, but component must be `Option<T>` in storage and you must handle `None` in logic.
- **Advanced filtering** (not present in current code): Would require query builder/combinator syntax.

This matrix should help ensure each system uses the correct query syntax for its needs. Update as systems/components evolve.
