use crate::{
    components::*,
    ecs::{systems::Schedulable, *},
};

pub fn build() -> impl Schedulable {
    SystemBuilder::<()>::new("MissingPreviousParentSystem")
        // Entities with missing `PreviousParent`
        .with_query(<(Entity, Read<Parent>)>::query().filter(
            component::<LocalToParent>()
                & component::<LocalToWorld>()
                & !component::<PreviousParent>(),
        ))
        .build(move |commands, world, _resource, query| {
            // Add missing `PreviousParent` components
            for (entity, _parent) in query.iter(world) {
                log::trace!("Adding missing PreviousParent to {:?}", entity);
                commands.add_component(*entity, PreviousParent(None));
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn previous_parent_added() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut resources = Resources::default();
        let mut world = Universe::new().create_world();

        let mut schedule = Schedule::builder().add_system(build()).build();

        let e1 = world.push((
            Translation::identity(),
            LocalToParent::identity(),
            LocalToWorld::identity(),
        ));

        let e2 = world.push((
            Translation::identity(),
            LocalToParent::identity(),
            LocalToWorld::identity(),
            Parent(e1),
        ));

        schedule.execute(&mut world, &mut resources);

        assert_eq!(
            world
                .entry(e1)
                .unwrap()
                .get_component::<PreviousParent>()
                .is_ok(),
            false
        );

        assert_eq!(
            world
                .entry(e2)
                .unwrap()
                .get_component::<PreviousParent>()
                .is_ok(),
            true
        );
    }
}
