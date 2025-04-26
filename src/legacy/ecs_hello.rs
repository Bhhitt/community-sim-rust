// LEGACY: This file is a minimal Legion ECS hello world for migration scaffolding. Not used in the main simulation.

//! Minimal Legion ECS hello world for migration scaffolding

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ecs_hello() {
        let mut world = legion::World::default();
        let mut resources = legion::Resources::default();
        world.push((Position { x: 1.0, y: 2.0 },));
        world.push((Position { x: 3.0, y: 4.0 },));
        let mut schedule = legion::Schedule::builder().build();
        schedule.execute(&mut world, &mut resources);
    }
}
