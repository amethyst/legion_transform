#![allow(dead_code)]
use crate::{
    components::*,
    ecs::{systems::ParallelRunnable, *},
    math::Matrix4,
};

pub fn build() -> impl ParallelRunnable {
    SystemBuilder::<()>::new("LocalToWorldUpdateSystem")
        // Translation
        .with_query(<(Write<LocalToWorld>, Read<Translation>)>::query().filter(
            !component::<Parent>()
                & !component::<Rotation>()
                & !component::<Scale>()
                & !component::<NonUniformScale>()
                & (maybe_changed::<Translation>()),
        ))
        // Rotation
        .with_query(<(Write<LocalToWorld>, Read<Rotation>)>::query().filter(
            !component::<Parent>()
                & !component::<Translation>()
                & !component::<Scale>()
                & !component::<NonUniformScale>()
                & (maybe_changed::<Rotation>()),
        ))
        // Scale
        .with_query(<(Write<LocalToWorld>, Read<Scale>)>::query().filter(
            !component::<Parent>()
                & !component::<Translation>()
                & !component::<Rotation>()
                & !component::<NonUniformScale>()
                & (maybe_changed::<Scale>()),
        ))
        // NonUniformScale
        .with_query(
            <(Write<LocalToWorld>, Read<NonUniformScale>)>::query().filter(
                !component::<Parent>()
                    & !component::<Translation>()
                    & !component::<Rotation>()
                    & !component::<Scale>()
                    & (maybe_changed::<NonUniformScale>()),
            ),
        )
        // Translation + Rotation
        .with_query(
            <(Write<LocalToWorld>, Read<Translation>, Read<Rotation>)>::query().filter(
                !component::<Parent>()
                    & !component::<Scale>()
                    & !component::<NonUniformScale>()
                    & (maybe_changed::<Translation>() | maybe_changed::<Rotation>()),
            ),
        )
        // Translation + Scale
        .with_query(
            <(Write<LocalToWorld>, Read<Translation>, Read<Scale>)>::query().filter(
                !component::<Parent>()
                    & !component::<Rotation>()
                    & !component::<NonUniformScale>()
                    & (maybe_changed::<Translation>() | maybe_changed::<Scale>()),
            ),
        )
        // Translation + NonUniformScale
        .with_query(
            <(
                Write<LocalToWorld>,
                Read<Translation>,
                Read<NonUniformScale>,
            )>::query()
            .filter(
                !component::<Parent>()
                    & !component::<Rotation>()
                    & !component::<Scale>()
                    & (maybe_changed::<Translation>() | maybe_changed::<NonUniformScale>()),
            ),
        )
        // Rotation + Scale
        .with_query(
            <(Write<LocalToWorld>, Read<Rotation>, Read<Scale>)>::query().filter(
                !component::<Parent>()
                    & !component::<Translation>()
                    & !component::<NonUniformScale>()
                    & (maybe_changed::<Rotation>() | maybe_changed::<Scale>()),
            ),
        )
        // Rotation + NonUniformScale
        .with_query(
            <(Write<LocalToWorld>, Read<Rotation>, Read<NonUniformScale>)>::query().filter(
                !component::<Parent>()
                    & !component::<Translation>()
                    & !component::<Scale>()
                    & (maybe_changed::<Rotation>() | maybe_changed::<NonUniformScale>()),
            ),
        )
        // Translation + Rotation + Scale
        .with_query(
            <(
                Write<LocalToWorld>,
                Read<Translation>,
                Read<Rotation>,
                Read<Scale>,
            )>::query()
            .filter(
                !component::<Parent>()
                    & !component::<NonUniformScale>()
                    & (maybe_changed::<Translation>()
                        | maybe_changed::<Rotation>()
                        | maybe_changed::<Scale>()),
            ),
        )
        // Translation + Rotation + NonUniformScale
        .with_query(
            <(
                Write<LocalToWorld>,
                Read<Translation>,
                Read<Rotation>,
                Read<NonUniformScale>,
            )>::query()
            .filter(
                !component::<Parent>()
                    & !component::<Scale>()
                    & (maybe_changed::<Translation>()
                        | maybe_changed::<Rotation>()
                        | maybe_changed::<NonUniformScale>()),
            ),
        )
        // Just to issue warnings: Scale + NonUniformScale
        .with_query(
            <(
                Entity,
                Read<LocalToWorld>,
                Read<Scale>,
                Read<NonUniformScale>,
            )>::query()
            .filter(!component::<Parent>()),
        )
        .build(move |_commands, world, _, queries| {
            let (a, b, c, d, e, f, g, h, i, j, k, l) = queries;
            rayon::scope(|s| {
                s.spawn(|_| unsafe {
                    // Translation
                    a.for_each_unchecked(world, |(ltw, translation)| {
                        *ltw = LocalToWorld(translation.to_homogeneous());
                    });
                });
                s.spawn(|_| unsafe {
                    // Rotation
                    b.for_each_unchecked(world, |(ltw, rotation)| {
                        *ltw = LocalToWorld(rotation.to_homogeneous());
                    });
                });
                s.spawn(|_| unsafe {
                    // Scale
                    c.for_each_unchecked(world, |(ltw, scale)| {
                        *ltw = LocalToWorld(Matrix4::new_scaling(scale.0));
                    });
                });
                s.spawn(|_| unsafe {
                    // NonUniformScale
                    d.for_each_unchecked(world, |(ltw, non_uniform_scale)| {
                        *ltw = LocalToWorld(Matrix4::new_nonuniform_scaling(&non_uniform_scale.0));
                    });
                });
                s.spawn(|_| unsafe {
                    // Translation + Rotation
                    e.for_each_unchecked(world, |(ltw, translation, rotation)| {
                        *ltw = LocalToWorld(
                            rotation
                                .to_homogeneous()
                                .append_translation(&translation.vector),
                        );
                    });
                });
                s.spawn(|_| unsafe {
                    // Translation + Scale
                    f.for_each_unchecked(world, |(ltw, translation, scale)| {
                        *ltw = LocalToWorld(translation.to_homogeneous().prepend_scaling(scale.0));
                    });
                });
                s.spawn(|_| unsafe {
                    // Translation + NonUniformScale
                    g.for_each_unchecked(world, |(ltw, translation, non_uniform_scale)| {
                        *ltw = LocalToWorld(
                            translation
                                .to_homogeneous()
                                .prepend_nonuniform_scaling(&non_uniform_scale.0),
                        );
                    });
                });
                s.spawn(|_| unsafe {
                    // Rotation + Scale
                    h.for_each_unchecked(world, |(ltw, rotation, scale)| {
                        *ltw = LocalToWorld(rotation.to_homogeneous().prepend_scaling(scale.0));
                    });
                });
                s.spawn(|_| unsafe {
                    // Rotation + NonUniformScale
                    i.for_each_unchecked(world, |(ltw, rotation, non_uniform_scale)| {
                        *ltw = LocalToWorld(
                            rotation
                                .to_homogeneous()
                                .prepend_nonuniform_scaling(&non_uniform_scale.0),
                        );
                    });
                });
                s.spawn(|_| unsafe {
                    // Translation + Rotation + Scale
                    j.for_each_unchecked(world, |(ltw, translation, rotation, scale)| {
                        *ltw = LocalToWorld(
                            rotation
                                .to_homogeneous()
                                .append_translation(&translation.vector)
                                .prepend_scaling(scale.0),
                        );
                    });
                });
                s.spawn(|_| unsafe {
                    // Translation + Rotation + NonUniformScale
                    k.for_each_unchecked(
                        world,
                        |(ltw, translation, rotation, non_uniform_scale)| {
                            *ltw = LocalToWorld(
                                rotation
                                    .to_homogeneous()
                                    .append_translation(&translation.vector)
                                    .prepend_nonuniform_scaling(&non_uniform_scale.0),
                            );
                        },
                    );
                });

                // Just to issue warnings: Scale + NonUniformScale
                l.iter(world)
                    .for_each(|(entity, mut _ltw, _scale, _non_uniform_scale)| {
                        log::warn!(
                            "Entity {:?} has both a Scale and NonUniformScale component.",
                            entity
                        );
                    });
            });
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_world_transformation() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut resources = Resources::default();
        let mut world = World::default();
        let mut schedule = Schedule::builder().add_system(build()).build();

        let ltw = LocalToWorld::identity();
        let t = Translation::new(1.0, 2.0, 3.0);
        let r = Rotation::from_euler_angles(1.0, 2.0, 3.0);
        let s = Scale(2.0);
        let nus = NonUniformScale::new(1.0, 2.0, 3.0);

        // Add every combination of transform types.
        let translation = world.push((ltw, t));
        let rotation = world.push((ltw, r));
        let scale = world.push((ltw, s));
        let non_uniform_scale = world.push((ltw, nus));
        let translation_and_rotation = world.push((ltw, t, r));
        let translation_and_scale = world.push((ltw, t, s));
        let translation_and_nus = world.push((ltw, t, nus));
        let rotation_scale = world.push((ltw, r, s));
        let rotation_nus = world.push((ltw, r, nus));
        let translation_rotation_scale = world.push((ltw, t, r, s));
        let translation_rotation_nus = world.push((ltw, t, r, nus));

        // Run the system
        schedule.execute(&mut world, &mut resources);

        // Verify that each was transformed correctly.
        assert_eq!(
            world
                .entry(translation)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            t.to_homogeneous()
        );
        assert_eq!(
            world
                .entry(rotation)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            r.to_homogeneous()
        );
        assert_eq!(
            world
                .entry(scale)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            Matrix4::new_scaling(s.0),
        );
        assert_eq!(
            world
                .entry(non_uniform_scale)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            Matrix4::new_nonuniform_scaling(&nus.0),
        );
        assert_eq!(
            world
                .entry(translation_and_rotation)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            r.to_homogeneous().append_translation(&t.vector),
        );
        assert_eq!(
            world
                .entry(translation_and_scale)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            t.to_homogeneous().prepend_scaling(s.0),
        );
        assert_eq!(
            world
                .entry(translation_and_nus)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            t.to_homogeneous().prepend_nonuniform_scaling(&nus.0),
        );
        assert_eq!(
            world
                .entry(rotation_scale)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            r.to_homogeneous().prepend_scaling(s.0)
        );
        assert_eq!(
            world
                .entry(rotation_nus)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            r.to_homogeneous().prepend_nonuniform_scaling(&nus.0)
        );
        assert_eq!(
            world
                .entry(translation_rotation_scale)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            r.to_homogeneous()
                .append_translation(&t.vector)
                .prepend_scaling(s.0)
        );
        assert_eq!(
            world
                .entry(translation_rotation_nus)
                .unwrap()
                .get_component::<LocalToWorld>()
                .unwrap()
                .0,
            r.to_homogeneous()
                .append_translation(&t.vector)
                .prepend_nonuniform_scaling(&nus.0)
        );
    }
}
