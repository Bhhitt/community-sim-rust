use crate::map::Map;
use crate::agent::components::AgentType;
use rand::Rng;

pub fn random_passable_target<R: Rng>(map: &Map, agent_type: &AgentType, rng: &mut R) -> (f32, f32) {
    let mut tries = 0;
    loop {
        let x = rng.gen_range(0..map.width) as i32;
        let y = rng.gen_range(0..map.height) as i32;
        let terrain = map.tiles[y as usize][x as usize];
        let is_scout = agent_type.name.as_deref() == Some("scout");
        let passable = match terrain {
            crate::map::Terrain::Mountain => is_scout, // only scouts cross mountains
            _ => terrain.movement_cost().is_some(),
        };
        if passable { return (x as f32 + rng.gen_range(0.0..1.0), y as f32 + rng.gen_range(0.0..1.0)); }
        tries += 1;
        if tries > 1000 { return (x as f32, y as f32); } // fallback
    }
}
