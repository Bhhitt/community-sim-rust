// Dedicated agent logging systems for ECS events.
// Each system handles logging for a specific agent event type (arrival, move, spawn, etc.).

use legion::{Entity, IntoQuery};
use legion::systems::Runnable;
use std::sync::{Arc, Mutex};
use log;

/// Logs agent arrival events (e.g., when AgentState::Arrived is detected)
pub fn agent_arrival_logging_system() -> impl legion::systems::Runnable {
    log::debug!("[SYSTEM] START agent_arrival_logging_system");
    let sys = legion::SystemBuilder::new("AgentArrivalLoggingSystem")
        .with_query(<(Entity, &crate::ecs_components::Position, &crate::agent::AgentState)>::query())
        .write_resource::<Arc<Mutex<crate::event_log::EventLog>>>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(|_, world, resources, query| {
            for (entity, pos, agent_state) in query.iter(world) {
                if *agent_state == crate::agent::AgentState::Arrived && !resources.1.quiet {
                    let msg = format!("[ARRIVAL] Agent {:?} arrived at ({:.2}, {:.2})", entity, pos.x, pos.y);
                    resources.0.lock().unwrap().push(msg.clone());
                    log::debug!("{}", msg);
                }
            }
        });
    log::debug!("[SYSTEM] END agent_arrival_logging_system");
    sys
}

/// Logs agent movement events (e.g., when an agent moves position)
/// NOTE: You can expand this system to include movement deltas, speed, etc.
pub fn agent_move_logging_system() -> impl legion::systems::Runnable {
    log::debug!("[SYSTEM] START agent_move_logging_system");
    let sys = legion::SystemBuilder::new("AgentMoveLoggingSystem")
        .with_query(<(Entity, &crate::ecs_components::Position, &crate::agent::AgentState)>::query())
        .write_resource::<Arc<Mutex<crate::event_log::EventLog>>>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(|_, world, resources, query| {
            for (entity, pos, agent_state) in query.iter(world) {
                if *agent_state == crate::agent::AgentState::Moving && !resources.1.quiet {
                    let msg = format!("[MOVE] Agent {:?} moved to ({:.2}, {:.2})", entity, pos.x, pos.y);
                    resources.0.lock().unwrap().push(msg.clone());
                    log::debug!("{}", msg);
                }
            }
        });
    log::debug!("[SYSTEM] END agent_move_logging_system");
    sys
}

/// Logs agent spawn events (e.g., when an agent is created)
pub fn agent_spawn_logging_system() -> impl legion::systems::Runnable {
    log::debug!("[SYSTEM] START agent_spawn_logging_system");
    let sys = legion::SystemBuilder::new("AgentSpawnLoggingSystem")
        .with_query(<(Entity, &crate::ecs_components::Position, &crate::agent::AgentType)>::query())
        .write_resource::<Arc<Mutex<crate::event_log::EventLog>>>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(|_, world, resources, query| {
            log::debug!("[SYSTEM] [CLOSURE] ENTER agent_spawn_logging_system");
            for (entity, pos, agent_type) in query.iter(world) {
                if !resources.1.quiet {
                    resources.0.lock().unwrap().push(
                        format!("[SPAWN] Agent {:?} of type '{}' spawned at ({:.2}, {:.2})", entity, agent_type.name, pos.x, pos.y)
                    );
                }
            }
            log::debug!("[SYSTEM] [CLOSURE] EXIT agent_spawn_logging_system");
        });
    log::debug!("[SYSTEM] END agent_spawn_logging_system");
    sys
}
