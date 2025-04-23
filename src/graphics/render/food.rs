// Food rendering logic will be moved here from sim_render.rs

// Food rendering logic for the simulation
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use legion::*;
use crate::ecs_components::{Position};
use crate::food::Food;

/// Draws all food entities on the map as green squares with a yellow border
pub fn draw_food(
    canvas: &mut Canvas<Window>,
    world: &World,
    camera_x: f32,
    camera_y: f32,
    cell_size: f32,
    selected_entity: Option<legion::Entity>,
) {
    for (entity, (pos, _food)) in <(Entity, (&Position, &Food))>::query().iter(world) {
        let rect = Rect::new(
            ((pos.x - camera_x) * cell_size) as i32,
            ((pos.y - camera_y) * cell_size) as i32,
            cell_size as u32,
            cell_size as u32,
        );
        // Fill with green
        canvas.set_draw_color(Color::RGB(0, 220, 0));
        let _ = canvas.fill_rect(rect);
        // Draw yellow border
        canvas.set_draw_color(Color::RGB(255, 220, 40));
        let _ = canvas.draw_rect(rect);
        // If selected, draw a highlight (e.g., red border)
        if Some(*entity) == selected_entity {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            let highlight_rect = Rect::new(
                rect.x - 2,
                rect.y - 2,
                rect.width() + 4,
                rect.height() + 4,
            );
            let _ = canvas.draw_rect(highlight_rect);
        }
    }
}
