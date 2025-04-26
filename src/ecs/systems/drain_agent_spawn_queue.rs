use legion::systems::SystemBuilder;
use crate::ecs::agent_spawn_queue::AGENT_SPAWN_QUEUE;
use crate::ecs::systems::pending_agent_spawns::PendingAgentSpawns;

/// ECS system: Drains the global agent spawn queue into the ECS PendingAgentSpawns resource.
pub fn drain_agent_spawn_queue_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("DrainAgentSpawnQueueSystem")
        .write_resource::<PendingAgentSpawns>()
        .build(|_, _world, pending_spawns, _| {
            let mut queue = AGENT_SPAWN_QUEUE.lock().unwrap();
            for req in queue.drain(..) {
                pending_spawns.add(req.pos, req.agent_type);
            }
        })
}
