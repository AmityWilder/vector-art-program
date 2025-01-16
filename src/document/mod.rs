use std::{cell::RefCell, rc::Rc};
use layer::{Group, LayerContent};
use raylib::prelude::*;

pub mod layer;
pub mod artboard;

use crate::ui::panel::Panel;
use self::{layer::Layer, artboard::ArtBoard};

const DEFAULT_LAYER_COLORS: [Color; 10] = [
    Color::BLUEVIOLET,
    Color::OLIVE,
    Color::ORANGERED,
    Color::LAVENDER,
    Color::CYAN,
    Color::FORESTGREEN,
    Color::FUCHSIA,
    Color::MAROON,
    Color::SEAGREEN,
    Color::SLATEBLUE,
];

pub struct Document {
    pub title: String,
    pub camera: Camera2D,
    pub paper_color: Color,
    pub layers: Vec<Rc<RefCell<Layer>>>,
    pub art_boards: Vec<ArtBoard>,
    pub current_layer: Option<Rc<RefCell<Layer>>>,
    layer_color_acc: usize,
    layer_name_acc: usize,
}

impl Document {
    fn next_auto_color(&mut self) -> Color {
        let idx = self.layer_color_acc;
        self.layer_color_acc = if idx + 1 >= DEFAULT_LAYER_COLORS.len() { 0 } else { idx + 1 };
        DEFAULT_LAYER_COLORS[idx]
    }
    fn next_auto_name(&mut self) -> String {
        let idx = self.layer_name_acc;
        self.layer_name_acc = idx.wrapping_add(1);
        format!("layer {idx}")
    }

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
            layer_color_acc: 0,
            layer_name_acc: 0,
        }
    }

    pub fn create_layer(&mut self, name: Option<String>, color: Option<Color>, content: LayerContent) -> Rc<RefCell<Layer>> {
        let name = name.unwrap_or_else(|| self.next_auto_name());
        let color = color.unwrap_or_else(|| self.next_auto_color());
        let layer = Rc::new(RefCell::new(Layer::new(name, color, content)));
        self.layers.push(layer.clone());
        layer
    }

    pub fn group_layers(&mut self) {
        todo!("layer grouping")
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

    pub fn foreach_layer_tree_item<T>(&self, mut f: impl FnMut(&Layer, usize) -> Option<T>) -> Option<T> {
        fn recursive<T>(layers: &Vec<Rc<RefCell<Layer>>>, depth: usize, f: &mut impl FnMut(&Layer, usize) -> Option<T>) -> Option<T> {
            for layer in layers.iter().rev() {
                let layer = layer.borrow();
                f(&layer, depth);
                if let LayerContent::Group(Group { group, is_expanded: true, .. }) = &layer.content {
                    let result = recursive(group, depth + 1, f);
                    if result.is_some() { return result; }
                }
            }
            None
        }

        recursive(&self.layers, 0, &mut f)
    }

    pub fn foreach_layer_tree_item_mut<T>(&mut self, mut f: impl FnMut(&mut Layer, usize) -> Option<T>) -> Option<T> {
        fn recursive<T>(layers: &mut Vec<Rc<RefCell<Layer>>>, depth: usize, f: &mut impl FnMut(&mut Layer, usize) -> Option<T>) -> Option<T> {
            for layer in layers.iter().rev() {
                let mut layer = layer.borrow_mut();
                f(&mut layer, depth);
                if let LayerContent::Group(Group { group, is_expanded: true, .. }) = &mut layer.content {
                    let result = recursive(group, depth + 1, f);
                    if result.is_some() { return result; }
                }
            }
            None
        }

        recursive(&mut self.layers, 0, &mut f)
    }

    pub fn update_layer_tree_recs(&mut self, container: &Rectangle) {
        let mut top = container.y + layer::INSET;
        self.foreach_layer_tree_item_mut(|layer, depth| -> Option<()> {
            layer.update_ui_recs(container, top, depth);
            top += layer::LAYER_HEIGHT + layer::GAP;
            None
        });
    }

    /// Assumes `update_layer_tree_recs` is up to date
    pub fn draw_layer_tree(&self, d: &mut impl RaylibDraw, panel: &Panel) {
        let panel_rec: Rectangle = panel.rec_cache.into();
        let mut d = d.begin_scissor_mode(panel_rec.x as i32, panel_rec.y as i32, panel_rec.width as i32, panel_rec.height as i32);
        d.draw_rectangle_rec(panel_rec, panel.background);
        self.foreach_layer_tree_item(|layer, _depth| -> Option<()> {
            d.draw_rectangle_rec(layer.slot_rec, Color::new(32,32,32,255));
            d.draw_rectangle_rec(layer.color_rec, layer.color);
            d.draw_rectangle_rec(layer.thumbnail_rec, Color::GRAY);
            d.draw_text(&layer.name, layer.name_rec.x as i32, layer.name_rec.y as i32, 10, Color::new(200,200,200,255));
            // expand icon
            if let LayerContent::Group(Group { is_expanded, expand_button_rec, .. }) = &layer.content {
                let p0 = Vector2::new(expand_button_rec.x, expand_button_rec.y);
                let [p1, p2] = if *is_expanded {
                    [
                        Vector2::new(
                            expand_button_rec.x + 5.0,
                            expand_button_rec.y + 6.0,
                        ),
                        Vector2::new(
                            expand_button_rec.x + expand_button_rec.height,
                            expand_button_rec.y,
                        ),
                    ]
                } else {
                    [
                        Vector2::new(
                            expand_button_rec.x,
                            expand_button_rec.y + expand_button_rec.height,
                        ),
                        Vector2::new(
                            expand_button_rec.x + 6.0,
                            expand_button_rec.y + 5.0,
                        ),
                    ]
                };
                d.draw_triangle(p0, p1, p2, Color::new(200,200,200,255));
            }
            None
        });
    }
}
