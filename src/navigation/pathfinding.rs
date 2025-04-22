use crate::agent::AgentType;
use crate::agent::MovementEffect;
use crate::map::{Map, Terrain};
use crate::terrain::types::TerrainType;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

fn terrain_to_terrain_type(terrain: Terrain) -> TerrainType {
    match terrain {
        Terrain::Grass => TerrainType::Grass,
        Terrain::Forest => TerrainType::Forest,
        Terrain::Water => TerrainType::Water,
        Terrain::Mountain => TerrainType::Mountain,
    }
}

#[derive(Copy, Clone, Debug)]
struct Node {
    pub x: i32,
    pub y: i32,
    pub cost: f32,
    pub est_total: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.est_total == other.est_total
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap
        other.est_total.partial_cmp(&self.est_total).unwrap_or(Ordering::Equal)
    }
}

pub fn a_star_path(map: &Map, agent_type: &AgentType, start: (i32, i32), goal: (i32, i32), max_distance: i32) -> Option<Vec<(f32, f32)>> {
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), f32> = HashMap::new();
    let mut f_score: HashMap<(i32, i32), f32> = HashMap::new();
    let h = |x: i32, y: i32| ((x - goal.0).abs() + (y - goal.1).abs()) as f32;
    g_score.insert(start, 0.0);
    f_score.insert(start, h(start.0, start.1));
    open.push(Node { x: start.0, y: start.1, cost: 0.0, est_total: h(start.0, start.1) });
    let directions = [(-1,0), (1,0), (0,-1), (0,1)];
    while let Some(Node { x, y, cost: _, est_total: _ }) = open.pop() {
        if (x, y) == goal {
            let mut path = vec![(x as f32 + 0.5, y as f32 + 0.5)];
            let mut current = (x, y);
            while let Some(&prev) = came_from.get(&current) {
                path.push((prev.0 as f32 + 0.5, prev.1 as f32 + 0.5));
                current = prev;
            }
            path.reverse();
            return Some(path);
        }
        // Enforce max search distance
        if (x - start.0).abs() > max_distance || (y - start.1).abs() > max_distance {
            continue;
        }
        for (dx, dy) in directions.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= map.width || ny >= map.height {
                continue;
            }
            let terrain = map.tiles[ny as usize][nx as usize];
            let terrain_type = terrain_to_terrain_type(terrain);
            let effect = agent_type.movement_profile.movement_effect_for(terrain_type);
            let (passable, move_cost) = match effect {
                MovementEffect::Normal => (true, 1.0),
                MovementEffect::Slow(mult) => (true, mult),
                MovementEffect::Impassable => (false, 0.0),
            };
            if !passable {
                continue;
            }
            let tentative_g = g_score.get(&(x, y)).unwrap_or(&f32::INFINITY) + move_cost;
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
