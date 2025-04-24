// Overlay/UI rendering logic will be moved here from sim_render.rs

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use crate::event_log::EventLog;
use crate::config::StatsWindowConfig;
use std::path::Path;
use crate::graphics::sim_state::CachedStats;
use legion::{World, Resources, EntityStore};

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
    cached_stats: &CachedStats,
    selected_agent: Option<legion::Entity>,
    world: &World,
    _resources: &Resources,
    log_stats: bool,
) {
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
                    for (ty, n) in cached_stats.agent_counts.iter() {
                        render_stat_row(
                            canvas,
                            font,
                            &texture_creator,
                            &format!("{}: {}", ty, n),
                            Color::RGB(180, 255, 180),
                            &mut y,
                            line_height,
                        );
                    }
                    let total: usize = cached_stats.agent_counts.iter().map(|(_, n)| n).sum();
                    render_stat_row(
                        canvas,
                        font,
                        &texture_creator,
                        &format!("Total agents: {}", total),
                        Color::RGB(180, 255, 180),
                        &mut y,
                        line_height,
                    );
                }
                "food_counts" => {
                    render_stat_row(
                        canvas,
                        font,
                        &texture_creator,
                        &format!("Food: {}", cached_stats.food_count),
                        Color::RGB(255, 220, 180),
                        &mut y,
                        line_height,
                    );
                    render_stat_row(
                        canvas,
                        font,
                        &texture_creator,
                        &format!("Food spawned/tick: {}", cached_stats.food_spawned_per_tick),
                        Color::RGB(255, 220, 100),
                        &mut y,
                        line_height,
                    );
                    render_stat_row(
                        canvas,
                        font,
                        &texture_creator,
                        &format!("Food collected/tick: {}", cached_stats.food_collected_per_tick),
                        Color::RGB(255, 180, 100),
                        &mut y,
                        line_height,
                    );
                }
                "interaction_stats" => {
                    render_stat_row(
                        canvas,
                        font,
                        &texture_creator,
                        &format!("Agent interactions: {}", cached_stats.agent_interactions),
                        Color::RGB(180, 200, 255),
                        &mut y,
                        line_height,
                    );
                    render_stat_row(
                        canvas,
                        font,
                        &texture_creator,
                        &format!("Active interactions: {}", cached_stats.active_interactions),
                        Color::RGB(120, 180, 255),
                        &mut y,
                        line_height,
                    );
                    // Draw line graph for interaction history
                    draw_line_graph(
                        canvas,
                        10,
                        y,
                        300,
                        60,
                        &cached_stats.active_interactions_history,
                        Color::RGB(120, 180, 255),
                    );
                    y += 70;
                }
                "selected_agent" => {
                    if let Some(agent) = selected_agent {
                        if let Ok(entry) = world.entry_ref(agent) {
                            // Gather all components for the agent
                            let pos = entry.get_component::<crate::ecs_components::Position>().ok();
                            let agent_type = entry.get_component::<crate::agent::AgentType>().ok();
                            let hunger = entry.get_component::<crate::agent::Hunger>().ok();
                            let energy = entry.get_component::<crate::agent::Energy>().ok();
                            let state = entry.get_component::<crate::agent::AgentState>().ok();
                            let interaction = entry.get_component::<crate::agent::InteractionState>().ok();

                            render_stat_row(
                                canvas,
                                font,
                                &texture_creator,
                                &format!("Selected Agent: {:?}", agent),
                                Color::RGB(255, 255, 180),
                                &mut y,
                                line_height,
                            );
                            if let Some(pos) = pos {
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Position: ({:.1}, {:.1})", pos.x, pos.y),
                                    Color::RGB(200, 255, 200),
                                    &mut y,
                                    line_height,
                                );
                            }
                            if let Some(agent_type) = agent_type {
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Type: {}", agent_type.name),
                                    Color::RGB(200, 200, 255),
                                    &mut y,
                                    line_height,
                                );
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Move Speed: {:.2}", agent_type.movement_profile.speed),
                                    Color::RGB(180, 220, 255),
                                    &mut y,
                                    line_height,
                                );
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Strength: N/A"),
                                    Color::RGB(180, 220, 255),
                                    &mut y,
                                    line_height,
                                );
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Stamina: N/A"),
                                    Color::RGB(180, 220, 255),
                                    &mut y,
                                    line_height,
                                );
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Vision: N/A"),
                                    Color::RGB(180, 220, 255),
                                    &mut y,
                                    line_height,
                                );
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Work Rate: N/A"),
                                    Color::RGB(180, 220, 255),
                                    &mut y,
                                    line_height,
                                );
                            }
                            if let Some(hunger) = hunger {
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Hunger: {:.1}", hunger.value),
                                    Color::RGB(255, 200, 180),
                                    &mut y,
                                    line_height,
                                );
                            }
                            if let Some(energy) = energy {
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Energy: {:.1}", energy.value),
                                    Color::RGB(255, 255, 200),
                                    &mut y,
                                    line_height,
                                );
                            }
                            if let Some(state) = state {
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("State: {:?}", state),
                                    Color::RGB(255, 220, 180),
                                    &mut y,
                                    line_height,
                                );
                            }
                            if let Some(interaction) = interaction {
                                let status = if interaction.target.is_some() {
                                    "Interacting"
                                } else if interaction.cooldown > 0 {
                                    "Cooldown"
                                } else {
                                    "Idle"
                                };
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Interaction: {}", status),
                                    Color::RGB(255, 180, 255),
                                    &mut y,
                                    line_height,
                                );
                                if let Some(target) = interaction.target {
                                    render_stat_row(
                                        canvas,
                                        font,
                                        &texture_creator,
                                        &format!("Target: {:?}", target),
                                        Color::RGB(255, 180, 255),
                                        &mut y,
                                        line_height,
                                    );
                                }
                                if let Some(last_partner) = interaction.last_partner {
                                    render_stat_row(
                                        canvas,
                                        font,
                                        &texture_creator,
                                        &format!("Last Partner: {:?}", last_partner),
                                        Color::RGB(255, 180, 255),
                                        &mut y,
                                        line_height,
                                    );
                                }
                                render_stat_row(
                                    canvas,
                                    font,
                                    &texture_creator,
                                    &format!("Recent Partners: {}", interaction.recent_partners.len()),
                                    Color::RGB(255, 180, 255),
                                    &mut y,
                                    line_height,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    // ... (rest of function unchanged, e.g., selected agent details)
    canvas.present();
}

