// ECS System: Agent Action Decision System
// Decides the next action for each agent (rules-based or MLP-based) and writes it to the IntendedAction component.

use legion::{Entity, IntoQuery, systems::Runnable, systems::SystemBuilder};
use crate::agent::components::{IntendedAction, AgentState, Hunger};
use crate::agent::AgentType;
use std::time::Instant;
use log::info;

pub fn agent_action_decision_system() -> impl Runnable {
    SystemBuilder::new("AgentActionDecisionSystem")
        .with_query(<(Entity, &AgentType, &Hunger, &AgentState)>::query())
        .write_component::<IntendedAction>()
        .build(|cmd, world, _, query| {
            let start = Instant::now();
            for (entity, _agent_type, hunger, agent_state) in query.iter(world) {
                // Only select actions for agents that are Idle (not paused, not moving, not arrived)
                if *agent_state != AgentState::Idle {
                    continue;
                }
                // TODO: Use agent_type.decision_engine to select rules or MLP
                // For now, implement rules-based as placeholder
                let intended_action = if hunger.value < hunger.threshold {
                    IntendedAction::SeekFood
                } else {
                    IntendedAction::Wander
                };
                cmd.add_component(*entity, intended_action);
            }
            let duration = start.elapsed();
            info!(target: "ecs_profile", "[PROFILE] System agent_action_decision_system took {:?}", duration);
        })
}
