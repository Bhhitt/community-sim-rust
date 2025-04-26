use legion::systems::Builder;

pub fn add_agent_spawning_systems(builder: &mut Builder) {
    builder.add_system(crate::ecs::systems::drain_agent_spawn_queue::drain_agent_spawn_queue_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::agent_spawn::agent_spawning_system());
}
