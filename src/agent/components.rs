use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub move_probability: Option<f32>, // Probability to move each tick (0.0-1.0)
    pub name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hunger {
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Energy {
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InteractionState {
    pub target: Option<legion::Entity>,
    pub ticks: u32,
    pub last_partner: Option<legion::Entity>,
    pub cooldown: u32,
}
