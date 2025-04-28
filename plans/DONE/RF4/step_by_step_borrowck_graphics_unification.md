# Step-by-Step Guide: Fixing Borrow Checker Issues for Unified Simulation Loop (Graphics Mode)

This guide will help you refactor the simulation loop and graphics mode interface to resolve Rust borrow checker errors and achieve a clean, unified simulation architecture.

---

## 1. **Understand the Problem**

- **Rust does not allow multiple mutable borrows** of the same struct (or its fields) at the same time.
- You cannot pass `&mut sim_ui_state` to one place and also pass `&mut sim_ui_state.world`, `&mut sim_ui_state.resources`, etc., elsewhere.
- This causes E0499/E0505 errors in your simulation loop and graphics code.

---

## 2. **Refactor Trait Interfaces**

### **Current Problematic Pattern:**
```rust
// Traits take split-out fields
fn render(&mut self, world: &World, resources: &Resources, tick: usize);
fn handle_input(&mut self, world: &mut World, resources: &mut Resources, tick: usize);
```

### **Unified, Borrow-Checker-Friendly Pattern:**
- Change your traits to accept a single mutable reference to your simulation/UI state context (e.g., `&mut SimUIState`).
- This ensures only one mutable borrow exists at a time.

```rust
fn render(&mut self, sim_ui_state: &mut SimUIState, tick: usize);
fn handle_input(&mut self, sim_ui_state: &mut SimUIState, tick: usize);
```

---

## 3. **Update Simulation Loop**

- In your unified simulation loop (e.g., `run_simulation_loop`), pass `&mut sim_ui_state` to the renderer, input, and profiler trait methods.
- Only split out fields (like `world`, `resources`, etc.) when absolutely necessary, and never while another mutable borrow exists.

**Example:**
```rust
for tick in 0..ticks {
    profiler.on_tick_start(tick);
    sim_ui_state.schedule.execute(&mut sim_ui_state.world, &mut sim_ui_state.resources);
    renderer.render(&mut sim_ui_state, tick);
    input.handle_input(&mut sim_ui_state, tick);
    profiler.on_tick_end(tick);
}
```

---

## 4. **Update Implementations**

- Refactor `SdlRenderer`, `SdlInput`, and `SdlProfiler` to work with `&mut SimUIState`.
- Access any required fields (camera, canvas, etc.) through the passed-in `sim_ui_state` reference.
- If you need additional context (like SDL2 objects), store them in the struct, but **do not store references to fields of `SimUIState`**.

---

## 5. **Update Call Sites**

- In `run_sim_render`, create a single `SimUIState` and pass a mutable reference to it everywhere needed.
- Do **not** split out mutable references to its fields.
- Pass `&mut sim_ui_state` to all trait objects and the simulation loop.

---

## 6. **Fix Ownership Issues in Agent Spawning**

- If a method (like `PendingAgentSpawns::add`) takes ownership of a value, you cannot use that value again after passing it.
- **Solution:**
    - Change the method to accept a reference if possible (`&AgentType` instead of `AgentType`), or
    - Clone the value at the call site if needed.

---

## 7. **Test and Iterate**

- Rebuild and test your simulation in both headless and graphics modes.
- If you encounter more errors, repeat the above steps, ensuring you never split mutable borrows.

---

## 8. **Optional: Generalize Context**

- If you want to further unify headless and graphics modes, consider defining a `SimulationContext` trait or struct that abstracts over both modes.

---

## 9. **Summary Checklist**
- [ ] Traits take `&mut SimUIState` (or similar), not split fields.
- [ ] Only one mutable borrow of `SimUIState` exists at a time.
- [ ] No references to fields of a struct while the struct itself is borrowed.
- [ ] Ownership/borrowing issues in agent spawning are resolved.
- [ ] All code compiles and runs in both modes.

---

**Following this guide will ensure your simulation architecture is idiomatic, safe, and maximally unified.**

---

**[COMPLETION NOTE — 2025-04-27]**
All items in this guide have been completed. The simulation architecture is now unified, borrow-checker safe, and runs correctly in both ASCII and graphics modes. See project memories and commit history for details.
