use std::{fs::File, io::{self, BufRead, BufReader, BufWriter, Read, Write}, path::Path, sync::{Arc, RwLock}};
use raylib::prelude::*;
use crate::{
    document::{artboard::ArtBoard, layer::*, Document},
    appearance::*,
    layer::{
        tree::*,
        group::Group,
    },
    raster::Raster,
    vector_path::{
        fill,
        path_point::*,
        stroke,
        VectorPath,
    }
};

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

fn read_bytes<const N: usize>(reader: &mut BufReader<File>) -> io::Result<[u8; N]> {
    let mut buf = [0u8; N];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

fn write_u64(writer: &mut BufWriter<File>, n: u64) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes())
}
fn read_u64(reader: &mut BufReader<File>) -> io::Result<u64> {
    read_bytes::<{size_of::<u64>()}>(reader)
        .map(|buf| u64::from_le_bytes(buf))
}

fn write_f32(writer: &mut BufWriter<File>, n: f32) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes())
}
fn read_f32(reader: &mut BufReader<File>) -> io::Result<f32> {
    read_bytes::<{size_of::<f32>()}>(reader)
        .map(|buf| f32::from_le_bytes(buf))
}

fn write_color_rgb(writer: &mut BufWriter<File>, c: Color) -> io::Result<()> {
    writer.write_all(&[c.r, c.g, c.b])
}
fn read_color_rgb(reader: &mut BufReader<File>) -> io::Result<Color> {
    read_bytes::<3>(reader)
        .map(|[r, g, b]| Color { r, g, b, a: 255 })
}

fn write_color_rgba(writer: &mut BufWriter<File>, c: Color) -> io::Result<()> {
    writer.write_all(&[c.r, c.g, c.b, c.a])
}
fn read_color_rgba(reader: &mut BufReader<File>) -> io::Result<Color> {
    read_bytes::<4>(reader)
        .map(|[r, g, b, a]| Color { r, g, b, a })
}

fn write_rectangle(writer: &mut BufWriter<File>, r: &Rectangle) -> io::Result<()> {
    write_f32(writer, r.x)?;
    write_f32(writer, r.y)?;
    write_f32(writer, r.width)?;
    write_f32(writer, r.height)
}
fn read_rectangle(reader: &mut BufReader<File>) -> io::Result<Rectangle> {
    Ok(Rectangle {
        x:      read_f32(reader)?,
        y:      read_f32(reader)?,
        width:  read_f32(reader)?,
        height: read_f32(reader)?,
    })
}

fn write_vector2(writer: &mut BufWriter<File>, v: &Vector2) -> io::Result<()> {
    write_f32(writer, v.x)?;
    write_f32(writer, v.y)
}
fn read_vector2(reader: &mut BufReader<File>) -> io::Result<Vector2> {
    Ok(Vector2 {
        x: read_f32(reader)?,
        y: read_f32(reader)?,
    })
}

fn read_vec(reader: &mut BufReader<File>) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; read_u64(reader)? as usize];
    reader.read_exact(buf.as_mut_slice())?;
    Ok(buf)
}

