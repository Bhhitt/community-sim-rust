# Refactor Plan: run_profile_from_yaml

## Objective
Refactor the `run_profile_from_yaml` function to properly utilize the unified `SimProfile` logic and YAML-driven profiles, eliminate hardcoded parameters, and ensure maintainability and clarity. This will align the simulation launch process with the new profile system and ECS best practices.

## Current Issues
- The function currently hardcodes map size, agent count, and tick parameters instead of loading them from the YAML profile.
- There are commented-out legacy lines and references to old profile loading logic.
- The function does not leverage the full structure of `SimProfile` or its extensibility (e.g., benchmark, quiet).

## Refactor Steps

1. **Profile Loading** ✅
   - Hardcoded parameters have been removed. The function now loads values from the selected `SimProfile` in YAML.
   - Uses the `find_profile` helper to select the correct profile by name.

2. **Parameter Passing** ✅
   - The loaded `SimProfile` is passed directly to downstream functions (such as `run_sim_render`).
   - All fields (including `benchmark`, `quiet`, etc.) are respected and passed through.

3. **Legacy Cleanup** ✅
   - All commented-out or unused legacy code related to the old profile loading and parameter passing has been removed.
   - Log messages now reflect the use of profile-driven parameters.

4. **Error Handling** ✅
   - The function now logs an error and aborts if the requested profile is not found.

5. **Testing**
   - [ ] Test the refactored function with various profiles (including edge cases: missing fields, invalid names, etc.).
   - [ ] Confirm both GUI and headless modes work correctly with the unified profile logic.

6. **Documentation**
   - [ ] Update or add doc comments to the refactored function.
   - [ ] Briefly document the new profile-driven launch flow in the project’s main README or relevant developer docs.

## Acceptance Criteria
- No hardcoded simulation parameters remain in `run_profile_from_yaml`. ✅
- All simulation launches (GUI/headless) use the selected profile from YAML. ✅
- No legacy or commented-out code remains. ✅
- Error cases are handled gracefully with clear logs. ✅
- Function is documented and tested. ⬜
