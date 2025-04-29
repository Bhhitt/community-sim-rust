# TODO List: Agent-Agent Interaction ECS Implementation

This checklist breaks down the design from `ecs_design.md` into actionable implementation steps.

---

## Components
- [x] Implement `InteractionIntent` component
- [x] Implement `InteractionQueue` component
- [x] Implement `Interacting` component

## Systems
- [x] Intent Assignment System (automatic intent selection for now)
- [ ] Pursuit/Movement System (pursue target, abandon after N ticks)
- [ ] Interaction Range System (queue and start interaction when in range)
- [ ] Interaction Duration System (lock, decrement, unlock, process queue)
- [ ] Idle/Decision System (handle idle and post-interaction logic)

## Extensibility
- [ ] Add `InteractionKind` enum or trait for future interaction types
- [ ] Design for queue/accept/decline logic (future extension)

## Edge Cases & Testing
- [ ] Test: Multiple agents queueing for one target
- [ ] Test: Agents locked during interaction
- [ ] Test: Pursuer gives up after max pursue ticks
- [ ] Test: Agents go idle after interruption
- [ ] Test: Queue is processed in order after each interaction

## Integration
- [ ] Integrate new components and systems into main ECS schedule
- [ ] Update documentation and diagrams as needed

---

**Feel free to add, remove, or reorder items as the implementation progresses!**
