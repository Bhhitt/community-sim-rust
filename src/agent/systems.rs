// Agent-related ECS systems will be moved here.

// NOTE: The agent_pause_system is no longer used or implemented here.
// If the parameter limit is ever increased or the query simplified, consider consolidating for clarity. See pause_system.rs for details.

use legion::{Entity, IntoQuery};

// --- ECS Agent Death System ---
pub fn agent_death_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDeathSystem")
        .with_query(<(Entity, &crate::agent::Hunger, &crate::agent::Energy)>::query())
        .build(|cmd, world, _, query| {
            let mut to_remove = Vec::new();
            for (entity, hunger, energy) in query.iter(world) {
                if hunger.value <= 0.0 || energy.value <= 0.0 {
                    to_remove.push(entity);
                }
            }
            for entity in to_remove {
                cmd.remove(*entity);
            }
        })
}

fn find_closest_food(x: f32, y: f32, food_positions: &crate::ecs_components::FoodPositions) -> Option<(i32, i32)> {
    food_positions.0.iter()
        .map(|&(fx, fy)| (fx as i32, fy as i32, ((fx - x).powi(2) + (fy - y).powi(2)).sqrt()))
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .map(|(fx, fy, _)| (fx, fy))
}
