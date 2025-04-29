// DEPRECATED: This module's simulation loop is superseded by the unified simulation loop in sim_loop_unified.rs.
// All graphics simulation logic should be routed through the unified setup and loop.
// This file is retained for reference and will be removed after migration is complete.

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;
use sdl2::EventPump;
use std::time::Duration;
use std::collections::HashMap;
use log;
use legion::{IntoQuery};
use std::sync::{Arc, Mutex};

use crate::graphics::camera::Camera;
use crate::agent::AgentType;
use crate::log_config::LogConfig;
use crate::graphics::render::terrain::draw_terrain;
use crate::graphics::sim_state::SimUIState;
use crate::event_log::EventLog;

pub fn init_sdl2(
    map_width: i32,
    map_height: i32,
    _cell_size: f32,
    window_width: u32,
    window_height: u32,
    log_config: &LogConfig,
) -> (Canvas<Window>, Canvas<Window>, Option<Canvas<Window>>, EventPump, u32, u32, Option<u32>, Camera, &'static Font<'static, 'static>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Community Simulator", window_width, window_height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let window_id = canvas.window().id();
    let _texture_creator = canvas.texture_creator();
    let event_pump = sdl_context.event_pump().unwrap();
    let camera = Camera::new(map_width, map_height, window_width, window_height);
    // Allocate ttf_context and font on the heap and leak both for 'static lifetime
    let ttf_context = Box::new(sdl2::ttf::init().unwrap());
    let ttf_context_ref: &'static sdl2::ttf::Sdl2TtfContext = Box::leak(ttf_context);
    let font_path = "/System/Library/Fonts/Supplemental/Arial.ttf";
    let font = Box::new(ttf_context_ref.load_font(font_path, 18).unwrap());
    let font_ref: &'static Font<'static, 'static> = Box::leak(font);
    if log_config.stats {
        log::info!("[FONT] Loaded font from {}", font_path);
    }
    let stats_window_canvas = match video_subsystem.window("Stats", 320, 700)
        .position(0, 0)
        .resizable()
        .build() {
        Ok(window) => match window.into_canvas().software().build() {
            Ok(canvas) => {
                if log_config.stats {
                    log::info!("[STATS] Stats window canvas created with software renderer");
                }
                canvas
            },
            Err(e) => {
                log::error!("[STATS] Failed to create stats window canvas (software): {}", e);
                panic!("Failed to create stats window canvas: {}", e);
            }
        },
        Err(e) => {
            log::error!("[STATS] Failed to create stats window: {}", e);
            panic!("Failed to create stats window: {}", e);
        }
    };
    let stats_canvas = stats_window_canvas;
    let _stats_window_id = stats_canvas.window().id();
    let (log_canvas, _log_window_id) = if !log_config.quiet {
        let log_window_canvas = video_subsystem.window("Event Log", 600, 480)
            .position(340, 0)
            .resizable()
            .build().unwrap()
            .into_canvas().build().unwrap();
        let log_canvas = log_window_canvas;
        let _log_window_id = log_canvas.window().id();
        (Some(log_canvas), Some(_log_window_id))
    } else {
        (None, None)
    };
    (canvas, stats_canvas, log_canvas, event_pump, window_id, _stats_window_id, _log_window_id, camera, font_ref)
}

