use crate::math::Matrix4;

#[derive(Clone, Copy)]
pub struct LocalToParent(pub Matrix4<f32>);

impl LocalToParent {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
}
