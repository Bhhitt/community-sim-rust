// Agent-related ECS systems will be moved here next.

use crate::agent::components::{AgentType, Hunger, Energy, InteractionState};
use crate::navigation::{Target, Path};
use crate::ecs_components::{Position, FoodPositions, EventLog};
use crate::map::Map;
use legion::*;

pub fn spawn_agent(world: &mut legion::World, pos: crate::ecs_components::Position, agent_type: crate::agent::components::AgentType, map: &crate::map::Map) -> legion::Entity {
    let _color = agent_type.color.clone();
    let mut rng = rand::thread_rng();
    let (tx, ty) = crate::navigation::random_passable_target(map, &agent_type, &mut rng);
    world.push((pos, agent_type, crate::agent::components::Hunger { value: 100.0 }, crate::agent::components::Energy { value: 100.0 }, crate::agent::components::InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0 }, crate::navigation::Target { x: tx, y: ty, stuck_ticks: 0, path_ticks: None, ticks_to_reach: None }, crate::navigation::Path { waypoints: std::collections::VecDeque::new() }))
}

// --- ECS Agent Movement System ---
pub fn agent_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementSystem")
        .with_query(<(&mut Position, &AgentType, &mut Hunger, &mut Energy, Option<&mut Target>, Option<&mut Path>, Option<&mut InteractionState>)>::query())
        .read_resource::<Map>()
        .read_resource::<FoodPositions>()
        .write_resource::<EventLog>()
        .build(|_, world, (map, food_positions, event_log), query| {
            for (pos, agent_type, hunger, energy, maybe_target, _maybe_path, maybe_interaction) in query.iter_mut(world) {
                let mut arrived = false;
                if let Some(target) = maybe_target {
                    let dx = target.x - pos.x;
                    let dy = target.y - pos.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist > 0.01 {
                        let speed = agent_type.move_speed.max(0.1);
                        let step = speed.min(dist);
                        pos.x += dx / dist * step;
                        pos.y += dy / dist * step;
                        hunger.value -= 0.01 * step;
                        energy.value -= 0.01 * step;
                        event_log.log(format!("[MOVE] Agent moved to ({:.2}, {:.2})", pos.x, pos.y));
                    } else {
                        arrived = true;
                    }
                }
                // If arrived, trigger interaction/searching/other state changes
                if arrived {
                    if let Some(interaction) = maybe_interaction {
                        // Example: set a cooldown, clear target, or increment ticks
                        interaction.cooldown = 10;
                        interaction.ticks += 1;
                        event_log.log(format!("[ARRIVE] Agent at ({:.2}, {:.2}) triggered interaction/state change", pos.x, pos.y));
                    }
                    // Optionally, clear the agent's target to stop moving, or assign a new one for wandering
                    // if let Some(target) = maybe_target { /* clear or assign new target logic here */ }
                }
            }
        })
}

// --- ECS Agent Death System ---
pub fn agent_death_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDeathSystem")
        .with_query(<(legion::Entity, &Hunger, &Energy)>::query())
        .build(|cmd, _world, _, _query| {
            let mut to_remove = Vec::new();
            for (entity, hunger, energy) in _query.iter(_world) {
                if hunger.value <= 0.0 || energy.value <= 0.0 {
                    to_remove.push(entity);
                }
            }
            for entity in to_remove {
                cmd.remove(*entity);
            }
        })
}
