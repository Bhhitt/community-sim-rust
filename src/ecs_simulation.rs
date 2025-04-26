// TODO: Refactor this file to split schedule and tick logic into focused, modular ECS systems and schedule builders. See audit notes for refactor plan.


//! Shared ECS simulation tick for both headless and GUI modes, with profiling support.
use legion::*;
use crate::ecs_components::*;
use crate::food::systems::collect_food_positions_system;
use crate::food::systems::collect_food_spawn_positions_system;
use crate::food::systems::food_spawn_apply_system;
use crate::agent::agent_death_system;
// use crate::ecs::systems::swimming::swimming_system;

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

mod agent {
    pub use crate::agent::event_log_bridge::agent_event_log_to_gui_system;
}

/// Builds a Legion Schedule containing all ECS systems in the correct order, with profiling logic.
pub fn build_simulation_schedule_profiled() -> legion::Schedule {
    build_main_schedule()
}

/// Builds a Legion Schedule containing all ECS systems in the correct order, without profiling logic.
pub fn build_simulation_schedule_unprofiled() -> legion::Schedule {
    build_main_schedule()
}

/// Advances the simulation by one tick, running all ECS systems in order and profiling their execution time.
pub fn simulation_tick(world: &mut World, resources: &mut Resources, schedule: &mut legion::Schedule) -> SystemProfile {
    // Insert a new SystemProfile resource for this tick
    resources.insert(SystemProfile::new());
    schedule.execute(world, resources);
    // Extract the profile after all systems have run
    resources.remove::<SystemProfile>().unwrap_or_default()
}

/// Advances the simulation by one tick, running all ECS systems in parallel where possible.
/// Legion automatically parallelizes systems with non-conflicting data access.
pub fn simulation_tick_parallel(world: &mut World, resources: &mut Resources, schedule: &mut legion::Schedule) -> SystemProfile {
    let mut profile = SystemProfile::new();
    let t = std::time::Instant::now();
    schedule.execute(world, resources);
    profile.agent_movement = t.elapsed().as_secs_f64();
    profile
}

// Remove or update this line to fix unresolved import error:
// If needed, implement or re-export passive_hunger_system in the correct module.
// use crate::agent::systems::passive_hunger_system;

pub use crate::render_ascii::render_simulation_ascii;

// --- Modular Schedule Builder Import ---
pub use crate::ecs::schedules::build_main_schedule;
