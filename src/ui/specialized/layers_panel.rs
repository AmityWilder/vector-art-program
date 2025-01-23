use raylib::prelude::*;
use crate::{layer::{group::Group, tree::TreeIterDir, Layer, LayerType}, Document};
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
            document.layers
                .tree_iter_mut(TreeIterDir::ForeToBack, |group| group.is_expanded)
                .find_map(|(mut layer, _depth)| {
                    let mut layer = layer.write();
                    if layer.settings().slot_rec.check_collision_point_rec(mouse_screen_pos) {
                        if let Layer::Group(Group { is_expanded, expand_button_rec, .. }) = &mut *layer {
                            if expand_button_rec.check_collision_point_rec(mouse_screen_pos) {
                                *is_expanded = !*is_expanded;
                                return Some(());
                            }
                        }
                    }
                    None
                });
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document) {
        document.draw_layer_tree(d, &self.panel);
    }
}
