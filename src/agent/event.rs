use legion::Entity;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum AgentEvent {
    Spawned {
        agent: Entity,
        agent_type: String,
        pos: (f32, f32),
    },
    Moved {
        agent: Entity,
        from: (f32, f32),
        to: (f32, f32),
    },
    AteFood {
        agent: Entity,
        food: Entity,
        nutrition: f32,
    },
    Interacted {
        agent: Entity,
        with: Entity,
    },
    StateChanged {
        agent: Entity,
        from: String,
        to: String,
    },
    // Add more event types as needed
}

impl AgentEvent {
    pub fn to_log_string(&self) -> String {
        match self {
            AgentEvent::Spawned { agent, agent_type, pos } =>
                format!("[SPAWNED] Agent {:?} (type: {}) at ({:.2}, {:.2})", agent, agent_type, pos.0, pos.1),
            AgentEvent::Moved { agent, from, to } =>
                format!("[MOVED] Agent {:?} from ({:.2}, {:.2}) to ({:.2}, {:.2})", agent, from.0, from.1, to.0, to.1),
            AgentEvent::AteFood { agent, food, nutrition } =>
                format!("[ATE] Agent {:?} ate food {:?} (+{:.1})", agent, food, nutrition),
            AgentEvent::Interacted { agent, with } =>
                format!("[INTERACTED] Agent {:?} with {:?}", agent, with),
            AgentEvent::StateChanged { agent, from, to } =>
                format!("[STATE] Agent {:?} changed state from {} to {}", agent, from, to),
        }
    }
}

// ECS resource to store agent events for the current tick
#[derive(Default)]
pub struct AgentEventLog(pub VecDeque<AgentEvent>);

impl AgentEventLog {
    pub fn push(&mut self, event: AgentEvent) {
        self.0.push_back(event);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}
