# Simulation Profile Refactor Plan

## 1. Consolidate Simulation Profile Logic
- Create a new module: Move the SimProfile struct and the YAML loading function (load_profiles_from_yaml) into a new file, e.g., `src/sim_profile.rs`.
- Single Source of Truth: All code (graphics and headless) should use this shared module to load and access simulation profiles.

## 2. Unify Profile Usage in Main Entry Point
- Main logic: In main.rs, if the user provides a `--profile` argument, load the corresponding profile from YAML and use its parameters (map size, agent count, ticks, etc.).
- Fallback: If no profile is specified, use CLI arguments as defaults.
- Apply to both modes: Ensure both graphics and headless modes can use profiles.

## 3. Refactor Simulation Launch
- Parameter Passing: Refactor simulation/graphics entry points to accept a SimProfile (or equivalent parameters) instead of separate map size, agent count, etc.
- Batch/Benchmark: For batch or benchmark runs, allow running all or selected profiles from YAML.

## 4. Remove Legacy Code and Comments
- Delete or migrate: Remove the entire legacy/schedule.rs file and any other legacy files that are now redundant.
- Remove commented-out code: Delete commented-out function calls and legacy references from main.rs and other modules.
- Clean up comments: Remove or update any comments referring to “legacy,” “to be migrated,” or “old simulation module.”

## 5. Documentation and Testing
- Document the new profile system: Add comments and a README section describing how to add or use simulation profiles.
- Test: Ensure both graphics and headless modes work with and without profiles, and that batch/benchmark runs are possible.

---

## Summary Table

| Step | Action                                                                 | Outcome                                      |
|------|------------------------------------------------------------------------|----------------------------------------------|
| 1    | Move SimProfile/YAML loader to `sim_profile.rs`                        | Shared, modern profile loading               |
| 2    | Update main.rs to use profile if specified                             | Unified entry point for all modes            |
| 3    | Refactor simulation entry points to use profile struct                 | No more duplicated or scattered parameters   |
| 4    | Remove legacy files, comments, and commented-out code                  | Cleaner, more maintainable codebase          |
| 5    | Document and test                                                      | Easy to use, robust, and future-proof        |
