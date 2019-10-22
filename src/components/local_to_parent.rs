use crate::math::Matrix4;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LocalToParent(pub Matrix4<f32>);

impl LocalToParent {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
}

impl fmt::Display for LocalToParent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
