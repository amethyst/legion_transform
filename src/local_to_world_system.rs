#![allow(dead_code)]
use crate::{components::*, ecs::prelude::*, math::Matrix4};

#[derive(Default)]
pub struct LocalToWorldSystemBundle;
impl LocalToWorldSystemBundle {
    pub fn build(&mut self, _: &mut World) -> Box<dyn Schedulable> {
        SystemBuilder::<()>::new("LocalToWorldUpdateSystem")
            // Translation
            .with_query(
                <(Write<TLocalToWorld>, Read<TTranslation>)>::query().filter(
                    !component::<Parent>()
                        & !component::<TRotation>()
                        & !component::<TScale>()
                        & !component::<TNonUniformScale>()
                        & (changed::<TTranslation>()),
                ),
            )
            // Rotation
            .with_query(<(Write<TLocalToWorld>, Read<TRotation>)>::query().filter(
                !component::<Parent>()
                    & !component::<TTranslation>()
                    & !component::<TScale>()
                    & !component::<TNonUniformScale>()
                    & (changed::<TRotation>()),
            ))
            // Scale
            .with_query(<(Write<TLocalToWorld>, Read<TScale>)>::query().filter(
                !component::<Parent>()
                    & !component::<TTranslation>()
                    & !component::<TRotation>()
                    & !component::<TNonUniformScale>()
                    & (changed::<TScale>()),
            ))
            // NonUniformScale
            .with_query(
                <(Write<TLocalToWorld>, Read<TNonUniformScale>)>::query().filter(
                    !component::<Parent>()
                        & !component::<TTranslation>()
                        & !component::<TRotation>()
                        & !component::<TScale>()
                        & (changed::<TNonUniformScale>()),
                ),
            )
            // Translation + Rotation
            .with_query(
                <(Write<TLocalToWorld>, Read<TTranslation>, Read<TRotation>)>::query().filter(
                    !component::<Parent>()
                        & !component::<TScale>()
                        & !component::<TNonUniformScale>()
                        & (changed::<TTranslation>() | changed::<TRotation>()),
                ),
            )
            // Translation + Scale
            .with_query(
                <(Write<TLocalToWorld>, Read<TTranslation>, Read<TScale>)>::query().filter(
                    !component::<Parent>()
                        & !component::<TRotation>()
                        & !component::<TNonUniformScale>()
                        & (changed::<TTranslation>() | changed::<TScale>()),
                ),
            )
            // Translation + NonUniformScale
            .with_query(
                <(
                    Write<TLocalToWorld>,
                    Read<TTranslation>,
                    Read<TNonUniformScale>,
                )>::query()
                .filter(
                    !component::<Parent>()
                        & !component::<TRotation>()
                        & !component::<TScale>()
                        & (changed::<TTranslation>() | changed::<TNonUniformScale>()),
                ),
            )
            // Rotation + Scale
            .with_query(
                <(Write<TLocalToWorld>, Read<TRotation>, Read<TScale>)>::query().filter(
                    !component::<Parent>()
                        & !component::<TTranslation>()
                        & !component::<TNonUniformScale>()
                        & (changed::<TRotation>() | changed::<TScale>()),
                ),
            )
            // Rotation + NonUniformScale
            .with_query(
                <(
                    Write<TLocalToWorld>,
                    Read<TRotation>,
                    Read<TNonUniformScale>,
                )>::query()
                .filter(
                    !component::<Parent>()
                        & !component::<TTranslation>()
                        & !component::<TScale>()
                        & (changed::<TRotation>() | changed::<TNonUniformScale>()),
                ),
            )
            // Translation + Rotation + Scale
            .with_query(
                <(
                    Write<TLocalToWorld>,
                    Read<TTranslation>,
                    Read<TRotation>,
                    Read<TScale>,
                )>::query()
                .filter(
                    !component::<Parent>()
                        & !component::<TNonUniformScale>()
                        & (changed::<TTranslation>()
                            | changed::<TRotation>()
                            | changed::<TScale>()),
                ),
            )
            // Translation + Rotation + NonUniformScale
            .with_query(
                <(
                    Write<TLocalToWorld>,
                    Read<TTranslation>,
                    Read<TRotation>,
                    Read<TNonUniformScale>,
                )>::query()
                .filter(
                    !component::<Parent>()
                        & !component::<TScale>()
                        & (changed::<TTranslation>()
                            | changed::<TRotation>()
                            | changed::<TNonUniformScale>()),
                ),
            )
            // Just to issue warnings: Scale + NonUniformScale
            .with_query(
                <(Read<TLocalToWorld>, Read<TScale>, Read<TNonUniformScale>)>::query()
                    .filter(!component::<Parent>()),
            )
            .build(move |_commands, _world, _, queries| {
                // Translation
                queries.0.par_for_each(|(mut ltw, translation)| {
                    *ltw = TLocalToWorld(translation.to_homogeneous());
                });

                // Rotation
                queries.1.par_for_each(|(mut ltw, rotation)| {
                    *ltw = TLocalToWorld(rotation.to_homogeneous());
                });

                // Scale
                queries.2.par_for_each(|(mut ltw, scale)| {
                    *ltw = TLocalToWorld(Matrix4::new_scaling(scale.0));
                });

                // NonUniformScale
                queries.3.par_for_each(|(mut ltw, non_uniform_scale)| {
                    *ltw = TLocalToWorld(Matrix4::new_nonuniform_scaling(&non_uniform_scale.0));
                });

                // Translation + Rotation
                queries.4.par_for_each(|(mut ltw, translation, rotation)| {
                    *ltw = TLocalToWorld(
                        rotation
                            .to_homogeneous()
                            .append_translation(&translation.vector),
                    );
                });

                // Translation + Scale
                queries.5.par_for_each(|(mut ltw, translation, scale)| {
                    *ltw = TLocalToWorld(translation.to_homogeneous().prepend_scaling(scale.0));
                });

                // Translation + NonUniformScale
                queries
                    .6
                    .par_for_each(|(mut ltw, translation, non_uniform_scale)| {
                        *ltw = TLocalToWorld(
                            translation
                                .to_homogeneous()
                                .prepend_nonuniform_scaling(&non_uniform_scale.0),
                        );
                    });

                // Rotation + Scale
                queries.7.par_for_each(|(mut ltw, rotation, scale)| {
                    *ltw = TLocalToWorld(rotation.to_homogeneous().prepend_scaling(scale.0));
                });

                // Rotation + NonUniformScale
                queries
                    .8
                    .par_for_each(|(mut ltw, rotation, non_uniform_scale)| {
                        *ltw = TLocalToWorld(
                            rotation
                                .to_homogeneous()
                                .prepend_nonuniform_scaling(&non_uniform_scale.0),
                        );
                    });

                // Translation + Rotation + Scale
                queries
                    .9
                    .par_for_each(|(mut ltw, translation, rotation, scale)| {
                        *ltw = TLocalToWorld(
                            rotation
                                .to_homogeneous()
                                .append_translation(&translation.vector)
                                .prepend_scaling(scale.0),
                        );
                    });

                // Translation + Rotation + NonUniformScale
                queries
                    .10
                    .par_for_each(|(mut ltw, translation, rotation, non_uniform_scale)| {
                        *ltw = TLocalToWorld(
                            rotation
                                .to_homogeneous()
                                .append_translation(&translation.vector)
                                .prepend_nonuniform_scaling(&non_uniform_scale.0),
                        );
                    });

                // Just to issue warnings: Scale + NonUniformScale
                queries.11.iter_entities().for_each(
                    |(entity, (mut _ltw, _scale, _non_uniform_scale))| {
                        log::warn!(
                            "Entity {:?} has both a Scale and NonUniformScale component.",
                            entity
                        );
                    },
                );
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use legion::resource::Resources;

    #[test]
    fn correct_transformation() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut world = Universe::new().create_world();
        let resources = Resources::default();
        let system = LocalToWorldSystemBundle::default().build(&mut world);

        let ltw = TLocalToWorld::identity();
        let t = TTranslation::new(1.0, 2.0, 3.0);
        let r = TRotation::from_euler_angles(1.0, 2.0, 3.0);
        let s = TScale(2.0);
        let nus = TNonUniformScale::new(1.0, 2.0, 3.0);

        // Add every combination of transform types.
        let translation = *world.insert((), vec![(ltw, t)]).first().unwrap();
        let rotation = *world.insert((), vec![(ltw, r)]).first().unwrap();
        let scale = *world.insert((), vec![(ltw, s)]).first().unwrap();
        let non_uniform_scale = *world.insert((), vec![(ltw, nus)]).first().unwrap();
        let translation_and_rotation = *world.insert((), vec![(ltw, t, r)]).first().unwrap();
        let translation_and_scale = *world.insert((), vec![(ltw, t, s)]).first().unwrap();
        let translation_and_nus = *world.insert((), vec![(ltw, t, nus)]).first().unwrap();
        let rotation_scale = *world.insert((), vec![(ltw, r, s)]).first().unwrap();
        let rotation_nus = *world.insert((), vec![(ltw, r, nus)]).first().unwrap();
        let translation_rotation_scale = *world.insert((), vec![(ltw, t, r, s)]).first().unwrap();
        let translation_rotation_nus = *world.insert((), vec![(ltw, t, r, nus)]).first().unwrap();

        // Run the system
        system.run(&resources, &mut world);
        system.command_buffer_mut().write(&mut world);

        // Verify that each was transformed correctly.
        assert_eq!(
            world.get_component::<TLocalToWorld>(translation).unwrap().0,
            t.to_homogeneous()
        );
        assert_eq!(
            world.get_component::<TLocalToWorld>(rotation).unwrap().0,
            r.to_homogeneous()
        );
        assert_eq!(
            world.get_component::<TLocalToWorld>(scale).unwrap().0,
            Matrix4::new_scaling(s.0),
        );
        assert_eq!(
            world
                .get_component::<TLocalToWorld>(non_uniform_scale)
                .unwrap()
                .0,
            Matrix4::new_nonuniform_scaling(&nus.0),
        );
        assert_eq!(
            world
                .get_component::<TLocalToWorld>(translation_and_rotation)
                .unwrap()
                .0,
            r.to_homogeneous().append_translation(&t.vector),
        );
        assert_eq!(
            world
                .get_component::<TLocalToWorld>(translation_and_scale)
                .unwrap()
                .0,
            t.to_homogeneous().prepend_scaling(s.0),
        );
        assert_eq!(
            world
                .get_component::<TLocalToWorld>(translation_and_nus)
                .unwrap()
                .0,
            t.to_homogeneous().prepend_nonuniform_scaling(&nus.0),
        );
        assert_eq!(
            world
                .get_component::<TLocalToWorld>(rotation_scale)
                .unwrap()
                .0,
            r.to_homogeneous().prepend_scaling(s.0)
        );
        assert_eq!(
            world
                .get_component::<TLocalToWorld>(rotation_nus)
                .unwrap()
                .0,
            r.to_homogeneous().prepend_nonuniform_scaling(&nus.0)
        );
        assert_eq!(
            world
                .get_component::<TLocalToWorld>(translation_rotation_scale)
                .unwrap()
                .0,
            r.to_homogeneous()
                .append_translation(&t.vector)
                .prepend_scaling(s.0)
        );
        assert_eq!(
            world
                .get_component::<TLocalToWorld>(translation_rotation_nus)
                .unwrap()
                .0,
            r.to_homogeneous()
                .append_translation(&t.vector)
                .prepend_nonuniform_scaling(&nus.0)
        );
    }
}
