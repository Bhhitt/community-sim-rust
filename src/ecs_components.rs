//! ECS Components for community simulator (Legion)
use legion;
use legion::IntoQuery;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AgentType {
    pub name: &'static str,
    pub move_speed: f32,
    pub color: &'static str,
    // Add other agent properties as needed
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hunger {
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Energy {
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub icon: char,
    pub color: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Food;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InteractionState {
    pub target: Option<legion::Entity>,
    pub ticks: u32,
    pub last_partner: Option<legion::Entity>,
    pub cooldown: u32,
}

#[derive(Default)]
pub struct InteractionStats {
    pub agent_interactions: usize,
    pub active_interactions: usize,
    pub active_interactions_history: VecDeque<usize>,
}

// --- Entity Spawning Functions ---
pub fn spawn_agent(world: &mut legion::World, pos: Position, agent_type: AgentType) -> legion::Entity {
    let color = agent_type.color;
    world.push((pos, agent_type.clone(), Hunger { value: 100.0 }, Energy { value: 100.0 }, Renderable { icon: '@', color }, InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0 }))
}

pub fn spawn_food(world: &mut legion::World, pos: Position) -> legion::Entity {
    world.push((pos, Food, Renderable { icon: '*', color: "green" }))
}

// --- ECS Agent Movement System ---
use crate::map::Map;
use rand::Rng;

pub fn agent_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementSystem")
        .with_query(<(&mut Position, &AgentType, &mut Hunger, &mut Energy)>::query())
        .read_resource::<Map>()
        .build(|_, _world, map, query| {
            let map = &*map;
            let mut rng = rand::thread_rng();
            for (pos, agent_type, hunger, energy) in query.iter_mut(_world) {
                // Random walk: pick a random direction and move
                let dx = rng.gen_range(-1.0..=1.0) * agent_type.move_speed;
                let dy = rng.gen_range(-1.0..=1.0) * agent_type.move_speed;
                let mut new_x = pos.x + dx;
                let mut new_y = pos.y + dy;
                // Clamp to map bounds
                new_x = new_x.max(0.0).min(map.width as f32 - 1.0);
                new_y = new_y.max(0.0).min(map.height as f32 - 1.0);
                pos.x = new_x;
                pos.y = new_y;
                // Simple hunger/energy decay for demo
                hunger.value -= 0.1;
                energy.value -= 0.05;
            }
        })
}

// --- ECS Agent Interaction System ---
pub fn agent_interaction_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentInteractionSystem")
        .with_query(<(legion::Entity, &Position)>::query())
        .build(|_cmd, _world, _, query| {
            // --- Pass 1: Collect all agent info for matching (read-only) ---
            // let all_agents: Vec<(legion::Entity, Option<legion::Entity>, u32, u32, Option<legion::Entity>)> = unsafe {
            //     query.iter_unchecked(_world)
            //         .map(|(e, _p)| (*e, None, 0, 0, None))
            //         .collect()
            // };
            // --- Pass 2: Mutate only the current entity, use info from pass 1 ---
            for (_entity, _pos) in query.iter(_world) {
                // No-op
            }
            // --- Phase 2/3: Decrement ticks, clear when done ---
            for (_entity, _pos) in query.iter(_world) {
                // No-op
            }
        })
}

// --- ECS Interaction System (agent-agent, agent-food) ---
pub fn entity_interaction_system() -> impl legion::systems::Runnable {
    
    legion::SystemBuilder::new("EntityInteractionSystem")
        .write_resource::<InteractionStats>()
        .with_query(<(legion::Entity, &Position)>::query())
        .with_query(<(legion::Entity, &Position)>::query())
        .with_query(<(legion::Entity, &Position, &Food)>::query())
        .build(|cmd, _world, stats, (_entity_query, _agent_query, _food_query)| {
            let mut interactions_this_tick = 0;
            let mut active_interactions = 0;
            // Gather all agent entities and positions
            let agents: Vec<_> = _agent_query.iter(_world).map(|(entity, pos)| (*entity, pos.x, pos.y)).collect();
            let food_entities: Vec<_> = _food_query.iter(_world).map(|(e, pos, _food)| (*e, pos.x, pos.y)).collect();
            let mut interacted = vec![false; agents.len()];
            for i in 0..agents.len() {
                let (_, x, y) = agents[i];
                if !interacted[i] {
                    // Agent-agent interaction
                    for j in (i+1)..agents.len() {
                        let (_, ox, oy) = agents[j];
                        if (x - ox).abs() < 1.5 && (y - oy).abs() < 1.5 {
                            interactions_this_tick += 1;
                            active_interactions += 1;
                            interacted[i] = true;
                            interacted[j] = true;
                            break;
                        }
                    }
                    // Agent-food interaction
                    for (food_e, fx, fy) in &food_entities {
                        if (x - *fx).abs() < 1.0 && (y - *fy).abs() < 1.0 {
                            cmd.remove(*food_e);
                        }
                    }
                }
            }
            stats.agent_interactions += interactions_this_tick;
            stats.active_interactions = active_interactions;
            // Maintain a history of active_interactions
            if stats.active_interactions_history.len() >= 100 {
                stats.active_interactions_history.pop_front();
            }
            stats.active_interactions_history.push_back(active_interactions);
        })
}

// --- ECS Food Spawning System ---
pub fn food_spawn_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("FoodSpawnSystem")
        .read_resource::<crate::map::Map>()
        .build(|cmd, _world, map, _| {
            let mut rng = rand::thread_rng();
            let map = &*map;
            let num_to_spawn = (map.width * map.height / 50000).max(1);
            for _ in 0..num_to_spawn {
                let x = rng.gen_range(0..map.width) as f32;
                let y = rng.gen_range(0..map.height) as f32;
                let ix = x as i32;
                let iy = y as i32;
                let exists = Arc::new(Mutex::new(false));
                let exists_clone = Arc::clone(&exists);
                cmd.exec_mut(move |world, _| {
                    let found = <(&Position, &Food)>::query()
                        .iter(world)
                        .any(|(pos, _)| pos.x.round() as i32 == ix && pos.y.round() as i32 == iy);
                    *exists_clone.lock().unwrap() = found;
                });
                if !*exists.lock().unwrap() {
                    cmd.push((Position { x, y }, Food, Renderable { icon: '*', color: "green" }));
                }
            }
        })
}

// --- ECS Agent Death System ---
pub fn agent_death_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDeathSystem")
        .with_query(<(legion::Entity, &Hunger, &Energy)>::query())
        .build(|cmd, world, _, query| {
            let mut to_remove = Vec::new();
            for (entity, hunger, energy) in query.iter(world) {
                if hunger.value <= 0.0 || energy.value <= 0.0 {
                    to_remove.push(*entity);
                }
            }
            for entity in to_remove {
                cmd.remove(entity);
            }
        })
}

// Example usage (in tests or migration):
// let agent = spawn_agent(&mut world, Position { x: 1.0, y: 2.0 }, AgentType { name: "worker", move_speed: 1.0, color: "blue" });
// let food = spawn_food(&mut world, Position { x: 3.0, y: 4.0 });
