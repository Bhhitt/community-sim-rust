// Re-export all ECS systems here for easy use in schedule.rs and elsewhere
// Example:
mod agent;
pub mod food;
pub mod terrain;
pub mod agent_logging;
pub mod swimming;
pub mod agent_action_decision;
pub mod agent_target_assignment;
pub mod agent_path_assignment;
pub mod agent_state_transition;
// Add more as you migrate systems

pub use agent::agent_state_transition_system;
pub use agent_action_decision::agent_action_decision_system;
pub use agent_target_assignment::agent_target_assignment_system;
pub use agent_path_assignment::agent_path_assignment_system;
// pub use agent_state_transition::agent_state_transition_system;

// TODO: Create one file per system (agent.rs, food.rs, etc.) and move system logic from ecs_sim.rs, ecs_simulation.rs, etc.
