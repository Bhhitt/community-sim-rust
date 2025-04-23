// Overlay/UI rendering logic will be moved here from sim_render.rs

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use crate::event_log::EventLog;
use crate::ecs_components::{InteractionStats};
use legion::*;

/// Draws the event log window
pub fn draw_event_log_window(canvas: &mut Canvas<Window>, font: &Font, event_log: &EventLog, log_window_enabled: bool) {
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();
    let texture_creator = canvas.texture_creator();
    if log_window_enabled {
        let mut y = 10;
        let line_height = 20;
        let max_lines = 22;
        let events_vec = &event_log.events;
        let events: Vec<_> = events_vec.iter().rev().take(max_lines).collect();
        for entry in events.iter().rev() {
            let surface = font.render(entry)
                .blended(Color::RGB(220, 220, 220)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let sdl2::render::TextureQuery { width, height, .. } = texture.query();
            let target = Rect::new(10, y, width, height);
            canvas.copy(&texture, None, Some(target)).unwrap();
            y += line_height;
        }
    } else {
        // Draw "Quiet mode" in the center
        let text = "Quiet mode";
        let surface = font.render(text)
            .blended(Color::RGB(180, 180, 180)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
        let (win_w, win_h) = canvas.window().size();
        let target = Rect::new(
            (win_w as i32 - width as i32) / 2,
            (win_h as i32 - height as i32) / 2,
            width,
            height,
        );
        canvas.copy(&texture, None, Some(target)).unwrap();
    }
    canvas.present();
}

/// Draws the flash highlight for empty cell clicks
pub fn draw_empty_cell_flash(canvas: &mut Canvas<Window>, fx: i32, fy: i32, camera_x: f32, camera_y: f32, cell_size: f32) {
    let rect = Rect::new(
        ((fx as f32 - camera_x) * cell_size) as i32,
        ((fy as f32 - camera_y) * cell_size) as i32,
        cell_size as u32,
        cell_size as u32,
    );
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.draw_rect(rect).ok();
}

/// Draws the stats window (agent/food counts, interactions, graph, selected agent details)
pub fn draw_stats_window(
    canvas: &mut Canvas<Window>,
    font: &Font,
    cached_agent_counts: &[(String, usize)],
    interaction_stats: Option<&InteractionStats>,
    selected_agent: Option<legion::Entity>,
    world: &World,
    log_stats: bool,
) {
    let (win_w, win_h) = canvas.window().size();
    if log_stats {
        log::info!("[STATS][DRAW] stats window size: {}x{} (id: {})", win_w, win_h, canvas.window().id());
    }
    // Set a dark background for readability
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();
    // Red rectangle for visibility
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let _ = canvas.fill_rect(Rect::new(0, 0, 40, 40));
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.draw_rect(Rect::new(0, 0, 320, 700)).ok();
    let mut y_offset = 10;
    let texture_creator = canvas.texture_creator();
    for (name, count) in cached_agent_counts {
        if log_stats {
            log::info!("[STATS] Drawing stat: {}: {} at y_offset {}", name, count, y_offset);
        }
        let text = format!("{}: {}", name, count);
        match font.render(&text).blended(Color::RGB(220, 220, 220)) {
            Ok(surface) => {
                match texture_creator.create_texture_from_surface(&surface) {
                    Ok(texture) => {
                        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
                        let target = Rect::new(10, y_offset, width, height);
                        canvas.copy(&texture, None, Some(target)).unwrap();
                        y_offset += height as i32 + 8;
                    },
                    Err(e) => {
                        log::error!("[STATS] Texture creation failed: {}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("[STATS] Font render failed: {}", e);
            }
        }
    }
    if let Some(stats) = interaction_stats {
        let active_interactions = stats.active_interactions;
        let text = format!("active interactions: {}", active_interactions);
        let surface = font.render(&text)
            .blended(Color::RGB(120, 200, 255)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(10, y_offset, width, height);
        canvas.copy(&texture, None, Some(target)).unwrap();
        y_offset += height as i32 + 12;
        let history = &stats.active_interactions_history;
        if !history.is_empty() {
            let graph_left = 10;
            let graph_top = y_offset + 10;
            let graph_width = 280;
            let graph_height = 60;
            canvas.set_draw_color(Color::RGB(80, 80, 80));
            let _ = canvas.draw_rect(Rect::new(graph_left, graph_top, graph_width, graph_height));
            let max_val = *history.iter().max().unwrap_or(&1) as f32;
            let min_val = *history.iter().min().unwrap_or(&0) as f32;
            let range = (max_val - min_val).max(1.0);
            let n = history.len().min(graph_width as usize);
            let x_step = graph_width as f32 / (n.max(2) - 1) as f32;
            let mut last_x = graph_left as f32;
            let mut last_y = graph_top as f32 + graph_height as f32 - ((history[0] as f32 - min_val) / range * graph_height as f32);
            canvas.set_draw_color(Color::RGB(120, 200, 255));
            for (i, &val) in history.iter().rev().take(n).collect::<Vec<_>>().into_iter().rev().enumerate() {
                let x = graph_left as f32 + i as f32 * x_step;
                let y = graph_top as f32 + graph_height as f32 - ((val as f32 - min_val) / range * graph_height as f32);
                if i > 0 {
                    let _ = canvas.draw_line((last_x as i32, last_y as i32), (x as i32, y as i32));
                }
                last_x = x;
                last_y = y;
            }
            y_offset += 80;
        }
    }
    // Selected agent details
    if let Some(sel) = selected_agent {
        let mut _shown = false;
        for (_entity, (pos, agent_type, hunger, energy, target, _path, interaction_state)) in <(legion::Entity, (&crate::ecs_components::Position, &crate::agent::AgentType, &crate::agent::Hunger, &crate::agent::Energy, Option<&crate::navigation::Target>, Option<&crate::navigation::Path>, Option<&crate::agent::components::InteractionState>))>::query().iter(world) {
            if *_entity == sel {
                let mut status = String::new();
                if let Some(target) = target {
                    let dist = ((pos.x - target.x).powi(2) + (pos.y - target.y).powi(2)).sqrt();
                    if dist > 0.2 {
                        status = format!("Moving to ({:.1}, {:.1})", target.x, target.y);
                    } else {
                        status = "Idle (at target)".to_string();
                    }
                }
                if let Some(inter) = interaction_state {
                    if let Some(_partner) = inter.target {
                        status = "Interacting with another agent".to_string();
                    }
                }
                let text = format!("Selected Agent:\nPos: ({:.1}, {:.1})\nType: {}\nHunger: {:.1}\nEnergy: {:.1}\nStatus: {}", pos.x, pos.y, agent_type.r#type, hunger.value, energy.value, status);
                for (i, line) in text.lines().enumerate() {
                    let surface = font.render(line).blended(Color::RGB(255, 255, 0)).unwrap();
                    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let sdl2::render::TextureQuery { width, height, .. } = texture.query();
                    let target = Rect::new(10, y_offset + i as i32 * (height as i32 + 2), width, height);
                    canvas.copy(&texture, None, Some(target)).unwrap();
                }
                _shown = true;
                break;
            }
        }
    }
    canvas.present();
}
