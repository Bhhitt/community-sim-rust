use legion::World;
use legion::IntoQuery;
use crate::agent::AgentType;
use crate::ecs_components::Position;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

/// Agent rendering function
pub fn agent_render_system(world: &World, canvas: &mut Canvas<Window>, camera_x: f32, camera_y: f32, cell_size: f32) {
    let mut query = <(&Position, Option<&AgentType>)>::query();
    for (pos, agent_type_opt) in query.iter(world) {
        let rect = Rect::new(
            ((pos.x - camera_x) * cell_size) as i32,
            ((pos.y - camera_y) * cell_size) as i32,
            cell_size as u32,
            cell_size as u32,
        );
        if let Some(agent_type) = agent_type_opt {
            let (r, g, b) = agent_type.color;
            canvas.set_draw_color(Color::RGB(r, g, b));
        } else {
            canvas.set_draw_color(Color::RGB(0, 220, 0));
        }
        let _ = canvas.fill_rect(rect);
    }
}
