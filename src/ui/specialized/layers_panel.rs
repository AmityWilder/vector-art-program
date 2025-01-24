use raylib::prelude::*;
use crate::{layer::{group::Group, Layer}, Document};
use super::panel::Panel;

pub struct LayersPanel {
    pub panel: Panel,
}

impl LayersPanel {
    pub const fn new(panel: Panel) -> Self {
        Self {
            panel,
        }
    }

    pub fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_screen_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            for (mut layer, _depth, recs) in document.layers.ui_iter_mut(self.panel.rec_cache, self.panel.rec_cache.ymin) {
                if recs.slot_rec.check_collision_point_rec(mouse_screen_pos) {
                    let mut layer = layer.write();
                    if let Layer::Group(Group { is_expanded, .. }) = &mut *layer {
                        let expand_button_rec = recs.expand_button_rec.expect("group should always have expand button");
                        if expand_button_rec.check_collision_point_rec(mouse_screen_pos) {
                            *is_expanded = !*is_expanded;
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document) {
        document.draw_layer_tree(d, &self.panel);
    }
}
