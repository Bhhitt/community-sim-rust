// Agent-related ECS systems will be moved here.

use crate::navigation::*;
use legion::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::VecDeque;
use std::io::Write;
use crate::agent::event::{AgentEvent, AgentEventLog};
use std::sync::{Arc, Mutex};

pub fn spawn_agent(world: &mut legion::World, pos: crate::ecs_components::Position, agent_type: crate::agent::AgentType, map: &crate::map::Map, agent_event_log: &mut AgentEventLog) -> legion::Entity {
    log::info!("[SPAWN_INFO] spawn_agent() called for agent type: {} at ({:.2},{:.2})", agent_type.name, pos.x, pos.y);
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("spawn_agent.log") {
        use std::io::Write;
        let _ = writeln!(file, "[SPAWN_FILE] Agent spawn_agent() called");
    }
    log::debug!("[SPAWN] Agent spawn_agent() called");
    std::io::stdout().flush().unwrap();
    let _color = agent_type.color.clone();
    let mut rng = rand::thread_rng();
    let (tx, ty) = random_passable_target(map, &agent_type, &mut rng, None);
    // Generate random swim_chance_percent (1-30) and add SwimmingProfile
    let swim_chance_percent = rand::Rng::gen_range(&mut rng, 1..=30);
    let swimming_profile = crate::agent::components::SwimmingProfile {
        swim_chance_percent,
        swim_ticks_remaining: 0,
    };
    // Push normal components
    let hunger_threshold = agent_type.hunger_threshold;
    let entity = world.push((
        pos,
        agent_type.clone(),
        crate::agent::Hunger { value: 100.0, threshold: hunger_threshold },
        crate::agent::Energy { value: 100.0 },
        crate::agent::InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0, recent_partners: VecDeque::new() },
        Target { x: tx, y: ty, stuck_ticks: 0, path_ticks: None, ticks_to_reach: None },
        Path { waypoints: VecDeque::new() },
        crate::agent::AgentState::Idle,
    ));
    agent_event_log.push(AgentEvent::Spawned {
        agent: entity,
        agent_type: agent_type.name.clone(),
        pos: (pos.x, pos.y),
    });
    log::info!("[SPAWN_INFO] Agent {:?} spawned at ({:.2},{:.2}) with state {:?}", entity, pos.x, pos.y, crate::agent::AgentState::Idle);
    log::debug!("[SPAWN] Agent {:?} spawned at ({:.2},{:.2}) with state {:?}", entity, pos.x, pos.y, crate::agent::AgentState::Idle);
    world.extend(vec![(entity, crate::agent::components::MovementHistory::new(12))]);
    world.extend(vec![(entity, swimming_profile)]);
    entity
}

