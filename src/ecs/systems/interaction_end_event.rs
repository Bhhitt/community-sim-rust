//! System to handle ending of agent-agent interactions and emit InteractionEnded events.

use legion::{systems::Runnable, SystemBuilder, Entity, IntoQuery, EntityStore};
use crate::ecs_components::{Interacting, InteractionQueue};
use crate::ecs::agent_events::{AgentEvent, AgentEventQueue};

/// System to detect when interactions end, emit InteractionEnded events, and trigger next in queue.
pub fn interaction_end_event_system() -> impl Runnable {
    SystemBuilder::new("InteractionEndEventSystem")
        .write_component::<Interacting>()
        .write_component::<InteractionQueue>()
        .write_resource::<AgentEventQueue>()
        .build(|cmd, world, events, _| {
            // Collect agents whose interaction just ended
            let mut ended_interactions = Vec::new();
            for (entity, interacting) in <(Entity, &Interacting)>::query().iter(world) {
                if interacting.ticks_remaining == 0 {
                    ended_interactions.push((entity.clone(), interacting.partner));
                }
            }
            // For each ended interaction:
            for (entity, partner) in ended_interactions {
                // Remove Interacting from both entities
                cmd.remove_component::<Interacting>(entity);
                cmd.remove_component::<Interacting>(partner);
                // Emit InteractionEnded event
                events.0.push(AgentEvent::InteractionEnded {
                    initiator: entity,
                    target: partner,
                });
                // Start next interaction for the partner (if any queued)
                if let Ok(mut entry) = world.entry_mut(partner) {
                    if let Ok(queue) = entry.get_component_mut::<InteractionQueue>() {
                        if let Some(next_agent) = queue.queue.pop_front() {
                            // Start new interaction
                            cmd.add_component(next_agent, Interacting { partner, ticks_remaining: 10 });
                            cmd.add_component(partner, Interacting { partner: next_agent, ticks_remaining: 10 });
                            events.0.push(AgentEvent::InteractionStarted {
                                initiator: next_agent,
                                target: partner,
                            });
                        }
                    }
                }
            }
        })
}
