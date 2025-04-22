// Main simulation rendering and event loop
// Will contain the main SDL2 rendering logic and event loop

use legion::*;
use crate::agent::AgentType;
use crate::agent::systems::{spawn_agent, agent_movement_system, agent_death_system};
use crate::food::food_spawn_apply_system;
use crate::food::PendingFoodSpawns;
use rand::Rng;
use crate::graphics::camera::Camera;
use crate::graphics::render::terrain::draw_terrain;
use crate::graphics::render::agent::{draw_agents, draw_selected_agent_path};
use crate::graphics::render::overlays::{draw_event_log_window, draw_empty_cell_flash, draw_stats_window};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

const CELL_SIZE: f32 = 6.0;
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

// Main SDL2 rendering/event loop, extracted from graphics.rs
#[allow(clippy::too_many_arguments)]
pub fn run_sim_render(
    _map_width: i32,
    _map_height: i32,
    _num_agents: usize,
    agent_types: &[AgentType],
    profile_systems: bool,
    profile_csv: &str,
    world: &mut World,
    resources: &mut Resources,
    schedule: &mut Schedule,
) {
    // --- ECS World Setup ---
    let map = crate::map::Map::new(_map_width, _map_height);
    let render_map = map.clone();
    let mut rng = rand::thread_rng();
    let mut _agent_count = 0;
    let mut _attempts = 0;
    if _num_agents > 0 {
        for i in 0.._num_agents {
            let mut x;
            let mut y;
            let mut tries = 0;
            loop {
                x = rng.gen_range(0.._map_width) as f32;
                y = rng.gen_range(0.._map_height) as f32;
                if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for agent after 1000 tries");
                }
            }
            let agent_type = agent_types[i % agent_types.len()].clone();
            spawn_agent(world, crate::ecs_components::Position { x, y }, agent_type, &map);
            _agent_count += 1;
            _attempts += tries;
        }
    }
    let agent_count_check = <(Read<crate::ecs_components::Position>,)>::query().iter(world).count();
    log::debug!("[DEBUG] Number of agents spawned: {}", agent_count_check);
    resources.insert(map.clone());
    resources.insert(crate::ecs_components::InteractionStats::default());
    resources.insert(crate::event_log::EventLog::new(200));
    resources.insert(PendingFoodSpawns(std::collections::VecDeque::new()));
    resources.insert(crate::ecs_components::FoodPositions(Vec::new()));
    // --- Use PARALLEL schedule ---
    // Profiling support
    let mut csv_file = if profile_systems {
        Some(File::create(profile_csv).expect("Failed to create csv file"))
    } else {
        None
    };
    if let Some(csv_file) = csv_file.as_mut() {
        writeln!(csv_file, "tick,agent_movement,entity_interaction,agent_death,food_spawn_collect,food_spawn_apply").unwrap();
    }
    let mut tick = 0;
    let mut agent_movement = agent_movement_system();
    let mut entity_interaction = crate::ecs_components::entity_interaction_system();
    let mut agent_death = agent_death_system();
    let mut food_spawn_collect = crate::collect_food_spawn_positions_system();
    let mut food_spawn_apply = food_spawn_apply_system();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Community Simulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let window_id = canvas.window().id();
    let _texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut camera = Camera::new(_map_width, _map_height, CELL_SIZE as u32, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut paused = false;
    let mut advance_one = false;
    let _ascii_snapshots: Vec<String> = Vec::new();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = "/System/Library/Fonts/Supplemental/Arial.ttf";
    let font = ttf_context.load_font(font_path, 18).unwrap();
    log::info!("[FONT] Loaded font from {}", font_path);

    // Try to create stats window canvas with software renderer for diagnostics
    let stats_window_canvas = match video_subsystem.window("Stats", 320, 700)
        .position(0, 0)
        .resizable()
        .build() {
        Ok(window) => match window.into_canvas().software().build() {
            Ok(canvas) => {
                log::info!("[STATS] Stats window canvas created with software renderer");
                canvas
            },
            Err(e) => {
                log::error!("[STATS] Failed to create stats window canvas (software): {}", e);
                panic!("Failed to create stats window canvas: {}", e);
            }
        },
        Err(e) => {
            log::error!("[STATS] Failed to create stats window: {}", e);
            panic!("Failed to create stats window: {}", e);
        }
    };
    let mut stats_canvas = stats_window_canvas;
    let _stats_window_id = stats_canvas.window().id();

    // --- New: Event Log Window ---
    let log_window_canvas = video_subsystem.window("Event Log", 600, 480)
        .position(340, 0)
        .resizable()
        .build().unwrap()
        .into_canvas().build().unwrap();
    let mut log_canvas = log_window_canvas;
    let _log_window_id = log_canvas.window().id();

    let mut cached_agent_counts: Vec<(String, usize)> = Vec::new();
    let mut last_stats_update = std::time::Instant::now();
    let mut selected_agent: Option<legion::Entity> = None;
    let mut empty_cell_flash: Option<(i32, i32, std::time::Instant)> = None;

    'running: loop {
        // --- Run ECS systems ---
        if !paused || advance_one {
            if profile_systems {
                use crate::ecs_simulation::simulation_tick_profiled;
                let profile = simulation_tick_profiled(
                    world,
                    resources,
                    &mut agent_movement,
                    &mut entity_interaction,
                    &mut agent_death,
                    &mut food_spawn_collect,
                    &mut food_spawn_apply,
                );
                if let Some(csv_file) = csv_file.as_mut() {
                    writeln!(csv_file, "{}{}{}", tick, ",", profile.to_csv_row()).unwrap();
                }
            } else {
                // Use parallel tick
                let _ = crate::ecs_simulation::simulation_tick_parallel(world, resources, schedule);
            }
            tick += 1;
            advance_one = false;
        }
        // --- Print latest EventLog entry to console ---
        if let Some(event_log) = resources.get::<crate::event_log::EventLog>() {
            if !event_log.events.is_empty() {
                if let Some(last_event) = event_log.events.back() {
                    log::debug!("{}", last_event);
                }
            }
        }
        // --- Render Event Log Window ---
        if let Some(event_log) = resources.get::<crate::event_log::EventLog>() {
            draw_event_log_window(&mut log_canvas, &font, &event_log);
        }
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => paused = !paused,
                Event::KeyDown {
                    keycode: Some(Keycode::Right), .. } => camera.move_by(5.0 as f32, 0.0 as f32, _map_width, _map_height, CELL_SIZE as u32),
                Event::KeyDown {
                    keycode: Some(Keycode::Left), .. } => camera.move_by(-5.0 as f32, 0.0 as f32, _map_width, _map_height, CELL_SIZE as u32),
                Event::KeyDown {
                    keycode: Some(Keycode::Up), .. } => camera.move_by(0.0 as f32, -5.0 as f32, _map_width, _map_height, CELL_SIZE as u32),
                Event::KeyDown {
                    keycode: Some(Keycode::Down), .. } => camera.move_by(0.0 as f32, 5.0 as f32, _map_width, _map_height, CELL_SIZE as u32),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    // Add agent at a random passable location
                    let mut rng = rand::thread_rng();
                    let mut x;
                    let mut y;
                    let mut tries = 0;
                    loop {
                        x = rng.gen_range(0.._map_width) as f32;
                        y = rng.gen_range(0.._map_height) as f32;
                        if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                            break;
                        }
                        tries += 1;
                        if tries > 1000 {
                            panic!("Could not find passable tile for agent after 1000 tries");
                        }
                    }
                    // Use the first agent type for simplicity
                    if let Some(agent_type) = agent_types.get(0) {
                        let agent_type = agent_type.clone();
                        spawn_agent(world, crate::ecs_components::Position { x, y }, agent_type, &map);
                        log::debug!("[DEBUG] Added agent at ({}, {})", x, y);
                    } else {
                        log::debug!("[ERROR] No agent types defined!");
                    }
                },
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    // Spawn 100 random agents at 100 different locations, each with a random agent type
                    let mut rng = rand::thread_rng();
                    let mut spawned = 0;
                    let mut attempts = 0;
                    let max_agents = 100;
                    let max_tries_per_agent = 1000;
                    let num_types = agent_types.len().max(1);
                    while spawned < max_agents && attempts < max_agents * max_tries_per_agent {
                        let x = rng.gen_range(0.._map_width) as f32;
                        let y = rng.gen_range(0.._map_height) as f32;
                        if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                            let type_idx = rng.gen_range(0..num_types);
                            let agent_type = agent_types[type_idx].clone();
                            spawn_agent(world, crate::ecs_components::Position { x, y }, agent_type, &map);
                            spawned += 1;
                        }
                        attempts += 1;
                    }
                    log::debug!("[DEBUG] Spawned {} agents ({} attempts)", spawned, attempts);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Period),
                    ..
                } => {
                    // Advance one tick if paused
                    if paused {
                        advance_one = true;
                        log::debug!("[DEBUG] Advance one tick (paused)");
                    }
                },
                Event::MouseButtonDown { x, y, window_id: evt_win_id, .. } => {
                    if evt_win_id == window_id {
                        let mouse_x = x;
                        let mouse_y = y;
                        let mut found_agent = None;
                        let mut topmost_y = -1.0_f32;
                        for (_entity, (pos,)) in <(legion::Entity, (&crate::ecs_components::Position,))>::query().iter(world) {
                            let rect = Rect::new(
                                ((pos.x - camera.x) * CELL_SIZE as f32) as i32,
                                ((pos.y - camera.y) * CELL_SIZE as f32) as i32,
                                CELL_SIZE as u32,
                                CELL_SIZE as u32,
                            );
                            if mouse_x >= rect.x && mouse_x < rect.x + rect.width() as i32 &&
                               mouse_y >= rect.y && mouse_y < rect.y + rect.height() as i32 {
                                if pos.y > topmost_y {
                                    found_agent = Some(_entity);
                                    topmost_y = pos.y;
                                }
                            }
                        }
                        if found_agent.is_none() {
                            let map_x = (x as f32 / CELL_SIZE as f32 + camera.x).floor();
                            let map_y = (y as f32 / CELL_SIZE as f32 + camera.y).floor();
                            for (_entity, (pos,)) in <(legion::Entity, (&crate::ecs_components::Position,))>::query().iter(world) {
                                let food_cell_x = pos.x.floor();
                                let food_cell_y = pos.y.floor();
                                if (food_cell_x - map_x).abs() < 0.5 && (food_cell_y - map_y).abs() < 0.5 {
                                    found_agent = Some(_entity);
                                    break;
                                }
                            }
                        }
                        selected_agent = found_agent.copied();
                        if let Some(sel) = selected_agent {
                            log::debug!("[DEBUG] Selected entity {:?}", sel);
                        } else {
                            log::debug!("[DEBUG] No agent or food found at clicked cell");
                            let map_x = (x as f32 / CELL_SIZE as f32 + camera.x).floor();
                            let map_y = (y as f32 / CELL_SIZE as f32 + camera.y).floor();
                            empty_cell_flash = Some((map_x as i32, map_y as i32, std::time::Instant::now()));
                        }
                    } else {
                        // For stats/log windows: just log and discard
                        log::debug!("[DEBUG] Mouse click in non-main window (id {})", evt_win_id);
                    }
                },
                _ => {
                    // Poll and discard all other events for ALL windows, including stats/log windows.
                    // This keeps all SDL2 windows alive and responsive.
                }
            }
        }
        // Draw terrain
        log::debug!("[DEBUG] About to render terrain");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_terrain(&mut canvas, &render_map, camera.x, camera.y, CELL_SIZE);
        // --- Draw selected agent's path on the map ---
        draw_selected_agent_path(&mut canvas, world, selected_agent, camera.x, camera.y, CELL_SIZE);
        // Draw ECS agents and food
        log::debug!("[DEBUG] About to query agents for rendering");
        draw_agents(&mut canvas, world, camera.x, camera.y, CELL_SIZE);
        // --- Flash highlight for empty cell click ---
        if let Some((fx, fy, t)) = empty_cell_flash {
            if t.elapsed().as_millis() < 200 {
                draw_empty_cell_flash(&mut canvas, fx, fy, camera.x, camera.y, CELL_SIZE);
            } else {
                empty_cell_flash = None;
            }
        }
        log::debug!("[DEBUG] About to present main canvas");
        canvas.present();

        // --- Stats window rendering ---
        // Diagnostics for stats window rendering
        let (stats_w, stats_h) = stats_canvas.window().size();
        log::info!("[STATS] Window size: {}x{}", stats_w, stats_h);
        if cached_agent_counts.is_empty() {
            log::warn!("[STATS] cached_agent_counts is empty!");
        } else {
            log::info!("[STATS] cached_agent_counts: {:?}", cached_agent_counts);
        }
        // Update cached_agent_counts once per second, but always render stats window every frame
        if last_stats_update.elapsed().as_secs_f32() >= 1.0 {
            let agent_counts = {
                let mut agent_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                for agent_type in <&AgentType>::query().iter(world) {
                    *agent_counts.entry(agent_type.r#type.clone()).or_insert(0) += 1;
                }
                agent_counts
            };
            let food_count = {
                <(Read<crate::ecs_components::Position>,)>::query()
                    .filter(component::<crate::food::Food>())
                    .iter(world)
                    .count()
            };
            let mut counts_vec: Vec<(String, usize)> = agent_counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
            counts_vec.sort_by(|a, b| a.0.cmp(&b.0));
            let interaction_count = resources.get::<crate::ecs_components::InteractionStats>().map_or(0, |stats| stats.agent_interactions);
            counts_vec.push(("food".to_string(), food_count));
            counts_vec.push(("interactions".to_string(), interaction_count));
            cached_agent_counts = counts_vec;
            last_stats_update = std::time::Instant::now();
        }
        draw_stats_window(&mut stats_canvas, &font, &cached_agent_counts, resources.get::<crate::ecs_components::InteractionStats>().as_deref(), selected_agent, world);
        log::debug!("[DEBUG] About to present stats canvas");
        stats_canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }
    // --- At end of simulation, write summary to simulation_ascii.txt ---
    use crate::sim_summary::write_simulation_summary_and_ascii;
    write_simulation_summary_and_ascii(
        world,
        resources,
        &render_map,
        tick,
        "simulation_ascii.txt",
    );
}
