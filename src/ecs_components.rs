//! ECS Components for community simulator (Legion)
use legion;
use legion::IntoQuery;
use legion::SystemBuilder;
use rand::Rng;
use log;
use crate::food::Food;
use crate::agent::{InteractionState, event::{AgentEvent, AgentEventLog}};
use std::sync::{Arc, Mutex};
// Example usage in entity_interaction_system:
// agent_event_log.push(AgentEvent::AteFood { agent, food, nutrition });

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

pub struct InteractionStats {
    pub agent_interactions: usize,
    pub active_interactions: usize,
    pub active_interactions_history: std::collections::VecDeque<usize>,
}

impl Default for InteractionStats {
    fn default() -> Self {
        Self {
            agent_interactions: 0,
            active_interactions: 0,
            active_interactions_history: std::collections::VecDeque::with_capacity(100),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct FoodStats {
    pub spawned_per_tick: usize,
    pub collected_per_tick: usize,
}

// --- Entity Spawning Functions ---
use legion::systems::CommandBuffer;
pub fn spawn_food(cmd: &mut CommandBuffer, pos: Position, food_stats: Option<&mut FoodStats>) -> legion::Entity {
    let nutrition = rand::thread_rng().gen_range(5.0..=10.0);
    if let Some(stats) = food_stats {
        stats.spawned_per_tick += 1;
    }
    cmd.push((pos, Food { nutrition }))
}

// --- Resource for food positions (for agent movement system) ---
pub struct FoodPositions(pub Vec<(f32, f32)>);

// --- ECS Agent Interaction System ---
pub fn agent_interaction_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentInteractionSystem")
        .with_query(<(legion::Entity, &Position)>::query())
        .build(|_cmd, _world, _, query| {
            // --- Pass 1: Collect all agent info for matching (read-only) ---
            for (_entity, _pos) in query.iter(_world) {
                // No-op
            }
            // --- Phase 2/3: Decrement ticks, clear when done ---
            for (_entity, _pos) in query.iter(_world) {
                // No-op
            }
        })
}

// --- ECS Interaction System (agent-agent, agent-food) ---
pub fn entity_interaction_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("EntityInteractionSystem")
        .write_resource::<InteractionStats>()
        .write_resource::<Arc<Mutex<crate::event_log::EventLog>>>()
        .write_resource::<FoodStats>()
        .write_resource::<AgentEventLog>()
        .with_query(<(legion::Entity, &Position, &InteractionState)>::query()) // agents
        .with_query(<(legion::Entity, &Position, &Food)>::query()) // food
        .with_query(<(legion::Entity, &mut Position)>::query())
        .build(|cmd, world, (stats, event_log, food_stats, agent_event_log), (agent_query, food_query, agent_stats_query)| {
            let mut event_log = event_log.lock().unwrap();
            let agent_count = agent_query.iter(world).count();
            let food_count = food_query.iter(world).count();
            event_log.push(format!("[TICK] Agents: {}, Food: {}", agent_count, food_count));
            let mut interactions_this_tick = 0;
            let mut active_interactions = 0;
            let agents: Vec<_> = agent_query.iter(world).map(|(entity, pos, _)| (*entity, pos.x, pos.y)).collect();
            let foods: Vec<_> = food_query.iter(world).map(|(e, pos, food)| (*e, pos.x, pos.y, food.nutrition)).collect();
            let mut interacted = vec![false; agents.len()];
            let mut rng = rand::thread_rng();
            // Collect interaction events first
            let mut food_eaten: Vec<(legion::Entity, legion::Entity, f32)> = Vec::new();
            for i in 0..agents.len() {
                let (agent_entity, x, y) = agents[i];
                if !interacted[i] {
                    // Agent-agent interaction
                    for j in (i+1)..agents.len() {
                        let (other_entity, ox, oy) = agents[j];
                        if (x - ox).abs() < 1.5 && (y - oy).abs() < 1.5 {
                            interactions_this_tick += 1;
                            active_interactions += 1;
                            interacted[i] = true;
                            interacted[j] = true;
                            event_log.push(format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent_entity, other_entity));
                            break;
                        }
                    }
                    // Agent-food interaction (randomize food selection if multiple in range)
                    let mut foods_in_range: Vec<_> = foods.iter()
                        .filter(|(_food_e, fx, fy, _nutrition)| (x - *fx).abs() < 1.0 && (y - *fy).abs() < 1.0)
                        .collect();
                    if !foods_in_range.is_empty() {
                        use rand::seq::SliceRandom;
                        foods_in_range.shuffle(&mut rng);
                        let (food_e, _fx, _fy, nutrition) = *foods_in_range[0];
                        food_eaten.push((agent_entity, food_e, nutrition));
                    }
                }
            }
            // Second pass: apply food eaten mutations
            for (agent_entity, food_e, nutrition) in food_eaten {
                if let Some((_entity, _pos)) = agent_stats_query.iter_mut(world).find(|(e, _pos)| **e == agent_entity) {
                    agent_event_log.push(AgentEvent::AteFood {
                        agent: agent_entity,
                        food: food_e,
                        nutrition,
                    });
                }
                cmd.remove(food_e);
                food_stats.collected_per_tick += 1;
            }
            stats.agent_interactions += interactions_this_tick;
            stats.active_interactions = active_interactions;
            if stats.active_interactions_history.len() >= 100 {
                stats.active_interactions_history.pop_front();
            }
            stats.active_interactions_history.push_back(active_interactions);
        })
}

// --- EventLog moved to event_log.rs ---
