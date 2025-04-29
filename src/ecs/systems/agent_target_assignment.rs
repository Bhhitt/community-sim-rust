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
        .with_query(<(Entity, &IntendedAction, &Position, &AgentType, &mut Target, &mut AgentState)>::query())
        .read_resource::<Map>()
        .read_resource::<FoodPositions>()
        .build(|_, world, resources, query| {
            let map = &resources.0;
            let food_positions = &resources.1;
            let mut count = 0;
            for (_entity, intended_action, pos, agent_type, target, _agent_state) in query.iter_mut(world) {
                count += 1;
                log::debug!("[TARGET_ASSIGN] Agent at ({:.2}, {:.2}) with action {:?}", pos.x, pos.y, intended_action);
                match intended_action {
                    IntendedAction::SeekFood => {
                        // Find closest food
                        if let Some((fx, fy)) = find_closest_food(pos.x, pos.y, &**food_positions) {
                            log::debug!("[TARGET_ASSIGN] SeekFood: Assigning food target ({}, {})", fx, fy);
                            *target = Target {
                                x: fx as f32,
                                y: fy as f32,
                                stuck_ticks: 0,
                                path_ticks: None,
                                ticks_to_reach: None,
                            };
                        } else {
                            log::warn!("[TARGET_ASSIGN] SeekFood: No food found for agent at ({:.2}, {:.2})", pos.x, pos.y);
                        }
                    }
                    IntendedAction::Wander => {
                        log::debug!("[TARGET_ASSIGN] Wander: Picking random target for agent at ({:.2}, {:.2})", pos.x, pos.y);
                        let mut rng = rand::thread_rng();
                        let (wx, wy) = crate::navigation::random_passable_target(
                            &**map,
                            agent_type,
                            &mut rng,
                            Some((pos.x, pos.y)),
                        );
                        log::debug!("[TARGET_ASSIGN] Wander: Assigned wander target ({:.2}, {:.2})", wx, wy);
                        *target = Target {
                            x: wx as f32,
                            y: wy as f32,
                            stuck_ticks: 0,
                            path_ticks: None,
                            ticks_to_reach: None,
                        };
                    }
                    _ => {
                        log::debug!("[TARGET_ASSIGN] Other action: {:?} for agent at ({:.2}, {:.2})", intended_action, pos.x, pos.y);
                    }
                }
            }
            log::info!("[TARGET_ASSIGN] Total agents matched by query: {}", count);
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
