use legion::systems::Builder;

pub fn add_agent_logging_systems(builder: &mut Builder) {
    builder.add_system(crate::ecs::systems::agent_logging::agent_arrival_logging_system());
    builder.add_system(crate::ecs::systems::agent_logging::agent_move_logging_system());
    builder.add_system(crate::ecs::systems::agent_logging::agent_spawn_logging_system());
    builder.add_system(crate::ecs::systems::interaction_event_logging::interaction_event_logging_system());
}
