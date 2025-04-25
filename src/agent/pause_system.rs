// NOTE: This file exists separately from agent/systems.rs because Legion's SystemBuilder limits queries to 8 parameters.
// This system was split out to avoid exceeding that limit in the main agent system queries.
// If the parameter limit is ever increased or the query simplified, consider merging back for clarity.

use legion::*;
use crate::agent::components::IdlePause;

/// System to decrement IdlePause for all agents and log when unpaused.
pub fn agent_pause_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("AgentPauseSystem")
        .with_query(<(Entity, &mut IdlePause)>::query())
        .build(|_, world, _, query| {
            for (entity, idle_pause) in query.iter_mut(world) {
                log::debug!("[PAUSE] Agent {:?} ticks_remaining: {}", entity, idle_pause.ticks_remaining);
                if idle_pause.ticks_remaining > 0 {
                    idle_pause.ticks_remaining -= 1;
                    log::debug!("[PAUSE] Agent {:?} decremented to {}", entity, idle_pause.ticks_remaining);
                    if idle_pause.ticks_remaining == 0 {
                        log::debug!("[PAUSE] Agent {:?} is now unpaused", entity);
                    }
                }
            }
        })
}
