use std::path::PathBuf;
use amygui::panel::Panel;
use amymath::prelude::{*, Vector2};
use layer::{Layer, LayerTree};
use crate::{appearance::Appearance, raster::{Raster, RasterTex}, vector_path::VectorPath};
use amylib::rc::prelude::*;
use raylib::prelude::{*, Vector2 as RlVector2};

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
                offset: RlVector2::zero(),
                target: RlVector2::zero(),
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

    pub fn create_artboard(&mut self, name: Option<String>, pos: Option<IVector2>, size: IVector2) {
        const AUTO_GAP: i32 = 10;
        let name = name.unwrap_or_else(|| self.auto_artboard_name());
        let pos = pos.unwrap_or_else(|| self.artboards.last().map_or(IVector2::ZERO, |b| b.rect.max + IVector2 { x: AUTO_GAP, y: 0 }));
        self.artboards.push(ArtBoard {
            name,
            rect: IRect2 {
                min: pos,
                max: pos + size,
            },
        });
    }

    pub fn gen_layer_settings(&mut self, name: Option<String>, color: Option<Color>) -> LayerSettings {
        let name  =  name.unwrap_or_else(|| self.auto_layer_name ());
        let color = color.unwrap_or_else(|| self.auto_layer_color());
        LayerSettings::new(name, color)
    }

    pub fn create_path(&mut self, name: Option<String>, color: Option<Color>, appearance: Appearance) -> &mut StrongMut<VectorPath> {
        let settings = self.gen_layer_settings(name, color);
        self.layers.push(Layer::Path(StrongMut::new(VectorPath::new(settings, appearance))));
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
        for (layer, recs) in self.ui_iter(panel_rec, panel_rec.min.y) {
            let name = match layer {
                Layer::Group(group) => &group.settings.name,
                Layer::Path(path) => &path.read().settings.name,
                Layer::Raster(raster) => &raster.read().settings.name,
            };
            let color = layer.settings().color;
            d.draw_rectangle_irect2(&recs.slot, SLOT_COLOR);
            d.draw_rectangle_irect2(&recs.color, color);
            d.draw_rectangle_irect2(&recs.thumbnail, Color::GRAY);
            d.draw_text(name, recs.name.min.x, recs.name.min.y, 10, TEXT_COLOR);
            // expand icon
            if let Layer::Group(Group { is_expanded, .. }) = layer {
                let expand_button_rec = recs.expand_button.expect("group should always have expand button");
                let IRect2 {
                    min: IVector2 { x: xmin, y: ymin },
                    max: IVector2 { x: xmax, y: ymax },
                } = expand_button_rec;
                let min = RlVector2 { x: xmin as f32, y: ymin as f32 };
                let max = RlVector2 { x: xmax as f32, y: ymax as f32 };
                let p0 = min;
                let [p1, p2] = if *is_expanded {
                    [
                        Vector2::new(min.x + 5.0, min.y + 6.0),
                        Vector2::new(max.x, min.y),
                    ]
                } else {
                    [
                        Vector2::new(min.x, max.y),
                        Vector2::new(min.x + 6.0, min.y + 5.0),
                    ]
                };
                d.draw_triangle(p0, p1, p2, TEXT_COLOR);
            }
        }
    }
}
