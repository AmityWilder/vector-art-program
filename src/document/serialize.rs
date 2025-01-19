use std::{fmt::Write as _, fs::File, io::{self, BufRead, BufReader, BufWriter, Read, Write as _}, path::Path};
use raylib::prelude::*;
use crate::{appearance::{Appearance, Blending, StyleItem}, layer::tree::LayerTree, raster::Raster, vector_path::{fill::{self, Fill}, gradient, path_point::{CtrlPoint, PathPoint}, stroke::{self, Align, Stroke, WidthProfile}, VectorPath}};

use super::{artboard::ArtBoard, layer::{group::Group, tree::LayerIterDir, Layer, LayerSettings, LayerType}, Document};

/// Previous formats are structurally incompatible and cannot be converted \
/// ex: previous versions lack the necessary data to accurately reconstruct them, or rely on removed features
const VERSION_MAJOR: u8 = 0;

/// Previous formats are are distinctly different from the current format, but can be converted \
/// ex: previous versions are more verbose and store data that is more compressed in the current version
const VERSION_MINOR: u8 = 0;

/// Previous formats are fully compatible with the current format \
/// ex: previous versions are indistinguishable from a newer file that just doesn't use any of the new features
const VERSION_PATCH: u8 = 1;

const fn is_sterile(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == ' ' || c.is_ascii_punctuation()
}

