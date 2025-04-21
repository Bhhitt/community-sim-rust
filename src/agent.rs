//! Agent logic and data structures
// All legacy agent logic has been removed. Only AgentType is retained for ECS agent spawning.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AgentType {
    pub r#type: String,
    pub color: String,
    pub move_speed: f32,
    pub strength: i32,
    pub stamina: i32,
    pub vision: i32,
    pub work_rate: i32,
    pub icon: String,
    pub damping: Option<f32>,
}
