// Agent-related ECS systems will be moved here next.

use crate::agent::components::{AgentType, Hunger, Energy, InteractionState};
use crate::navigation::{Target, Path};
use crate::ecs_components::{Position, FoodPositions};
use crate::event_log::EventLog;
use crate::map::Map;
use legion::*;
use rand::Rng;
use std::collections::VecDeque;

pub fn spawn_agent(world: &mut legion::World, pos: crate::ecs_components::Position, agent_type: crate::agent::components::AgentType, map: &crate::map::Map) -> legion::Entity {
    let _color = agent_type.color.clone();
    let mut rng = rand::thread_rng();
    let (tx, ty) = crate::navigation::random_passable_target(map, &agent_type, &mut rng, None);
    world.push((pos, agent_type, crate::agent::components::Hunger { value: 100.0 }, crate::agent::components::Energy { value: 100.0 }, crate::agent::components::InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0, recent_partners: VecDeque::new() }, crate::navigation::Target { x: tx, y: ty, stuck_ticks: 0, path_ticks: None, ticks_to_reach: None }, crate::navigation::Path { waypoints: std::collections::VecDeque::new() }))
}

// --- ECS Agent Movement System ---
pub fn agent_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementSystem")
        .with_query(<(Entity, &mut Position, &AgentType, &mut Hunger, &mut Energy, Option<&mut Target>, Option<&mut Path>, Option<&mut InteractionState>)>::query())
        .read_resource::<Map>()
        .read_resource::<FoodPositions>()
        .write_resource::<EventLog>()
        .build(|_, world, (map, food_positions, event_log), query| {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            let hunger_threshold = 50.0; // Below this, agents seek food
            // Collect agent positions and entities for interaction targeting
            let agent_infos: Vec<(Entity, f32, f32)> = query.iter_mut(world)
                .map(|(e, p, _a, _h, _e, _t, _pa, _i)| (*e, p.x, p.y))
                .collect();
            for (entity, pos, agent_type, hunger, energy, mut maybe_target, _maybe_path, mut maybe_interaction) in query.iter_mut(world) {
                if let Some(ref mut interaction) = maybe_interaction {
                    interaction.update_recent();
                }
                let mut arrived = false;
                if let Some(ref mut target) = maybe_target {
                    let dx = target.x - pos.x;
                    let dy = target.y - pos.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist > 0.01 {
                        let speed = agent_type.move_speed.max(0.1);
                        let step = speed.min(dist);
                        pos.x += dx / dist * step;
                        pos.y += dy / dist * step;
                        hunger.value -= 0.01 * step;
                        energy.value -= 0.01 * step;
                        event_log.push(format!("[MOVE] Agent moved to ({:.2}, {:.2})", pos.x, pos.y));
                    } else {
                        arrived = true;
                    }
                }
                // Handle idle: if cooldown > 0, wander in place and decrement
                if let Some(ref mut interaction) = maybe_interaction {
                    if interaction.cooldown > 0 {
                        // Take a small random step (±1 or ±2)
                        let dx = rng.gen_range(-2.0..=2.0);
                        let dy = rng.gen_range(-2.0..=2.0);
                        let map_w = map.width as f32;
                        let map_h = map.height as f32;
                        pos.x = (pos.x + dx).clamp(0.0, map_w - 1.0);
                        pos.y = (pos.y + dy).clamp(0.0, map_h - 1.0);
                        interaction.cooldown -= 1;
                        event_log.push(format!("[IDLE-WANDER] Agent at ({:.2}, {:.2}) wanders while idling ({} steps left)", pos.x, pos.y, interaction.cooldown));
                        // Skip the rest of the logic for this tick
                        continue;
                    }
                }

                // If arrived, pick next action based on hunger, avoid recent partners
                if arrived {
                    use rand::seq::SliceRandom;
                    let mut possible_actions = Vec::new();
                    let food_positions = &food_positions.0;
                    // Option 1: Seek food if hungry and food exists, and food is at least 10 away
                    if hunger.value < hunger_threshold && !food_positions.is_empty() {
                        let far_food: Vec<_> = food_positions.iter()
                            .filter(|(fx, fy)| ((*fx - pos.x).powi(2) + (*fy - pos.y).powi(2)).sqrt() >= 10.0)
                            .collect();
                        if !far_food.is_empty() {
                            possible_actions.push("seek_food");
                        }
                    }
                    // Option 2: Seek another agent (at least 10 away, not recent partner)
                    let mut far_agents = Vec::new();
                    for &(other_entity, ax, ay) in agent_infos.iter() {
                        if other_entity != *entity {
                            let d = ((ax - pos.x).powi(2) + (ay - pos.y).powi(2)).sqrt();
                            let recently_interacted = maybe_interaction
                                .as_ref()
                                .map_or(false, |i| i.has_recently_interacted(other_entity));
                            if d >= 10.0 && !recently_interacted {
                                far_agents.push((other_entity, ax, ay));
                            }
                        }
                    }
                    if !far_agents.is_empty() {
                        possible_actions.push("seek_agent");
                    }
                    // Option 3: Wander (random point at least 10 away)
                    let map_w = map.width as f32;
                    let map_h = map.height as f32;
                    possible_actions.push("wander");
                    // Option 4: Idle (do nothing for 2-75 steps)
                    possible_actions.push("idle");
                    // Randomly pick an action (excluding repeating the last one if possible)
                    let action = possible_actions.choose(&mut rng).unwrap_or(&"wander");
                    match *action {
                        "seek_food" => {
                            let far_food: Vec<_> = food_positions.iter()
                                .filter(|(fx, fy)| ((*fx - pos.x).powi(2) + (*fy - pos.y).powi(2)).sqrt() >= 10.0)
                                .collect();
                            if let Some((fx, fy)) = far_food.choose(&mut rng) {
                                if let Some(ref mut target) = maybe_target {
                                    target.x = *fx;
                                    target.y = *fy;
                                    event_log.push(format!("[TARGET] Hungry agent at ({:.2}, {:.2}) seeks food at ({:.2}, {:.2})", pos.x, pos.y, fx, fy));
                                }
                            }
                        },
                        "seek_agent" => {
                            if let Some((other_entity, ax, ay)) = far_agents.choose(&mut rng) {
                                if let Some(ref mut target) = maybe_target {
                                    target.x = *ax;
                                    target.y = *ay;
                                    event_log.push(format!("[TARGET] Agent at ({:.2}, {:.2}) seeks to interact at ({:.2}, {:.2})", pos.x, pos.y, ax, ay));
                                    // Add to recent interaction list for this agent
                                    if let Some(ref mut interaction) = maybe_interaction {
                                        interaction.add_partner(*other_entity, rng.gen_range(5..=8));
                                    }
                                    // Repulsion force after interaction
                                    let repel_strength = 2.0;
                                    let repel_dx = pos.x - *ax;
                                    let repel_dy = pos.y - *ay;
                                    let repel_dist = (repel_dx * repel_dx + repel_dy * repel_dy).sqrt().max(0.01);
                                    pos.x += repel_dx / repel_dist * repel_strength;
                                    pos.y += repel_dy / repel_dist * repel_strength;
                                    event_log.push(format!("[REPEL] Agent at ({:.2}, {:.2}) repelled from ({:.2}, {:.2})", pos.x, pos.y, ax, ay));
                                }
                            }
                        },
                        "wander" => {
                            // Pick a random point within 120 units
                            let (rx, ry) = crate::navigation::random_passable_target(
                                map, agent_type, &mut rng, Some((pos.x, pos.y))
                            );
                            if let Some(ref mut target) = maybe_target {
                                target.x = rx;
                                target.y = ry;
                                event_log.push(format!("[TARGET] Agent at ({:.2}, {:.2}) wanders to ({:.2}, {:.2}) [local 120 units]", pos.x, pos.y, rx, ry));
                            }
                        },
                        "idle" => {
                            // Set cooldown to idle for 2-75 ticks
                            if let Some(ref mut interaction) = maybe_interaction {
                                interaction.cooldown = rng.gen_range(2..=75);
                                event_log.push(format!("[IDLE] Agent at ({:.2}, {:.2}) idles for {} ticks", pos.x, pos.y, interaction.cooldown));
                            }
                        },
                        _ => {}
                    }
                    // Update interaction state
                    if let Some(ref mut interaction) = maybe_interaction {
                        if *action != "idle" {
                            interaction.cooldown = 10;
                        }
                        interaction.ticks += 1;
                        event_log.push(format!("[ARRIVE] Agent at ({:.2}, {:.2}) triggered interaction/state change", pos.x, pos.y));
                    }
                }
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
