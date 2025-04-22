#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Food {
    pub nutrition: f32,
}

pub struct PendingFoodSpawns(pub std::collections::VecDeque<(f32, f32)>);
