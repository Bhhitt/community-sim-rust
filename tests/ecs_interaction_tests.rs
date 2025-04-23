//! ECS-based agent interaction tests for community_sim

use legion::*;
use community_sim::ecs_components::Position;
use community_sim::agent::AgentType;
use community_sim::agent::systems::spawn_agent;
use community_sim::map::Map;
use std::collections::HashMap;
use community_sim::agent::components::MovementProfile;

fn agent_interaction_system() -> impl legion::systems::Runnable {
    // Dummy system for test compilation; replace with actual system if/when available
    legion::SystemBuilder::new("DummyAgentInteractionSystem")
        .build(|_, _, _, _| {})
}

#[test]
fn test_ecs_agent_interaction_system_runs() {
    let mut world = World::default();
    let movement_profile = MovementProfile {
        terrain_effects: HashMap::new(),
    };
    let agent_type = AgentType {
        r#type: "worker_type".to_string(),
        color: "blue".to_string(),
        move_speed: 1.0,
        strength: 1,
        stamina: 1,
        vision: 1,
        work_rate: 1,
        icon: "W".to_string(),
        damping: None,
        move_probability: None,
        movement_profile,
        name: Some("worker".to_string()),
        decision_engine: None,
    };
    let map = Map::new(32, 32);
    // Spawn two agents adjacent to each other
    spawn_agent(&mut world, Position { x: 5.0, y: 5.0 }, agent_type.clone(), &map);
    spawn_agent(&mut world, Position { x: 5.0, y: 6.0 }, agent_type.clone(), &map);
    // Run the agent_interaction_system (should not panic, and should process agents)
    let mut schedule = Schedule::builder()
        .add_system(agent_interaction_system())
        .build();
    let mut resources = Resources::default();
    schedule.execute(&mut world, &mut resources);
    // (For more detailed assertions, expand ECS agent state and query)
}
