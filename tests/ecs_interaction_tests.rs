//! ECS-based tests for agent interaction components

use community_sim::ecs_components::{InteractionIntent, InteractionQueue, Interacting, Position};
use legion::{Entity, World, Resources, Schedule};
use std::collections::VecDeque;

// Add logger initialization for test log output
fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_interaction_intent_creation() {
    let mut world = World::default();
    let target = world.push(());
    let intent = InteractionIntent {
        target,
        ticks_pursued: 0,
        max_pursue_ticks: 50,
    };
    assert_eq!(intent.target, target);
    assert_eq!(intent.ticks_pursued, 0);
    assert_eq!(intent.max_pursue_ticks, 50);
}

#[test]
fn test_interaction_intent_progress() {
    let mut world = World::default();
    let target = world.push(());
    let mut intent = InteractionIntent {
        target,
        ticks_pursued: 10,
        max_pursue_ticks: 50,
    };
    intent.ticks_pursued += 1;
    assert_eq!(intent.ticks_pursued, 11);
    assert!(intent.ticks_pursued < intent.max_pursue_ticks);
}

#[test]
fn test_interaction_queue_basic() {
    let mut world = World::default();
    let a = world.push(());
    let b = world.push(());
    let mut queue = InteractionQueue { queue: VecDeque::new() };
    queue.queue.push_back(a);
    queue.queue.push_back(b);
    assert_eq!(queue.queue.len(), 2);
    assert_eq!(queue.queue.pop_front(), Some(a));
    assert_eq!(queue.queue.pop_front(), Some(b));
    assert_eq!(queue.queue.pop_front(), None);
}

#[test]
fn test_interacting_basic() {
    let mut world = World::default();
    let a = world.push(());
    let b = world.push(());
    let mut interacting = Interacting { partner: b, ticks_remaining: 5 };
    assert_eq!(interacting.partner, b);
    assert_eq!(interacting.ticks_remaining, 5);
    interacting.ticks_remaining -= 1;
    assert_eq!(interacting.ticks_remaining, 4);
}

#[test]
fn integration_intent_and_pursuit() {
    init_logger();
    use community_sim::ecs_components::{InteractionIntent, Position};
    use community_sim::ecs::systems::agent_agent_interaction::{intent_assignment_system, pursuit_movement_system};
    use legion::*;

    // Setup world and schedule
    let mut world = World::default();
    // Place agents farther apart to ensure pursuit lasts at least 5 ticks
    let a = world.push((Position { x: 0.0, y: 0.0 },));
    let b = world.push((Position { x: 20.0, y: 0.0 },));
    // ECS schedule with both systems
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .build();

    // Run simulation for 5 ticks
    let mut resources = Resources::default();
    for _ in 0..5 {
        schedule.execute(&mut world, &mut resources);
    }

    // After a few ticks, agent a should have moved toward b
    let a_entry = world.entry_ref(a).unwrap();
    let apos = a_entry.get_component::<Position>().unwrap();
    let b_entry = world.entry_ref(b).unwrap();
    let bpos = b_entry.get_component::<Position>().unwrap();
    // a should have moved right (increasing x)
    assert!(apos.x > 0.0 && apos.x < bpos.x);
    // a should have an InteractionIntent targeting b
    let a_intent_entry = world.entry_ref(a).unwrap();
    let a_intent_result = a_intent_entry.get_component::<InteractionIntent>();
    if let Ok(a_intent) = a_intent_result {
        assert_eq!(a_intent.target, b);
    } else {
        panic!("Agent a is missing InteractionIntent after 5 ticks! This likely means the system removed it early.");
    }
    // b should not have moved
    assert_eq!(bpos.x, 20.0);
}
