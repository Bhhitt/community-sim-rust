//! ECS Components for community simulator (Legion)
use legion;
use legion::IntoQuery;
use legion::SystemBuilder;
use std::collections::VecDeque;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::Rng;

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
    pub name: String,
    pub move_speed: f32,
    pub move_probability: Option<f32>,
    pub color: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hunger {
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Energy {
    pub value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Renderable {
    pub icon: char,
    pub color: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Food {
    pub nutrition: f32,
}

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

pub struct EventLog {
    pub events: VecDeque<String>,
    pub max_len: usize,
}

impl EventLog {
    pub fn new(max_len: usize) -> Self {
        Self { events: VecDeque::with_capacity(max_len), max_len }
    }
    pub fn log(&mut self, msg: String) {
        if self.events.len() >= self.max_len {
            self.events.pop_front();
        }
        self.events.push_back(msg);
    }
    pub fn get(&self) -> &VecDeque<String> {
        &self.events
    }
}

// --- Entity Spawning Functions ---
pub fn spawn_agent(world: &mut legion::World, pos: Position, agent_type: AgentType) -> legion::Entity {
    let color = agent_type.color.clone();
    world.push((pos, agent_type, Hunger { value: 100.0 }, Energy { value: 100.0 }, Renderable { icon: '@', color }, InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0 }))
}

pub fn spawn_food(world: &mut legion::World, pos: Position) -> legion::Entity {
    use rand::Rng;
    let nutrition = rand::thread_rng().gen_range(5.0..=10.0);
    world.push((pos, Food { nutrition }, Renderable { icon: '*', color: "green".to_string() }))
}

// --- ECS Agent Movement System ---
use crate::map::Map;

pub fn agent_movement_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentMovementSystem")
        .with_query(<(&mut Position, &AgentType, &mut Hunger, &mut Energy)>::query())
        .read_resource::<Map>()
        .build(|_, world, map, query| {
            let map = &*map;
            query.for_each_mut(world, |(pos, agent_type, hunger, energy)| {
                let mut rng = SmallRng::from_entropy();
                // --- Movement probability logic ---
                let move_prob = agent_type.move_probability.unwrap_or(1.0);
                if rng.gen::<f32>() > move_prob {
                    // Skip movement this tick
                    return;
                }
                // Random walk: pick a random direction and move
                let dx = rng.gen_range(-1.0..=1.0) * agent_type.move_speed;
                let dy = rng.gen_range(-1.0..=1.0) * agent_type.move_speed;
                let mut new_x = pos.x + dx;
                let mut new_y = pos.y + dy;
                // Clamp to map bounds
                new_x = new_x.max(0.0).min(map.width as f32 - 1.0);
                new_y = new_y.max(0.0).min(map.height as f32 - 1.0);
                // Calculate distance traveled
                let distance = ((new_x - pos.x).powi(2) + (new_y - pos.y).powi(2)).sqrt();
                pos.x = new_x;
                pos.y = new_y;
                // Hunger decay (unchanged)
                hunger.value -= 0.1;
                // Energy decay is now proportional to distance traveled, but scaled down
                energy.value -= distance * 0.1;
            });
        })
}

// --- ECS Agent Interaction System ---
pub fn agent_interaction_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentInteractionSystem")
        .with_query(<(legion::Entity, &Position)>::query())
        .build(|_cmd, _world, _, query| {
            // --- Pass 1: Collect all agent info for matching (read-only) ---
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
        .write_resource::<EventLog>()
        .with_query(<(legion::Entity, &Position, &AgentType)>::query()) // agents
        .with_query(<(legion::Entity, &Position, &Food)>::query()) // food
        .with_query(<(legion::Entity, &Position, &mut Hunger, &mut Energy)>::query())
        .build(|cmd, _world, (stats, event_log), (agent_query, food_query, agent_stats_query)| {
            let agent_count = agent_query.iter(_world).count();
            let food_count = food_query.iter(_world).count();
            event_log.log(format!("[TICK] Agents: {}, Food: {}", agent_count, food_count));
            let mut interactions_this_tick = 0;
            let mut active_interactions = 0;
            let agents: Vec<_> = agent_query.iter(_world).map(|(entity, pos, _)| (*entity, pos.x, pos.y)).collect();
            let foods: Vec<_> = food_query.iter(_world).map(|(e, pos, food)| (*e, pos.x, pos.y, food.nutrition)).collect();
            let mut interacted = vec![false; agents.len()];
            for i in 0..agents.len() {
                let (agent_entity, x, y) = agents[i];
                if !interacted[i] {
                    // Agent-agent interaction
                    for j in (i+1)..agents.len() {
                        let (other_entity, ox, oy) = agents[j];
                        if (x - ox).abs() < 1.5 && (y - oy).abs() < 1.5 {
                            interactions_this_tick += 1;
                            active_interactions += 1;
                            interacted[i] = true;
                            interacted[j] = true;
                            event_log.log(format!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent_entity, other_entity));
                            break;
                        }
                    }
                    // Agent-food interaction
                    for (food_e, fx, fy, nutrition) in &foods {
                        if (x - *fx).abs() < 1.0 && (y - *fy).abs() < 1.0 {
                            if let Some((_entity, _pos, hunger, energy)) = agent_stats_query.iter_mut(_world).find(|(e, _pos, _h, _en)| **e == agent_entity) {
                                hunger.value += *nutrition;
                                energy.value += *nutrition;
                                event_log.log(format!("[EAT] Agent {:?} ate food {:?} (+{:.1})", agent_entity, food_e, nutrition));
                            }
                            cmd.remove(*food_e);
                        }
                    }
                }
            }
            stats.agent_interactions += interactions_this_tick;
            stats.active_interactions = active_interactions;
            if stats.active_interactions_history.len() >= 100 {
                stats.active_interactions_history.pop_front();
            }
            stats.active_interactions_history.push_back(active_interactions);
        })
}

// --- Food spawn collection as a regular function (not a system) ---
// pub fn collect_food_spawn_positions(world: &legion::World, map: &crate::map::Map) -> Vec<(f32, f32)> {
//     let mut rng = rand::thread_rng();
//     let num_to_spawn = (map.width * map.height / 20000).max(2);
//     let mut positions_to_spawn = Vec::new();
//     for _ in 0..num_to_spawn {
//         let x = rng.gen_range(0..map.width) as f32;
//         let y = rng.gen_range(0..map.height) as f32;
//         let ix = x as i32;
//         let iy = y as i32;
//         let found = <(&Position, &Food)>::query()
//             .iter(world)
//             .any(|(pos, _)| pos.x.round() as i32 == ix && pos.y.round() as i32 == iy);
//         if !found {
//             positions_to_spawn.push((x, y));
//         }
//     }
//     positions_to_spawn
// }

// --- Food spawn collection as an ECS system ---
pub fn collect_food_spawn_positions_system() -> impl legion::systems::Runnable {
    use legion::*;
    use rand::Rng;
    SystemBuilder::new("CollectFoodSpawnPositionsSystem")
        .write_resource::<crate::ecs_components::PendingFoodSpawns>()
        .read_resource::<crate::map::Map>()
        .build(|_, world, (pending_food, map), _| {
            let num_to_spawn = (map.width * map.height / 20000).max(2);
            let mut rng = rand::thread_rng();
            let mut positions_to_spawn = Vec::new();
            for _ in 0..num_to_spawn {
                let mut x;
                let mut y;
                let mut tries = 0;
                loop {
                    x = rng.gen_range(0..map.width) as f32;
                    y = rng.gen_range(0..map.height) as f32;
                    if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                        break;
                    }
                    tries += 1;
                    if tries > 1000 {
                        break;
                    }
                }
                positions_to_spawn.push((x, y));
            }
            pending_food.0 = positions_to_spawn;
        })
}

// --- Resource for pending food spawn positions ---
pub struct PendingFoodSpawns(pub Vec<(f32, f32)>);

// --- ECS Food Spawn Apply System (mutation only) ---
pub fn food_spawn_apply_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("FoodSpawnApplySystem")
        .write_resource::<PendingFoodSpawns>()
        .build(|cmd, _world, pending, _| {
            for (x, y) in pending.0.drain(..) {
                cmd.push((
                    Position { x, y },
                    Food { nutrition: rand::thread_rng().gen_range(5.0..=10.0) },
                    Renderable { icon: '*', color: "green".to_string() }
                ));
            }
        })
}

// --- ECS Agent Death System ---
pub fn agent_death_system() -> impl legion::systems::Runnable {
    legion::SystemBuilder::new("AgentDeathSystem")
        .with_query(<(legion::Entity, &Hunger, &Energy)>::query())
        .build(|cmd, _world, _, query| {
            let mut to_remove = Vec::new();
            for (entity, hunger, energy) in query.iter(_world) {
                if hunger.value <= 0.0 || energy.value <= 0.0 {
                    to_remove.push(*entity);
                }
            }
            for entity in to_remove {
                cmd.remove(entity);
            }
        })
}
