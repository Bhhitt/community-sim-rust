//! SDL2 graphics frontend

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::TextureQuery;
use rand::Rng;
use std::fs::File;
use std::io::Write;

const CELL_SIZE: u32 = 6;
const AGENT_COLOR: Color = Color::RGB(220, 40, 40);
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(_map_width: i32, _map_height: i32, cell_size: u32) -> Self {
        let width = WINDOW_WIDTH / cell_size;
        let height = WINDOW_HEIGHT / cell_size;
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }
    pub fn move_by(&mut self, dx: f32, dy: f32, _map_width: i32, _map_height: i32, _cell_size: u32) {
        let max_x = (self.width as f32).max(0.0);
        let max_y = (self.height as f32).max(0.0);
        self.x = (self.x + dx).clamp(0.0, max_x);
        self.y = (self.y + dy).clamp(0.0, max_y);
    }
}

fn terrain_color(terrain: &super::map::Terrain) -> Color {
    match terrain {
        super::map::Terrain::Grass => Color::RGB(60, 180, 75),
        super::map::Terrain::Water => Color::RGB(0, 120, 220),
        super::map::Terrain::Forest => Color::RGB(34, 139, 34),
        super::map::Terrain::Mountain => Color::RGB(120, 120, 120),
    }
}

pub fn run_with_graphics_profile(_map_width: i32, _map_height: i32, _num_agents: usize, agent_types: &[crate::agent::AgentType], profile_systems: bool, profile_csv: &str) {
    use crate::ecs_components::{spawn_agent, AgentType, agent_movement_system, entity_interaction_system, food_spawn_apply_system, agent_death_system, PendingFoodSpawns};
    use legion::*;
    // --- ECS World Setup ---
    let mut world = World::default();
    let map = super::map::Map::new(_map_width, _map_height);
    let render_map = map.clone(); // OK to clone for rendering only
    let mut rng = rand::thread_rng();
    // Convert agent_types from agent.rs to ecs_components::AgentType
    let ecs_agent_types: Vec<AgentType> = agent_types.iter().map(|a| AgentType {
        name: a.r#type.clone(),
        move_speed: a.move_speed,
        move_probability: a.move_probability,
        color: a.color.clone(),
    }).collect();
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
                if map.tiles[y as usize][x as usize] == super::map::Terrain::Grass || map.tiles[y as usize][x as usize] == super::map::Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for agent after 1000 tries");
                }
            }
            let agent_type = ecs_agent_types[i % ecs_agent_types.len()].clone();
            spawn_agent(&mut world, crate::ecs_components::Position { x, y }, agent_type);
            _agent_count += 1;
            _attempts += tries;
        }
    }
    // DEBUG: Print number of agents spawned
    let agent_count_check = <(Read<crate::ecs_components::Position>,)>::query().iter(&world).count();
    println!("[DEBUG] Number of agents spawned: {}", agent_count_check);
    let mut resources = Resources::default();
    resources.insert(map.clone());
    resources.insert(crate::ecs_components::InteractionStats::default());
    resources.insert(crate::ecs_components::EventLog::new(200));
    resources.insert(PendingFoodSpawns(Vec::new()));
    // --- Use PARALLEL schedule ---
    let mut schedule = crate::ecs_simulation::build_simulation_schedule_parallel();
    // DEBUG: Print number of entities matching agent_movement_system query
    let agent_query_count = <(
        &mut crate::ecs_components::Position,
        &crate::ecs_components::AgentType,
        &mut crate::ecs_components::Hunger,
        &mut crate::ecs_components::Energy,
    )>::query().iter_mut(&mut world).count();
    println!("[DEBUG] Entities matching agent_movement_system query: {}", agent_query_count);
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
    // --- Build ECS systems ONCE for profiling ---
    let mut agent_movement = agent_movement_system();
    let mut entity_interaction = entity_interaction_system();
    let mut agent_death = agent_death_system();
    let mut food_spawn_collect = crate::ecs_components::collect_food_spawn_positions_system();
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
    let mut camera = Camera::new(_map_width, _map_height, CELL_SIZE);
    let mut paused = false;
    let mut advance_one = false;
    let _ascii_snapshots: Vec<String> = Vec::new();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("/System/Library/Fonts/Supplemental/Arial.ttf", 18).unwrap();
    let stats_window_canvas = video_subsystem.window("Stats", 320, 480)
        .position(0, 0)
        .resizable()
        .build().unwrap()
        .into_canvas().build().unwrap();
    let mut stats_canvas = stats_window_canvas;
    let _stats_window_id = stats_canvas.window().id();

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
                    &mut world,
                    &mut resources,
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
                let _ = crate::ecs_simulation::simulation_tick_parallel(&mut world, &mut resources, &mut schedule);
            }
            tick += 1;
            advance_one = false;
        }
        // --- Print latest EventLog entry to console ---
        if let Some(event_log) = resources.get::<crate::ecs_components::EventLog>() {
            if let Some(last_event) = event_log.get().back() {
                println!("{}", last_event);
            }
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
                    keycode: Some(Keycode::Right), .. } => camera.move_by(5.0, 0.0, _map_width, _map_height, CELL_SIZE),
                Event::KeyDown {
                    keycode: Some(Keycode::Left), .. } => camera.move_by(-5.0, 0.0, _map_width, _map_height, CELL_SIZE),
                Event::KeyDown {
                    keycode: Some(Keycode::Up), .. } => camera.move_by(0.0, -5.0, _map_width, _map_height, CELL_SIZE),
                Event::KeyDown {
                    keycode: Some(Keycode::Down), .. } => camera.move_by(0.0, 5.0, _map_width, _map_height, CELL_SIZE),
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
                        if map.tiles[y as usize][x as usize] == super::map::Terrain::Grass || map.tiles[y as usize][x as usize] == super::map::Terrain::Forest {
                            break;
                        }
                        tries += 1;
                        if tries > 1000 {
                            println!("[ERROR] Could not find passable tile for agent after 1000 tries");
                            break;
                        }
                    }
                    // Use the first agent type for simplicity
                    if let Some(agent_type) = ecs_agent_types.get(0) {
                        let agent_type = agent_type.clone();
                        spawn_agent(&mut world, crate::ecs_components::Position { x, y }, agent_type);
                        println!("[DEBUG] Added agent at ({}, {})", x, y);
                    } else {
                        println!("[ERROR] No agent types defined!");
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
                    let num_types = ecs_agent_types.len().max(1);
                    while spawned < max_agents && attempts < max_agents * max_tries_per_agent {
                        let x = rng.gen_range(0.._map_width) as f32;
                        let y = rng.gen_range(0.._map_height) as f32;
                        if map.tiles[y as usize][x as usize] == super::map::Terrain::Grass || map.tiles[y as usize][x as usize] == super::map::Terrain::Forest {
                            let type_idx = rng.gen_range(0..num_types);
                            let agent_type = ecs_agent_types[type_idx].clone();
                            spawn_agent(&mut world, crate::ecs_components::Position { x, y }, agent_type);
                            spawned += 1;
                        }
                        attempts += 1;
                    }
                    println!("[DEBUG] Spawned {} agents ({} attempts)", spawned, attempts);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Period),
                    ..
                } => {
                    // Advance one tick if paused
                    if paused {
                        advance_one = true;
                        println!("[DEBUG] Advance one tick (paused)");
                    }
                },
                Event::MouseButtonDown { x, y, window_id: evt_win_id, .. } => {
                    // DEBUG: Print mouse click info
                    println!("[DEBUG] Mouse click at ({}, {}) in window {}", x, y, evt_win_id);
                    if evt_win_id == window_id {
                        let map_x = (x as f32 / CELL_SIZE as f32 + camera.x).floor();
                        let map_y = (y as f32 / CELL_SIZE as f32 + camera.y).floor();
                        println!("[DEBUG] Map coords: ({}, {})", map_x, map_y);
                        let mut found_agent = None;
                        let mut topmost_y = -1.0_f32;
                        for (entity, (pos, renderable)) in <(legion::Entity, (&crate::ecs_components::Position, &crate::ecs_components::Renderable))>::query().iter(&world) {
                            if renderable.icon != '@' { continue; }
                            let agent_cell_x = pos.x.floor();
                            let agent_cell_y = pos.y.floor();
                            if (agent_cell_x - map_x).abs() < 0.5 && (agent_cell_y - map_y).abs() < 0.5 {
                                // Select the agent with the highest Y (lowest on screen) if multiple overlap
                                if agent_cell_y > topmost_y {
                                    found_agent = Some(*entity);
                                    topmost_y = agent_cell_y;
                                }
                            }
                        }
                        // If no agent found, check for food at the cell
                        if found_agent.is_none() {
                            for (entity, (pos, _food)) in <(legion::Entity, (&crate::ecs_components::Position, &crate::ecs_components::Food))>::query().iter(&world) {
                                let food_cell_x = pos.x.floor();
                                let food_cell_y = pos.y.floor();
                                if (food_cell_x - map_x).abs() < 0.5 && (food_cell_y - map_y).abs() < 0.5 {
                                    found_agent = Some(*entity);
                                    break;
                                }
                            }
                        }
                        selected_agent = found_agent;
                        if let Some(sel) = selected_agent {
                            println!("[DEBUG] Selected entity {:?}", sel);
                        } else {
                            println!("[DEBUG] No agent or food found at clicked cell");
                            // Store the cell and the current time for highlighting
                            empty_cell_flash = Some((map_x as i32, map_y as i32, std::time::Instant::now()));
                        }
                    }
                },
                _ => {}
            }
        }
        // Draw terrain
        println!("[DEBUG] About to render terrain");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for y in 0..render_map.height as usize {
            for x in 0..render_map.width as usize {
                let rect = Rect::new(
                    ((x as f32 - camera.x) * CELL_SIZE as f32) as i32,
                    ((y as f32 - camera.y) * CELL_SIZE as f32) as i32,
                    CELL_SIZE,
                    CELL_SIZE,
                );
                canvas.set_draw_color(terrain_color(&render_map.tiles[y][x]));
                canvas.fill_rect(rect).unwrap();
            }
        }
        // Draw ECS agents
        println!("[DEBUG] About to query agents for rendering");
        for (entity, (pos, renderable)) in <(legion::Entity, (&crate::ecs_components::Position, &crate::ecs_components::Renderable))>::query().iter(&world) {
            let rect = Rect::new(
                ((pos.x - camera.x) * CELL_SIZE as f32) as i32,
                ((pos.y - camera.y) * CELL_SIZE as f32) as i32,
                CELL_SIZE,
                CELL_SIZE,
            );
            // Render food as green squares, agents as their icon/color
            if renderable.icon == '*' {
                canvas.set_draw_color(Color::RGB(0, 220, 0)); // Bright green for food
                canvas.fill_rect(rect).ok();
            } else {
                // Render agents as colored '@' squares (or whatever icon/color they use)
                let color = match renderable.color.as_str() {
                    "blue" => Color::RGB(0, 128, 255),
                    "green" => Color::RGB(0, 200, 0),
                    "red" => Color::RGB(220, 40, 40),
                    "gray" => Color::RGB(120, 120, 120),
                    "yellow" => Color::RGB(255, 255, 0),
                    "brown" => Color::RGB(139, 69, 19),
                    "white" => Color::RGB(255, 255, 255),
                    _ => AGENT_COLOR,
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(rect).ok();
                // Highlight selected agent
                if let Some(sel) = selected_agent {
                    if sel == *entity {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                        let border = Rect::new(rect.x - 2, rect.y - 2, rect.width() + 4, rect.height() + 4);
                        canvas.draw_rect(border).ok();
                    }
                }
            }
        }
        // --- Flash highlight for empty cell click ---
        if let Some((fx, fy, t)) = empty_cell_flash {
            if t.elapsed().as_millis() < 200 {
                let rect = Rect::new(
                    ((fx as f32 - camera.x) * CELL_SIZE as f32) as i32,
                    ((fy as f32 - camera.y) * CELL_SIZE as f32) as i32,
                    CELL_SIZE,
                    CELL_SIZE,
                );
                canvas.set_draw_color(Color::RGB(255, 255, 0)); // Yellow border
                canvas.draw_rect(rect).ok();
            } else {
                empty_cell_flash = None;
            }
        }
        println!("[DEBUG] About to present main canvas");
        canvas.present();

        // --- Stats window rendering ---
        if last_stats_update.elapsed().as_secs_f32() >= 1.0 {
            // Update agent counts for stats display
            use crate::ecs_components::AgentType;
            let agent_counts = {
                let mut agent_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                for (_entity, agent_type) in <(Read<crate::ecs_components::Position>, Read<AgentType>)>::query().iter(&world) {
                    *agent_counts.entry(agent_type.name.clone()).or_insert(0) += 1;
                }
                agent_counts
            };
            let food_count = {
                <(Read<crate::ecs_components::Position>,)>::query()
                    .filter(component::<crate::ecs_components::Food>())
                    .iter(&world)
                    .count()
            };
            let mut counts_vec: Vec<(String, usize)> = agent_counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
            counts_vec.sort_by(|a, b| a.0.cmp(&b.0));
            // Add food and interactions
            let interaction_count = resources.get::<crate::ecs_components::InteractionStats>().map_or(0, |stats| stats.agent_interactions);
            counts_vec.push(("food".to_string(), food_count));
            counts_vec.push(("interactions".to_string(), interaction_count));
            cached_agent_counts = counts_vec;
            last_stats_update = std::time::Instant::now();
        }
        // Query current stats window size in case it was resized
        let (_stats_window_width, _stats_window_height) = stats_canvas.window().size();
        stats_canvas.set_draw_color(Color::RGB(30, 30, 30));
        stats_canvas.clear();
        // Render cached agent type counts as static text
        let mut y_offset = 10;
        for (name, count) in &cached_agent_counts {
            let text = format!("{}: {}", name, count);
            let surface = font.render(&text)
                .blended(Color::RGB(220, 220, 220)).unwrap();
            let texture_creator = stats_canvas.texture_creator();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let target = Rect::new(10, y_offset, width, height);
            stats_canvas.copy(&texture, None, Some(target)).unwrap();
            y_offset += height as i32 + 8;
        }
        // --- Show current active interactions ---
        let active_interactions = resources.get::<crate::ecs_components::InteractionStats>().map_or(0, |stats| stats.active_interactions);
        let text = format!("active interactions: {}", active_interactions);
        let surface = font.render(&text)
            .blended(Color::RGB(120, 200, 255)).unwrap();
        let texture_creator = stats_canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(10, y_offset, width, height);
        stats_canvas.copy(&texture, None, Some(target)).unwrap();
        y_offset += height as i32 + 12;
        // --- Draw active interaction history graph ---
        if let Some(stats) = resources.get::<crate::ecs_components::InteractionStats>() {
            let history = &stats.active_interactions_history;
            if !history.is_empty() {
                // Draw axes
                let graph_left = 10;
                let graph_top = y_offset + 10;
                let graph_width = 280;
                let graph_height = 60;
                stats_canvas.set_draw_color(Color::RGB(80, 80, 80));
                let _ = stats_canvas.draw_rect(Rect::new(graph_left, graph_top, graph_width, graph_height));
                // Find max value for scaling
                let max_val = *history.iter().max().unwrap_or(&1) as f32;
                let min_val = *history.iter().min().unwrap_or(&0) as f32;
                let range = (max_val - min_val).max(1.0);
                let n = history.len().min(graph_width as usize);
                let x_step = graph_width as f32 / (n.max(2) - 1) as f32;
                let mut last_x = graph_left as f32;
                let mut last_y = graph_top as f32 + graph_height as f32 - ((history[0] as f32 - min_val) / range * graph_height as f32);
                stats_canvas.set_draw_color(Color::RGB(120, 200, 255));
                for (i, &val) in history.iter().rev().take(n).collect::<Vec<_>>().into_iter().rev().enumerate() {
                    let x = graph_left as f32 + i as f32 * x_step;
                    let y = graph_top as f32 + graph_height as f32 - ((val as f32 - min_val) / range * graph_height as f32);
                    if i > 0 {
                        let _ = stats_canvas.draw_line((last_x as i32, last_y as i32), (x as i32, y as i32));
                    }
                    last_x = x;
                    last_y = y;
                }
            }
            y_offset += 80;
        }
        // Show selected agent or food stats
        if let Some(sel) = selected_agent {
            // Try to show agent stats first
            let mut shown = false;
            println!("[DEBUG] About to query agent stats");
            for (entity, (pos, agent_type, hunger, energy)) in <(legion::Entity, (&crate::ecs_components::Position, &crate::ecs_components::AgentType, &crate::ecs_components::Hunger, &crate::ecs_components::Energy))>::query().iter(&world) {
                if *entity == sel {
                    let text = format!("Selected Agent:\nPos: ({:.1}, {:.1})\nType: {}\nHunger: {:.1}\nEnergy: {:.1}", pos.x, pos.y, agent_type.name, hunger.value, energy.value);
                    for (i, line) in text.lines().enumerate() {
                        let surface = font.render(line).blended(Color::RGB(255, 200, 50)).unwrap();
                        let texture_creator = stats_canvas.texture_creator();
                        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                        let TextureQuery { width, height, .. } = texture.query();
                        let target = Rect::new(10, y_offset + (i as i32) * (height as i32 + 2), width, height);
                        stats_canvas.copy(&texture, None, Some(target)).unwrap();
                    }
                    shown = true;
                    break;
                }
            }
            // If not an agent, try to show food stats
            if !shown {
                println!("[DEBUG] About to query food stats");
                for (entity, (pos, food)) in <(legion::Entity, (&crate::ecs_components::Position, &crate::ecs_components::Food))>::query().iter(&world) {
                    if *entity == sel {
                        let text = format!("Selected Food:\nPos: ({:.1}, {:.1})\nNutrition: {:.1}", pos.x, pos.y, food.nutrition);
                        for (i, line) in text.lines().enumerate() {
                            let surface = font.render(line).blended(Color::RGB(200, 255, 50)).unwrap();
                            let texture_creator = stats_canvas.texture_creator();
                            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                            let TextureQuery { width, height, .. } = texture.query();
                            let target = Rect::new(10, y_offset + (i as i32) * (height as i32 + 2), width, height);
                            stats_canvas.copy(&texture, None, Some(target)).unwrap();
                        }
                        break;
                    }
                }
            }
        }
        println!("[DEBUG] About to present stats canvas");
        stats_canvas.present();
        ::std::thread::sleep(Duration::from_millis(16)); // ~60 FPS for main window
    }
    // --- At end of simulation, write summary to simulation_ascii.txt ---
    // (This block is adapted from simulation.rs headless mode)
    use legion::IntoQuery;
    use crate::ecs_components::InteractionStats;
    use std::collections::HashMap;
    // Count agent types at end
    let mut agent_type_counts: HashMap<String, usize> = HashMap::new();
    let mut agent_query = <(&crate::ecs_components::AgentType,)>::query();
    for (agent_type,) in agent_query.iter(&world) {
        *agent_type_counts.entry(agent_type.name.clone()).or_insert(0) += 1;
    }
    // Get interaction stats
    let stats = resources.get::<InteractionStats>().expect("No InteractionStats resource");
    let total_interactions = stats.agent_interactions;
    let avg_interactions_per_tick = if tick > 0 { total_interactions as f64 / tick as f64 } else { 0.0 };
    // Prepare summary string
    let mut summary = String::new();
    summary.push_str(&format!("# Simulation Summary\n"));
    summary.push_str(&format!("Total interactions: {}\n", total_interactions));
    summary.push_str(&format!("Average interactions per tick: {:.2}\n", avg_interactions_per_tick));
    summary.push_str("Agent counts at end:\n");
    for (name, count) in agent_type_counts.iter() {
        summary.push_str(&format!("  {}: {}\n", name, count));
    }
    summary.push_str("\n");
    // Optionally render ASCII snapshot of the map
    let ascii_snapshot = crate::ecs_simulation::render_simulation_ascii(&world, &render_map);
    let mut file = std::fs::File::create("simulation_ascii.txt").expect("Unable to create ascii output file");
    file.write_all(summary.as_bytes()).expect("Unable to write summary");
    file.write_all(ascii_snapshot.as_bytes()).expect("Unable to write ascii output");
    println!("[INFO] Simulation summary and final ASCII snapshot written to simulation_ascii.txt");
}
