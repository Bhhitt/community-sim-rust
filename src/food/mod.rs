// Public API for the food system: only export what is needed outside
pub use self::components::{Food, PendingFoodSpawns};
pub use self::systems::{collect_food_positions_system};
// Removed collect_food_spawn_positions_system and food_spawn_apply_system from pub use, as they are commented out or missing

pub mod components;
pub mod systems;
