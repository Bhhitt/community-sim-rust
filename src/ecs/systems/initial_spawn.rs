//! ECS system for initial agent/food spawning (RF6)

use legion::{system, IntoQuery, Resources, World, systems::{SystemBuilder, Runnable}};
use crate::ecs::resources::init_config::InitConfig;
use crate::ecs::systems::pending_agent_spawns::PendingAgentSpawns;
use crate::food::PendingFoodSpawns;

pub fn initial_spawn_system() -> impl Runnable {
    SystemBuilder::new("initial_spawn")
        .write_resource::<PendingAgentSpawns>()
        .write_resource::<PendingFoodSpawns>()
        .write_resource::<InitConfig>()
        .build(|_, _, (pending_agents, pending_food, init_config), _| {
            // Only run if not already initialized
            if init_config.initialized {
                return;
            }
            // Enqueue agent spawns
            for (x, y, agent_type) in &init_config.agent_spawns {
                pending_agents.add(
                    crate::ecs_components::Position { x: *x, y: *y },
                    agent_type.clone(),
                );
            }
            // Enqueue food spawns
            for (x, y) in &init_config.food_spawns {
                pending_food.0.push_back((*x, *y));
            }
            // Mark as initialized and clear vectors
            init_config.initialized = true;
            init_config.agent_spawns.clear();
            init_config.food_spawns.clear();
        })
}
