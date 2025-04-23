// Encapsulates all mutable simulation and UI state for easier passing between functions
use legion::*;
use sdl2::ttf::Font;
use crate::graphics::camera::Camera;

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
    // All ECS systems are added to the Legion Schedule; no need to store boxed systems here.
    // Add other fields as needed
}
