use std::{collections::VecDeque, fs::File, io::{self, BufRead, BufReader, BufWriter, Read, Write}, path::Path};
use raylib::prelude::*;
use amylib::{rc::*, collections::tree::*};
use crate::{
    document::{
        artboard::{ArtBoard, IntRect2},
        layer::*,
        Document,
    },
    appearance::*,
    layer::group::Group,
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

fn write_i32(writer: &mut BufWriter<File>, n: i32) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes())
}
fn read_i32(reader: &mut BufReader<File>) -> io::Result<i32> {
    read_bytes::<{size_of::<i32>()}>(reader)
        .map(|buf| i32::from_le_bytes(buf))
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

// fn write_rectangle(writer: &mut BufWriter<File>, r: &Rectangle) -> io::Result<()> {
//     write_f32(writer, r.x)?;
//     write_f32(writer, r.y)?;
//     write_f32(writer, r.width)?;
//     write_f32(writer, r.height)
// }
// fn read_rectangle(reader: &mut BufReader<File>) -> io::Result<Rectangle> {
//     Ok(Rectangle {
//         x:      read_f32(reader)?,
//         y:      read_f32(reader)?,
//         width:  read_f32(reader)?,
//         height: read_f32(reader)?,
//     })
// }

fn write_irect2(writer: &mut BufWriter<File>, r: &IntRect2) -> io::Result<()> {
    write_i32(writer, r.x)?;
    write_i32(writer, r.y)?;
    write_i32(writer, r.width)?;
    write_i32(writer, r.height)
}
fn read_irect2(reader: &mut BufReader<File>) -> io::Result<IntRect2> {
    Ok(IntRect2 {
        x:      read_i32(reader)?,
        y:      read_i32(reader)?,
        width:  read_i32(reader)?,
        height: read_i32(reader)?,
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

fn extract_blend_mode(bits: u8) -> BlendMode {
    unsafe { std::mem::transmute((bits & 0b111) as i32) }
}

impl Document {
    /// Save as binary
    ///
    /// Safety: Mutating the document outside this function while it is being saved is undefined behavior
    pub fn save_bin(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut writer = BufWriter::new(File::create(path)?);
        writer.write_all(b"amyvec")?;
        writer.write_all(&[VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, b'\n'])?;

        let Self {
            path: _,
            title,
            camera,
            paper_color,
            layers,
            selection: _,
            artboards,
            active_artboard: _,
            history: _,
            future: _,
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

        let camera_peculiar = camera.target * camera.zoom - camera.offset;
        write_f32(&mut writer, camera.zoom)?;
        write_vector2(&mut writer, &camera_peculiar)?;

        // artboards
        write_u64(&mut writer, artboards.len() as u64)?;
        for ArtBoard { name, rect } in artboards {
            name.retain(is_sterile);
            write_str(&mut writer, &name)?;
            write_irect2(&mut writer, &rect)?;
        }

        // layers
        writer.write_all(&(layers.len() as u64).to_le_bytes())?;
        for mut layer in layers.dfs_iter_mut(|_| true) {
            let mut layer = layer.write();

            // settings
            {
                let is_expanded = matches!(&*layer, Layer::Group(Group { is_expanded: true, .. }));

                let LayerSettings {
                    name,
                    color,
                    is_hidden,
                    is_locked,
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
                    is_closed,
                }) => {
                    writer.write_all(&[b'p', *is_closed as u8])?; // todo: an entire byte for one bit? :c

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
                    writer.write_all(
                        &points.make_contiguous().chunks(2)
                            .map(|pair| {
                                let mut byte = 0u8;
                                debug_assert!(pair.len() <= 2);
                                for i in 0..pair.len() {
                                    byte |= match pair[i].ctrls.as_ref() {
                                        None => 0,
                                        Some(CtrlPt1 { c1: (side, _), c2 }) => (match side {
                                            Ctrl::In  => 0b0000,
                                            Ctrl::Out => 0b1000,
                                        }) | (match c2 {
                                            None                     => 0b001,
                                            Some(CtrlPt2::Smooth)    => 0b011,
                                            Some(CtrlPt2::Mirror(_)) => 0b101,
                                            Some(CtrlPt2::Exact(_))  => 0b111,
                                        })
                                    } << (4 * i);
                                }
                                byte
                            })
                            .collect::<Box<[u8]>>()
                    )?;

                    for pp in points.iter_mut() {
                        write_vector2(&mut writer, &pp.p)?;
                        if let Some(CtrlPt1 { c1: (_, c1), c2 }) = pp.ctrls.as_ref() {
                            write_vector2(&mut writer, c1)?;
                            if let Some(c2) = c2.as_ref() {
                                match c2 {
                                    Smooth => (),
                                    Mirror(s2) => write_f32(&mut writer, *s2)?,
                                    Exact(c2) => write_vector2(&mut writer, c2)?,
                                }
                            }
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
    pub fn load_bin(path: impl AsRef<Path>, mouse_screen_pos: Vector2) -> io::Result<Self> {
        let mut reader = BufReader::new(File::open(&path)?);
        let mut version_line = String::new();
        if !matches!(&read_bytes(&mut reader)?, b"amyvec") { Err(io::Error::other("incompatible file extension"))? }
        reader.read_line(&mut version_line)?;
        if let &[version_major, version_minor, version_patch, b'\n'] = version_line.as_bytes() {
            if version_major != VERSION_MAJOR || version_minor > VERSION_MINOR || version_patch > VERSION_PATCH {
                Err(io::Error::other("incompatible version"))
            } else if version_minor < VERSION_MINOR {
                Err(io::Error::other("todo: implement backwards compatibility"))
            } else {
                let mut document = Self::new();

                document.path = Some(path.as_ref().to_path_buf());

                document.title = read_str(&mut reader)?;

                document.paper_color = read_color_rgb(&mut reader)?;

                document.layer_color_acc   = read_u64(&mut reader)? as usize;
                document.layer_name_acc    = read_u64(&mut reader)? as usize;
                document.artboard_name_acc = read_u64(&mut reader)? as usize;

                document.camera.zoom = read_f32(&mut reader)?;
                // camera.target * camera.zoom - camera.offset;
                let camera_peculiar = read_vector2(&mut reader)?;
                document.camera.target = (camera_peculiar + mouse_screen_pos) / document.camera.zoom;
                document.camera.offset = mouse_screen_pos;

                // artboards
                {
                    let num_artboards = read_u64(&mut reader)? as usize;
                    document.artboards.reserve(num_artboards);

                    for _ in 0..num_artboards {
                        document.artboards.push(ArtBoard {
                            name: read_str(&mut reader)?,
                            rect: read_irect2(&mut reader)?,
                        });
                    }
                }

                // layers
                fn read_layer_tree(reader: &mut BufReader<File>) -> io::Result<LayerTree> {
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
                            name,
                            color,
                            is_hidden,
                            is_locked,
                            is_group,
                            blend: Blending {
                                opacity,
                                mode: blend_mode,
                            },
                            artwork_bounds: Rectangle::default(),
                        };

                        let [layer_type] = read_bytes::<1>(reader)?;
                        tree.push(StrongMut::new(
                            match layer_type {
                                b'g' => {
                                    Layer::Group(Group {
                                        settings,
                                        is_expanded,
                                        items: read_layer_tree(reader)?,
                                    })
                                },

                                b'p' => {
                                    let [is_closed] = read_bytes::<1>(reader)?;
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
                                    let flags = {
                                        let mut mashed_flags = vec![0u8; num_points / 2 + num_points % 2];
                                        reader.read_exact(mashed_flags.as_mut_slice())?;
                                        let mut flags = Vec::with_capacity(num_points);
                                        for byte in mashed_flags.iter() {
                                            flags.push(byte & 0b1111);
                                            if flags.len() < flags.capacity() {
                                                flags.push((byte >> 4) & 0b1111);
                                            }
                                        }
                                        flags
                                    };
                                    let mut points = VecDeque::with_capacity(num_points);
                                    for byte in flags {
                                        points.push_back(PathPoint {
                                            p: read_vector2(reader)?,
                                            ctrls: if byte & 0b001 != 0 {
                                                Some(CtrlPt1 {
                                                    c1: (if byte & 0b1000 != 0 { Ctrl::Out } else { Ctrl::In }, read_vector2(reader)?),
                                                    c2: match byte & 0b111 {
                                                        0b001 => None,
                                                        0b011 => Some(CtrlPt2::Smooth),
                                                        0b101 => Some(CtrlPt2::Mirror(read_f32(reader)?)),
                                                        0b111 => Some(CtrlPt2::Exact(read_vector2(reader)?)),
                                                        _ => unreachable!("we did a lot of bitmasks to get here"),
                                                    },
                                                })
                                            } else { None },
                                        });
                                    }

                                    Layer::Path(VectorPath {
                                        settings,
                                        points,
                                        appearance: Appearance { items: style_items },
                                        is_closed: is_closed != 0,
                                    })
                                },

                                b'r' => {
                                    unimplemented!("not doing until supported")
                                },

                                x => Err(io::Error::other(format!("unknown layer type: {x:?}")))?,
                            }
                        ))
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
}
