use legion::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;
use crate::ecs_components::InteractionStats;

/// ECS-style stats window rendering as a plain function
pub fn stats_window_render(
    world: &World,
    canvas: &mut Canvas<Window>,
    font: &Font,
    cached_agent_counts: &[(String, usize)],
    interaction_stats: Option<&InteractionStats>,
    selected_agent: Option<legion::Entity>,
    log_stats: bool,
) {
    // For now, call the original draw_stats_window logic, but this will be refactored
    crate::graphics::render::overlays::draw_stats_window(
        canvas,
        font,
        cached_agent_counts,
        interaction_stats,
        selected_agent,
        world,
        log_stats,
    );
}
