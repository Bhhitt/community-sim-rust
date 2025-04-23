// Camera module for graphics

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(_map_width: i32, _map_height: i32, cell_size: u32, window_width: u32, window_height: u32) -> Self {
        let width = window_width / cell_size;
        let height = window_height / cell_size;
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
