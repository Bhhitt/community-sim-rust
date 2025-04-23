use legion::*;
use crate::agent::AgentType;
use crate::ecs_components::Position;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// ECS-style agent rendering, but as a plain function for now
pub fn agent_render(world: &World, canvas: &mut Canvas<Window>, camera_x: f32, camera_y: f32, cell_size: f32) {
    for (_entity, (pos, agent_type_opt)) in <(Entity, (&Position, Option<&AgentType>))>::query().iter(world) {
        if let Some(agent_type) = agent_type_opt {
            let rect = Rect::new(
                ((pos.x - camera_x) * cell_size) as i32,
                ((pos.y - camera_y) * cell_size) as i32,
                cell_size as u32,
                cell_size as u32,
            );
            let color_str = agent_type.color.trim();
            if let Some(stripped) = color_str.strip_prefix('#') {
                if stripped.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&stripped[0..2], 16),
                        u8::from_str_radix(&stripped[2..4], 16),
                        u8::from_str_radix(&stripped[4..6], 16),
                    ) {
                        canvas.set_draw_color(Color::RGB(r, g, b));
                    } else {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }
                } else {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
            } else {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            }
            let _ = canvas.fill_rect(rect);
        }
    }
}
