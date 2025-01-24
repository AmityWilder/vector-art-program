use std::path::PathBuf;
use crate::{raster::Raster, stack::{StackAdaptor, VecStack}, ui::panel::Panel, vector_path::VectorPath};
use layer::rc::StrongRef;
use raylib::prelude::*;

pub mod layer;
pub mod artboard;
pub mod serialize;

use self::{
    artboard::{
        ArtBoard,
        IntRect2,
    },
    layer::{
        group::Group,
        rc::{StrongMut, WeakMut},
        tree::LayerTree,
        Layer,
        LayerSettings,
        LayerType,
    }
};

pub trait Change {
    fn unapply(&self, document: &mut Document);
    fn apply  (&self, document: &mut Document);
}

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
    pub path: Option<PathBuf>,
    pub title: String,
    pub camera: Camera2D,
    pub paper_color: Color,
    pub layers: LayerTree,
    pub selection: Vec<WeakMut<Layer>>,
    pub artboards: Vec<ArtBoard>,
    pub active_artboard: Option<usize>,

    history: VecStack<Box<dyn Change>>,
    future:  VecStack<Box<dyn Change>>,

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
            path: None,
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

            history: VecStack::with_capacity(128),
            future:  VecStack::with_capacity(128),

            layer_color_acc: 0,
            layer_name_acc: 0,
            artboard_name_acc: 0,
        }
    }

    /// Apply a change that can be undone/redone and add it to the history
    pub fn apply(&mut self, change: Box<dyn Change>) {
        self.future.clear();
        change.apply(self);
        self.history.push_no_resize(change);
    }

    pub fn undo(&mut self) {
        if let Some(change) = self.history.pop() {
            change.unapply(self);
            self.future.push_no_resize(change);
        }
    }

    pub fn redo(&mut self) {
        if let Some(change) = self.future.pop() {
            change.apply(self);
            self.history.push_no_resize(change);
        }
    }

    pub fn create_artboard(&mut self, name: Option<String>, xy: Option<(i32, i32)>, width: i32, height: i32) {
        let name = name.unwrap_or_else(|| self.auto_artboard_name());
        let (x, y) = xy.unwrap_or_else(|| self.artboards.last().map_or((0, 0), |b| (b.rect.x + b.rect.width + 10, b.rect.y)));
        self.artboards.push(ArtBoard {
            name,
            rect: IntRect2 { x, y, width, height },
        });
    }

    pub fn gen_layer_settings(&mut self, name: Option<String>, color: Option<Color>) -> LayerSettings {
        let name  =  name.unwrap_or_else(|| self.auto_layer_name ());
        let color = color.unwrap_or_else(|| self.auto_layer_color());
        LayerSettings::new(name, color)
    }

    pub fn create_path(&mut self, name: Option<String>, color: Option<Color>) -> StrongMut<Layer> {
        let path = StrongMut::new(Layer::Path(VectorPath::new(self.gen_layer_settings(name, color))));
        self.layers.push(path.clone_mut());
        path
    }

    pub fn create_raster(&mut self, name: Option<String>, color: Option<Color>) -> StrongMut<Layer> {
        let path = StrongMut::new(Layer::Raster(Raster::new(self.gen_layer_settings(name, color))));
        self.layers.push(path.clone_mut());
        path
    }

    pub fn create_group(&mut self, name: Option<String>, color: Option<Color>) -> StrongMut<Layer> {
        let path = StrongMut::new(Layer::Group(Group::new(self.gen_layer_settings(name, color))));
        self.layers.push(path.clone_mut());
        path
    }

    /// Assumes `update_layer_tree_recs` is up to date
    pub fn draw_layer_tree(&self, d: &mut impl RaylibDraw, panel: &Panel) {
        const SLOT_COLOR: Color = Color::new(32,32,32,255);
        const TEXT_COLOR: Color = Color::new(200,200,200,255);
        let panel_rec: Rectangle = panel.rec_cache.into();
        let mut d = d.begin_scissor_mode(panel_rec.x as i32, panel_rec.y as i32, panel_rec.width as i32, panel_rec.height as i32);
        d.draw_rectangle_rec(panel_rec, panel.background);
        for (layer, _depth, recs) in self.layers.ui_iter(panel.rec_cache, panel.rec_cache.ymin) {
            let layer = layer.read();
            let settings = layer.settings();
            let name = settings.name.as_str();
            d.draw_rectangle_rec(recs.slot_rec, SLOT_COLOR);
            d.draw_rectangle_rec(recs.color_rec, layer.settings().color);
            d.draw_rectangle_rec(recs.thumbnail_rec, Color::GRAY);
            d.draw_text(name, recs.name_rec.x as i32, recs.name_rec.y as i32, 10, TEXT_COLOR);
            // expand icon
            if let Layer::Group(Group { is_expanded, .. }) = &*layer {
                let expand_button_rec = recs.expand_button_rec.expect("group should always have expand button");
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
                d.draw_triangle(p0, p1, p2, TEXT_COLOR);
            }
        }
    }
}
