use community_sim::map::{Map, Terrain};

#[test]
fn test_map_creation() {
    let map = Map::new(10, 15);
    assert_eq!(map.width, 10);
    assert_eq!(map.height, 15);
    assert_eq!(map.tiles.len(), 15);
    assert_eq!(map.tiles[0].len(), 10);
}

#[test]
fn test_map_is_passable() {
    let map = Map::new(5, 5);
    for y in 0..5 {
        for x in 0..5 {
            // Should not panic, all tiles are valid
            let _ = map.is_passable(x, y);
        }
    }
}

#[test]
fn test_terrain_variants() {
    let map = Map::new(8, 8);
    let mut found = [false; 4];
    for row in &map.tiles {
        for cell in row {
            match cell {
                Terrain::Grass => found[0] = true,
                Terrain::Water => found[1] = true,
                Terrain::Forest => found[2] = true,
                Terrain::Mountain => found[3] = true,
            }
        }
    }
    assert!(found.iter().any(|f| *f), "At least one terrain type should be present");
}