// --- ECS Agent Path Following System ---
pub fn path_following_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("PathFollowingSystem")
        .with_query(<(Entity, &mut crate::ecs_components::Position, &crate::agent::AgentType, &mut crate::agent::Hunger, &mut crate::agent::Energy, Option<&mut Target>, Option<&mut Path>, &mut crate::agent::AgentState)>::query())
        .read_resource::<crate::map::Map>()
        .write_resource::<Arc<Mutex<crate::event_log::EventLog>>>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(move |_command_buffer, world, resources, query| {
            let log_config = &resources.2;
            log::info!("[DEBUG] PathFollowingSystem running. Iterating agents...");
            let mut moved = 0;
            let mut agent_count = 0;
            use std::time::Instant;
            let total_start = Instant::now();
            let move_time = 0.0f64;
            let mut waypoint_time = 0.0f64;
            let mut snap_time = 0.0f64;
            for (entity, pos, agent_type, hunger, _energy, mut maybe_target, mut maybe_path, agent_state) in query.iter_mut(world) {
                agent_count += 1;
                log::debug!("[PATHFOLLOW] Agent {:?} state: {:?} pos: ({:.2},{:.2}) hunger: {:.2}/{:.2} state: {:?}", entity, agent_state, pos.x, pos.y, hunger.value, hunger.threshold, agent_state);
                if *agent_state == crate::agent::AgentState::Idle
                    || *agent_state == crate::agent::AgentState::Arrived
                    || *agent_state == crate::agent::AgentState::Moving {
                    if let Some(target) = maybe_target.as_mut() {
                        if let Some(path) = maybe_path.as_mut() {
                            log::debug!("[PATHFOLLOW] Agent {:?} path waypoints: {:?}", entity, path.waypoints);
                            if !path.waypoints.is_empty() {
                                let wp_start = Instant::now();
                                let (tx, ty) = path.waypoints[0];
                                let dx = tx as f32 - pos.x;
                                let dy = ty as f32 - pos.y;
                                let dist = (dx * dx + dy * dy).sqrt();
                                let step = agent_type.movement_profile.speed.min(dist);
                                log::debug!("[PATHFOLLOW] Agent {:?} moving from ({:.2},{:.2}) toward ({:.2},{:.2}) step: {:.2} dist: {:.2}", entity, pos.x, pos.y, tx, ty, step, dist);
                                pos.x += dx / dist * step;
                                pos.y += dy / dist * step;
                                hunger.value -= agent_type.hunger_rate * step;
                                if !log_config.quiet {
                                    resources.1.lock().unwrap().push(format!("[MOVE] Agent {:?} moved to ({:.2}, {:.2}) via path (speed {:.2})", entity, pos.x, pos.y, agent_type.movement_profile.speed));
                                }
                                path.waypoints.pop_front();
                                waypoint_time += wp_start.elapsed().as_secs_f64();
                                moved += 1;
                            } else {
                                log::debug!("[PATHFOLLOW] Agent {:?} path empty, snapping to target ({:.2},{:.2})", entity, target.x, target.y);
                                let snap_start = Instant::now();
                                pos.x = target.x;
                                pos.y = target.y;
                                *agent_state = crate::agent::AgentState::Arrived;
                                if !log_config.quiet {
                                    resources.1.lock().unwrap().push(format!("[ARRIVE] Agent {:?} arrived at ({:.2}, {:.2}) [no waypoints]", entity, pos.x, pos.y));
                                }
                                snap_time += snap_start.elapsed().as_secs_f64();
                                moved += 1;
                            }
                        } else {
                            log::debug!("[PATHFOLLOW] Agent {:?} has no path, moving directly toward target ({:.2},{:.2})", entity, target.x, target.y);
                            let dist = ((target.x - pos.x).powi(2) + (target.y - pos.y).powi(2)).sqrt();
                            let step = agent_type.movement_profile.speed.min(dist);
                            log::debug!("[PATHFOLLOW] Agent {:?} direct move step: {:.2} dist: {:.2}", entity, step, dist);
                            if dist > 0.1 {
                                pos.x += (target.x - pos.x) / dist * step;
                                pos.y += (target.y - pos.y) / dist * step;
                                hunger.value -= agent_type.hunger_rate * step;
                                if !log_config.quiet {
                                    resources.1.lock().unwrap().push(format!("[MOVE] Agent {:?} moved to ({:.2}, {:.2}) (speed {:.2})", entity, pos.x, pos.y, agent_type.movement_profile.speed));
                                }
                                moved += 1;
                            } else {
                                log::debug!("[PATHFOLLOW] Agent {:?} close to target, snapping to ({:.2},{:.2})", entity, target.x, target.y);
                                pos.x = target.x;
                                pos.y = target.y;
                                *agent_state = crate::agent::AgentState::Arrived;
                                if !log_config.quiet {
                                    resources.1.lock().unwrap().push(format!("[ARRIVE] Agent {:?} arrived at ({:.2}, {:.2})", entity, pos.x, pos.y));
                                }
                                moved += 1;
                            }
                        }
                    }
                }
                // Always set Idle/Arrived agents back to Idle after reaching a target
                if *agent_state == crate::agent::AgentState::Arrived {
                    *agent_state = crate::agent::AgentState::Idle;
                }
            }
            log::info!("[DEBUG] PathFollowingSystem matched {} agents this tick", agent_count);
            let total_time = total_start.elapsed().as_secs_f64();
            if !log_config.quiet {
                log::info!("[PROFILE][PATH_FOLLOW] total: {:.6} move: {:.6} wp: {:.6} snap: {:.6}", total_time, move_time, waypoint_time, snap_time);
            }
            log::info!("[DEBUG] PathFollowingSystem finished. Total moved: {}", moved);
        })
}

