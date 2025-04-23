// Encapsulates all mutable simulation and UI state for easier passing between functions
use legion::*;
use sdl2::ttf::Font;
use crate::graphics::camera::Camera;
use crate::graphics::input_intent::InputQueue;

// All unused imports removed for a clean build
pub struct SimUIState<'a> {
    pub world: &'a mut World,
    pub resources: &'a mut Resources,
    pub schedule: &'a mut Schedule,
    pub camera: &'a mut Camera,
    pub font: &'a Font<'static, 'static>,
    pub cached_stats: CachedStats,
    pub last_stats_update: std::time::Instant,
    pub selected_agent: Option<legion::Entity>,
    pub empty_cell_flash: Option<(i32, i32, std::time::Instant)>,
    pub tick: i32,
    // Input intents collected from SDL2 events for ECS processing
    pub input_queue: InputQueue,
    // All ECS systems are added to the Legion Schedule; no need to store boxed systems here.
    // Add other fields as needed
}

// Struct to cache all stats for the stats window
#[derive(Clone, Default)]
pub struct CachedStats {
    pub agent_counts: Vec<(String, usize)>,
    pub food_count: usize,
    pub food_spawned_per_tick: usize,
    pub food_collected_per_tick: usize,
    pub agent_interactions: usize,
    pub active_interactions: usize,
    pub active_interactions_history: std::collections::VecDeque<usize>,
}

/// Updates the cached stats in SimUIState by querying the ECS world and resources.
pub fn update_cached_stats(world: &World, resources: &Resources, cached: &mut CachedStats) {
    // Agent counts
    use legion::IntoQuery;
    use crate::agent::AgentType;
    let mut agent_type_counts = std::collections::HashMap::<String, usize>::new();
    let mut query = <(&AgentType,)>::query();
    for (agent_type,) in query.iter(world) {
        *agent_type_counts.entry(agent_type.r#type.clone()).or_insert(0) += 1;
    }
    let mut counts_vec: Vec<_> = agent_type_counts.into_iter().collect();
    counts_vec.sort_by(|a, b| a.0.cmp(&b.0));
    cached.agent_counts = counts_vec;

    // Food count
    cached.food_count = <(&crate::ecs_components::Position, &crate::food::Food)>::query().iter(world).count();
    // Food stats
    if let Some(food_stats) = resources.get::<crate::ecs_components::FoodStats>() {
        cached.food_spawned_per_tick = food_stats.spawned_per_tick;
        cached.food_collected_per_tick = food_stats.collected_per_tick;
    } else {
        cached.food_spawned_per_tick = 0;
        cached.food_collected_per_tick = 0;
    }
    // Interaction stats
    if let Some(stats) = resources.get::<crate::ecs_components::InteractionStats>() {
        cached.agent_interactions = stats.agent_interactions;
        cached.active_interactions = stats.active_interactions;
        cached.active_interactions_history = stats.active_interactions_history.clone();
    } else {
        cached.agent_interactions = 0;
        cached.active_interactions = 0;
        cached.active_interactions_history.clear();
    }
}
