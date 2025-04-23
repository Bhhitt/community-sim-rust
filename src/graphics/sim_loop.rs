// Contains the main simulation loop logic (without rendering)

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;
use sdl2::EventPump;
use std::time::Duration;
use crate::graphics::camera::Camera;
use crate::agent::AgentType;
use crate::log_config::LogConfig;
use crate::graphics::render::terrain::draw_terrain;
use crate::graphics::render::food_system::food_render;
use crate::graphics::render::agent_system::agent_render;
use crate::graphics::render::selected_agent_path_system::selected_agent_path_render;
use crate::graphics::render::stats_system::stats_window_render;
use crate::graphics::render::event_log_system::event_log_window_render;
use crate::graphics::sim_state::SimUIState;
use crate::graphics::render::overlays::draw_empty_cell_flash;
// use crate::graphics::render::overlays::draw_stats_window;
// use legion::systems::Runnable;

pub fn init_sdl2(
    map_width: i32,
    map_height: i32,
    cell_size: f32,
    window_width: u32,
    window_height: u32,
    log_config: &LogConfig,
) -> (Canvas<Window>, Canvas<Window>, Canvas<Window>, EventPump, u32, u32, u32, Camera, &'static Font<'static, 'static>) {
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
    let camera = Camera::new(map_width, map_height, cell_size as u32, window_width, window_height);
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
    let log_window_canvas = video_subsystem.window("Event Log", 600, 480)
        .position(340, 0)
        .resizable()
        .build().unwrap()
        .into_canvas().build().unwrap();
    let log_canvas = log_window_canvas;
    let _log_window_id = log_canvas.window().id();
    (canvas, stats_canvas, log_canvas, event_pump, window_id, _stats_window_id, _log_window_id, camera, font_ref)
}

#[allow(clippy::too_many_arguments)]
pub fn main_sim_loop(
    sim_ui_state: &mut SimUIState,
    canvas: &mut Canvas<Window>,
    stats_canvas: &mut Canvas<Window>,
    log_canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    window_id: u32,
    agent_types: &[AgentType],
    render_map: &crate::map::Map,
    log_config: &LogConfig,
    profile_systems: bool,
    csv_file: &mut Option<std::fs::File>,
    _map_width: i32,
    _map_height: i32,
    cell_size: f32,
    _window_width: u32,
    _window_height: u32,
) {
    use std::io::Write;
    let mut paused = false;
    let mut advance_one = false;
    loop {
        // --- Run ECS systems ---
        if !paused || advance_one {
            if profile_systems {
                use crate::ecs_simulation::simulation_tick;
                let profile = simulation_tick(
                    &mut sim_ui_state.world,
                    &mut sim_ui_state.resources,
                    &mut sim_ui_state.schedule,
                );
                if let Some(csv_file) = csv_file.as_mut() {
                    writeln!(csv_file, "{}{}{}", sim_ui_state.tick, ",", profile.to_csv_row()).unwrap();
                }
            } else {
                // Use parallel tick
                let _ = crate::ecs_simulation::simulation_tick_parallel(&mut sim_ui_state.world, &mut sim_ui_state.resources, &mut sim_ui_state.schedule);
            }
            sim_ui_state.tick += 1;
            advance_one = false;
        }
        // --- Print latest EventLog entry to console ---
        if let Some(event_log) = sim_ui_state.resources.get::<crate::event_log::EventLog>() {
            if !event_log.events.is_empty() {
                if let Some(last_event) = event_log.events.back() {
                    log::debug!("{}", last_event);
                }
            }
        }
        // --- Render Event Log Window ---
        if let Some(event_log) = sim_ui_state.resources.get::<crate::event_log::EventLog>() {
            // crate::graphics::overlays::draw_event_log_window(
            //     log_canvas,
            //     sim_ui_state.font,
            //     &event_log,
            //     log_config.interact,
            // );
            // --- ECS event log window rendering (plain function) ---
            event_log_window_render(
                &event_log,
                log_canvas,
                sim_ui_state.font,
                log_config.interact,
            );
        }
        // Handle events (refactored: collect input events into InputQueue)
        crate::graphics::input::collect_input_events(
            event_pump,
            window_id,
            sim_ui_state,
            agent_types,
            render_map,
            cell_size,
            log_config,
            paused,
        );
        // Process input intents (ECS-friendly event handling)
        crate::graphics::input_systems::process_input_intents(
            sim_ui_state,
            agent_types,
            render_map,
            cell_size,
            &mut paused,
            &mut advance_one,
        );
        // --- Update cached agent counts BEFORE destructuring sim_ui_state or passing any fields ---
        crate::graphics::sim_state::update_cached_agent_counts(
            &*sim_ui_state.world,
            &mut sim_ui_state.cached_agent_counts,
        );
        // Now destructure sim_ui_state for rendering
        let SimUIState {
            world,
            resources,
            // schedule,
            camera,
            font,
            cached_agent_counts,
            selected_agent,
            empty_cell_flash,
            // tick,
            ..
        } = sim_ui_state;
        log::debug!("[DEBUG] About to render terrain");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_terrain(canvas, render_map, camera.x, camera.y, cell_size);
        // draw_food(canvas, world, camera.x, camera.y, cell_size, *selected_agent);
        // --- ECS food rendering (plain function) ---
        food_render(world, canvas, camera.x, camera.y, cell_size, *selected_agent);
        log::debug!("[DEBUG] sim_loop: selected_agent = {:?}", selected_agent);
        // draw_selected_agent_path(canvas, world, *selected_agent, camera.x, camera.y, cell_size);
        // --- ECS selected agent path rendering (plain function) ---
        selected_agent_path_render(world, canvas, *selected_agent, camera.x, camera.y, cell_size);
        // --- ECS agent rendering (plain function) ---
        agent_render(world, canvas, camera.x, camera.y, cell_size);
        if let Some((fx, fy, t)) = *empty_cell_flash {
            if t.elapsed().as_millis() < 200 {
                crate::graphics::overlays::draw_empty_cell_flash(canvas, fx, fy, camera.x, camera.y, cell_size);
            } else {
                *empty_cell_flash = None;
            }
        }
        log::debug!("[DEBUG] About to present main canvas");
        canvas.present();
        let (stats_w, stats_h) = stats_canvas.window().size();
        if log_config.stats {
            log::info!("[STATS] Window size: {}x{}", stats_w, stats_h);
        }
        let interaction_stats = resources.get::<crate::ecs_components::InteractionStats>();
        // draw_stats_window(
        //     stats_canvas,
        //     font,
        //     &cached_agent_counts[..],
        //     interaction_stats.as_ref().map(|v| &**v),
        //     *selected_agent,
        //     world,
        //     log_config.stats,
        // );
        // --- ECS stats window rendering (plain function) ---
        stats_window_render(
            world,
            resources,
            stats_canvas,
            font,
            &cached_agent_counts[..],
            interaction_stats.as_ref().map(|v| &**v),
            *selected_agent,
            log_config.stats,
        );
        log::debug!("[DEBUG] About to present stats canvas");
        stats_canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }
}
