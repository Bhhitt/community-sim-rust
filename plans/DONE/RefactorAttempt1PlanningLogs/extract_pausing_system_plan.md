# Plan: Extract Pausing Logic into Its Own System (2025-04-25)

## Objective
Move all pausing/IdlePause logic out of the agent movement/path-following system into a dedicated ECS system for clarity and single responsibility.

## Steps
1. **Audit Existing Pausing Logic:**
   - Identify all code in `agent_movement_system` (src/ecs/systems/agent.rs) that checks or modifies `IdlePause`.
2. **Design Pausing System:**
   - Create a new system (e.g., `agent_pausing_system`) that:
     - Checks `IdlePause` for each agent.
     - Updates `ticks_remaining`, unpauses agents as needed.
     - Does not perform movement or state transitions.
3. **Refactor Movement System:**
   - Remove all pausing checks from `agent_movement_system`.
   - Ensure it only moves agents that are not paused (i.e., after the pausing system runs).
4. **Update ECS Schedule:**
   - Register the new pausing system before movement-related systems.
5. **Test:**
   - Verify that agents pause and unpause correctly, and that movement is unaffected by the refactor.

## Discussion
- Should pausing logic be further split (e.g., handle different pause types separately)?
- How should system ordering be managed for correct pause/movement behavior?
