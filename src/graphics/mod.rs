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
use legion::*;

// pub use sim_render::run_sim_render;
// pub use crate::graphics::sim_loop::main_sim_loop;

pub fn run_with_graphics_profile(
    _map_width: i32,
    _map_height: i32,
    _num_agents: usize,
    agent_types: &[AgentType],
    profile_systems: bool,
    profile_csv: &str,
    log_config: &LogConfig,
    event_log: std::sync::Arc<std::sync::Mutex<crate::event_log::EventLog>>,
) {
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(log_config.clone());
    resources.insert(event_log);
    resources.insert(AgentEventLog::default());
    crate::graphics::sim_render::run_sim_render(
        _map_width,
        _map_height,
        _num_agents,
        agent_types,
        profile_systems,
        profile_csv,
        &mut world,
        &mut resources,
    );
}
