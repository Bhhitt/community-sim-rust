//! Map/grid logic

use crate::terrain::{generator, types::TerrainType};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terrain {
    Grass, // normal
    Forest,
    Water,
    Mountain,
}

impl Terrain {
    pub fn to_char(&self) -> char {
        match self {
            Terrain::Grass => '.',
            Terrain::Forest => 'T',
            Terrain::Water => '~',
            Terrain::Mountain => '^',
        }
    }

    pub fn movement_cost(&self) -> Option<f32> {
        match self {
            Terrain::Grass => Some(1.0),
            Terrain::Forest => Some(2.0),
            Terrain::Mountain => None, // impassable
            Terrain::Water => None,    // impassable (could be Some(3.0) for special agents)
        }
    }
}

impl From<TerrainType> for Terrain {
    fn from(tt: TerrainType) -> Self {
        match tt {
            TerrainType::Water => Terrain::Water,
            TerrainType::Dirt => Terrain::Grass, // No Dirt in original, map to Grass for now
            TerrainType::Grass => Terrain::Grass,
            TerrainType::Forest => Terrain::Forest,
            TerrainType::Mountain => Terrain::Mountain,
            TerrainType::Beach => Terrain::Grass, // No Beach in original, map to Grass for now
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Food {
    None,
    Present,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Terrain>>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen();
        let terrain_grid = generator::generate_terrain(
            width as usize,
            height as usize,
            seed,
            0.015,   // scale
            4,       // octaves
            0.5,     // persistence
            2.0      // lacunarity
        );
        let tiles = terrain_grid
            .into_iter()
            .map(|row| row.into_iter().map(Terrain::from).collect())
            .collect();
        Self { width, height, tiles }
    }

    /// Render the map as ASCII (no agents)
    pub fn render_ascii(&self) -> String {
        let mut ascii = String::new();
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                ascii.push(self.tiles[y][x].to_char());
            }
            ascii.push('\n');
        }
        ascii
    }

    /// Checks if a tile at (x, y) is passable for an agent, taking into account the agent's state.
    ///
    /// - Water tiles are only passable if the agent is in the `Swimming` state.
    /// - Mountain tiles are never passable.
    /// - All other tiles are passable for any agent state.
    ///
    /// # Arguments
    /// * `x` - X coordinate of the tile.
    /// * `y` - Y coordinate of the tile.
    /// * `agent_state` - Optional reference to the agent's state.
    ///
    /// # Returns
    /// * `true` if the tile is passable for the agent, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use community_sim::map::{Map, Terrain};
    /// use community_sim::agent::components::AgentState;
    /// let map = Map { width: 1, height: 1, tiles: vec![vec![Terrain::Water]] };
    /// assert_eq!(map.is_passable(0, 0, Some(&AgentState::Swimming)), true);
    /// assert_eq!(map.is_passable(0, 0, Some(&AgentState::Idle)), false);
    /// ```
    pub fn is_passable(&self, x: i32, y: i32, agent_state: Option<&crate::agent::components::AgentState>) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }
        match self.tiles[y as usize][x as usize] {
            Terrain::Water => {
                if let Some(crate::agent::components::AgentState::Swimming) = agent_state {
                    true // Water is passable if agent is swimming
                } else {
                    false // Otherwise, water is impassable
                }
            },
            Terrain::Mountain => false,
            _ => true,
        }
    }

    /// Find the nearest water tile to the given (x, y) position using BFS.
    /// Returns Some((wx, wy, distance)) if a water tile is found, else None.
    pub fn find_nearest_water(&self, x: i32, y: i32) -> Option<(i32, i32, i32)> {
        use std::collections::{VecDeque, HashSet};
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((x, y, 0));
        visited.insert((x, y));
        let directions = [(-1,0),(1,0),(0,-1),(0,1)];
        while let Some((cx, cy, dist)) = queue.pop_front() {
            if cx >= 0 && cy >= 0 && cx < self.width && cy < self.height {
                if let Terrain::Water = self.tiles[cy as usize][cx as usize] {
                    return Some((cx, cy, dist));
                }
                for (dx, dy) in &directions {
                    let nx = cx + dx;
                    let ny = cy + dy;
                    if nx >= 0 && ny >= 0 && nx < self.width && ny < self.height {
                        if !visited.contains(&(nx, ny)) {
                            queue.push_back((nx, ny, dist + 1));
                            visited.insert((nx, ny));
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::components::AgentState;

    #[test]
    fn test_water_passable_only_when_swimming() {
        let map = Map {
            width: 3,
            height: 3,
            tiles: vec![
                vec![Terrain::Grass, Terrain::Water, Terrain::Grass],
                vec![Terrain::Grass, Terrain::Water, Terrain::Grass],
                vec![Terrain::Grass, Terrain::Water, Terrain::Grass],
            ],
        };
        // Water tile at (1, 1)
        // Passable for Swimming
        assert_eq!(map.is_passable(1, 1, Some(&AgentState::Swimming)), true);
        // Not passable for Idle
        assert_eq!(map.is_passable(1, 1, Some(&AgentState::Idle)), false);
        // Not passable for Moving
        assert_eq!(map.is_passable(1, 1, Some(&AgentState::Moving)), false);
        // Not passable for Arrived
        assert_eq!(map.is_passable(1, 1, Some(&AgentState::Arrived)), false);
        // Not passable for None
        assert_eq!(map.is_passable(1, 1, None), false);
        // Grass always passable
        assert_eq!(map.is_passable(0, 0, Some(&AgentState::Idle)), true);
        assert_eq!(map.is_passable(0, 0, Some(&AgentState::Swimming)), true);
        // Mountain never passable
        let map2 = Map {
            width: 1,
            height: 1,
            tiles: vec![vec![Terrain::Mountain]],
        };
        assert_eq!(map2.is_passable(0, 0, Some(&AgentState::Swimming)), false);
    }
}
