// ECS System: agent_spawning_system
// Consumes PendingAgentSpawns and creates new agent entities in the ECS world.

use legion::systems::Runnable;
use legion::systems::SystemBuilder;
use crate::agent::components::{Hunger, Energy, AgentState, IdlePause, SwimmingProfile, InteractionState, MovementHistory, Path, Target};
use crate::ecs::systems::pending_agent_spawns::{PendingAgentSpawns, AgentSpawnRequest};
use crate::agent::event::{AgentEvent, AgentEventLog};
use crate::map::Map;
use rand;
use std::collections::VecDeque;

pub fn agent_spawning_system() -> impl Runnable {
    SystemBuilder::new("AgentSpawningSystem")
        .write_resource::<PendingAgentSpawns>()
        .write_resource::<AgentEventLog>()
        .read_resource::<Map>()
        .build(|cmd, _world, (pending_spawns, agent_event_log, map), _| {
            let mut to_spawn = Vec::new();
            while let Some(request) = pending_spawns.pop() {
                to_spawn.push(request);
            }
            for AgentSpawnRequest { pos, agent_type } in to_spawn {
                let mut rng = rand::thread_rng();
                let swim_chance_percent = rand::Rng::gen_range(&mut rng, 1..=30);
                let swimming_profile = SwimmingProfile {
                    swim_chance_percent,
                    swim_ticks_remaining: 0,
                };
                let hunger_threshold = agent_type.hunger_threshold;
                let entity = cmd.push((pos, agent_type.clone(), Hunger { value: 100.0, threshold: hunger_threshold }, Energy { value: 100.0 }, AgentState::Idle));
                cmd.add_component(entity, IdlePause::default());
                cmd.add_component(entity, SwimmingProfile::default());
                cmd.add_component(entity, InteractionState::default());
                cmd.add_component(entity, MovementHistory::new(12));
                cmd.add_component(entity, Path::default());
                cmd.add_component(entity, Target::default());
                agent_event_log.push(AgentEvent::Spawned {
                    agent: entity,
                    agent_type: agent_type.name.clone(),
                    pos: (pos.x, pos.y),
                });
            }
        })
}