#[allow(clippy::too_many_arguments)]
pub fn main_sim_loop(
    sim_ui_state: &mut SimUIState,
    canvas: &mut Canvas<Window>,
    stats_canvas: &mut Canvas<Window>,
    log_canvas: &mut Option<Canvas<Window>>,
    event_pump: &mut EventPump,
    window_id: u32,
    agent_types: &[AgentType],
    render_map: &crate::map::Map,
    log_config: &LogConfig,
    profile_systems: bool,
    csv_file: &mut Option<std::fs::File>,
    _map_width: i32,
    _map_height: i32,
    _cell_size: f32,
    _window_width: u32,
    _window_height: u32,
) {
    use std::io::Write;
    let mut paused = false;
    let mut advance_one = false;
    let mut last_stats_update = std::time::Instant::now();
    // Initialize cached_stats for the first time
    crate::graphics::sim_state::update_cached_stats(
        &*sim_ui_state.world,
        &*sim_ui_state.resources,
        &mut sim_ui_state.cached_stats,
    );
    loop {
        // --- Run ECS systems ---
        if !paused || advance_one {
            // Run all ECS systems for this tick
            sim_ui_state.schedule.execute(
                &mut sim_ui_state.world,
                &mut sim_ui_state.resources,
            );
            sim_ui_state.tick += 1;
            advance_one = false;
        }
        // --- Print latest EventLog entry to console ---
        if let Some(event_log) = sim_ui_state.resources.get::<Arc<Mutex<EventLog>>>() {
            let event_log = event_log.lock().unwrap();
            if !event_log.events.is_empty() {
                if let Some(last_event) = event_log.events.back() {
                    log::debug!("{}", last_event);
                }
            }
        }
        // --- Event log window rendering ---
        if let Some(log_canvas) = log_canvas {
            crate::graphics::render::event_log_system::event_log_window_render(
                &sim_ui_state.world,
                &sim_ui_state.resources,
                log_canvas,
                sim_ui_state.font,
                !log_config.quiet,
            );
        }
        // Handle events (refactored: collect input events into InputQueue)
        crate::graphics::input::collect_input_events(
            event_pump,
            window_id,
            sim_ui_state,
            agent_types,
            render_map,
            _cell_size,
            log_config,
            paused,
        );
        // Process input intents (ECS-friendly event handling)
        crate::graphics::input_systems::process_input_intents(
            sim_ui_state,
            agent_types,
            render_map,
            _cell_size,
            &mut paused,
            &mut advance_one,
        );
        // --- Update cached stats ONCE PER SECOND ---
        if last_stats_update.elapsed().as_secs_f32() >= 1.0 {
            crate::graphics::sim_state::update_cached_stats(
                &*sim_ui_state.world,
                &*sim_ui_state.resources,
                &mut sim_ui_state.cached_stats,
            );
            last_stats_update = std::time::Instant::now();
        }
        // Now destructure sim_ui_state for rendering
        let SimUIState {
            world,
            // schedule,
            camera,
            font,
            cached_stats,
            selected_agent,
            empty_cell_flash,
            // tick,
            ..
        } = sim_ui_state;
        log::debug!("[DEBUG] About to render terrain");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_terrain(canvas, render_map, camera.x, camera.y, _cell_size);
        // --- ECS food rendering system integration ---
        crate::graphics::render::food_system::food_render_system(
            world,
            canvas,
            camera.x,
            camera.y,
            _cell_size,
            *selected_agent,
        );
        log::debug!("[DEBUG] sim_loop: selected_agent = {:?}", selected_agent);
        // --- Selected agent path rendering ---
        crate::graphics::render::selected_agent_path_system::selected_agent_path_render(
            world,
            canvas,
            *selected_agent,
            camera.x,
            camera.y,
            _cell_size,
        );
        // --- Agent rendering ---
        crate::graphics::render::agent_system::agent_render_system(
            world,
            canvas,
            camera.x,
            camera.y,
            _cell_size,
        );
        // --- Empty cell flash overlay rendering ---
        crate::graphics::render::overlays::empty_cell_flash_render(
            world,
            canvas,
            *empty_cell_flash,
            camera.x,
            camera.y,
            _cell_size,
        );
        // Clear the flash if expired
        if let Some((_, _, t)) = *empty_cell_flash {
            if t.elapsed().as_millis() >= 200 {
                *empty_cell_flash = None;
            }
        }
        log::debug!("[DEBUG] About to present main canvas");
        canvas.present();
        let (stats_w, stats_h) = stats_canvas.window().size();
        if log_config.stats {
            log::info!("[STATS] Window size: {}x{}", stats_w, stats_h);
        }
        let mut agent_type_counts = HashMap::<String, usize>::new();
        let mut agent_query = <(&AgentType,)>::query();
        for (agent_type,) in agent_query.iter(*world) {
            *agent_type_counts.entry(agent_type.name.clone()).or_insert(0) += 1;
        }
        // --- Stats window rendering ---
        crate::graphics::render::stats_system::stats_window_render(
            world,
            stats_canvas,
            font,
            cached_stats,
            *selected_agent,
            log_config.stats,
        );
        log::debug!("[DEBUG] About to present stats canvas");
        stats_canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }
}
