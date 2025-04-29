// Interaction Stats Update System
// Updates InteractionStats and its history buffer.

use legion::systems::{Runnable, SystemBuilder};
use crate::ecs_components::InteractionStats;
use std::time::Instant;
use log::info;

pub fn interaction_stats_update_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("InteractionStatsUpdateSystem")
        .write_resource::<InteractionStats>()
        .build(|_cmd, _world, stats, ()| {
            let start = std::time::Instant::now();
            // Maintain the active_interactions_history buffer size
            let active = stats.active_interactions;
            if stats.active_interactions_history.len() >= 100 {
                stats.active_interactions_history.pop_front();
            }
            stats.active_interactions_history.push_back(active);
            let duration = start.elapsed();
            info!(target: "ecs_profile", "[PROFILE] System interaction_stats_update_system took {:?}", duration);
        })
}
