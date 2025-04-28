use serde::{Serialize, Deserialize};
use legion::Entity;
use std::collections::VecDeque;
use crate::agent::mlp::MLPConfig;

pub mod agent_state;
/// Represents the state of an agent in the simulation.
///
/// - `Idle`: Agent is idle and not moving.
/// - `Moving`: Agent is moving on land.
/// - `Arrived`: Agent has reached its destination.
/// - `Swimming`: Agent is in water and can move through water tiles. Only agents in this state can traverse water.
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
pub enum AgentStatus {
    Normal,
    Hungry,
    // Add more statuses as needed
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentType {
    pub name: String,
    pub color: (u8, u8, u8),
    pub movement_profile: MovementProfile,
    pub decision_engine: DecisionEngineConfig,
    pub hunger_rate: f32,
    pub hunger_threshold: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hunger {
    pub value: f32,
    pub threshold: f32, // Configurable per agent type
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

// Removed duplicate Path struct definition. Use crate::navigation::Path instead.

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SwimmingProfile {
    pub swim_chance_percent: u8, // 1-30, how likely this agent is to want to swim
    pub swim_ticks_remaining: u32, // how many ticks left to swim, 0 if not swimming
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IdlePause {
    pub ticks_remaining: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum IntendedAction {
    SeekFood,
    Wander,
    Idle,
    // Add more as needed
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Target {
    pub x: f32,
    pub y: f32,
    pub stuck_ticks: u32,
    pub path_ticks: Option<u32>,
    pub ticks_to_reach: Option<u32>,
}
