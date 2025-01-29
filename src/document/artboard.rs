use amymath::prelude::IntRect2;

pub struct ArtBoard {
    pub name: String,
    pub rect: IntRect2,
}

impl ArtBoard {
    pub fn new(name: String, rect: IntRect2) -> Self {
        Self { name, rect }
    }
}
