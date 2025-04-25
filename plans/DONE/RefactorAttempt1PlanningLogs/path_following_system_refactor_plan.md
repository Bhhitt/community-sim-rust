# Path Following System Refactor Plan (2025-04-25)

## Objective
Refactor the `path_following_system` in `src/agent/systems.rs` to align with ECS best practices:
- Reduce query tuple size to 6 or fewer.
- Ensure each system has a single responsibility (movement, state transitions, hunger/energy, pausing, logging, etc.).
- Improve maintainability and testability.

## Proposed Steps
1. **Audit Current Responsibilities:**
   - Identify exactly what logic is handled in `path_following_system` (movement, state, hunger, etc.).
2. **Extract Movement Logic:**
   - Create a `path_following_movement_system` that only updates agent positions along a path.
3. **Extract State Transitions:**
   - Move any state transition logic (e.g., setting `Arrived`, `Idle`) to a dedicated `agent_state_transition_system` if not already present.
4. **Extract Hunger/Energy Updates:**
   - Move hunger/energy updates to their own system(s) if not already present.
5. **Extract Pausing/Idle Logic:**
   - Move pausing/idle logic to a dedicated system if needed.
6. **Extract Logging/Event Emission:**
   - Ensure all logging/event logic is handled by dedicated systems.
7. **Update ECS Schedule:**
   - Register new/refactored systems in the correct order.
8. **Test and Validate:**
   - Ensure all agent path following, state transitions, and side effects still function as expected.

## Discussion Points
- Should pausing and hunger/energy be handled in the same system, or split further?
- Are there any legacy side effects (logging, events) that need to be audited?
- How should system ordering be managed for correct simulation behavior?

## 2025-04-25: Refactor Complete
- All path following, pausing, hunger/energy, state transitions, and logging responsibilities are now handled by dedicated systems.
- path_following_system has been fully removed from the codebase.
- See audits/path_following_system_audit.md for details.

---

**Review this plan and discuss any changes or additions before proceeding with the refactor.**
