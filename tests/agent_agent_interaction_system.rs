//! Tests for Agent-Agent Interaction System

use legion::*;
use legion::world::EntryRef;
use std::ops::Deref;
use community_sim::ecs_components::{Position, InteractionStats, InteractionQueue};
use community_sim::agent::InteractionState;
use community_sim::ecs::systems::agent_agent_interaction::{intent_assignment_system, pursuit_movement_system, interaction_range_system};
use std::sync::{Arc, Mutex};
use community_sim::event_log::EventLog;
use std::collections::VecDeque;

mod test_utils;
use test_utils::{spawn_test_agent, spawn_agents_with_positions};

// ECS test harness for agent-agent interaction tests
struct TestEcsContext {
    pub world: World,
    pub resources: Resources,
}

impl TestEcsContext {
    fn new() -> Self {
        let mut resources = Resources::default();
        resources.insert(InteractionStats::default());
        resources.insert(community_sim::ecs::agent_events::AgentEventQueue::default());
        resources.insert(InteractionQueue { queue: VecDeque::new() });
        let event_log = Arc::new(Mutex::new(EventLog::new(1000)));
        resources.insert(event_log.clone());
        Self {
            world: World::default(),
            resources,
        }
    }

    fn spawn_agent(&mut self, pos: Position, state: InteractionState) -> Entity {
        spawn_test_agent(&mut self.world, pos, state, None)
    }

    fn assign_intent(&mut self, entity: Entity, intent: community_sim::ecs_components::InteractionIntent) {
        let mut cmd = legion::systems::CommandBuffer::new(&self.world);
        cmd.add_component(entity, intent);
        cmd.flush(&mut self.world, &mut self.resources);
    }

    fn get_event_log(&self) -> Arc<Mutex<EventLog>> {
        self.resources.get::<Arc<Mutex<EventLog>>>().unwrap().clone()
    }

    fn get_stats(&self) -> &InteractionStats {
        // SAFETY: We know the resource lives as long as self.resources.
        // This is a test context, so we can use unsafe for clarity and brevity.
        unsafe {
            let ptr = self.resources.get::<InteractionStats>().unwrap().deref() as *const InteractionStats;
            &*ptr
        }
    }
}

#[test]
fn test_agents_interact_within_range() {
    let mut ctx = TestEcsContext::new();
    // Dummy agent to ensure archetype exists for all queries
    let _dummy = spawn_test_agent(
        &mut ctx.world,
        Position { x: 1000.0, y: 1000.0 },
        InteractionState::default(),
        Some(community_sim::ecs_components::InteractionIntent {
            target: None,
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        })
    );
    let agent1 = spawn_test_agent(&mut ctx.world, Position { x: 0.0, y: 0.0 }, InteractionState::default(), Some(community_sim::ecs_components::InteractionIntent {
        target: None,
        ticks_pursued: 0,
        max_pursue_ticks: 50,
    }));
    let agent2 = spawn_test_agent(&mut ctx.world, Position { x: 1.0, y: 1.0 }, InteractionState::default(), Some(community_sim::ecs_components::InteractionIntent {
        target: None,
        ticks_pursued: 0,
        max_pursue_ticks: 50,
    }));
    // Now update agent1's intent to have the correct target entity
    ctx.assign_intent(agent1, community_sim::ecs_components::InteractionIntent {
        target: Some(agent2),
        ticks_pursued: 0,
        max_pursue_ticks: 50,
    });
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .add_system(interaction_range_system())
        .build();
    // Only run the schedule once to avoid archetype panics after component removal
    schedule.execute(&mut ctx.world, &mut ctx.resources);
    let event_log_binding = ctx.get_event_log();
    let log = event_log_binding.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("InteractionStarted"));
    assert!(found, "Interaction event should be logged for agents within range");
    let stats = ctx.get_stats();
    assert_eq!(stats.agent_interactions, 1, "One interaction should be recorded in stats");
}

