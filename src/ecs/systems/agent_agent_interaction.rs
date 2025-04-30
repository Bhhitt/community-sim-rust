// Agent-Agent Interaction System
// Handles detection and logging of agent-agent interactions.

use legion::systems::Runnable;
use legion::{SystemBuilder, Entity, IntoQuery, EntityStore};
use crate::ecs_components::{Position, InteractionStats, InteractionIntent, Interacting, InteractionQueue};
use crate::ecs::agent_events::{AgentEvent, AgentEventQueue};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::event_log::EventLog;
use std::time::Instant;
use log;
use std::collections::HashMap; // Fix: add back HashMap import for pursuit_movement_system
use crate::agent::components::{InteractionState, RecentInteraction};

pub fn agent_agent_interaction_system() -> impl Runnable {
    SystemBuilder::new("AgentAgentInteractionSystem")
        .write_resource::<InteractionStats>()
        .write_resource::<Arc<Mutex<EventLog>>>()
        .with_query(<(Entity, &Position, &InteractionIntent)>::query()) // agents
        .with_query(<(Entity, &mut InteractionState)>::query()) // for updating recent partners
        .build(|_cmd, world, (stats, event_log), (agent_query, interactionstate_query)| {
            let start = Instant::now();
            let mut event_log = match event_log.lock() {
                Ok(lock) => lock,
                Err(e) => {
                    log::error!("[INTERACT_LOG] Failed to acquire lock on event_log: {}", e);
                    return;
                }
            };
            let agents: Vec<_> = agent_query.iter(world).map(|(entity, pos, _)| (*entity, pos.x, pos.y)).collect();
            let mut interacted = vec![false; agents.len()];
            let mut interactions_this_tick = 0;
            let mut active_interactions = 0;
            let mut interacted_pairs = Vec::new();
            for i in 0..agents.len() {
                let (agent_entity, x, y) = agents[i];
                if !interacted[i] {
                    for j in (i+1)..agents.len() {
                        let (other_entity, ox, oy) = agents[j];
                        if (x - ox).abs() < 1.5 && (y - oy).abs() < 1.5 {
                            // Check recent partners for both agents
                            let mut skip = false;
                            for (state_entity, mut state) in interactionstate_query.iter_mut(world) {
                                if *state_entity == agent_entity || *state_entity == other_entity {
                                    if state.recent_partners.iter().any(|ri| ri.partner == Some(other_entity)) {
                                        skip = true;
                                        break;
                                    }
                                }
                            }
                            if skip { continue; }
                            interactions_this_tick += 1;
                            active_interactions += 1;
                            interacted[i] = true;
                            interacted[j] = true;
                            interacted_pairs.push((agent_entity, other_entity));
                            event_log.push(format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent_entity, other_entity));
                            break;
                        }
                    }
                }
            }
            // After all interactions, update recent partners
            for (a, b) in interacted_pairs {
                for (state_entity, mut state) in interactionstate_query.iter_mut(world) {
                    if *state_entity == a {
                        state.recent_partners.push_back(RecentInteraction { partner: Some(b), ticks_since: 0 });
                    } else if *state_entity == b {
                        state.recent_partners.push_back(RecentInteraction { partner: Some(a), ticks_since: 0 });
                    }
                }
            }
            // Increment ticks_since for all recent partners and remove any older than 20 ticks
            for (_state_entity, mut state) in interactionstate_query.iter_mut(world) {
                state.recent_partners.iter_mut().for_each(|ri| ri.ticks_since += 1);
                state.recent_partners.retain(|ri| ri.ticks_since <= 20);
            }
            stats.agent_interactions += interactions_this_tick;
            log::info!("[INTERACT_STATS] Total agent interactions: {} (added {} this tick)", stats.agent_interactions, interactions_this_tick);
            stats.active_interactions = active_interactions;
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_agent_interaction_system took {:?}", duration);
        })
}

