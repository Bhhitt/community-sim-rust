// Food ECS system implementation
// TODO: Move food-related system logic from ecs_sim.rs, ecs_simulation.rs, or food modules here.

use legion::{Entity, IntoQuery, systems::Runnable, systems::SystemBuilder};
use crate::food::Food;
use crate::agent::{event::AgentEvent, event::AgentEventLog};
use crate::ecs_components::{Position, FoodStats};

/// Food Collection System: Handles agent-food interactions, food removal, and event logging.
pub fn food_collection_system() -> impl Runnable {
    SystemBuilder::new("FoodCollectionSystem")
        .write_resource::<FoodStats>()
        .write_resource::<AgentEventLog>()
        .with_query(<(Entity, &Position, &crate::agent::InteractionState)>::query()) // agents
        .with_query(<(Entity, &Position, &Food)>::query()) // food
        .build(|cmd, world, (food_stats, agent_event_log), (agent_query, food_query)| {
            let agents: Vec<_> = agent_query.iter(world).map(|(entity, pos, _)| (*entity, pos.x, pos.y)).collect();
            let foods: Vec<_> = food_query.iter(world).map(|(e, pos, food)| (*e, pos.x, pos.y, food.nutrition)).collect();
            let mut rng = rand::thread_rng();
            let mut food_eaten: Vec<(Entity, Entity, f32)> = Vec::new();
            for (agent_entity, x, y) in &agents {
                // Agent-food interaction (randomize food selection if multiple in range)
                let mut foods_in_range: Vec<_> = foods.iter()
                    .filter(|(_food_e, fx, fy, _nutrition)| (*x - *fx).abs() < 1.0 && (*y - *fy).abs() < 1.0)
                    .collect();
                if !foods_in_range.is_empty() {
                    use rand::seq::SliceRandom;
                    foods_in_range.shuffle(&mut rng);
                    let (food_e, _fx, _fy, nutrition) = *foods_in_range[0];
                    food_eaten.push((*agent_entity, food_e, nutrition));
                }
            }
            // Remove eaten food and log events
            for (agent_entity, food_e, nutrition) in food_eaten {
                agent_event_log.push(AgentEvent::AteFood {
                    agent: agent_entity,
                    food: food_e,
                    nutrition,
                });
                cmd.remove(food_e);
                food_stats.collected_per_tick += 1;
            }
        })
}

// Example:
// pub fn food_system() -> Box<dyn Schedulable> { ... }
