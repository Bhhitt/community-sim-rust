// use legion::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;
use crate::event_log::EventLog;
use legion::World;
use std::sync::{Arc, Mutex};

/// Event log window rendering function
pub fn event_log_window_render(_world: &World, canvas: &mut Canvas<Window>, font: &Font, event_log: &Arc<Mutex<EventLog>>, log_window_enabled: bool) {
    crate::graphics::render::overlays::draw_event_log_window(
        canvas,
        font,
        event_log,
        log_window_enabled,
    );
}
