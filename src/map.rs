//! Map/grid logic

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
        let mut tiles = vec![vec![Terrain::Grass; width as usize]; height as usize];
        // Randomly assign terrain types
        for y in 0..height as usize {
            for x in 0..width as usize {
                let r: f32 = rng.gen();
                tiles[y][x] = if r < 0.1 {
                    Terrain::Water
                } else if r < 0.2 {
                    Terrain::Forest
                } else if r < 0.25 {
                    Terrain::Mountain
                } else {
                    Terrain::Grass
                };
            }
        }
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
