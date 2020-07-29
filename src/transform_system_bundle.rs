use crate::{
    ecs::systems::ParallelRunnable, local_to_parent_system, local_to_world_propagate_system,
    local_to_world_system, missing_previous_parent_system, parent_update_system,
};

pub fn build() -> Vec<Box<dyn ParallelRunnable>> {
    let mut all_systems = Vec::<Box<dyn ParallelRunnable>>::with_capacity(5);
    all_systems.push(Box::new(missing_previous_parent_system::build()));
    all_systems.push(Box::new(parent_update_system::build()));
    all_systems.push(Box::new(local_to_parent_system::build()));
    all_systems.push(Box::new(local_to_world_system::build()));
    all_systems.push(Box::new(local_to_world_propagate_system::build()));

    all_systems
}
