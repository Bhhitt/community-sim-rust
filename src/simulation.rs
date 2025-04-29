// DEPRECATED: This module's simulation entrypoint is superseded by the unified simulation loop and setup in main.rs and sim_loop_unified.rs.
// All core simulation logic should be routed through the unified setup and loop.
// This file is retained for reference and will be removed after migration is complete.

//! Main simulation loop and logic

use crate::agent::{AgentType, event::AgentEventLog};
use crate::map::{Map, Terrain};
use crate::ecs_components::{Position, InteractionStats, FoodPositions, FoodStats};
use crate::food::{PendingFoodSpawns, Food};
// TODO: Refactor to use new ECS schedule/tick logic. The following import is legacy:
// use crate::ecs_simulation::{simulation_tick, build_simulation_schedule_profiled, SystemProfile};
use crate::log_config::LogConfig;
use crate::event_log::EventLog;
// use serde::Deserialize;
use std::io::Write;
use std::fs::File;
use legion::IntoQuery;
use rand::Rng;
use legion::{World, Resources};
use log;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::sim_core::{create_world_and_resources, enqueue_initial_spawns, build_simulation_schedule, insert_init_config, SimInit};
use crate::spawn_config::SpawnConfig;
use crate::sim_loop_unified::{SimulationRenderer, SimulationInput, SimulationProfiler, run_simulation_loop, NoOpInput, NoOpProfiler, NoOpRenderer};
use crate::sim_state::SimState;

// DEPRECATED: Use the unified simulation loop instead of this function.
pub fn run_simulation(
    map_width: i32,
    map_height: i32,
    num_agents: usize,
    ticks: usize,
    label: &str,
    agent_types: &[AgentType],
    _profile_systems: bool,
    _profile_csv: &str,
    spawn_config: Option<&SpawnConfig>,
) -> (f64, f64, f64) {
    log::info!("[TEST] Entered run_simulation");
    log::info!("\n=== Running {}: map {}x{}, {} agents, {} ticks ===", label, map_width, map_height, num_agents, ticks);
    // --- ECS World Setup (MATCH graphics mode) ---
    // [RF6] ECS-driven initialization: use insert_init_config instead of setup_simulation_world_and_resources
    let (mut world, mut resources, map) = create_world_and_resources(map_width, map_height);
    // Extract food spawns from spawn_config if provided, else use empty vec
    let food_spawns = if let Some(cfg) = spawn_config {
        if let Some(food_entries) = &cfg.food {
            food_entries.iter().flat_map(|entry| {
                let count = entry.count.unwrap_or(1);
                std::iter::repeat((entry.pos.x as f32, entry.pos.y as f32)).take(count)
            }).collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };
    insert_init_config(
        &mut resources,
        agent_types.to_vec(),
        num_agents,
        food_spawns,
        {
            // --- Generate agent_spawns with random positions/types ---
            // TODO: Refactor this logic into a reusable helper for both initial and dynamic (GUI-triggered) agent spawning.
            // In the future, agent spawning should be triggered from GUI input by enqueuing spawn requests.
            let mut rng = rand::thread_rng();
            let mut agent_spawns = Vec::with_capacity(num_agents);
            for _ in 0..num_agents {
                let mut tries = 0;
                let (x, y) = loop {
                    let x = rng.gen_range(0..map.width) as f32;
                    let y = rng.gen_range(0..map.height) as f32;
                    if map.tiles[y as usize][x as usize] == Terrain::Grass || map.tiles[y as usize][x as usize] == Terrain::Forest {
                        break (x, y);
                    }
                    tries += 1;
                    if tries > 1000 {
                        panic!("Could not find passable tile for agent after 1000 tries");
                    }
                };
                let agent_type = agent_types[rng.gen_range(0..agent_types.len())].clone();
                agent_spawns.push((x, y, agent_type));
            }
            agent_spawns
        },
    );
    std::io::stdout().flush().unwrap();
    log::debug!("[DEBUG] Spawned {} agents", num_agents);
    // --- ECS Schedule Setup ---
    let mut schedule = build_simulation_schedule();
    // Use unified simulation state for headless mode
    let mut sim_state = SimState::new(&mut world, &mut resources, &mut schedule);
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
    // --- Write simulation summary and ASCII snapshot at end of headless sim ---
    use crate::sim_summary::write_simulation_summary_and_ascii;
    write_simulation_summary_and_ascii(
        sim_state.world,
        sim_state.resources,
        &map,
        ticks,
        "simulation_ascii.txt",
    );
    (ticks as f64, 0.0, 0.0)
}

pub fn run_profile_from_yaml(
    path: &str,
    profile_name: &str,
    agent_types: &[AgentType],
    profile_systems: bool,
    profile_csv: &str,
    log_config: &LogConfig,
    event_log: Arc<Mutex<EventLog>>,
) {
    use crate::sim_profile::{load_profiles_from_yaml, find_profile};
    log::info!("[TEST] Entered run_profile_from_yaml");
    let profiles = load_profiles_from_yaml(path);
    let profile = match find_profile(&profiles, profile_name) {
        Some(p) => p,
        None => {
            log::error!("Profile '{}' not found in {}. Aborting simulation.", profile_name, path);
            return;
        }
    };
    log::info!(
        "Launching simulation with profile: {} (map {}x{}, {} agents, {} ticks)",
        profile.name,
        profile.map_width.unwrap_or(0),
        profile.map_height.unwrap_or(0),
        profile.num_agents,
        profile.ticks
    );
    let mut world = legion::World::default();
    let mut resources = legion::Resources::default();
    resources.insert(log_config.clone());
    resources.insert(event_log);
    crate::graphics::sim_render::run_sim_render(
        profile,
        agent_types,
        profile_systems,
        profile_csv,
        &mut world,
        &mut resources,
    );
}

pub fn run_scaling_benchmarks(agent_types: &[AgentType]) {
    let configs = [
        (20, 20, 10, 10, "base"),        // Smallest, 10 ticks
        (200, 200, 100, 100, "10x"),     // Medium, 100 ticks
        (400, 400, 400, 500, "20x"),     // Large, 500 ticks
        (2000, 2000, 10000, 1000, "100x"), // Very large, 1000 ticks
    ];
    log::info!("\n===== Scaling Benchmarks =====");
    for &(map_width, map_height, num_agents, ticks, label) in &configs {
        let (total, move_time, interact_time) = run_simulation(map_width, map_height, num_agents, ticks, label, agent_types, false, "scaling_benchmark.csv", None);
        log::info!("{}: total {:.3}s, move {:.3}s, interact {:.3}s", label, total, move_time, interact_time);
    }
}

// TODO: Remove unused import
// use crate::graphics::run_with_graphics_profile;
