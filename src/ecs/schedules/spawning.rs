use legion::systems::Builder;

/// Deprecated: All agent spawning systems are now registered via agent core systems.
/// This module is retained for compatibility but is no longer used.
#[deprecated(note = "Agent spawning systems are now registered in agent.rs. This module is obsolete.")]
pub fn add_agent_spawning_systems(_builder: &mut Builder) {
    // No-op. Remove any calls to this function from the schedule builder.
}
