use crate::math::Matrix4;

#[derive(Clone, Copy)]
pub struct LocalToWorld(pub Matrix4<f32>);

impl LocalToWorld {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
}
