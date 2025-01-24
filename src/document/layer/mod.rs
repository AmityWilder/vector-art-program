use raylib::prelude::*;
use crate::{appearance::Blending, raster::Raster, vector_path::VectorPath};

pub mod ui_iter;
pub mod rc;
pub mod group;
pub mod tree;

use group::Group;

pub struct LayerSettings {
    /// Name of the layer in the layers panel
    pub name: String,
    /// Color of paths
    pub color: Color,
    /// Skip in rendering
    pub is_hidden: bool,
    /// Disallow selection and editing
    pub is_locked: bool,
    /// Items move with layer
    pub is_group: bool,
    pub blend: Blending,
    pub artwork_bounds: Rectangle,
}

impl LayerSettings {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            name,
            color,
            is_hidden: false,
            is_locked: false,
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
