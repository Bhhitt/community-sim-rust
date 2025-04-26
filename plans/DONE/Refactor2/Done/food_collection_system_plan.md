# Food Collection System Refactor Plan

## Objective
Extract all logic related to agent-food interactions from the monolithic entity_interaction_system into a dedicated Food Collection System.

## Steps
1. **Audit Current Logic**
    - Identify all code handling agent-food proximity, food removal, and nutrition gain in entity_interaction_system.
2. **Design System Interface**
    - Define queries for agents and food entities.
    - Specify required resources (e.g., FoodStats, AgentEventLog).
3. **Implement System**
    - Write a Legion system that:
        - Detects agents within range of food.
        - Randomizes food selection if multiple are in range.
        - Removes eaten food and updates FoodStats.
        - Logs agent-food events to AgentEventLog.
4. **Register System**
    - Add the new system to the appropriate schedule builder (e.g., interaction or food schedule).
5. **Test and Validate**
    - Ensure food collection works as intended and does not interfere with other systems.
6. **Document**
    - Update documentation and audit files to reflect the new system.

## Status
**[x] Refactored and registered in ECS schedule.**
