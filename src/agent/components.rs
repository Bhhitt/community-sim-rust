use serde::{Serialize, Deserialize};
use legion::Entity;
use std::collections::VecDeque;
use crate::agent::mlp::MLPConfig;

pub mod agent_state;
pub use agent_state::AgentState;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MovementEffect {
    None,
    Blocked,
    Slowed(f32),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovementProfile {
    pub speed: f32,
    pub effect: MovementEffect,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DecisionEngineConfig {
    Simple,
    MLP(MLPConfig),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentType {
    pub name: String,
    pub color: (u8, u8, u8),
    pub movement_profile: MovementProfile,
    pub decision_engine: DecisionEngineConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hunger {
    pub value: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Energy {
    pub value: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecentInteraction {
    pub partner: Option<Entity>,
    pub ticks_since: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractionState {
    pub target: Option<Entity>,
    pub ticks: u32,
    pub last_partner: Option<Entity>,
    pub cooldown: u32,
    pub recent_partners: VecDeque<RecentInteraction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovementHistory {
    pub positions: VecDeque<(f32, f32)>,
    pub max_len: usize,
}

impl MovementHistory {
    pub fn new(max_len: usize) -> Self {
        Self {
            positions: VecDeque::with_capacity(max_len),
            max_len,
        }
    }
    pub fn add(&mut self, x: f32, y: f32) {
        if self.positions.len() == self.max_len {
            self.positions.pop_front();
        }
        self.positions.push_back((x, y));
    }
}
