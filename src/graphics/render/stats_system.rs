use legion::World;
use crate::graphics::sim_state::CachedStats;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;

/// Stats window rendering function
pub fn stats_window_render(
    world: &World,
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
        &legion::Resources::default(),
        log_stats,
    );
}

// Removed never-used function: stats_window_render_old
