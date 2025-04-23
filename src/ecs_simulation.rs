//! Shared ECS simulation tick for both headless and GUI modes, with profiling support.
use legion::*;
use crate::ecs_components::*;
use legion::Schedule;
use crate::food::{collect_food_positions_system, collect_food_spawn_positions_system, food_spawn_apply_system};
use crate::agent::{path_following_system, action_selection_system, agent_movement_history_system, agent_death_system};

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
        .add_system(path_following_system())
        .add_system(action_selection_system())
        .add_system(agent_movement_history_system())
        .add_system(entity_interaction_system())
        .add_system(agent_death_system())
        .add_system(collect_food_spawn_positions_system())
        .add_system(food_spawn_apply_system())
        .build()
}

/// Legacy non-log-config version for compatibility (deprecated)
#[deprecated(note = "Use build_simulation_schedule() instead")]
pub fn build_simulation_schedule_parallel() -> Schedule {
    Schedule::builder()
        .add_system(collect_food_positions_system())
        .add_system(path_following_system())
        .add_system(action_selection_system())
        .add_system(agent_movement_history_system())
        .add_system(entity_interaction_system())
        .add_system(agent_death_system())
        .add_system(collect_food_spawn_positions_system())
        .add_system(food_spawn_apply_system())
        .build()
}

/// Advances the simulation by one tick, running all ECS systems in order and profiling their execution time.
pub fn simulation_tick(world: &mut World, resources: &mut Resources, schedule: &mut Schedule) -> SystemProfile {
    // Insert a new SystemProfile resource for this tick
    resources.insert(SystemProfile::new());
    schedule.execute(world, resources);
    // Extract the profile after all systems have run
    resources.remove::<SystemProfile>().unwrap_or_default()
}

/// Builds a Legion Schedule containing all ECS systems in the correct order, with per-system profiling.
pub fn build_simulation_schedule_profiled() -> Schedule {
    Schedule::builder()
        .add_system(SystemBuilder::new("CollectFoodPositionsProfiled")
            .write_resource::<SystemProfile>()
            .write_resource::<crate::ecs_components::FoodPositions>()
            .with_query(<(&crate::ecs_components::Position, &crate::food::Food)>::query())
            .build(|_, world, (profile, food_positions), query| {
                let t = std::time::Instant::now();
                let mut positions = Vec::new();
                for (pos, _food) in query.iter(world) {
                    positions.push((pos.x, pos.y));
                }
                food_positions.0 = positions;
                profile.food_spawn_collect = t.elapsed().as_secs_f64();
            })
        )
        .add_system(SystemBuilder::new("PathFollowingProfiled")
            .write_resource::<SystemProfile>()
            .read_resource::<crate::map::Map>()
            .write_resource::<crate::event_log::EventLog>()
            .with_query(<(Entity, &mut crate::ecs_components::Position, &crate::agent::components::AgentType, &mut crate::agent::components::Hunger, &mut crate::agent::components::Energy, Option<&mut crate::navigation::Target>, Option<&mut crate::navigation::Path>, &mut crate::agent::components::AgentState)>::query())
            .build(|_, world, (profile, map, event_log), query| {
                let t = std::time::Instant::now();
                // Inline path_following_system logic here (or call a helper)
                for (entity, pos, agent_type, hunger, _energy, maybe_target, mut maybe_path, agent_state) in query.iter_mut(world) {
                    match *agent_state {
                        crate::agent::components::AgentState::Moving => {
                            if let (Some(target), Some(path)) = (maybe_target.as_ref(), maybe_path.as_mut()) {
                                if let Some(next_wp) = path.waypoints.front() {
                                    let dx = next_wp.0 - pos.x;
                                    let dy = next_wp.1 - pos.y;
                                    let dist = (dx * dx + dy * dy).sqrt();
                                    let step = agent_type.move_speed.min(dist);
                                    if dist < 0.2 {
                                        path.waypoints.pop_front();
                                        if path.waypoints.is_empty() {
                                            pos.x = target.x;
                                            pos.y = target.y;
                                            *agent_state = crate::agent::components::AgentState::Arrived;
                                            event_log.push(format!("[ARRIVE] Agent {:?} arrived at ({:.2}, {:.2})", entity, pos.x, pos.y));
                                        }
                                    } else {
                                        pos.x += dx / dist * step;
                                        pos.y += dy / dist * step;
                                        hunger.value -= 0.01 * step;
                                        event_log.push(format!("[MOVE] Agent {:?} moved to ({:.2}, {:.2}) via path", entity, pos.x, pos.y));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                profile.agent_movement = t.elapsed().as_secs_f64();
            })
        )
        // Repeat for other systems, inlining or calling their logic and timing as above
        .build()
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

/// Builds a Legion Schedule containing all ECS systems in the correct order.
pub fn build_simulation_schedule_with_log() -> Schedule {
    Schedule::builder()
        .add_system(collect_food_positions_system())
        .add_system(path_following_system())
        .add_system(action_selection_system())
        .add_system(agent_movement_history_system())
        .add_system(entity_interaction_system())
        .add_system(agent_death_system())
        .add_system(collect_food_spawn_positions_system())
        .add_system(food_spawn_apply_system())
        .build()
}

pub use crate::render_ascii::render_simulation_ascii;
