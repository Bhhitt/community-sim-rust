// TODO: Fix all usages of Path and Target to expect them from agent::components
// and ensure all ECS queries and logic are up to date with new modular system design.

// Agent ECS system implementation
// Refactored: Movement logic split for clarity and ECS best practices.

// use crate::navigation::*; // (Unused)
use legion::*;
// use std::sync::{Arc, Mutex}; // (Unused)
use crate::agent::components::{Target, Path};

/// Pure agent movement system: only updates positions along path/waypoints or directly toward target.
pub fn agent_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementSystem")
        .with_query::<(
            &mut crate::ecs_components::Position,
            &crate::agent::AgentType,
            Option<&mut crate::agent::components::Target>,
            Option<&mut crate::agent::components::Path>,
            &mut crate::agent::AgentState,
            &mut crate::agent::components::IdlePause,
        ), ()>()
        .build(|_, world, _, query| {
            for (pos, agent_type, maybe_target, maybe_path, agent_state, _idle_pause) in query.iter_mut(world) {
                // Pausing logic removed: all pausing is now handled by agent_pausing_system
                if *agent_state == crate::agent::AgentState::Idle || *agent_state == crate::agent::AgentState::Moving {
                    if let Some(target) = maybe_target.as_mut() {
                        if let Some(path) = maybe_path.as_mut() {
                            if !path.waypoints.is_empty() {
                                let (tx, ty) = path.waypoints[0];
                                let dx = tx as f32 - pos.x;
                                let dy = ty as f32 - pos.y;
                                let dist = (dx * dx + dy * dy).sqrt();
                                let step = agent_type.movement_profile.speed.min(dist);
                                pos.x += dx / dist * step;
                                pos.y += dy / dist * step;
                                path.waypoints.pop_front();
                            } else {
                                pos.x = target.x;
                                pos.y = target.y;
                                // State transition removed: handled by separate system
                            }
                        } else {
                            let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                            let step = agent_type.movement_profile.speed.min(dist);
                            if dist > 0.1 {
                                pos.x += (target.x - pos.x) / dist * step;
                                pos.y += (target.y - pos.y) / dist * step;
                            } else {
                                pos.x = target.x;
                                pos.y = target.y;
                                // State transition removed: handled by separate system
                            }
                        }
                    }
                }
            }
        })
}

/// Agent state transition system: sets AgentState::Arrived when agent position matches target.
pub fn agent_state_transition_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentStateTransitionSystem")
        .with_query::<(
            &mut crate::ecs_components::Position,
            Option<&crate::agent::components::Target>,
            &mut crate::agent::AgentState,
        ), ()>()
        .build(|_, world, _, query| {
            for (pos, maybe_target, agent_state) in query.iter_mut(world) {
                if *agent_state == crate::agent::AgentState::Moving || *agent_state == crate::agent::AgentState::Idle {
                    if let Some(target) = maybe_target {
                        let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                        if dist <= 0.1 {
                            *agent_state = crate::agent::AgentState::Arrived;
                        }
                    }
                }
            }
        })
}

/// Agent pausing system: handles all IdlePause logic (decrementing ticks_remaining).
pub fn agent_pausing_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentPausingSystem")
        .with_query::<(&mut crate::agent::components::IdlePause,), ()>()
        .build(|_, world, _, query| {
            for (idle_pause,) in query.iter_mut(world) {
                if idle_pause.ticks_remaining > 0 {
                    idle_pause.ticks_remaining -= 1;
                }
            }
        })
}

/// Agent movement history system: records each agent's recent positions for analytics/debugging.
pub fn agent_movement_history_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementHistorySystem")
        .with_query::<(Entity, &crate::ecs_components::Position, &mut crate::agent::components::MovementHistory), ()>()
        .build(|_, world, _, query| {
            for (entity, pos, history) in query.iter_mut(world) {
                history.add(pos.x, pos.y);
            }
        })
}

// --- ECS Agent Hunger/Energy System ---
pub fn agent_hunger_energy_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentHungerEnergySystem")
        .with_query::<(
            &crate::agent::AgentType,
            &mut crate::agent::Hunger,
            &mut crate::agent::Energy,
            &crate::agent::AgentState,
        ), ()>()
        .build(|_, world, _, query| {
            for (agent_type, hunger, energy, agent_state) in query.iter_mut(world) {
                // Hunger logic (mirrors previous passive_hunger_system)
                if *agent_state == crate::agent::AgentState::Idle || *agent_state == crate::agent::AgentState::Arrived {
                    hunger.value -= agent_type.hunger_rate * 0.1;
                    energy.value -= 0.1; // Example: slow energy drain when idle/arrived
                } else if *agent_state == crate::agent::AgentState::Moving {
                    hunger.value -= agent_type.hunger_rate;
                    energy.value -= 1.0; // Example: faster energy drain when moving
                }
            }
        })
}
