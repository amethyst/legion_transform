#![allow(dead_code)]
use crate::{
    components::*,
    ecs::{prelude::*, system::PreparedWorld},
};

#[derive(Default)]
pub struct LocalToWorldPropagateSystem;

impl LocalToWorldPropagateSystem {
    pub fn build(&mut self) -> Box<dyn Schedulable> {
        SystemBuilder::<()>::new("LocalToWorldPropagateSystem")
            // Entities with a `Children` and `LocalToWorld` but NOT a `Parent` (ie those that are
            // roots of a hierarchy).
            .with_query(
                <(Read<Children>, Read<LocalToWorld>)>::query().filter(!component::<Parent>()),
            )
            .read_component::<Children>()
            .read_component::<LocalToParent>()
            .build(move |commands, world, _resource, query| {
                for (children, local_to_world) in query.iter() {
                    for child in children.0.iter() {
                        LocalToWorldPropagateSystem::propagate_recursive(
                            *local_to_world,
                            world,
                            *child,
                            commands,
                        );
                    }
                }
            })
    }

    fn propagate_recursive(
        parent_local_to_world: LocalToWorld,
        world: &mut PreparedWorld,
        entity: Entity,
        commands: &mut CommandBuffer,
    ) {
        log::trace!("Updating LocalToWorld for {}", entity);
        let local_to_parent = {
            if let Some(local_to_parent) = world.get_component::<LocalToParent>(entity) {
                *local_to_parent
            } else {
                log::warn!(
                    "Entity {} is a child in the hierarchy but does not have a LocalToParent",
                    entity
                );
                return;
            }
        };

        let new_local_to_world = LocalToWorld(parent_local_to_world.0 * local_to_parent.0);
        commands.add_component(entity, new_local_to_world);

        // Collect children
        let children = world
            .get_component::<Children>(entity)
            .map(|e| e.0.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default();

        for child in children {
            LocalToWorldPropagateSystem::propagate_recursive(
                new_local_to_world,
                world,
                child,
                commands,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        hierarchy_maintenance_system::HierarchyMaintenanceSystem,
        local_to_parent_system::LocalToParentSystem, local_to_world_system::LocalToWorldSystem,
    };

    #[test]
    fn did_propagate() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut world = Universe::new().create_world();

        let hierarchy_maintenance_systems = HierarchyMaintenanceSystem::default().build();
        let local_to_parent_system = LocalToParentSystem::default().build();
        let local_to_world_system = LocalToWorldSystem::default().build();
        let local_to_world_propagate_system = LocalToWorldPropagateSystem::default().build();

        // Root entity
        let parent = *world
            .insert(
                (),
                vec![(Translation::new(1.0, 0.0, 0.0), LocalToWorld::identity())],
            )
            .first()
            .unwrap();

        let children = world.insert(
            (),
            vec![
                (
                    Translation::new(0.0, 2.0, 0.0),
                    LocalToParent::identity(),
                    LocalToWorld::identity(),
                ),
                (
                    Translation::new(0.0, 0.0, 3.0),
                    LocalToParent::identity(),
                    LocalToWorld::identity(),
                ),
            ],
        );
        let (e1, e2) = (children[0], children[1]);

        // Parent `e1` and `e2` to `parent`.
        world.add_component(e1, Parent(parent));
        world.add_component(e2, Parent(parent));

        // Run the needed systems on it.
        for system in hierarchy_maintenance_systems.iter() {
            system.run(&mut world);
            system.command_buffer_mut().write(&mut world);
        }
        local_to_parent_system.run(&mut world);
        local_to_parent_system
            .command_buffer_mut()
            .write(&mut world);
        local_to_world_system.run(&mut world);
        local_to_world_system.command_buffer_mut().write(&mut world);
        local_to_world_propagate_system.run(&mut world);
        local_to_world_propagate_system
            .command_buffer_mut()
            .write(&mut world);

        assert_eq!(
            world.get_component::<LocalToWorld>(e1).unwrap().0,
            Translation::new(1.0, 0.0, 0.0).to_homogeneous()
                * Translation::new(0.0, 2.0, 0.0).to_homogeneous()
        );

        assert_eq!(
            world.get_component::<LocalToWorld>(e2).unwrap().0,
            Translation::new(1.0, 0.0, 0.0).to_homogeneous()
                * Translation::new(0.0, 0.0, 3.0).to_homogeneous()
        );
    }
}
