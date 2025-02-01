use amymath::prelude::Rect2;
use raylib::prelude::*;
use crate::rec::UIRect;

pub struct Panel {
    pub is_hidden: bool,
    pub rec: UIRect,
    pub rec_cache: Rect2,
    pub background: Color,
}

impl Panel {
    pub fn new(container: &Rect2, rec: UIRect, background: Color) -> Self {
        Self {
            is_hidden: false,
            background,
            rec_cache: rec.rect(container),
            rec,
        }
    }

    pub fn update_rec(&mut self, container: &Rect2) {
        self.rec_cache = self.rec.rect(container);
    }

    pub fn is_overlapping_point(&self, point: Vector2) -> bool {
        self.rec_cache.is_overlapping_point(point)
    }
}
