// use legion::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;
use std::sync::{Arc, Mutex};
use crate::event_log::EventLog;
use legion::World;
use legion::Resources;

/// Event log window rendering function
pub fn event_log_window_render(world: &World, resources: &Resources, canvas: &mut Canvas<Window>, font: &Font, log_window_enabled: bool) {
    // use legion::world::SubWorld;
    // use legion::systems::ResourceSet;
    // use crate::event_log::EventLog;
    // use std::sync::{Arc, Mutex};
    let _ = world;
    if let Some(event_log) = resources.get::<Arc<Mutex<EventLog>>>() {
        crate::graphics::render::overlays::draw_event_log_window(
            canvas,
            font,
            &*event_log,
            log_window_enabled,
        );
    } else {
        // Optionally, clear or display error if EventLog missing
        canvas.set_draw_color(sdl2::pixels::Color::RGB(50, 0, 0));
        canvas.clear();
    }
}
