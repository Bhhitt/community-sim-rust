// Dedicated agent logging systems for ECS events.
// Each system handles logging for a specific agent event type (arrival, move, spawn, etc.).

use legion::{Entity, IntoQuery};
use legion::systems::Runnable;
use std::sync::{Arc, Mutex};
use log;
use std::time::Instant;

/// Logs agent arrival events (e.g., when AgentState::Arrived is detected)
pub fn agent_arrival_logging_system() -> impl legion::systems::Runnable {
    log::debug!("[SYSTEM] START agent_arrival_logging_system");
    let sys = legion::SystemBuilder::new("AgentArrivalLoggingSystem")
        .with_query(<(Entity, &crate::ecs_components::Position, &crate::agent::AgentState)>::query())
        .write_resource::<Arc<Mutex<crate::event_log::EventLog>>>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(|_, world, resources, query| {
            let start = Instant::now();
            if resources.1.quiet { return; }
            for (entity, pos, agent_state) in query.iter(world) {
                if *agent_state == crate::agent::AgentState::Arrived {
                    let msg = format!("[ARRIVAL] Agent {:?} arrived at ({:.2}, {:.2})", entity, pos.x, pos.y);
                    if let Ok(mut event_log) = resources.0.lock() {
                        event_log.push(msg.clone());
                    } else {
                        log::error!("[ARRIVAL_LOG] Failed to acquire lock on event_log for agent arrival");
                    }
                    log::debug!("{}", msg);
                }
            }
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_arrival_logging_system took {:?}", duration);
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
            let start = Instant::now();
            if resources.1.quiet { return; }
            for (entity, pos, agent_state) in query.iter(world) {
                if *agent_state == crate::agent::AgentState::Moving {
                    let msg = format!("[MOVE] Agent {:?} moved to ({:.2}, {:.2})", entity, pos.x, pos.y);
                    if let Ok(mut event_log) = resources.0.lock() {
                        event_log.push(msg.clone());
                    } else {
                        log::error!("[MOVE_LOG] Failed to acquire lock on event_log for agent move");
                    }
                    log::debug!("{}", msg);
                }
            }
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_move_logging_system took {:?}", duration);
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
            let start = Instant::now();
            if resources.1.quiet { return; }
            for (entity, pos, agent_type) in query.iter(world) {
                let msg = format!("[SPAWN] Agent {:?} of type '{}' spawned at ({:.2}, {:.2})", entity, agent_type.name, pos.x, pos.y);
                if resources.0.lock().is_ok() {
                    if let Ok(mut event_log) = resources.0.lock() {
                        event_log.push(msg.clone());
                    }
                } else {
                    log::error!("[SPAWN_LOG] Failed to acquire lock on event_log for agent spawn");
                }
            }
            log::debug!("[SYSTEM] [CLOSURE] EXIT agent_spawn_logging_system");
            let duration = start.elapsed();
            log::info!(target: "ecs_profile", "[PROFILE] System agent_spawn_logging_system took {:?}", duration);
        });
    log::debug!("[SYSTEM] END agent_spawn_logging_system");
    sys
}
