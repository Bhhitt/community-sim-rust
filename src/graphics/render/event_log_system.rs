// use legion::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;
use crate::event_log::EventLog;

/// ECS-style event log window rendering as a plain function
pub fn event_log_window_render(
    event_log: &EventLog,
    canvas: &mut Canvas<Window>,
    font: &Font,
    log_window_enabled: bool,
) {
    // For now, call the original draw_event_log_window logic, but this will be refactored
    crate::graphics::render::overlays::draw_event_log_window(
        canvas,
        font,
        event_log,
        log_window_enabled,
    );
}
