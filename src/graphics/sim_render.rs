// DEPRECATED: This module's simulation entrypoint is superseded by the unified simulation loop and setup in main.rs and sim_loop_unified.rs.
// All graphics simulation logic should be routed through the unified setup and loop.
// This file is retained for reference and will be removed after migration is complete.

// Unified Event Log Access Pattern: ALWAYS access EventLog via ECS resources as Arc<Mutex<EventLog>>.
// Do not pass EventLog as a direct argument. This ensures a single, obvious pattern for all systems.

// Main simulation rendering and event loop
// Will contain the main SDL2 rendering logic and event loop

use crate::log_config::LogConfig;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::EventPump;
use crate::graphics::camera::Camera;

const CELL_SIZE: f32 = 6.0;

// --- Make SDL2 plug-in types public for unified simulation entry ---
pub struct SdlRenderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub stats_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub log_canvas_opt: Option<sdl2::render::Canvas<sdl2::video::Window>>,
}

impl crate::sim_loop_unified::SimulationRenderer for SdlRenderer {
    fn render_ui(&mut self, sim_ui_state: &mut crate::graphics::sim_state::SimUIState, tick: usize) {
        // Clear the main window BEFORE drawing terrain
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 20, 20));
        self.canvas.clear();

        // --- Terrain rendering ---
        crate::graphics::render::terrain::draw_terrain(
            &mut self.canvas,
            &*sim_ui_state.resources.get::<crate::map::Map>().expect("Map missing in resources for terrain rendering"),
            sim_ui_state.camera.x,
            sim_ui_state.camera.y,
            CELL_SIZE,
        );

        // Draw food
        crate::graphics::render::food_system::food_render_system(
            sim_ui_state.world,
            &mut self.canvas,
            sim_ui_state.camera.x,
            sim_ui_state.camera.y,
            CELL_SIZE,
            sim_ui_state.selected_agent,
        );

        // Draw agents
        crate::graphics::render::agent_system::agent_render_system(
            sim_ui_state.world,
            &mut self.canvas,
            sim_ui_state.camera.x,
            sim_ui_state.camera.y,
            CELL_SIZE,
        );

        // Draw selected agent path
        crate::graphics::render::selected_agent_path_system::selected_agent_path_render(
            sim_ui_state.world,
            &mut self.canvas,
            sim_ui_state.selected_agent,
            sim_ui_state.camera.x,
            sim_ui_state.camera.y,
            CELL_SIZE,
        );

        // Present the main window
        self.canvas.present();

        // Update cached stats before rendering overlays
        crate::graphics::sim_state::update_cached_stats(
            sim_ui_state.world,
            sim_ui_state.resources,
            &mut sim_ui_state.cached_stats,
        );

        // Render overlays (stats, event log) to their canvases if those windows are enabled
        if let Some(ref mut log_canvas) = self.log_canvas_opt {
            crate::graphics::render::event_log_system::event_log_window_render(
                sim_ui_state.world,
                sim_ui_state.resources,
                log_canvas,
                &sim_ui_state.font,
                true, // or toggle based on UI state
            );
            log_canvas.present();
        }
        crate::graphics::render::stats_system::stats_window_render(
            sim_ui_state.world,
            &mut self.stats_canvas,
            &sim_ui_state.font,
            &sim_ui_state.cached_stats,
            sim_ui_state.selected_agent,
            false, // or toggle based on UI state
        );
        self.stats_canvas.present();
    }
}

pub struct SdlInput {
    pub event_pump: sdl2::EventPump,
    pub window_id: u32,
}

impl crate::sim_loop_unified::SimulationInput for SdlInput {
    fn handle_input_ui(&mut self,
        sim_ui_state: &mut crate::graphics::sim_state::SimUIState,
        agent_types: &[crate::agent::AgentType],
        render_map: &crate::map::Map,
        cell_size: f32,
        log_config: &LogConfig,
        paused: &mut bool,
        tick: usize,
    ) {
        crate::graphics::input::collect_input_events(
            &mut self.event_pump,
            self.window_id,
            sim_ui_state,
            agent_types,
            render_map,
            cell_size,
            log_config,
            *paused,
        );
    }
}

pub struct SdlProfiler;

impl crate::sim_loop_unified::SimulationProfiler for SdlProfiler {
    fn on_simulation_end_ui(&mut self, sim_ui_state: &crate::graphics::sim_state::SimUIState, ticks: usize) {
        // TODO: profiling/summary logic for graphics mode
    }
}

// Add SDL2 initialization function for graphics mode

/// Initializes SDL2, creates windows and canvases, event pump, camera, and font
pub fn init_sdl2(
    map_width: i32,
    map_height: i32,
    cell_size: f32,
    window_width: u32,
    window_height: u32,
    log_config: &LogConfig,
) -> (
    Canvas<Window>,
    Canvas<Window>,
    Option<Canvas<Window>>,
    EventPump,
    u32,
    u32,
    u32,
    Camera,
    &'static Font<'static, 'static>,
) {
    let sdl_context = sdl2::init().expect("Failed to init SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to get SDL2 video subsystem");
    // Leak the ttf_context and font for 'static lifetime
    let ttf_context = Box::leak(Box::new(sdl2::ttf::init().expect("Failed to init TTF")));
    // Use a .ttc font file that exists on macOS
    let font_path = "/System/Library/Fonts/Helvetica.ttc";
    let font = Box::leak(Box::new(
        ttf_context.load_font(font_path, 16).expect("Failed to load font")
    ));

    // Main window
    let window = video_subsystem
        .window("CommunitySim", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create SDL2 window");
    let mut canvas = window.into_canvas().accelerated().present_vsync().build().expect("Failed to create SDL2 canvas");
    let window_id = canvas.window().id();

    // Make the stats window resizable and set initial size to 800 px tall
    let stats_window = video_subsystem
        .window("Stats", 320, 800)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .expect("Failed to create stats window");
    let stats_canvas = stats_window.into_canvas().accelerated().present_vsync().build().expect("Failed to create stats canvas");
    let stats_window_id = stats_canvas.window().id();

    // Log window (optional)
    let log_window_enabled = false;
    let log_canvas_opt = if log_window_enabled {
        Some(
            video_subsystem
                .window("Event Log", 640, 480)
                .position_centered()
                .build()
                .expect("Failed to create log window")
                .into_canvas()
                .build()
                .expect("Failed to create log canvas"),
        )
    } else {
        None
    };
    let log_window_id = log_canvas_opt.as_ref().map(|c| c.window().id()).unwrap_or(0);

    let event_pump = sdl_context.event_pump().expect("Failed to get SDL2 event pump");
    let camera = Camera::new(map_width, map_height, window_width, window_height);

    // Return the leaked font reference as &'static Font<'static, 'static>
    (
        canvas,
        stats_canvas,
        log_canvas_opt,
        event_pump,
        map_width as u32,
        map_height as u32,
        cell_size as u32,
        camera,
        font,
    )
}
