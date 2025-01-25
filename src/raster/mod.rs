use raylib::prelude::*;

use crate::layer::{LayerSettings, LayerType};

pub struct Raster {
    pub settings: LayerSettings,
    pub texture: Option<RenderTexture2D>,
}

impl Raster {
    pub fn new(settings: LayerSettings) -> Self {
        Self {
            settings,
            texture: None,
        }
    }
}

impl LayerType for Raster {
    fn settings(&self) -> &LayerSettings {
        &self.settings
    }

    fn settings_mut(&mut self) -> &mut LayerSettings {
        &mut self.settings
    }

    fn draw_rendered(&self, _d: &mut impl RaylibDraw) {
        // todo
    }

    fn draw_selected(&self, _d: &mut impl RaylibDraw, _px_world_size: f32) {
        // todo
    }
}
