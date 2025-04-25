use community_sim::agent::event::{AgentEvent, AgentEventLog};
use community_sim::event_log::EventLog;
use community_sim::agent::event_log_bridge::agent_event_log_to_gui_system;
use legion::*;

#[test]
fn agent_event_log_to_gui_system_pushes_events() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut agent_event_log = AgentEventLog::default();
    let dummy_agent = world.push(());
    agent_event_log.push(AgentEvent::Spawned {
        agent: dummy_agent,
        agent_type: "TestType".to_string(),
        pos: (1.0, 2.0),
    });
    let mut event_log = EventLog::new(10);
    resources.insert(agent_event_log);
    resources.insert(event_log);
    let mut schedule = Schedule::builder()
        .add_system(agent_event_log_to_gui_system())
        .build();
    schedule.execute(&mut world, &mut resources);
    let event_log = resources.get::<EventLog>().unwrap();
    assert!(event_log.events.iter().any(|msg| msg.contains("[SPAWNED] Agent")));
    let agent_event_log = resources.get::<AgentEventLog>().unwrap();
    assert!(agent_event_log.0.is_empty(), "AgentEventLog should be cleared after bridge system");
}
