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

// Main SDL2 rendering/event loop, extracted from graphics.rs
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
    let map = crate::map::Map::new(map_width, map_height);
    let render_map = map.clone();
    let mut rng = rand::thread_rng();
    let mut _agent_count = 0;
    let mut _attempts = 0;

    // Insert standard resources before any resource access!
    insert_standard_resources(resources, &map);

    if num_agents > 0 {
        let mut passable_count = 0;
        for row in &map.tiles {
            for tile in row {
                if *tile == crate::map::Terrain::Grass || *tile == crate::map::Terrain::Forest {
                    passable_count += 1;
                }
            }
        }
        log::debug!("[DIAG] Passable tiles: {}, Agents to spawn: {}", passable_count, num_agents);
        for i in 0..num_agents {
            let mut x;
            let mut y;
            let mut tries = 0;
            loop {
                x = rng.gen_range(0..map_width) as f32;
                y = rng.gen_range(0..map_height) as f32;
                if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for agent after 1000 tries");
                }
            }
            let agent_type = agent_types[i % agent_types.len()].clone();
            // In your render logic, replace:
            // AGENT_SPAWN_QUEUE.lock().unwrap().push(...);
            // With:
            // let mut pending_spawns = resources.get_mut::<PendingAgentSpawns>().unwrap();
            // pending_spawns.add(pos, agent_type);
            //
            // You may need to pass Resources to the render logic or refactor this to run as an ECS system.
            // let mut pending_spawns = resources.get_mut::<PendingAgentSpawns>().unwrap();
            // pending_spawns.add(crate::ecs_components::Position { x, y }, agent_type.clone());
            _agent_count += 1;
            _attempts += tries;
        }
    }
    let agent_count_check = <(Read<crate::ecs_components::Position>,)>::query().iter(world).count();
    log::debug!("[DEBUG] Number of agents spawned: {}", agent_count_check);

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
        world,
        resources,
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
    struct SdlRenderer {
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
        stats_canvas: sdl2::render::Canvas<sdl2::video::Window>,
        log_canvas_opt: Option<sdl2::render::Canvas<sdl2::video::Window>>,
    }
    impl SimulationRenderer for SdlRenderer {
        fn render_ui(&mut self, sim_ui_state: &mut SimUIState, tick: usize) {
            // TODO: Move SDL2 rendering logic here, using sim_ui_state
        }
    }
    struct SdlInput {
        event_pump: sdl2::EventPump,
        window_id: u32,
    }
    impl SimulationInput for SdlInput {
        fn handle_input_ui(&mut self, sim_ui_state: &mut SimUIState, tick: usize) {
            // TODO: Move SDL2 input logic here, using sim_ui_state
        }
    }
    struct SdlProfiler { }
    impl SimulationProfiler for SdlProfiler {
        fn on_simulation_end_ui(&mut self, sim_ui_state: &SimUIState, ticks: usize) {
            // TODO: profiling/summary logic for graphics mode
        }
    }
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
