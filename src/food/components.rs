#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Food {
    pub nutrition: f32,
}

use std::collections::VecDeque;

pub struct PendingFoodSpawns(pub VecDeque<(f32, f32)>);

impl Default for PendingFoodSpawns {
    fn default() -> Self {
        PendingFoodSpawns(VecDeque::new())
    }
}
