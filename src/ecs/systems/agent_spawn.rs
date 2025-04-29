// ECS System: agent_spawning_system
// Consumes PendingAgentSpawns and creates new agent entities in the ECS world.

use legion::systems::Runnable;
use legion::systems::SystemBuilder;
use legion::IntoQuery;
use crate::agent::components::{Hunger, Energy, AgentState, IdlePause, SwimmingProfile, InteractionState, MovementHistory, Target, IntendedAction};
use crate::ecs::systems::pending_agent_spawns::{PendingAgentSpawns, AgentSpawnRequest};
use crate::agent::event::{AgentEvent, AgentEventLog};
use crate::map::Map;
use crate::navigation::Path;
use rand;
use std::collections::VecDeque;
use log;
use std::time::Instant;

pub fn agent_spawning_system() -> impl Runnable {
    let start = Instant::now();
    log::debug!("[SYSTEM] START agent_spawning_system");
    let sys = SystemBuilder::new("AgentSpawningSystem")
        .write_resource::<PendingAgentSpawns>()
        .write_resource::<AgentEventLog>()
        .read_resource::<Map>()
        .build(|cmd, _world, (pending_spawns, agent_event_log, map), _| {
            log::debug!("[SYSTEM] [CLOSURE] ENTER agent_spawning_system");
            log::debug!("[SYSTEM] Entering agent_spawning_system");
            let mut to_spawn = Vec::new();
            let mut tick_spawn_count = 0;
            // Scope all mutable borrows (pending_spawns, agent_event_log, cmd)
            {
                while let Some(request) = pending_spawns.pop() {
                    log::debug!("[DEBUG][AgentSpawningSystem] Popped AgentSpawnRequest: pos=({}, {}), type={}", request.pos.x, request.pos.y, request.agent_type.name);
                    to_spawn.push(request);
                }
                log::debug!("[DEBUG][AgentSpawningSystem] Total AgentSpawnRequests to spawn this tick: {}", to_spawn.len());
                if to_spawn.is_empty() {
                    log::warn!("[DEBUG][AgentSpawningSystem] No AgentSpawnRequests to process this tick!");
                }
                for AgentSpawnRequest { pos, agent_type } in &to_spawn {
                    let mut rng = rand::thread_rng();
                    let swim_chance_percent = rand::Rng::gen_range(&mut rng, 1..=30);
                    let swimming_profile = SwimmingProfile {
                        swim_chance_percent,
                        swim_ticks_remaining: 0,
                    };
                    let hunger_threshold = agent_type.hunger_threshold;
                    let entity = cmd.push((pos.clone(), agent_type.clone(), Hunger { value: 100.0, threshold: hunger_threshold }, Energy { value: 100.0 }, AgentState::Idle, IntendedAction::Wander));
                    log::debug!("[DEBUG][AgentSpawningSystem] Spawned agent entity {:?} at ({}, {}) of type {}", entity, pos.x, pos.y, agent_type.name);
                    tick_spawn_count += 1;
                    cmd.add_component(entity, IdlePause::default());
                    cmd.add_component(entity, swimming_profile);
                    cmd.add_component(entity, InteractionState::default());
                    cmd.add_component(entity, MovementHistory::new(12));
                    cmd.add_component(entity, Path::default());
                    cmd.add_component(entity, Target::default());
                    log::info!("[SPAWN_DEBUG] Entity {:?} components: Position=({}, {}), AgentType={}, Hunger=100, Energy=100, AgentState=Idle, IdlePause=default, SwimmingProfile={{swim_chance_percent:{}, swim_ticks_remaining:0}}, InteractionState=default, MovementHistory=12, Path=default, Target=default", entity, pos.x, pos.y, agent_type.name, swim_chance_percent);
                    agent_event_log.push(AgentEvent::Spawned {
                        agent: entity,
                        agent_type: agent_type.name.clone(),
                        pos: (pos.x, pos.y),
                    });
                }
            }
            // Logging of agent count moved to agent_spawn_log_system
            log::debug!("[SYSTEM] [CLOSURE] EXIT agent_spawning_system");
        });
    let duration = start.elapsed();
    log::info!(target: "ecs_profile", "[PROFILE] System agent_spawning_system took {:?}", duration);
    log::debug!("[SYSTEM] END agent_spawning_system");
    sys
}
