use legion::systems::SystemBuilder;
use std::sync::{Arc, Mutex};
use crate::agent::event::AgentEventLog;
use crate::event_log::EventLog;

/// System to bridge AgentEventLog to EventLog for GUI display
pub fn agent_event_log_to_gui_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("AgentEventLogToGui")
        .write_resource::<Arc<Mutex<EventLog>>>()
        .write_resource::<AgentEventLog>()
        .build(|_cmd, _world, (event_log, agent_event_log), _| {
            for event in &agent_event_log.0 {
                event_log.lock().unwrap().push(event.to_log_string());
            }
            agent_event_log.clear();
        })
}
