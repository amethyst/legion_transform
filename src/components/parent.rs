use crate::ecs::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parent {
    pub entity: Entity,
    pub(crate) previous_parent: Option<Entity>,
    pub(crate) depth: u32,
}

impl Parent {
    pub fn new(parent_entity: Entity) -> Self {
        Self {
            entity: parent_entity,
            previous_parent: None,
            depth: 0,
        }
    }
}
