use crate::math::Vector3;

#[derive(Clone, Copy)]
pub struct NonUniformScale(pub Vector3<f32>);

impl NonUniformScale {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector3::new(x, y, z))
    }
}
