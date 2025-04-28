# Plan RF4: Refactor Agent Spawn Queue to ECS-Native

## Motivation
- Eliminate global mutable state (`AGENT_SPAWN_QUEUE`) and static queues.
- Make agent spawning fully ECS-idiomatic, safer, and more testable.
- Remove borrow conflicts and reliance on flush boundaries for global queue access.

## Steps

1. **Remove the Global Static Queue**
    - Delete `AGENT_SPAWN_QUEUE` and its uses (including its `Mutex` and `Arc`).
    - All agent spawn requests should go through ECS resources, not global statics.

2. **Use an ECS Resource for Pending Spawns**
    - Use `PendingAgentSpawns` as the authoritative queue for all pending agent spawns.
    - Register `PendingAgentSpawns` as a resource at simulation startup (if not already).

3. **Refactor External Event Producers**
    - Instead of pushing to a global queue, external producers (input handlers, GUI events) should:
      - Use an ECS system to mutably borrow `PendingAgentSpawns` and push requests directly.
      - If input is outside ECS, use a thread-safe channel to send requests, then drain into `PendingAgentSpawns` at the start of each tick.

4. **Input Handling in ECS**
    - Refactor input systems to run as ECS systems that mutably borrow `PendingAgentSpawns` and push requests directly.
    - If input cannot be handled as an ECS system, provide a safe API to enqueue into the ECS resource (e.g., via a channel, then drain into `PendingAgentSpawns`).

5. **Remove `drain_agent_spawn_queue_system`**
    - If all spawn requests are now routed through ECS resources, remove `drain_agent_spawn_queue_system`.
    - The `agent_spawning_system` can now be the only system that mutably borrows and processes `PendingAgentSpawns`.

6. **Update Schedule and Tests**
    - Remove flush boundaries and the system for draining the global queue.
    - Ensure only one system mutably borrows and processes `PendingAgentSpawns` per tick.

## Notes
- This approach ensures you can still spawn agents at the press of a button, but in a more idiomatic and robust way.
- If input comes from outside ECS, use a channel and a draining system at the start of each tick.
- The result is a cleaner, safer, and more idiomatic ECS codebase.

---

**[COMPLETION NOTE — 2025-04-27]**
All steps in this plan have been completed. Agent spawning is now fully ECS-idiomatic, with `PendingAgentSpawns` as the only authoritative queue. No global static queues remain. Only one system mutably borrows and processes `PendingAgentSpawns` per tick, and the schedule is clean and safe. See project memories and commit history for details.
