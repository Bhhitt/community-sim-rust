# Implementation Plan: Pursuit/Movement System

## Purpose
Moves agents with an `InteractionIntent` toward their target agent. If the agent cannot reach the target within a maximum number of ticks (`max_pursue_ticks`), the intent is removed and the agent goes idle.

---

## 1. System Inputs & Outputs
- **Inputs:**
  - Agents/entities with an `InteractionIntent` component
  - Positions of all agents/entities
- **Outputs:**
  - Updates agent positions to move them toward their target
  - Increments `ticks_pursued` in `InteractionIntent`
  - Removes `InteractionIntent` if `ticks_pursued >= max_pursue_ticks` or target disappears

---

## 2. ECS Query Design
- Query for all agents/entities with:
  - `InteractionIntent`
  - `Position`
- For each such agent:
  - Get the target entity from `InteractionIntent`
  - If target exists and has a `Position`, move toward it
  - Increment `ticks_pursued`
  - If `ticks_pursued >= max_pursue_ticks`, remove `InteractionIntent` (agent goes idle)
  - If target entity no longer exists or is unreachable, remove `InteractionIntent`

---

## 3. System Logic
1. For each agent with `InteractionIntent`:
    - Look up the target's position
    - If target exists:
        - Move agent's position a step toward the target (use simple linear movement or pathfinding as appropriate)
        - Increment `ticks_pursued`
        - If `ticks_pursued >= max_pursue_ticks`, remove `InteractionIntent`
    - If target does not exist:
        - Remove `InteractionIntent`
2. (Optional) Log pursuit events for debugging

---

## 4. Extensibility
- Support more sophisticated movement (e.g., pathfinding, obstacles)
- Allow for different pursuit policies (e.g., group pursuit, avoidance)
- Integrate with other movement systems if needed

---

## 5. Pseudocode Example
```rust
SystemBuilder::new("PursuitMovementSystem")
    .with_query(<(Entity, &mut Position, &mut InteractionIntent)>::query())
    .build(|cmd, world, _, query| {
        for (entity, pos, intent) in query.iter_mut(world) {
            if let Ok(target_pos) = world.get_component::<Position>(intent.target) {
                // Move agent toward target (simple step)
                let dx = target_pos.x - pos.x;
                let dy = target_pos.y - pos.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > 0.01 {
                    let step = 1.0_f32.min(dist); // step size
                    pos.x += dx / dist * step;
                    pos.y += dy / dist * step;
                }
                intent.ticks_pursued += 1;
                if intent.ticks_pursued >= intent.max_pursue_ticks {
                    cmd.remove_component::<InteractionIntent>(entity);
                }
            } else {
                // Target gone
                cmd.remove_component::<InteractionIntent>(entity);
            }
        }
    })
```

---

## 6. Checklist for Completion
- [ ] System moves agents toward their targets
- [ ] System increments `ticks_pursued` and removes intent if needed
- [ ] System handles missing targets gracefully
- [ ] System is tested with unit/integration tests
- [ ] System is documented in code and planning docs
