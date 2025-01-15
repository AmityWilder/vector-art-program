use std::{cell::RefCell, rc::Rc};
use raylib::prelude::*;

pub mod layer;
pub mod artboard;

use self::{layer::{Layer, LayerPanelTreeItemData}, artboard::ArtBoard};

pub struct Document {
    pub title: String,
    pub camera: Camera2D,
    pub paper_color: Color,
    pub layers: Vec<Rc<RefCell<Layer>>>,
    pub art_boards: Vec<ArtBoard>,
    pub current_layer: Option<Rc<RefCell<Layer>>>,
}

impl Document {
    pub fn new(title: String, width: f32, height: f32) -> Self {
        Self {
            title,
            camera: Camera2D {
                offset: Vector2::zero(),
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            },
            paper_color: Color::GRAY,
            layers: Vec::new(),
            art_boards: vec![ArtBoard::new("artboard 0".to_string(), rrect(0.0, 0.0, width, height))],
            current_layer: None,
        }
    }

    pub fn pan(&mut self, v: Vector2) {
        self.camera.target -= v / self.camera.zoom;
    }

    pub fn zoom(&mut self, amount: f32) {
        const ZOOM_SPEED: f32 = 1.5;
        if amount > 0.0 && self.camera.zoom < 16.0 {
            self.camera.zoom *= ZOOM_SPEED;
        } else if amount < 0.0 && self.camera.zoom > 0.125 {
            self.camera.zoom /= ZOOM_SPEED;
        }
    }

    pub fn for_each_layer_tree_item<T>(&mut self, panel: &Rectangle, mut f: impl FnMut(LayerPanelTreeItemData) -> Option<T>) -> Option<T> {
        const INSET: f32 = 2.0;

        let mut y = panel.y + INSET;
        let x = panel.x + INSET;
        let width = panel.width - INSET * 2.0;

        for layer in self.layers.iter_mut().rev() {
            let result = layer.borrow_mut().for_each_layer_tree_item_internal(panel, &mut y, x, width, &mut f);
            if result.is_some() { return result; }
        }

        None
    }

    pub fn draw_layer_tree(&mut self, d: &mut impl RaylibDraw, panel: &Rectangle) {
        let mut d = d.begin_scissor_mode(panel.x as i32, panel.y as i32, panel.width as i32, panel.height as i32);
        d.draw_rectangle_rec(*panel, Color::new(24,24,24,255));
        self.for_each_layer_tree_item(panel, |data| -> Option<()> {
            match data {
                LayerPanelTreeItemData::Layer {
                    slot,
                    color_rec,
                    thumbnail_rec,
                    name_rec,
                    expand_collapse_rec,
                    layer,
                } => {
                    d.draw_rectangle_rec(slot, Color::new(32,32,32,255));
                    d.draw_rectangle_rec(color_rec, layer.color);
                    d.draw_rectangle_rec(thumbnail_rec, Color::GRAY);
                    d.draw_text(&layer.name, name_rec.x as i32, name_rec.y as i32, 10, Color::new(200,200,200,255));

                    // expand icon
                    {
                        let [p0, p1, p2] = if layer.is_expanded {
                            [
                                Vector2::new(expand_collapse_rec.x, expand_collapse_rec.y),
                                Vector2::new(expand_collapse_rec.x + 5.0, expand_collapse_rec.y + 6.0),
                                Vector2::new(expand_collapse_rec.x + expand_collapse_rec.height, expand_collapse_rec.y),
                            ]
                        } else {
                            [
                                Vector2::new(expand_collapse_rec.x, expand_collapse_rec.y),
                                Vector2::new(expand_collapse_rec.x, expand_collapse_rec.y + expand_collapse_rec.height),
                                Vector2::new(expand_collapse_rec.x + 6.0, expand_collapse_rec.y + 5.0),
                            ]
                        };
                        d.draw_triangle(p0, p1, p2, Color::new(200,200,200,255));
                    }
                }

                LayerPanelTreeItemData::Path {
                    slot,
                    thumbnail_rec,
                    path: _,
                } => {
                    d.draw_rectangle_rec(slot, Color::new(32,32,32,255));
                    d.draw_rectangle_rec(thumbnail_rec, Color::GRAY);
                }

                LayerPanelTreeItemData::Bitmap {
                    slot,
                    thumbnail_rec,
                    bitmap: _,
                } => {
                    d.draw_rectangle_rec(slot, Color::new(32,32,32,255));
                    d.draw_rectangle_rec(thumbnail_rec, Color::GRAY);
                }
            }

            None
        });
    }
}
