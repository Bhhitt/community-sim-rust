//! Integration tests for agent-agent interaction across multiple ECS systems.

use community_sim::ecs_components::{Position, InteractionStats};
use community_sim::agent::components::InteractionState;
use community_sim::ecs::systems::agent_agent_interaction::{intent_assignment_system, agent_agent_interaction_system};
use legion::*;
use std::sync::{Arc, Mutex};
use community_sim::event_log::EventLog;

#[test]
fn test_agent_moves_and_then_interacts() {
    // Setup ECS world with two agents out of range initially
    let mut world = World::default();
    let agent1 = world.push((Position { x: 0.0, y: 0.0 }, InteractionState::default()));
    let agent2 = world.push((Position { x: 10.0, y: 0.0 }, InteractionState::default()));
    // Resources
    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());
    // Simulate movement: move agent2 into range of agent1
    // (In a real integration test, you would run a movement system; here we simulate it directly)
    let mut query = <(Entity, &mut Position)>::query();
    for (entity, pos) in query.iter_mut(&mut world) {
        if *entity == agent2 {
            pos.x = 1.0;
            pos.y = 0.0;
        }
    }
    // Run intent assignment system, then interaction system (two ticks)
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources); // Assign intents
    schedule.execute(&mut world, &mut resources); // Process interactions
    // Check event log for interaction
    let log = event_log.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent1, agent2)) || entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent2, agent1)));
    assert!(found, "Agents should interact after movement brings them into range");
    // Check stats
    let stats = resources.get::<InteractionStats>().unwrap();
    assert_eq!(stats.agent_interactions, 1, "Expected one interaction after agent movement");
}

#[test]
fn test_sequential_agent_movement() {
    // Three agents, only some pairs interact per tick
    let mut world = World::default();
    let agent1 = world.push((Position { x: 0.0, y: 0.0 }, InteractionState::default()));
    let agent2 = world.push((Position { x: 10.0, y: 0.0 }, InteractionState::default()));
    let agent3 = world.push((Position { x: 20.0, y: 0.0 }, InteractionState::default()));

    let mut resources = Resources::default();
    let stats = InteractionStats::default();
    resources.insert(stats);
    let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
    resources.insert(event_log.clone());

    // --- Tick 1: Move agent2 near agent1, agent3 remains far ---
    {
        let mut query = <(Entity, &mut Position)>::query();
        for (entity, pos) in query.iter_mut(&mut world) {
            if *entity == agent2 {
                pos.x = 1.0;
                pos.y = 0.0;
            }
        }
    }
    // Run intent assignment system, then interaction system (two ticks)
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources); // Assign intents
    schedule.execute(&mut world, &mut resources); // Process interactions
    let log = event_log.lock().unwrap();
    println!("Tick 1 event log:");
    for entry in log.iter() {
        println!("{}", entry);
    }
    let found_1 = log.iter().any(|entry| entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent1, agent2)) || entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent2, agent1)));
    assert!(found_1, "Agent1 and Agent2 should interact in tick 1");
    let found_2 = log.iter().any(|entry| entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent1, agent3)) || entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent3, agent1)));
    assert!(!found_2, "Agent1 and Agent3 should NOT interact in tick 1");
    drop(log);

    // --- Tick 2: Move agent3 near agent2, all are in range for some interactions ---
    {
        let mut query = <(Entity, &mut Position)>::query();
        for (entity, pos) in query.iter_mut(&mut world) {
            if *entity == agent3 {
                pos.x = 1.5;
                pos.y = 0.0;
            }
        }
    }
    // Run intent assignment system, then interaction system (two ticks)
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(agent_agent_interaction_system())
        .build();
    schedule.execute(&mut world, &mut resources); // Assign intents
    schedule.execute(&mut world, &mut resources); // Process interactions
    let log = event_log.lock().unwrap();
    println!("Tick 2 event log:");
    for entry in log.iter() {
        println!("{}", entry);
    }
    // At least one interaction involving agent3 should now exist
    let found_3 = log.iter().any(|entry| entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent2, agent3)) || entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent3, agent2)));
    assert!(found_3, "Agent2 and Agent3 should interact in tick 2");
    // There should still be no agent1-agent3 interaction (they are just out of range)
    let found_1_3 = log.iter().any(|entry| entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent1, agent3)) || entry.contains(&format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent3, agent1)));
    assert!(!found_1_3, "Agent1 and Agent3 should NOT interact in tick 2");
}
