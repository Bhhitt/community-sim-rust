# RF4: Unify Simulation Execution Logic (Headless & Graphics Modes)

## Objective
Unify the simulation execution logic so that both headless and graphics modes share the same ECS world/resource setup, schedule construction, and simulation tick loop. Only rendering and input should differ between modes, ensuring consistent behavior, easier debugging, and improved maintainability.

---

## Concrete Refactor Plan

### 1. Centralize ECS World & Resource Setup
- Ensure all simulation entry points (headless and graphics) call a single function (e.g., `setup_simulation_world_and_resources`) for ECS world/resource/entity setup.
- This function should return all necessary simulation state (world, resources, map, agent count, etc.).

### 2. Centralize Schedule Construction
- Create a single function (e.g., `build_main_schedule`) that builds the ECS schedule for both modes.
- Remove any mode-specific or duplicated schedule-building logic.

### 3. Create a Unified Simulation Loop
- Implement a function (e.g., `run_simulation_loop`) that:
  - Accepts the ECS world, resources, schedule, tick count, and a rendering callback/trait object.
  - Runs the schedule on each tick.
  - Calls the rendering callback (ASCII, GUI, or no-op) at the appropriate time.
  - Optionally accepts an input handler callback for GUI mode.
- Ensure this loop is called by both headless and graphics entry points.

### 4. Refactor Rendering and Input Handling
- Define a trait or closure type for rendering (e.g., `SimulationRenderer`).
- Implement concrete renderers for:
  - Headless mode: ASCII output, every 50 ticks, printing the map as it is.
  - Graphics mode: GUI renderer.
- Pass the appropriate renderer to the simulation loop based on mode.
- Input handling is only needed for graphics mode for now (no interactive headless mode).

### 5. Update All Entry Points
- Headless and graphics entry points should only:
  - Parse mode-specific config/input
  - Set up mode-specific rendering/input callbacks
  - Call the unified simulation loop

### 6. Remove Legacy/Redundant Code
- Delete or fully migrate any legacy simulation loops or tick logic.
- Ensure there is only one place where simulation ticks are processed.

### 7. Test and Validate
- Run both headless and graphics modes.
- Confirm that all ECS systems run, logs are generated, and bugs/panics surface in both modes.
- Ensure rendering and input are handled correctly for each mode.

---

## What the End State Should Look Like

- One function for ECS world/resource setup:  `setup_simulation_world_and_resources`
- One function for schedule construction:  `build_main_schedule`
- One unified simulation loop:  `run_simulation_loop(world, resources, schedule, ticks, renderer, input_handler)`
- Renderer and input handler are passed in as callbacks or trait objects.
- Entry points (main.rs, CLI, GUI) only differ in how they set up rendering/input, not in simulation logic.
- No duplicated or divergent simulation logic between modes.

---

## Clarifying Answers (as of 2025-04-26)

1. **Rendering:**
   - ASCII output is required in headless mode, printing the map as it is, every 50 ticks.
   - Graphics mode uses GUI rendering.
2. **Input:**
   - Input handling is only needed for graphics mode. No interactive headless mode for now.
3. **Profiling/Logging:**
   - [To discuss] Unsure whether profiling/logging should be inside the unified loop or as separate callbacks.
4. **Statistics/Results:**
   - The simulation loop should return some minimal statistics at the end.
5. **Testing:**
   - Integration tests are desired to ensure both modes behave identically for a given seed/config, though priority is currently moderate.

---

**Next:**
- Discuss and decide on profiling/logging architecture (inside the loop vs. as callbacks).
- Begin implementation of the unified simulation loop and rendering callback structure.
- Review/update existing tests as needed after refactor.
