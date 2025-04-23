// Handles all agent drawing and visual representation
// Agent rendering logic migrated from render/agent.rs

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use legion::*;
use crate::agent::AgentType;

pub fn draw_selected_agent_path(
    canvas: &mut Canvas<Window>,
    world: &World,
    selected_agent: Option<legion::Entity>,
    camera_x: f32,
    camera_y: f32,
    cell_size: f32,
) {
    if let Some(sel) = selected_agent {
        if let Ok(entry) = world.entry_ref(sel) {
            let pos = entry.get_component::<crate::ecs_components::Position>();
            let path = entry.get_component::<crate::navigation::Path>();
            if let (Ok(pos), Ok(path)) = (pos, path) {
                let waypoints: Vec<_> = path.waypoints.iter().collect();
                if waypoints.len() > 0 {
                    canvas.set_draw_color(Color::RGB(0, 200, 255));
                    let mut last = ((pos.x - camera_x) * cell_size, (pos.y - camera_y) * cell_size);
                    for (wx, wy) in waypoints.iter() {
                        let next = ((*wx - camera_x) * cell_size, (*wy - camera_y) * cell_size);
                        let _ = canvas.draw_line((last.0 as i32, last.1 as i32), (next.0 as i32, next.1 as i32));
                        last = next;
                    }
                    if let Some((end_x, end_y)) = waypoints.last() {
                        let dot_rect = Rect::new(
                            ((*end_x - camera_x) * cell_size) as i32 - 3,
                            ((*end_y - camera_y) * cell_size) as i32 - 3,
                            7, 7
                        );
                        canvas.set_draw_color(Color::RGB(255, 0, 200));
                        let _ = canvas.fill_rect(dot_rect);
                    }
                }
            }
        }
    }
}

pub fn draw_agents(
    canvas: &mut Canvas<Window>,
    world: &World,
    camera_x: f32,
    camera_y: f32,
    cell_size: f32,
) {
    for (_entity, (pos, agent_type_opt)) in <(legion::Entity, (&crate::ecs_components::Position, Option<&AgentType>))>::query().iter(world) {
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
        // else: skip entities that are not agents (e.g., food)
    }
}
