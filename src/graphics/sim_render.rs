// Unified Event Log Access Pattern: ALWAYS access EventLog via ECS resources as Arc<Mutex<EventLog>>.
// Do not pass EventLog as a direct argument. This ensures a single, obvious pattern for all systems.

// Main simulation rendering and event loop
// Will contain the main SDL2 rendering logic and event loop

use legion::*;
use crate::agent::{AgentType, spawn_agent};
use rand::Rng;
use crate::log_config::LogConfig;
use std::fs::File;
use crate::graphics::sim_state::SimUIState;
use crate::ecs_simulation::{build_simulation_schedule_profiled, build_simulation_schedule_unprofiled};
use crate::agent::event::AgentEventLog;
use crate::ecs::resources::insert_standard_resources;

const CELL_SIZE: f32 = 6.0;

// All unused imports removed for a clean build

// Main SDL2 rendering/event loop, extracted from graphics.rs
#[allow(clippy::too_many_arguments)]
pub fn run_sim_render(
    _map_width: i32,
    _map_height: i32,
    _num_agents: usize,
    agent_types: &[AgentType],
    profile_systems: bool,
    profile_csv: &str,
    world: &mut World,
    resources: &mut Resources,
) {
    // --- ECS World Setup ---
    let map = crate::map::Map::new(_map_width, _map_height);
    let render_map = map.clone();
    let mut rng = rand::thread_rng();
    let mut _agent_count = 0;
    let mut _attempts = 0;
    if _num_agents > 0 {
        for i in 0.._num_agents {
            let mut x;
            let mut y;
            let mut tries = 0;
            loop {
                x = rng.gen_range(0.._map_width) as f32;
                y = rng.gen_range(0.._map_height) as f32;
                if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for agent after 1000 tries");
                }
            }
            let agent_type = agent_types[i % agent_types.len()].clone();
            let mut agent_event_log = resources.get_mut::<AgentEventLog>().expect("AgentEventLog missing");
            spawn_agent(world, crate::ecs_components::Position { x, y }, agent_type, &map, &mut *agent_event_log);
            _agent_count += 1;
            _attempts += tries;
        }
    }
    let agent_count_check = <(Read<crate::ecs_components::Position>,)>::query().iter(world).count();
    log::debug!("[DEBUG] Number of agents spawned: {}", agent_count_check);
    insert_standard_resources(resources, &map);

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
    let map_pixel_width = (_map_width as f32 * CELL_SIZE).ceil() as u32;
    let map_pixel_height = (_map_height as f32 * CELL_SIZE).ceil() as u32;
    // Set minimum window size (for large maps, keep current default; for small, fit map)
    let default_window_width: u32 = 1280;
    let default_window_height: u32 = 800;
    let window_width = map_pixel_width.max(320).min(default_window_width);
    let window_height = map_pixel_height.max(240).min(default_window_height);

    let (mut canvas, mut stats_canvas, mut log_canvas_opt, mut event_pump, window_id, _stats_window_id, _log_window_id, mut camera, font) =
        crate::graphics::sim_loop::init_sdl2(
            _map_width,
            _map_height,
            CELL_SIZE,
            window_width,
            window_height,
            &log_config,
        );

    let mut _paused = false;
    let mut _advance_one = false;
    let _ascii_snapshots: Vec<String> = Vec::new();
    let mut sim_ui_state = SimUIState {
        world,
        resources,
        schedule: if profile_systems {
            &mut build_simulation_schedule_profiled()
        } else {
            &mut build_simulation_schedule_unprofiled()
        },
        camera: &mut camera,
        font: &font,
        cached_stats: crate::graphics::sim_state::CachedStats::default(),
        selected_agent: None,
        empty_cell_flash: None,
        tick: 0,
        input_queue: crate::graphics::input_intent::InputQueue::default(),
    };

    // --- MAIN SIMULATION LOOP ---
    crate::graphics::sim_loop::main_sim_loop(
        &mut sim_ui_state,
        &mut canvas,
        &mut stats_canvas,
        &mut log_canvas_opt,
        &mut event_pump,
        window_id,
        agent_types,
        &render_map,
        &log_config,
        profile_systems,
        &mut csv_file,
        _map_width,
        _map_height,
        CELL_SIZE,
        window_width,
        window_height,
    );

//     use crate::graphics::input::handle_events;
//     handle_events(
//         &mut event_pump,
//         window_id,
//         &mut sim_ui_state,
//         agent_types,
//         &render_map,
//         CELL_SIZE,
//         &resources.get::<LogConfig>().unwrap(),
//         &mut paused,
//         &mut advance_one,
//     );

    // --- At end of simulation, write summary to simulation_ascii.txt ---
    // Drop sim_ui_state to release mutable borrows before summary
    let tick = sim_ui_state.tick;
    drop(sim_ui_state);
    use crate::sim_summary::write_simulation_summary_and_ascii;
    write_simulation_summary_and_ascii(
        world,
        resources,
        &render_map,
        tick as usize,
        "simulation_ascii.txt",
    );
}
