// use std::sync::{Arc, Mutex};
// use once_cell::sync::Lazy;
use crate::ecs::systems::pending_agent_spawns::AgentSpawnRequest;

// [RF4] Global, thread-safe agent spawn queue removed. Use ECS resource PendingAgentSpawns instead.
// pub static AGENT_SPAWN_QUEUE: Lazy<Arc<Mutex<Vec<AgentSpawnRequest>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
