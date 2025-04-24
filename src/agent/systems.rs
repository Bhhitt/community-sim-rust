// Agent-related ECS systems will be moved here.

use crate::navigation::*;
use crate::ecs_components::{Position, FoodPositions};
use legion::*;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

pub fn spawn_agent(world: &mut legion::World, pos: crate::ecs_components::Position, agent_type: crate::agent::AgentType, map: &crate::map::Map) -> legion::Entity {
    let _color = agent_type.color.clone();
    let mut rng = rand::thread_rng();
    let (tx, ty) = random_passable_target(map, &agent_type, &mut rng, None);
    // Push with 8 components, then insert the 9th
    let entity = world.push((
        pos,
        agent_type,
        crate::agent::Hunger { value: 100.0 },
        crate::agent::Energy { value: 100.0 },
        crate::agent::InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0, recent_partners: VecDeque::new() },
        Target { x: tx, y: ty, stuck_ticks: 0, path_ticks: None, ticks_to_reach: None },
        Path { waypoints: VecDeque::new() },
        crate::agent::AgentState::Idle,
    ));
    world.entry(entity).unwrap().add_component(crate::agent::components::MovementHistory::new(12));
    entity
}

// --- ECS Agent Path Following System ---
pub fn path_following_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("PathFollowingSystem")
        .with_query(<(Entity, &mut crate::ecs_components::Position, &crate::agent::AgentType, &mut crate::agent::Hunger, &mut crate::agent::Energy, Option<&mut Target>, Option<&mut Path>, &mut crate::agent::AgentState)>::query())
        .read_resource::<crate::map::Map>()
        .write_resource::<crate::event_log::EventLog>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(move |_command_buffer, world, resources, query| {
            let log_config = &resources.2;
            use std::time::Instant;
            let total_start = Instant::now();
            let mut move_time = 0.0f64;
            let mut waypoint_time = 0.0f64;
            let mut snap_time = 0.0f64;
            for (entity, pos, _agent_type, hunger, _energy, mut maybe_target, mut maybe_path, agent_state) in query.iter_mut(world) {
                if let Some(target) = maybe_target.as_mut() {
                    if let Some(path) = maybe_path.as_mut() {
                        if !path.waypoints.is_empty() {
                            let wp_start = Instant::now();
                            let (tx, ty) = path.waypoints[0];
                            let dx = tx as f32 - pos.x;
                            let dy = ty as f32 - pos.y;
                            let dist = (dx * dx + dy * dy).sqrt();
                            let step = 1.0f32.min(dist);
                            pos.x += dx / dist * step;
                            pos.y += dy / dist * step;
                            hunger.value -= 0.01 * step;
                            if !log_config.quiet {
                                resources.1.push(format!("[MOVE] Agent {:?} moved to ({:.2}, {:.2}) via path", entity, pos.x, pos.y));
                            }
                            path.waypoints.remove(0);
                            waypoint_time += wp_start.elapsed().as_secs_f64();
                        } else {
                            let snap_start = Instant::now();
                            pos.x = target.x;
                            pos.y = target.y;
                            *agent_state = crate::agent::AgentState::Arrived;
                            if !log_config.quiet {
                                resources.1.push(format!("[ARRIVE] Agent {:?} arrived at ({:.2}, {:.2}) [no waypoints]", entity, pos.x, pos.y));
                            }
                            snap_time += snap_start.elapsed().as_secs_f64();
                        }
                    } else {
                        let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                        let step = 1.0f32.min(dist);
                        if dist > 0.1 {
                            pos.x += (target.x - pos.x) / dist * step;
                            pos.y += (target.y - pos.y) / dist * step;
                            hunger.value -= 0.01 * step;
                            if !log_config.quiet {
                                resources.1.push(format!("[MOVE] Agent {:?} moved to ({:.2}, {:.2})", entity, pos.x, pos.y));
                            }
                        } else {
                            pos.x = target.x;
                            pos.y = target.y;
                            *agent_state = crate::agent::AgentState::Arrived;
                            if !log_config.quiet {
                                resources.1.push(format!("[ARRIVE] Agent {:?} arrived at ({:.2}, {:.2})", entity, pos.x, pos.y));
                            }
                        }
                    }
                }
            }
            let total_time = total_start.elapsed().as_secs_f64();
            if !log_config.quiet {
                log::info!("[PROFILE][PATH_FOLLOW] total: {:.6} move: {:.6} wp: {:.6} snap: {:.6}", total_time, move_time, waypoint_time, snap_time);
            }
        })
}

