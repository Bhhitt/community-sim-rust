//! ECS resource for initialization configuration (RF6)

use crate::agent::AgentType;

#[derive(Clone, Debug)]
pub struct InitConfig {
    pub agent_types: Vec<AgentType>,
    pub num_agents: usize,
    pub food_spawns: Vec<(f32, f32)>,
    pub agent_spawns: Vec<(f32, f32, AgentType)>,
    pub initialized: bool, // Add one-shot flag
    // Extend with additional fields as needed (e.g., items, money, etc.)
}

impl InitConfig {
    pub fn new(
        agent_types: Vec<AgentType>,
        num_agents: usize,
        food_spawns: Vec<(f32, f32)>,
        agent_spawns: Vec<(f32, f32, AgentType)>,
    ) -> Self {
        Self {
            agent_types,
            num_agents,
            food_spawns,
            agent_spawns,
            initialized: false,
        }
    }
}
