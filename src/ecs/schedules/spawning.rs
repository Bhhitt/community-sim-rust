use legion::systems::Builder;
use crate::ecs::systems::agent_spawn_log::agent_spawn_log_system;

pub fn add_agent_spawning_systems(builder: &mut Builder) {
    // [RF4] drain_agent_spawn_queue_system removed; all agent spawns now go directly into PendingAgentSpawns via ECS resource.
    // Spawn new agents
    builder.add_system(crate::ecs::systems::agent_spawn::agent_spawning_system());
    builder.flush();
    builder.add_system(agent_spawn_log_system()); // still disabled
    builder.flush();
    // Other agent-related systems (commented out for now)
    builder.add_system(crate::ecs::systems::agent::agent_path_movement_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::agent::agent_direct_movement_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::agent_state_transition::agent_state_transition_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::agent::agent_movement_history_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::agent::agent_pausing_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::agent_hunger_energy_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::agent_logging::agent_spawn_logging_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::agent_logging::agent_arrival_logging_system());
    builder.flush();
    // Add any other agent-related systems here as needed, but keep them commented out for now.
}
