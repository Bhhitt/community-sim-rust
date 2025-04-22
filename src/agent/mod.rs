pub mod components;
pub mod systems;

pub use components::{AgentType, Hunger, Energy, MovementProfile, MovementEffect};
pub use systems::*;
pub use systems::{agent_movement_system, agent_death_system};
