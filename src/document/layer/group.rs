use amylib::rc::*;
use raylib::prelude::*;
use super::{LayerSettings, LayerTree, LayerType};

pub struct Group {
    pub settings: StrongMut<LayerSettings>,
    pub items: LayerTree,
    pub is_expanded: bool,
}

impl Group {
    pub fn new(settings: &StrongMut<LayerSettings>) -> Self {
        Self {
            settings: settings.clone_mut(),
            items: LayerTree::new(),
            is_expanded: false,
        }
    }
}

impl LayerType for Group {
    fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        for item in self.items.shallow_iter() {
            item.draw_rendered(d);
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32) {
        for item in self.items.shallow_iter() {
            item.draw_selected(d, px_world_size);
        }
    }
}
