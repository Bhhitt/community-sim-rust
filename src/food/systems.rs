use legion::*;
use crate::food::components::*;
use crate::ecs_components::{Position};
use rand::Rng;

pub fn collect_food_positions_system() -> impl systems::Runnable {
    SystemBuilder::new("CollectFoodPositionsSystem")
        .write_resource::<crate::ecs_components::FoodPositions>()
        .with_query(<(&crate::ecs_components::Position, &crate::food::Food)>::query())
        .build(|_, world, food_positions, query| {
            let mut positions = Vec::new();
            for (pos, _food) in query.iter(world) {
                positions.push((pos.x, pos.y));
            }
            food_positions.0 = positions;
        })
}

pub fn collect_food_spawn_positions_system() -> impl systems::Runnable {
    SystemBuilder::new("CollectFoodSpawnPositionsSystem")
        .write_resource::<crate::food::PendingFoodSpawns>()
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
            pending_food.0 = positions_to_spawn.into();
        })
}

pub fn food_spawn_apply_system() -> impl systems::Runnable {
    SystemBuilder::new("FoodSpawnApplySystem")
        .write_resource::<PendingFoodSpawns>()
        .build(|cmd, _world, pending, _| {
            for (x, y) in pending.0.drain(..) {
                cmd.push((
                    Position { x, y },
                    Food { nutrition: rand::thread_rng().gen_range(5.0..=10.0) }
                ));
            }
        })
}
