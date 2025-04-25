// ECS Swimming System Implementation
// Moved from agent/swimming.rs for ECS modularity.

use legion::*;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use crate::ecs_components::Position;
use crate::agent::components::{SwimmingProfile, AgentState};
use crate::agent::Hunger;
use crate::agent::AgentType;
use crate::map::Map;
use crate::event_log::EventLog;
use crate::log_config::LogConfig;

/// System for random swimming movement and swim duration countdown.
pub fn swimming_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("SwimmingSystem")
        .with_query(<(Entity, &mut Position, &mut Hunger, &mut AgentState, &mut SwimmingProfile, &AgentType)>::query())
        .read_resource::<Map>()
        .write_resource::<Arc<Mutex<EventLog>>>()
        .read_resource::<LogConfig>()
        .build(|_cmd, world, (map, event_log, log_config), query| {
            let map = map;
            let mut rng = rand::thread_rng();
            for (entity, pos, hunger, agent_state, swimming_profile, agent_type) in query.iter_mut(world) {
                if swimming_profile.swim_ticks_remaining > 0 {
                    // Borrow map to call methods
                    let map_ref = map;
                    let water_neighbors: Vec<(f32, f32)> = map_ref
                        .neighbors(pos.x as i32, pos.y as i32)
                        .into_iter()
                        .filter(|&(wx, wy)| map_ref.is_water(wx, wy))
                        .map(|(wx, wy)| (wx as f32, wy as f32))
                        .collect();
                    if !water_neighbors.is_empty() {
                        let &(wx, wy) = water_neighbors.choose(&mut rng).unwrap();
                        pos.x = wx;
                        pos.y = wy;
                        hunger.value -= agent_type.hunger_rate;
                        if !log_config.quiet {
                            event_log.lock().unwrap().push(format!("[SWIM] Agent {:?} swims to ({:.2}, {:.2})", entity, pos.x, pos.y));
                        }
                    }
                    swimming_profile.swim_ticks_remaining = swimming_profile.swim_ticks_remaining.saturating_sub(1);
                }
            }
        })
}
