//! System to handle agent arrival events and perform state transitions or trigger new behaviors.

use legion::{systems::Runnable, Entity, World, IntoQuery, SystemBuilder, EntityStore};
use crate::agent::components::{AgentState};
use crate::ecs::agent_events::{AgentEvent, AgentEventQueue};

pub fn agent_arrival_event_system() -> impl Runnable {
    SystemBuilder::new("AgentArrivalEventSystem")
        .write_component::<AgentState>()
        .write_resource::<AgentEventQueue>()
        .build(|cmd, world, resources, _| {
            let events = resources;
            for event in events.0.iter() {
                match event {
                    AgentEvent::Arrived { entity, position } => {
                        if let Ok(mut entry) = world.entry_ref(*entity) {
                            if let Ok(agent_state) = entry.get_component::<AgentState>() {
                                if *agent_state == AgentState::Arrived {
                                    cmd.add_component(*entity, AgentState::Idle);
                                    log::debug!("[ARRIVAL_EVENT] Agent {:?} at ({:.2}, {:.2}) transitioned Arrived -> Idle", entity, position.x, position.y);
                                }
                            }
                        }
                    },
                    _ => {}, // Ignore other event types for now
                }
            }
            events.0.clear();
        })
}
