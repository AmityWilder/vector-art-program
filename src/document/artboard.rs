use amymath::prelude::IntRectangle;

pub struct ArtBoard {
    pub name: String,
    pub rect: IntRectangle,
}

impl ArtBoard {
    pub fn new(name: String, rect: IntRectangle) -> Self {
        Self { name, rect }
    }
}
