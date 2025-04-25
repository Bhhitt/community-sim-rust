use legion::*;
use rand::seq::SliceRandom;

/// System for random swimming movement and swim duration countdown.
pub fn swimming_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("SwimmingSystem")
        .with_query(<(Entity, &mut crate::ecs_components::Position, &mut crate::agent::Hunger, &mut crate::agent::AgentState, &mut crate::agent::components::SwimmingProfile, &crate::agent::AgentType)>::query())
        .read_resource::<crate::map::Map>()
        .write_resource::<crate::event_log::EventLog>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(move |_command_buffer, _world, resources, query| {
            let log_config = &resources.2;
            for (entity, pos, hunger, agent_state, swimming_profile, agent_type) in query.iter_mut(_world) {
                if *agent_state == crate::agent::AgentState::Swimming {
                    if swimming_profile.swim_ticks_remaining == 0 {
                        // Done swimming, become idle
                        *agent_state = crate::agent::AgentState::Idle;
                        if !log_config.quiet {
                            resources.1.push(format!("[SWIM] Agent {:?} finished swimming at ({:.2}, {:.2})", entity, pos.x, pos.y));
                        }
                        continue;
                    }
                    // Pick a random adjacent water tile (including diagonals)
                    let mut rng = rand::thread_rng();
                    let directions = [(-1,0),(1,0),(0,-1),(0,1),(-1,-1),(-1,1),(1,-1),(1,1)];
                    let mut water_neighbors = vec![];
                    for (dx, dy) in &directions {
                        let nx = pos.x as i32 + dx;
                        let ny = pos.y as i32 + dy;
                        if nx >= 0 && ny >= 0 && nx < resources.0.width && ny < resources.0.height {
                            if let crate::map::Terrain::Water = resources.0.tiles[ny as usize][nx as usize] {
                                water_neighbors.push((nx as f32, ny as f32));
                            }
                        }
                    }
                    if !water_neighbors.is_empty() {
                        let &(wx, wy) = water_neighbors.choose(&mut rng).unwrap();
                        pos.x = wx;
                        pos.y = wy;
                        hunger.value -= agent_type.hunger_rate;
                        if !log_config.quiet {
                            resources.1.push(format!("[SWIM] Agent {:?} swims to ({:.2}, {:.2})", entity, pos.x, pos.y));
                        }
                    }
                    swimming_profile.swim_ticks_remaining = swimming_profile.swim_ticks_remaining.saturating_sub(1);
                }
            }
        })
}
