use legion::systems::Builder;

pub fn add_agent_event_log_bridge_system(builder: &mut Builder) {
    builder.add_system(crate::agent::event_log_bridge::agent_event_log_to_gui_system());
}
