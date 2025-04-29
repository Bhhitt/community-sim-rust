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
            let start = std::time::Instant::now();
            for (path,) in query.iter_mut(world) {
                if !path.waypoints.is_empty() {
                    path.waypoints.remove(0);
                }
            }
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_path_movement_system took {:?}", duration);
        })
}

// --- ECS Agent Direct Movement System ---
/// Moves agent directly toward target if no path is present.
pub fn agent_direct_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDirectMovementSystem")
        .with_query(<(&mut Position, &AgentType, &Path, &Target, &AgentState)>::query())
        .build(|_, world, _, query| {
            let start = std::time::Instant::now();
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
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_direct_movement_system took {:?}", duration);
        })
}

// --- [DEPRECATED/REMOVED] agent_state_transition_system ---
// The old agent_state_transition_system has been removed.
// Use ecs/systems/agent_state_transition.rs for the current implementation.
//
// pub fn agent_state_transition_system() -> impl legion::systems::Runnable {
//     panic!("[DEPRECATED] agent_state_transition_system in agent.rs is deprecated. Use ecs/systems/agent_state_transition.rs");
// }

// --- ECS Agent Pausing System ---
/// Agent pausing system: handles all IdlePause logic (decrementing ticks_remaining).
pub fn agent_pausing_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentPausingSystem")
        .with_query(<(Entity, &mut crate::agent::components::IdlePause)>::query())
        .build(|_, world, _, query| {
            let start = std::time::Instant::now();
            log::debug!("[SYSTEM] Entering agent_pausing_system");
            log::debug!("[DEBUG] agent_pausing_system: before query iteration");
            // Defensive: collect to Vec to avoid Legion archetype invalidation
            let entities: Vec<_> = query.iter_mut(world).collect();
            log::debug!("[DEBUG] agent_pausing_system: collected {} entities to Vec", entities.len());
            let mut count = 0;
            for (_entity, idle_pause) in entities {
                count += 1;
                log::trace!("[DEBUG] agent_pausing_system: inside loop, entity count {}", count);
                if idle_pause.ticks_remaining > 0 {
                    idle_pause.ticks_remaining -= 1;
                }
            }
            log::debug!("[DEBUG] agent_pausing_system: after query iteration");
            log::debug!("[DEBUG] agent_pausing_system: matched {} entities with IdlePause", count);
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_pausing_system took {:?}", duration);
        })
}

// --- ECS Agent Movement History System ---
/// Agent movement history system: records each agent's recent positions for analytics/debugging.
pub fn agent_movement_history_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementHistorySystem")
        .with_query(<(Entity, &Position, &mut crate::agent::components::MovementHistory)>::query())
        .build(|_, world, _, query| {
            let start = std::time::Instant::now();
            for (_entity, pos, history) in query.iter_mut(world) {
                history.add(pos.x, pos.y);
            }
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_movement_history_system took {:?}", duration);
        })
}

// --- ecs agent hunger/energy system ---
/// agent hunger/energy system: manages hunger and energy levels for agents.
pub fn agent_hunger_energy_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("agent_hunger_energy_system")
        .with_query(<(Entity, &AgentType, &mut crate::agent::Hunger, &mut crate::agent::Energy, &AgentState)>::query())
        .build(|_, world, _, query| {
            let start = std::time::Instant::now();
            log::debug!("[SYSTEM] Entering agent_hunger_energy_system");
            for (_entity, agent_type, hunger, energy, agent_state) in query.iter_mut(world) {
                // Hunger logic for idle/arrived and moving states
                if *agent_state == AgentState::Idle || *agent_state == AgentState::Arrived {
                    hunger.value -= agent_type.hunger_rate * 0.1;
                    // energy.value -= 0.1; // Disable energy depletion
                } else if *agent_state == AgentState::Moving {
                    hunger.value -= agent_type.hunger_rate;
                    // energy.value -= 1.0; // Disable energy depletion
                }
            }
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_hunger_energy_system took {:?}", duration);
        })
}
