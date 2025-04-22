//! ECS Components for community simulator (Legion)
use legion;
use legion::IntoQuery;
use legion::SystemBuilder;
use std::collections::VecDeque;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::Rng;
use std::ops::DerefMut;
use std::sync::Mutex;
use log;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Target {
    pub x: f32,
    pub y: f32,
    pub stuck_ticks: u32, // Track how many ticks agent is stuck
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub waypoints: VecDeque<(f32, f32)>,
}

#[derive(Default)]
pub struct InteractionStats {
    pub agent_interactions: usize,
    pub active_interactions: usize,
    pub active_interactions_history: VecDeque<usize>,
}

pub struct EventLog {
    pub events: Mutex<VecDeque<String>>,
    pub max_len: usize,
}

impl EventLog {
    pub fn new(max_len: usize) -> Self {
        Self { events: Mutex::new(VecDeque::with_capacity(max_len)), max_len }
    }
    pub fn log(&self, msg: String) {
        let mut events = self.events.lock().unwrap();
        if events.len() >= self.max_len {
            events.pop_front();
        }
        events.push_back(msg);
    }
    pub fn get(&self) -> Vec<String> {
        // Return a cloned Vec for thread safety
        let events = self.events.lock().unwrap();
        events.iter().cloned().collect()
    }
}

// Helper: Pick a random passable tile for this agent type
fn random_passable_target<R: Rng>(map: &crate::map::Map, agent_type: &AgentType, rng: &mut R) -> (f32, f32) {
    let mut tries = 0;
    loop {
        let x = rng.gen_range(0..map.width) as i32;
        let y = rng.gen_range(0..map.height) as i32;
        let terrain = map.tiles[y as usize][x as usize];
        let is_scout = agent_type.name == "scout";
        let passable = match terrain {
            crate::map::Terrain::Mountain => is_scout, // only scouts cross mountains
            _ => terrain.movement_cost().is_some(),
        };
        if passable { return (x as f32 + rng.gen_range(0.0..1.0), y as f32 + rng.gen_range(0.0..1.0)); }
        tries += 1;
        if tries > 1000 { return (x as f32, y as f32); } // fallback
    }
}

