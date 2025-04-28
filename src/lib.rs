// lib.rs for integration tests and re-exports
// Only re-export public APIs from each module tree

pub mod agent; // agent/mod.rs handles its own pub uses
pub mod ecs_components;
pub mod map;
pub mod food; // food/mod.rs handles its own pub uses
pub mod navigation; // navigation/mod.rs handles its own pub uses
pub mod terrain; // terrain/mod.rs handles its own pub uses
pub mod log_config;
pub mod simulation;
pub mod interaction;
pub mod graphics;
pub mod event_log;
pub mod render_ascii;
pub mod ecs;
pub mod config;
pub mod sim_summary;
pub mod ecs_simulation;
pub mod sim_profile;
pub mod sim_core;
pub mod sim_loop;
pub mod sim_loop_unified;
pub mod util;
pub mod sim_state;
pub mod spawn_config;
mod legacy;

// If you want to restrict the public API, you can `pub use` only what you want to expose here.
// For now, this setup allows both main.rs and integration tests to access all needed modules.
// Removed unused import: agent::swimming::swimming_system
