# Plan RF6: Pass Real Simulation Data to Input Handling

## Objective
Wire up the SDL2 input handling so that all input actions (pause, advance, spawn agents, etc.) use the actual simulation state and ECS data, not placeholders or defaults.

## Steps

1. **Identify All Required Data**
    - `agent_types: &[AgentType]`
    - `render_map: &Map`
    - `cell_size: f32`
    - `log_config: &LogConfig`
    - `paused: bool`

2. **Update the Input Handler Signature**
    - Change `SdlInput.handle_input_ui` to accept references to the above data (or make these accessible via fields or context).

3. **Propagate Data Through Simulation Loop**
    - Ensure that wherever `handle_input_ui` is called, you have access to the real values for agent types, map, etc.
    - Pass these real values down to the input handler.

4. **Update the Call to `collect_input_events`**
    - Replace all placeholder/default arguments in the call with the real references.

5. **Refactor Construction/Ownership if Needed**
    - If any of these values are not easily accessible where needed, refactor your simulation state or renderer structs to store references or pointers to them.
    - Consider using a context struct or passing them explicitly as arguments.

6. **Test Input Actions**
    - Verify that all input actions (pause, advance, spawn, select, etc.) work correctly and interact with the actual simulation state.

## Additional Considerations

- **Ownership/Lifetimes:** Ensure references are valid for the duration of the simulation loop.
- **Shared Context:** If passing many arguments becomes unwieldy, consider creating a context struct holding all necessary references for input handling.
- **Minimal Duplication:** Ensure both ASCII and graphical modes use the same input logic to avoid code duplication.

## ECS Safety Note
- The `input_queue` is not a Legion ECS component; it is a field on `SimUIState`, which is a simulation UI state struct, not an entity component. Modifying it outside of ECS systems is safe as long as you are not mutating ECS world data directly from outside the ECS schedule.
- **However, if in the future the queue is moved into ECS resources/components, direct mutation outside ECS systems could cause panics or borrow checker issues.**
- For now, the design is safe, but keep this in mind if refactoring input handling to be a true ECS resource/component.

---

**Author:** Cascade AI
**Date:** 2025-04-28
