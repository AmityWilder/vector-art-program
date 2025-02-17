use amymath::prelude::{*, Vector2};
use raylib::prelude::Vector2 as RlVector2;

pub struct ArtBoard {
    pub name: String,
    pub rect: IRect2,
}

impl ArtBoard {
    pub fn new(name: String, rect: IRect2) -> Self {
        Self { name, rect }
    }

    pub fn get_screen_tl_br(&self, world_to_screen: impl Fn(RlVector2) -> RlVector2) -> (Vector2, Vector2) {
        let tl_world = RlVector2::new(self.rect.min.x as f32, self.rect.min.y as f32);
        let br_world = RlVector2::new(self.rect.max.x as f32, self.rect.max.y as f32);
        (world_to_screen(tl_world).into(), world_to_screen(br_world).into())
    }
}
