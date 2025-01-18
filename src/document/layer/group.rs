use raylib::prelude::*;
use super::{tree::LayerTree, LayerSettings, LayerType};

pub struct Group {
    pub settings: LayerSettings,
    pub items: LayerTree,
    pub is_expanded: bool,
    pub expand_button_rec: Rectangle,
}

impl Group {
    pub fn new(layer: LayerSettings) -> Self {
        Self {
            settings: layer,
            items: LayerTree::new(),
            is_expanded: false,
            expand_button_rec: Rectangle::default(),
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
        for layer in self.items.iter() {
            layer.read().expect("error handling not yet implemented").draw_rendered(d);
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw) {
        for layer in self.items.iter() {
            layer.read().expect("error handling not yet implemented").draw_selected(d);
        }
    }
}
