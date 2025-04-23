// Agent-related ECS systems will be moved here next.

use crate::agent::components::{AgentType, Hunger, Energy, AgentState, DecisionEngineConfig};
use crate::agent::mlp::{MLP, MLPConfig};
use crate::navigation::{Target, Path, pathfinding::a_star_path};
use crate::ecs_components::{Position, FoodPositions};
use crate::event_log::EventLog;
use crate::map::Map;
use legion::*;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

pub fn spawn_agent(world: &mut legion::World, pos: crate::ecs_components::Position, agent_type: crate::agent::components::AgentType, map: &crate::map::Map) -> legion::Entity {
    let _color = agent_type.color.clone();
    let mut rng = rand::thread_rng();
    let (tx, ty) = crate::navigation::random_passable_target(map, &agent_type, &mut rng, None);
    // Push with 8 components, then insert the 9th
    let entity = world.push((
        pos,
        agent_type,
        crate::agent::components::Hunger { value: 100.0 },
        crate::agent::components::Energy { value: 100.0 },
        crate::agent::components::InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0, recent_partners: VecDeque::new() },
        crate::navigation::Target { x: tx, y: ty, stuck_ticks: 0, path_ticks: None, ticks_to_reach: None },
        crate::navigation::Path { waypoints: std::collections::VecDeque::new() },
        AgentState::Idle,
    ));
    world.entry(entity).unwrap().add_component(crate::agent::components::MovementHistory::new(12));
    entity
}

