use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub struct Target {
    pub x: f32,
    pub y: f32,
    pub stuck_ticks: u32, // Track how many ticks agent is stuck
    pub path_ticks: Option<u32>,
    pub ticks_to_reach: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub waypoints: VecDeque<(f32, f32)>,
}

impl Default for Path {
    fn default() -> Self {
        Path { waypoints: std::collections::VecDeque::new() }
    }
}
