use raylib::prelude::*;

pub struct ArtBoard {
    pub name: String,
    pub rect: Rectangle,
}

impl ArtBoard {
    pub fn new(name: String, rect: Rectangle) -> Self {
        Self { name, rect }
    }
}
