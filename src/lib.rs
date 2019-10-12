pub use legion as ecs;
pub use nalgebra as math;

mod components;
mod hierarchy_maintenance_system;
mod local_to_parent_system;
mod local_to_world_propagate_system;
mod local_to_world_system;
mod transform_system_bundle;

pub mod prelude {
    pub use crate::{components::*, transform_system_bundle::TransformSystemBundle};
}
