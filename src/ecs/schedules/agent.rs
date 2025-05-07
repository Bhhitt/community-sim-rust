use legion::systems::Builder;

pub fn add_agent_core_systems(builder: &mut Builder) {
    // --- Agent core systems ---
    builder.add_system(crate::ecs::systems::agent_spawn::agent_spawning_system()); 
    builder.flush();
   // builder.add_system(crate::ecs::systems::agent::agent_pausing_system()); 
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent_action_decision::agent_action_decision_system());
   // builder.flush();
    // Assigns InteractionIntent to eligible agents after action decision
   // builder.add_system(crate::ecs::systems::agent_agent_interaction::intent_assignment_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent_target_assignment::agent_target_assignment_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent_path_assignment::agent_path_assignment_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent_agent_interaction::agent_agent_interaction_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent::agent_path_movement_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent::agent_direct_movement_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent::agent_movement_history_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent::agent_hunger_energy_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent_state_transition::agent_state_transition_system());
   // builder.flush();
   // builder.add_system(crate::ecs::systems::agent_arrival_event::agent_arrival_event_system());
   // builder.flush();
    //builder.add_system(crate::ecs::systems::interaction_end_event::interaction_end_event_system());
    //builder.flush();
    //builder.add_system(crate::ecs::systems::agent::add_profile_system());
    //builder.flush();
}
