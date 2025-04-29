# Implementation Plan: Generalizing the Agent Event System for Extensible Event-Driven Behaviors

## Objective
Refactor the agent event system to support multiple event types (e.g., arrivals, interactions, needs, etc.) using an enum or trait-based approach. This enables extensible, modular, and decoupled event-driven agent behaviors in the ECS framework.

---

## Step-by-Step Instructions

### 1. Define a Generalized AgentEvent Enum
- Create an `enum AgentEvent` with variants for all event types you want to support, e.g.:
  ```rust
  pub enum AgentEvent {
      Arrived { entity: Entity, position: Position },
      InteractionStarted { initiator: Entity, target: Entity },
      InteractionEnded { initiator: Entity, target: Entity },
      // Add more as needed
  }
  ```

### 2. Refactor the Event Queue Resource
- Change `AgentEventQueue` to:
  ```rust
  pub struct AgentEventQueue(pub Vec<AgentEvent>);
  ```
- Update resource registration to use the new type.

### 3. Update Event Emission
- Refactor systems (e.g., state transition, interaction) to emit the appropriate `AgentEvent` variant:
  ```rust
  events.0.push(AgentEvent::Arrived { entity, position: pos.clone() });
  // For interaction:
  events.0.push(AgentEvent::InteractionStarted { initiator, target });
  ```

### 4. Update Event Handling Systems
- Refactor event handling systems to match on `AgentEvent` and process each variant accordingly:
  ```rust
  for event in events.0.iter() {
      match event {
          AgentEvent::Arrived { entity, position } => { /* handle arrival */ },
          AgentEvent::InteractionStarted { initiator, target } => { /* handle start */ },
          AgentEvent::InteractionEnded { initiator, target } => { /* handle end */ },
      }
  }
  ```

### 5. Add/Refactor Debug Logging
- Ensure all event emission and handling sites include clear debug logs for traceability.

### 6. Test Existing and New Event Flows
- Run simulation and verify that all event types are emitted and handled as expected.
- Add or update tests to cover new event types and behaviors.

### 7. (Optional) Further Extensibility
- Consider using traits for event payloads if you need dynamic dispatch or more complex event data.
- Document the event system for future contributors.

---

## Benefits
- Enables modular, extensible, and decoupled agent behaviors.
- Makes it easy to add new event-driven features (e.g., richer interactions, needs, etc.).
- Improves debugging and maintainability.

---

## Next Steps
- Once generalized, proceed to implement agent-agent interaction events and systems as planned in your interaction design doc.
