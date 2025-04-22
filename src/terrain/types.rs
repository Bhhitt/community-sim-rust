use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainType {
    Water,
    Dirt,
    Grass,
    Forest,
    Mountain,
    Beach,
    // Extend as needed
}
