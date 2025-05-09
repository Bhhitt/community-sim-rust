// Handles SDL2 input and event handling

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use crate::graphics::sim_state::SimUIState;
use crate::agent;
use crate::log_config::LogConfig;
use crate::graphics::input_intent::InputIntent;

/// Collects SDL2 events and populates the InputQueue resource with user intents.
pub fn collect_input_events(
    event_pump: &mut EventPump,
    window_id: u32,
    sim_ui_state: &mut SimUIState,
    _agent_types: &[agent::AgentType],
    _render_map: &crate::map::Map,
    _cell_size: f32,
    _log_config: &LogConfig,
    _paused: bool,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                sim_ui_state.input_queue.push(InputIntent::Quit);
            }
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                sim_ui_state.input_queue.push(InputIntent::TogglePause);
            }
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                sim_ui_state.input_queue.push(InputIntent::MoveCamera { dx: 5.0, dy: 0.0 });
            }
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                sim_ui_state.input_queue.push(InputIntent::MoveCamera { dx: -5.0, dy: 0.0 });
            }
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                sim_ui_state.input_queue.push(InputIntent::MoveCamera { dx: 0.0, dy: -5.0 });
            }
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                sim_ui_state.input_queue.push(InputIntent::MoveCamera { dx: 0.0, dy: 5.0 });
            }
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                sim_ui_state.input_queue.push(InputIntent::SpawnAgentRandom);
            }
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                sim_ui_state.input_queue.push(InputIntent::SpawnAgentsRandom { count: 100 });
            }
            Event::KeyDown { keycode: Some(Keycode::Period), .. } => {
                if _paused {
                    sim_ui_state.input_queue.push(InputIntent::AdvanceOneTick);
                }
            }
            Event::MouseButtonDown { x, y, window_id: evt_win_id, .. } => {
                if evt_win_id == window_id {
                    sim_ui_state.input_queue.push(InputIntent::SelectAgentAt { x, y });
                }
            }
            _ => {}
        }
    }
}

