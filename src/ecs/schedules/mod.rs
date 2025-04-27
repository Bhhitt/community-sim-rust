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

    log::debug!("[SCHEDULE] Before food systems");
    food::add_food_systems(&mut builder);
    log::debug!("[SCHEDULE] After food systems");
    builder.flush();

    log::debug!("[SCHEDULE] Before spawning systems");
    spawning::add_agent_spawning_systems(&mut builder);
    log::debug!("[SCHEDULE] After spawning systems");
    builder.flush();

    log::debug!("[SCHEDULE] Before agent core systems");
    agent::add_agent_core_systems(&mut builder);
    log::debug!("[SCHEDULE] After agent core systems");
    builder.flush();

    log::debug!("[SCHEDULE] Before logging systems");
    logging::add_agent_logging_systems(&mut builder);
    log::debug!("[SCHEDULE] After logging systems");
    builder.flush();

    log::debug!("[SCHEDULE] Before interaction systems");
    interaction::add_interaction_systems(&mut builder);
    log::debug!("[SCHEDULE] After interaction systems");
    builder.flush();

    log::debug!("[SCHEDULE] Before death systems");
    death::add_agent_death_systems(&mut builder);
    log::debug!("[SCHEDULE] After death systems");
    builder.flush();

    log::debug!("[SCHEDULE] Before event log bridge system");
    event_log_bridge::add_agent_event_log_bridge_system(&mut builder);
    log::debug!("[SCHEDULE] After event log bridge system");
    builder.flush();

    builder.build()
}
