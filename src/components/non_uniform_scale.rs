use crate::math::Vector3;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NonUniformScale(pub Vector3<f32>);

impl NonUniformScale {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector3::new(x, y, z))
    }
}

impl fmt::Display for NonUniformScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NonUniformScale({}, {}, {})",
            self.0.x, self.0.y, self.0.z
        )
    }
}
