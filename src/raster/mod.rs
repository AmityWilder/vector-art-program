use amymath::prelude::IRect2;
use raylib::prelude::*;
use crate::layer::{LayerSettings, LayerType};

pub mod raster_brush;

pub struct RasterTex {
    rtex: RenderTexture2D,
    /// Allows cropping
    pub src_rec: IRect2,
    /// Allows stretching
    pub dest_rec: Rectangle,
    pub origin: Vector2,
    pub rotation: f32,
}

impl RasterTex {
    #[inline]
    pub fn new(d: &mut RaylibHandle, thread: &RaylibThread, width: u32, height: u32, dest_rec: Rectangle) -> Result<Self, String> {
        Self::with_rotation(d, thread, width, height, dest_rec, Vector2::zero(), 0.0)
    }

    pub fn with_rotation(
        d: &mut RaylibHandle,
        thread: &RaylibThread,
        width: u32,
        height: u32,
        dest_rec: Rectangle,
        origin: Vector2,
        rotation: f32,
    ) -> Result<Self, String> {
        assert!(width < i32::MAX as u32 && height < i32::MAX as u32);
        let rtex = d.load_render_texture(thread, width, height)?;
        let src_rec = IRect2 {
            xmin: 0,
            ymin: 0,
            xmax: width  as i32,
            ymax: height as i32,
        };
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

    pub fn draw_tinted(&self, d: &mut impl RaylibDraw, tint: Color) {
        let src = Rectangle {
            x: self.src_rec.xmin as f32,
            y: self.src_rec.ymin as f32,
            width: self.src_rec.width() as f32,
            height: self.src_rec.height() as f32,
        };
        d.draw_texture_pro(&self.rtex, src, self.dest_rec, self.origin, self.rotation, tint);
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
    fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        self.texture.draw(d);
    }

    fn draw_selected(&self, _d: &mut impl RaylibDraw, _px_world_size: f32) {
        // todo
    }
}
