// ECS System: agent_spawning_system
// Consumes PendingAgentSpawns and creates new agent entities in the ECS world.

use legion::{World, Resources, Entity, systems::SystemBuilder};
use crate::ecs_components::{Position};
use crate::agent::{AgentType, Hunger, Energy, AgentState, components::{IdlePause, SwimmingProfile, MovementHistory, InteractionState}, event::{AgentEvent, AgentEventLog}};
use crate::ecs::systems::pending_agent_spawns::{PendingAgentSpawns, AgentSpawnRequest};
use crate::map::Map;
use std::collections::VecDeque;

pub fn agent_spawning_system() -> impl legion::systems::Runnable {
    SystemBuilder::new("AgentSpawningSystem")
        .write_resource::<PendingAgentSpawns>()
        .write_resource::<AgentEventLog>()
        .read_resource::<Map>()
        .build(|cmd, world, (pending_spawns, agent_event_log, map), _| {
            let mut to_spawn = Vec::new();
            while let Some(request) = pending_spawns.pop() {
                to_spawn.push(request);
            }
            for AgentSpawnRequest { pos, agent_type } in to_spawn {
                // Randomize swim_chance_percent
                let mut rng = rand::thread_rng();
                let swim_chance_percent = rand::Rng::gen_range(&mut rng, 1..=30);
                let swimming_profile = SwimmingProfile {
                    swim_chance_percent,
                    swim_ticks_remaining: 0,
                };
                let hunger_threshold = agent_type.hunger_threshold;
                let entity = world.push((
                    pos,
                    agent_type.clone(),
                    Hunger { value: 100.0, threshold: hunger_threshold },
                    Energy { value: 100.0 },
                    InteractionState { target: None, ticks: 0, last_partner: None, cooldown: 0, recent_partners: VecDeque::new() },
                    AgentState::Idle,
                    IdlePause { ticks_remaining: 0 },
                ));
                world.extend(vec![(entity, crate::ecs_components::Target { x: pos.x, y: pos.y, stuck_ticks: 0, path_ticks: None, ticks_to_reach: None })]);
                world.extend(vec![(entity, crate::ecs_components::Path { waypoints: VecDeque::new() })]);
                agent_event_log.push(AgentEvent::Spawned {
                    agent: entity,
                    agent_type: agent_type.name.clone(),
                    pos: (pos.x, pos.y),
                });
                world.extend(vec![(entity, MovementHistory::new(12))]);
                world.extend(vec![(entity, swimming_profile)]);
            }
        })
}
