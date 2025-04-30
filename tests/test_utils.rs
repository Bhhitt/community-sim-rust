//! Shared test utilities for ECS agent-based simulation tests

use legion::*;
use std::collections::VecDeque;
use community_sim::ecs_components::{Position, InteractionIntent, InteractionQueue};
use community_sim::agent::components::InteractionState;

/// Spawn an agent with required components. Optionally attach an InteractionIntent.
pub fn spawn_test_agent(
    world: &mut World,
    pos: Position,
    state: InteractionState,
    intent: Option<InteractionIntent>,
) -> Entity {
    match intent {
        Some(intent) => world.push((pos, state, InteractionQueue { queue: VecDeque::new() }, intent)),
        None => world.push((pos, state, InteractionQueue { queue: VecDeque::new() })),
    }
}

/// Spawn multiple agents with positions and default state.
pub fn spawn_agents_with_positions(
    world: &mut World,
    positions: &[Position],
) -> Vec<Entity> {
    positions.iter().map(|&pos| {
        world.push((pos, InteractionState::default(), InteractionQueue { queue: VecDeque::new() }))
    }).collect()
}
