// Handles all agent drawing and visual representation
// Agent rendering logic migrated from render/agent.rs

use sdl2::render::Canvas;
use sdl2::video::Window;
use legion::*;

// [DEPRECATED: migrated to render/selected_agent_path_system.rs]
#[allow(dead_code)]
#[deprecated(note = "Use selected_agent_path_render in render/selected_agent_path_system.rs instead")] 
pub fn draw_selected_agent_path(
    _canvas: &mut Canvas<Window>,
    _world: &World,
    _selected_agent: Option<legion::Entity>,
    _camera_x: f32,
    _camera_y: f32,
    _cell_size: f32,
) {
    unimplemented!("draw_selected_agent_path is deprecated. Use selected_agent_path_render instead.");
}

// [DEPRECATED: migrated to render/agent_system.rs]
#[allow(dead_code)]
#[deprecated(note = "Use agent_render in render/agent_system.rs instead")]
pub fn draw_agents(
    _canvas: &mut Canvas<Window>,
    _world: &World,
    _camera_x: f32,
    _camera_y: f32,
    _cell_size: f32,
) {
    unimplemented!("draw_agents is deprecated. Use agent_render instead.");
}
