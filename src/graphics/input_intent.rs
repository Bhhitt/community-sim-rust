// Represents high-level user input actions/intents for the simulation
#[derive(Debug, Clone)]
pub enum InputIntent {
    Quit,
    TogglePause,
    AdvanceOneTick,
    MoveCamera { dx: f32, dy: f32 },
    SpawnAgentRandom,
    SpawnAgentsRandom { count: usize },
    SelectAgentAt { x: i32, y: i32 },
    // Add more as needed
}

// A queue/resource for storing user input intents, to be processed by ECS systems
#[derive(Default, Debug)]
pub struct InputQueue {
    pub intents: Vec<InputIntent>,
}

impl InputQueue {
    pub fn push(&mut self, intent: InputIntent) {
        self.intents.push(intent);
    }
    pub fn drain(&mut self) -> Vec<InputIntent> {
        std::mem::take(&mut self.intents)
    }
}
