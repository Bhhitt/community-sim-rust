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
use crate::sim_core::{setup_simulation_world_and_resources, SimInit, build_simulation_schedule};
use crate::sim_loop_unified::{SimulationRenderer, SimulationInput, SimulationProfiler, run_simulation_loop, NoOpInput, NoOpProfiler, NoOpRenderer};
use crate::sim_state::SimState;

pub fn run_simulation(
    map_width: i32,
    map_height: i32,
    num_agents: usize,
    ticks: usize,
    label: &str,
    agent_types: &[AgentType],
    _profile_systems: bool,
    _profile_csv: &str,
) -> (f64, f64, f64) {
    log::info!("[TEST] Entered run_simulation");
    log::info!("\n=== Running {}: map {}x{}, {} agents, {} ticks ===", label, map_width, map_height, num_agents, ticks);
    // --- ECS World Setup (MATCH graphics mode) ---
    let SimInit { mut world, mut resources, map, agent_count } = setup_simulation_world_and_resources(
        map_width, map_height, num_agents, agent_types
    );
    let mut rng = rand::thread_rng();
    // Convert agent_types from agent.rs::AgentType to ecs_components::AgentType directly
    let ecs_agent_types: Vec<AgentType> = agent_types.iter().map(|a| AgentType {
        name: a.name.clone(),
        color: a.color,
        movement_profile: a.movement_profile,
        decision_engine: a.decision_engine.clone(),
        hunger_rate: a.hunger_rate,
        hunger_threshold: a.hunger_threshold,
    }).collect();
    let mut attempts = 0;
    let _agent_event_log = AgentEventLog::default();
    if num_agents > 0 {
        for i in 0..num_agents {
            // Find a random passable tile
            let mut x;
            let mut y;
            let mut tries = 0;
            loop {
                x = rng.gen_range(0..map_width) as f32;
                y = rng.gen_range(0..map_height) as f32;
                if map.tiles[y as usize][x as usize] == Terrain::Grass || map.tiles[y as usize][x as usize] == Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for agent after 1000 tries");
                }
            }
            let _agent_type = ecs_agent_types[i % ecs_agent_types.len()].clone();
            // Instead of spawn_agent, queue spawn request for ECS system
            // TODO: Fix or remove usage of undefined value `resources` at line 60
            // This may require passing or initializing the correct resource context.
            // resources.get_mut::<PendingAgentSpawns>().unwrap().add(Position { x, y }, agent_type.clone());
            attempts += tries;
        }
    }
    // --- Spawn initial food entities (1 per 10 agents, minimum 1 if agents exist) ---
    let food_count = if agent_count > 0 { std::cmp::max(1, agent_count / 10) } else { 0 };
    for _ in 0..food_count {
        let mut tries = 0;
        let (mut x, mut y);
        loop {
            x = rng.gen_range(0..map_width) as f32;
            y = rng.gen_range(0..map_height) as f32;
            if map.tiles[y as usize][x as usize] == Terrain::Grass || map.tiles[y as usize][x as usize] == Terrain::Forest {
                break;
            }
            tries += 1;
            if tries > 1000 {
                panic!("Could not find passable tile for food after 1000 tries");
            }
        }
        world.push((Position { x, y }, Food { nutrition: rng.gen_range(5.0..=10.0) }));
    }
    log::debug!("[DEBUG] Total spawn attempts: {} (avg {:.2} per agent)", attempts, attempts as f32 / agent_count as f32);
    let total_entities = world.len();
    log::debug!("[DEBUG] Total entities in world after spawning: {}", total_entities);
    std::io::stdout().flush().unwrap();
    log::debug!("[DEBUG] Spawned {} agents", agent_count);
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
        let (total, move_time, interact_time) = run_simulation(map_width, map_height, num_agents, ticks, label, agent_types, false, "scaling_benchmark.csv");
        log::info!("{}: total {:.3}s, move {:.3}s, interact {:.3}s", label, total, move_time, interact_time);
    }
}

// TODO: Remove unused import
// use crate::graphics::run_with_graphics_profile;
