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

    pub fn is_passable(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }
        match self.tiles[y as usize][x as usize] {
            Terrain::Water | Terrain::Mountain => false,
            _ => true,
        }
    }
}
