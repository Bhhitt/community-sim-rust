// Agent-Agent Interaction System
// Handles detection and logging of agent-agent interactions.

use legion::{Entity, IntoQuery, systems::Runnable, systems::SystemBuilder};
use crate::ecs_components::{Position, InteractionStats};
use crate::agent::{InteractionState};
use std::sync::{Arc, Mutex};
use crate::event_log::EventLog;

pub fn agent_agent_interaction_system() -> impl Runnable {
    SystemBuilder::new("AgentAgentInteractionSystem")
        .write_resource::<InteractionStats>()
        .write_resource::<Arc<Mutex<EventLog>>>()
        .with_query(<(Entity, &Position, &InteractionState)>::query()) // agents
        .build(|_cmd, world, (stats, event_log), agent_query| {
            let mut event_log = event_log.lock().unwrap();
            let agents: Vec<_> = agent_query.iter(world).map(|(entity, pos, _)| (*entity, pos.x, pos.y)).collect();
            let mut interacted = vec![false; agents.len()];
            let mut interactions_this_tick = 0;
            let mut active_interactions = 0;
            for i in 0..agents.len() {
                let (agent_entity, x, y) = agents[i];
                if !interacted[i] {
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
                }
            }
            stats.agent_interactions += interactions_this_tick;
            stats.active_interactions = active_interactions;
        })
}
