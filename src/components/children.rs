use crate::{dynamic_array::DynamicArray8, ecs::prelude::*};

#[derive(Debug, Clone)]
pub struct Children(pub(crate) DynamicArray8<Entity>);

impl Children {
    pub fn with(entity: Entity) -> Self {
        Self(DynamicArray8::with(entity))
    }
}
