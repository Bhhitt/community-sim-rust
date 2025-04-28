use legion::systems::Builder;

pub fn add_food_systems(builder: &mut Builder) {
    builder.add_system(crate::food::systems::collect_food_positions_system());
    builder.flush();
    builder.add_system(crate::ecs::systems::food::food_collection_system());
    builder.flush();
    builder.add_system(crate::food::systems::collect_food_spawn_positions_system());
    builder.flush();
    builder.add_system(crate::food::systems::food_spawn_apply_system());
    builder.flush();
}
