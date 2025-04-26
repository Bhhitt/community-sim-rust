# Audit: agent_movement_system (Refactor2)

## Current Query
```rust
.with_query::<(
    &mut crate::ecs_components::Position,
    &crate::agent::AgentType,
    Option<&mut crate::agent::components::Target>,
    Option<&mut crate::agent::components::Path>,
    &mut crate::agent::AgentState,
    &mut crate::agent::components::IdlePause,
), ()>(())
```
- **Tuple size:** 6 (Position, AgentType, Target, Path, AgentState, IdlePause)
- **Notes:** Large tuple, includes optional Target/Path, and both state and pause logic.

## Responsibilities Identified

### 1. Path Movement
- If agent has a path (`Some(path)` and `!path.waypoints.is_empty()`):
  - Move agent toward the next waypoint (`tx, ty`).
  - Update `pos.x`, `pos.y` by a step determined by `agent_type.movement_profile.speed`.
  - Remove waypoint when reached.

### 2. Direct Target Movement
- If agent has a target but no path:
  - Move agent directly toward `target.x`, `target.y` by a step (`speed`).
  - Snap to target if within threshold.

### 3. State Transitions
- Comments indicate state transitions (e.g., to Arrived) are now handled by a separate system, not here.

### 4. Pausing
- Pausing logic has been removed from this system and is handled by `agent_pausing_system`.

## Components Actually Used
- **Position** (mut): always updated
- **AgentType**: used for speed profile
- **Target** (optional, mut): used for both direct and path movement
- **Path** (optional, mut): used for path movement
- **AgentState** (mut): only read to check Idle/Moving (could be delegated)
- **IdlePause** (mut): not used in this system (can be removed from query)

## Potential for Decomposition
- **Path movement** and **direct movement** can be split into two systems:
  - Path movement: requires Position, Path, AgentType
  - Direct movement: requires Position, Target, AgentType
- **State checking** can be moved to a dedicated state transition system.
- **IdlePause** is not needed in the movement system.

## Next Steps
- Remove `IdlePause` from this system's query.
- Split into two systems: one for path movement, one for direct movement.
- Move any state logic to a dedicated system if not already done.
- Ensure all systems are registered in the correct order.

---

*This audit documents the responsibilities, components, and decomposition potential for `agent_movement_system` as part of Refactor2.*
