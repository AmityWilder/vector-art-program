use amylib::{prelude::RoundToInt, rc::prelude::*};
use amymath::prelude::{DistanceSqr, FlipRectangle};
use raylib::prelude::*;
use crate::{appearance::Blending, document::Document, shaders::ShaderTable, tool::ToolType};
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

pub struct RasterBrush {
    /// Flushes to target when stroke finishes.
    ///
    /// This is not a performance optimization.
    /// This is to hide the discrete nature of brush stroke ticks.
    buffer: RenderTexture2D,
    target: StrongMut<Raster>,
    mouse_prev: Option<Vector2>,
    mouse_curr: Vector2,
    pub stroke: Stroke,
}

impl RasterBrush {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, target: StrongMut<Raster>, stroke: Stroke) -> Result<Self, String> {
        let (width, height) = {
            let texture = &target.read().texture.rtex;
            (texture.width() as u32, texture.height() as u32)
        };
        let buffer = rl.load_render_texture(thread, width, height)?;
        Ok(Self { buffer, target, mouse_curr: Vector2::default(), mouse_prev: None, stroke })
    }

    fn begin_stroke(&mut self, mut _rl: &mut RaylibHandle, _thread: &RaylibThread, mouse_world_pos: Vector2) {
        self.mouse_prev = Some(mouse_world_pos);
    }

    fn continue_stroke(&mut self, mut rl: &mut RaylibHandle, thread: &RaylibThread, mouse_world_pos: Vector2) {
        self.mouse_curr = mouse_world_pos;
        let mouse_prev = self.mouse_prev.expect("should be set in begin_stroke");
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
                    let dist_sqr = delta.length_sqr();
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
                    debug_assert!(pos.distance_sqr_to(self.mouse_curr) < interval, "should have placed a stamp at every interval");
                    self.mouse_prev = Some(pos);
                }
            }
        }
    }

    fn end_stroke(&mut self, mut rl: &mut RaylibHandle, thread: &RaylibThread, _mouse_world_pos: Vector2) {
        let mut raster = self.target.write();
        let (src_rec, dest_rec) = (raster.texture.src_rec, raster.texture.dest_rec);
        {
            let mut d = raster.texture.begin_texture_mode(&mut rl, thread);
            {
                let mut d = d.begin_blend_mode(self.stroke.blend.mode);
                d.draw_texture_pro(&self.buffer, src_rec, dest_rec, Vector2::zero(), 0.0, Color::WHITE.alpha(self.stroke.blend.opacity));
            }
        }
        {
            let mut d = rl.begin_texture_mode(thread, &mut self.buffer);
            d.clear_background(Color::BLANK);
        }
        self.mouse_prev = None;
    }
}

impl ToolType for RasterBrush {
    fn tick(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, _document: &mut Document, mouse_world_pos: Vector2) {
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

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, _shader_table: &ShaderTable) {
        let inv_zoom = document.camera.zoom.recip();
        let raster = self.target.read();
        let (src_rec, dest_rec) = (raster.texture.src_rec.flipped(), raster.texture.dest_rec);
        // dest_rec.y += dest_rec.height;
        // dest_rec.height = -dest_rec.height;
        // d.draw_rectangle_gradient_ex(dest_rec, Color::RED, Color::GREEN, Color::BLUE, Color::WHITE);
        d.draw_texture_pro(&self.buffer, src_rec, dest_rec, Vector2::zero(), 0.0, Color::WHITE.alpha(self.stroke.blend.opacity));
        d.draw_ring(self.mouse_curr, (self.stroke.thick - inv_zoom) * 0.5, (self.stroke.thick + inv_zoom) * 0.5, 0.0, 360.0, 72, Color::WHITE);
    }
}