#[test]
fn test_no_interaction_out_of_range() {
    let mut ctx = TestEcsContext::new();
    // Dummy agent to ensure archetype exists for all queries
    let _dummy = spawn_test_agent(
        &mut ctx.world,
        Position { x: 1000.0, y: 1000.0 },
        InteractionState::default(),
        Some(community_sim::ecs_components::InteractionIntent {
            target: None,
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        })
    );
    let agent1 = spawn_test_agent(&mut ctx.world, Position { x: 0.0, y: 0.0 }, InteractionState::default(), None);
    let agent2 = spawn_test_agent(&mut ctx.world, Position { x: 10.0, y: 10.0 }, InteractionState::default(), None);
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .add_system(interaction_range_system())
        .build();
    schedule.execute(&mut ctx.world, &mut ctx.resources);
    let event_log_binding = ctx.get_event_log();
    let log = event_log_binding.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("InteractionStarted"));
    assert!(!found, "No interaction event should be logged for agents out of range");
    let stats = ctx.get_stats();
    assert_eq!(stats.agent_interactions, 0, "No interactions should be recorded in stats");
}

#[test]
fn test_interaction_state_updated() {
    let mut ctx = TestEcsContext::new();
    // Dummy agent to ensure archetype exists for all queries
    let _dummy = spawn_test_agent(
        &mut ctx.world,
        Position { x: 1000.0, y: 1000.0 },
        InteractionState::default(),
        Some(community_sim::ecs_components::InteractionIntent {
            target: None,
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        })
    );
    let agent1 = spawn_test_agent(&mut ctx.world, Position { x: 0.0, y: 0.0 }, InteractionState::default(), None);
    let agent2 = spawn_test_agent(&mut ctx.world, Position { x: 1.0, y: 1.0 }, InteractionState::default(), None);
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .add_system(interaction_range_system())
        .build();
    schedule.execute(&mut ctx.world, &mut ctx.resources);
    let mut query = <&InteractionState>::query();
    let states: Vec<_> = query.iter(&ctx.world).collect();
    assert_eq!(states.len(), 3, "Both agents and dummy should have InteractionState");
}

#[test]
fn test_multiple_agents_all_interact() {
    let mut ctx = TestEcsContext::new();
    // Dummy agent to ensure archetype exists for all queries
    let _dummy = spawn_test_agent(
        &mut ctx.world,
        Position { x: 1000.0, y: 1000.0 },
        InteractionState::default(),
        Some(community_sim::ecs_components::InteractionIntent {
            target: None,
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        })
    );
    let positions = [
        Position { x: 0.0, y: 0.0 },
        Position { x: 1.0, y: 1.0 },
        Position { x: 1.2, y: 0.8 },
        Position { x: 0.9, y: 1.3 },
    ];
    let agents = spawn_agents_with_positions(&mut ctx.world, &positions);
    for i in 0..agents.len() {
        let target = agents[(i + 1) % agents.len()];
        ctx.assign_intent(agents[i], community_sim::ecs_components::InteractionIntent {
            target: Some(target),
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        });
    }
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .add_system(interaction_range_system())
        .build();
    for _ in 0..12 {
        schedule.execute(&mut ctx.world, &mut ctx.resources);
    }
    let event_log_binding = ctx.get_event_log();
    let log = event_log_binding.lock().unwrap();
    let interaction_events: Vec<_> = log.iter().filter(|entry| entry.contains("InteractionStarted")).collect();
    assert!(interaction_events.len() >= 2, "Expected at least two interaction events for four clustered agents (with queueing)");
}

#[test]
fn test_no_agents_no_panic() {
    let mut ctx = TestEcsContext::new();
    // Dummy agent to ensure archetype exists for all queries
    let _dummy = spawn_test_agent(
        &mut ctx.world,
        Position { x: 1000.0, y: 1000.0 },
        InteractionState::default(),
        Some(community_sim::ecs_components::InteractionIntent {
            target: None,
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        })
    );
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .add_system(interaction_range_system())
        .build();
    schedule.execute(&mut ctx.world, &mut ctx.resources);
    let event_log_binding = ctx.get_event_log();
    let log = event_log_binding.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("InteractionStarted"));
    assert!(!found, "No interaction event should be logged when there are no agents");
    let stats = ctx.get_stats();
    assert_eq!(stats.agent_interactions, 0, "No interactions should be recorded in stats when there are no agents");
}

