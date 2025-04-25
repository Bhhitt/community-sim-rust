// Agent-related ECS systems will be moved here.

// NOTE: The agent_pause_system is implemented in a separate file (pause_system.rs) due to Legion's 8-parameter limit on system queries.
// If the parameter limit is ever increased or the query simplified, consider consolidating for clarity. See pause_system.rs for details.

use crate::navigation::*;
use legion::*;
use std::collections::VecDeque;
use crate::agent::event::{AgentEvent, AgentEventLog};
use std::sync::{Arc, Mutex};
use crate::agent::components::{Target, Path};

// pub fn spawn_agent(world: &mut legion::World, pos: crate::ecs_components::Position, agent_type: crate::agent::AgentType, map: &crate::map::Map, agent_event_log: &mut AgentEventLog) -> legion::Entity {
//     // Logic now handled by agent_spawning_system
// }

// Removed legacy agent_arrival_system; now handled by agent_state_transition_system in agent.rs

// --- ECS Agent Path Following System ---
// (REMOVED: All responsibilities split into dedicated systems for pausing, movement, hunger/energy, state transitions, and logging. See plans/path_following_system_refactor_plan.md)

// --- ECS Agent Action Selection System (LEGACY, REMOVED) ---
// Legacy agent_action_selection_system removed; now replaced by modular ECS systems

fn find_closest_food(x: f32, y: f32, food_positions: &crate::ecs_components::FoodPositions) -> Option<(i32, i32)> {
    food_positions.0.iter()
        .map(|&(fx, fy)| (fx as i32, fy as i32, ((fx - x).powi(2) + (fy - y).powi(2)).sqrt()))
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .map(|(fx, fy, _)| (fx, fy))
}

// --- ECS Agent Passive Hunger System ---
// (REMOVED: logic now handled by agent_hunger_energy_system in ecs/systems/agent.rs)

// --- ECS Agent Movement History System ---
// (Moved to ecs/systems/agent.rs for modular ECS organization)

// --- ECS Agent Death System ---
pub fn agent_death_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDeathSystem")
        .with_query::<(Entity, &crate::agent::Hunger, &crate::agent::Energy), ()>(())
        .build(|cmd, _world, _, _query| {
            let mut to_remove = Vec::new();
            for (entity, hunger, energy) in _query.iter(_world) {
                if hunger.value <= 0.0 || energy.value <= 0.0 {
                    to_remove.push(entity);
                }
            }
            for entity in to_remove {
                cmd.remove(*entity);
            }
        })
}

// swimming_system moved to ecs/systems/swimming.rs
