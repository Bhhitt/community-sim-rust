# EventLog Access Unification Checklist

This checklist helps ensure all ECS systems access `EventLog` through ECS resources only, following the unified pattern.

| System Name                        | File                                   | Accesses EventLog? | Pattern to Check/Unify            | Checked |
|-------------------------------------|----------------------------------------|--------------------|-----------------------------------|---------|
| action_selection_system             | src/agent/systems.rs                   | REMOVED             | Deprecated, logic now in agent_action_selection_system | [x]     |
| path_following_system               | src/agent/systems.rs                   | Yes                | Should use ECS resource           | [x]     |
| swimming_system                     | src/agent/swimming.rs                  | Yes                | Should use ECS resource           | [x]     |
| agent_event_log_to_gui_system       | src/agent/event_log_bridge.rs          | Yes                | Should use ECS resource           | [x]     |
| entity_interaction_system           | src/ecs_components.rs                  | Yes                | Should use ECS resource           | [x]     |
| passive_hunger_system               | src/agent/systems.rs                   | No                 | --                                | [x]     |
| agent_death_system                  | src/agent/systems.rs                   | No                 | --                                | [x]     |
| agent_movement_history_system       | src/agent/systems.rs                   | No                 | --                                | [x]     |
| collect_food_spawn_positions_system | src/food/systems.rs                    | No                 | --                                | [x]     |
| collect_food_positions_system       | src/food/systems.rs                    | No                 | --                                | [x]     |
| food_spawn_apply_system             | src/food/systems.rs                    | No                 | --                                | [x]     |

- Mark `[x]` in the `Checked` column as you verify each system.
- For systems that access `EventLog`, ensure **only** ECS resource access is used.
- For others, confirm there is no direct or legacy access to `EventLog`.