// --- ECS Agent Action Selection System ---
pub fn action_selection_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("ActionSelectionSystem")
        .with_query(<(Entity, &mut crate::ecs_components::Position, &crate::agent::AgentType, &mut crate::agent::Hunger, &mut crate::agent::Energy, Option<&mut Target>, Option<&mut Path>, &mut crate::agent::AgentState)>::query())
        .read_resource::<crate::map::Map>()
        .read_resource::<crate::ecs_components::FoodPositions>()
        .write_resource::<Arc<Mutex<crate::event_log::EventLog>>>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(move |_command_buffer, world, resources, query| {
            let log_config = &resources.3;
            let mut rng = rand::thread_rng();
            let food_positions = &resources.1.0;
            let mut matched = 0;
            for (entity, pos, agent_type, hunger, _energy, mut maybe_target, mut maybe_path, agent_state) in query.iter_mut(world) {
                matched += 1;
                log::debug!("[ACTION] Matching agent {:?} state: {:?} hunger: {:.2}/{:.2}", entity, agent_state, hunger.value, hunger.threshold);
                if *agent_state == crate::agent::AgentState::Idle || *agent_state == crate::agent::AgentState::Arrived {
                    log::debug!("[ACTION_BRANCH] Agent {:?} entered Idle/Arrived branch. State: {:?} Hunger: {:.2}/{:.2}", entity, agent_state, hunger.value, hunger.threshold);
                    let mut possible_actions = Vec::new();
                    if hunger.value < agent_type.hunger_threshold && !food_positions.is_empty() {
                        let far_food: Vec<_> = food_positions.iter()
                            .filter(|(fx, fy)| ((*fx - pos.x).powi(2) + (*fy - pos.y).powi(2)).sqrt() >= 5.0)
                            .collect();
                        if let Some((fx, fy)) = far_food.get(0) {
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
                                        resources.2.lock().unwrap().push(format!("[TARGET][MLP] Agent {:?} seeks food at ({:.2}, {:.2})", entity, ax, ay));
                                    }
                                    if let Some(ref mut path) = maybe_path.as_mut() {
                                        if let Some(astar_path) = pathfinding::a_star_path(&resources.0, agent_type, agent_state, (pos.x as i32, pos.y as i32), (*ax as i32, *ay as i32), 120) {
                                            path.waypoints = std::collections::VecDeque::from_iter(astar_path.into_iter());
                                            if !log_config.quiet {
                                                resources.2.lock().unwrap().push(format!("[PATHFIND] Path assigned: {} waypoints", path.waypoints.len()));
                                            }
                                            *agent_state = crate::agent::AgentState::Moving;
                                        } else {
                                            if !log_config.quiet {
                                                resources.2.lock().unwrap().push("[PATHFIND] No path found".to_string());
                                            }
                                            *agent_state = crate::agent::AgentState::Idle;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        log::debug!("[WANDER_BRANCH] Agent {:?} entering wander branch. State: {:?} Hunger: {:.2}/{:.2}", entity, agent_state, hunger.value, hunger.threshold);
                        // Example wander: pick a random nearby tile
                        let rx = pos.x + rng.gen_range(-5.0..=5.0);
                        let ry = pos.y + rng.gen_range(-5.0..=5.0);
                        if let Some(ref mut target) = maybe_target.as_mut() {
                            target.x = rx;
                            target.y = ry;
                            if !log_config.quiet {
                                resources.2.lock().unwrap().push(format!("[TARGET] Agent {:?} wanders to ({:.2}, {:.2}) [local 120 units]", entity, rx, ry));
                            }
                            if let Some(ref mut path) = maybe_path.as_mut() {
                                if let Some(astar_path) = pathfinding::a_star_path(&resources.0, agent_type, agent_state, (pos.x as i32, pos.y as i32), (rx as i32, ry as i32), 120) {
                                    path.waypoints = std::collections::VecDeque::from_iter(astar_path.into_iter());
                                    if !log_config.quiet {
                                        resources.2.lock().unwrap().push(format!("[PATHFIND] Path assigned: {} waypoints", path.waypoints.len()));
                                    }
                                    *agent_state = crate::agent::AgentState::Moving;
                                } else {
                                    if !log_config.quiet {
                                        resources.2.lock().unwrap().push("[PATHFIND] No path found".to_string());
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

// --- ECS Agent Passive Hunger System ---
pub fn passive_hunger_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("PassiveHungerSystem")
        .with_query(<(&crate::agent::AgentType, &mut crate::agent::Hunger, &crate::agent::AgentState)>::query())
        .build(|_cmd, world, _res, query| {
            for (agent_type, hunger, agent_state) in query.iter_mut(world) {
                if *agent_state == crate::agent::AgentState::Idle || *agent_state == crate::agent::AgentState::Arrived {
                    // Reduce hunger by 10% of normal rate when not moving
                    hunger.value -= agent_type.hunger_rate * 0.1;
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