// --- Entity Spawning Functions ---
pub fn spawn_agent(world: &mut legion::World, pos: Position, agent_type: AgentType, map: &crate::map::Map) -> legion::Entity {
    let color = agent_type.color.clone();
    let mut rng = rand::thread_rng();
    let (tx, ty) = random_passable_target(map, &agent_type, &mut rng);
    world.push((pos, agent_type, Hunger { value: 100.0 }, Energy { value: 100.0 }, Renderable { icon: '@', color }, InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0 }, Target { x: tx, y: ty, stuck_ticks: 0 }, Path { waypoints: VecDeque::new() }))
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
        .with_query(<(&mut Position, &AgentType, &mut Hunger, &mut Energy, Option<&mut Target>, Option<&mut Path>)>::query())
        .read_resource::<Map>()
        .write_resource::<crate::ecs_components::EventLog>()
        .build(|_, world, (map, event_log), query| {
            let map = &*map;
            let event_log = event_log;
            query.par_for_each_mut(
                world,
                |(
                    pos,
                    agent_type,
                    hunger,
                    energy,
                    mut target,
                    mut path
                ): (
                    &mut Position,
                    &AgentType,
                    &mut Hunger,
                    &mut Energy,
                    Option<&mut Target>,
                    Option<&mut Path>
                )| {
                // --- Movement probability logic ---
                // let move_prob = agent_type.move_probability.unwrap_or(1.0);
                // if rng.gen::<f32>() > move_prob {
                //     return;
                // }
                // --- Pathfinding logic ---
                let _map_w = map.width as f32;
                let _map_h = map.height as f32;
                let (target_x, target_y, _stuck_ticks) = if let Some(ref mut target) = target {
                    let mut stuck_ticks = target.stuck_ticks;
                    let progress = ((pos.x - target.x).abs() + (pos.y - target.y).abs()) > 0.1;
                    // Only recalc path if target changed or path is empty (not every time stuck)
                    let target_changed = (target.x - pos.x).abs() > 0.1 || (target.y - pos.y).abs() > 0.1;
                    if !progress {
                        stuck_ticks += 1;
                    } else {
                        stuck_ticks = 0;
                    }
                    if (pos.x - target.x).abs() < 0.1 && (pos.y - target.y).abs() < 0.1 || stuck_ticks > 10 {
                        // Pick a new target within 10 squares
                        let mut tx = pos.x;
                        let mut ty = pos.y;
                        let mut tries = 0;
                        let mut found = false;
                        while tries < 20 {
                            let dx = 0;
                            let dy = 0;
                            let candidate_tx = (pos.x.round() as i32 + dx).clamp(0, map.width-1) as f32;
                            let candidate_ty = (pos.y.round() as i32 + dy).clamp(0, map.height-1) as f32;
                            if map.is_passable(candidate_tx as i32, candidate_ty as i32) {
                                tx = candidate_tx;
                                ty = candidate_ty;
                                found = true;
                                break;
                            }
                            tries += 1;
                        }
                        if !found {
                            // Fallback: try a global random passable tile
                            let mut global_tries = 0;
                            while global_tries < 100 {
                                let candidate_gtx = pos.x;
                                let candidate_gty = pos.y;
                                if map.is_passable(candidate_gtx as i32, candidate_gty as i32) {
                                    tx = candidate_gtx;
                                    ty = candidate_gty;
                                    found = true;
                                    break;
                                }
                                global_tries += 1;
                            }
                            if !found {
                                event_log.log(format!("[STUCK] Agent at ({:.1}, {:.1}) could not find a passable target after {} tries (local+global)", pos.x, pos.y, tries + global_tries));
                                log::info!("[STUCK] Agent at ({:.1}, {:.1}) could not find a passable target after {} tries (local+global)", pos.x, pos.y, tries + global_tries);
                                target.stuck_ticks = stuck_ticks;
                                if let Some(ref mut path) = path {
                                    path.waypoints.clear();
                                }
                                return;
                            }
                        }
                        target.x = tx;
                        target.y = ty;
                        target.stuck_ticks = 0;
                        if let Some(ref mut path) = path {
                            if let Some(new_path) = a_star_path(map, agent_type, (pos.x.round() as i32, pos.y.round() as i32), (tx.round() as i32, ty.round() as i32), 10) {
                                path.waypoints = new_path.into();
                            } else {
                                path.waypoints.clear();
                                event_log.log(format!("[STUCK] Agent at ({:.1}, {:.1}) could not find a path to new target ({:.1}, {:.1})", pos.x, pos.y, tx, ty));
                                log::info!("[STUCK] Agent at ({:.1}, {:.1}) could not find a path to new target ({:.1}, {:.1})", pos.x, pos.y, tx, ty);
                            }
                        }
                        (tx, ty, 0)
                    } else {
                        target.stuck_ticks = stuck_ticks;
                        // Only recalc path if empty or target changed
                        if let Some(ref mut path) = path {
                            if path.waypoints.is_empty() || target_changed {
                                if let Some(new_path) = a_star_path(map, agent_type, (pos.x.round() as i32, pos.y.round() as i32), (target.x.round() as i32, target.y.round() as i32), 10) {
                                    path.waypoints = new_path.into();
                                } else {
                                    path.waypoints.clear();
                                    event_log.log(format!("[STUCK] Agent at ({:.1}, {:.1}) could not find a path to target ({:.1}, {:.1})", pos.x, pos.y, target.x, target.y));
                                    log::info!("[STUCK] Agent at ({:.1}, {:.1}) could not find a path to target ({:.1}, {:.1})", pos.x, pos.y, target.x, target.y);
                                }
                            }
                        }
                        (target.x, target.y, stuck_ticks)
                    }
                } else {
                    // No target component, skip
                    (pos.x, pos.y, 0)
                };
                // --- Measure how close agents get to their destinations ---
                if let Some(ref target) = target {
                    let dist_to_target = ((pos.x - target.x).powi(2) + (pos.y - target.y).powi(2)).sqrt();
                    if dist_to_target < 1.5 && dist_to_target > 0.15 {
                        event_log.log(format!("[NEAR_MISS] Agent at ({:.2}, {:.2}) is {:.2} units from target ({:.2}, {:.2}) and did not reach destination", pos.x, pos.y, dist_to_target, target.x, target.y));
                        log::info!("[NEAR_MISS] Agent at ({:.2}, {:.2}) is {:.2} units from target ({:.2}, {:.2}) and did not reach destination", pos.x, pos.y, dist_to_target, target.x, target.y);
                    }
                }
                // --- Measure agents that are stuck or oscillating far from their target ---
                if let Some(ref target) = target {
                    let dist_to_target = ((pos.x - target.x).powi(2) + (pos.y - target.y).powi(2)).sqrt();
                    if dist_to_target > 2.0 && dist_to_target < 15.0 {
                        // Check for oscillation: agent is moving but not getting closer
                        static mut LAST_POS: Option<(f32, f32)> = None;
                        static mut LAST_DIST: Option<f32> = None;
                        let mut oscillating = false;
                        unsafe {
                            if let Some((last_x, last_y)) = LAST_POS {
                                let last_dist = LAST_DIST.unwrap_or(999.0);
                                let moved = ((pos.x - last_x).powi(2) + (pos.y - last_y).powi(2)).sqrt() > 0.1;
                                let not_closer = dist_to_target >= last_dist - 0.01;
                                if moved && not_closer {
                                    oscillating = true;
                                }
                            }
                            LAST_POS = Some((pos.x, pos.y));
                            LAST_DIST = Some(dist_to_target);
                        }
                        if oscillating {
                            event_log.log(format!("[OSCILLATE] Agent at ({:.2}, {:.2}) is {:.2} units from target ({:.2}, {:.2}) and not making progress", pos.x, pos.y, dist_to_target, target.x, target.y));
                            log::info!("[OSCILLATE] Agent at ({:.2}, {:.2}) is {:.2} units from target ({:.2}, {:.2}) and not making progress", pos.x, pos.y, dist_to_target, target.x, target.y);
                        }
                    }
                }
                // --- Path following with a small stray/noise ---
                let mut next_x = target_x;
                let mut next_y = target_y;
                if let Some(ref mut path) = path {
                    if let Some(&(wx, wy)) = path.waypoints.front() {
                        let dist = ((pos.x - wx).powi(2) + (pos.y - wy).powi(2)).sqrt();
                        if dist < 0.5 {
                            path.waypoints.pop_front();
                        }
                        if let Some(&(wx, wy)) = path.waypoints.front() {
                            next_x = wx;
                            next_y = wy;
                        }
                    }
                }
                let mut rng = rand::thread_rng();
                let stray_angle: f32 = rng.gen_range(-0.08..0.08); // Tiny stray (radians)
                let dx = next_x - pos.x;
                let dy = next_y - pos.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < 0.01 {
                    // Already at target
                    return;
                }
                let step = agent_type.move_speed.min(dist);
                let mut dir_x = dx / dist;
                let mut dir_y = dy / dist;
                // Apply stray (rotate direction by stray_angle)
                let cos_a = stray_angle.cos();
                let sin_a = stray_angle.sin();
                let stray_x = dir_x * cos_a - dir_y * sin_a;
                let stray_y = dir_x * sin_a + dir_y * cos_a;
                dir_x = 0.95 * dir_x + 0.05 * stray_x;
                dir_y = 0.95 * dir_y + 0.05 * stray_y;
                let new_x = (pos.x + dir_x * step).max(0.0).min(map.width as f32 - 1.0);
                let new_y = (pos.y + dir_y * step).max(0.0).min(map.height as f32 - 1.0);
                let tx = new_x.round() as i32;
                let ty = new_y.round() as i32;
                if tx < 0 || ty < 0 || tx >= map.width || ty >= map.height {
                    return;
                }
                let terrain = map.tiles[ty as usize][tx as usize];
                let is_scout = agent_type.name == "scout";
                let mountain_cost = if is_scout { Some(3.0) } else { None };
                let cost = match terrain {
                    crate::map::Terrain::Mountain => mountain_cost,
                    _ => terrain.movement_cost(),
                };
                if let Some(cost) = cost {
                    let distance = ((new_x - pos.x).powi(2) + (new_y - pos.y).powi(2)).sqrt();
                    pos.x = new_x;
                    pos.y = new_y;
                    hunger.value -= 0.1 * cost;
                    energy.value -= distance * 0.1 * cost;
                }
                // else: impassable, do not move

                // --- Enhanced stuck logic: try to unstick if stuck for a while ---
                if let Some(ref mut target) = target {
                    if target.stuck_ticks > 10 {
                        // Try to pick a new random nearby target to unstick
                        let mut tries = 0;
                        let mut found = false;
                        let mut tx = pos.x;
                        let mut ty = pos.y;
                        while tries < 20 {
                            let dx = rng.gen_range(-5..=5);
                            let dy = rng.gen_range(-5..=5);
                            let candidate_tx = (pos.x.round() as i32 + dx).clamp(0, map.width-1) as f32;
                            let candidate_ty = (pos.y.round() as i32 + dy).clamp(0, map.height-1) as f32;
                            if map.is_passable(candidate_tx as i32, candidate_ty as i32) {
                                tx = candidate_tx;
                                ty = candidate_ty;
                                found = true;
                                break;
                            }
                            tries += 1;
                        }
                        if found {
                            event_log.log(format!("[UNSTUCK] Agent at ({:.1}, {:.1}) picked new target ({:.1}, {:.1}) after being stuck {} ticks", pos.x, pos.y, tx, ty, target.stuck_ticks));
                            log::info!("[UNSTUCK] Agent at ({:.1}, {:.1}) picked new target ({:.1}, {:.1}) after being stuck {} ticks", pos.x, pos.y, tx, ty, target.stuck_ticks);
                            target.x = tx;
                            target.y = ty;
                            target.stuck_ticks = 0;
                            if let Some(ref mut path) = path {
                                if let Some(new_path) = a_star_path(map, agent_type, (pos.x.round() as i32, pos.y.round() as i32), (tx.round() as i32, ty.round() as i32), 10) {
                                    path.waypoints = new_path.into();
                                } else {
                                    path.waypoints.clear();
                                    event_log.log(format!("[STUCK] Agent at ({:.1}, {:.1}) could not find a path to new unstuck target ({:.1}, {:.1})", pos.x, pos.y, tx, ty));
                                    log::info!("[STUCK] Agent at ({:.1}, {:.1}) could not find a path to new unstuck target ({:.1}, {:.1})", pos.x, pos.y, tx, ty);
                                }
                            }
                        }
                    }
                }
            });
        })
}

