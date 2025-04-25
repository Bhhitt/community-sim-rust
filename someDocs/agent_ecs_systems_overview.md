# Agent ECS Systems Overview

This document provides an overview of the core and supporting agent-related ECS systems in the simulation. Each system is described with its primary responsibilities and role in the agent lifecycle.

---

## Core Agent Systems

### 1. spawn_agent
Handles the creation and initialization of new agents. Sets up all necessary components (position, agent type, hunger, etc.), assigns random or specified starting properties, and logs the spawn event for tracking and debugging.

### 2. agent_movement_system
Moves agents toward their assigned targets or along precomputed paths. Updates the agent's position each tick, handles movement speed, and transitions the agent's state to `Arrived` when the destination is reached. Skips movement if the agent is currently paused (IdlePause > 0).

### 3. agent_arrival_system
Detects when agents have reached their destination. When arrival is detected, sets an IdlePause (a pause duration before the agent can act again) and transitions the agent's state to `Idle`. This helps simulate realistic pauses between actions.

### 4. path_following_system
Manages path traversal for agents with a path assigned. Ensures agents follow their path waypoints and resets their state to `Idle` after reaching the end of the path. Also clears path data as needed.

### 5. agent_action_selection_system
Determines what action an agent should take next based on its state and environment. Assigns new targets or paths (e.g., to food or a random location for wandering), and transitions the agent to the `Moving` state. This system is responsible for initiating purposeful or idle movement.

### 6. passive_hunger_system
Updates agent hunger values based on their activity. Agents lose hunger more slowly when idle or paused, and at a normal rate when moving. If hunger drops below a threshold, the agent may switch to seeking food.

### 7. agent_movement_history_system
Records each agent's recent positions in a movement history buffer. This can be used for analytics, debugging, or visualizing agent paths over time.

### 8. agent_death_system
Removes agents from the simulation when their hunger or energy reaches zero. Ensures that dead agents are properly cleaned up and do not participate in further simulation steps.

---

## Additional/Supporting Systems

### 9. agent_pause_system
Decrements the IdlePause timer for all agents each tick. When the timer reaches zero, the agent is unpaused and can act again. Logs pause state changes for debugging.

### 10. swimming_system
Handles swimming behavior for agents with a swimming profile. Moves agents through water tiles, decrements swim duration, and updates hunger accordingly. Only agents in the `Swimming` state can traverse water.

### 11. agent_event_log_to_gui_system
Bridges the agent event log to the GUI event log for display. Transfers recent agent events (spawns, moves, state changes, etc.) to the main event log for visualization in the user interface.

---

## Notes
- Each system is designed for a specific aspect of agent behavior or lifecycle.
- The IdlePause logic is managed both in the movement/arrival systems and in a dedicated pause system.
- System execution order is important for correct agent behavior (e.g., pause system should run before movement/action selection).

---

## Agent ECS Systems Execution Flow Diagram

Below is a diagram showing the typical execution order and data flow between the main agent ECS systems each simulation tick:

```mermaid
flowchart TD
    subgraph Core
        PAUSE[agent_pause_system](Decrement IdlePause)
        ACTION[agent_action_selection_system](Decide actions, assign targets)
        PATH[path_following_system](Manage paths, reset state)
        MOVE[agent_movement_system](Move agents, update positions)
        ARRIVE[agent_arrival_system](Handle arrival, set IdlePause)
        HUNGER[passive_hunger_system](Update hunger)
        HISTORY[agent_movement_history_system](Record positions)
        DEATH[agent_death_system](Remove dead agents)
    end
    subgraph Supporting
        SWIM[swimming_system](Handle swimming)
        LOG[agent_event_log_to_gui_system](Bridge event logs)
    end
    PAUSE --> ACTION
    ACTION --> PATH
    PATH --> MOVE
    MOVE --> ARRIVE
    ARRIVE --> PAUSE
    MOVE --> HUNGER
    MOVE --> HISTORY
    HUNGER --> DEATH
    SWIM --> MOVE
    LOG --> ARRIVE
    LOG --> MOVE
```

**Legend:**
- Arrows indicate primary data or state flow between systems.
- Some systems (like `swimming_system` and `agent_event_log_to_gui_system`) run in parallel or asynchronously to the main agent lifecycle.
- The actual ECS schedule may include other systems (e.g., rendering, environment updates) not shown here.

---
