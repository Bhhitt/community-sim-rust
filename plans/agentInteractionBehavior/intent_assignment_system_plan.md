# Implementation Plan: Intent Assignment System

## Purpose
Automatically assigns an `InteractionIntent` to agents that are idle and detect a valid target agent nearby. This is the first step in the agent-agent interaction pipeline.

---

## 1. System Inputs & Outputs
- **Inputs:**
  - Agents/entities with no `InteractionIntent` and no `Interacting` component (i.e., idle/available agents)
  - List of other agents/entities in the ECS world
  - (Optional) Agent vision/range, agent state, or other filters
- **Outputs:**
  - For each eligible agent, attach an `InteractionIntent` targeting a valid agent (e.g., nearest available agent)

---

## 2. ECS Query Design
- Query for all agents/entities that:
  - Do **not** have `InteractionIntent`
  - Do **not** have `Interacting`
  - (Optional) Are not already in someone else’s queue
- For each such agent:
  - Find candidate targets (other agents that are not themselves, not currently interacting, and not already being pursued by this agent)
  - Select a target (e.g., nearest agent, random agent, etc.)
  - Attach an `InteractionIntent` to the agent with `target`, `ticks_pursued = 0`, `max_pursue_ticks = 50`

---

## 3. System Logic
1. For each idle agent:
    - Gather all possible targets (other agents who are not self, not interacting, not already being pursued by this agent)
    - If no targets, skip
    - Else, select a target (nearest, random, or by some policy)
    - Insert `InteractionIntent` component with appropriate fields
2. (Optional) Log or emit an event for debugging or analytics

---

## 4. Extensibility
- In the future, support richer selection logic (e.g., filter by agent type, state, or preferences)
- Allow for explicit intent (agent chooses to interact based on internal logic, not just proximity)
- Add support for interaction types (trade, chat, etc.) via an enum in `InteractionIntent`

---

## 5. Pseudocode Example
```rust
// Legion system builder pseudocode
SystemBuilder::new("IntentAssignmentSystem")
    .with_query(<(Entity, &Position)>::query())
    .build(|cmd, world, _, query| {
        for (entity, pos) in query.iter(world) {
            if world.get_component::<InteractionIntent>(entity).is_none()
                && world.get_component::<Interacting>(entity).is_none()
            {
                // Find candidate targets
                let candidates = ...;
                if let Some(target) = select_target(candidates) {
                    cmd.add_component(
                        entity,
                        InteractionIntent {
                            target,
                            ticks_pursued: 0,
                            max_pursue_ticks: 50,
                        },
                    );
                }
            }
        }
    })
```

---

## 6. Checklist for Completion
- [ ] System queries only eligible (idle) agents
- [ ] System selects valid targets based on policy
- [ ] System inserts `InteractionIntent` with correct fields
- [ ] System is tested with unit/integration tests
- [ ] System is documented in code and planning docs
