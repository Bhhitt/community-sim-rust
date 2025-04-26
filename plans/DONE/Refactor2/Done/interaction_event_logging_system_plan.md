# Interaction Event Logging System Refactor Plan

## Objective
Extract all logic related to logging interaction events from the monolithic entity_interaction_system into a dedicated Interaction Event Logging System.

## Steps
1. **Audit Current Logic**
    - Identify all code that pushes agent-agent and agent-food events to the event log in entity_interaction_system.
2. **Design System Interface**
    - Specify required resources (EventLog, AgentEventLog).
3. **Implement System**
    - Write a Legion system that:
        - Logs relevant interaction events (agent-agent, agent-food) to the event log resources.
4. **Register System**
    - Add the new system to the appropriate schedule builder.
5. **Test and Validate**
    - Ensure event logging works as intended and is not duplicated by other systems.
6. **Document**
    - Update documentation and audit files to reflect the new system.

## Status
**[x] Refactored, implemented, and registered in ECS schedule.**
