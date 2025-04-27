//! Unified simulation core logic: world/resource setup for both headless and graphics modes.

use crate::agent::{AgentType, event::AgentEventLog};
use crate::map::{Map, Terrain};
use crate::ecs_components::{Position, InteractionStats, FoodPositions, FoodStats};
use crate::food::{PendingFoodSpawns, Food};
use crate::log_config::LogConfig;
use crate::event_log::EventLog;
use legion::{World, Resources, Schedule};
use rand::Rng;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Result of simulation world/resource setup
pub struct SimInit {
    pub world: World,
    pub resources: Resources,
    pub map: Map,
    pub agent_count: usize,
}

/// Sets up the ECS world, resources, and spawns agents/food for the simulation.
pub fn setup_simulation_world_and_resources(
    map_width: i32,
    map_height: i32,
    num_agents: usize,
    agent_types: &[AgentType],
) -> SimInit {
    let mut world = World::default();
    let map = Map::new(map_width, map_height);
    let mut rng = rand::thread_rng();
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
    if num_agents > 0 {
        use crate::ecs::agent_spawn_queue::AGENT_SPAWN_QUEUE;
        use crate::ecs::systems::pending_agent_spawns::AgentSpawnRequest;
        let mut queue = AGENT_SPAWN_QUEUE.lock().unwrap();
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
            let agent_type = ecs_agent_types[i % ecs_agent_types.len()].clone();
            // Instead of pushing directly to world, enqueue spawn request:
            queue.push(AgentSpawnRequest { pos: Position { x, y }, agent_type });
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
    // === ECS Resource Setup ===
    let mut resources = Resources::default();
    crate::ecs::resources::insert_standard_resources(&mut resources, &map);
    SimInit { world, resources, map, agent_count }
}

/// Returns a Legion Schedule with all standard simulation systems (agent spawning, movement, food, etc.)
pub fn build_simulation_schedule() -> Schedule {
    crate::ecs::schedules::build_main_schedule()
}
