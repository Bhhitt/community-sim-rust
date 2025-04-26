pub mod components;
pub mod systems;
pub mod mlp;
pub mod swimming;
pub mod event;
pub mod event_log_bridge;
// pub mod pause_system; // [REMOVED as of 2025-04-26]

// Public API: only export what is needed outside the agent module
pub use components::AgentType;
pub use components::Hunger;
pub use components::Energy;
pub use components::MovementProfile;
pub use components::MovementEffect;
pub use components::agent_state::AgentState;
pub use components::MovementHistory;
pub use components::DecisionEngineConfig;
pub use components::InteractionState;
pub use components::RecentInteraction;
// Removed unresolved/legacy system exports
pub use systems::agent_death_system;
pub use mlp::{MLP, MLPConfig};

// TODO: Remove unresolved imports for missing systems
// use systems::path_following_system;
// use systems::agent_movement_history_system;