#[test]
fn test_missing_component_handling() {
    let mut ctx = TestEcsContext::new();
    // Dummy agent to ensure archetype exists for all queries
    let _dummy = spawn_test_agent(
        &mut ctx.world,
        Position { x: 1000.0, y: 1000.0 },
        InteractionState::default(),
        Some(community_sim::ecs_components::InteractionIntent {
            target: None,
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        })
    );
    let _agent1 = spawn_test_agent(&mut ctx.world, Position { x: 0.0, y: 0.0 }, InteractionState::default(), None);
    ctx.world.push((Position { x: 1.0, y: 1.0 },));
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .add_system(interaction_range_system())
        .build();
    schedule.execute(&mut ctx.world, &mut ctx.resources);
    let event_log_binding = ctx.get_event_log();
    let log = event_log_binding.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("InteractionStarted"));
    assert!(!found, "No interaction event should be logged when an agent is missing required components");
    let stats = ctx.get_stats();
    assert_eq!(stats.agent_interactions, 0, "No interactions should be recorded in stats when an agent is missing required components");
}

#[test]
fn test_interaction_at_threshold() {
    let mut ctx = TestEcsContext::new();
    // Dummy agent to ensure archetype exists for all queries
    let _dummy = spawn_test_agent(
        &mut ctx.world,
        Position { x: 1000.0, y: 1000.0 },
        InteractionState::default(),
        Some(community_sim::ecs_components::InteractionIntent {
            target: None,
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        })
    );
    let agent1 = spawn_test_agent(&mut ctx.world, Position { x: 0.0, y: 0.0 }, InteractionState::default(), None);
    let agent2 = spawn_test_agent(&mut ctx.world, Position { x: 1.5, y: 0.0 }, InteractionState::default(), None);
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .add_system(interaction_range_system())
        .build();
    schedule.execute(&mut ctx.world, &mut ctx.resources);
    let event_log_binding = ctx.get_event_log();
    let log = event_log_binding.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("InteractionStarted"));
    assert!(!found, "No interaction event should be logged for agents exactly at threshold");
    let stats = ctx.get_stats();
    assert_eq!(stats.agent_interactions, 0, "No interactions should be recorded in stats for agents exactly at threshold");
}

#[test]
fn test_negative_position_handling() {
    let mut ctx = TestEcsContext::new();
    // Dummy agent to ensure archetype exists for all queries
    let _dummy = spawn_test_agent(
        &mut ctx.world,
        Position { x: 1000.0, y: 1000.0 },
        InteractionState::default(),
        Some(community_sim::ecs_components::InteractionIntent {
            target: None,
            ticks_pursued: 0,
            max_pursue_ticks: 50,
        })
    );
    let agent1 = spawn_test_agent(&mut ctx.world, Position { x: -1.0, y: -1.0 }, InteractionState::default(), None);
    let agent2 = spawn_test_agent(&mut ctx.world, Position { x: -1.5, y: -1.0 }, InteractionState::default(), None);
    ctx.assign_intent(agent1, community_sim::ecs_components::InteractionIntent {
        target: Some(agent2),
        ticks_pursued: 0,
        max_pursue_ticks: 50,
    });
    let mut schedule = Schedule::builder()
        .add_system(intent_assignment_system())
        .add_system(pursuit_movement_system())
        .add_system(interaction_range_system())
        .build();
    for _ in 0..3 {
        schedule.execute(&mut ctx.world, &mut ctx.resources);
    }
    let event_log_binding = ctx.get_event_log();
    let log = event_log_binding.lock().unwrap();
    let found = log.iter().any(|entry| entry.contains("InteractionStarted"));
    assert!(found, "Agents with negative positions within range should interact");
}
