use legion::systems::Builder;
use crate::ecs::systems::agent_spawn_log::agent_spawn_log_system;

pub fn add_agent_spawning_systems(builder: &mut Builder) {
    // [RF4] drain_agent_spawn_queue_system removed; all agent spawns now go directly into PendingAgentSpawns via ECS resource.
    // Spawn new agents
    // builder.add_system(crate::ecs::systems::agent_spawn::agent_spawning_system());
    // builder.flush();
    // Reactivated: agent_spawn_log_system
    //builder.add_system(agent_spawn_log_system());
    //builder.flush();
    // Add any other agent-related systems here as needed, but keep them commented out for now.
}