// --- ECS Agent Path Following System ---
pub fn path_following_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("PathFollowingSystem")
        .with_query(<(Entity, &mut Position, &AgentType, &mut Hunger, &mut Energy, Option<&mut Target>, Option<&mut Path>, &mut AgentState)>::query())
        .read_resource::<Map>()
        .write_resource::<EventLog>()
        .build(|_, world, (_map, event_log), query| {
            for (entity, pos, agent_type, hunger, _energy, maybe_target, mut maybe_path, agent_state) in query.iter_mut(world) {
                match *agent_state {
                    AgentState::Moving => {
                        if let (Some(target), Some(path)) = (maybe_target.as_ref(), maybe_path.as_mut()) {
                            if let Some(next_wp) = path.waypoints.front() {
                                let dx = next_wp.0 - pos.x;
                                let dy = next_wp.1 - pos.y;
                                let dist = (dx * dx + dy * dy).sqrt();
                                let step = agent_type.move_speed.min(dist);
                                if dist < 0.2 {
                                    path.waypoints.pop_front();
                                    if path.waypoints.is_empty() {
                                        pos.x = target.x;
                                        pos.y = target.y;
                                        *agent_state = AgentState::Arrived;
                                        event_log.push(format!("[ARRIVE] Agent {:?} arrived at ({:.2}, {:.2})", entity, pos.x, pos.y));
                                    }
                                } else {
                                    pos.x += dx / dist * step;
                                    pos.y += dy / dist * step;
                                    hunger.value -= 0.01 * step;
                                    event_log.push(format!("[MOVE] Agent {:?} moved to ({:.2}, {:.2}) via path", entity, pos.x, pos.y));
                                }
                            } else {
                                // No waypoints left, snap to target if close
                                let dx = target.x - pos.x;
                                let dy = target.y - pos.y;
                                let dist = (dx * dx + dy * dy).sqrt();
                                if dist < 0.2 {
                                    pos.x = target.x;
                                    pos.y = target.y;
                                    *agent_state = AgentState::Arrived;
                                    event_log.push(format!("[ARRIVE] Agent {:?} arrived at ({:.2}, {:.2}) [no waypoints]", entity, pos.x, pos.y));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
}

// --- ECS Agent Action Selection System ---
pub fn action_selection_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("ActionSelectionSystem")
        .with_query(<(Entity, &mut Position, &AgentType, &mut Hunger, &mut Energy, Option<&mut Target>, Option<&mut Path>, &mut AgentState)>::query())
        .read_resource::<Map>()
        .read_resource::<FoodPositions>()
        .write_resource::<EventLog>()
        .build(|_, world, (_map, food_positions, event_log), query| {
            let mut rng = rand::thread_rng();
            let hunger_threshold = 50.0;
            let agent_infos: Vec<(Entity, f32, f32)> = query.iter_mut(world)
                .map(|(e, p, _a, _h, _e, _t, _pa, _s)| (*e, p.x, p.y))
                .collect();
            for (entity, pos, agent_type, hunger, _energy, mut maybe_target, mut maybe_path, agent_state) in query.iter_mut(world) {
                if *agent_state == AgentState::Idle || *agent_state == AgentState::Arrived {
                    let mut possible_actions = Vec::new();
                    let food_positions = &food_positions.0;
                    if hunger.value < hunger_threshold && !food_positions.is_empty() {
                        let far_food: Vec<_> = food_positions.iter()
                            .filter(|(fx, fy)| ((*fx - pos.x).powi(2) + (*fy - pos.y).powi(2)).sqrt() >= 10.0)
                            .collect();
                        if !far_food.is_empty() {
                            possible_actions.push("seek_food");
                        }
                    }
                    let mut far_agents = Vec::new();
                    for &(_other_entity, ax, ay) in agent_infos.iter() {
                        let d = ((ax - pos.x).powi(2) + (ay - pos.y).powi(2)).sqrt();
                        // Skipping recent interaction logic for now
                        if d >= 10.0 {
                            far_agents.push((ax, ay));
                        }
                    }
                    if !far_agents.is_empty() {
                        possible_actions.push("seek_agent");
                    }
                    possible_actions.push("wander");
                    possible_actions.push("idle");

                    // --- MLP Integration ---
                    let action = if let Some(DecisionEngineConfig::MLP(ref mlp_config)) = agent_type.decision_engine {
                        // Prepare input vector (minimal: hunger, pos.x, pos.y)
                        let input = vec![hunger.value / 100.0, pos.x / 100.0, pos.y / 100.0];
                        let mlp = MLP::from_config(mlp_config);
                        let output = mlp.forward(input);
                        // Choose the action with the highest output
                        let max_idx = output.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(i,_)| i).unwrap_or(0);
                        possible_actions.get(max_idx).unwrap_or(&"wander")
                    } else {
                        possible_actions.choose(&mut rng).unwrap_or(&"wander")
                    };

                    match *action {
                        "seek_food" => {
                            let far_food: Vec<_> = food_positions.iter()
                                .filter(|(fx, fy)| ((*fx - pos.x).powi(2) + (*fy - pos.y).powi(2)).sqrt() >= 10.0)
                                .collect();
                            if let Some((fx, fy)) = far_food.choose(&mut rng) {
                                if let Some(ref mut target) = maybe_target {
                                    target.x = *fx;
                                    target.y = *fy;
                                    event_log.push(format!("[TARGET][MLP] Agent {:?} seeks food at ({:.2}, {:.2})", entity, fx, fy));
                                    if let Some(ref mut path) = maybe_path {
                                        if let Some(astar_path) = a_star_path(_map, agent_type, (pos.x as i32, pos.y as i32), (*fx as i32, *fy as i32), 120) {
                                            path.waypoints = astar_path.into_iter().collect();
                                            event_log.push(format!("[PATHFIND] Path assigned: {} waypoints", path.waypoints.len()));
                                            *agent_state = AgentState::Moving;
                                        } else {
                                            path.waypoints.clear();
                                            event_log.push("[PATHFIND] No path found".to_string());
                                            *agent_state = AgentState::Idle;
                                        }
                                    }
                                }
                            }
                        },
                        "seek_agent" => {
                            if let Some((ax, ay)) = far_agents.choose(&mut rng) {
                                if let Some(ref mut target) = maybe_target {
                                    target.x = *ax;
                                    target.y = *ay;
                                    event_log.push(format!("[TARGET][MLP] Agent {:?} seeks to interact at ({:.2}, {:.2})", entity, ax, ay));
                                    if let Some(ref mut path) = maybe_path {
                                        if let Some(astar_path) = a_star_path(_map, agent_type, (pos.x as i32, pos.y as i32), (*ax as i32, *ay as i32), 120) {
                                            path.waypoints = astar_path.into_iter().collect();
                                            event_log.push(format!("[PATHFIND] Path assigned: {} waypoints", path.waypoints.len()));
                                            *agent_state = AgentState::Moving;
                                        } else {
                                            path.waypoints.clear();
                                            event_log.push("[PATHFIND] No path found".to_string());
                                            *agent_state = AgentState::Idle;
                                        }
                                    }
                                }
                            }
                        },
                        "wander" => {
                            // Existing wander logic
                            let (rx, ry) = crate::navigation::random_passable_target(
                                _map, agent_type, &mut rng, Some((pos.x, pos.y))
                            );
                            if let Some(ref mut target) = maybe_target {
                                target.x = rx;
                                target.y = ry;
                                event_log.push(format!("[TARGET] Agent {:?} wanders to ({:.2}, {:.2}) [local 120 units]", entity, rx, ry));
                                if let Some(ref mut path) = maybe_path {
                                    if let Some(astar_path) = a_star_path(_map, agent_type, (pos.x as i32, pos.y as i32), (rx as i32, ry as i32), 120) {
                                        path.waypoints = astar_path.into_iter().collect();
                                        event_log.push(format!("[PATHFIND] Path assigned: {} waypoints", path.waypoints.len()));
                                        *agent_state = AgentState::Moving;
                                    } else {
                                        path.waypoints.clear();
                                        event_log.push("[PATHFIND] No path found".to_string());
                                        *agent_state = AgentState::Idle;
                                    }
                                }
                            }
                        },
                        "idle" => {
                            *agent_state = AgentState::Idle;
                        },
                        _ => {}
                    }
                }
            }
        })
}

// --- ECS Agent Movement History System ---
pub fn agent_movement_history_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementHistorySystem")
        .with_query(<(&Position, &mut crate::agent::components::MovementHistory)>::query())
        .build(|_, world, _, query| {
            for (pos, history) in query.iter_mut(world) {
                history.push((pos.x, pos.y));
            }
        })
}

// --- ECS Agent Death System ---
pub fn agent_death_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDeathSystem")
        .with_query(<(legion::Entity, &Hunger, &Energy)>::query())
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
