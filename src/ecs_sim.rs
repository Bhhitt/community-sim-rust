//! Minimal ECS simulation loop using new ECS components
use legion::*;
use crate::ecs_components::*;
use crate::Food;
use crate::agent::{AgentType, MovementProfile, MovementEffect, spawn_agent, DecisionEngineConfig};
use log::debug;
use std::collections::HashMap;
use rand::Rng;

pub fn run_ecs_sim() {
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(crate::agent::event::AgentEventLog::default());
    // Spawn some agents
    let agent_types = [
        AgentType {
            name: "worker".to_string(),
            color: (0, 0, 255),
            movement_profile: MovementProfile { speed: 1.0, effect: MovementEffect::None },
            decision_engine: DecisionEngineConfig::Simple,
        },
        AgentType {
            name: "scout".to_string(),
            color: (255, 255, 0),
            movement_profile: MovementProfile { speed: 1.5, effect: MovementEffect::Slowed(0.8) },
            decision_engine: DecisionEngineConfig::Simple,
        },
    ];
    for i in 0..5 {
        let pos = Position { x: i as f32, y: 0.0 };
        let agent_type = agent_types[i % agent_types.len()].clone();
        // NOTE: This is a minimal ECS sim; pass a dummy map for now (or refactor if needed)
        let dummy_map = crate::map::Map::new(10, 10);
        spawn_agent(&mut world, pos, agent_type, &dummy_map);
    }
    // Spawn some food
    for i in 0..3 {
        let pos = Position { x: i as f32, y: 2.0 };
        // Use ECS world directly (not CommandBuffer) for this minimal example
        let nutrition = rand::thread_rng().gen_range(5.0..=10.0);
        world.push((pos, Food { nutrition }));
    }
    // --- Example system: Print all entities and their positions ---
    let mut schedule = Schedule::builder()
        .add_system(
            SystemBuilder::new("PrintEntities")
                .with_query(<(&Position, Option<&Food>, Option<&AgentType>)>::query())
                .build(|_, world, _, query| {
                    for (pos, food, agent_type) in query.iter(world) {
                        if let Some(_food) = food {
                            debug!("Food at ({}, {})", pos.x, pos.y);
                        } else if let Some(agent_type) = agent_type {
                            debug!("Agent '{}' at ({}, {})", agent_type.name, pos.x, pos.y);
                        }
                    }
                })
        )
        .build();
    schedule.execute(&mut world, &mut resources);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ecs_sim() {
        run_ecs_sim();
    }
}