impl Document {
    /// Save as binary
    ///
    /// Safety: Mutating the document outside this function while it is being saved is undefined behavior
    pub fn save_bin(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut writer = BufWriter::new(File::create(path)?);
        writer.write_all(&[VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, b'\n'])?;
        let Self {
            title,
            camera: _,
            paper_color,
            layers,
            selection: _,
            artboards,
            active_artboard: _,
            layer_color_acc,
            layer_name_acc,
            artboard_name_acc,
        } = self;

        title.retain(is_sterile);
        let title_bytes = title.as_bytes();
        writer.write_all(&(title_bytes.len() as u64).to_le_bytes())?;
        writer.write_all(title_bytes)?;

        writer.write_all(&[paper_color.r, paper_color.g, paper_color.b])?;

        writer.write_all(&(*layer_color_acc as u64).to_le_bytes())?;
        writer.write_all(&(*layer_name_acc as u64).to_le_bytes())?;
        writer.write_all(&(*artboard_name_acc as u64).to_le_bytes())?;

        writer.write_all(&(artboards.len() as u64).to_le_bytes())?;
        for ArtBoard { name, rect } in artboards {
            name.retain(is_sterile);
            let name_bytes = name.as_bytes();
            writer.write_all(&(name_bytes.len() as u64).to_le_bytes())?;
            writer.write_all(name_bytes)?;
            writer.write_all(&rect.x.to_le_bytes())?;
            writer.write_all(&rect.y.to_le_bytes())?;
            writer.write_all(&rect.width.to_le_bytes())?;
            writer.write_all(&rect.height.to_le_bytes())?;
        }

        writer.write_all(&(layers.len() as u64).to_le_bytes())?;
        for (layer, _depth) in layers.tree_iter(LayerIterDir::default(), |_| true) {
            let mut layer = layer.write().map_err(|e| io::Error::other(format!("layer {:?} poisoned", e.get_ref().settings().name)))?;
            let LayerSettings {
                slot_rec: _,
                thumbnail_rec: _,
                name,
                name_rec: _,
                color,
                color_rec: _,
                is_hidden,
                hide_button_rec: _,
                is_locked,
                lock_button_rec: _,
                is_group,
                blend: Blending {
                    opacity,
                    mode: blend_mode,
                },
                artwork_bounds: _,
            } = layer.settings_mut();

            name.retain(is_sterile);
            let name_bytes = name.as_bytes();
            writer.write_all(&(name_bytes.len() as u64).to_le_bytes())?;
            writer.write_all(name_bytes)?;

            writer.write_all(&[
                ((*is_hidden as u8) << 5)
                | ((*is_locked as u8) << 4)
                | ((*is_group as u8) << 3)
                | (*blend_mode as i32 as u8)
            ])?;

            let opacity_byte = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
            *opacity = opacity_byte as f32 / 255.0;
            writer.write_all(&[color.r, color.g, color.b, opacity_byte])?;

            match &mut *layer {
                Layer::Group(Group {
                    settings: _, // already handled
                    items: _, // handled by containing loop
                    is_expanded,
                    expand_button_rec: _,
                }) => {
                    writer.write_all(&[b'g', *is_expanded as u8])?;
                }

                Layer::Path(VectorPath {
                    settings: _, // already handled
                    points,
                    appearance: Appearance { items: style_items },
                }) => {
                    writer.write_all(&[b'p'])?;

                    writer.write_all(&(style_items.len() as u64).to_le_bytes())?;
                    for style_item in style_items.iter_mut() {
                        match style_item {
                            StyleItem::Stroke(Stroke {
                                blend: Blending {
                                    opacity,
                                    mode: blend_mode,
                                },
                                pattern,
                                thick,
                                align,
                            }) => {
                                writer.write_all(&[b's'])?;

                                let opacity_byte = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
                                *opacity = opacity_byte as f32 / 255.0;
                                writer.write_all(&[
                                    ((matches!(pattern, stroke::Pattern::Gradient { .. }) as u8) << 6)
                                    | ((matches!(thick, WidthProfile::Variable(_)) as u8) << 5)
                                    | (match align { Align::Inside => 0, Align::Middle => 1, Align::Outside => 2 } << 3)
                                    | (*blend_mode as i32 as u8),
                                    opacity_byte])?;

                                match pattern {
                                    stroke::Pattern::Solid(color) => writer.write_all(&[color.r, color.g, color.b, color.a])?,
                                    stroke::Pattern::Gradient { .. } => unimplemented!("not doing until supported"),
                                }

                                match thick {
                                    WidthProfile::Constant(thick) => writer.write_all(&thick.to_le_bytes())?,
                                    WidthProfile::Variable(_) => unimplemented!("not doing until supported"),
                                }
                            }

                            StyleItem::Fill(Fill {
                                blend: Blending {
                                    opacity,
                                    mode: blend_mode,
                                },
                                pattern,
                            }) => {
                                writer.write_all(&[b'f'])?;

                                let opacity_byte = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
                                *opacity = opacity_byte as f32 / 255.0;
                                writer.write_all(&[
                                    ((matches!(pattern, fill::Pattern::Gradient { .. }) as u8) << 3)
                                    | (*blend_mode as i32 as u8),
                                    opacity_byte])?;

                                match pattern {
                                    fill::Pattern::Solid(color) => writer.write_all(&[color.r, color.g, color.b, color.a])?,
                                    fill::Pattern::Gradient { .. } => unimplemented!("not doing until supported"),
                                }
                            }
                        }
                    }

                    writer.write_all(&(points.len() as u64).to_le_bytes())?;
                    for PathPoint { c_in, p, c_out } in points.iter_mut() {
                        if matches!((&c_in, &c_out), (CtrlPoint::Corner | CtrlPoint::Smooth, CtrlPoint::Corner | CtrlPoint::Smooth)) {
                            (*c_in, *c_out) = (CtrlPoint::Corner, CtrlPoint::Corner);
                        }
                        let mut ctrl_byte = 0;
                        let c_in_bytes = match c_in {
                            CtrlPoint::Corner => None,
                            CtrlPoint::Smooth => {
                                ctrl_byte |= 0b0001;
                                None
                            }
                            CtrlPoint::Exact(c_in) => {
                                ctrl_byte |= 0b0010;
                                Some((c_in.x.to_le_bytes(), c_in.y.to_le_bytes()))
                            }
                        };
                        let c_out_bytes = match c_out {
                            CtrlPoint::Corner => None,
                            CtrlPoint::Smooth => {
                                ctrl_byte |= 0b0100;
                                None
                            }
                            CtrlPoint::Exact(c_out) => {
                                ctrl_byte |= 0b1000;
                                Some((c_out.x.to_le_bytes(), c_out.y.to_le_bytes()))
                            }
                        };
                        writer.write_all(&[ctrl_byte])?;
                        if let Some((x, y)) = c_in_bytes {
                            writer.write_all(&x)?;
                            writer.write_all(&y)?;
                        }
                        writer.write_all(&p.x.to_le_bytes())?;
                        writer.write_all(&p.y.to_le_bytes())?;
                        if let Some((x, y)) = c_out_bytes {
                            writer.write_all(&x)?;
                            writer.write_all(&y)?;
                        }
                    }
                }

                Layer::Raster(Raster {
                    settings: _, // already handled
                    texture: _, // TODO
                }) => {
                    writer.write_all(&[b'r'])?;
                    unimplemented!("not doing until supported")
                }
            }
        }

        Ok(())
    }

    /// Load as binary
    pub fn load_bin(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut writer = BufReader::new(File::open(path)?);
        let mut version_line = String::new();
        writer.read_line(&mut version_line)?;
        if let &[version_major, version_minor, version_patch, b'\n'] = version_line.as_bytes() {
            if version_major != VERSION_MAJOR || version_minor > VERSION_MINOR || version_patch > VERSION_PATCH {
                Err(io::Error::other("incompatible version"))
            } else {
                if version_minor < VERSION_MINOR {
                    Err(io::Error::other("todo: implement backwards compatibility"))
                } else {

                    Err(io::Error::other("under construction"))
                }
            }
        } else {
            Err(io::Error::other("version so new i dont know how to read the number"))
        }
    }

    pub fn export_svg(&self, _path: impl AsRef<Path>) -> io::Result<()> {
        Err(io::Error::other("under construction"))
    }

    pub fn render_png(&self, _path: impl AsRef<Path>) -> io::Result<()> {
        Err(io::Error::other("under construction"))
    }
}
