# Plan: Event-Driven Agent State Transitions in ECS

## Objective
Refactor agent state transitions to use an event-driven approach, so that agents emit events (e.g., on arrival) and other systems respond by updating state or triggering new behaviors. This decouples state changes and enables extensible agent interactions.

---

## Steps

### 1. Define Event Type
- Create a struct for arrival events (and future agent events):
  ```rust
  pub struct AgentArrivedEvent {
      pub entity: legion::Entity,
      pub position: Position,
      // Add more context as needed
  }
  ```

### 2. Add Event Resource
- Add a resource to the ECS world to store events each tick:
  ```rust
  pub struct AgentEventQueue(pub Vec<AgentArrivedEvent>);
  ```
- Insert an empty `AgentEventQueue` at world/resource initialization.

### 3. Emit Events in State Transition System
- In `agent_state_transition_system`, when an agent reaches its target:
  - Push an `AgentArrivedEvent` into the queue instead of (or in addition to) setting state directly.
  - Example:
    ```rust
    events.0.push(AgentArrivedEvent { entity, position: pos.clone() });
    ```

### 4. Handle Events in a Dedicated System
- Create a new system, e.g., `agent_arrival_event_system`, that:
  - Iterates over the event queue
  - Sets the agent's state to `Idle` (or triggers other behaviors)
  - Optionally triggers further events or side effects

### 5. Clear Events Each Tick
- After processing, clear the event queue:
  ```rust
  events.0.clear();
  ```

### 6. (Optional) Generalize for More Event Types
- If you want to handle more agent events (e.g., interactions, needs met), define an enum or trait for agent events and use a single event queue.

### 7. Add Debug Logging
- Add debug logs for event emission and handling to aid in debugging and tracing agent behavior.

---

## Benefits
- Decouples state transitions from movement/path logic
- Makes it easy to add new reactions to agent events
- Improves extensibility for future agent behaviors

---

## Estimated Effort
- Minimal event system: 1-2 hours
- More robust/abstract event bus: 3-6 hours

---

## Next Steps
1. Review this plan and adapt to your ECS codebase structure
2. Implement steps 1-5 for arrival events
3. Test and iterate on the event-driven design

---

*Created by Cascade AI — 2025-04-29*
