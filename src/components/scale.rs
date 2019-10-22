use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Scale(pub f32);

impl Scale {
    pub fn identity() -> Self {
        Scale(1.0)
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scale({})", self.0)
    }
}
