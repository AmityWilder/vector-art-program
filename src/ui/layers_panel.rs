use amygui::panel::Panel;
use raylib::prelude::*;
use crate::{layer::{group::Group, Layer}, Document};

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
            for (layer, recs) in document.ui_iter_mut(self.panel.rect(), self.panel.rect().ymin) {
                if recs.slot_rec.is_overlapping_point(mouse_screen_pos.x as i32, mouse_screen_pos.y as i32) {
                    if let Layer::Group(Group { is_expanded, .. }) = layer {
                        let expand_button_rec = recs.expand_button_rec.expect("group should always have expand button");
                        if expand_button_rec.is_overlapping_point(mouse_screen_pos.x as i32, mouse_screen_pos.y as i32) {
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
