# ECS Schedule Refactor Plan

## Objective
Create a modular, maintainable, and well-documented ECS schedule using best practices, and remove or archive all legacy/unused schedules. Ensure each system has a single responsibility and that schedule dependencies are clear.

---

## Current State
- The main ECS schedule is built in `src/ecs/schedules/mod.rs` using `build_main_schedule()`.
- Modular domain-specific builders (e.g., `add_food_systems`, `add_agent_core_systems`, etc.) are called from the main builder to add systems for each domain.
- Legacy schedules exist in `/legacy` but are not used in production.
- Some systems (e.g., `entity_interaction_system`) are still monolithic and need to be split.

---

## Refactor Goals & Steps

### 1. Split Monolithic Systems
- [x] Break up any remaining multi-responsibility systems into smaller, focused systems (single responsibility principle).
    - Example: Refactor `entity_interaction_system` into food collection, agent-agent interaction, stats update, and event logging systems. **(COMPLETE)**

### 2. Modularize and Review Schedule Builders
- Ensure each domain-specific builder only adds relevant systems.
- Confirm all new/refactored systems are registered in the correct builder.
- Use `flush()` appropriately to manage dependencies and borrows.

### 3. Document System Dependencies and Order
- Add comments in each domain builder and in `build_main_schedule` to document why the order matters.
- Optionally, add assertions or runtime checks for required dependencies/resources.

### 4. Remove/Archive Legacy Schedules
- Archive or delete `/legacy` schedule files after confirming no dependencies remain.
- Update documentation to clearly state which schedule is authoritative.

### 5. Audit for Orphaned or Unused Systems
- Check for any systems defined but not scheduled.
- Document, remove, or mark as intentionally unused.

### 6. Testing and Validation
- Add/expand tests to cover new modular systems and their integration.
- Validate that the simulation behaves as expected after all changes.

### 7. Update Documentation
- Update README, plan, and audit files to reflect the new modular structure.
- Clearly explain the modular schedule pattern for future contributors.

---

## Checklist
- [x] Split all monolithic systems into focused, single-responsibility systems
- [x] Modularize all schedule builders and ensure correct system registration
- [ ] Document all system dependencies and schedule order
- [ ] Remove or archive all legacy/unused schedules
- [ ] Audit for and document/remove orphaned systems
- [ ] Expand tests and validate simulation correctness
- [ ] Update all documentation to match new structure

---

## Notes
- `/legacy` is not used in production and can be ignored for mainline development. Archive or remove after confirming no dependencies.
- The modular schedule pattern provides a single entry point (`build_main_schedule`) and organizes system registration by domain for clarity and maintainability.
- Document the reasoning for system order and dependencies to avoid future confusion.
