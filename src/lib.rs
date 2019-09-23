#[macro_use]
extern crate getset;
#[macro_use]
extern crate derive_new;

pub use legion as ecs;
pub use nalgebra as math;

pub mod components;
pub mod systems;

pub use components::Transform;
pub use systems::TransformSystem;
