//! ASCII rendering module for ECS simulation
use legion::World;
use legion::IntoQuery;
use crate::map::Map;

/// Renders the simulation state (terrain, agents, food) as ASCII.
pub fn render_simulation_ascii(world: &World, map: &Map) -> String {
    // Build a 2D buffer of chars
    let mut buffer = vec![vec![' '; map.width as usize]; map.height as usize];
    // Fill with terrain
    for y in 0..map.height as usize {
        for x in 0..map.width as usize {
            buffer[y][x] = map.tiles[y][x].to_char();
        }
    }
    // Overlay food
    let mut food_query = <(&crate::ecs_components::Position, &crate::food::Food)>::query();
    for (pos, _food) in food_query.iter(world) {
        let x = pos.x.round() as i32;
        let y = pos.y.round() as i32;
        if x >= 0 && y >= 0 && (x as usize) < map.width as usize && (y as usize) < map.height as usize {
            buffer[y as usize][x as usize] = 'F';
        }
    }
    // Overlay agents (must come after food to ensure agents are visible on top)
    let mut agent_query = <(&crate::ecs_components::Position, &crate::agent::AgentType)>::query();
    for (pos, _agent_type) in agent_query.iter(world) {
        let x = pos.x.round() as i32;
        let y = pos.y.round() as i32;
        if x >= 0 && y >= 0 && (x as usize) < map.width as usize && (y as usize) < map.height as usize {
            buffer[y as usize][x as usize] = 'A';
        }
    }
    // Convert buffer to String
    let mut ascii = String::new();
    for row in buffer {
        for ch in row {
            ascii.push(ch);
        }
        ascii.push('\n');
    }
    ascii
}
