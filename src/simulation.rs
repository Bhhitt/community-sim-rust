//! Main simulation loop and logic

use crate::{agent::AgentType, map::Map};
use crate::graphics;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use serde_yaml;

fn run_simulation(map_size: i32, num_agents: usize, ticks: usize, label: &str, agent_types: &[AgentType]) -> (f64, f64, f64) {
    println!("\n=== Running {}: map {}x{}, {} agents, {} ticks ===", label, map_size, map_size, num_agents, ticks);
    // Cycle through agent_types for agent creation
    // ECS MIGRATION: Create Legion World and spawn ECS agents
    use crate::ecs_components::{spawn_agent, Position as ECSPosition, AgentType as ECSAgentType, agent_movement_system, agent_interaction_system, food_spawn_system, agent_death_system};
    use legion::*;
    let mut world = World::default();
    // Spawn ECS agents using Legion
    for i in 0..num_agents {
        let t = &agent_types[i % agent_types.len()];
        // Map legacy AgentType to ECSAgentType (convert as needed)
        let ecs_type = ECSAgentType {
            name: Box::leak(t.r#type.clone().into_boxed_str()),
            move_speed: t.move_speed,
            color: Box::leak(t.color.clone().into_boxed_str()),
        };
        let pos = ECSPosition { x: (i as i32 % map_size) as f32, y: (i as i32 / map_size) as f32 };
        spawn_agent(&mut world, pos, ecs_type);
    }
    // --- Legacy agent Vec and logic removed ---
    let map = Map::new(map_size, map_size);
    let start = Instant::now();
    let mut move_time = 0.0;
    let mut interact_time = 0.0;
    // ECS: Setup Legion schedule for agent movement, interaction, and food spawn
    let mut resources = Resources::default();
    resources.insert(map.clone());
    let mut schedule = Schedule::builder()
        .add_system(agent_movement_system())
        .add_system(agent_interaction_system())
        .add_system(food_spawn_system())
        .add_system(agent_death_system())
        .build();
    for tick in 0..ticks {
        println!("Tick {}", tick);
        let t1 = Instant::now();
        // ECS: Run agent movement, interaction, and food spawn systems
        schedule.execute(&mut world, &mut resources);
        move_time += t1.elapsed().as_secs_f64();
        let t2 = Instant::now();
        interact_time += t2.elapsed().as_secs_f64();
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
