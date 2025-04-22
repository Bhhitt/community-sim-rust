//! ECS-based agent interaction tests for community_sim

use legion::*;
use community_sim::ecs_components::{spawn_agent, Position, AgentType, agent_interaction_system};
use community_sim::map::Map;

#[test]
fn test_ecs_agent_interaction_system_runs() {
    let mut world = World::default();
    let agent_type = AgentType {
        name: "worker".to_string(),
        move_speed: 1.0,
        move_probability: None,
        color: "blue".to_string(),
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
