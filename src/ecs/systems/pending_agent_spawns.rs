// ECS Resource: PendingAgentSpawns
// Holds pending agent spawn requests to be processed by the agent_spawning_system.

use crate::agent::AgentType;
use crate::ecs_components::Position;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AgentSpawnRequest {
    pub pos: Position,
    pub agent_type: AgentType,
}

#[derive(Debug, Default)]
pub struct PendingAgentSpawns(pub VecDeque<AgentSpawnRequest>);

impl PendingAgentSpawns {
    pub fn add(&mut self, pos: Position, agent_type: AgentType) {
        self.0.push_back(AgentSpawnRequest { pos, agent_type });
    }
    pub fn pop(&mut self) -> Option<AgentSpawnRequest> {
        self.0.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
