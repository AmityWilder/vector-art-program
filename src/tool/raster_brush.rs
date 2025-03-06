use amylib::prelude::RoundToInt;
use amymath::prelude::{*, Vector2};
use raylib::prelude::*;
use crate::{appearance::{Appearance, Blending}, document::Document, editor::Editor, shaders::ShaderTable, tool::ToolType};
use super::Raster;

pub enum Pattern {
    /// The entire stroke is one color.
    Solid(Color),

    /// Brush stamps out clones of the image at fixed intervals.
    Image(f32, Texture2D),

    /// Brush marks out a mask, revealing a repeating,
    /// canvas-aligned image within the stroke.
    Chowder(Texture2D),
}

pub struct Stroke {
    pub blend: Blending,
    pub pattern: Pattern,
    pub thick: f32,
}

pub struct RasterBrush<'a> {
    /// Flushes to target when stroke finishes.
    ///
    /// This is not a performance optimization.
    /// This is to hide the discrete nature of brush stroke ticks.
    buffer: RenderTexture2D,
    pub shader: Option<Shader>,
    target: &'a mut Raster,
    mouse_prev: Option<Vector2>,
    mouse_curr: Vector2,
    pub stroke: Stroke,
}

impl<'a> RasterBrush<'a> {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        shader: Option<Shader>,
        target: &'a mut Raster,
        stroke: Stroke,
    ) -> Result<Self, String> {
        let (width, height) = {
            let texture = &target.texture.rtex;
            (texture.width() as u32, texture.height() as u32)
        };
        let buffer = rl.load_render_texture(thread, width, height)?;
        Ok(Self { buffer, shader, target, mouse_curr: Vector2::default(), mouse_prev: None, stroke })
    }

    fn draw_buffer(d: &mut impl RaylibDraw, buffer: &RenderTexture2D, shader: Option<&Shader>, src_rec: Rectangle, dest_rec: Rectangle, stroke: &Stroke) {
        let mut d = d.begin_blend_mode(stroke.blend.mode);
        if let Some(shader) = shader {
            let mut d = d.begin_shader_mode(shader);
            let mut d = d.begin_blend_mode(BlendMode::BLEND_ALPHA_PREMULTIPLY);
            d.draw_texture_pro(buffer, src_rec, dest_rec, Vector2::ZERO, 0.0, Color::WHITE.alpha(stroke.blend.opacity));
        } else {
            d.draw_texture_pro(buffer, src_rec, dest_rec, Vector2::ZERO, 0.0, Color::WHITE.alpha(stroke.blend.opacity));
        }
    }

    fn begin_stroke(&mut self, mut _rl: &mut RaylibHandle, _thread: &RaylibThread, mouse_world_pos: Vector2) {
        self.mouse_prev = Some(mouse_world_pos);
    }

    fn continue_stroke(&mut self, mut rl: &mut RaylibHandle, thread: &RaylibThread, mouse_world_pos: Vector2) {
        self.mouse_curr = mouse_world_pos;
        let mouse_prev = self.mouse_prev.unwrap_or(mouse_world_pos);
        {
            let mut d = rl.begin_texture_mode(thread, &mut self.buffer);

            match &self.stroke.pattern {
                Pattern::Solid(_) | Pattern::Chowder(_) => {
                    let color = if let Pattern::Solid(color) = &self.stroke.pattern { *color } else { Color::WHITE };
                    // todo: ew, low-poly
                    d.draw_line_ex(mouse_prev, self.mouse_curr, self.stroke.thick, color);
                    d.draw_circle_v(mouse_prev, self.stroke.thick * 0.5, color);
                    d.draw_circle_v(self.mouse_curr, self.stroke.thick * 0.5, color);
                    self.mouse_prev = Some(self.mouse_curr);
                }

                Pattern::Image(interval, tex) => {
                    let interval = *interval;
                    assert!(interval.is_normal() && interval > 0.001, "are you insane?");
                    let delta = self.mouse_curr - mouse_prev;
                    let dist_sqr = delta.magnitude_sqr();
                    let mut pos = mouse_prev;
                    let offset = Vector2::new(-tex.width as f32 * 0.5, -tex.height as f32 * 0.5);
                    if dist_sqr > interval {
                        let dist = dist_sqr.sqrt();
                        let step_size = interval / dist;
                        let num_intervals = step_size.recip().floori();
                        let step = delta * interval / dist;
                        for _ in 0..num_intervals {
                            d.draw_texture_ex(tex, pos + offset, 0.0, 1.0, Color::WHITE); // todo: custom tint?
                            pos += step;
                        }
                    }
                    debug_assert!(pos.distance_sqr(self.mouse_curr) < interval, "should have placed a stamp at every interval");
                    self.mouse_prev = Some(pos);
                }
            }
        }
    }

    fn end_stroke(&mut self, mut rl: &mut RaylibHandle, thread: &RaylibThread, _mouse_world_pos: Vector2) {
        let raster = &mut *self.target;
        let (src_rec, dest_rec) = (raster.texture.src_rec, raster.texture.dest_rec);
        {
            let mut d = raster.texture.begin_texture_mode(&mut rl, thread);
            Self::draw_buffer(&mut d, &self.buffer, self.shader.as_ref(), src_rec, dest_rec, &self.stroke);
        }
        {
            let mut d = rl.begin_texture_mode(thread, &mut self.buffer);
            d.clear_background(Color::BLANK);
        }
        self.mouse_prev = None;
    }
}

impl<'a> ToolType<'a> for RasterBrush<'a> {
    fn tick<'b: 'a>(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        _current_appearance: &mut Appearance,
        _document: &'b mut Document,
        _scratch_rtex: &mut Vec<RenderTexture2D>,
        mouse_world_pos: Vector2,
        _px_world_size: f32,
    ) {
        self.mouse_curr = mouse_world_pos;
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.begin_stroke(rl, thread, mouse_world_pos);
        }
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            self.continue_stroke(rl, thread, mouse_world_pos);
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.end_stroke(rl, thread, mouse_world_pos);
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, _editor: &Editor, _shader_table: &ShaderTable, px_world_size: f32, _viewport: &Rect2, #[cfg(dev)] _mouse_world_pos: Vector2) {
        let raster = &*self.target;
        let (src_rec, dest_rec) = (raster.texture.src_rec.flipped(), raster.texture.dest_rec);
        Self::draw_buffer(d, &self.buffer, self.shader.as_ref(), src_rec, dest_rec, &self.stroke);
        d.draw_ring(self.mouse_curr, (self.stroke.thick - px_world_size) * 0.5, (self.stroke.thick + px_world_size) * 0.5, 0.0, 360.0, 72, Color::WHITE);
    }
}
