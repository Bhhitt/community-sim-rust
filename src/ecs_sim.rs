//! Minimal ECS simulation loop using new ECS components
use legion::*;
use crate::ecs_components::*;
use crate::food::Food;
use crate::agent::{AgentType, MovementProfile};
use crate::agent::systems::spawn_agent;
use log::debug;
use std::collections::HashMap;
use rand::Rng;

pub fn run_ecs_sim() {
    let mut world = World::default();
    let mut resources = Resources::default();
    // Spawn some agents
    let agent_types = [
        AgentType {
            r#type: "worker".to_string(),
            color: "blue".to_string(),
            move_speed: 1.0,
            move_probability: Some(1.0),
            strength: 1,
            stamina: 1,
            vision: 1,
            work_rate: 1,
            icon: "w".to_string(),
            damping: None,
            name: Some("worker".to_string()),
            movement_profile: MovementProfile { terrain_effects: HashMap::new() },
            decision_engine: None,
        },
        AgentType {
            r#type: "scout".to_string(),
            color: "green".to_string(),
            move_speed: 2.0,
            move_probability: Some(1.0),
            strength: 1,
            stamina: 1,
            vision: 1,
            work_rate: 1,
            icon: "s".to_string(),
            damping: None,
            name: Some("scout".to_string()),
            movement_profile: MovementProfile { terrain_effects: HashMap::new() },
            decision_engine: None,
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
        world.push((pos, crate::food::Food { nutrition }));
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
                            debug!("Agent '{}' at ({}, {})", agent_type.r#type, pos.x, pos.y);
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
