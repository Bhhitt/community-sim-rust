#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentState {
    Idle,
    Moving,
    Arrived,
    Swimming, // New state for agents in water
}
