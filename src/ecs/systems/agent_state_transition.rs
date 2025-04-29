// ECS System: Agent State Transition System
// Updates the agent's state based on target/path assignment.

use legion::{systems::Runnable, SystemBuilder, IntoQuery, world::Entity};
use crate::agent::components::{Target, AgentState};
use crate::ecs_components::Position;
use crate::ecs::agent_events::{AgentEvent, AgentEventQueue};

pub fn agent_state_transition_system() -> impl Runnable {
    SystemBuilder::new("AgentStateTransitionSystem")
        .with_query(<(Entity, &mut Position, Option<&Target>, &mut AgentState)>::query())
        .write_resource::<AgentEventQueue>()
        .build(|_, world, resources, query| {
            let events = resources;
            for (entity, pos, maybe_target, agent_state) in query.iter_mut(world) {
                log::debug!("[STATE_TRANSITION] Agent {:?} at ({:.2}, {:.2}) current state: {:?}", entity, pos.x, pos.y, agent_state);
                if *agent_state == AgentState::Moving || *agent_state == AgentState::Idle {
                    if let Some(target) = maybe_target {
                        let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                        log::debug!("[STATE_TRANSITION] Distance to target: {:.3} (target=({:.2}, {:.2}))", dist, target.x, target.y);
                        if dist <= 0.1 {
                            log::debug!("[STATE_TRANSITION] Agent {:?} at ({:.2}, {:.2}) reached target, emitting Arrived event", entity, pos.x, pos.y);
                            events.0.push(AgentEvent::Arrived {
                                entity: *entity,
                                position: pos.clone(),
                            });
                            *agent_state = AgentState::Arrived;
                        }
                    } else {
                        log::debug!("[STATE_TRANSITION] No target assigned for agent {:?} at ({:.2}, {:.2})", entity, pos.x, pos.y);
                    }
                }
            }
        })
}
