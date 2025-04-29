# Agent-Agent Interaction System Test Plan

## 1. Understand the System
- Read the source code for `agent_agent_interaction.rs` to clarify:
  - What triggers an interaction between agents (distance, state, etc.)?
  - What components are required for an agent to participate?
  - What are the side effects (event log updates, state changes, etc.)?
  - How are errors or edge cases handled?

---

## 2. Identify All Features and Behaviors to Test

### A. Core Features
- Agents interact when within a certain distance.
- No interaction occurs if agents are out of range.
- Agent state(s) may change after interaction.
- Interaction events are logged.

### B. Edge Cases
- No agents present.
- Agents missing required components.
- Multiple agents in proximity (all valid pairs interact, no duplicates).
- Agents exactly at the interaction threshold.
- Negative or extreme position values.

### C. Integration Points
- Does the event log reflect all interactions?
- Are agent stats or other system components updated after interaction?

---

## 3. Define Test Cases

| Step | Test Name                                | Goal/Scenario                                         | Type         |
|------|------------------------------------------|-------------------------------------------------------|--------------|
|  1   | test_agents_interact_within_range        | Agents within range interact and event is logged      | Unit         |
|  2   | test_no_interaction_out_of_range         | Agents out of range do not interact                   | Unit         |
|  3   | test_interaction_state_updated           | Agent state reflects interaction                      | Unit         |
|  4   | test_multiple_agents_all_interact        | All valid agent pairs interact, no duplicates/misses  | Integration  |
|  5   | test_no_agents_no_panic                  | System runs with no agents                            | Unit/Edge    |
|  6   | test_missing_component_handling          | Agents missing data are skipped gracefully            | Unit/Edge    |
|  7   | test_interaction_at_threshold            | Agents exactly at threshold distance interact         | Unit         |
|  8   | test_negative_position_handling          | Agents with negative/extreme positions handled safely | Unit/Edge    |
|  9   | test_sequential_agent_movement              | Move agents in sequence; only some pairs interact per tick   | Integration  |
| 10   | test_no_interaction_when_out_of_range       | Move agents so they remain out of range                      | Integration  |
| 11   | test_simultaneous_multi_agent_interactions  | Multiple agents in range; all possible pairs interact        | Integration  |
| 12   | test_interaction_cooldown_repeat_prevention | Move apart and together; ensure cooldown/repeat logic works  | Integration  |
| 13   | test_negative_and_large_coordinates         | Agents at negative/large positions interact/move correctly   | Integration  |

---

## 4. For Each Test Case, Specify:
- **Purpose:** What are we verifying?
- **Setup:** What world, agents, and components are needed?
- **Action:** What system function is called?
- **Assertions:** What should be true after the system runs?
- **Clean-up:** Any teardown needed? (Usually not for Rust unit tests.)

### Example: test_agents_interact_within_range
- **Purpose:** Verify two agents within interaction range trigger an interaction and event log entry.
- **Setup:** Create a world with two agents, both with required components, positioned within the interaction distance.
- **Action:** Run `agent_agent_interaction_system`.
- **Assertions:**
  - Event log contains an interaction event for these agents.
  - Agent states are updated if applicable.
- **Clean-up:** N/A

---

## 5. Implementation Steps

1. Scaffold a new test file (if not present) or add to `agent_agent_interaction_system.rs`.
2. For each test:
   - Create a minimal ECS world with required components.
   - Insert agents with test-specific properties.
   - Insert mocks or real event logs as needed.
   - Run the system function.
   - Assert on world state, event log, and agent components.
3. Use idiomatic Rust test patterns:
   - Use `#[test]` and `assert!`, `assert_eq!`.
   - Prefer clear, descriptive test names.
   - Isolate tests (no shared state).
4. Add edge case and integration tests last (they may require more setup).
5. Document each test with comments on its purpose and what it covers.
6. Update your test TODO table as you implement each test.

---

## 6. Review and Maintain
- After each test is written, run `cargo test` and ensure it passes.
- Fix or refactor the system code as needed to make tests pass.
- Regularly review and expand tests as new features or bugs are discovered.

---

## 7. (Optional) Automation & Coverage
- Use `cargo tarpaulin` or similar to measure code coverage and identify gaps.
- Integrate tests into CI (GitHub Actions, etc.) for regression prevention.

---

**Review this plan and suggest any additions or changes. Once approved, implementation can begin!**
