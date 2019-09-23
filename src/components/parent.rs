use crate::ecs::prelude::Entity;

/// Component for defining a parent entity.
///
/// The entity with this component *has* a parent, rather than *is* a parent.
///
/// If the parent entity contains a transform, then the child's transform
/// will be applied relative to the parent's transform. For example, if a
/// parent rotates 45 degrees around the Z axis, then the child's coordinate
/// system will start out also rotated by 45 degrees around the Z axis.
///
/// If the parent entity has its own parent (and so on) then the transforms
/// will all be applied in order from the oldest ancestor to the child.
#[derive(Debug, Clone, Eq, PartialEq, new)]
pub struct Parent (pub Entity);
