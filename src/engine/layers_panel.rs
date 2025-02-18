use amygui::panel::Panel;
use amymath::prelude::{Vector2, IVector2};
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
        let mouse_pos =
            #[allow(clippy::cast_possible_truncation, reason = "value ceil'd before cast")]
            IVector2::new(mouse_screen_pos.x.ceil() as i32, mouse_screen_pos.y.ceil() as i32);

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            for (layer, recs) in document.ui_iter_mut(self.panel.rect(), self.panel.rect().min.y) {
                if recs.slot.contains_v(&mouse_pos) {
                    if let Layer::Group(Group { is_expanded, .. }) = layer {
                        let expand_button_rec = recs.expand_button.expect("group should always have expand button");
                        if expand_button_rec.contains_v(&mouse_pos) {
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
