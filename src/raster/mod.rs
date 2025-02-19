use raylib::prelude::*;
use crate::layer::{LayerSettings, LayerType};

pub struct RasterTex {
    pub rtex: RenderTexture2D,
    /// Allows cropping
    pub src_rec: Rectangle,
    /// Allows stretching
    pub dest_rec: Rectangle,
    pub origin: Vector2,
    pub rotation: f32,
}

impl RasterTex {
    #[inline]
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        width: u32,
        height: u32,
        dest_rec: Rectangle,
    ) -> Result<Self, String> {
        Self::with_rotation(rl, thread, width, height, dest_rec, Vector2::zero(), 0.0)
    }

    pub fn with_rotation(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        width: u32,
        height: u32,
        dest_rec: Rectangle,
        origin: Vector2,
        rotation: f32,
    ) -> Result<Self, String> {
        assert!(width < i32::MAX as u32 && height < i32::MAX as u32);
        let rtex = rl.load_render_texture(thread, width, height)?;
        let src_rec = Rectangle { x: 0.0, y: 0.0, width: width as f32, height: height as f32 };
        Ok(Self { rtex, src_rec, dest_rec, origin, rotation })
    }

    #[inline]
    pub fn begin_texture_mode<'tex, 'd: 'tex, D: RaylibTextureModeExt>(&'tex mut self, d: &'d mut D, thread: &RaylibThread) -> RaylibTextureMode<'tex, D> {
        d.begin_texture_mode(thread, &mut self.rtex)
    }

    #[inline]
    pub fn draw(&self, d: &mut impl RaylibDraw) {
        self.draw_tinted(d, Color::WHITE);
    }

    #[inline]
    pub fn draw_tinted(&self, d: &mut impl RaylibDraw, tint: Color) {
        d.draw_texture_pro(&self.rtex, self.src_rec, self.dest_rec, self.origin, self.rotation, tint);
    }
}

pub struct Raster {
    pub settings: LayerSettings,
    pub texture: RasterTex,
}

impl Raster {
    pub fn new(settings: LayerSettings, texture: RasterTex) -> Self {
        Self {
            settings,
            texture,
        }
    }
}

impl LayerType for Raster {
    fn draw_rendered(&self, d: &mut impl RaylibDraw, _scratch_rtex: &mut [RenderTexture2D]) {
        self.texture.draw(d);
    }

    fn draw_selected(&self, _d: &mut impl RaylibDraw, _px_world_size: f32) {
        // todo
    }
}