/// System that assigns InteractionIntent to idle agents who detect a valid target nearby.
pub fn intent_assignment_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("IntentAssignmentSystem")
        .with_query(<(legion::Entity, &Position)>::query())
        .build(|cmd, world, _resources, query| {
            log::debug!("[INTENT_ASSIGN] intent_assignment_system running");
            // Collect all agents and their positions
            let all_agents: Vec<(legion::Entity, f32, f32)> = query.iter(world)
                .map(|(entity, pos)| (*entity, pos.x, pos.y))
                .collect();
            log::debug!("[INTENT_ASSIGN] Total agents matched by query: {}", all_agents.len());
            for (entity, x, y) in &all_agents {
                // Skip if already has InteractionIntent or Interacting
                if world.entry_ref(*entity).map_or(false, |entry| entry.get_component::<InteractionIntent>().is_ok()) {
                    continue;
                }
                if world.entry_ref(*entity).map_or(false, |entry| entry.get_component::<Interacting>().is_ok()) {
                    continue;
                }
                // Find nearest eligible target (not self, not Interacting)
                let mut nearest: Option<(legion::Entity, f32)> = None;
                for (other, ox, oy) in &all_agents {
                    if other == entity { continue; }
                    if world.entry_ref(*other).map_or(false, |entry| entry.get_component::<Interacting>().is_ok()) {
                        continue;
                    }
                    let dist2 = (x - ox).powi(2) + (y - oy).powi(2);
                    if nearest.is_none() || dist2 < nearest.unwrap().1 {
                        nearest = Some((*other, dist2));
                    }
                }
                if let Some((target, _)) = nearest {
                    cmd.add_component(
                        *entity,
                        InteractionIntent {
                            target: Some(target),
                            ticks_pursued: 0,
                            max_pursue_ticks: 50,
                        },
                    );
                    log::debug!("[INTENT_ASSIGN] Assigned InteractionIntent to {:?} targeting {:?}", entity, target);
                }
            }
        })
}

/// System that moves agents with InteractionIntent toward their target, increments pursuit ticks, and removes the intent if pursuit fails or target disappears.
pub fn pursuit_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("PursuitMovementSystem")
        .with_query(<(legion::Entity, &mut Position, &mut InteractionIntent)>::query())
        .build(|cmd, world, _resources, query| {
            // Collect all positions into a hashmap to avoid double borrow
            let positions: HashMap<Entity, Position> = <(Entity, &Position)>::query()
                .iter(world)
                .map(|(e, p)| (*e, *p))
                .collect();
            let mut matched = false;
            for (entity, pos, intent) in query.iter_mut(world) {
                matched = true;
                if let Some(target) = intent.target {
                    if let Some(target_pos) = positions.get(&target) {
                        // Move agent toward target (simple step)
                        let dx = target_pos.x - pos.x;
                        let dy = target_pos.y - pos.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist > 0.01 {
                            let step = 1.0_f32.min(dist); // step size
                            pos.x += dx / dist * step;
                            pos.y += dy / dist * step;
                        }
                        intent.ticks_pursued += 1;
                        log::info!("[pursuit] Entity {:?} pursuing {:?}: pos=({:.2},{:.2}), ticks_pursued={}, max_pursue_ticks={}", entity, intent.target, pos.x, pos.y, intent.ticks_pursued, intent.max_pursue_ticks);
                        if intent.ticks_pursued >= intent.max_pursue_ticks {
                            log::info!("[pursuit] Entity {:?} removing InteractionIntent: reached max ticks", entity);
                            cmd.remove_component::<InteractionIntent>(*entity);
                        }
                    } else {
                        log::info!("[pursuit] Entity {:?} removing InteractionIntent: target {:?} not found (gone or no position)", entity, intent.target);
                        // Target gone
                        cmd.remove_component::<InteractionIntent>(*entity);
                    }
                } else {
                    log::info!("[pursuit] Entity {:?} removing InteractionIntent: no target set", entity);
                    cmd.remove_component::<InteractionIntent>(*entity);
                }
            }
            if !matched {
                log::debug!("[pursuit] No agents matched by (Entity, &mut Position, &mut InteractionIntent) query");
            }
        })
}

