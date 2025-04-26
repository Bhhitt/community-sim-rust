# Agent-Agent Interaction System Refactor Plan

## Objective
Extract all logic related to agent-agent interactions from the monolithic entity_interaction_system into a dedicated Agent-Agent Interaction System.

## Steps
1. **Audit Current Logic**
    - Identify code handling agent-agent proximity and interaction events in entity_interaction_system.
2. **Design System Interface**
    - Define queries for agents (positions, states).
    - Specify required resources (e.g., InteractionStats, EventLog).
3. **Implement System**
    - Write a Legion system that:
        - Detects agents within interaction range.
        - Updates InteractionStats and logs events to EventLog.
4. **Register System**
    - Add the new system to the appropriate schedule builder (e.g., interaction schedule).
5. **Test and Validate**
    - Ensure agent-agent interactions work as intended and do not interfere with other systems.
6. **Document**
    - Update documentation and audit files to reflect the new system.

## Status
**[x] Refactored and registered in ECS schedule.**