// Helper function for rendering a single stats row
fn render_stat_row(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &sdl2::ttf::Font,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    text: &str,
    color: sdl2::pixels::Color,
    y: &mut i32,
    line_height: i32,
) {
    let surface = font.render(text).blended(color).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let sdl2::render::TextureQuery { width, height, .. } = texture.query();
    let target = sdl2::rect::Rect::new(10, *y, width, height);
    canvas.copy(&texture, None, Some(target)).unwrap();
    *y += line_height;
}

// Helper function for drawing a simple line graph of recent values
fn draw_line_graph(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    data: &std::collections::VecDeque<usize>,
    color: Color,
) {
    use sdl2::rect::Point;
    let graph_height = height;
    let graph_width = width; // px
    let x_offset = x;
    let y_offset = y;
    let max = data.iter().copied().max().unwrap_or(1) as i32;
    let len = data.len().min(graph_width as usize);
    if len < 2 { return; }
    let scale_y = if max > 0 { graph_height as f32 / max as f32 } else { 1.0 };
    let points: Vec<Point> = data.iter().rev().take(len).enumerate().map(|(i, v)| {
        let x = x_offset + graph_width - i as i32;
        let y = y_offset + graph_height - (*v as f32 * scale_y) as i32;
        Point::new(x, y)
    }).collect();
    let _ = canvas.set_draw_color(color);
    let _ = canvas.draw_lines(&points[..]);
}
