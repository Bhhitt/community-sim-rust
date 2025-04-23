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
    pub cached_agent_counts: Vec<(String, usize)>,
    pub last_stats_update: std::time::Instant,
    pub selected_agent: Option<legion::Entity>,
    pub empty_cell_flash: Option<(i32, i32, std::time::Instant)>,
    pub tick: i32,
    // Input intents collected from SDL2 events for ECS processing
    pub input_queue: InputQueue,
    // All ECS systems are added to the Legion Schedule; no need to store boxed systems here.
    // Add other fields as needed
}

/// Updates the cached agent counts in SimUIState by querying the ECS world.
pub fn update_cached_agent_counts(world: &World, cached_agent_counts: &mut Vec<(String, usize)>) {
    use legion::IntoQuery;
    use crate::agent::AgentType;
    let mut agent_type_counts = std::collections::HashMap::<String, usize>::new();
    let mut query = <(&AgentType,)>::query();
    for (agent_type,) in query.iter(world) {
        *agent_type_counts.entry(agent_type.r#type.clone()).or_insert(0) += 1;
    }
    // Overwrite the vector with new counts, sorted by name for stable display
    let mut counts_vec: Vec<_> = agent_type_counts.into_iter().collect();
    counts_vec.sort_by(|a, b| a.0.cmp(&b.0));
    cached_agent_counts.clear();
    cached_agent_counts.extend(counts_vec);
}
