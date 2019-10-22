use crate::ecs::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Parent(pub Entity);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PreviousParent(pub Option<Entity>);
