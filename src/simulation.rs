//! Main simulation loop and logic

use crate::agent::{AgentType, event::AgentEventLog};
use crate::map::{Map, Terrain};
use crate::ecs_components::{Position, InteractionStats, FoodPositions, FoodStats};
use crate::food::{PendingFoodSpawns, Food};
use crate::ecs_simulation::{simulation_tick, build_simulation_schedule_profiled, SystemProfile};
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

// TODO: Remove or fix unresolved import for pending_agent_spawns
// use crate::ecs::systems::pending_agent_spawns::PendingAgentSpawns;

fn run_simulation(map_width: i32, map_height: i32, num_agents: usize, ticks: usize, label: &str, agent_types: &[AgentType], profile_systems: bool, profile_csv: &str) -> (f64, f64, f64) {
    log::info!("[TEST] Entered run_simulation");
    log::info!("\n=== Running {}: map {}x{}, {} agents, {} ticks ===", label, map_width, map_height, num_agents, ticks);
    // --- ECS World Setup (MATCH graphics mode) ---
    let mut world = World::default();
    let map = Map::new(map_width, map_height);
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
    let mut agent_count = 0;
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
            agent_count += 1;
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
    // --- DEBUG: Print all entities with Position and their component type names before tick loop ---
    log::debug!("[DEBUG] Entities with Position and their component types before tick loop:");
    let mut query = <(
        legion::Entity,
        &Position,
        Option<&AgentType>,
        Option<&Food>
    )>::query();
    for (entity, _pos, agent_type, food) in query.iter(&world) {
        let mut comps = vec!["Position"];
        if agent_type.is_some() { comps.push("AgentType"); }
        if food.is_some() { comps.push("Food"); }
        log::debug!("  Entity {:?}: [{}]", entity, comps.join(", "));
    }
    // --- Main simulation loop ---
    let mut resources = Resources::default();
    resources.insert(map.clone());
    resources.insert(PendingFoodSpawns(VecDeque::new()));
    resources.insert(FoodPositions(Vec::new()));
    resources.insert(FoodStats::default());
    resources.insert(InteractionStats::default());
    resources.insert(Arc::new(Mutex::new(EventLog::new(200))));
    resources.insert(LogConfig::default()); // Insert LogConfig resource
    resources.insert(AgentEventLog::default()); // Ensure AgentEventLog is always present
    // Insert other resources as needed for ECS systems
    if profile_systems {
        let mut csv_file = File::create(profile_csv).expect("Failed to create csv file");
        writeln!(csv_file, "tick,agent_movement,entity_interaction,agent_death,food_spawn_collect,food_spawn_apply").unwrap();
        let mut sum_profile = SystemProfile::new();
        let mut min_profile: Option<SystemProfile> = None;
        let mut max_profile: Option<SystemProfile> = None;
        let mut schedule = build_simulation_schedule_profiled();
        for tick in 0..ticks {
            log::debug!("Tick {}", tick);
            let profile = simulation_tick(
                &mut world,
                &mut resources,
                &mut schedule,
            );
            writeln!(csv_file, "{}{}{}", tick, if tick == 0 { "," } else { "," }, profile.to_csv_row()).unwrap();
            // Optionally render ASCII after ECS update
            if profile_systems {
                let ascii = crate::render_ascii::render_simulation_ascii(&world, &map);
                println!("ASCII after tick {}:\n{}", tick, ascii);
            }
            log::debug!("[PROFILE] agent_movement: {:.6}s, entity_interaction: {:.6}s, agent_death: {:.6}s, food_spawn_collect: {:.6}s, food_spawn_apply: {:.6}s", 
                profile.agent_movement, profile.entity_interaction, profile.agent_death, profile.food_spawn_collect, profile.food_spawn_apply);
            sum_profile.add(&profile);
            min_profile = Some(match min_profile {
                None => profile.clone(),
                Some(mut min) => {
                    min.agent_movement = min.agent_movement.min(profile.agent_movement);
                    min.entity_interaction = min.entity_interaction.min(profile.entity_interaction);
                    min.agent_death = min.agent_death.min(profile.agent_death);
                    min.food_spawn_collect = min.food_spawn_collect.min(profile.food_spawn_collect);
                    min.food_spawn_apply = min.food_spawn_apply.min(profile.food_spawn_apply);
                    min
                }
            });
            max_profile = Some(match max_profile {
                None => profile.clone(),
                Some(mut max) => {
                    max.agent_movement = max.agent_movement.max(profile.agent_movement);
                    max.entity_interaction = max.entity_interaction.max(profile.entity_interaction);
                    max.agent_death = max.agent_death.max(profile.agent_death);
                    max.food_spawn_collect = max.food_spawn_collect.max(profile.food_spawn_collect);
                    max.food_spawn_apply = max.food_spawn_apply.max(profile.food_spawn_apply);
                    max
                }
            });
        }
        let ticks_f = ticks as f64;
        let mut avg_profile = sum_profile.clone();
        avg_profile.div_assign(ticks_f);
        log::debug!("\n=== System Profile Summary ===");
        log::debug!("Average:   agent_movement: {:.6}s, entity_interaction: {:.6}s, agent_death: {:.6}s, food_spawn_collect: {:.6}s, food_spawn_apply: {:.6}s", 
            avg_profile.agent_movement, avg_profile.entity_interaction, avg_profile.agent_death, avg_profile.food_spawn_collect, avg_profile.food_spawn_apply);
        if let Some(min) = min_profile {
            log::debug!("Minimum:   agent_movement: {:.6}s, entity_interaction: {:.6}s, agent_death: {:.6}s, food_spawn_collect: {:.6}s, food_spawn_apply: {:.6}s", 
                min.agent_movement, min.entity_interaction, min.agent_death, min.food_spawn_collect, min.food_spawn_apply);
        }
        if let Some(max) = max_profile {
            log::debug!("Maximum:   agent_movement: {:.6}s, entity_interaction: {:.6}s, agent_death: {:.6}s, food_spawn_collect: {:.6}s, food_spawn_apply: {:.6}s", 
                max.agent_movement, max.entity_interaction, max.agent_death, max.food_spawn_collect, max.food_spawn_apply);
        }
    } else {
        let mut schedule = build_simulation_schedule_profiled();
        let mut last_ascii = String::new();
        for tick in 0..ticks {
            log::debug!("Tick {}", tick);
            simulation_tick(
                &mut world,
                &mut resources,
                &mut schedule,
            );
            // Generate ASCII snapshot at each tick (optional, but we'll save the last)
            last_ascii = crate::render_ascii::render_simulation_ascii(&world, &map);
            // Optionally print: println!("{}", last_ascii);
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
    }
    // Optionally: write last snapshot to file or keep for further processing
    (0.0, 0.0, 0.0)
}

// TODO: Remove or fix unresolved import for pending_agent_spawns
// use crate::ecs::systems::pending_agent_spawns::PendingAgentSpawns;

pub fn run_profile_from_yaml(
    path: &str,
    profile_name: &str,
    agent_types: &[AgentType],
    profile_systems: bool,
    profile_csv: &str,
    log_config: &LogConfig,
    event_log: Arc<Mutex<EventLog>>,
) {
    log::info!("[TEST] Entered run_profile_from_yaml");
    let profiles = crate::ecs::schedule::load_profiles_from_yaml(path);
    let profile = profiles.iter().find(|p| p.name == profile_name)
        .expect("Profile not found in sim_profiles.yaml");
    let width = profile.map_width.or(profile.map_size).unwrap_or(20);
    let height = profile.map_height.or(profile.map_size).unwrap_or(20);
    let num_agents = profile.num_agents;
    let ticks = profile.ticks;
    log::info!("\n===== Simulation Profile: {} =====", profile_name);
    log::info!(
        "Launching GUI with profile: {} (map {}x{}, {} agents, {} ticks)",
        profile_name, width, height, num_agents, ticks
    );
    // Derive quiet mode from YAML or profile name
    let mut log_config = log_config.clone();
    if let Some(quiet) = profile.quiet {
        log_config.quiet = quiet;
    } else if profile.name.ends_with("quiet") {
        log_config.quiet = true;
    }
    crate::graphics::run_with_graphics_profile(
        width,
        height,
        num_agents,
        agent_types,
        profile_systems,
        profile_csv,
        &log_config,
        event_log,
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
