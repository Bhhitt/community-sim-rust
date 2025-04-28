use legion::systems::SystemBuilder;
use legion::IntoQuery;
use log;

pub fn agent_spawn_log_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("AgentSpawnLogSystem")
        .read_resource::<crate::map::Map>()
        .build(|_cmd, world, _resources, _| {
            let agent_count = <(&crate::ecs_components::Position,)>::query().iter(world).count();
            log::info!("[DEBUG][AgentSpawnLogSystem] Number of agents in world after spawn: {}", agent_count);
        })
}
