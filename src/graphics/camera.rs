// Camera module for graphics

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(_map_width: i32, _map_height: i32, // cell_size: u32
               window_width: u32, window_height: u32) -> Self {
        let width = window_width / 1; // cell_size;
        let height = window_height / 1; // cell_size;
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }
    pub fn move_by(&mut self, dx: f32, dy: f32, map_width: i32, map_height: i32, // cell_size: u32
                   ) {
        // Clamp camera so you can't pan past the map border
        let viewport_w = self.width as f32;
        let viewport_h = self.height as f32;
        let min_x = 0.0;
        let min_y = 0.0;
        let max_x = (map_width as f32 - viewport_w).max(min_x);
        let max_y = (map_height as f32 - viewport_h).max(min_y);
        self.x = (self.x + dx).clamp(min_x, max_x);
        self.y = (self.y + dy).clamp(min_y, max_y);
    }
}