// --- ECS Agent Action Selection System ---
pub fn action_selection_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("ActionSelectionSystem")
        .with_query(<(Entity, &mut crate::ecs_components::Position, &crate::agent::AgentType, &mut crate::agent::Hunger, &mut crate::agent::Energy, Option<&mut Target>, Option<&mut Path>, &mut crate::agent::AgentState)>::query())
        .read_resource::<crate::map::Map>()
        .read_resource::<crate::ecs_components::FoodPositions>()
        .write_resource::<crate::event_log::EventLog>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(move |_command_buffer, world, resources, query| {
            let log_config = &resources.3;
            let mut rng = rand::thread_rng();
            let hunger_threshold = 50.0;
            let food_positions = &resources.1.0;
            for (entity, pos, agent_type, hunger, _energy, mut maybe_target, mut maybe_path, agent_state) in query.iter_mut(world) {
                if *agent_state == crate::agent::AgentState::Idle || *agent_state == crate::agent::AgentState::Arrived {
                    let mut possible_actions = Vec::new();
                    if hunger.value < hunger_threshold && !food_positions.is_empty() {
                        let far_food: Vec<_> = food_positions.iter()
                            .filter(|(fx, fy)| ((*fx - pos.x).powi(2) + (*fy - pos.y).powi(2)).sqrt() >= 10.0)
                            .collect();
                        if let Some((fx, fy)) = far_food.choose(&mut rng) {
                            possible_actions.push(("seek_food", *fx, *fy));
                        }
                    }
                    if let Some((action, ax, ay)) = possible_actions.choose(&mut rng) {
                        match *action {
                            "seek_food" => {
                                if let Some(ref mut target) = maybe_target.as_mut() {
                                    target.x = *ax;
                                    target.y = *ay;
                                    if !log_config.quiet {
                                        resources.2.push(format!("[TARGET][MLP] Agent {:?} seeks food at ({:.2}, {:.2})", entity, ax, ay));
                                    }
                                    if let Some(ref mut path) = maybe_path.as_mut() {
                                        if let Some(astar_path) = pathfinding::a_star_path(&resources.0, agent_type, (pos.x as i32, pos.y as i32), (*ax as i32, *ay as i32), 120) {
                                            path.waypoints = astar_path.into_iter().collect();
                                            if !log_config.quiet {
                                                resources.2.push(format!("[PATHFIND] Path assigned: {} waypoints", path.waypoints.len()));
                                            }
                                            *agent_state = crate::agent::AgentState::Moving;
                                        } else {
                                            if !log_config.quiet {
                                                resources.2.push("[PATHFIND] No path found".to_string());
                                            }
                                            *agent_state = crate::agent::AgentState::Idle;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        // Wander
                        let (rx, ry) = random_passable_target(&resources.0, agent_type, &mut rng, Some((pos.x, pos.y)));
                        if let Some(ref mut target) = maybe_target.as_mut() {
                            target.x = rx;
                            target.y = ry;
                            if !log_config.quiet {
                                resources.2.push(format!("[TARGET] Agent {:?} wanders to ({:.2}, {:.2}) [local 120 units]", entity, rx, ry));
                            }
                            if let Some(ref mut path) = maybe_path.as_mut() {
                                if let Some(astar_path) = pathfinding::a_star_path(&resources.0, agent_type, (pos.x as i32, pos.y as i32), (rx as i32, ry as i32), 120) {
                                    path.waypoints = astar_path.into_iter().collect();
                                    if !log_config.quiet {
                                        resources.2.push(format!("[PATHFIND] Path assigned: {} waypoints", path.waypoints.len()));
                                    }
                                    *agent_state = crate::agent::AgentState::Moving;
                                } else {
                                    if !log_config.quiet {
                                        resources.2.push("[PATHFIND] No path found".to_string());
                                    }
                                    *agent_state = crate::agent::AgentState::Idle;
                                }
                            }
                        }
                    }
                }
            }
        })
}

// --- ECS Agent Movement History System ---
pub fn agent_movement_history_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementHistorySystem")
        .with_query(<(&crate::ecs_components::Position, &mut crate::agent::components::MovementHistory)>::query())
        .build(|_, world, _, query| {
            for (pos, history) in query.iter_mut(world) {
                history.add(pos.x, pos.y);
            }
        })
}

// --- ECS Agent Death System ---
pub fn agent_death_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDeathSystem")
        .with_query(<(legion::Entity, &crate::agent::Hunger, &crate::agent::Energy)>::query())
        .build(|cmd, _world, _, _query| {
            let mut to_remove = Vec::new();
            for (entity, hunger, energy) in _query.iter(_world) {
                if hunger.value <= 0.0 || energy.value <= 0.0 {
                    to_remove.push(entity);
                }
            }
            for entity in to_remove {
                cmd.remove(*entity);
            }
        })
}
