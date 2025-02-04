use std::{io, path::Path};
use amymath::prelude::IntRectangle;
use raylib::prelude::*;
use amylib::iter::directed::DirectibleDoubleEndedIterator;
use crate::{document::Document, layer::{BackToFore, LayerType}};

#[derive(Debug, Clone, Copy)]
pub enum DownscaleAlgorithm {
    Nearest,
    Bicubic,
    Lanczos,
}

impl Document {
    pub fn render_png(
        &self,
        path: impl AsRef<Path>,
        artboard: usize,
        mut rl: &mut RaylibHandle,
        thread: &RaylibThread,
        supersampling: Option<DownscaleAlgorithm>,
        background: Color,
    ) -> io::Result<()> {
        const SUPERSAMPLE_FACTOR: u32 = 8;
        let is_supersampled = supersampling.is_some();
        let artboard = self.artboards.get(artboard).ok_or_else(|| io::Error::other("invalid artboard index"))?.rect;
        let filename = path.as_ref().to_str().ok_or_else(|| io::Error::other("could not convert path to &str"))?;

        let (width, height) = (artboard.width(), artboard.height());
        let (artboard_width, artboard_height) = {
            assert!(width.is_positive() && height.is_positive());
            #[allow(clippy::cast_sign_loss, reason = "guarded by assertion `artboard.width.is_positive() && artboard.height.is_positive()`")]
            (width as u32, height as u32)
        };

        let (artboard_x, artboard_y) = {
            assert!(artboard.xmin.ilog2() < f32::MANTISSA_DIGITS && artboard.ymin.ilog2() < f32::MANTISSA_DIGITS);
            #[allow(clippy::cast_precision_loss, reason = "guarded by assertion `artboard.x.ilog2() < f32::MANTISSA_DIGITS && artboard.y.ilog2() < f32::MANTISSA_DIGITS`")]
            (artboard.xmin as f32, artboard.ymin as f32)
        };

        let mut rtex = rl.load_render_texture(
            thread,
            if is_supersampled { artboard_width  * SUPERSAMPLE_FACTOR } else { artboard_width  },
            if is_supersampled { artboard_height * SUPERSAMPLE_FACTOR } else { artboard_height },
        ).map_err(io::Error::other)?;

        let camera = Camera2D {
            offset: Vector2::zero(),
            target: Vector2::new(artboard_x, artboard_y),
            rotation: 0.0,
            zoom: if is_supersampled {
                const _: () = assert!(SUPERSAMPLE_FACTOR.ilog2() < f32::MANTISSA_DIGITS); // proof
                #[allow(clippy::cast_precision_loss, reason = "SUPERSAMPLE_FACTOR can be represented losslessly")]
                (SUPERSAMPLE_FACTOR as f32)
            } else {
                1.0
            },
        };

        {
            let mut d = rl.begin_texture_mode(thread, &mut rtex);
            d.clear_background(background);
            {
                let mut d = d.begin_mode2D(camera);
                for layer in self.layers.dfs_iter(|g| !g.settings.is_hidden).cdir::<BackToFore>() {
                    layer.draw_rendered(&mut d);
                }
            }
        }

        let mut image = rtex.load_image().map_err(io::Error::other)?;
        if let Some(downscale_algo) = supersampling {
            fn l(x: f32, y: f32) -> f32 { lanczos(x, 1.0) * lanczos(y, 1.0) }
            match downscale_algo {
                DownscaleAlgorithm::Nearest => image.resize_nn(width, height),
                DownscaleAlgorithm::Bicubic => image.resize(width, height),
                DownscaleAlgorithm::Lanczos => image.resize_custom(width, height, l),
            }
        }
        image.flip_vertical();
        image.export_image(filename);

        Ok(())
    }
}

fn lanczos(x: f32, a: f32) -> f32 {
    use std::f32::consts::PI;
    if x == 0.0 {
        1.0
    } else if x.abs() >= a {
        0.0
    } else {
        a * (PI * x).sin() * (PI * x / a).sin() / (PI * PI * x * x)
    }
}

/// Takes `u` (-1..1), `v` (-1..1) and returns that relative coordinate's influence
pub type WeightFn = fn(f32, f32) -> f32;

