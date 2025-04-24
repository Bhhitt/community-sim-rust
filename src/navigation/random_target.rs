use crate::map::Map;
use crate::agent::AgentType;
use rand::Rng;

/// Returns a random passable target within a given radius of (cx, cy).
fn random_passable_target_within_radius<R: Rng>(
    map: &Map,
    agent_type: &AgentType,
    rng: &mut R,
    cx: f32,
    cy: f32,
    radius: f32,
) -> (f32, f32) {
    let mut tries = 0;
    let radius_sq = radius * radius;
    loop {
        // Pick a random point in a square, then check if it's within the circle
        let dx = rng.gen_range(-(radius as i32)..=(radius as i32));
        let dy = rng.gen_range(-(radius as i32)..=(radius as i32));
        let nx = (cx as i32 + dx).clamp(0, map.width - 1);
        let ny = (cy as i32 + dy).clamp(0, map.height - 1);
        let dist_sq = (nx as f32 - cx).powi(2) + (ny as f32 - cy).powi(2);
        if dist_sq > radius_sq {
            tries += 1;
            if tries > 1000 { break; }
            continue;
        }
        let terrain = map.tiles[ny as usize][nx as usize];
        let is_scout = agent_type.name == "scout";
        let passable = match terrain {
            crate::map::Terrain::Mountain => is_scout, // only scouts cross mountains
            _ => terrain.movement_cost().is_some(),
        };
        if passable {
            return (
                nx as f32 + rng.gen_range(0.0..1.0),
                ny as f32 + rng.gen_range(0.0..1.0),
            );
        }
        tries += 1;
        if tries > 1000 { break; }
    }
    // fallback: pick a global random
    let x = rng.gen_range(0..map.width) as i32;
    let y = rng.gen_range(0..map.height) as i32;
    (x as f32, y as f32)
}

pub fn random_passable_target<R: Rng>(map: &Map, agent_type: &AgentType, rng: &mut R, origin: Option<(f32, f32)>) -> (f32, f32) {
    if let Some((cx, cy)) = origin {
        // 120-unit local random
        random_passable_target_within_radius(map, agent_type, rng, cx, cy, 120.0)
    } else {
        // fallback to global random
        let mut tries = 0;
        loop {
            let x = rng.gen_range(0..map.width) as i32;
            let y = rng.gen_range(0..map.height) as i32;
            let terrain = map.tiles[y as usize][x as usize];
            let is_scout = agent_type.name == "scout";
            let passable = match terrain {
                crate::map::Terrain::Mountain => is_scout,
                _ => terrain.movement_cost().is_some(),
            };
            if passable { return (x as f32 + rng.gen_range(0.0..1.0), y as f32 + rng.gen_range(0.0..1.0)); }
            tries += 1;
            if tries > 1000 { return (x as f32, y as f32); }
        }
    }
}
