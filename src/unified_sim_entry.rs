//! Unified simulation entrypoint: always runs the ECS schedule and core logic; rendering/input is a plug-in (headless or graphics)

use crate::agent::AgentType;
use crate::sim_profile::SimProfile;
use crate::ecs::schedules::build_main_schedule;
use crate::sim_loop_unified::{SimulationRenderer, SimulationInput, SimulationProfiler, run_simulation_loop, run_simulation_loop_ui, NoOpRenderer, NoOpInput, NoOpProfiler};
use crate::graphics::sim_render::{SdlRenderer, SdlInput, SdlProfiler};
use crate::graphics::sim_state::SimUIState;
use crate::log_config::LogConfig;
use legion::{World, Resources, Schedule};
use crate::map::Map;
use crate::ecs::resources::{insert_standard_resources, init_config};

/// Sets up world, resources, schedule, and map for the simulation.
pub fn setup_simulation_core(
    profile: &SimProfile,
    agent_types: &[AgentType],
) -> (World, Resources, Schedule) {
    let map_width = profile.map_width.or(profile.map_size).unwrap();
    let map_height = profile.map_height.or(profile.map_size).unwrap();
    let num_agents = profile.num_agents;
    let map = Map::new(map_width, map_height);
    let world = World::default();
    let mut resources = Resources::default();
    insert_standard_resources(&mut resources, &map);
    resources.insert(init_config::InitConfig::new(
        agent_types.to_vec(),
        num_agents,
        vec![],
        vec![],
    ));
    let schedule = build_main_schedule();
    (world, resources, schedule)
}

/// Runs the unified simulation loop, selecting renderer/input as a plug-in.
pub fn run_unified_simulation(
    profile: &SimProfile,
    agent_types: &[AgentType],
    headless: bool,
    ticks: usize,
    log_config: &LogConfig,
) {
    let (world, resources, schedule) = setup_simulation_core(profile, agent_types);
    let mut world = world;
    let mut resources = resources;
    let mut schedule = schedule;
    if headless {
        let mut sim_state = crate::sim_state::SimState::new(&mut world, &mut resources, &mut schedule);
        let mut renderer = NoOpRenderer;
        let mut input = NoOpInput;
        let mut profiler = NoOpProfiler;
        run_simulation_loop(
            &mut sim_state,
            ticks,
            &mut renderer,
            &mut profiler,
            &mut input,
        );
        // --- Print summary and ASCII snapshot at end of headless sim ---
        if let Some(map) = sim_state.resources.get::<Map>() {
            crate::sim_summary::write_simulation_summary_and_ascii(
                sim_state.world,
                sim_state.resources,
                &*map,
                ticks,
                "simulation_ascii.txt",
            );
        } else {
            println!("[WARN] No map found in resources; cannot print ASCII summary.");
        }
    } else {
        // SDL2/graphics mode
        let map_width = profile.map_width.or(profile.map_size).unwrap();
        let map_height = profile.map_height.or(profile.map_size).unwrap();
        let log_config = log_config.clone();
        let (canvas, stats_canvas, log_canvas_opt, event_pump, window_id, _stats_window_id, _log_window_id, camera, font) =
            crate::graphics::sim_render::init_sdl2(
                map_width, map_height, 6.0, 1280, 800, &log_config);
        let mut schedule = build_main_schedule();
        let mut sim_ui_state = SimUIState {
            world: &mut world,
            resources: &mut resources,
            schedule: &mut schedule,
            camera: Box::leak(Box::new(camera)),
            font: font, // font: &'static Font<'static, 'static>
            cached_stats: crate::graphics::sim_state::CachedStats::default(),
            selected_agent: None,
            empty_cell_flash: None,
            tick: 0,
            input_queue: crate::graphics::input_intent::InputQueue::default(),
        };
        let mut renderer = SdlRenderer { canvas, stats_canvas, log_canvas_opt };
        let mut input = SdlInput { event_pump, window_id };
        let mut profiler = SdlProfiler {};
        let render_map = {
            let render_map_ref = sim_ui_state.resources.get::<Map>().expect("Map not found in resources");
            render_map_ref.clone()
        };
        let cell_size = 6.0; // TODO: Use actual cell size if variable
        let paused = false;
        run_simulation_loop_ui(
            &mut sim_ui_state,
            ticks,
            &mut renderer,
            &mut profiler,
            &mut input,
            agent_types,
            &render_map,
            cell_size,
            &log_config,
            paused,
        );
    }
}