/// System that checks if agents with InteractionIntent are in range of their target, queues them, and starts interaction if possible.
pub fn interaction_range_system() -> impl legion::systems::Runnable {
    use std::collections::VecDeque;
    const INTERACTION_RANGE: f32 = 2.0;

    legion::SystemBuilder::new("InteractionRangeSystem")
        .with_query(<(legion::Entity, &Position, &InteractionIntent)>::query())
        .with_query(<(&mut InteractionQueue,)>::query())
        .with_query(<(&Interacting,)>::query())
        .write_resource::<AgentEventQueue>()
        .build(|cmd, world, events, (intent_query, queue_query, interacting_query)| {
            // Collect all positions for lookup
            let positions: std::collections::HashMap<Entity, Position> = <(Entity, &Position)>::query()
                .iter(world)
                .map(|(e, p)| (*e, *p))
                .collect();
            // Collect all agents with InteractionIntent and their targets
            let mut to_process = Vec::new();
            for (agent_entity, agent_pos, intent) in intent_query.iter(world) {
                to_process.push((*agent_entity, *agent_pos, intent.target));
            }
            if to_process.is_empty() {
                log::debug!("[range] No agents matched by (Entity, &Position, &InteractionIntent) query");
            }
            for (agent_entity, agent_pos, target) in to_process {
                if let Some(target_entity) = target {
                    if let (Some(target_pos), Some(_agent_pos)) = (positions.get(&target_entity), positions.get(&agent_entity)) {
                        let dx = target_pos.x - agent_pos.x;
                        let dy = target_pos.y - agent_pos.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist <= INTERACTION_RANGE {
                            // Try to mutably borrow target's InteractionQueue
                            match world.entry_mut(target_entity) {
                                Ok(mut entry) => {
                                    match entry.get_component_mut::<InteractionQueue>() {
                                        Ok(queue) => {
                                            if !queue.queue.contains(&agent_entity) {
                                                queue.queue.push_back(agent_entity);
                                                log::info!("[range] Agent {:?} queued for interaction with {:?}", agent_entity, target_entity);
                                            }
                                        }
                                        Err(_) => {
                                            log::warn!("[range] Target entity {:?} does not have InteractionQueue (queue phase)", target_entity);
                                            continue;
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::warn!("[range] Could not mutably borrow target entity {:?} (queue phase): {}", target_entity, e);
                                    continue;
                                }
                            }
                            // Remove agent's InteractionIntent (they are now waiting in queue)
                            cmd.remove_component::<InteractionIntent>(agent_entity);
                            // If target is not currently Interacting, start interaction with next in queue
                            let target_is_interacting = interacting_query.get(world, target_entity).is_ok();
                            if !target_is_interacting {
                                match world.entry_mut(target_entity) {
                                    Ok(mut entry) => {
                                        match entry.get_component_mut::<InteractionQueue>() {
                                            Ok(queue) => {
                                                if let Some(next_agent) = queue.queue.pop_front() {
                                                    cmd.add_component(next_agent, Interacting { partner: target_entity, ticks_remaining: 5 });
                                                    cmd.add_component(target_entity, Interacting { partner: next_agent, ticks_remaining: 5 });
                                                    log::info!("[range] Started interaction: {:?} <-> {:?}", next_agent, target_entity);
                                                    events.0.push(AgentEvent::InteractionStarted {
                                                        initiator: next_agent,
                                                        target: target_entity,
                                                    });
                                                }
                                            }
                                            Err(_) => {
                                                log::warn!("[range] Target entity {:?} does not have InteractionQueue (start phase)", target_entity);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log::warn!("[range] Could not mutably borrow target entity {:?} (start phase): {}", target_entity, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
}
