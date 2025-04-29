// Terrain rendering logic will be moved here from sim_render.rs

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::graphics::terrain::terrain_color;

pub fn draw_terrain(canvas: &mut Canvas<Window>, render_map: &crate::map::Map, camera_x: f32, camera_y: f32, cell_size: f32) {
    // Fill the entire window with black before drawing terrain
    // let (win_w, win_h) = canvas.window().size();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    // canvas.clear();
    for y in 0..render_map.height as usize {
        for x in 0..render_map.width as usize {
            let rect = Rect::new(
                ((x as f32 - camera_x) * cell_size) as i32,
                ((y as f32 - camera_y) * cell_size) as i32,
                cell_size as u32,
                cell_size as u32,
            );
            canvas.set_draw_color(terrain_color(&render_map.tiles[y][x]));
            canvas.fill_rect(rect).unwrap();
        }
    }
}
