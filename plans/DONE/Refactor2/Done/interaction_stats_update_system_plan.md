# Interaction Stats Update System Refactor Plan

## Objective
Extract all logic related to updating interaction statistics from the monolithic entity_interaction_system into a dedicated Interaction Stats Update System.

## Steps
1. **Audit Current Logic**
    - Identify all code updating InteractionStats and its history in entity_interaction_system.
2. **Design System Interface**
    - Specify required resources (InteractionStats).
3. **Implement System**
    - Write a Legion system that:
        - Updates agent_interactions, active_interactions, and active_interactions_history.
        - Ensures history buffer size is managed correctly.
4. **Register System**
    - Add the new system to the appropriate schedule builder.
5. **Test and Validate**
    - Ensure stats are updated correctly and reflect simulation state.
6. **Document**
    - Update documentation and audit files to reflect the new system.

## Status
**[x] Refactored, implemented, and registered in ECS schedule.**
