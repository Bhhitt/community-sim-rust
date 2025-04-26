// Interaction Stats Update System
// Updates InteractionStats and its history buffer.

use legion::systems::{Runnable, SystemBuilder};
use crate::ecs_components::InteractionStats;

pub fn interaction_stats_update_system() -> impl Runnable {
    SystemBuilder::new("InteractionStatsUpdateSystem")
        .write_resource::<InteractionStats>()
        .build(|_cmd, _world, stats, ()| {
            // Maintain the active_interactions_history buffer size
            let active = stats.active_interactions;
            if stats.active_interactions_history.len() >= 100 {
                stats.active_interactions_history.pop_front();
            }
            stats.active_interactions_history.push_back(active);
        })
}
