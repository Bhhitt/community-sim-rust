mod food;
mod agent;
mod spawning;
mod logging;
mod interaction;
mod death;
mod event_log_bridge;
pub mod init;

use legion::systems::Builder;

/// Helper to wrap a Legion system with debug logging before execution
pub fn log_system<S>(name: &'static str, system: S) -> impl FnMut(&mut legion::world::World, &mut legion::systems::Resources)
where
    S: FnMut(&mut legion::world::World, &mut legion::systems::Resources) + 'static,
{
    let mut sys = system;
    move |world, resources| {
        log::debug!(target: "ecs_system", "Running system: {}", name);
        sys(world, resources);
    }
}

/// Helper to wrap a Legion system with timing and logging
pub fn profile_system<S>(name: &'static str, mut system: S) -> impl FnMut(&mut legion::world::World, &mut legion::systems::Resources)
where
    S: FnMut(&mut legion::world::World, &mut legion::systems::Resources) + 'static,
{
    move |world, resources| {
        let start = std::time::Instant::now();
        system(world, resources);
        let duration = start.elapsed();
        log::info!(target: "ecs_profile", "[PROFILE] System {} took {:?}", name, duration);
    }
}

pub fn build_main_schedule() -> legion::Schedule {
    let mut builder = legion::Schedule::builder();

    // --- Initialization systems (RF6) ---
    init::add_init_systems(&mut builder);
    builder.flush();

    // --- Food systems ---
    food::add_food_systems(&mut builder);
    builder.flush();

    // --- Agent core systems ---
    agent::add_agent_core_systems(&mut builder);
    builder.flush();

    // --- Logging systems ---
    logging::add_agent_logging_systems(&mut builder);
    builder.flush();

    // --- Interaction systems ---
    interaction::add_interaction_systems(&mut builder);
    builder.flush();

    // --- Death systems ---
    death::add_agent_death_systems(&mut builder);
    builder.flush();

    // --- Event log bridge system ---
    event_log_bridge::add_agent_event_log_bridge_system(&mut builder);
    builder.flush();

    builder.build()
}
