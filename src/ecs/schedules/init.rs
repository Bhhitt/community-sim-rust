//! Adds ECS initialization systems to the schedule (RF6)

use legion::systems::Builder;
use crate::ecs::systems::initial_spawn::initial_spawn_system;

pub fn add_init_systems(builder: &mut Builder) {
    builder.add_system(initial_spawn_system());
    builder.flush();
}
