use crate::ecs::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parent(pub Entity);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PreviousParent(pub Option<Entity>);
