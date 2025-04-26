// Graphics module root
pub mod camera;
pub mod input;
pub mod input_intent;
pub mod input_systems;
pub mod render;
pub mod sim_render;
pub mod sim_loop;
pub mod terrain;
pub mod utils;
pub mod agent_render;
pub mod sim_state;
// Add further submodules as the migration proceeds

use crate::agent::AgentType;
use crate::agent::event::AgentEventLog;
use crate::log_config::LogConfig;
use crate::sim_profile::SimProfile;
use legion::*;

// pub use sim_render::run_sim_render;
// pub use crate::graphics::sim_loop::main_sim_loop;
