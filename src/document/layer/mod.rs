use raylib::prelude::*;
use crate::{appearance::Blending, raster::Raster, vector_path::VectorPath};

pub mod rc;
pub mod group;
pub mod tree;

use group::Group;

pub const INSET: f32 = 2.0;
pub const GAP: f32 = 2.0;
pub const INDENT: f32 = 6.0;
pub const THUMBNAIL_SIZE: f32 = 32.0;
pub const THUMBNAIL_INSET: f32 = 6.0;
pub const LAYER_HEIGHT: f32 = 2.0 * THUMBNAIL_INSET + THUMBNAIL_SIZE;
pub const LAYER_COLOR_WIDTH: f32 = 4.0;
pub const TEXT_FONT_SIZE: f32 = 10.0;
pub const EXPAND_COLLAPSE_SIZE: f32 = 10.0;

pub struct LayerSettings {
    pub slot_rec: Rectangle,
    pub thumbnail_rec: Rectangle,
    /// Name of the layer in the layers panel
    pub name: String,
    pub name_rec: Rectangle,
    /// Color of paths
    pub color: Color,
    pub color_rec: Rectangle,
    /// Skip in rendering
    pub is_hidden: bool,
    pub hide_button_rec: Rectangle,
    /// Disallow selection and editing
    pub is_locked: bool,
    pub lock_button_rec: Rectangle,
    /// Items move with layer
    pub is_group: bool,
    pub blend: Blending,
    pub artwork_bounds: Rectangle,
}

impl LayerSettings {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            slot_rec: Rectangle::default(),
            thumbnail_rec: Rectangle::default(),
            name,
            name_rec: Rectangle::default(),
            color,
            color_rec: Rectangle::default(),
            is_hidden: false,
            hide_button_rec: Rectangle::default(),
            is_locked: false,
            lock_button_rec: Rectangle::default(),
            is_group: false,
            blend: Blending::default(),
            artwork_bounds: Rectangle::default(),
        }
    }
}

pub enum Layer {
    Group(Group),
    Path(VectorPath),
    Raster(Raster),
}

pub trait LayerType {
    fn settings(&self) -> &LayerSettings;
    fn settings_mut(&mut self) -> &mut LayerSettings;

    /// Draw without helper visuals
    fn draw_rendered(&self, d: &mut impl RaylibDraw);

    /// Draw with helper visuals
    fn draw_selected(&self, d: &mut impl RaylibDraw, camera: &Camera2D, zoom_inv: f32);
}

impl LayerType for Layer {
    fn settings(&self) -> &LayerSettings {
        match self {
            Layer::Group(group) => group.settings(),
            Layer::Path(path) => path.settings(),
            Layer::Raster(raster) => raster.settings(),
        }
    }

    fn settings_mut(&mut self) -> &mut LayerSettings {
        match self {
            Layer::Group(group) => group.settings_mut(),
            Layer::Path(path) => path.settings_mut(),
            Layer::Raster(raster) => raster.settings_mut(),
        }
    }

    fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        if !self.settings().is_hidden {
            match self {
                Layer::Group(group) => group.draw_rendered(d),
                Layer::Path(path) => path.draw_rendered(d),
                Layer::Raster(raster) => raster.draw_rendered(d),
            }
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, camera: &Camera2D, zoom_inv: f32) {
        if !self.settings().is_hidden {
            match self {
                Layer::Group(group) => group.draw_selected(d, camera, zoom_inv),
                Layer::Path(path) => path.draw_selected(d, camera, zoom_inv),
                Layer::Raster(raster) => raster.draw_selected(d, camera, zoom_inv),
            }
        }
    }
}
