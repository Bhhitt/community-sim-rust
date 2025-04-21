//! Shared ECS simulation tick for both headless and GUI modes, with profiling support.
use legion::*;
use crate::ecs_components::*;
use std::time::Instant;
use legion::systems::Runnable;
use crate::ecs_components::collect_food_spawn_positions_system;
use legion::Schedule;

pub struct SystemProfile {
    pub agent_movement: f64,
    pub entity_interaction: f64,
    pub agent_death: f64,
    pub food_spawn_apply: f64,
}

impl SystemProfile {
    pub fn new() -> Self {
        Self {
            agent_movement: 0.0,
            entity_interaction: 0.0,
            agent_death: 0.0,
            food_spawn_apply: 0.0,
        }
    }
}

fn profile_system<F: FnMut(&mut World, &mut Resources)>(mut system: F, world: &mut World, resources: &mut Resources) -> f64 {
    let t = std::time::Instant::now();
    system(world, resources);
    t.elapsed().as_secs_f64()
}

/// Builds a Legion Schedule containing all ECS systems in the correct order.
pub fn build_simulation_schedule(map: crate::map::Map) -> Schedule {
    Schedule::builder()
        .add_system(agent_movement_system())
        .add_system(entity_interaction_system())
        .add_system(agent_death_system())
        .add_system(collect_food_spawn_positions_system(map.clone()))
        .add_system(food_spawn_apply_system())
        .build()
}

/// Advances the simulation by one tick, running all ECS systems in order and profiling their execution time.
pub fn simulation_tick(world: &mut World, resources: &mut Resources, schedule: &mut Schedule) -> SystemProfile {
    let mut profile = SystemProfile::new();
    let t = std::time::Instant::now();
    schedule.execute(world, resources);
    profile.agent_movement = t.elapsed().as_secs_f64();
    profile
}

/// Renders the simulation state (terrain, agents, food) as ASCII.
pub fn render_simulation_ascii(world: &legion::World, map: &crate::map::Map) -> String {
    // Build a 2D buffer of chars
    let mut buffer = vec![vec![' '; map.width as usize]; map.height as usize];
    // Fill with terrain
    for y in 0..map.height as usize {
        for x in 0..map.width as usize {
            buffer[y][x] = map.tiles[y][x].to_char();
        }
    }
    // Overlay food and agents (entities with Position + Renderable)
    let mut query = <(&crate::ecs_components::Position, &crate::ecs_components::Renderable)>::query();
    for (pos, renderable) in query.iter(world) {
        let x = pos.x.round() as i32;
        let y = pos.y.round() as i32;
        if x >= 0 && y >= 0 && (x as usize) < map.width as usize && (y as usize) < map.height as usize {
            buffer[y as usize][x as usize] = renderable.icon;
        }
    }
    // Convert buffer to String
    let mut ascii = String::new();
    for row in buffer {
        for ch in row {
            ascii.push(ch);
        }
        ascii.push('\n');
    }
    ascii
}
