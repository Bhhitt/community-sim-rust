//! ECS system for initial agent/food spawning (RF6)

use legion::{system, IntoQuery, Resources, World, systems::{SystemBuilder, Runnable}};
use crate::ecs::resources::init_config::InitConfig;
use crate::ecs::systems::pending_agent_spawns::PendingAgentSpawns;
use crate::food::PendingFoodSpawns;
use crate::map::Map;
use rand::Rng;
use std::time::Instant;
use log::info;

pub fn initial_spawn_system() -> impl Runnable {
    SystemBuilder::new("initial_spawn")
        .write_resource::<PendingAgentSpawns>()
        .write_resource::<PendingFoodSpawns>()
        .write_resource::<InitConfig>()
        .read_resource::<Map>()
        .build(|_, _, (pending_agents, pending_food, init_config, map), _| {
            let start = std::time::Instant::now();
            // Only run if not already initialized
            if init_config.initialized {
                let duration = start.elapsed();
                info!(target: "ecs_profile", "[PROFILE] System initial_spawn_system took {:?}", duration);
                return;
            }
            // If agent_spawns is empty, generate random spawns
            if init_config.agent_spawns.is_empty() && init_config.num_agents > 0 && !init_config.agent_types.is_empty() {
                let mut rng = rand::thread_rng();
                let width = map.width as f32;
                let height = map.height as f32;
                for _ in 0..init_config.num_agents {
                    let agent_type = init_config.agent_types[rng.gen_range(0..init_config.agent_types.len())].clone();
                    // Find a passable tile
                    let (x, y) = loop {
                        let x = rng.gen_range(0.0..width);
                        let y = rng.gen_range(0.0..height);
                        let tile = map.tiles[y as usize][x as usize];
                        if tile == crate::map::Terrain::Grass || tile == crate::map::Terrain::Forest {
                            break (x, y);
                        }
                    };
                    pending_agents.add(crate::ecs_components::Position { x, y }, agent_type);
                }
            } else {
                // Enqueue explicit agent spawns
                for (x, y, agent_type) in &init_config.agent_spawns {
                    pending_agents.add(crate::ecs_components::Position { x: *x, y: *y }, agent_type.clone());
                }
            }
            // Enqueue food spawns
            for (x, y) in &init_config.food_spawns {
                pending_food.0.push_back((*x, *y));
            }
            // Mark as initialized and clear vectors
            init_config.initialized = true;
            init_config.agent_spawns.clear();
            init_config.food_spawns.clear();
            let duration = start.elapsed();
            info!(target: "ecs_profile", "[PROFILE] System initial_spawn_system took {:?}", duration);
        })
}
