# Agent Systems Audit

This table lists all agent-related ECS systems, with a brief description, file location, resource/component borrows, and signature. This audit helps identify systems that could be involved in ECS borrow conflicts or the AccessDenied panic.

| System Name | Description | File | Borrows (Resource/Component, Mut/Immut) | Signature | Notes |
|-------------|-------------|------|-----------------------------------------|-----------|-------|
| agent_spawning_system | Consumes PendingAgentSpawns and creates new agent entities in the ECS world. | src/ecs/systems/agent_spawn.rs | PendingAgentSpawns (write, resource), AgentEventLog (write, resource), Map (read, resource) | `pub fn agent_spawning_system() -> impl Runnable` | Mutably borrows PendingAgentSpawns and AgentEventLog. |
| agent_path_assignment_system | Assigns paths to agents for navigation. | src/ecs/systems/agent_path_assignment.rs | Map (read, resource) | `pub fn agent_path_assignment_system() -> impl Runnable` | Only borrows Map as ECS resource (read). |
| agent_path_movement_system | Removes the first waypoint from the agent's path if present (does not update position). | src/ecs/systems/agent.rs | Path (mut, component) | `pub fn agent_path_movement_system() -> impl Runnable` | No ECS resource borrows. |
| agent_direct_movement_system | Moves agent directly toward target if no path is present. | src/ecs/systems/agent.rs | Position (mut, component), AgentType (immut, component), Path (immut, component), Target (immut, component), AgentState (immut, component) | `pub fn agent_direct_movement_system() -> impl Runnable` | No ECS resource borrows. |
| agent_state_transition_system | Sets AgentState::Arrived when agent position matches target. | src/ecs/systems/agent.rs | Position (mut, component), Target (immut, component), AgentState (mut, component), Path (immut, component) | `pub fn agent_state_transition_system() -> impl Runnable` | No ECS resource borrows. |
| agent_pausing_system | Handles IdlePause logic (decrementing ticks_remaining). | src/ecs/systems/agent.rs | IdlePause (mut, component) | `pub fn agent_pausing_system() -> impl Runnable` | No ECS resource borrows. |
| agent_movement_history_system | Records each agent's recent positions for analytics/debugging. | src/ecs/systems/agent.rs | Position (immut, component), MovementHistory (mut, component) | `pub fn agent_movement_history_system() -> impl Runnable` | No ECS resource borrows. |
| agent_hunger_energy_system | Manages hunger and energy levels for agents. | src/ecs/systems/agent.rs | AgentType (immut, component), Hunger (mut, component), Energy (mut, component), AgentState (immut, component) | `pub fn agent_hunger_energy_system() -> impl Runnable` | No ECS resource borrows. |
| agent_spawn_log_system | Logs the number of agents in the world after spawning. | src/ecs/systems/agent_spawn_log.rs | Map (read, resource) | `pub fn agent_spawn_log_system() -> impl Runnable` | Only borrows Map as ECS resource (read). |
| agent_arrival_logging_system | Logs agent arrival events. | src/ecs/systems/agent_logging.rs | EventLog (write, via Arc<Mutex<_>>), LogConfig (read) | `pub fn agent_arrival_logging_system() -> impl Runnable` | Uses Arc<Mutex> for logging, not ECS resource. |
| agent_move_logging_system | Logs agent movement events. | src/ecs/systems/agent_logging.rs | EventLog (write, via Arc<Mutex<_>>), LogConfig (read) | `pub fn agent_move_logging_system() -> impl Runnable` | Uses Arc<Mutex> for logging, not ECS resource. |
| agent_spawn_logging_system | Logs agent spawn events. | src/ecs/systems/agent_logging.rs | EventLog (write, via Arc<Mutex<_>>), LogConfig (read) | `pub fn agent_spawn_logging_system() -> impl Runnable` | Uses Arc<Mutex> for logging, not ECS resource. |

---

**Next Steps:**
- Pay special attention to any system that borrows `PendingAgentSpawns`, `AgentEventLog`, or other shared ECS resources, especially as write (mutable) borrows.
- Use this table to cross-reference with the ECS schedule and identify possible borrow conflicts.
