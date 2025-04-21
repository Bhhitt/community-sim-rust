//! SDL2 graphics frontend

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::TextureQuery;
use rand::Rng;

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
    pub fn new(_map_width: i32, _map_height: i32, _cell_size: u32) -> Self {
        let width = WINDOW_WIDTH / CELL_SIZE;
        let height = WINDOW_HEIGHT / CELL_SIZE;
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

pub fn run_with_graphics_profile(map_size: i32, _num_agents: usize, agent_types: &[crate::agent::AgentType]) {
    use crate::ecs_components::{spawn_agent, AgentType as ECSAgentType, agent_movement_system, entity_interaction_system, collect_food_spawn_positions, food_spawn_apply_system, agent_death_system, PendingFoodSpawns};
    use legion::*;
    use std::io::Write;
    // --- ECS World Setup ---
    let mut world = World::default();
    let map = super::map::Map::new(map_size, map_size);
    let render_map = map.clone();
    let mut rng = rand::thread_rng();
    // Convert agent_types from agent.rs to ECSAgentType
    let ecs_agent_types: Vec<ECSAgentType> = agent_types.iter().map(|a| ECSAgentType {
        name: Box::leak(a.r#type.clone().into_boxed_str()),
        move_speed: a.move_speed,
        move_probability: a.move_probability,
        color: Box::leak(a.color.clone().into_boxed_str()),
    }).collect();
    println!("[DEBUG] Loaded {} agent types", ecs_agent_types.len());
    // Restore agent count for normal simulation
    let num_agents = 1000;
    // Only spawn agents if num_agents > 0
    let mut agent_count = 0;
    let mut attempts = 0;
    if num_agents > 0 {
        for i in 0..num_agents {
            // Find a random passable tile
            let mut x;
            let mut y;
            let mut tries = 0;
            loop {
                x = rng.gen_range(0..map_size) as f32;
                y = rng.gen_range(0..map_size) as f32;
                if map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                    break;
                }
                tries += 1;
                if tries > 1000 {
                    panic!("Could not find passable tile for agent after 1000 tries");
                }
            }
            let agent_type = ecs_agent_types[i % ecs_agent_types.len()].clone();
            spawn_agent(&mut world, crate::ecs_components::Position { x, y }, agent_type);
            agent_count += 1;
            attempts += tries;
        }
    }
    println!("[DEBUG] Total spawn attempts: {} (avg {:.2} per agent)", attempts, attempts as f32 / agent_count as f32);
    // Print total entity count in the world after spawning
    let total_entities = world.len();
    println!("[DEBUG] Total entities in world after spawning: {}", total_entities);
    std::io::stdout().flush().unwrap();
    println!("[DEBUG] Spawned {} agents", agent_count);
    let mut camera = Camera::new(render_map.width, render_map.height, CELL_SIZE);

    // --- ECS Resources & Schedule ---
    let mut resources = Resources::default();
    resources.insert(map);
    resources.insert(crate::ecs_components::InteractionStats::default());
    resources.insert(crate::ecs_components::EventLog::new(200));
    resources.insert(PendingFoodSpawns(Vec::new()));
    let mut pre_food_schedule = Schedule::builder()
        .add_system(agent_movement_system())
        .add_system(entity_interaction_system())
        .flush()
        .build();
    // SPLIT: Separate schedules for agent_death and food_spawn
    let mut agent_death_schedule = Schedule::builder()
        .add_system(agent_death_system())
        .build();
    let mut food_spawn_apply_schedule = Schedule::builder()
        .add_system(food_spawn_apply_system())
        .build();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Community Sim", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered().build().unwrap();
    let main_window_id = window.id();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    // --- Stats window setup ---
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("/System/Library/Fonts/Supplemental/Arial.ttf", 18).unwrap();
    let stats_window = video_subsystem.window("Stats", 320, 480)
        .position(0, 0) // Spawn at top-left corner
        .resizable()
        .build().unwrap();
    let mut stats_canvas = stats_window.into_canvas().present_vsync().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_stats_update = std::time::Instant::now();
    let mut cached_agent_counts: Vec<(String, usize)> = Vec::new();
    let mut paused = false;
    let mut advance_one = false;
    let mut selected_agent: Option<legion::Entity> = None;
    let mut empty_cell_flash: Option<(i32, i32, std::time::Instant)> = None;
    'running: loop {
        // --- Run ECS systems ---
        if !paused || advance_one {
            println!("[DEBUG] About to run pre_food_schedule");
            pre_food_schedule.execute(&mut world, &mut resources);
            println!("[DEBUG] Finished pre_food_schedule");
            println!("[DEBUG] About to run agent_death_schedule");
            agent_death_schedule.execute(&mut world, &mut resources);
            println!("[DEBUG] Finished agent_death_schedule");
            // Collect food spawn positions outside ECS schedule
            {
                let map = resources.get::<crate::map::Map>().unwrap();
                let positions = collect_food_spawn_positions(&world, &map);
                resources.get_mut::<PendingFoodSpawns>().unwrap().0 = positions;
            }
            println!("[DEBUG] About to run food_spawn_apply_schedule");
            food_spawn_apply_schedule.execute(&mut world, &mut resources);
            println!("[DEBUG] Finished food_spawn_apply_schedule");
            if advance_one {
                advance_one = false;
            }
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
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Left => camera.move_by(-10.0, 0.0, render_map.width, render_map.height, CELL_SIZE),
                        Keycode::Right => camera.move_by(10.0, 0.0, render_map.width, render_map.height, CELL_SIZE),
                        Keycode::Up => camera.move_by(0.0, -10.0, render_map.width, render_map.height, CELL_SIZE),
                        Keycode::Down => camera.move_by(0.0, 10.0, render_map.width, render_map.height, CELL_SIZE),
                        Keycode::Space => paused = !paused,
                        Keycode::Period => advance_one = true,
                        Keycode::A => {
                            // Spawn a new agent at a random passable tile
                            let mut rng = rand::thread_rng();
                            let mut x;
                            let mut y;
                            let mut tries = 0;
                            loop {
                                x = rng.gen_range(0..render_map.width) as f32;
                                y = rng.gen_range(0..render_map.height) as f32;
                                if render_map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || render_map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                                    break;
                                }
                                tries += 1;
                                if tries > 1000 {
                                    println!("[WARN] Could not find passable tile for agent after 1000 tries");
                                    return;
                                }
                            }
                            // Pick a random agent type
                            if !ecs_agent_types.is_empty() {
                                let idx = rng.gen_range(0..ecs_agent_types.len());
                                let agent_type = ecs_agent_types[idx].clone();
                                spawn_agent(&mut world, crate::ecs_components::Position { x, y }, agent_type);
                                println!("[DEBUG] Spawned agent at ({}, {})", x, y);
                            }
                        },
                        Keycode::S => {
                            // Spawn 100 new agents at random passable tiles
                            let mut rng = rand::thread_rng();
                            for _ in 0..100 {
                                let mut x;
                                let mut y;
                                let mut tries = 0;
                                loop {
                                    x = rng.gen_range(0..render_map.width) as f32;
                                    y = rng.gen_range(0..render_map.height) as f32;
                                    if render_map.tiles[y as usize][x as usize] == crate::map::Terrain::Grass || render_map.tiles[y as usize][x as usize] == crate::map::Terrain::Forest {
                                        break;
                                    }
                                    tries += 1;
                                    if tries > 1000 {
                                        println!("[WARN] Could not find passable tile for agent after 1000 tries");
                                        break;
                                    }
                                }
                                if !ecs_agent_types.is_empty() {
                                    let idx = rng.gen_range(0..ecs_agent_types.len());
                                    let agent_type = ecs_agent_types[idx].clone();
                                    spawn_agent(&mut world, crate::ecs_components::Position { x, y }, agent_type);
                                }
                            }
                            println!("[DEBUG] Spawned 100 random agents");
                        },
                        _ => {}
                    }
                }
                Event::MouseButtonDown { x, y, window_id, .. } => {
                    // DEBUG: Print mouse click info
                    println!("[DEBUG] Mouse click at ({}, {}) in window {}", x, y, window_id);
                    if window_id == main_window_id {
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
                }
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
                let color = match renderable.color {
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
            println!("[DEBUG] About to count agent types for stats");
            use crate::ecs_components::AgentType as ECSAgentType;
            let mut agent_counts: std::collections::HashMap<&'static str, usize> = std::collections::HashMap::new();
            println!("[DEBUG] About to query agent types for stats");
            for (_entity, agent_type) in <(Read<crate::ecs_components::Position>, Read<ECSAgentType>)>::query().iter(&world) {
                *agent_counts.entry(agent_type.name).or_insert(0) += 1;
            }
            // Count food entities
            println!("[DEBUG] About to query food entities for stats");
            let food_count = <(Read<crate::ecs_components::Position>,)>::query()
                .filter(component::<crate::ecs_components::Food>())
                .iter(&world)
                .count();
            // Store as sorted Vec for static rendering
            let mut counts_vec: Vec<(String, usize)> = agent_counts.iter().map(|(k, v)| ((*k).to_string(), *v)).collect();
            counts_vec.sort_by(|a, b| a.0.cmp(&b.0));
            cached_agent_counts = counts_vec;
            // Fetch interaction count
            let interaction_count = resources.get::<crate::ecs_components::InteractionStats>().map_or(0, |stats| stats.agent_interactions);
            cached_agent_counts.push(("food".to_string(), food_count));
            cached_agent_counts.push(("interactions".to_string(), interaction_count));
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
}
