use amymath::prelude::IRect2;
use raylib::prelude::*;
use crate::rec::UIRect;

pub struct Panel {
    pub is_hidden: bool,
    pub rec: UIRect,
    rec_cache: IRect2,
    pub background: Color,
}

impl Panel {
    pub fn new(container: &IRect2, rec: UIRect, background: Color) -> Self {
        Self {
            is_hidden: false,
            rec_cache: rec.rect(container),
            rec,
            background,
        }
    }

    #[inline]
    pub fn update_rec(&mut self, container: &IRect2) {
        self.rec_cache = self.rec.rect(container);
    }

    #[inline]
    pub fn rect(&self) -> &IRect2 {
        &self.rec_cache
    }
}
