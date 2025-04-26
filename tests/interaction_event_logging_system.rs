//! Tests for Interaction Event Logging System
//! (Blueprints, not meant to be run yet.)

use legion::*;
use std::sync::{Arc, Mutex};
use crate::event_log::EventLog;
use crate::agent::event::AgentEventLog;
use crate::ecs::systems::interaction_event_logging::interaction_event_logging_system;

#[test]
fn test_agent_event_log_drained_to_event_log() {
    // Setup Arc<Mutex<EventLog>> and AgentEventLog with events
    // Run interaction_event_logging_system
    // Assert that events are transferred from AgentEventLog to EventLog
}

#[test]
fn test_no_events_noop() {
    // Setup empty AgentEventLog and EventLog
    // Run interaction_event_logging_system
    // Assert that nothing changes
}
