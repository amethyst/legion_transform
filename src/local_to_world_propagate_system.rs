#![allow(dead_code)]
use crate::{
    components::*,
    ecs::{
        systems::{CommandBuffer, Schedulable},
        world::SubWorld,
        *,
    },
};

pub fn build(_: &mut World, _: &mut Resources) -> impl Schedulable {
    SystemBuilder::<()>::new("LocalToWorldPropagateSystem")
        // Entities with a `Children` and `LocalToWorld` but NOT a `Parent` (ie those that are
        // roots of a hierarchy).
        .with_query(<(Read<Children>, Read<LocalToWorld>)>::query().filter(!component::<Parent>()))
        .read_component::<Children>()
        .read_component::<LocalToParent>()
        .build(move |commands, world, _resource, query| {
            for (children, local_to_world) in query.iter(world) {
                for child in children.0.iter() {
                    propagate_recursive(*local_to_world, world, *child, commands);
                }
            }
        })
}

fn propagate_recursive(
    parent_local_to_world: LocalToWorld,
    world: &SubWorld,
    entity: Entity,
    commands: &mut CommandBuffer,
) {
    log::trace!("Updating LocalToWorld for {:?}", entity);
    let local_to_parent = {
        if let Some(entry) = world.entry_ref(entity) {
            if let Some(local_to_parent) = entry.get_component::<LocalToParent>() {
                *local_to_parent
            } else {
                log::warn!(
                    "Entity {:?} is a child in the hierarchy but does not have a LocalToParent",
                    entity
                );
                return;
            }
        } else {
            log::warn!("Entity {:?} does not exist but is a list of childs", entity);
            return;
        }
    };

    let new_local_to_world = LocalToWorld(parent_local_to_world.0 * local_to_parent.0);
    commands.add_component(entity, new_local_to_world);

    // Collect children
    let children = if let Some(entry) = world.entry_ref(entity) {
        entry
            .get_component::<Children>()
            .map(|e| e.0.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    } else {
        Vec::default()
    };

    for child in children {
        propagate_recursive(new_local_to_world, world, child, commands);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        local_to_parent_system, local_to_world_propagate_system, local_to_world_system,
        missing_previous_parent_system, parent_update_system,
    };

    #[test]
    fn did_propagate() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut resources = Resources::default();
        let mut world = Universe::new().create_world();

        let mut schedule = Schedule::builder()
            .add_system(missing_previous_parent_system::build(
                &mut world,
                &mut resources,
            ))
            .flush()
            .add_system(parent_update_system::build(&mut world, &mut resources))
            .flush()
            .add_system(local_to_parent_system::build(&mut world, &mut resources))
            .flush()
            .add_system(local_to_world_system::build(&mut world, &mut resources))
            .flush()
            .add_system(local_to_world_propagate_system::build(
                &mut world,
                &mut resources,
            ))
            .build();

        // Root entity
        let parent = world.push((Translation::new(1.0, 0.0, 0.0), LocalToWorld::identity()));

        let children = world.extend(vec![
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
        ]);
        let (e1, e2) = (children[0], children[1]);

        // Parent `e1` and `e2` to `parent`.
        world.entry(e1).unwrap().add_component(Parent(parent));
        world.entry(e2).unwrap().add_component(Parent(parent));

        // Run the needed systems on it.
        schedule.execute(&mut world, &mut resources);

        assert_eq!(
            world
                .entry(e1)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            Translation::new(1.0, 0.0, 0.0).to_homogeneous()
                * Translation::new(0.0, 2.0, 0.0).to_homogeneous()
        );

        assert_eq!(
            world
                .entry(e2)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            Translation::new(1.0, 0.0, 0.0).to_homogeneous()
                * Translation::new(0.0, 0.0, 3.0).to_homogeneous()
        );
    }
}
