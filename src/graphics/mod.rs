// Graphics module root
pub mod camera;
pub mod sim_render;
pub mod terrain;
pub mod utils;
// Add further submodules as the migration proceeds

use crate::agent::AgentType;
use legion::*;

pub use sim_render::run_sim_render;

pub fn run_with_graphics_profile(_map_width: i32, _map_height: i32, _num_agents: usize, agent_types: &[AgentType], profile_systems: bool, profile_csv: &str) {
    use crate::ecs_simulation::build_simulation_schedule_parallel;
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = build_simulation_schedule_parallel();
    run_sim_render(_map_width, _map_height, _num_agents, agent_types, profile_systems, profile_csv, &mut world, &mut resources, &mut schedule);
}
