// DEPRECATED: This module's schedule builder and setup are superseded by the unified simulation loop and schedule in sim_loop_unified.rs and ecs/schedules/mod.rs.
// All simulation logic should be routed through the unified setup and loop.
// This file is retained for reference and will be removed after migration is complete.

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
use crate::ecs::systems::pending_agent_spawns::PendingAgentSpawns;
use crate::spawn_config::SpawnConfig;

/// Result of simulation world/resource setup
pub struct SimInit {
    pub world: World,
    pub resources: Resources,
    pub map: Map,
    pub agent_count: usize,
}

/// Split ECS world/resource setup into explicit stages for clarity and debug hooks.
pub struct SimSetupStages {
    pub world: World,
    pub resources: Resources,
    pub map: Map,
    pub agent_count: usize,
}

/// Stage 1: Create world/resources and map (no entities yet)
pub fn create_world_and_resources(map_width: i32, map_height: i32) -> (World, Resources, Map) {
    log::info!("[INIT] Creating world and resources...");
    let world = World::default();
    let map = Map::new(map_width, map_height);
    let mut resources = Resources::default();
    crate::ecs::resources::insert_standard_resources(&mut resources, &map);
    log::info!("[INIT] World and resources created.");
    (world, resources, map)
}

/// Stage 2: Enqueue initial spawn requests for agents and food
/// [DEPRECATED: Will be replaced by ECS-driven initialization. See RF6.]
#[deprecated(note = "Use ECS-driven initialization systems instead. See plans/RF6.")]
pub fn enqueue_initial_spawns(
    world: &mut World,
    resources: &mut Resources,
    map: &Map,
    num_agents: usize,
    agent_types: &[AgentType],
    spawn_config: Option<&SpawnConfig>,
) -> usize {
    // [DEPRECATED: This imperative initialization logic will be replaced by ECS systems.]
    // [RF6] See plans/RF6/ecs_init_refactor_plan.md
    log::info!("[INIT] Enqueueing initial spawn requests...");
    let mut agent_count = 0;
    // 1. If spawn_config is present, enqueue explicit spawns from config
    if let Some(cfg) = spawn_config {
        if let Some(agent_entries) = &cfg.agents {
            let mut pending_agents = resources.get_mut::<PendingAgentSpawns>().unwrap();
            for entry in agent_entries {
                let agent_type = agent_types.iter().find(|a| a.name == entry.r#type)
                    .expect("Agent type not found").clone();
                let count = entry.count.unwrap_or(1);
                for _ in 0..count {
                    pending_agents.add(
                        crate::ecs_components::Position {
                            x: entry.pos.x as f32,
                            y: entry.pos.y as f32,
                        },
                        agent_type.clone(),
                    );
                    agent_count += 1;
                }
            }
        }
        if let Some(food_entries) = &cfg.food {
            let mut pending_food = resources.get_mut::<PendingFoodSpawns>().unwrap();
            for entry in food_entries {
                let count = entry.count.unwrap_or(1);
                for _ in 0..count {
                    pending_food.0.push_back((entry.pos.x as f32, entry.pos.y as f32));
                }
            }
        }
        // TODO: Items, money, etc.
    }
    // 2. If spawn_config is None or does not cover all agents/food, fall back to procedural/random
    // (existing logic here, but only for remaining agents/food)
    let mut rng = rand::thread_rng();
    let ecs_agent_types: Vec<AgentType> = agent_types.iter().map(|a| AgentType {
        name: a.name.clone(),
        color: a.color,
        movement_profile: a.movement_profile,
        decision_engine: a.decision_engine.clone(),
        hunger_rate: a.hunger_rate,
        hunger_threshold: a.hunger_threshold,
    }).collect();
    let mut attempts = 0;
    // --- Agent spawn queue ---
    if num_agents > agent_count {
        resources.insert(PendingAgentSpawns::default());
        let mut pending_agents = resources.get_mut::<PendingAgentSpawns>().unwrap();
        for i in agent_count..num_agents {
            let mut x;
            let mut y;
            let mut tries = 0;
            loop {
                x = rng.gen_range(0..map.width) as f32;
                y = rng.gen_range(0..map.height) as f32;
                if map.tiles[y as usize][x as usize] == Terrain::Grass || map.tiles[y as usize][x as usize] == Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for agent after 1000 tries");
                }
            }
            let agent_type = ecs_agent_types[i % ecs_agent_types.len()].clone();
            pending_agents.add(Position { x, y }, agent_type);
            agent_count += 1;
            attempts += tries;
        }
        log::info!("[INIT] Enqueued {} agent spawn requests ({} attempts)", agent_count, attempts);
    }
    // --- Food spawn queue ---
    let food_count = if agent_count > 0 { std::cmp::max(1, agent_count / 10) } else { 0 };
    {
        use crate::food::PendingFoodSpawns;
        use legion::systems::Resource;
        resources.insert(PendingFoodSpawns::default());
        let mut pending_food = resources.get_mut::<PendingFoodSpawns>().unwrap();
        for _ in 0..food_count {
            let mut tries = 0;
            let (mut x, mut y);
            loop {
                x = rng.gen_range(0..map.width) as f32;
                y = rng.gen_range(0..map.height) as f32;
                if map.tiles[y as usize][x as usize] == Terrain::Grass || map.tiles[y as usize][x as usize] == Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for food after 1000 tries");
                }
            }
            pending_food.0.push_back((x, y));
        }
        log::info!("[INIT] Enqueued {} food spawn requests", food_count);
    }
    log::info!("[INIT] Initial spawn requests enqueued.");
    agent_count
}

/// Inserts InitConfig resource for ECS-driven initialization (RF6)
pub fn insert_init_config(
    resources: &mut legion::Resources,
    agent_types: Vec<AgentType>,
    num_agents: usize,
    food_spawns: Vec<(f32, f32)>,
    agent_spawns: Vec<(f32, f32, AgentType)>,
) {
    use crate::ecs::resources::init_config::InitConfig;
    resources.insert(InitConfig::new(agent_types, num_agents, food_spawns, agent_spawns));
}

/// Stage 3: Combined legacy entry point for compatibility
/// [DEPRECATED: Will be replaced by ECS-driven initialization. See RF6.]
#[deprecated(note = "Use ECS-driven initialization systems instead. See plans/RF6.")]
pub fn setup_simulation_world_and_resources(
    map_width: i32,
    map_height: i32,
    num_agents: usize,
    agent_types: &[AgentType],
    spawn_config: Option<&SpawnConfig>,
) -> SimInit {
    // [DEPRECATED: This entry point will be replaced by ECS-driven initialization.]
    // [RF6] See plans/RF6/ecs_init_refactor_plan.md
    log::info!("[INIT] Setting up simulation world and resources...");
    let (mut world, mut resources, map) = create_world_and_resources(map_width, map_height);
    let agent_count = enqueue_initial_spawns(&mut world, &mut resources, &map, num_agents, agent_types, spawn_config);
    log::info!("[INIT] Simulation world and resources set up.");
    SimInit { world, resources, map, agent_count }
}

/// Returns a Legion Schedule with all standard simulation systems (agent spawning, movement, food, etc.)
pub fn build_simulation_schedule() -> Schedule {
    crate::ecs::schedules::build_main_schedule()
}
