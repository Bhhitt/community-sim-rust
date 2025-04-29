//! Unified simulation loop interface for both graphics and headless (ASCII) modes.
//! This file defines a single simulation entry point and traits for rendering, input, and profiling.
//! Both modes should use this interface to maximize code sharing and minimize divergence.

use legion::{World, Resources, Schedule};
use crate::graphics::sim_state::SimUIState;
use crate::sim_state::SimState;

/// Trait for rendering the simulation (ASCII, SDL2, etc.)
pub trait SimulationRenderer {
    fn render(&mut self, sim_state: &mut SimState, tick: usize) { let _ = (sim_state, tick); }
    fn render_ui(&mut self, _sim_ui_state: &mut SimUIState, _tick: usize) {}
}

/// Trait for handling simulation input (GUI, headless, etc.)
pub trait SimulationInput {
    fn handle_input(&mut self, sim_state: &mut SimState, tick: usize) { let _ = (sim_state, tick); }
    fn handle_input_ui(&mut self, _sim_ui_state: &mut SimUIState, _agent_types: &[crate::agent::AgentType], _render_map: &crate::map::Map, _cell_size: f32, _log_config: &crate::log_config::LogConfig, _paused: &mut bool, _tick: usize) {}
}

/// Trait for profiling the simulation (no-op, CSV, etc.)
pub trait SimulationProfiler {
    fn on_tick_start(&mut self, tick: usize) {}
    fn on_tick_end(&mut self, tick: usize) {}
    fn on_simulation_end(&mut self, _sim_state: &SimState, _ticks: usize) {}
    fn on_simulation_end_ui(&mut self, _sim_ui_state: &SimUIState, _ticks: usize) {}
}

/// Unified simulation loop for both graphics and headless modes.
pub fn run_simulation_loop<R, P, I>(
    sim_state: &mut SimState,
    ticks: usize,
    renderer: &mut R,
    profiler: &mut P,
    input: &mut I,
) where
    R: SimulationRenderer,
    P: SimulationProfiler,
    I: SimulationInput,
{
    if ticks == 0 {
        let mut tick = 0;
        loop {
            profiler.on_tick_start(tick);
            sim_state.schedule.execute(&mut sim_state.world, &mut sim_state.resources);
            renderer.render(sim_state, tick);
            input.handle_input(sim_state, tick);
            profiler.on_tick_end(tick);
            tick += 1;
        }
    } else {
        for tick in 0..ticks {
            profiler.on_tick_start(tick);
            sim_state.schedule.execute(&mut sim_state.world, &mut sim_state.resources);
            renderer.render(sim_state, tick);
            input.handle_input(sim_state, tick);
            profiler.on_tick_end(tick);
        }
        profiler.on_simulation_end(sim_state, ticks);
    }
}

/// Unified simulation loop for graphics mode.
pub fn run_simulation_loop_ui<R, P, I>(
    sim_ui_state: &mut SimUIState,
    ticks: usize,
    renderer: &mut R,
    profiler: &mut P,
    input: &mut I,
    agent_types: &[crate::agent::AgentType],
    render_map: &crate::map::Map,
    cell_size: f32,
    log_config: &crate::log_config::LogConfig,
    mut paused: bool,
) where
    R: SimulationRenderer,
    P: SimulationProfiler,
    I: SimulationInput,
{
    if ticks == 0 {
        let mut tick = 0;
        let mut advance_one = false;
        loop {
            input.handle_input_ui(sim_ui_state, agent_types, render_map, cell_size, log_config, &mut paused, tick);
            crate::graphics::input_systems::process_input_intents(
                sim_ui_state,
                agent_types,
                render_map,
                cell_size,
                &mut paused,
                &mut advance_one,
            );
            profiler.on_tick_start(tick);
            if !paused || advance_one {
                sim_ui_state.schedule.execute(&mut sim_ui_state.world, &mut sim_ui_state.resources);
                advance_one = false;
            }
            renderer.render_ui(sim_ui_state, tick);
            profiler.on_tick_end(tick);
            tick += 1;
        }
    } else {
        let mut advance_one = false;
        for tick in 0..ticks {
            input.handle_input_ui(sim_ui_state, agent_types, render_map, cell_size, log_config, &mut paused, tick);
            crate::graphics::input_systems::process_input_intents(
                sim_ui_state,
                agent_types,
                render_map,
                cell_size,
                &mut paused,
                &mut advance_one,
            );
            profiler.on_tick_start(tick);
            if !paused || advance_one {
                sim_ui_state.schedule.execute(&mut sim_ui_state.world, &mut sim_ui_state.resources);
                advance_one = false;
            }
            renderer.render_ui(sim_ui_state, tick);
            profiler.on_tick_end(tick);
        }
        profiler.on_simulation_end_ui(sim_ui_state, ticks);
    }
}

/// No-op renderer for headless mode.
pub struct NoOpRenderer;
impl SimulationRenderer for NoOpRenderer {}

/// No-op input handler for headless mode.
pub struct NoOpInput;
impl SimulationInput for NoOpInput {}

/// No-op profiler for headless mode.
pub struct NoOpProfiler;
impl SimulationProfiler for NoOpProfiler {}
