//! Main simulation loop and logic

use crate::{agent::AgentType, map::Map};
use crate::graphics;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use serde_yaml;
use legion::IntoQuery;
use rand::Rng;
use crate::ecs_components::{spawn_agent, agent_movement_system, entity_interaction_system, collect_food_spawn_positions, food_spawn_apply_system, agent_death_system, PendingFoodSpawns};
use legion::{World, Resources, Schedule};

fn run_simulation(map_size: i32, num_agents: usize, ticks: usize, label: &str, agent_types: &[AgentType]) -> (f64, f64, f64) {
    println!("[TEST] Entered run_simulation");
    println!("\n=== Running {}: map {}x{}, {} agents, {} ticks ===", label, map_size, map_size, num_agents, ticks);
    // --- ECS World Setup (MATCH graphics mode) ---
    let mut world = World::default();
    let map = Map::new(map_size, map_size);
    let mut rng = rand::thread_rng();
    // Convert agent_types from agent.rs::AgentType to ecs_components::AgentType directly
    let ecs_agent_types: Vec<crate::ecs_components::AgentType> = agent_types.iter().map(|a| crate::ecs_components::AgentType {
        name: Box::leak(a.r#type.clone().into_boxed_str()),
        move_speed: a.move_speed,
        move_probability: a.move_probability,
        color: Box::leak(a.color.clone().into_boxed_str()),
    }).collect();
    let mut agent_count = 0;
    let mut attempts = 0;
    if num_agents > 0 {
        for i in 0..num_agents {
            // Find a random passable tile
            let mut x;
            let mut y;
            let mut tries = 0;
            loop {
                x = rng.gen_range(0..map_size) as f32;
                y = rng.gen_range(0..map_size) as f32;
                if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for agent after 1000 tries");
                }
            }
            let agent_type = ecs_agent_types[i % ecs_agent_types.len()].clone();
            spawn_agent(&mut world, crate::ecs_components::Position { x, y }, agent_type);
            agent_count += 1;
            attempts += tries;
        }
    }
    println!("[DEBUG] Total spawn attempts: {} (avg {:.2} per agent)", attempts, attempts as f32 / agent_count as f32);
    let total_entities = world.len();
    println!("[DEBUG] Total entities in world after spawning: {}", total_entities);
    std::io::stdout().flush().unwrap();
    println!("[DEBUG] Spawned {} agents", agent_count);
    // --- DEBUG: Print all entities with Position and their component type names before tick loop ---
    println!("[DEBUG] Entities with Position and their component types before tick loop:");
    let mut query = <(
        legion::Entity,
        &crate::ecs_components::Position,
        Option<&crate::ecs_components::AgentType>,
        Option<&crate::ecs_components::Food>
    )>::query();
    for (entity, _pos, agent_type, food) in query.iter(&world) {
        let mut comps = vec!["Position"];
        if agent_type.is_some() { comps.push("AgentType"); }
        if food.is_some() { comps.push("Food"); }
        println!("  Entity {:?}: [{}]", entity, comps.join(", "));
    }
    // --- END DEBUG ---
    // --- Seed initial food entities to ensure food archetype exists ---
    let initial_food = (map_size * map_size / 20000).max(2);
    for _ in 0..initial_food {
        let mut tries = 0;
        let (mut x, mut y);
        loop {
            x = rng.gen_range(0..map_size) as f32;
            y = rng.gen_range(0..map_size) as f32;
            if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                break;
            }
            tries += 1;
            if tries > 1000 {
                panic!("Could not find passable tile for food after 1000 tries");
            }
        }
        crate::ecs_components::spawn_food(&mut world, crate::ecs_components::Position { x, y });
    }
    println!("[DEBUG] Seeded {} initial food entities", initial_food);
    // --- END FOOD SEED ---
    let start = Instant::now();
    let move_time = 0.0;
    let interact_time = 0.0;
    // --- SETUP SYSTEM SCHEDULE ---
    let mut schedule = Schedule::builder()
        .add_system(agent_movement_system())
        .add_system(entity_interaction_system())
        .add_system(agent_death_system())
        .build();
    let mut food_spawn_apply_schedule = Schedule::builder()
        .add_system(food_spawn_apply_system())
        .build();
    // ECS: Setup Legion resources
    let mut resources = Resources::default();
    resources.insert(map.clone());
    resources.insert(crate::ecs_components::InteractionStats::default());
    resources.insert(crate::ecs_components::EventLog::new(200));
    resources.insert(PendingFoodSpawns(Vec::new()));
    // Silence unused variable warning for agent_types
    let _ = agent_types;
    for tick in 0..ticks {
        println!("Tick {}", tick);
        println!("[DEBUG] Before schedule execution");
        let t1 = Instant::now();
        schedule.execute(&mut world, &mut resources);
        println!("[DEBUG] About to run agent_death_schedule");
        let mut agent_death_schedule = Schedule::builder()
            .add_system(agent_death_system())
            .build();
        agent_death_schedule.execute(&mut world, &mut resources);
        println!("[DEBUG] Finished agent_death_schedule");
        // Collect food spawn positions outside ECS schedule
        {
            let map = resources.get::<crate::map::Map>().unwrap();
            let positions = collect_food_spawn_positions(&world, &map);
            resources.get_mut::<PendingFoodSpawns>().unwrap().0 = positions;
        }
        println!("[DEBUG] About to run food_spawn_apply_schedule");
        food_spawn_apply_schedule.execute(&mut world, &mut resources);
        println!("[DEBUG] Finished food_spawn_apply_schedule");
        let total_elapsed = t1.elapsed().as_secs_f64();
        println!("[DEBUG] After schedule execution");
        println!("[DEBUG] Tick time: {:.6}s", total_elapsed);
        std::io::stdout().flush().unwrap();
    }
    let duration = start.elapsed().as_secs_f64();
    let ascii = map.render_ascii();
    let fname = format!("simulation_map_{}.txt", label);
    let mut file = File::create(&fname).expect("Unable to create file");
    file.write_all(ascii.as_bytes()).expect("Unable to write file");
    println!("ASCII map saved to {} (elapsed: {:.3}s, move: {:.3}s, interact: {:.3}s)", fname, duration, move_time, interact_time);
    (duration, move_time, interact_time)
}

