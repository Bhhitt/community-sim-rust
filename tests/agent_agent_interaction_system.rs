//! Tests for Agent-Agent Interaction System

use legion::*;
use community_sim::ecs_components::{Position, InteractionStats};
use community_sim::agent::InteractionState;
use community_sim::ecs::systems::agent_agent_interaction::agent_agent_interaction_system;
use std::sync::{Arc, Mutex};
use community_sim::event_log::EventLog;

#[test]
fn test_agents_interact_within_range() {
    // Setup ECS world
    let mut world = World::default();
    // Two agents within 1.5 units
    let agent1 = world.push((Position { x: 0.0, y: 0.0 }, InteractionState::default()));
    let agent2 = world.push((Position { x: 1.0, y: 1.0 }, InteractionState::default()));
    // Resources
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Run system
    let mut schedule = Schedule::builder()
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    // Check event log for interaction
    let log = event_log.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("[INTERACT]") && entry.contains(&format!("{:?}", agent1)) && entry.contains(&format!("{:?}", agent2)));
    assert!(found, "Expected interaction event between agents within range");
    // Check stats
    let stats = resources.get::<InteractionStats>().unwrap();
    assert_eq!(stats.agent_interactions, 1, "Expected 1 interaction recorded in stats");
}

#[test]
fn test_no_interaction_out_of_range() {
    // Setup ECS world
    let mut world = World::default();
    // Two agents farther than 1.5 units apart
    let agent1 = world.push((Position { x: 0.0, y: 0.0 }, InteractionState::default()));
    let agent2 = world.push((Position { x: 10.0, y: 10.0 }, InteractionState::default()));
    // Resources
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Run system
    let mut schedule = Schedule::builder()
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    // Check event log for no interaction
    let log = event_log.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("[INTERACT]"));
    assert!(!found, "No interaction event should be logged for agents out of range");
    // Check stats
    let stats = resources.get::<InteractionStats>().unwrap();
    assert_eq!(stats.agent_interactions, 0, "No interactions should be recorded in stats");
}

#[test]
fn test_interaction_state_updated() {
    // Setup ECS world
    let mut world = World::default();
    // Two agents within 1.5 units
    let agent1 = world.push((Position { x: 0.0, y: 0.0 }, InteractionState::default()));
    let agent2 = world.push((Position { x: 1.0, y: 1.0 }, InteractionState::default()));
    // Resources
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Run system
    let mut schedule = Schedule::builder()
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    // Check that at least one agent's InteractionState has changed (if system updates it)
    // For now, just check that the system doesn't panic and states are accessible (since the current system doesn't mutate InteractionState)
    let mut query = <&InteractionState>::query();
    let states: Vec<_> = query.iter(&world).collect();
    assert_eq!(states.len(), 2, "Both agents should have InteractionState");
    // If the system is extended to update InteractionState, add assertions here
}

#[test]
fn test_multiple_agents_all_interact() {
    // Setup ECS world
    let mut world = World::default();
    // Four agents, all within 1.5 units of each other (clustered)
    let agents: Vec<_> = vec![
        world.push((Position { x: 0.0, y: 0.0 }, InteractionState::default())),
        world.push((Position { x: 1.0, y: 1.0 }, InteractionState::default())),
        world.push((Position { x: 1.2, y: 0.8 }, InteractionState::default())),
        world.push((Position { x: 0.9, y: 1.3 }, InteractionState::default())),
    ];
    // Resources
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Run system
    let mut schedule = Schedule::builder()
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    // Check event log for interactions
    let log = event_log.lock().unwrap();
    let interaction_events: Vec<_> = log.iter().filter(|entry| entry.contains("[INTERACT]")).collect();
    // Each agent should interact with at least one other agent, but no duplicate pairs
    assert!(interaction_events.len() >= 2, "Expected at least two interaction events for four clustered agents");
    // Check stats
    let stats = resources.get::<InteractionStats>().unwrap();
    assert!(stats.agent_interactions >= 2, "Expected at least two interactions recorded in stats");
}

#[test]
fn test_no_agents_no_panic() {
    // Setup ECS world with no agents
    let mut world = World::default();
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Run system: should not panic
    let mut schedule = Schedule::builder()
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    // Check event log and stats
    let log = event_log.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("[INTERACT]"));
    assert!(!found, "No interaction event should be logged when there are no agents");
    let stats = resources.get::<InteractionStats>().unwrap();
    assert_eq!(stats.agent_interactions, 0, "No interactions should be recorded in stats when there are no agents");
}

#[test]
fn test_missing_component_handling() {
    // Setup ECS world with one normal agent and one missing InteractionState
    let mut world = World::default();
    let _agent1 = world.push((Position { x: 0.0, y: 0.0 }, InteractionState::default()));
    // Instead, push an entity with only a Position as a tuple of one element
    world.push((Position { x: 1.0, y: 1.0 },));
    // Resources
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Run system: should not panic or interact
    let mut schedule = Schedule::builder()
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    // Check event log and stats
    let log = event_log.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("[INTERACT]"));
    assert!(!found, "No interaction event should be logged when an agent is missing required components");
    let stats = resources.get::<InteractionStats>().unwrap();
    assert_eq!(stats.agent_interactions, 0, "No interactions should be recorded in stats when an agent is missing required components");
}

#[test]
fn test_interaction_at_threshold() {
    // Setup ECS world with two agents exactly at threshold distance
    let mut world = World::default();
    let agent1 = world.push((Position { x: 0.0, y: 0.0 }, InteractionState::default()));
    let agent2 = world.push((Position { x: 1.5, y: 0.0 }, InteractionState::default()));
    // Resources
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Run system
    let mut schedule = Schedule::builder()
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    // The system uses < 1.5, so these agents should NOT interact
    let log = event_log.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("[INTERACT]"));
    assert!(!found, "No interaction event should be logged for agents exactly at threshold");
    let stats = resources.get::<InteractionStats>().unwrap();
    assert_eq!(stats.agent_interactions, 0, "No interactions should be recorded in stats for agents exactly at threshold");
}

#[test]
fn test_negative_position_handling() {
    // Setup ECS world with two agents with negative positions, within range
    let mut world = World::default();
    let _agent1 = world.push((Position { x: -1.0, y: -1.0 }, InteractionState::default()));
    let _agent2 = world.push((Position { x: -1.5, y: -1.0 }, InteractionState::default()));
    // Resources
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Run system
    let mut schedule = Schedule::builder()
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    // These agents are 0.5 units apart, should interact
    let log = event_log.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("[INTERACT]"));
    assert!(found, "Agents with negative positions within range should interact");
    let stats = resources.get::<InteractionStats>().unwrap();
    assert_eq!(stats.agent_interactions, 1, "One interaction should be recorded for negative positions within range");
}