fn write_str(writer: &mut BufWriter<File>, s: &str) -> io::Result<()> {
    let bytes = s.as_bytes();
    writer.write_all(&(bytes.len() as u64).to_le_bytes())?;
    writer.write_all(bytes)
}
fn read_str(reader: &mut BufReader<File>) -> io::Result<String> {
    String::from_utf8(read_vec(reader)?)
        .map_err(|e| io::Error::other(e))
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
        write_str(&mut writer, &title)?;

        write_color_rgb(&mut writer, *paper_color)?;

        write_u64(&mut writer, *layer_color_acc   as u64)?;
        write_u64(&mut writer, *layer_name_acc    as u64)?;
        write_u64(&mut writer, *artboard_name_acc as u64)?;

        // artboards
        write_u64(&mut writer, artboards.len() as u64)?;
        for ArtBoard { name, rect } in artboards {
            name.retain(is_sterile);
            write_str(&mut writer, &name)?;
            write_rectangle(&mut writer, &rect)?;
        }

        // layers
        writer.write_all(&(layers.len() as u64).to_le_bytes())?;
        for (layer, _depth) in layers.tree_iter(LayerIterDir::default(), |_| true) {
            let mut layer = layer.write().map_err(|e| io::Error::other(format!("layer {:?} poisoned", e.get_ref().settings().name)))?;

            // settings
            {
                let is_expanded = matches!(&*layer, Layer::Group(Group { is_expanded: true, .. }));

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
                write_str(&mut writer, &name)?;

                writer.write_all(&[
                    ((is_expanded as u8) << 6)
                    | ((*is_hidden as u8) << 5)
                    | ((*is_locked as u8) << 4)
                    | ((*is_group as u8) << 3)
                    | (*blend_mode as i32 as u8)
                ])?;

                write_color_rgb(&mut writer, *color)?;

                let opacity_byte = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
                *opacity = opacity_byte as f32 / 255.0;
                writer.write_all(&[opacity_byte])?;
            }

            // specializations
            match &mut *layer {
                Layer::Group(Group {
                    settings: _, // already handled
                    items,
                    is_expanded: _, // already handled
                    expand_button_rec: _,
                }) => {
                    writer.write_all(&[b'g'])?;
                    write_u64(&mut writer, items.len() as u64)?;
                    // actual items handled by containing loop
                }

                Layer::Path(VectorPath {
                    settings: _, // already handled
                    points,
                    appearance: Appearance {
                        items: style_items,
                    },
                }) => {
                    writer.write_all(&[b'p'])?;

                    write_u64(&mut writer, style_items.len() as u64)?;
                    for style_item in style_items.iter_mut() {
                        match style_item {
                            StyleItem::Stroke(stroke::Stroke {
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
                                    | ((matches!(thick, stroke::WidthProfile::Variable(_)) as u8) << 5)
                                    | (match align {
                                        stroke::Align::Inside  => 0,
                                        stroke::Align::Middle  => 1,
                                        stroke::Align::Outside => 2,
                                    } << 3)
                                    | (*blend_mode as i32 as u8),
                                    opacity_byte])?;

                                match pattern {
                                    stroke::Pattern::Solid(color) => write_color_rgba(&mut writer, *color)?,
                                    stroke::Pattern::Gradient { .. } => unimplemented!("not doing until supported"),
                                }

                                match thick {
                                    stroke::WidthProfile::Constant(thick) => write_f32(&mut writer, *thick)?,
                                    stroke::WidthProfile::Variable(_) => unimplemented!("not doing until supported"),
                                }
                            }

                            StyleItem::Fill(fill::Fill {
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
                                    fill::Pattern::Solid(color) => write_color_rgba(&mut writer, *color)?,
                                    fill::Pattern::Gradient { .. } => unimplemented!("not doing until supported"),
                                }
                            }
                        }
                    }

                    write_u64(&mut writer, points.len() as u64)?;
                    for PathPoint { c_in, p, c_out } in points.iter_mut() {
                        if matches!((&c_in, &c_out), (CtrlPoint::Corner | CtrlPoint::Smooth, CtrlPoint::Corner | CtrlPoint::Smooth)) {
                            (*c_in, *c_out) = (CtrlPoint::Corner, CtrlPoint::Corner);
                        }
                        writer.write_all(&[
                            (match c_in {
                                CtrlPoint::Corner   => 0,
                                CtrlPoint::Smooth   => 0b01,
                                CtrlPoint::Exact(_) => 0b10,
                            })
                            | (match c_out {
                                CtrlPoint::Corner   => 0,
                                CtrlPoint::Smooth   => 0b01,
                                CtrlPoint::Exact(_) => 0b10,
                            } << 2)
                        ])?;
                        if let CtrlPoint::Exact(c_in) = c_in {
                            write_vector2(&mut writer, c_in)?;
                        }
                        write_vector2(&mut writer, p)?;
                        if let CtrlPoint::Exact(c_out) = c_out {
                            write_vector2(&mut writer, c_out)?;
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
        let mut reader = BufReader::new(File::open(path)?);
        let mut version_line = String::new();
        reader.read_line(&mut version_line)?;
        if let &[version_major, version_minor, version_patch, b'\n'] = version_line.as_bytes() {
            if version_major != VERSION_MAJOR || version_minor > VERSION_MINOR || version_patch > VERSION_PATCH {
                Err(io::Error::other("incompatible version"))
            } else if version_minor < VERSION_MINOR {
                Err(io::Error::other("todo: implement backwards compatibility"))
            } else {
                let mut document = Self::new();

                document.title = read_str(&mut reader)?;

                document.paper_color = read_color_rgb(&mut reader)?;

                document.layer_color_acc   = read_u64(&mut reader)? as usize;
                document.layer_name_acc    = read_u64(&mut reader)? as usize;
                document.artboard_name_acc = read_u64(&mut reader)? as usize;

                // artboards
                {
                    let num_artboards = read_u64(&mut reader)? as usize;
                    document.artboards.reserve(num_artboards);

                    for _ in 0..num_artboards {
                        document.artboards.push(ArtBoard {
                            name: read_str(&mut reader)?,
                            rect: read_rectangle(&mut reader)?,
                        });
                    }
                }

                // layers
                fn read_layer_tree(reader: &mut BufReader<File>) -> io::Result<LayerTree> {
                    fn extract_blend_mode(bits: u8) -> BlendMode {
                        match bits & 0b111 {
                            0 => BlendMode::BLEND_ALPHA,
                            1 => BlendMode::BLEND_ADDITIVE,
                            2 => BlendMode::BLEND_MULTIPLIED,
                            3 => BlendMode::BLEND_ADD_COLORS,
                            4 => BlendMode::BLEND_SUBTRACT_COLORS,
                            5 => BlendMode::BLEND_ALPHA_PREMULTIPLY,
                            6 => BlendMode::BLEND_CUSTOM,
                            7 => BlendMode::BLEND_CUSTOM_SEPARATE,
                            _ => unreachable!("bitmask restricts possible cases"),
                        }
                    }

                    let mut tree = LayerTree::new();
                    let num_layers = read_u64(reader)? as usize;
                    tree.reserve(num_layers);

                    for _ in 0..num_layers {
                        let name = read_str(reader)?;
                        let [flags] = read_bytes::<1>(reader)?;
                        let is_expanded = ((flags >> 6) & 1) != 0;
                        let is_hidden   = ((flags >> 5) & 1) != 0;
                        let is_locked   = ((flags >> 4) & 1) != 0;
                        let is_group    = ((flags >> 3) & 1) != 0;
                        let blend_mode = extract_blend_mode(flags);
                        let color = read_color_rgb(reader)?;
                        let [opacity_byte] = read_bytes::<1>(reader)?;
                        let opacity = opacity_byte as f32 / 255.0;

                        let settings = LayerSettings {
                            slot_rec: Rectangle::default(),
                            thumbnail_rec: Rectangle::default(),
                            name,
                            name_rec: Rectangle::default(),
                            color,
                            color_rec: Rectangle::default(),
                            is_hidden,
                            hide_button_rec: Rectangle::default(),
                            is_locked,
                            lock_button_rec: Rectangle::default(),
                            is_group,
                            blend: Blending {
                                opacity,
                                mode: blend_mode,
                            },
                            artwork_bounds: Rectangle::default(),
                        };

                        let [layer_type] = read_bytes::<1>(reader)?;
                        tree.push(Arc::new(RwLock::new(
                            match layer_type {
                                b'g' => {
                                    Layer::Group(Group {
                                        settings,
                                        is_expanded,
                                        expand_button_rec: Rectangle::default(),
                                        items: read_layer_tree(reader)?,
                                    })
                                },

                                b'p' => {
                                    let num_style_items = read_u64(reader)? as usize;
                                    let mut style_items = Vec::with_capacity(num_style_items);
                                    for _ in 0..num_style_items {
                                        let [style_item_type] = read_bytes::<1>(reader)?;
                                        style_items.push(match style_item_type {
                                            b's' => {
                                                let [flags, opacity_byte] = read_bytes::<2>(reader)?;
                                                let opacity = opacity_byte as f32 / 255.0;
                                                let is_gradient = (flags >> 6) & 1 != 0;
                                                let is_variable_thickness = (flags >> 5) & 1 != 0;
                                                let align = match (flags >> 3) & 0b11 {
                                                    0 => stroke::Align::Inside,
                                                    1 => stroke::Align::Middle,
                                                    2 => stroke::Align::Outside,
                                                    _ => unreachable!("bitmask restricts possible cases"),
                                                };
                                                let blend_mode = extract_blend_mode(flags);

                                                let pattern = if !is_gradient {
                                                    stroke::Pattern::Solid(read_color_rgba(reader)?)
                                                } else {
                                                    unimplemented!("not doing until supported")
                                                };

                                                let thick = if !is_variable_thickness {
                                                    stroke::WidthProfile::Constant(read_f32(reader)?)
                                                } else {
                                                    unimplemented!("not doing until supported")
                                                };

                                                StyleItem::Stroke(stroke::Stroke {
                                                    blend: Blending {
                                                        opacity,
                                                        mode: blend_mode,
                                                    },
                                                    pattern,
                                                    thick,
                                                    align,
                                                })
                                            }

                                            b'f' => {
                                                let [flags, opacity_byte] = read_bytes::<2>(reader)?;
                                                let opacity = opacity_byte as f32 / 255.0;
                                                let is_gradient = (flags >> 3) & 1 != 0;
                                                let blend_mode = extract_blend_mode(flags);
                                                let pattern = if !is_gradient {
                                                    fill::Pattern::Solid(read_color_rgba(reader)?)
                                                } else {
                                                    unimplemented!("not doing until supported")
                                                };
                                                StyleItem::Fill(fill::Fill {
                                                    blend: Blending {
                                                        opacity,
                                                        mode: blend_mode,
                                                    },
                                                    pattern,
                                                })
                                            }

                                            x => Err(io::Error::other(format!("unknown style type: {x:?}")))?,
                                        });
                                    }

                                    let num_points = read_u64(reader)? as usize;
                                    let mut points = Vec::with_capacity(num_points);
                                    for _ in 0..num_points {
                                        let [flags] = read_bytes::<1>(reader)?;
                                        points.push(PathPoint {
                                            c_in: match flags & 0b11 {
                                                0b00 => CtrlPoint::Corner,
                                                0b01 => CtrlPoint::Smooth,
                                                0b10 => CtrlPoint::Exact(read_vector2(reader)?),
                                                _ => unreachable!("bitmask restricts possible cases")
                                            },
                                            p: read_vector2(reader)?,
                                            c_out: match (flags >> 2) & 0b11 {
                                                0b00 => CtrlPoint::Corner,
                                                0b01 => CtrlPoint::Smooth,
                                                0b10 => CtrlPoint::Exact(read_vector2(reader)?),
                                                _ => unreachable!("bitmask restricts possible cases")
                                            },
                                        });
                                    }

                                    Layer::Path(VectorPath {
                                        settings,
                                        points,
                                        appearance: Appearance { items: style_items },
                                    })
                                },

                                b'r' => {
                                    unimplemented!("not doing until supported")
                                },

                                x => Err(io::Error::other(format!("unknown layer type: {x:?}")))?,
                            }
                        )))
                    }

                    Ok(tree)
                }
                document.layers = read_layer_tree(&mut reader)?;

                Ok(document)
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
