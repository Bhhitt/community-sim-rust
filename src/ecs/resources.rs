use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use legion::Resources;
use crate::event_log::EventLog;
use crate::agent::event::AgentEventLog;
use crate::log_config::LogConfig;
use crate::ecs_components::{FoodPositions, FoodStats, InteractionStats};
use crate::food::PendingFoodSpawns;
use crate::map::Map;
use crate::ecs::systems::pending_agent_spawns::PendingAgentSpawns;

pub fn insert_standard_resources(resources: &mut Resources, map: &Map) {
    resources.insert(map.clone());
    resources.insert(PendingFoodSpawns(VecDeque::new()));
    resources.insert(FoodPositions(Vec::new()));
    resources.insert(FoodStats::default());
    resources.insert(InteractionStats::default());
    resources.insert(Arc::new(Mutex::new(EventLog::new(200))));
    resources.insert(AgentEventLog::default());
    resources.insert(LogConfig::default());
    // Insert PendingAgentSpawns for agent spawning
    resources.insert(PendingAgentSpawns::default());
}
