// TODO: Fix all usages of Path and Target to expect them from agent::components
// and ensure all ECS queries and logic are up to date with new modular system design.

// Agent ECS system implementation
// Refactored: Movement logic split for clarity and ECS best practices.

use legion::{Entity, IntoQuery};
use crate::agent::{AgentType, AgentState};
use crate::ecs_components::Position;
use crate::agent::components::Target;
use crate::navigation::Path;

// --- ECS Agent Path Movement System ---
/// Removes the first waypoint from the agent's path if present (does not update position).
pub fn agent_path_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentPathMovementSystem")
        .with_query(<(&mut Path,)>::query())
        .build(|_, world, _, query| {
            for (path,) in query.iter_mut(world) {
                if !path.waypoints.is_empty() {
                    path.waypoints.remove(0);
                }
            }
        })
}

// --- ECS Agent Direct Movement System ---
/// Moves agent directly toward target if no path is present.
pub fn agent_direct_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDirectMovementSystem")
        .with_query(<(&mut Position, &AgentType, &Path, &Target, &AgentState)>::query())
        .build(|_, world, _, query| {
            for (pos, agent_type, path, target, agent_state) in query.iter_mut(world) {
                if (*agent_state == AgentState::Idle || *agent_state == AgentState::Moving)
                    && path.waypoints.is_empty()
                {
                    let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                    let step = agent_type.movement_profile.speed.min(dist);
                    if dist > 0.1 {
                        pos.x += (target.x - pos.x) / dist * step;
                        pos.y += (target.y - pos.y) / dist * step;
                    } else {
                        pos.x = target.x;
                        pos.y = target.y;
                    }
                }
            }
        })
}

/// Agent state transition system: sets AgentState::Arrived when agent position matches target.
pub fn agent_state_transition_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentStateTransitionSystem")
        .with_query(<(&mut Position, &Target, &mut AgentState, &Path)>::query())
        .build(|_, world, _, query| {
            log::debug!("[SYSTEM] Entering agent_state_transition_system");
            for (pos, target, agent_state, path) in query.iter_mut(world) {
                if (*agent_state == AgentState::Moving || *agent_state == AgentState::Idle)
                    && path.waypoints.is_empty()
                {
                    let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                    if dist <= 0.1 {
                        log::debug!("[StateTransition] Entity: {:?} arrived at target ({}, {})", pos, target.x, target.y);
                        *agent_state = AgentState::Arrived;
                    } else if dist > 0.1 {
                        *agent_state = AgentState::Moving;
                    }
                } else if path.waypoints.is_empty() {
                    *agent_state = AgentState::Idle;
                }
            }
        })
}

// --- ECS Agent Pausing System ---
/// Agent pausing system: handles all IdlePause logic (decrementing ticks_remaining).
pub fn agent_pausing_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentPausingSystem")
        .with_query(<(Entity, &mut crate::agent::components::IdlePause)>::query())
        .build(|_, world, _, query| {
            log::debug!("[SYSTEM] Entering agent_pausing_system");
            for (_entity, idle_pause) in query.iter_mut(world) {
                if idle_pause.ticks_remaining > 0 {
                    idle_pause.ticks_remaining -= 1;
                }
            }
        })
}

// --- ECS Agent Movement History System ---
/// Agent movement history system: records each agent's recent positions for analytics/debugging.
pub fn agent_movement_history_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementHistorySystem")
        .with_query(<(Entity, &Position, &mut crate::agent::components::MovementHistory)>::query())
        .build(|_, world, _, query| {
            for (_entity, pos, history) in query.iter_mut(world) {
                history.add(pos.x, pos.y);
            }
         })
}

// --- ecs agent hunger/energy system ---
/// agent hunger/energy system: manages hunger and energy levels for agents.
pub fn agent_hunger_energy_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("agent_hunger_energy_system")
        .with_query(<(Entity, &AgentType, &mut crate::agent::Hunger, &mut crate::agent::Energy, &AgentState)>::query())
        .build(|_, world, _, query| {
            log::debug!("[SYSTEM] Entering agent_hunger_energy_system");
            for (_entity, agent_type, hunger, energy, agent_state) in query.iter_mut(world) {
                // Hunger logic for idle/arrived and moving states
                if *agent_state == AgentState::Idle || *agent_state == AgentState::Arrived {
                    hunger.value -= agent_type.hunger_rate * 0.1;
                    energy.value -= 0.1;
                } else if *agent_state == AgentState::Moving {
                    hunger.value -= agent_type.hunger_rate;
                    energy.value -= 1.0;
                }
            }
        })
}
