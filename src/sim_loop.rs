//! Unified simulation loop traits for rendering, profiling, and input handling.

use legion::{World, Resources, Schedule};

/// Trait for rendering simulation state (ASCII, GUI, etc).
pub trait SimulationRenderer {
    fn render(&mut self, world: &World, resources: &Resources, tick: usize);
}

/// Trait for profiling/logging simulation ticks.
pub trait SimulationProfiler {
    fn on_tick_start(&mut self, tick: usize) {}
    fn on_tick_end(&mut self, tick: usize) {}
    fn on_simulation_end(&mut self, world: &World, resources: &Resources, tick: usize) {}
}

/// Trait for handling simulation input (GUI or headless).
pub trait SimulationInput {
    fn handle_input(&mut self, world: &mut World, resources: &mut Resources, tick: usize);
}

/// Minimal statistics returned by the simulation loop.
pub struct SimulationStats {
    pub ticks_run: usize,
    // Add more fields as needed (e.g., agent count, food count, etc.)
}

/// ASCII renderer for headless mode: prints map every 50 ticks.
pub struct AsciiRenderer;
impl SimulationRenderer for AsciiRenderer {
    fn render(&mut self, world: &World, resources: &Resources, tick: usize) {
        if tick % 50 == 0 {
            // Use the render_ascii module to print the simulation state as ASCII
            use crate::render_ascii::render_simulation_ascii;
            use crate::map::Map;
            // Try to get the map from resources
            if let Some(map) = resources.get::<crate::map::Map>() {
                let ascii = render_simulation_ascii(world, &*map);
                println!("[ASCII] Tick {}:\n{}", tick, ascii);
            } else {
                println!("[ASCII] Tick {}: (map missing, cannot render)", tick);
            }
        }
    }
}

/// No-op renderer for headless mode.
pub struct NoOpRenderer;
impl SimulationRenderer for NoOpRenderer {
    fn render(&mut self, _world: &World, _resources: &Resources, _tick: usize) {}
}

/// No-op input handler for headless mode.
pub struct NoOpInput;
impl SimulationInput for NoOpInput {
    fn handle_input(&mut self, _world: &mut World, _resources: &mut Resources, _tick: usize) {}
}

/// No-op profiler for headless mode.
pub struct NoOpProfiler;
impl SimulationProfiler for NoOpProfiler {}

/// Unified simulation loop for both headless and graphics modes.
pub fn run_simulation_loop<R, P, I>(
    world: &mut World,
    resources: &mut Resources,
    schedule: &mut Schedule,
    ticks: usize,
    renderer: &mut R,
    profiler: &mut P,
    input: &mut I,
) -> SimulationStats
where
    R: SimulationRenderer,
    P: SimulationProfiler,
    I: SimulationInput,
{
    for tick in 0..ticks {
        profiler.on_tick_start(tick);
        schedule.execute(world, resources);
        renderer.render(world, resources, tick);
        input.handle_input(world, resources, tick);
        profiler.on_tick_end(tick);
    }
    profiler.on_simulation_end(world, resources, ticks);
    SimulationStats { ticks_run: ticks }
}
