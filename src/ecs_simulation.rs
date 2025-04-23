//! Shared ECS simulation tick for both headless and GUI modes, with profiling support.
use legion::*;
use crate::ecs_components::*;
use legion::Schedule;
use crate::food::{collect_food_positions_system, collect_food_spawn_positions_system, food_spawn_apply_system};
use crate::agent::{agent_movement_system, agent_death_system};

/// All unused imports removed for a clean build

#[derive(Debug, Clone, Default)]
pub struct SystemProfile {
    pub agent_movement: f64,
    pub entity_interaction: f64,
    pub agent_death: f64,
    pub food_spawn_collect: f64,
    pub food_spawn_apply: f64,
}

impl SystemProfile {
    pub fn new() -> Self {
        Self {
            agent_movement: 0.0,
            entity_interaction: 0.0,
            agent_death: 0.0,
            food_spawn_collect: 0.0,
            food_spawn_apply: 0.0,
        }
    }
    pub fn to_csv_row(&self) -> String {
        format!("{:.6},{:.6},{:.6},{:.6},{:.6}",
            self.agent_movement,
            self.entity_interaction,
            self.agent_death,
            self.food_spawn_collect,
            self.food_spawn_apply)
    }
    pub fn add(&mut self, other: &SystemProfile) {
        self.agent_movement += other.agent_movement;
        self.entity_interaction += other.entity_interaction;
        self.agent_death += other.agent_death;
        self.food_spawn_collect += other.food_spawn_collect;
        self.food_spawn_apply += other.food_spawn_apply;
    }
    pub fn div_assign(&mut self, divisor: f64) {
        self.agent_movement /= divisor;
        self.entity_interaction /= divisor;
        self.agent_death /= divisor;
        self.food_spawn_collect /= divisor;
        self.food_spawn_apply /= divisor;
    }
}

/// Builds a Legion Schedule containing all ECS systems in the correct order.
pub fn build_simulation_schedule() -> Schedule {
    Schedule::builder()
        .add_system(collect_food_positions_system())
        .add_system(agent_movement_system())
        .add_system(entity_interaction_system())
        .add_system(agent_death_system())
        .add_system(collect_food_spawn_positions_system())
        .add_system(food_spawn_apply_system())
        .build()
}

/// Builds a Legion Schedule containing all ECS systems in the correct order.
/// Legion will automatically parallelize systems that do not have conflicting data access.
pub fn build_simulation_schedule_parallel() -> Schedule {
    Schedule::builder()
        .add_system(collect_food_positions_system())
        .add_system(agent_movement_system())
        .add_system(entity_interaction_system())
        .add_system(agent_death_system())
        .add_system(collect_food_spawn_positions_system())
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

/// Advances the simulation by one tick, running all ECS systems in parallel where possible.
/// Legion automatically parallelizes systems with non-conflicting data access.
pub fn simulation_tick_parallel(world: &mut World, resources: &mut Resources, schedule: &mut Schedule) -> SystemProfile {
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
    // Overlay food and agents (entities with Position)
    let mut query = <&crate::ecs_components::Position>::query();
    for pos in query.iter(world) {
        let x = pos.x.round() as i32;
        let y = pos.y.round() as i32;
        if x >= 0 && y >= 0 && (x as usize) < map.width as usize && (y as usize) < map.height as usize {
            buffer[y as usize][x as usize] = 'A';
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
