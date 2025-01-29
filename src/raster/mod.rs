use amylib::rc::*;
use raylib::prelude::*;

use crate::layer::{LayerSettings, LayerType};

pub struct Raster {
    pub settings: StrongMut<LayerSettings>,
    pub texture: Option<RenderTexture2D>,
}

impl Raster {
    pub fn new(settings: &StrongMut<LayerSettings>) -> Self {
        Self {
            settings: settings.clone_mut(),
            texture: None,
        }
    }
}

impl LayerType for Raster {
    fn draw_rendered(&self, _d: &mut impl RaylibDraw) {
        // todo
    }

    fn draw_selected(&self, _d: &mut impl RaylibDraw, _px_world_size: f32) {
        // todo
    }
}
