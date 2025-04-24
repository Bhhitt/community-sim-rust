// Agent rendering logic will be moved here from sim_render.rs

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use legion::*;
use log;

/// Draws the selected agent's path if selected_agent is Some
pub fn draw_selected_agent_path(
    canvas: &mut Canvas<Window>,
    world: &World,
    selected_agent: Option<legion::Entity>,
    camera_x: f32,
    camera_y: f32,
    cell_size: f32,
) {
    log::debug!("[DEBUG] Entered draw_selected_agent_path: selected_agent = {:?}", selected_agent);
    if let Some(sel) = selected_agent {
        if let Ok(entry) = world.entry_ref(sel) {
            let pos = entry.get_component::<crate::ecs_components::Position>();
            let path = entry.get_component::<crate::navigation::Path>();
            match (&pos, &path) {
                (Ok(pos), Ok(path)) => {
                    log::debug!("[DEBUG] Selected agent {:?} position: ({}, {})", sel, pos.x, pos.y);
                    log::debug!("[DEBUG] Path waypoints length: {}", path.waypoints.len());
                    log::debug!("[DEBUG] Path waypoints: {:?}", path.waypoints);
                },
                (Err(_), _) => {
                    log::debug!("[DEBUG] Selected agent {:?} has no Position component", sel);
                },
                (_, Err(_)) => {
                    log::debug!("[DEBUG] Selected agent {:?} has no Path component", sel);
                },
            }
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

/// Draws all agents on the map
pub fn draw_agents(
    canvas: &mut Canvas<Window>,
    world: &World,
    camera_x: f32,
    camera_y: f32,
    cell_size: f32,
) {
    for (_entity, (pos, agent_type_opt)) in <(legion::Entity, (&crate::ecs_components::Position, Option<&crate::agent::AgentType>))>::query().iter(world) {
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
        canvas.fill_rect(rect).ok();
    }
}
