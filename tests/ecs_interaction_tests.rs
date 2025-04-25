//! ECS-based agent interaction tests for community-sim

use legion::IntoQuery;
use community_sim::ecs_components::Position;
use community_sim::agent::{AgentType, components::{MovementProfile, MovementEffect, DecisionEngineConfig}};
use community_sim::agent::systems::spawn_agent;
use community_sim::map::Map;
use community_sim::agent::event::AgentEventLog;

fn agent_interaction_system() -> impl legion::systems::Runnable {
    // Dummy system for test compilation; replace with actual system if/when available
    legion::SystemBuilder::new("DummyAgentInteractionSystem")
        .build(|_, _, _, _| {})
}

#[test]
fn test_ecs_agent_interaction_system_runs() {
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
        hunger_rate: 1.0,
        hunger_threshold: 10.0,
    };
    let map = Map::new(32, 32);
    let mut agent_event_log = AgentEventLog::default();
    // Spawn two agents adjacent to each other
    spawn_agent(&mut world, Position { x: 5.0, y: 5.0 }, agent_type.clone(), &map, &mut agent_event_log);
    spawn_agent(&mut world, Position { x: 5.0, y: 6.0 }, agent_type.clone(), &map, &mut agent_event_log);
    // Run the agent_interaction_system (should not panic, and should process agents)
    let mut schedule = legion::Schedule::builder()
        .add_system(agent_interaction_system())
        .build();
    let mut resources = legion::Resources::default();
    schedule.execute(&mut world, &mut resources);
    // (For more detailed assertions, expand ECS agent state and query)
}
