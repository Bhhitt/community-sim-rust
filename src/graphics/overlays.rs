// Handles all overlay and stats rendering
// Overlay/UI rendering logic migrated from render/overlays.rs

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use crate::event_log::EventLog;

pub fn draw_event_log_window(canvas: &mut Canvas<Window>, font: &Font, event_log: &EventLog, log_window_enabled: bool) {
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();
    let texture_creator = canvas.texture_creator();
    if log_window_enabled {
        let mut y = 10;
        let line_height = 20;
        let max_lines = 22;
        let events_vec = &event_log.events;
        let events: Vec<_> = events_vec.iter().rev().take(max_lines).collect();
        for entry in events.iter().rev() {
            let surface = font.render(entry)
                .blended(Color::RGB(220, 220, 220)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let sdl2::render::TextureQuery { width, height, .. } = texture.query();
            let target = Rect::new(10, y, width, height);
            canvas.copy(&texture, None, Some(target)).unwrap();
            y += line_height;
        }
    } else {
        let text = "Quiet mode";
        let surface = font.render(text)
            .blended(Color::RGB(180, 180, 180)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
        let (win_w, win_h) = canvas.window().size();
        let target = Rect::new(
            (win_w as i32 - width as i32) / 2,
            (win_h as i32 - height as i32) / 2,
            width,
            height,
        );
        canvas.copy(&texture, None, Some(target)).unwrap();
    }
    canvas.present();
}

pub fn draw_empty_cell_flash(canvas: &mut Canvas<Window>, fx: i32, fy: i32, camera_x: f32, camera_y: f32, cell_size: f32) {
    let rect = Rect::new(
        ((fx as f32 - camera_x) * cell_size) as i32,
        ((fy as f32 - camera_y) * cell_size) as i32,
        cell_size as u32,
        cell_size as u32,
    );
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.draw_rect(rect).ok();
}

// All unused imports removed for a clean build
