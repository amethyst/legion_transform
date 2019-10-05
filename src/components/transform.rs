use crate::math::{Affine3, Rotation3, Similarity3, Translation3, Vector3};

pub type TransformSimilarity3 = Similarity3<f32>;
pub type TransformAffine3 = Affine3<f32>;
pub type TransformTranslation3 = Translation3<f32>;
pub type TransformRation3 = Rotation3<f32>;
pub type TransformNonUniformScale3 = Vector3<f32>;
pub type TransformUniformScale = f32;
