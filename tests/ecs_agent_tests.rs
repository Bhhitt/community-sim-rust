//! ECS-based agent tests for community_sim

use legion::*;
use community_sim::ecs_components::Position;
use community_sim::agent::AgentType;
use community_sim::agent::systems::spawn_agent;
use community_sim::map::Map;
use std::collections::HashMap;
use community_sim::agent::components::MovementProfile;

#[test]
fn test_ecs_agent_spawn_and_query() {
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
    let pos = Position { x: 10.0, y: 20.0 };
    let map = Map::new(32, 32);
    let _entity = spawn_agent(&mut world, pos, agent_type.clone(), &map);
    // Query for the agent entity
    let mut found = false;
    for (pos, agent_type_ref) in <(Read<Position>, Read<AgentType>)>::query().iter(&world) {
        if pos.x == 10.0 && pos.y == 20.0 && agent_type_ref.name.as_deref() == Some("worker") {
            found = true;
        }
    }
    assert!(found, "ECS agent was not spawned or queried correctly");
}