// --- A* Pathfinding Helper ---
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
struct Node {
    x: i32,
    y: i32,
    cost: f32,
    est_total: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.est_total == other.est_total
    }
}
impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap
        other.est_total.partial_cmp(&self.est_total).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star_path(map: &crate::map::Map, agent_type: &AgentType, start: (i32, i32), goal: (i32, i32), max_distance: i32) -> Option<Vec<(f32, f32)>> {
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), f32> = HashMap::new();
    let mut f_score: HashMap<(i32, i32), f32> = HashMap::new();
    let is_scout = agent_type.name == "scout";
    let h = |x: i32, y: i32| ((x - goal.0).abs() + (y - goal.1).abs()) as f32;
    g_score.insert(start, 0.0);
    f_score.insert(start, h(start.0, start.1));
    open.push(Node { x: start.0, y: start.1, cost: 0.0, est_total: h(start.0, start.1) });
    let neighbors = [(-1,0),(1,0),(0,-1),(0,1)];
    while let Some(Node { x, y, cost: _, .. }) = open.pop() {
        if (x, y) == goal {
            // reconstruct path
            let mut path = vec![(x as f32 + 0.5, y as f32 + 0.5)];
            let mut curr = (x, y);
            while let Some(&prev) = came_from.get(&curr) {
                path.push((prev.0 as f32 + 0.5, prev.1 as f32 + 0.5));
                curr = prev;
            }
            path.reverse();
            return Some(path);
        }
        // Enforce max search distance
        if (x - start.0).abs() > max_distance || (y - start.1).abs() > max_distance {
            continue;
        }
        for (dx, dy) in neighbors.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= map.width || ny >= map.height { continue; }
            let terrain = map.tiles[ny as usize][nx as usize];
            let passable = match terrain {
                crate::map::Terrain::Mountain => is_scout,
                _ => terrain.movement_cost().is_some(),
            };
            if !passable { continue; }
            let tentative_g = g_score.get(&(x, y)).unwrap_or(&f32::INFINITY) + 1.0;
            if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&f32::INFINITY) {
                came_from.insert((nx, ny), (x, y));
                g_score.insert((nx, ny), tentative_g);
                let f = tentative_g + h(nx, ny);
                f_score.insert((nx, ny), f);
                open.push(Node { x: nx, y: ny, cost: tentative_g, est_total: f });
            }
        }
    }
    None
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
    SystemBuilder::new("EntityInteractionSystem")
        .write_resource::<InteractionStats>()
        .write_resource::<EventLog>()
        .with_query(<(legion::Entity, &Position, &AgentType)>::query()) // agents
        .with_query(<(legion::Entity, &Position, &Food)>::query()) // food
        .with_query(<(legion::Entity, &mut Position, &mut Hunger, &mut Energy)>::query())
        .build(|cmd, world, (stats, event_log), (agent_query, food_query, agent_stats_query)| {
            let event_log = event_log;
            let agent_count = agent_query.iter(world).count();
            let food_count = food_query.iter(world).count();
            event_log.log(format!("[TICK] Agents: {}, Food: {}", agent_count, food_count));
            let mut interactions_this_tick = 0;
            let mut active_interactions = 0;
            let agents: Vec<_> = agent_query.iter(world).map(|(entity, pos, _)| (*entity, pos.x, pos.y)).collect();
            let foods: Vec<_> = food_query.iter(world).map(|(e, pos, food)| (*e, pos.x, pos.y, food.nutrition)).collect();
            let mut interacted = vec![false; agents.len()];
            let mut rng = rand::thread_rng();
            // Collect interaction events first
            let mut food_eaten: Vec<(legion::Entity, legion::Entity, f32)> = Vec::new();
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
                            log::info!("[INTERACT] Agent {:?} interacted with Agent {:?}", agent_entity, other_entity);
                            break;
                        }
                    }
                    // Agent-food interaction (randomize food selection if multiple in range)
                    let mut foods_in_range: Vec<_> = foods.iter()
                        .filter(|(_food_e, fx, fy, _nutrition)| (x - *fx).abs() < 1.0 && (y - *fy).abs() < 1.0)
                        .collect();
                    if !foods_in_range.is_empty() {
                        use rand::seq::SliceRandom;
                        foods_in_range.shuffle(&mut rng);
                        let (food_e, _fx, _fy, nutrition) = *foods_in_range[0];
                        food_eaten.push((agent_entity, food_e, nutrition));
                    }
                }
            }
            // Second pass: apply food eaten mutations
            for (agent_entity, food_e, nutrition) in food_eaten {
                if let Some((_entity, _pos, hunger, energy)) = agent_stats_query.iter_mut(world).find(|(e, _pos, _h, _en)| **e == agent_entity) {
                    hunger.value += nutrition;
                    energy.value += nutrition;
                    event_log.log(format!("[EAT] Agent {:?} ate food {:?} (+{:.1})", agent_entity, food_e, nutrition));
                    log::info!("[EAT] Agent {:?} ate food {:?} (+{:.1})", agent_entity, food_e, nutrition);
                }
                cmd.remove(food_e);
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
        .build(|_, _world, (pending_food, map), _| {
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
                    to_remove.push(entity);
                }
            }
            for entity in to_remove {
                cmd.remove(*entity);
            }
        })
}