pub trait Resize {
    fn resize_custom(&mut self, new_width: i32, new_height: i32, weight_fn: WeightFn);
}

impl Resize for Image {
    fn resize_custom(&mut self, new_width: i32, new_height: i32, weight_fn: WeightFn) {
        let old_width = self.width;
        let old_height = self.height;
        if self.data.is_null() || old_width <= 0 || old_height <= 0 { return; }
        if self.format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32 {
            let bytes_per_pixel =
                #[allow(clippy::cast_sign_loss, reason = "data should never take up a negative amount of space")]
                (unsafe { ffi::GetPixelDataSize(1, 1, self.format) } as u32);
            let (old_data_len, new_data_len) =
                #[allow(clippy::cast_sign_loss, reason = "guarded by `if old_width <= 0 || old_height <= 0 { return; }`")] {(
                    old_width as u32 * old_height as u32 * bytes_per_pixel,
                    new_width as u32 * new_height as u32 * bytes_per_pixel,
                )};
            let new_data = unsafe { ffi::MemAlloc(new_data_len) };

            if !new_data.is_null() {
                let new_data_color_slice = unsafe {
                    std::slice::from_raw_parts_mut(new_data.cast::<Color>(), new_data_len as usize)
                };
                let old_data_color_slice = unsafe {
                    std::slice::from_raw_parts(self.data.cast::<Color>(), old_data_len as usize)
                };

                let src_rec = IntRectangle {
                    x: 0,
                    y: 0,
                    width:  old_width,
                    height: old_height,
                };
                let dest_rec = IntRectangle {
                    x: 0,
                    y: 0,
                    width:  new_width,
                    height: new_height,
                };

                let (ratio_x, ratio_y) = (
                    f64::from(old_width ) / f64::from(new_width ),
                    f64::from(old_height) / f64::from(new_height),
                );
                let sample_rec =
                    #[allow(clippy::cast_possible_truncation, reason = "they're being floor'd and ceil'd")]
                    IntRectangle {
                        x: (ratio_x * -0.5).floor() as i32,
                        y: (ratio_y * -0.5).floor() as i32,
                        width:  (ratio_x).ceil() as i32,
                        height: (ratio_y).ceil() as i32,
                    };
                // let weights: Box<[f32]> = sample_rec
                //     .iter_uv_row_col()
                //     .map(|(_, (u, v))| weight_fn(2.0 * (u - 0.5), 2.0 * (v - 0.5)))
                //     .collect();

                for (dest_x, dest_y) in dest_rec.iter_xy_row_col() {
                    let sample_rec = sample_rec.with_offset(dest_x, dest_y).intersect(&src_rec);
                    let (sum, sum_weight) = sample_rec.iter_uv_row_col().fold(
                        (Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }, 0.0),
                        |(sum, sum_weight), ((src_x, src_y), (u, v))| {
                            let weight = weight_fn(2.0 * (u - 0.5), 2.0 * (v - 0.5));
                            let normalized = old_data_color_slice[src_rec.index_of(src_x, src_y)].color_normalize();
                            (
                                Vector4 {
                                    x: sum.x + normalized.x * weight,
                                    y: sum.y + normalized.y * weight,
                                    z: sum.z + normalized.z * weight,
                                    w: sum.w + normalized.w * weight,
                                },
                                sum_weight + weight,
                            )
                        }
                    );
                    let sum_weight_inv = sum_weight.recip();
                    new_data_color_slice[dest_rec.index_of(dest_x, dest_y)] = Color::color_from_normalized(Vector4 {
                        x: (sum.x * sum_weight_inv).clamp(0.0, 1.0),
                        y: (sum.y * sum_weight_inv).clamp(0.0, 1.0),
                        z: (sum.z * sum_weight_inv).clamp(0.0, 1.0),
                        w: (sum.w * sum_weight_inv).clamp(0.0, 1.0),
                    });
                }
            }

            unsafe { ffi::MemFree(self.data) };
            self.data = new_data;
            self.width = new_width;
            self.height = new_height;
        } else {
            println!("unsupported pixel format, enjoy your oversized image");
        }
    }
}
