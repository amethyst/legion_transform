use crate::{
    ecs::{systems::Schedulable, *},
    local_to_parent_system, local_to_world_propagate_system, local_to_world_system,
    missing_previous_parent_system, parent_update_system,
};

pub fn build(world: &mut World, resources: &mut Resources) -> Vec<Box<dyn Schedulable>> {
    let mut all_systems = Vec::<Box<dyn Schedulable>>::with_capacity(5);
    all_systems.push(Box::new(missing_previous_parent_system::build(
        world, resources,
    )));
    all_systems.push(Box::new(parent_update_system::build(world, resources)));
    all_systems.push(Box::new(local_to_parent_system::build(world, resources)));
    all_systems.push(Box::new(local_to_world_system::build(world, resources)));
    all_systems.push(Box::new(local_to_world_propagate_system::build(
        world, resources,
    )));

    all_systems
}
