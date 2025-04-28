// Minimal simulation state for unified simulation loop (headless mode)
use legion::*;

pub struct SimState<'a> {
    pub world: &'a mut World,
    pub resources: &'a mut Resources,
    pub schedule: &'a mut Schedule,
    pub tick: i32,
}

impl<'a> SimState<'a> {
    pub fn new(world: &'a mut World, resources: &'a mut Resources, schedule: &'a mut Schedule) -> Self {
        Self {
            world,
            resources,
            schedule,
            tick: 0,
        }
    }
}
