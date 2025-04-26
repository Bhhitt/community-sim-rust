mod food;
mod agent;
mod spawning;
mod logging;
mod interaction;
mod death;
mod event_log_bridge;

use legion::systems::Builder;

pub fn build_main_schedule() -> legion::Schedule {
    let mut builder = legion::Schedule::builder();

    food::add_food_systems(&mut builder);
    builder.flush();

    spawning::add_agent_spawning_systems(&mut builder);
    builder.flush();

    agent::add_agent_core_systems(&mut builder);
    builder.flush();

    logging::add_agent_logging_systems(&mut builder);
    builder.flush();

    interaction::add_interaction_systems(&mut builder);
    builder.flush();

    death::add_agent_death_systems(&mut builder);
    builder.flush();

    event_log_bridge::add_agent_event_log_bridge_system(&mut builder);
    builder.flush();

    builder.build()
}
