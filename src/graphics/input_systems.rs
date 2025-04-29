use crate::graphics::input_intent::InputIntent;
use crate::graphics::sim_state::SimUIState;
use crate::agent::{AgentType, event::AgentEventLog};
use crate::ecs_components::Position;
use crate::map::Terrain;
use crate::ecs::systems::pending_agent_spawns::{AgentSpawnRequest, PendingAgentSpawns};
use legion::IntoQuery;
use legion::systems::Resources;

/// Processes all input intents from the InputQueue, mutating the ECS world and UI state as needed.
pub fn process_input_intents(
    sim_ui_state: &mut SimUIState,
    agent_types: &[AgentType],
    render_map: &crate::map::Map,
    _cell_size: f32,
    paused: &mut bool,
    advance_one: &mut bool,
) {
    let resources = &mut sim_ui_state.resources;
    let intents = sim_ui_state.input_queue.drain();
    for intent in intents {
        log::debug!("[INPUT] Processing intent: {:?}", intent);
        match intent {
            InputIntent::Quit => {
                std::process::exit(0);
            }
            InputIntent::TogglePause => {
                *paused = !*paused;
                log::info!("[INPUT] Paused state toggled: {}", *paused);
            }
            InputIntent::AdvanceOneTick => {
                *advance_one = true;
                log::info!("[INPUT] Advance one tick triggered");
            }
            InputIntent::MoveCamera { dx, dy } => {
                sim_ui_state.camera.move_by(dx, dy, render_map.width, render_map.height);
                log::info!("[INPUT] Camera move: dx={}, dy={}", dx, dy);
            }
            InputIntent::SpawnAgentRandom => {
                log::info!("[INPUT] SpawnAgentRandom intent received");
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let mut x;
                let mut y;
                let mut tries = 0;
                loop {
                    x = rng.gen_range(0..render_map.width) as f32;
                    y = rng.gen_range(0..render_map.height) as f32;
                    if render_map.tiles[y as usize][x as usize] == Terrain::Grass || render_map.tiles[y as usize][x as usize] == Terrain::Forest {
                        break;
                    }
                    tries += 1;
                    if tries > 1000 {
                        log::error!("Could not find passable tile for agent after 1000 tries");
                        return;
                    }
                }
                let mut agent_event_log = resources.get_mut::<AgentEventLog>().expect("AgentEventLog missing");
                if let Some(agent_type) = agent_types.get(0) {
                    let agent_type = agent_type.clone();
                    // TODO: Re-enable agent spawning in graphics mode
                    // let mut pending_spawns = resources.get_mut::<PendingAgentSpawns>().unwrap();
                    // pending_spawns.add(Position { x, y }, agent_type);
                    // log::debug!("[DEBUG] Added agent at ({}, {})", x, y);
                } else {
                    log::debug!("[ERROR] No agent types defined!");
                }
            }
            InputIntent::SpawnAgentsRandom { count } => {
                log::info!("[INPUT] SpawnAgentsRandom intent received: count={}", count);
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let mut spawned = 0;
                let mut attempts = 0;
                let max_tries_per_agent = 1000;
                let num_types = agent_types.len().max(1);
                let mut agent_event_log = resources.get_mut::<AgentEventLog>().expect("AgentEventLog missing");
                while spawned < count && attempts < count * max_tries_per_agent {
                    let x = rng.gen_range(0..render_map.width) as f32;
                    let y = rng.gen_range(0..render_map.height) as f32;
                    if render_map.tiles[y as usize][x as usize] == Terrain::Grass || render_map.tiles[y as usize][x as usize] == Terrain::Forest {
                        let type_idx = rng.gen_range(0..num_types);
                        let agent_type = agent_types[type_idx].clone();
                        // TODO: Re-enable agent spawning in graphics mode
                        // let mut pending_spawns = resources.get_mut::<PendingAgentSpawns>().unwrap();
                        // pending_spawns.add(Position { x, y }, agent_type.clone());
                        // spawned += 1;
                        // log::debug!("[DEBUG] Enqueued AgentSpawnRequest at ({}, {}) type: {}", x, y, agent_type.name);
                    }
                    attempts += 1;
                }
                log::debug!("[DEBUG] Spawned {} agents ({} attempts)", spawned, attempts);
            }
            InputIntent::SelectAgentAt { x, y } => {
                log::info!("[INPUT] SelectAgentAt intent: x={}, y={}", x, y);
                use sdl2::rect::Rect;
                let mouse_x = x;
                let mouse_y = y;
                let mut found_agent = None;
                let mut topmost_y = -1.0_f32;
                for (entity, (pos,)) in <(legion::Entity, (&Position,) )>::query().iter(sim_ui_state.world) {
                    let rect = Rect::new(
                        ((pos.x - sim_ui_state.camera.x) * _cell_size as f32) as i32,
                        ((pos.y - sim_ui_state.camera.y) * _cell_size as f32) as i32,
                        _cell_size as u32,
                        _cell_size as u32,
                    );
                    if mouse_x >= rect.x && mouse_x < rect.x + rect.width() as i32 &&
                        mouse_y >= rect.y && mouse_y < rect.y + rect.height() as i32 {
                        if pos.y > topmost_y {
                            found_agent = Some(*entity);
                            topmost_y = pos.y;
                        }
                    }
                }
                if found_agent.is_none() {
                    let map_x = (x as f32 / _cell_size as f32 + sim_ui_state.camera.x).floor();
                    let map_y = (y as f32 / _cell_size as f32 + sim_ui_state.camera.y).floor();
                    for (entity, (pos,)) in <(legion::Entity, (&Position,) )>::query().iter(sim_ui_state.world) {
                        let food_cell_x = pos.x.floor();
                        let food_cell_y = pos.y.floor();
                        if (food_cell_x - map_x).abs() < 0.5 && (food_cell_y - map_y).abs() < 0.5 {
                            found_agent = Some(*entity);
                            break;
                        }
                    }
                }
                sim_ui_state.selected_agent = found_agent;
                if let Some(sel) = sim_ui_state.selected_agent {
                    log::debug!("[DEBUG] Selected entity {:?}", sel);
                } else {
                    log::debug!("[DEBUG] No agent or food found at clicked cell");
                    let map_x = (x as f32 / _cell_size as f32 + sim_ui_state.camera.x).floor();
                    let map_y = (y as f32 / _cell_size as f32 + sim_ui_state.camera.y).floor();
                    sim_ui_state.empty_cell_flash = Some((map_x as i32, map_y as i32, std::time::Instant::now()));
                }
            }
        }
    }
}
