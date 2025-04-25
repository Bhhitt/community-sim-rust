pub mod components;
pub mod systems;
pub mod mlp;
pub mod swimming;

// Public API: only export what is needed outside the agent module
pub use components::{AgentType, Hunger, Energy, MovementProfile, MovementEffect, agent_state::AgentState, MovementHistory, DecisionEngineConfig, InteractionState, RecentInteraction};
pub use systems::{spawn_agent, path_following_system, action_selection_system, agent_movement_history_system, agent_death_system};
pub use mlp::{MLP, MLPConfig};
