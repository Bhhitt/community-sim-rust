use legion::systems::Builder;

pub fn add_interaction_systems(builder: &mut Builder) {
    builder.add_system(crate::ecs::systems::agent_agent_interaction::agent_agent_interaction_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::interaction_stats_update::interaction_stats_update_system());
    builder.flush();
}
