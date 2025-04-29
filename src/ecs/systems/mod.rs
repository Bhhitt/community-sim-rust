// Re-export all ECS systems here for easy use in schedule.rs and elsewhere
// Example:
pub mod agent;
pub mod food;
pub mod terrain;
pub mod agent_logging;
pub mod swimming;
pub mod agent_action_decision;
pub mod agent_target_assignment;
pub mod agent_path_assignment;
pub mod agent_state_transition;
pub mod agent_spawn;
pub mod pending_agent_spawns;
pub mod drain_agent_spawn_queue;
pub mod agent_spawn_log;
pub mod initial_spawn;
pub mod agent_arrival_event;
pub mod interaction_end_event;
// Add more as you migrate systems

// --- Modular interaction systems ---
pub mod agent_agent_interaction;
pub mod interaction_stats_update;
pub mod interaction_event_logging;

// Re-export public ECS systems for schedule.rs and other modules
// pub use agent::agent_state_transition_system; // DEPRECATED: use agent_state_transition_system from agent_state_transition.rs
// pub use agent::agent_movement_system;
pub use agent::agent_pausing_system;
pub use agent::agent_hunger_energy_system;
// If you want to re-export agent_movement_history_system, uncomment below:
// pub use agent::agent_movement_history_system;

// pub use agent_action_decision::agent_action_decision_system;
// pub use agent_target_assignment::agent_target_assignment_system;
// pub use agent_path_assignment::agent_path_assignment_system;
// pub use pending_agent_spawns::{PendingAgentSpawns, AgentSpawnRequest};
// pub use agent_spawn::agent_spawning_system;

// Re-export modular interaction systems for schedule.rs
// pub use agent_agent_interaction::agent_agent_interaction_system;
// pub use interaction_stats_update::interaction_stats_update_system;
// pub use interaction_event_logging::interaction_event_logging_system;

// TODO: Create one file per system (agent.rs, food.rs, etc.) and move system logic from ecs_sim.rs, ecs_simulation.rs, etc.
