use legion::systems::SystemBuilder;
use legion::IntoQuery;
use log;

pub fn agent_spawn_log_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("AgentSpawnLogSystem")
        .read_resource::<crate::map::Map>()
        .read_resource::<crate::log_config::LogConfig>()
        .build(|_cmd, world, (map, log_config), _| {
            if log_config.quiet { return; }
            let agent_count = <(&crate::ecs_components::Position,)>::query().iter(world).count();
            log::info!("[DEBUG][AgentSpawnLogSystem] Number of agents in world after spawn: {}", agent_count);
        })
}
