// Interaction Event Logging System
// Logs all relevant interaction events to the event log resources.

use legion::systems::{Runnable, SystemBuilder};
use std::sync::{Arc, Mutex};
use crate::event_log::EventLog;
use crate::agent::event::AgentEventLog;

pub fn interaction_event_logging_system() -> impl Runnable {
    log::debug!("[SYSTEM] START interaction_event_logging_system");
    let sys = SystemBuilder::new("InteractionEventLoggingSystem")
        .write_resource::<Arc<Mutex<EventLog>>>()
        .write_resource::<AgentEventLog>()
        .build(|_cmd, _world, (event_log, agent_event_log), ()| {
            log::debug!("[SYSTEM] [CLOSURE] ENTER interaction_event_logging_system");
            log::debug!("[SYSTEM] Entering interaction_event_logging_system");
            let mut event_log = event_log.lock().unwrap();
            // Drain all agent events and log them to the main event log
            while let Some(agent_event) = agent_event_log.0.pop_front() {
                event_log.push(format!("[AGENT EVENT] {:?}", agent_event));
            }
            // Optionally: Add other logging logic here if needed
            log::debug!("[SYSTEM] [CLOSURE] EXIT interaction_event_logging_system");
        });
    log::debug!("[SYSTEM] END interaction_event_logging_system");
    sys
}
