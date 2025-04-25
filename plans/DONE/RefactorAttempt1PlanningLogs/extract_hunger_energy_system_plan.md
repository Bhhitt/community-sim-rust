# Plan: Extract Hunger/Energy Logic into Its Own System (2025-04-25)

## Objective
Move all hunger and energy update logic out of the agent movement/path-following system into a dedicated ECS system for clarity and single responsibility.

## Steps
1. **Audit Existing Hunger/Energy Logic:**
   - Identify all code in `agent_movement_system` (src/ecs/systems/agent.rs) or related systems that updates `Hunger` or `Energy`.
2. **Design Hunger/Energy System:**
   - Create a new system (e.g., `agent_hunger_energy_system`) that:
     - Updates `Hunger` and `Energy` for agents each tick or when appropriate.
     - Handles starvation, exhaustion, or related state transitions if needed.
     - Does not perform movement or pausing logic.
3. **Refactor Movement System:**
   - Remove all hunger/energy updates from `agent_movement_system`.
   - Ensure it only moves agents, delegating all resource management to the new system.
4. **Update ECS Schedule:**
   - Register the new hunger/energy system in the correct order (likely after movement, before state transitions).
5. **Test:**
   - Verify that hunger and energy are updated correctly, and that agent behavior is unaffected by the refactor.

## Discussion
- Should hunger and energy be split into two systems?
- How should starvation/exhaustion be handled (in this system or a dedicated one)?
