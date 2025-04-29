// DEPRECATED: This module's simulation entrypoint is superseded by the unified simulation loop and setup in main.rs and sim_loop_unified.rs.
// All graphics simulation logic should be routed through the unified setup and loop.
// This file is retained for reference and will be removed after migration is complete.

// Unified Event Log Access Pattern: ALWAYS access EventLog via ECS resources as Arc<Mutex<EventLog>>.
// Do not pass EventLog as a direct argument. This ensures a single, obvious pattern for all systems.

// Main simulation rendering and event loop
// Will contain the main SDL2 rendering logic and event loop

use legion::*;
use crate::agent::AgentType;
use rand::Rng;
use crate::log_config::LogConfig;
use std::fs::File;
use crate::graphics::sim_state::SimUIState;
use crate::ecs::{resources::insert_standard_resources, systems::pending_agent_spawns::PendingAgentSpawns};
use crate::ecs::systems::pending_agent_spawns::AgentSpawnRequest;
use crate::sim_profile::SimProfile;
use crate::sim_loop_unified::{SimulationRenderer, SimulationInput, SimulationProfiler, run_simulation_loop_ui};
use crate::sim_state::SimState;

const CELL_SIZE: f32 = 6.0;

// All unused imports removed for a clean build

// --- Make SDL2 plug-in types public for unified simulation entry ---
pub struct SdlRenderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub stats_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub log_canvas_opt: Option<sdl2::render::Canvas<sdl2::video::Window>>,
}

impl crate::sim_loop_unified::SimulationRenderer for SdlRenderer {
    fn render_ui(&mut self, sim_ui_state: &mut crate::graphics::sim_state::SimUIState, tick: usize) {
        // Clear the main window BEFORE drawing terrain
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 20, 20));
        self.canvas.clear();

        // --- Terrain rendering ---
        crate::graphics::render::terrain::draw_terrain(
            &mut self.canvas,
            &*sim_ui_state.resources.get::<crate::map::Map>().expect("Map missing in resources for terrain rendering"),
            sim_ui_state.camera.x,
            sim_ui_state.camera.y,
            CELL_SIZE,
        );

        // Draw food
        crate::graphics::render::food_system::food_render_system(
            sim_ui_state.world,
            &mut self.canvas,
            sim_ui_state.camera.x,
            sim_ui_state.camera.y,
            CELL_SIZE,
            sim_ui_state.selected_agent,
        );

        // Draw agents
        crate::graphics::render::agent_system::agent_render_system(
            sim_ui_state.world,
            &mut self.canvas,
            sim_ui_state.camera.x,
            sim_ui_state.camera.y,
            CELL_SIZE,
        );

        // Draw selected agent path
        crate::graphics::render::selected_agent_path_system::selected_agent_path_render(
            sim_ui_state.world,
            &mut self.canvas,
            sim_ui_state.selected_agent,
            sim_ui_state.camera.x,
            sim_ui_state.camera.y,
            CELL_SIZE,
        );

        // Present the main window
        self.canvas.present();

        // Update cached stats before rendering overlays
        crate::graphics::sim_state::update_cached_stats(
            sim_ui_state.world,
            sim_ui_state.resources,
            &mut sim_ui_state.cached_stats,
        );

        // Render overlays (stats, event log) to their canvases if those windows are enabled
        if let Some(ref mut log_canvas) = self.log_canvas_opt {
            crate::graphics::render::event_log_system::event_log_window_render(
                sim_ui_state.world,
                sim_ui_state.resources,
                log_canvas,
                sim_ui_state.font,
                true, // or toggle based on UI state
            );
            log_canvas.present();
        }
        crate::graphics::render::stats_system::stats_window_render(
            sim_ui_state.world,
            &mut self.stats_canvas,
            sim_ui_state.font,
            &sim_ui_state.cached_stats,
            sim_ui_state.selected_agent,
            false, // or toggle based on UI state
        );
        self.stats_canvas.present();
    }
}

pub struct SdlInput {
    pub event_pump: sdl2::EventPump,
    pub window_id: u32,
}

impl crate::sim_loop_unified::SimulationInput for SdlInput {
    fn handle_input_ui(&mut self,
        sim_ui_state: &mut crate::graphics::sim_state::SimUIState,
        agent_types: &[crate::agent::AgentType],
        render_map: &crate::map::Map,
        cell_size: f32,
        log_config: &crate::log_config::LogConfig,
        paused: &mut bool,
        tick: usize,
    ) {
        crate::graphics::input::collect_input_events(
            &mut self.event_pump,
            self.window_id,
            sim_ui_state,
            agent_types,
            render_map,
            cell_size,
            log_config,
            *paused,
        );
    }
}

