//! Agent event types and event queue resource for event-driven agent behaviors.

use legion::Entity;
use crate::ecs_components::Position;

#[derive(Debug, Clone)]
pub enum AgentEvent {
    Arrived {
        entity: Entity,
        position: Position,
    },
    InteractionStarted {
        initiator: Entity,
        target: Entity,
    },
    InteractionEnded {
        initiator: Entity,
        target: Entity,
    },
    // Add more event variants as needed
}

#[derive(Default)]
pub struct AgentEventQueue(pub Vec<AgentEvent>);
