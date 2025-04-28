# TODO

## Simulation Metrics
- [ ] Implement survival curve metrics for agents:
    - Track agent population over time (per tick)
    - Log or export survival curves for analysis
    - Optionally: break down by agent type or cause of death

## Refactoring and ECS Improvements
- [ ] Refactor `ecs_simulation.rs` to split schedule and tick logic into focused, modular ECS systems and schedule builders. See audit notes for refactor plan.
- [ ] Move relevant component structs to `ecs/components.rs` from `ecs_components.rs`, `agent/components.rs`, `food/components.rs`, etc.
- [ ] Fix all usages of `Path` and `Target` to expect them from `agent::components`, and ensure all ECS queries and logic are up to date with new modular system design.
- [ ] Create one file per system (e.g., `agent.rs`, `food.rs`, etc.) in `ecs/systems/` and move system logic from `ecs_sim.rs`, `ecs_simulation.rs`, etc.
- [ ] Move food-related system logic from `ecs_sim.rs`, `ecs_simulation.rs`, or food modules to `ecs/systems/food.rs`.
- [ ] Move terrain-related system logic from `ecs_sim.rs`, `ecs_simulation.rs`, or terrain modules to `ecs/systems/terrain.rs`.

## Simulation Logic
- [ ] Refactor simulation agent spawning logic into a reusable helper for both initial and dynamic (GUI-triggered) agent spawning.
- [ ] Remove unresolved imports for missing systems in `agent/mod.rs`.
- [ ] Remove unused imports in `simulation.rs`.
- [ ] Implement real food search logic in `agent_target_assignment.rs`.
- [ ] Make `max_distance` in `agent_path_assignment.rs` configurable if needed.
- [ ] Use `agent_type.decision_engine` to select rules or MLP for agent action decision in `agent_action_decision.rs`.
- [ ] Items, money, etc. in `sim_core.rs`.

## Graphics Mode / UI
- [ ] Re-enable agent spawning in graphics mode (`input_systems.rs`).
- [ ] Fill food/agent spawn positions in `sim_render.rs` as needed.
- [ ] Move SDL2 rendering logic to `render_ui` in `sim_render.rs` using `sim_ui_state`.
- [ ] Move SDL2 input logic to `handle_input_ui` in `sim_render.rs` using `sim_ui_state`.
- [ ] Add profiling/summary logic for graphics mode in `sim_render.rs`.
- [ ] Use proper tick count or exit condition in `sim_render.rs` main loop.

## Performance
- [ ] Optimize flamegraph SVGs as noted in flamegraph files.

## Notes
Survival curves are important for understanding population dynamics and simulation health. Consider integrating this into `sim_summary.rs` or as a separate metrics/export module.