pub struct SdlProfiler;

impl crate::sim_loop_unified::SimulationProfiler for SdlProfiler {
    fn on_simulation_end_ui(&mut self, sim_ui_state: &crate::graphics::sim_state::SimUIState, ticks: usize) {
        // TODO: profiling/summary logic for graphics mode
    }
}

// DEPRECATED: Use the unified simulation loop instead of this function.
#[allow(clippy::too_many_arguments)]
pub fn run_sim_render(
    profile: &SimProfile,
    agent_types: &[AgentType],
    profile_systems: bool,
    profile_csv: &str,
    world: &mut World,
    resources: &mut legion::systems::Resources,
) {
    // --- ECS World Setup ---
    let map_width = profile.map_width.or(profile.map_size).unwrap();
    let map_height = profile.map_height.or(profile.map_size).unwrap();
    let num_agents = profile.num_agents;
    // --- NEW SPLIT STAGE INITIALIZATION ---
    let (mut world, mut resources, map) = crate::sim_core::create_world_and_resources(map_width, map_height);
    // [RF6] ECS-driven initialization: Insert InitConfig resource instead of imperative spawn queue mutation
    crate::sim_core::insert_init_config(
        &mut resources,
        agent_types.to_vec(),
        num_agents,
        vec![], // TODO: fill with food spawn positions if needed
        vec![], // TODO: fill with agent spawn positions if needed
    );
    // let agent_count = crate::sim_core::enqueue_initial_spawns(&mut world, &mut resources, &map, num_agents, agent_types, None); // Pass None for spawn_config in graphics mode
    let render_map = map.clone();
    log::debug!("[DEBUG] Agents enqueued: {}", 0);

    // Instead of borrowing LogConfig from resources while resources is mutably borrowed,
    // get LogConfig at the start and pass as a plain reference to downstream functions.
    let log_config = resources.get::<LogConfig>().unwrap().clone();

    // --- Use PARALLEL schedule ---
    // Profiling support
    let mut csv_file = if profile_systems {
        Some(File::create(profile_csv).expect("Failed to create csv file"))
    } else {
        None
    };

    // --- SDL2 CONTEXT AND WINDOW SETUP ---
    // Compute window size based on map size and cell size, but do not exceed defaults
    let map_pixel_width = (map_width as f32 * CELL_SIZE).ceil() as u32;
    let map_pixel_height = (map_height as f32 * CELL_SIZE).ceil() as u32;
    // Set minimum window size (for large maps, keep current default; for small, fit map)
    let default_window_width: u32 = 1280;
    let default_window_height: u32 = 800;
    let window_width = map_pixel_width.max(320).min(default_window_width);
    let window_height = map_pixel_height.max(240).min(default_window_height);

    let (mut canvas, mut stats_canvas, mut log_canvas_opt, mut event_pump, window_id, _stats_window_id, _log_window_id, mut camera, font) =
        crate::graphics::sim_loop::init_sdl2(
            map_width,
            map_height,
            CELL_SIZE,
            window_width,
            window_height,
            &log_config,
        );

    let mut _paused = false;
    let mut _advance_one = false;
    let _ascii_snapshots: Vec<String> = Vec::new();

    // Use the actual schedule builder from ecs_simulation
    let mut schedule = crate::ecs_simulation::build_simulation_schedule_profiled();
    let mut sim_ui_state = SimUIState {
        world: &mut world,
        resources: &mut resources,
        schedule: &mut schedule,
        camera: &mut camera,
        font: &font,
        cached_stats: crate::graphics::sim_state::CachedStats::default(),
        selected_agent: None,
        empty_cell_flash: None,
        tick: 0,
        input_queue: crate::graphics::input_intent::InputQueue::default(),
    };

    // --- MAIN SIMULATION LOOP ---
    let mut renderer = SdlRenderer {
        canvas,
        stats_canvas,
        log_canvas_opt,
    };
    let mut input = SdlInput {
        event_pump,
        window_id,
    };
    let mut profiler = SdlProfiler { };
    run_simulation_loop_ui(
        &mut sim_ui_state,
        1000000, // TODO: Proper tick count or exit condition
        &mut renderer,
        &mut profiler,
        &mut input,
        agent_types,
        &render_map,
        CELL_SIZE,
        &log_config,
        _paused,
    );
    let tick = sim_ui_state.tick;
    use crate::sim_summary::write_simulation_summary_and_ascii;
    write_simulation_summary_and_ascii(
        sim_ui_state.world,
        sim_ui_state.resources,
        &render_map,
        tick as usize,
        "simulation_ascii.txt",
    );
}
