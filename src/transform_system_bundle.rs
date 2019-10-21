use crate::{
    ecs::prelude::*, hierarchy_maintenance_system, local_to_parent_system,
    local_to_world_propagate_system, local_to_world_system,
};

pub struct TransformSystemBundle;

impl TransformSystemBundle {
    pub fn build(world: &mut World) -> Vec<Box<dyn Schedulable>> {
        let mut all_systems = Vec::with_capacity(5);

        let mut hierarchy_maintenance_systems = hierarchy_maintenance_system::build(world);
        let local_to_parent_system = local_to_parent_system::build(world);
        let local_to_world_system = local_to_world_system::build(world);
        let local_to_world_propagate_system = local_to_world_propagate_system::build(world);

        all_systems.append(&mut hierarchy_maintenance_systems);
        all_systems.push(local_to_parent_system);
        all_systems.push(local_to_world_system);
        all_systems.push(local_to_world_propagate_system);

        all_systems
    }
}

/*

Bundles exist in amethyst, but are not in legion, this is because legion doesnt have a direct dispatcher
This bundle in amethyst is implemented as:

use crate::legion::*;
use amethyst_error::Error;
use legion_transform::*;

#[derive(Default)]
pub struct TransformBundle;
impl SystemBundle for TransformBundle {
    fn build(mut self, world: &mut World, builder: &mut DispatcherBuilder) -> Result<(), Error> {
        hierarchy_maintenance_system::build(world)
            .into_iter()
            .for_each(|system| builder.add_system(Stage::Begin, move |_| system));

        builder.add_system(Stage::Begin, local_to_parent_system::build);
        builder.add_system(Stage::Begin, local_to_world_system::build);
        builder.add_system(Stage::Begin, local_to_world_propagate_system::build);

        Ok(())
    }
}


*/
