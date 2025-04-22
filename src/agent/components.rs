use serde::{Serialize, Deserialize};
use legion::Entity;
use std::collections::VecDeque;

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

#[derive(Clone, Debug, PartialEq)]
pub struct RecentInteraction {
    pub partner: Entity,
    pub ticks_left: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InteractionState {
    pub target: Option<Entity>,
    pub ticks: u32,
    pub last_partner: Option<Entity>,
    pub cooldown: u32,
    pub recent_partners: VecDeque<RecentInteraction>,
}

impl InteractionState {
    pub fn update_recent(&mut self) {
        self.recent_partners.retain_mut(|ri| {
            if ri.ticks_left > 0 {
                ri.ticks_left -= 1;
                true
            } else {
                false
            }
        });
    }
    pub fn add_partner(&mut self, partner: Entity, ticks: u8) {
        self.recent_partners.push_back(RecentInteraction { partner, ticks_left: ticks });
    }
    pub fn has_recently_interacted(&self, partner: Entity) -> bool {
        self.recent_partners.iter().any(|ri| ri.partner == partner && ri.ticks_left > 0)
    }
}
