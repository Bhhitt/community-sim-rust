# ECS Design for Extensible Agent-Agent Interactions

## 1. Components

### a. `InteractionIntent`
Tracks an agent's intent to interact and pursuit status.
```rust
pub struct InteractionIntent {
    pub target: Entity,
    pub ticks_pursued: u32,
    pub max_pursue_ticks: u32, // e.g., 50
}
```

### b. `InteractionQueue`
Queue of agents waiting to interact with a target agent.
```rust
pub struct InteractionQueue {
    pub queue: VecDeque<Entity>,
}
```

### c. `Interacting`
Marks both agents as currently interacting.
```rust
pub struct Interacting {
    pub partner: Entity,
    pub ticks_remaining: u32,
}
```

---

## 2. Systems

- **Intent Assignment System:**
  - (Automatic for now) Each agent checks for nearby agents and, if not busy, creates an `InteractionIntent` toward the nearest available agent.
- **Pursuit/Movement System:**
  - Agents with `InteractionIntent` move toward their target; abandon after `max_pursue_ticks` if not successful.
- **Interaction Range System:**
  - When in range, add to target's `InteractionQueue`. If target is free, start interaction; otherwise, wait in queue.
- **Interaction Duration System:**
  - Decrement `ticks_remaining` for all `Interacting` agents. When done, remove `Interacting`, start next in queue if present.
- **Idle/Decision System:**
  - Agents that lose their target or finish interaction go idle and pick a new activity.

---

## 3. Extensibility

- Add interaction types with an enum or trait:
```rust
pub enum InteractionKind { Trade, Chat, Fight, ... }
// Extend InteractionIntent and Interacting as needed
```
- Queue/accept/decline logic can be added later for richer behaviors.

---

## 4. Edge Cases Handled
- Multiple agents can queue for one target.
- Agents are “locked” during interaction.
- Pursuers give up after N ticks if target moves away.
- If interrupted, agents go idle and can pick a new activity.

---

## 5. Summary Table

| Component           | On         | Purpose                                              |
|---------------------|------------|------------------------------------------------------|
| InteractionIntent   | Initiator  | Track who/what agent is pursuing and for how long    |
| InteractionQueue    | Target     | Queue of agents waiting to interact                  |
| Interacting         | Both       | Mark agents as locked in interaction, with duration  |

---

## Next Steps
- Confirm this design fits your goals.
- Implement components and systems.
- Write integration tests for queueing, pursuit, locking, and edge cases.
