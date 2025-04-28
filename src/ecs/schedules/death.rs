use legion::systems::Builder;
use crate::agent::agent_death_system;
use log;

/// Adds all agent death/removal systems to the ECS schedule.
///
/// # System Order Dependency
/// This must run *after* agent hunger/energy update systems, so that agents are only removed
/// after their latest hunger/energy state is computed for the tick.
///
/// See: agent_death_system_audit.md for audit details.
pub fn add_agent_death_systems(builder: &mut Builder) {
    log::debug!("[ECS_SCHEDULE] About to add agent_death_system");
    builder.add_system(agent_death_system());
    builder.flush();
    log::debug!("[ECS_SCHEDULE] Finished agent_death_system");
}
