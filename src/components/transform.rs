//! All the different transform types supported. Note that you can add any combination of
//! components, except for both TScale and TNonUniformScale.
use crate::math::{Matrix4, Rotation3, Translation3, Vector3};

pub type TTranslation = Translation3<f32>;
pub type TRotation = Rotation3<f32>;

#[derive(Clone, Copy)]
pub struct TScale(pub f32);

#[derive(Clone, Copy)]
pub struct TNonUniformScale(pub Vector3<f32>);

impl TNonUniformScale {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector3::new(x, y, z))
    }
}

#[derive(Clone, Copy)]
pub struct TLocalToWorld(pub Matrix4<f32>);

impl TLocalToWorld {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
}

#[derive(Clone, Copy)]
pub struct TLocalToParent(pub Matrix4<f32>);

impl TLocalToParent {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
}
