use raylib::prelude::*;
use super::{LayerSettings, LayerTree, LayerType};
use amylib::rc::*;

pub struct Group {
    pub settings: LayerSettings,
    pub items: LayerTree,
    pub is_expanded: bool,
}

impl Group {
    pub fn new(layer: LayerSettings) -> Self {
        Self {
            settings: layer,
            items: LayerTree::new(),
            is_expanded: false,
        }
    }
}

impl LayerType for Group {
    fn settings(&self) -> &LayerSettings {
        &self.settings
    }

    fn settings_mut(&mut self) -> &mut LayerSettings {
        &mut self.settings
    }

    fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        for item in self.items.iter() {
            item.read().draw_rendered(d);
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32) {
        for item in self.items.iter() {
            item.read().draw_selected(d, px_world_size);
        }
    }
}
