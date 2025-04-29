//! Integration tests for event-driven agent-agent interaction (queueing, event emission, chaining).

use legion::*;
use community_sim::ecs_components::{Position, InteractionIntent, InteractionQueue, Interacting};
use community_sim::ecs::systems::agent_agent_interaction::interaction_range_system;
use std::collections::VecDeque;
use community_sim::ecs::agent_events::{AgentEvent, AgentEventQueue};

/// Helper to set up a world, resources, and schedule for event-driven agent interaction tests.
pub fn setup_event_driven_test_world() -> (World, Resources, Schedule) {
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(InteractionQueue { queue: VecDeque::new() });
    resources.insert(AgentEventQueue(Vec::new()));
    let schedule = Schedule::builder()
        .add_system(interaction_range_system())
        .build();
    (world, resources, schedule)
}

#[test]
fn test_interaction_events_emitted_and_chain() {
    let (mut world, mut resources, mut schedule) = setup_event_driven_test_world();

    // Create agent1 with a dummy target (None), set correct target after creation
    let agent1 = world.push((
        Position { x: 0.0, y: 0.0 },
        InteractionIntent {
            target: None, 
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));
    // Create agent2 with agent1 as target
    let agent2 = world.push((
        Position { x: 1.0, y: 1.0 },
        InteractionIntent {
            target: Some(agent1), 
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));

    // Set agent1 to target agent2
    world.entry_mut(agent1).unwrap().get_component_mut::<InteractionIntent>().unwrap().target = Some(agent2);

    // Run schedule for several ticks to allow interaction to start and end
    for _ in 0..12 {
        schedule.execute(&mut world, &mut resources);
    }

    // Check AgentEventQueue resource for InteractionStarted and InteractionEnded
    let event_queue = resources.get::<AgentEventQueue>().unwrap();
    let started = event_queue.0.iter().any(|e| match e {
        AgentEvent::InteractionStarted { initiator, target } => *initiator == agent1 && *target == agent2,
        _ => false,
    });
    let ended = event_queue.0.iter().any(|e| match e {
        AgentEvent::InteractionEnded { initiator, target } => *initiator == agent1 && *target == agent2,
        _ => false,
    });
    assert!(started, "InteractionStarted event should be emitted");
    assert!(ended, "InteractionEnded event should be emitted");

    // Both agents should no longer be Interacting
    let agent1_interacting = world.entry_ref(agent1).unwrap().get_component::<Interacting>().is_ok();
    let agent2_interacting = world.entry_ref(agent2).unwrap().get_component::<Interacting>().is_ok();
    assert!(!agent1_interacting && !agent2_interacting, "Both agents should not be Interacting after interaction ends");
}

// --- SCAFFOLDED TESTS FOR ADVANCED AGENT-AGENT INTERACTION ---

/// Test: Multiple agents queueing for a single target
/// - Create three agents: A, B, and Target.
/// - Both A and B set their InteractionIntent.target = Some(Target).
/// - Simulate enough ticks for A to finish interaction, then B should start.
/// - Assert: Only one agent is Interacting with the target at a time, both interact in order, queue is empty at end.
#[test]
fn test_multiple_agents_queue_for_single_target() {
    // 1. Setup world, resources, schedule
    let (mut world, mut resources, mut schedule) = setup_event_driven_test_world();

    // 2. Create agents: Target, A, B
    let target = world.push((
        Position { x: 0.0, y: 0.0 },
        InteractionIntent {
            target: None,
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));
    let agent_a = world.push((
        Position { x: 1.0, y: 0.0 },
        InteractionIntent {
            target: Some(target),
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));
    let agent_b = world.push((
        Position { x: 2.0, y: 0.0 },
        InteractionIntent {
            target: Some(target),
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));

    // 3. Simulate ticks: enough for A to finish, then B to start and finish
    let mut saw_a_interacting = false;
    let mut saw_b_interacting = false;
    let mut saw_both_interacting = false;
    for _ in 0..30 {
        schedule.execute(&mut world, &mut resources);
        let target_entry = world.entry_ref(target).unwrap();
        let queue = target_entry.get_component::<InteractionQueue>().unwrap();
        let a_interacting = world.entry_ref(agent_a).unwrap().get_component::<Interacting>().is_ok();
        let b_interacting = world.entry_ref(agent_b).unwrap().get_component::<Interacting>().is_ok();
        let target_interacting = target_entry.get_component::<Interacting>().is_ok();
        // Only one agent should be interacting with target at a time
        if a_interacting && target_interacting { saw_a_interacting = true; }
        if b_interacting && target_interacting { saw_b_interacting = true; }
        if a_interacting && b_interacting && target_interacting { saw_both_interacting = true; }
    }
    // 4. Assert correct interaction order and queue state
    assert!(saw_a_interacting, "Agent A should interact with target");
    assert!(saw_b_interacting, "Agent B should interact with target after A");
    assert!(!saw_both_interacting, "A and B should not interact with target at the same time");
    // After all, queue should be empty
    let target_entry = world.entry_ref(target).unwrap();
    let queue = target_entry.get_component::<InteractionQueue>().unwrap();
    assert!(queue.queue.is_empty(), "Target's queue should be empty after all interactions");
    // Both agents should no longer be Interacting
    let a_interacting = world.entry_ref(agent_a).unwrap().get_component::<Interacting>().is_ok();
    let b_interacting = world.entry_ref(agent_b).unwrap().get_component::<Interacting>().is_ok();
    let target_interacting = target_entry.get_component::<Interacting>().is_ok();
    assert!(!a_interacting && !b_interacting && !target_interacting, "No agent should be Interacting at end");
}

/// Test: Target disappears while agents are queued
/// - Create two agents and a target.
/// - Both agents target the same entity.
/// - Remove the target entity before interaction starts.
/// - Assert: Intents are cleared, no panics, no Interacting remains.
#[test]
fn test_target_disappears_while_agents_queued() {
    // 1. Setup world, resources, schedule
    let (mut world, mut resources, mut schedule) = setup_event_driven_test_world();

    // 2. Create agents and target, set intents
    let target = world.push((
        Position { x: 0.0, y: 0.0 },
        InteractionIntent {
            target: None,
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));
    let agent_a = world.push((
        Position { x: 1.0, y: 0.0 },
        InteractionIntent {
            target: Some(target),
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));
    let agent_b = world.push((
        Position { x: 2.0, y: 0.0 },
        InteractionIntent {
            target: Some(target),
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));

    // 3. Remove target entity before interaction starts
    world.remove(target);

    // 4. Simulate ticks, assert intents cleared, no Interacting
    let mut a_intent_cleared = false;
    let mut b_intent_cleared = false;
    let mut a_interacting = false;
    let mut b_interacting = false;
    for _ in 0..10 {
        schedule.execute(&mut world, &mut resources);
        // Check that intents are cleared
        let a_entry = world.entry_ref(agent_a).unwrap();
        let b_entry = world.entry_ref(agent_b).unwrap();
        let a_intent = a_entry.get_component::<InteractionIntent>();
        let b_intent = b_entry.get_component::<InteractionIntent>();
        if a_intent.is_err() { a_intent_cleared = true; }
        if b_intent.is_err() { b_intent_cleared = true; }
        // Check that no Interacting component remains
        if a_entry.get_component::<Interacting>().is_ok() { a_interacting = true; }
        if b_entry.get_component::<Interacting>().is_ok() { b_interacting = true; }
    }
    assert!(a_intent_cleared, "Agent A's intent should be cleared when target disappears");
    assert!(b_intent_cleared, "Agent B's intent should be cleared when target disappears");
    assert!(!a_interacting, "Agent A should not be Interacting after target disappears");
    assert!(!b_interacting, "Agent B should not be Interacting after target disappears");
}

/// Test: Agent gives up after max pursue ticks
/// - Place agent and target far apart, set low max_pursue_ticks.
/// - Simulate enough ticks.
/// - Assert: Intent is removed, no event emitted.
#[test]
fn test_agent_gives_up_after_max_pursue_ticks() {
    // 1. Setup world, resources, schedule
    let (mut world, mut resources, mut schedule) = setup_event_driven_test_world();

    // 2. Create agent and target, set intent with low max_pursue_ticks
    let target = world.push((
        Position { x: 100.0, y: 0.0 }, // Far away
        InteractionIntent {
            target: None,
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));
    let agent = world.push((
        Position { x: 0.0, y: 0.0 },
        InteractionIntent {
            target: Some(target),
            max_pursue_ticks: 3, // Low threshold for pursue
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));

    // 3. Simulate ticks, assert intent removed, no event
    let mut intent_removed = false;
    let mut agent_interacting = false;
    for _ in 0..10 {
        schedule.execute(&mut world, &mut resources);
        let agent_entry = world.entry_ref(agent).unwrap();
        let intent = agent_entry.get_component::<InteractionIntent>();
        if intent.is_err() { intent_removed = true; }
        if agent_entry.get_component::<Interacting>().is_ok() { agent_interacting = true; }
    }
    assert!(intent_removed, "Agent's intent should be removed after exceeding max_pursue_ticks");
    assert!(!agent_interacting, "Agent should not be Interacting if it never reached the target");
    // Optionally: check that no event was emitted (if you have an event queue, add this check)
}

/// Test: Agent can re-queue after idle
/// - Agent A interacts with B, then targets C after completion.
/// - Assert: A is queued for C after B, correct event chain.
#[test]
fn test_agent_can_requeue_after_idle() {
    // 1. Setup world, resources, schedule
    let (mut world, mut resources, mut schedule) = setup_event_driven_test_world();

    // 2. Create agents A, B, C
    let agent_b = world.push((
        Position { x: 0.0, y: 0.0 },
        InteractionIntent {
            target: None,
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));
    let agent_a = world.push((
        Position { x: 1.0, y: 0.0 },
        InteractionIntent {
            target: Some(agent_b),
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));
    let agent_c = world.push((
        Position { x: 2.0, y: 0.0 },
        InteractionIntent {
            target: None,
            max_pursue_ticks: 10,
            ticks_pursued: 0,
        },
        InteractionQueue { queue: VecDeque::new() },
    ));

    // 3. Simulate A->B interaction
    let mut a_interacted_with_b = false;
    for _ in 0..15 {
        schedule.execute(&mut world, &mut resources);
        let a_entry = world.entry_ref(agent_a).unwrap();
        let b_entry = world.entry_ref(agent_b).unwrap();
        let a_interacting = a_entry.get_component::<Interacting>().is_ok();
        let b_interacting = b_entry.get_component::<Interacting>().is_ok();
        if a_interacting && b_interacting {
            a_interacted_with_b = true;
        }
    }
    assert!(a_interacted_with_b, "Agent A should interact with B first");

    // 4. After completion, A targets C
    world.entry_mut(agent_a).unwrap().get_component_mut::<InteractionIntent>().unwrap().target = Some(agent_c);
    // Reset ticks_pursued if needed
    world.entry_mut(agent_a).unwrap().get_component_mut::<InteractionIntent>().unwrap().ticks_pursued = 0;

    // 5. Simulate A->C interaction
    let mut a_interacted_with_c = false;
    for _ in 0..15 {
        schedule.execute(&mut world, &mut resources);
        let a_entry = world.entry_ref(agent_a).unwrap();
        let c_entry = world.entry_ref(agent_c).unwrap();
        let a_interacting = a_entry.get_component::<Interacting>().is_ok();
        let c_interacting = c_entry.get_component::<Interacting>().is_ok();
        if a_interacting && c_interacting {
            a_interacted_with_c = true;
        }
    }
    assert!(a_interacted_with_c, "Agent A should interact with C after B");
    // At end, A, B, and C should not be Interacting
    let a_interacting = world.entry_ref(agent_a).unwrap().get_component::<Interacting>().is_ok();
    let b_interacting = world.entry_ref(agent_b).unwrap().get_component::<Interacting>().is_ok();
    let c_interacting = world.entry_ref(agent_c).unwrap().get_component::<Interacting>().is_ok();
    assert!(!a_interacting && !b_interacting && !c_interacting, "No agent should be Interacting at end");
}
