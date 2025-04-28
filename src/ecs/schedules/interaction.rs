use legion::systems::Builder;
use log;

pub fn add_interaction_systems(builder: &mut Builder) {
    log::debug!("[ECS_SCHEDULE] About to add agent_agent_interaction_system");
    builder.add_system(crate::ecs::systems::agent_agent_interaction::agent_agent_interaction_system());
    builder.flush();
    log::debug!("[ECS_SCHEDULE] Finished agent_agent_interaction_system");

    log::debug!("[ECS_SCHEDULE] About to add interaction_stats_update_system");
    builder.add_system(crate::ecs::systems::interaction_stats_update::interaction_stats_update_system());
    builder.flush();
    log::debug!("[ECS_SCHEDULE] Finished interaction_stats_update_system");
}
