//! ECS-based agent tests for community_sim

use legion::IntoQuery;
use community_sim::ecs_components::Position;
use community_sim::agent::{AgentType, components::{MovementProfile, MovementEffect, DecisionEngineConfig}};
use community_sim::agent::systems::spawn_agent;
use community_sim::map::Map;

#[test]
fn test_ecs_agent_spawn_and_query() {
    let mut world = legion::World::default();
    let movement_profile = MovementProfile {
        speed: 1.0,
        effect: MovementEffect::None,
    };
    let agent_type = AgentType {
        name: "worker".to_string(),
        color: (0, 0, 255), // blue
        movement_profile,
        decision_engine: DecisionEngineConfig::Simple,
    };
    let pos = Position { x: 10.0, y: 20.0 };
    let map = Map::new(32, 32);
    let _entity = spawn_agent(&mut world, pos, agent_type.clone(), &map);
    // Query for the agent entity
    let mut found = false;
    for (pos, agent_type_ref) in <(legion::Read<Position>, legion::Read<AgentType>)>::query().iter(&world) {
        if pos.x == 10.0 && pos.y == 20.0 && agent_type_ref.name.as_str() == "worker" {
            found = true;
        }
    }
    assert!(found, "ECS agent was not spawned or queried correctly");
}
