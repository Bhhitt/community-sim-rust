# RF6: Unified Simulation Loop Refactor Plan

## Objective
Unify the simulation core so that both headless (ASCII) and graphics (SDL2) modes share the same ECS schedule, world/resource setup, and agent spawning logic. Graphics becomes an optional rendering/input plug-in, not a separate mode.

---

## Step-by-Step Refactor Instructions

### 1. Extract Unified Simulation Core Setup
- Move all world/resource creation, `InitConfig` insertion, and ECS schedule building into a single function (e.g., `setup_simulation_core`).
- This function should return:
  - `world`
  - `resources`
  - `schedule`
  - `map`
  - (optionally: `sim_profile`, `agent_types`, etc.)

### 2. Select Renderer and Input Plug-in
- At runtime, based on CLI args or config:
  - If headless: use `NoOpRenderer` and `NoOpInput`.
  - If graphics: use `SdlRenderer` and `SdlInput` (or any future graphical renderer).

### 3. Unified Simulation Loop
- Call a single `run_simulation_loop` function, passing in:
  - `world`, `resources`, `schedule`
  - The selected renderer and input handler
  - Other required state (profile, map, etc.)

### 4. Remove Duplicated Setup Logic
- Eliminate any branching or duplication in world/resource/schedule setup between modes.
- Only the renderer/input should differ.

### 5. Ensure ECS Schedule Always Runs
- The ECS schedule (including agent/food spawning) must run in both headless and graphics modes.
- Run the schedule at least once before the first render.

### 6. Update Documentation and Comments
- Update code comments and documentation to reflect the unified architecture.
- Remove or deprecate any legacy/unused schedule builders.

---

## Summary
- Simulation core (world/resources/ECS schedule) is always executed.
- Rendering/input is a plug-in: ASCII (headless) or SDL2 (graphics).
- No duplication; minimal branching.
- Agent spawning and all ECS logic are always consistent between modes.

---

**Review this plan and provide feedback or approval before implementation.**
