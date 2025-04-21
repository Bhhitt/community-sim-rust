//! ECS-based agent tests for community_sim

use legion::*;
use community_sim::ecs_components::{spawn_agent, Position, AgentType};

#[test]
fn test_ecs_agent_spawn_and_query() {
    let mut world = World::default();
    let agent_type = AgentType {
        name: "worker",
        move_speed: 1.0,
        move_probability: None,
        color: "blue",
    };
    let pos = Position { x: 10.0, y: 20.0 };
    let _entity = spawn_agent(&mut world, pos, agent_type.clone());
    // Query for the agent entity
    let mut found = false;
    for (pos, agent_type_ref) in <(Read<Position>, Read<AgentType>)>::query().iter(&world) {
        if pos.x == 10.0 && pos.y == 20.0 && agent_type_ref.name == "worker" {
            found = true;
        }
    }
    assert!(found, "ECS agent was not spawned or queried correctly");
}