// LEGACY: The old handle_events function is left for reference below, but should not be used.
// pub fn handle_events(
//     event_pump: &mut EventPump,
//     window_id: u32,
//     sim_ui_state: &mut SimUIState,
//     _agent_types: &[agent::AgentType],
//     _render_map: &crate::map::Map,
//     _cell_size: f32,
//     _log_config: &LogConfig,
//     _paused: &mut bool,
//     _advance_one: &mut bool,
// ) {
//     for event in event_pump.poll_iter() {
//         match event {
//             Event::Quit { .. }
//             | Event::KeyDown {
//                 keycode: Some(Keycode::Escape),
//                 ..
//             } => std::process::exit(0),
//             Event::KeyDown {
//                 keycode: Some(Keycode::Space),
//                 ..
//             } => *_paused = !*_paused,
//             Event::KeyDown {
//                 keycode: Some(Keycode::Right), .. } => sim_ui_state.camera.move_by(5.0 as f32, 0.0 as f32, _render_map.width, _render_map.height, _cell_size as u32),
//             Event::KeyDown {
//                 keycode: Some(Keycode::Left), .. } => sim_ui_state.camera.move_by(-5.0 as f32, 0.0 as f32, _render_map.width, _render_map.height, _cell_size as u32),
//             Event::KeyDown {
//                 keycode: Some(Keycode::Up), .. } => sim_ui_state.camera.move_by(0.0 as f32, -5.0 as f32, _render_map.width, _render_map.height, _cell_size as u32),
//             Event::KeyDown {
//                 keycode: Some(Keycode::Down), .. } => sim_ui_state.camera.move_by(0.0 as f32, 5.0 as f32, _render_map.width, _render_map.height, _cell_size as u32),
//             Event::KeyDown {
//                 keycode: Some(Keycode::A), .. } => {
//                 // Add agent at a random passable location
//                 let mut rng = rand::thread_rng();
//                 let mut x;
//                 let mut y;
//                 let mut tries = 0;
//                 loop {
//                     x = rng.gen_range(0.._render_map.width) as f32;
//                     y = rng.gen_range(0.._render_map.height) as f32;
//                     if _render_map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || _render_map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
//                         break;
//                     }
//                     tries += 1;
//                     if tries > 1000 {
//                         panic!("Could not find passable tile for agent after 1000 tries");
//                     }
//                 }
//                 // Use the first agent type for simplicity
//                 if let Some(agent_type) = _agent_types.get(0) {
//                     let agent_type = agent_type.clone();
//                     // spawn_agent(sim_ui_state.world, crate::ecs_components::Position { x, y }, agent_type, _render_map);
//                     // log::debug!("[DEBUG] Added agent at ({}, {})", x, y);
//                 } else {
//                     // log::debug!("[ERROR] No agent types defined!");
//                 }
//             },
//             Event::KeyDown {
//                 keycode: Some(Keycode::S), .. } => {
//                 // Spawn 100 random agents at 100 different locations, each with a random agent type
//                 let mut rng = rand::thread_rng();
//                 let mut spawned = 0;
//                 let mut attempts = 0;
//                 let max_agents = 100;
//                 let max_tries_per_agent = 1000;
//                 let num_types = _agent_types.len().max(1);
//                 while spawned < max_agents && attempts < max_agents * max_tries_per_agent {
//                     let x = rng.gen_range(0.._render_map.width) as f32;
//                     let y = rng.gen_range(0.._render_map.height) as f32;
//                     if _render_map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || _render_map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
//                         let type_idx = rng.gen_range(0..num_types);
//                         let agent_type = _agent_types[type_idx].clone();
//                         // spawn_agent(sim_ui_state.world, crate::ecs_components::Position { x, y }, agent_type, _render_map);
//                         spawned += 1;
//                     }
//                     attempts += 1;
//                 }
//                 // log::debug!("[DEBUG] Spawned {} agents ({} attempts)", spawned, attempts);
//             },
//             Event::KeyDown {
//                 keycode: Some(Keycode::Period), .. } => {
//                 // Advance one tick if paused
//                 if *_paused {
//                     *_advance_one = true;
//                     // log::debug!("[DEBUG] Advance one tick (paused)");
//                 }
//             },
//             Event::MouseButtonDown { x, y, window_id: evt_win_id, .. } => {
//                 if evt_win_id == window_id {
//                     let mouse_x = x;
//                     let mouse_y = y;
//                     let mut found_agent = None;
//                     let mut topmost_y = -1.0_f32;
//                     // for (_entity, (pos,)) in <(legion::Entity, (&crate::ecs_components::Position,))>::query().iter(sim_ui_state.world) {
//                     //     let rect = Rect::new(
//                     //         ((pos.x - sim_ui_state.camera.x) * _cell_size as f32) as i32,
//                     //         ((pos.y - sim_ui_state.camera.y) * _cell_size as f32) as i32,
//                     //         _cell_size as u32,
//                     //         _cell_size as u32,
//                     //     );
//                     //     if mouse_x >= rect.x && mouse_x < rect.x + rect.width() as i32 &&
//                     //        mouse_y >= rect.y && mouse_y < rect.y + rect.height() as i32 {
//                     //         if pos.y > topmost_y {
//                     //             found_agent = Some(_entity);
//                     //             topmost_y = pos.y;
//                     //         }
//                     //     }
//                     // }
//                     // if found_agent.is_none() {
//                     //     let map_x = (x as f32 / _cell_size as f32 + sim_ui_state.camera.x).floor();
//                     //     let map_y = (y as f32 / _cell_size as f32 + sim_ui_state.camera.y).floor();
//                     //     // for (_entity, (pos,)) in <(legion::Entity, (&crate::ecs_components::Position,))>::query().iter(sim_ui_state.world) {
//                     //     //     let food_cell_x = pos.x.floor();
//                     //     //     let food_cell_y = pos.y.floor();
//                     //     //     if (food_cell_x - map_x).abs() < 0.5 && (food_cell_y - map_y).abs() < 0.5 {
//                     //     //         found_agent = Some(_entity);
//                     //     //         break;
//                     //     //     }
//                     //     // }
//                     // }
//                     // sim_ui_state.selected_agent = found_agent.copied();
//                     // if let Some(sel) = sim_ui_state.selected_agent {
//                     //     // log::debug!("[DEBUG] Selected entity {:?}", sel);
//                     // } else {
//                     //     // log::debug!("[DEBUG] No agent or food found at clicked cell");
//                     //     let map_x = (x as f32 / _cell_size as f32 + sim_ui_state.camera.x).floor();
//                     //     let map_y = (y as f32 / _cell_size as f32 + sim_ui_state.camera.y).floor();
//                     //     // sim_ui_state.empty_cell_flash = Some((map_x as i32, map_y as i32, std::time::Instant::now()));
//                     // }
//                 } else {
//                     // For stats/log windows: just log and discard
//                     // log::debug!("[DEBUG] Mouse click in non-main window (id {})", evt_win_id);
//                 }
//             },
//             _ => {
//                 // Poll and discard all other events for ALL windows, including stats/log windows.
//                 // This keeps all SDL2 windows alive and responsive.
//             }
//         }
//     }
// }
