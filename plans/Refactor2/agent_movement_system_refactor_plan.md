# Refactor Plan: `agent_movement_system` (Refactor2)

## Context
The current `agent_movement_system` in `src/ecs/systems/agent.rs` is responsible for:
- Moving agents along a path or directly toward a target
- Handling both position updates and some state logic
- Using a large ECS query with six component types (Position, AgentType, Target, Path, AgentState, IdlePause)

This violates ECS best practices and exceeds Legion's recommended tuple size for queries. The system is not minimal and mixes several responsibilities.

## Refactor Goals
- **Split responsibilities**: Each system should do one thing (Single Responsibility Principle)
- **Reduce query tuple size**: Avoid Legion's tuple limit and improve clarity
- **Increase modularity and testability**: Smaller, focused systems are easier to understand and test
- **Lay groundwork for future agent behavior enhancements**

## Step-by-Step Refactor Plan

### 1. Remove Unused Components
- [ ] Remove `IdlePause` from the query in `agent_movement_system` (it is not used in the logic).

### 2. Split Movement Logic
- [ ] Create `agent_path_movement_system`:
    - Moves agent along waypoints if `Path` is present and not empty.
    - Query: Position (mut), AgentType, Path (mut), Target (optional, mut)
    - Handles only path/waypoint movement.
- [ ] Create `agent_direct_movement_system`:
    - Moves agent directly toward target if no path is present.
    - Query: Position (mut), AgentType, Target (mut)
    - Handles only direct movement to target.

### 3. Delegate State Logic
- [ ] Ensure all state transitions (e.g., to Arrived) are handled in `agent_state_transition_system`.
    - Query: AgentState (mut), Position, Target
    - Handles state changes based on position/target matching.

### 4. Verify Pausing Logic
- [ ] Confirm that all pausing logic is handled in `agent_pausing_system` and is not duplicated.

### 5. Clean Up Queries
- [ ] After splitting, ensure each system's query only requests the components it actually needs.

### 6. Update System Registration
- [ ] Register new systems in ECS schedule in the correct order (path movement, then direct movement, then state transition, then pausing).
- [ ] Remove or deprecate the original `agent_movement_system`.

### 7. Testing & Validation
- [ ] Add/update tests for each new system.
- [ ] Validate agent movement and state transitions in simulation.

---

*This plan is informed by the audit in `agent_movement_system_audit.md`. Refactor2 will deliver a modular, maintainable, and ECS-best-practices-compliant agent movement pipeline.*
