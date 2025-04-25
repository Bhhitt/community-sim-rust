pub mod components;
pub mod systems;
pub mod mlp;
pub mod swimming;
pub mod event;
pub mod event_log_bridge;
pub mod pause_system;

// Public API: only export what is needed outside the agent module
pub use components::{AgentType, Hunger, Energy, MovementProfile, MovementEffect, agent_state::AgentState, MovementHistory, DecisionEngineConfig, InteractionState, RecentInteraction};
pub use systems::{agent_action_selection_system, agent_death_system};
pub use mlp::{MLP, MLPConfig};

// TODO: Remove unresolved imports for missing systems
// use systems::path_following_system;
// use systems::agent_movement_history_system;
