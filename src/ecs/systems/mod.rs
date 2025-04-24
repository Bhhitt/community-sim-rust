// Re-export all ECS systems here for easy use in schedule.rs and elsewhere
// Example:
pub mod agent;
pub mod food;
pub mod terrain;
// Add more as you migrate systems

// TODO: Create one file per system (agent.rs, food.rs, etc.) and move system logic from ecs_sim.rs, ecs_simulation.rs, etc.
