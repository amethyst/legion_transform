#![feature(test)]

extern crate test;

use legion::*;
use legion_transform::{local_to_world_system, prelude::*};
use test::Bencher;

#[bench]
fn local_to_world_update_without_change(b: &mut Bencher) {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut resources = Resources::default();
    let mut world = Universe::new().create_world();
    let mut schedule = Schedule::builder()
        .add_system(local_to_world_system::build(&mut world, &mut resources))
        .build();

    let ltw = LocalToWorld::identity();
    let t = Translation::new(1.0, 2.0, 3.0);
    let r = Rotation::from_euler_angles(1.0, 2.0, 3.0);
    let s = Scale(2.0);
    let nus = NonUniformScale::new(1.0, 2.0, 3.0);

    // Add N of every combination of transform types.
    let n = 1000;
    world.extend(vec![(ltw, t); n]);
    world.extend(vec![(ltw, r); n]);
    world.extend(vec![(ltw, s); n]);
    world.extend(vec![(ltw, nus); n]);
    world.extend(vec![(ltw, t, r); n]);
    world.extend(vec![(ltw, t, s); n]);
    world.extend(vec![(ltw, t, nus); n]);
    world.extend(vec![(ltw, r, s); n]);
    world.extend(vec![(ltw, r, nus); n]);
    world.extend(vec![(ltw, t, r, s); n]);
    world.extend(vec![(ltw, t, r, nus); n]);

    // Run the system once outside the test (which should compute everything and it shouldn't be
    // touched again).
    schedule.execute(&mut world, &mut resources);

    // Then time the already-computed updates.
    b.iter(|| {
        schedule.execute(&mut world, &mut resources);
    });
}
