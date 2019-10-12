#[derive(Clone, Copy)]
pub struct Scale(pub f32);

impl Scale {
    pub fn identity() -> Self {
        Scale(1.0)
    }
}
