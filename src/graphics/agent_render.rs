// Handles all agent drawing and visual representation
// Agent rendering logic migrated from render/agent.rs

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use legion::*;
use crate::agent::AgentType;

// [DEPRECATED: migrated to render/selected_agent_path_system.rs]
#[allow(dead_code)]
#[deprecated(note = "Use selected_agent_path_render in render/selected_agent_path_system.rs instead")] 
pub fn draw_selected_agent_path(
    canvas: &mut Canvas<Window>,
    world: &World,
    selected_agent: Option<legion::Entity>,
    camera_x: f32,
    camera_y: f32,
    cell_size: f32,
) {
    unimplemented!("draw_selected_agent_path is deprecated. Use selected_agent_path_render instead.");
}

// [DEPRECATED: migrated to render/agent_system.rs]
#[allow(dead_code)]
#[deprecated(note = "Use agent_render in render/agent_system.rs instead")]
pub fn draw_agents(
    canvas: &mut Canvas<Window>,
    world: &World,
    camera_x: f32,
    camera_y: f32,
    cell_size: f32,
) {
    unimplemented!("draw_agents is deprecated. Use agent_render instead.");
}
