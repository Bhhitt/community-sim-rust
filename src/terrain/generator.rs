use crate::terrain::types::TerrainType;
use noise::{NoiseFn, Perlin};

pub fn generate_terrain(
    width: usize,
    height: usize,
    seed: u32,
    scale: f64,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
) -> Vec<Vec<TerrainType>> {
    let perlin = Perlin::new(seed);
    let mut map = vec![vec![TerrainType::Grass; width]; height];

    // Parameters for biome thresholds
    let beach_margin = 0.08;
    let mountain_thresh = 0.7;
    let forest_thresh = 0.4;
    let dirt_thresh = 0.25;
    let water_thresh = 0.28;

    for y in 0..height {
        for x in 0..width {
            let mut amplitude = 1.0;
            let mut frequency = 1.0;
            let mut noise_height = 0.0;
            let mut max_amplitude = 0.0;
            for _ in 0..octaves {
                let nx = x as f64 * scale * frequency;
                let ny = y as f64 * scale * frequency;
                let val = perlin.get([nx, ny]); // -1.0..1.0
                noise_height += val * amplitude;
                max_amplitude += amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }
            let norm_val = (noise_height / max_amplitude + 1.0) / 2.0; // 0.0..1.0
            map[y][x] = if norm_val < water_thresh {
                TerrainType::Water
            } else if norm_val < water_thresh + beach_margin {
                TerrainType::Beach
            } else if norm_val < dirt_thresh {
                TerrainType::Dirt
            } else if norm_val < forest_thresh {
                TerrainType::Grass
            } else if norm_val < mountain_thresh {
                TerrainType::Forest
            } else {
                TerrainType::Mountain
            };
        }
    }
    map
}
