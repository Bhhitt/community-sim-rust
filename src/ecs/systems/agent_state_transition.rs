// ECS System: Agent State Transition System
// Updates the agent's state based on target/path assignment.

use legion::{systems::Runnable, SystemBuilder, IntoQuery};
use crate::agent::components::{Target, AgentState};
use crate::ecs_components::Position;

pub fn agent_state_transition_system() -> impl Runnable {
    SystemBuilder::new("AgentStateTransitionSystem")
        .with_query(<(&mut Position, Option<&Target>, &mut AgentState)>::query())
        .build(|_, world, _, query| {
            for (pos, maybe_target, agent_state) in query.iter_mut(world) {
                if *agent_state == AgentState::Moving || *agent_state == AgentState::Idle {
                    if let Some(target) = maybe_target {
                        let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                        if dist <= 0.1 {
                            *agent_state = AgentState::Arrived;
                        }
                    }
                }
            }
        })
}
