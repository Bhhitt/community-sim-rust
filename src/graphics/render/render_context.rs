// Resource to hold rendering context for ECS rendering systems
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct RenderContext<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub camera_x: f32,
    pub camera_y: f32,
    pub cell_size: f32,
}