#[derive(Debug, Deserialize)]
pub struct SimProfile {
    pub name: String,
    pub map_size: i32,
    pub num_agents: usize,
    pub ticks: usize,
}

pub fn load_profiles_from_yaml(path: &str) -> Vec<SimProfile> {
    let yaml = fs::read_to_string(path).expect("Failed to read sim_profiles.yaml");
    serde_yaml::from_str(&yaml).expect("Failed to parse sim_profiles.yaml")
}

pub fn run_profiles_from_yaml(path: &str, agent_types: &[AgentType]) {
    let profiles = load_profiles_from_yaml(path);
    println!("\n===== Simulation Profiles (YAML) =====");
    for profile in profiles {
        println!("Running profile: {} (map {}x{}, {} agents, {} ticks)", profile.name, profile.map_size, profile.map_size, profile.num_agents, profile.ticks);
        let (total, move_time, interact_time) = run_simulation(profile.map_size, profile.num_agents, profile.ticks, &profile.name, agent_types);
        println!("{}: total {:.3}s, move {:.3}s, interact {:.3}s", profile.name, total, move_time, interact_time);
    }
}

pub fn run_profile_from_yaml(path: &str, profile_name: &str, agent_types: &[AgentType]) {
    println!("[TEST] Entered run_profile_from_yaml");
    let profiles = load_profiles_from_yaml(path);
    let profile = profiles.into_iter().find(|p| p.name == profile_name)
        .unwrap_or_else(|| panic!("Profile '{}' not found in {}", profile_name, path));
    println!("\n===== Simulation Profile: {} =====", profile.name);
    println!("Launching GUI with profile: {} (map {}x{}, {} agents, {} ticks)", profile.name, profile.map_size, profile.map_size, profile.num_agents, profile.ticks);
    graphics::run_with_graphics_profile(profile.map_size, profile.num_agents, agent_types);
}

pub fn run_gui_with_profile(_path: &str, _profile_name: &str, agent_types: &[crate::agent::AgentType]) {
    println!("[WARNING] run_gui_with_profile is a stub. Use run_with_graphics_profile instead.");
}

pub fn run_profiles(agent_types: &[AgentType]) {
    println!("\n===== Simulation Profiles =====");
    for profile in load_profiles_from_yaml("sim_profiles.yaml") {
        println!("Running profile: {} (map {}x{}, {} agents, {} ticks)", profile.name, profile.map_size, profile.map_size, profile.num_agents, profile.ticks);
        let (total, move_time, interact_time) = run_simulation(profile.map_size, profile.num_agents, profile.ticks, &profile.name, agent_types);
        println!("{}: total {:.3}s, move {:.3}s, interact {:.3}s", profile.name, total, move_time, interact_time);
    }
}

pub fn run_headless(map_size: i32, num_agents: usize, ticks: usize, agent_types: &[AgentType]) {
    let (total, move_time, interact_time) = run_simulation(map_size, num_agents, ticks, "custom", agent_types);
    println!("\nPerformance summary:");
    println!("  Total:    {:.3}s", total);
    println!("  Movement: {:.3}s", move_time);
    println!("  Interact: {:.3}s", interact_time);
}

pub fn run_scaling_benchmarks() {
    let configs = [
        (20, 10, 10, "base"),
        (200, 100, 10, "10x"),
        (400, 400, 10, "20x"),
        (2000, 10000, 10, "100x"),
    ];
    println!("\n===== Scaling Benchmarks =====");
    for &(map_size, num_agents, ticks, label) in &configs {
        let (total, move_time, interact_time) = run_simulation(map_size, num_agents, ticks, label, &[]);
        println!("{}: total {:.3}s, move {:.3}s, interact {:.3}s", label, total, move_time, interact_time);
    }
}
