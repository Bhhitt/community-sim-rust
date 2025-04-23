use legion::*;
use crate::ecs_components::Position;
use crate::food::Food;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// ECS-style food rendering as a plain function
pub fn food_render(world: &World, canvas: &mut Canvas<Window>, camera_x: f32, camera_y: f32, cell_size: f32, selected_entity: Option<legion::Entity>) {
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
        if Some(entity) == selected_entity.as_ref() {
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
