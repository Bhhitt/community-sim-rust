// ECS System: Agent Target Assignment System
// Assigns a target coordinate based on IntendedAction.

use legion::{Entity, IntoQuery, systems::Runnable, systems::SystemBuilder};
use crate::agent::components::{IntendedAction, AgentState, Target};
use crate::agent::AgentType;
use crate::ecs_components::Position;
use crate::map::Map;
use crate::ecs_components::FoodPositions;

pub fn agent_target_assignment_system() -> impl Runnable {
    SystemBuilder::new("AgentTargetAssignmentSystem")
        .with_query(<(Entity, &IntendedAction, &Position, &AgentType, &mut Option<Target>, &mut AgentState)>::query())
        .read_resource::<Map>()
        .read_resource::<FoodPositions>()
        .build(|_, world, resources, query| {
            let map = &resources.0;
            let food_positions = &resources.1;
            for (_entity, intended_action, pos, agent_type, maybe_target, _agent_state) in query.iter_mut(world) {
                match intended_action {
                    IntendedAction::SeekFood => {
                        // Find closest food
                        if let Some((fx, fy)) = find_closest_food(pos.x, pos.y, &**food_positions) {
                            let target = Target {
                                x: fx as f32,
                                y: fy as f32,
                                stuck_ticks: 0,
                                path_ticks: None,
                                ticks_to_reach: None,
                            };
                            *maybe_target = Some(target);
                        }
                    }
                    IntendedAction::Wander => {
                        let mut rng = rand::thread_rng();
                        let (wx, wy) = crate::navigation::random_passable_target(&**map, agent_type, &mut rng, None);
                        let target = Target {
                            x: wx as f32,
                            y: wy as f32,
                            stuck_ticks: 0,
                            path_ticks: None,
                            ticks_to_reach: None,
                        };
                        *maybe_target = Some(target);
                    }
                    _ => {}
                }
            }
        })
}

// Helper function: find_closest_food
fn find_closest_food(x: f32, y: f32, food_positions: &FoodPositions) -> Option<(i32, i32)> {
    // TODO: Implement real food search logic
    food_positions.0.iter().min_by_key(|(fx, fy)| {
        let dx = *fx as f32 - x;
        let dy = *fy as f32 - y;
        ((dx * dx + dy * dy) * 1000.0) as i32
    }).map(|(fx, fy)| (fx.round() as i32, fy.round() as i32))
}
