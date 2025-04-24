use legion::World;
use legion::EntityStore;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

/// Selected agent path rendering function
pub fn selected_agent_path_render(world: &World, canvas: &mut Canvas<Window>, selected_agent: Option<legion::Entity>, camera_x: f32, camera_y: f32, cell_size: f32) {
    if let Some(sel) = selected_agent {
        if let Ok(entry) = world.entry_ref(sel) {
            let pos = entry.get_component::<crate::ecs_components::Position>();
            let path = entry.get_component::<crate::navigation::Path>();
            let movement_history = entry.get_component::<crate::agent::components::MovementHistory>();
            // --- DEBUG LOGGING ---
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
            if let Ok(mh) = &movement_history {
                if mh.positions.len() > 1 {
                    canvas.set_draw_color(Color::RGB(220, 220, 80)); // faded yellow
                    let mut last = mh.positions[0];
                    for &(x, y) in mh.positions.iter().skip(1) {
                        let start = ((last.0 - camera_x) * cell_size, (last.1 - camera_y) * cell_size);
                        let end = ((x - camera_x) * cell_size, (y - camera_y) * cell_size);
                        let _ = canvas.draw_line((start.0 as i32, start.1 as i32), (end.0 as i32, end.1 as i32));
                        last = (x, y);
                    }
                }
            }
            // --- END DEBUG LOGGING ---
            if let (Ok(pos), Ok(path)) = (pos, path) {
                let waypoints: Vec<_> = path.waypoints.iter().collect();
                if !waypoints.is_empty() {
                    canvas.set_draw_color(Color::RGB(0, 200, 255));
                    let mut last = ((pos.x - camera_x) * cell_size, (pos.y - camera_y) * cell_size);
                    for (wx, wy) in waypoints.iter() {
                        let wx = *wx;
                        let wy = *wy;
                        let next = ((wx - camera_x) * cell_size, (wy - camera_y) * cell_size);
                        let _ = canvas.draw_line((last.0 as i32, last.1 as i32), (next.0 as i32, next.1 as i32));
                        last = (wx, wy);
                    }
                }
            }
        }
    }
}

// Old function left for reference. Remove after integration.
/// ECS-style selected agent path rendering as a plain function
pub fn selected_agent_path_render_old(
    world: &World,
    canvas: &mut Canvas<Window>,
    selected_agent: Option<legion::Entity>,
    camera_x: f32,
    camera_y: f32,
    cell_size: f32,
) {
    if let Some(sel) = selected_agent {
        if let Ok(entry) = world.entry_ref(sel) {
            let pos = entry.get_component::<crate::ecs_components::Position>();
            let path = entry.get_component::<crate::navigation::Path>();
            let movement_history = entry.get_component::<crate::agent::components::MovementHistory>();
            // --- DEBUG LOGGING ---
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
            if let Ok(mh) = &movement_history {
                if mh.positions.len() > 1 {
                    canvas.set_draw_color(Color::RGB(220, 220, 80)); // faded yellow
                    let mut last = mh.positions[0];
                    for &(x, y) in mh.positions.iter().skip(1) {
                        let start = ((last.0 - camera_x) * cell_size, (last.1 - camera_y) * cell_size);
                        let end = ((x - camera_x) * cell_size, (y - camera_y) * cell_size);
                        let _ = canvas.draw_line((start.0 as i32, start.1 as i32), (end.0 as i32, end.1 as i32));
                        last = (x, y);
                    }
                }
            }
            // --- END DEBUG LOGGING ---
            if let (Ok(pos), Ok(path)) = (pos, path) {
                let waypoints: Vec<_> = path.waypoints.iter().collect();
                if !waypoints.is_empty() {
                    canvas.set_draw_color(Color::RGB(0, 200, 255));
                    let mut last = ((pos.x - camera_x) * cell_size, (pos.y - camera_y) * cell_size);
                    for (wx, wy) in waypoints.iter() {
                        let next = ((wx - camera_x) * cell_size, (wy - camera_y) * cell_size);
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
