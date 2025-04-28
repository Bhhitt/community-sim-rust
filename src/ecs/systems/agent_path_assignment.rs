// ECS System: Agent Path Assignment System
// Computes and assigns a path to the agent's target.

use legion::{Entity, systems::Runnable, systems::SystemBuilder, IntoQuery};
use crate::agent::components::{Target, AgentState, AgentType};
use crate::ecs_components::Position;
use crate::navigation::pathfinding::a_star_path;
use crate::navigation::Path;
use crate::map::Map;

pub fn agent_path_assignment_system() -> impl Runnable {
    SystemBuilder::new("AgentPathAssignmentSystem")
        .with_query(<(Entity, &Position, &Target, &AgentType, &AgentState, &mut Option<crate::navigation::Path>)>::query())
        .read_resource::<Map>()
        .build(|_, world, resources, query| {
            let map = resources;
            for (_entity, pos, target, agent_type, agent_state, maybe_path) in query.iter_mut(world) {
                // Only assign path if agent is moving and has a target
                if *agent_state == AgentState::Moving {
                    let start = (pos.x.round() as i32, pos.y.round() as i32);
                    let goal = (target.x.round() as i32, target.y.round() as i32);
                    let max_distance = 100; // TODO: Make configurable if needed
                    if let Some(path_vec) = a_star_path(map, agent_type, agent_state, start, goal, max_distance) {
                        let waypoints = path_vec.into_iter().collect();
                        *maybe_path = Some(crate::navigation::Path { waypoints });
                    } else {
                        *maybe_path = None;
                    }
                }
            }
        })
}
