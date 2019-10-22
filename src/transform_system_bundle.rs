use crate::{
    ecs::prelude::*, hierarchy_maintenance_system::HierarchyMaintenanceSystem,
    local_to_parent_system::LocalToParentSystem,
    local_to_world_propagate_system::LocalToWorldPropagateSystem,
    local_to_world_system::LocalToWorldSystem,
};

#[derive(Default)]
pub struct TransformSystemBundle;

impl TransformSystemBundle {
    pub fn build(&mut self) -> Vec<Box<dyn Schedulable>> {
        let mut all_systems = Vec::with_capacity(5);

        let mut hierarchy_maintenance_systems = HierarchyMaintenanceSystem::default().build();
        let local_to_parent_system = LocalToParentSystem::default().build();
        let local_to_world_system = LocalToWorldSystem::default().build();
        let local_to_world_propagate_system = LocalToWorldPropagateSystem::default().build();

        all_systems.append(&mut hierarchy_maintenance_systems);
        all_systems.push(local_to_parent_system);
        all_systems.push(local_to_world_system);
        all_systems.push(local_to_world_propagate_system);

        all_systems
    }
}
