use community_sim::agent::event::{AgentEvent, AgentEventLog};
use community_sim::event_log::EventLog;
use legion::Entity;
use legion::World;

#[test]
fn agent_event_log_records_events() {
    let mut log = AgentEventLog::default();
    let mut world = legion::World::default();
    let dummy_agent = world.push(());
    let dummy_food = world.push(());
    log.push(AgentEvent::Spawned {
        agent: dummy_agent,
        agent_type: "test_type".to_string(),
        pos: (1.0, 2.0),
    });
    log.push(AgentEvent::AteFood {
        agent: dummy_agent,
        food: dummy_food,
        nutrition: 5.0,
    });
    assert_eq!(log.0.len(), 2);
    assert!(matches!(log.0[0], AgentEvent::Spawned { .. }));
    assert!(matches!(log.0[1], AgentEvent::AteFood { .. }));
}
