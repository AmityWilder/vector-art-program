use raylib::prelude::*;

pub struct Bitmap {
    texture: Texture2D,
    source_rec: Rectangle,
    dest_rec: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
}

impl Bitmap {
    pub fn new(texture: Texture2D, position: Vector2) -> Self {
        let (width, height) = (texture.width as f32, texture.height as f32);
        Self {
            texture,
            source_rec: Rectangle::new(0.0, 0.0, width, height),
            dest_rec: Rectangle::new(position.x, position.y, width, height),
            origin: Vector2::new(width * 0.5, height * 0.5),
            rotation: 0.0,
            tint: Color::WHITE,
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_texture_pro(&self.texture, self.source_rec, self.dest_rec, self.origin, self.rotation, self.tint);
    }
}
