# Agent Action Decision System Refactor Plan

## Objective
Refactor the agent action selection logic into modular ECS systems that support both rules-based and neural-network-based (MLP) decision engines, with configurability at the agent type level. This will enable per-agent-type behavioral diversity and future extensibility.

---

## Current State
- The `agent_action_selection_system` handles:
  - Checking if an agent is idle and ready for a new action.
  - Deciding (rules-based): seek food if hungry, otherwise wander.
  - Assigning a target coordinate.
  - Computing and assigning a path.
  - Setting agent state to `Moving`.
- MLP (multi-layer perceptron) code exists and is configurable via `MLPConfig`.
- Pathfinding logic is present and used inline.
- All logic is bundled in a single ECS system.

---

## Requirements
1. **Single Responsibility:** Each ECS system should do one thing (decision, target assignment, path assignment, state transition).
2. **Configurable Decision Engine:** Each agent type can specify `rules` or `mlp` for its action decision engine.
3. **MLP Integration:** If `mlp` is selected, use the MLP (from config) for decision-making.
4. **YAML Configuration:** Agent type config (agent_types.yaml) must specify the decision engine and, if `mlp`, the MLP parameters.
5. **Extensible:** Future decision engines should be easy to add.

---

## Proposed System Split

### 1. Action Decision System
- Checks if the agent is idle and not paused.
- Uses the agent type's configured engine (rules or MLP) to choose the next action (e.g., SeekFood, Wander, Idle).
- Stores the result in a new `IntendedAction` component.

### 2. Target Assignment System
- Reads `IntendedAction` and assigns a target coordinate if needed (e.g., food position for SeekFood, random for Wander).
- Updates/creates a `Target` component.

### 3. Path Assignment System
- Computes and assigns a path to the target using the existing pathfinding logic.
- Updates/creates a `Path` component.

### 4. State Transition System
- Updates the agent's state (e.g., Idle → Moving) once a target/path is set.

---

## Config Changes
- **agent_types.yaml:**
  - Add a `decision_engine` key ("rules" or "mlp").
  - If `mlp`, provide an `mlp_config` block or reference.

```yaml
# Example agent_types.yaml
agent_types:
  classic:
    decision_engine: rules
    ...
  neural:
    decision_engine: mlp
    mlp_config:
      input_size: 4
      hidden_sizes: [8, 8]
      output_size: 2
      weights: ...
      biases: ...
```

- **Rust Structs:**
  - Extend `AgentType` to include a `decision_engine: DecisionEngineType` field.
  - Define enum:
    ```rust
    pub enum DecisionEngineType {
        Rules,
        MLP(MLPConfig),
    }
    ```

---

## Migration Steps
1. Update `agent_types.yaml` and parsing logic.
2. Refactor/split current action selection system as described.
3. Implement `IntendedAction` component and enum.
4. Integrate MLP forward pass for agents with `mlp` engine.
5. Test both rules-based and MLP-based agents in simulation.
6. Document new architecture and usage.

---

## Future Extensions
- Support for additional decision engines (e.g., decision trees, external APIs).
- Training and updating MLP weights at runtime.
- Analytics and debugging tools for agent decisions.

---

## Open Questions
- What inputs should be provided to the MLP for decision-making?
- Should MLP configs be inline or referenced by name?
- Do we want to support hybrid or fallback engines?
