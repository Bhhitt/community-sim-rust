// Overlay/UI rendering logic will be moved here from sim_render.rs

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use crate::event_log::EventLog;
use crate::ecs_components::InteractionStats;
use legion::{World, Resources, IntoQuery};
use crate::config::StatsWindowConfig;
use std::path::Path;

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
    resources: &Resources,
    log_stats: bool,
) {
    // Always show this default info (e.g., "Simulation Stats")
    let (win_w, win_h) = canvas.window().size();
    if log_stats {
        log::info!("[STATS][DRAW] stats window size: {}x{} (id: {})", win_w, win_h, canvas.window().id());
    }
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();
    let texture_creator = canvas.texture_creator();
    let mut y = 10;
    let line_height = 22;
    // --- Always-present section ---
    let default_text = "Simulation Stats";
    let surface = font.render(default_text)
        .blended(Color::RGB(220, 220, 220)).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let sdl2::render::TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(10, y, width, height);
    canvas.copy(&texture, None, Some(target)).unwrap();
    y += line_height;
    // --- Configurable components ---
    let config = StatsWindowConfig::load_from_yaml(Path::new("config/stats_window.yaml"));
    if let Some(components) = config.components {
        for comp in components {
            match comp.as_str() {
                "agent_counts" => {
                    let text = format!("Agents: {}", cached_agent_counts.iter().map(|(_, n)| n).sum::<usize>());
                    let surface = font.render(&text).blended(Color::RGB(180, 255, 180)).unwrap();
                    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let sdl2::render::TextureQuery { width, height, .. } = texture.query();
                    let target = Rect::new(10, y, width, height);
                    canvas.copy(&texture, None, Some(target)).unwrap();
                    y += line_height;
                }
                "food_counts" => {
                    // Compute real food count from ECS world
                    let food_count = <(&crate::ecs_components::Position, &crate::food::Food)>::query().iter(world).count();
                    let text = format!("Food: {}", food_count);
                    let surface = font.render(&text).blended(Color::RGB(255, 220, 180)).unwrap();
                    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let sdl2::render::TextureQuery { width, height, .. } = texture.query();
                    let target = Rect::new(10, y, width, height);
                    canvas.copy(&texture, None, Some(target)).unwrap();
                    y += line_height;

                    // Show food spawned/collected per tick if FoodStats resource is available
                    if let Some(food_stats) = resources.get::<crate::ecs_components::FoodStats>() {
                        let spawned = food_stats.spawned_per_tick;
                        let collected = food_stats.collected_per_tick;
                        let text = format!("Food spawned/tick: {}", spawned);
                        let surface = font.render(&text).blended(Color::RGB(255, 220, 100)).unwrap();
                        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
                        let target = Rect::new(10, y, width, height);
                        canvas.copy(&texture, None, Some(target)).unwrap();
                        y += line_height;
                        let text = format!("Food collected/tick: {}", collected);
                        let surface = font.render(&text).blended(Color::RGB(255, 180, 100)).unwrap();
                        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
                        let target = Rect::new(10, y, width, height);
                        canvas.copy(&texture, None, Some(target)).unwrap();
                        y += line_height;
                    }
                }
                "interaction_stats" => {
                    if let Some(stats) = interaction_stats {
                        let text = format!("Interactions: {}", stats.agent_interactions);
                        let surface = font.render(&text).blended(Color::RGB(180, 180, 255)).unwrap();
                        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
                        let target = Rect::new(10, y, width, height);
                        canvas.copy(&texture, None, Some(target)).unwrap();
                        y += line_height;
                    }
                }
                "selected_agent" => {
                    if let Some(agent) = selected_agent {
                        let text = format!("Selected Agent: {:?}", agent);
                        let surface = font.render(&text).blended(Color::RGB(255, 255, 180)).unwrap();
                        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
                        let target = Rect::new(10, y, width, height);
                        canvas.copy(&texture, None, Some(target)).unwrap();
                        y += line_height;
                    }
                }
                _ => {}
            }
        }
    }
    canvas.present();
}
