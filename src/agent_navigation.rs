// Agent navigation, stuck detection, and pathfinding-related structs and helpers extracted from ecs_components.rs

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

// Any additional helper functions for stuck/pathfinding logic can be added here.
