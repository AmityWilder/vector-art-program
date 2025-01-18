use std::sync::{Arc, RwLock};
use layer::{group::Group, tree::{LayerTree, LayerIterDir}, Layer, LayerSettings, LayerType, StrongLayer, WeakLayer};
use raylib::prelude::*;

pub mod layer;
pub mod artboard;
pub mod serialize;

use crate::{raster::Raster, ui::panel::Panel, vector_path::VectorPath};
use self::artboard::ArtBoard;

const DEFAULT_LAYER_COLORS: [Color; 10] = [
    Color::BLUEVIOLET,
    Color::ORANGERED,
    Color::CYAN,
    Color::LAVENDER,
    Color::FORESTGREEN,
    Color::FUCHSIA,
    Color::OLIVE,
    Color::MAROON,
    Color::SEAGREEN,
    Color::SLATEBLUE,
];

pub struct Document {
    pub title: String,
    pub camera: Camera2D,
    pub paper_color: Color,
    pub layers: LayerTree,
    pub selection: Vec<WeakLayer>,
    pub artboards: Vec<ArtBoard>,
    pub active_artboard: Option<usize>,
    layer_color_acc: usize,
    layer_name_acc: usize,
    artboard_name_acc: usize,
}

impl Document {
    fn auto_layer_color(&mut self) -> Color {
        let idx = self.layer_color_acc;
        self.layer_color_acc = if idx + 1 >= DEFAULT_LAYER_COLORS.len() { 0 } else { idx + 1 };
        DEFAULT_LAYER_COLORS[idx]
    }
    fn auto_layer_name(&mut self) -> String {
        let idx = self.layer_name_acc;
        self.layer_name_acc = idx.wrapping_add(1);
        format!("layer {idx}")
    }
    fn auto_artboard_name(&mut self) -> String {
        let idx = self.artboard_name_acc;
        self.artboard_name_acc = idx.wrapping_add(1);
        format!("artboard {idx}")
    }

    pub fn new() -> Self {
        Self {
            title: "untitled".to_string(),
            camera: Camera2D {
                offset: Vector2::zero(),
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            },
            paper_color: Color::GRAY,
            layers: LayerTree::new(),
            selection: Vec::new(),
            artboards: Vec::new(),
            active_artboard: None,
            layer_color_acc: 0,
            layer_name_acc: 0,
            artboard_name_acc: 0,
        }
    }

    pub fn create_artboard(&mut self, name: Option<String>, xy: Option<(f32, f32)>, width: f32, height: f32) {
        let name = name.unwrap_or_else(|| self.auto_artboard_name());
        let (x, y) = xy.unwrap_or_else(|| self.artboards.last().map_or((0.0, 0.0), |b| (b.rect.x + b.rect.width, b.rect.y + b.rect.height)));
        self.artboards.push(ArtBoard {
            name,
            rect: Rectangle::new(x, y, width, height),
        });
    }

    pub fn gen_layer_settings(&mut self, name: Option<String>, color: Option<Color>) -> LayerSettings {
        let name  =  name.unwrap_or_else(|| self.auto_layer_name ());
        let color = color.unwrap_or_else(|| self.auto_layer_color());
        LayerSettings::new(name, color)
    }

    pub fn create_path(&mut self, name: Option<String>, color: Option<Color>) -> StrongLayer {
        let path = Arc::new(RwLock::new(Layer::Path(VectorPath::new(self.gen_layer_settings(name, color)))));
        self.layers.push(path.clone());
        path
    }

    pub fn create_raster(&mut self, name: Option<String>, color: Option<Color>) -> StrongLayer {
        let path = Arc::new(RwLock::new(Layer::Raster(Raster::new(self.gen_layer_settings(name, color)))));
        self.layers.push(path.clone());
        path
    }

    pub fn create_group(&mut self, name: Option<String>, color: Option<Color>) -> StrongLayer {
        let path = Arc::new(RwLock::new(Layer::Group(Group::new(self.gen_layer_settings(name, color)))));
        self.layers.push(path.clone());
        path
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

    pub fn update_layer_tree_recs(&mut self, container: &Rectangle) {
        self.layers.update_ui_recs(container, container.y + layer::INSET);
    }

    /// Assumes `update_layer_tree_recs` is up to date
    pub fn draw_layer_tree(&self, d: &mut impl RaylibDraw, panel: &Panel) {
        let panel_rec: Rectangle = panel.rec_cache.into();
        let mut d = d.begin_scissor_mode(panel_rec.x as i32, panel_rec.y as i32, panel_rec.width as i32, panel_rec.height as i32);
        d.draw_rectangle_rec(panel_rec, panel.background);
        for (layer, _depth) in self.layers.tree_iter(LayerIterDir::TopToBot, |group| group.is_expanded) {
            let layer = layer.read().expect("error handling not yet implemented");
            d.draw_rectangle_rec(layer.settings().slot_rec, Color::new(32,32,32,255));
            d.draw_rectangle_rec(layer.settings().color_rec, layer.settings().color);
            d.draw_rectangle_rec(layer.settings().thumbnail_rec, Color::GRAY);
            d.draw_text(&layer.settings().name, layer.settings().name_rec.x as i32, layer.settings().name_rec.y as i32, 10, Color::new(200,200,200,255));
            // expand icon
            if let Layer::Group(Group { is_expanded, expand_button_rec, .. }) = &*layer {
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
        }
    }
}
