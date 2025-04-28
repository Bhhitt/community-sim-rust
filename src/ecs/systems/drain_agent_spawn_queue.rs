use legion::systems::SystemBuilder;
// use crate::ecs::agent_spawn_queue::AGENT_SPAWN_QUEUE;
use crate::ecs::systems::pending_agent_spawns::PendingAgentSpawns;

// [RF4] DEPRECATED: This system has been removed. All agent spawn requests should go directly into PendingAgentSpawns ECS resource.
// (See Plan RF4)
// pub fn drain_agent_spawn_queue_system() -> impl legion::systems::Runnable {
//     SystemBuilder::new("DrainAgentSpawnQueueSystem")
//         .write_resource::<PendingAgentSpawns>()
//         .build(|_, _world, pending_spawns, _| {
//             log::debug!("[SYSTEM] Entering drain_agent_spawn_queue_system");
//             // let mut queue = AGENT_SPAWN_QUEUE.lock().unwrap();
//             // for req in queue.drain(..) {
//             //     pending_spawns.add(req.pos, req.agent_type);
//             // }
//         })
// }
