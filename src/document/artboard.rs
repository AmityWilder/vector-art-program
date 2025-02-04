use amymath::prelude::*;
use raylib::prelude::*;

pub struct ArtBoard {
    pub name: String,
    pub rect: IRect2,
}

impl ArtBoard {
    pub fn new(name: String, rect: IRect2) -> Self {
        Self { name, rect }
    }

    pub fn get_screen_tl_br(&self, world_to_screen: impl Fn(Vector2) -> Vector2) -> (Vector2, Vector2) {
        let tl_world = Vector2::new(self.rect.xmin as f32, self.rect.ymin as f32);
        let br_world = Vector2::new(self.rect.xmax as f32, self.rect.ymax as f32);
        (world_to_screen(tl_world), world_to_screen(br_world))
    }
}
