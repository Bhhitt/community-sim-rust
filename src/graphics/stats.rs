// Modular stats rendering for overlays
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use legion::*;
use crate::ecs_components::FoodStats;
use crate::food::Food;
use crate::agent::AgentType;

pub fn draw_stats(
    canvas: &mut Canvas<Window>,
    font: &Font,
    world: &World,
    food_stats: Option<&mut FoodStats>,
    log_stats: bool,
) {
    let texture_creator = canvas.texture_creator();
    let (win_w, win_h) = canvas.window().size();
    if log_stats {
        log::info!("[STATS][DRAW] stats window size: {}x{} (id: {})", win_w, win_h, canvas.window().id());
    }
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();
    let mut y = 10;
    let line_height = 22;

    // --- Agent stats ---
    let agent_count = <(&crate::ecs_components::Position, Option<&AgentType>)>::query()
        .iter(world)
        .filter(|(_, agent_type)| agent_type.is_some())
        .count();
    let text = format!("Agents: {}", agent_count);
    let surface = font.render(&text).blended(Color::RGB(200, 200, 255)).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let sdl2::render::TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(10, y, width, height);
    canvas.copy(&texture, None, Some(target)).unwrap();
    y += line_height;

    // --- Food stats ---
    let food_count = <(&crate::ecs_components::Position, &Food)>::query().iter(world).count();
    let text = format!("Food: {}", food_count);
    let surface = font.render(&text).blended(Color::RGB(180, 255, 180)).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let sdl2::render::TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(10, y, width, height);
    canvas.copy(&texture, None, Some(target)).unwrap();
    y += line_height;

    // --- Food spawned/collected per tick ---
    if let Some(stats) = food_stats {
        let spawned = stats.spawned_per_tick;
        let collected = stats.collected_per_tick;
        let text = format!("Food spawned/tick: {}", spawned);
        let surface = font.render(&text).blended(Color::RGB(255, 220, 100)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(10, y, width, height);
        canvas.copy(&texture, None, Some(target)).unwrap();
        let _ = y + line_height;
        let text = format!("Food collected/tick: {}", collected);
        let surface = font.render(&text).blended(Color::RGB(255, 180, 100)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(10, y, width, height);
        canvas.copy(&texture, None, Some(target)).unwrap();
        let _ = y + line_height;
        // Reset stats after display so they show per-tick values
        stats.spawned_per_tick = 0;
        stats.collected_per_tick = 0;
    }
    // ... (more stats as needed)
    canvas.present();
}
