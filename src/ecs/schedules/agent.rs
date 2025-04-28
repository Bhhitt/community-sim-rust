use legion::systems::Builder;

pub fn add_agent_core_systems(builder: &mut Builder) {
    builder
        .add_system(crate::ecs::systems::agent::agent_pausing_system())
        .flush()
        .add_system(crate::ecs::systems::agent::agent_hunger_energy_system())
        .flush()
        .add_system(crate::ecs::systems::agent::agent_path_movement_system())
        .flush()
        .add_system(crate::ecs::systems::agent::agent_direct_movement_system())
        .flush()
        .add_system(crate::ecs::systems::agent_spawn::agent_spawning_system())
        .flush()
        .add_system(crate::ecs::systems::agent::agent_state_transition_system())
        .flush();
}
