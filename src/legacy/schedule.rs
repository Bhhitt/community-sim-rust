// =============================
// LEGACY FILE (DEPRECATED, NOT USED IN PRODUCTION)
// This file contains schedule setup and simulation logic that is no longer referenced by the main codebase.
// All active ECS scheduling is handled in src/ecs/schedules/.
//
// This file will be removed in a future cleanup. Do not use for new development.
// =============================

// LEGACY: This file contains legacy or reference ECS schedule setup code. Not used in the main simulation. See main simulation loop for active schedule.

// ECS schedule setup for Legion ECS
// Define system ordering, stages, and schedule construction here.

use crate::agent::AgentType;
use crate::log_config::LogConfig;
use crate::event_log::EventLog;
use crate::map::{Map, Terrain};
use crate::ecs_simulation::{simulation_tick, build_simulation_schedule_profiled, SystemProfile};
use crate::render_ascii;
use crate::ecs_components::{Position, InteractionStats};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;
use rand::Rng;
use legion::{World, Resources, IntoQuery};
use log;
use serde::{Deserialize, /*Serialize*/};
use serde_yaml;
use crate::agent::event::AgentEventLog;
use crate::ecs::resources::insert_standard_resources;
// use crate::ecs::systems;

use crate::ecs::systems::agent_spawn;
use crate::sim_core::setup_simulation_world_and_resources;
use crate::ecs::schedules::build_main_schedule;

#[derive(Debug, Deserialize)]
pub struct SimProfile {
    pub name: String,
    pub map_width: Option<i32>,
    pub map_height: Option<i32>,
    pub map_size: Option<i32>,
    pub num_agents: usize,
    pub ticks: usize,
    pub benchmark: Option<bool>,
    pub quiet: Option<bool>,
}

pub fn load_profiles_from_yaml(path: &str) -> Vec<SimProfile> {
    let yaml = std::fs::read_to_string(path).expect("Failed to read config/sim_profiles.yaml");
    serde_yaml::from_str(&yaml).expect("Failed to parse config/sim_profiles.yaml")
}

// TODO: Move or re-export run_simulation, load_profiles_from_yaml, etc., as needed for full migration.

/// Runs a single simulation profile (non-GUI), with ECS setup and tick loop. Returns timing info.
pub fn run_simulation(
    map_width: i32,
    map_height: i32,
    num_agents: usize,
    ticks: usize,
    label: &str,
    agent_types: &[AgentType],
    profile_systems: bool,
    profile_csv: &str,
) -> (f64, f64, f64) {
    log::info!("[TEST] Entered run_simulation");
    log::info!("\n=== Running {}: map {}x{}, {} agents, {} ticks ===", label, map_width, map_height, num_agents, ticks);
    // --- Unified ECS World/Resource Setup ---
    let SimInit { mut world, mut resources, map, agent_count } = setup_simulation_world_and_resources(
        map_width, map_height, num_agents, agent_types
    );
    log::debug!("[DEBUG] Spawned {} agents", agent_count);
    let total_entities = world.len();
    log::debug!("[DEBUG] Total entities in world after spawning: {}", total_entities);
    // --- Build ECS Schedule ---
    let mut schedule = build_main_schedule();
    let mut last_ascii = String::new();
    for tick in 0..ticks {
        log::debug!("Tick {}", tick);
        schedule.execute(&mut world, &mut resources);
        // Optionally generate ASCII snapshot at each tick (optional, but we'll save the last)
        last_ascii = crate::render_ascii::render_simulation_ascii(&world, &map);
    }
    // --- Write simulation summary to map file ---
    use legion::IntoQuery;
    use std::collections::HashMap;
    // Count agent types at end
    let mut agent_type_counts: HashMap<String, usize> = HashMap::new();
    let mut agent_query = <(&AgentType,)>::query();
    for (agent_type,) in agent_query.iter(&world) {
        *agent_type_counts.entry(agent_type.name.clone()).or_insert(0) += 1;
    }
    // Get interaction stats
    let stats = resources.get::<InteractionStats>().expect("No InteractionStats resource");
    let total_interactions = stats.agent_interactions;
    let avg_interactions_per_tick = if ticks > 0 { total_interactions as f64 / ticks as f64 } else { 0.0 };
    // Prepare summary string
    let mut summary = String::new();
    summary.push_str(&format!("# Simulation Summary\n"));
    summary.push_str(&format!("Total interactions: {}\n", total_interactions));
    summary.push_str(&format!("Average interactions per tick: {:.2}\n", avg_interactions_per_tick));
    summary.push_str("Agent counts at end:\n");
    for (name, count) in agent_type_counts.iter() {
        summary.push_str(&format!("  {}: {}\n", name, count));
    }
    summary.push_str("\n");
    // Write summary + ascii to file
    let mut file = std::fs::File::create("simulation_ascii.txt").expect("Unable to create ascii output file");
    file.write_all(summary.as_bytes()).expect("Unable to write summary");
    file.write_all(last_ascii.as_bytes()).expect("Unable to write ascii output");
    log::info!("[INFO] Simulation summary and final ASCII snapshot written to simulation_ascii.txt");
    // --- Print map to stdout at end of sim ---
    println!("\n--- FINAL MAP ASCII ---\n{}", last_ascii);
    (0.0, 0.0, 0.0)
}

