use std::{fmt, path::PathBuf};
use amygui::panel::Panel;
use amymath::prelude::*;
use layer::{Layer, LayerTree};
use crate::{raster::{Raster, RasterTex}, vector_path::VectorPath};
use amylib::rc::prelude::*;
use raylib::prelude::*;

pub mod layer;
pub mod artboard;
pub mod serialize;

use self::{
    artboard::ArtBoard,
    layer::{
        group::Group,
        LayerSettings,
    }
};

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
            artboards: Vec::new(),
            active_artboard: None,
            layer_color_acc: 0,
            layer_name_acc: 0,
            artboard_name_acc: 0,
        }
    }

    pub fn create_artboard(&mut self, name: Option<String>, xy: Option<(i32, i32)>, width: i32, height: i32) {
        const AUTO_GAP: i32 = 10;
        let name = name.unwrap_or_else(|| self.auto_artboard_name());
        let (xmin, ymin) = xy.unwrap_or_else(|| self.artboards.last().map_or((0, 0), |b| (b.rect.xmax + AUTO_GAP, b.rect.ymin)));
        self.artboards.push(ArtBoard {
            name,
            rect: IRect2 {
                xmin,
                ymin,
                xmax: xmin + width,
                ymax: ymin + height,
            },
        });
    }

    pub fn gen_layer_settings(&mut self, name: Option<String>, color: Option<Color>) -> LayerSettings {
        let name  =  name.unwrap_or_else(|| self.auto_layer_name ());
        let color = color.unwrap_or_else(|| self.auto_layer_color());
        LayerSettings::new(name, color)
    }

    pub fn create_path(&mut self, name: Option<String>, color: Option<Color>) -> &mut StrongMut<VectorPath> {
        let settings = self.gen_layer_settings(name, color);
        self.layers.push(Layer::Path(StrongMut::new(VectorPath::new(settings))));
        match self.layers.last_mut() {
            Some(Layer::Path(path)) => path,
            _ => unreachable!("vector path layer should exist when one is pushed"),
        }
    }

    pub fn create_raster(&mut self, name: Option<String>, color: Option<Color>, texture: RasterTex) -> &mut StrongMut<Raster> {
        let settings = self.gen_layer_settings(name, color);
        self.layers.push(Layer::Raster(StrongMut::new(Raster::new(settings, texture))));
        match self.layers.last_mut() {
            Some(Layer::Raster(raster)) => raster,
            _ => unreachable!("raster layer should exist when one is pushed"),
        }
    }

    pub fn create_group(&mut self, name: Option<String>, color: Option<Color>) -> &mut Group {
        let settings = self.gen_layer_settings(name, color);
        self.layers.push(Layer::Group(Group::new(settings)));
        match self.layers.last_mut() {
            Some(Layer::Group(group)) => group,
            _ => unreachable!("group layer should exist when one is pushed"),
        }
    }

    /// Assumes `update_layer_tree_recs` is up to date
    pub fn draw_layer_tree(&self, d: &mut impl RaylibDraw, panel: &Panel) {
        const SLOT_COLOR: Color = Color::new(32,32,32,255);
        const TEXT_COLOR: Color = Color::new(200,200,200,255);
        let panel_rec = panel.rect();
        let mut d = d.begin_scissor_mode_irect2(panel_rec);
        d.draw_rectangle_irect2(panel_rec, panel.background);
        for (layer, recs) in self.ui_iter(panel_rec, panel_rec.ymin) {
            let name = match layer {
                Layer::Group(group) => &group.settings.name,
                Layer::Path(path) => &path.read().settings.name,
                Layer::Raster(raster) => &raster.read().settings.name,
            };
            let color = layer.settings().color;
            d.draw_rectangle_rec(recs.slot, SLOT_COLOR);
            d.draw_rectangle_rec(recs.color, color);
            d.draw_rectangle_rec(recs.thumbnail, Color::GRAY);
            d.draw_text(name, recs.name.xmin, recs.name.ymin, 10, TEXT_COLOR);
            // expand icon
            if let Layer::Group(Group { is_expanded, .. }) = layer {
                let expand_button_rec = recs.expand_button.expect("group should always have expand button");
                let Rect2 { xmin, ymin, xmax, ymax } = expand_button_rec.into();
                let p0 = Vector2::new(xmin, ymin);
                let [p1, p2] = if *is_expanded {
                    [
                        Vector2::new(xmin + 5.0, ymin + 6.0),
                        Vector2::new(xmax, ymin),
                    ]
                } else {
                    [
                        Vector2::new(xmin, ymax),
                        Vector2::new(xmin + 6.0, ymin + 5.0),
                    ]
                };
                d.draw_triangle(p0, p1, p2, TEXT_COLOR);
            }
        }
    }
}
