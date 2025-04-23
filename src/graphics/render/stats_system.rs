use legion::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;
use crate::graphics::sim_state::CachedStats;

/// ECS-style stats window rendering as a plain function
pub fn stats_window_render(
    world: &World,
    resources: &Resources,
    canvas: &mut Canvas<Window>,
    font: &Font,
    cached_stats: &CachedStats,
    selected_agent: Option<legion::Entity>,
    log_stats: bool,
) {
    crate::graphics::render::overlays::draw_stats_window(
        canvas,
        font,
        cached_stats,
        selected_agent,
        world,
        resources,
        log_stats,
    );
}