pub fn run_profiles_from_yaml(path: &str, agent_types: &[AgentType], profile_systems: bool, profile_csv: &str) {
    let profiles = load_profiles_from_yaml(path);
    log::info!("\n===== Simulation Profiles (YAML) =====");
    for profile in profiles {
        let width = profile.map_width.unwrap_or(profile.map_size.unwrap_or(20));
        let height = profile.map_height.unwrap_or(profile.map_size.unwrap_or(20));
        log::info!("Running profile: {} (map {}x{}, {} agents, {} ticks)", profile.name, width, height, profile.num_agents, profile.ticks);
        run_simulation(width, height, profile.num_agents, profile.ticks, &profile.name, agent_types, profile_systems, profile_csv);
    }
}

pub fn run_benchmark_profiles_from_yaml(
    path: &str,
    agent_types: &[AgentType],
    profile_systems: bool,
    profile_csv: &str,
) {
    let profiles = load_profiles_from_yaml(path);
    let mut found = false;
    log::info!("\n===== Benchmark Profiles (YAML) =====");
    for profile in profiles.iter().filter(|p| p.benchmark.unwrap_or(false)) {
        found = true;
        let width = profile.map_width.or(profile.map_size).unwrap_or(20);
        let height = profile.map_height.or(profile.map_size).unwrap_or(20);
        log::info!("Benchmarking profile: {} (map {}x{}, {} agents, {} ticks)", profile.name, width, height, profile.num_agents, profile.ticks);
        run_simulation(width, height, profile.num_agents, profile.ticks, &profile.name, agent_types, profile_systems, profile_csv);
    }
    if !found {
        log::warn!("[WARNING] No profiles with benchmark: true found in YAML. Falling back to hardcoded scaling benchmarks.");
        run_scaling_benchmarks(agent_types);
    }
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
    // TODO: Remove crate::simulation dependency after full migration
    crate::simulation::run_profile_from_yaml(
        path,
        profile_name,
        agent_types,
        profile_systems,
        profile_csv,
        log_config,
        event_log,
    )
}

pub fn run_scaling_benchmarks(agent_types: &[AgentType]) {
    // TODO: Remove crate::simulation dependency after full migration
    crate::simulation::run_scaling_benchmarks(agent_types)
}

// Example (pseudo-code):
// pub fn build_schedule() -> Schedule {
//     Schedule::builder()
//         .add_system(agent_system())
//         .add_system(movement_system())
//         .flush()
//         .add_system(render_system())
//         .build()
// }

// TODO: Move schedule-building logic from ecs_sim.rs, ecs_simulation.rs, or main.rs here.

// TODO: Insert these systems into the main ECS schedule, in the correct order:
// 1. agent_action_decision_system
// 2. agent_target_assignment_system
// 3. agent_path_assignment_system
// 4. agent_state_transition_system
//
// For example:
// schedule_builder.add_system(systems::agent_action_decision_system());
// schedule_builder.add_system(systems::agent_target_assignment_system());
// schedule_builder.add_system(systems::agent_path_assignment_system());
// schedule_builder.add_system(systems::agent_state_transition_system());
