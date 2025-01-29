use std::{collections::VecDeque, fs::File, io::{self, BufRead, BufReader, BufWriter, Read, Write}, path::Path};
use amymath::prelude::{IntRect2, Matrix2x2};
use raylib::prelude::*;
use amylib::{io::*, rc::*};
use crate::{
    document::{
        artboard::ArtBoard,
        layer::*,
        Document,
    },
    appearance::*,
    layer::group::Group,
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

fn write_color_rgb(writer: &mut BufWriter<File>, c: Color) -> io::Result<()> {
    writer.write_le_arr([c.r, c.g, c.b])
}
fn read_color_rgb(reader: &mut BufReader<File>) -> io::Result<Color> {
    reader.read_le_arr().map(|[r, g, b]| Color { r, g, b, a: 255 })
}

fn write_color_rgba(writer: &mut BufWriter<File>, c: Color) -> io::Result<()> {
    writer.write_le_arr([c.r, c.g, c.b, c.a])
}
fn read_color_rgba(reader: &mut BufReader<File>) -> io::Result<Color> {
    reader.read_le_arr().map(|[r, g, b, a]| Color { r, g, b, a })
}

fn write_irect2(writer: &mut BufWriter<File>, r: &IntRect2) -> io::Result<()> {
    writer.write_le_arr([r.x, r.y, r.width, r.height])
}
fn read_irect2(reader: &mut BufReader<File>) -> io::Result<IntRect2> {
    reader.read_le_arr().map(|[x, y, width, height]| IntRect2 { x, y, width, height  })
}

fn write_vector2(writer: &mut BufWriter<File>, v: &Vector2) -> io::Result<()> {
    writer.write_le_arr([v.x, v.y])
}
fn read_vector2(reader: &mut BufReader<File>) -> io::Result<Vector2> {
    reader.read_le_arr().map(|[x, y]| Vector2 { x, y  })
}

fn read_vec(reader: &mut BufReader<File>) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; reader.read_le()?];
    reader.read_exact(buf.as_mut_slice())?;
    Ok(buf)
}

fn write_str(writer: &mut BufWriter<File>, s: &str) -> io::Result<()> {
    let bytes = s.as_bytes();
    writer.write_le(bytes.len())?;
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

        writer.write_le(*layer_color_acc)?;
        writer.write_le(*layer_name_acc)?;
        writer.write_le(*artboard_name_acc)?;

        let camera_peculiar = camera.target * camera.zoom - camera.offset;
        writer.write_le(camera.zoom)?;
        write_vector2(&mut writer, &camera_peculiar)?;

        // artboards
        writer.write_le(artboards.len())?;
        for ArtBoard { name, rect } in artboards {
            name.retain(is_sterile);
            write_str(&mut writer, &name)?;
            write_irect2(&mut writer, &rect)?;
        }

        // layers
        writer.write_le(layers.len())?;
        for layer in layers.dfs_iter_mut(|_| true) {
            // settings
            {
                let is_expanded = matches!(layer, Layer::Group(Group { is_expanded: true, .. }));

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
                } = match layer {
                    Layer::Group(group) => &mut group.settings,
                    Layer::Path(path) => &mut path.write().settings,
                    Layer::Raster(raster) => &mut raster.write().settings,
                };

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
            match layer {
                Layer::Group(Group {
                    settings: _, // already handled
                    items,
                    is_expanded: _, // already handled
                }) => {
                    writer.write_le(b'g')?;
                    writer.write_le(items.len())?;
                    // actual items handled by containing loop
                }

                Layer::Path(path) => {
                    let path = &mut *path.write();
                    writer.write_all(&[b'p', path.is_closed as u8])?; // todo: an entire byte for one bit? :c

                    writer.write_le(path.appearance.items.len())?;
                    for style_item in path.appearance.items.iter_mut() {
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
                                writer.write_le(b's')?;

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
                                    stroke::WidthProfile::Constant(thick) => writer.write_le(*thick)?,
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
                                writer.write_le(b'f')?;

                                let opacity_byte = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
                                *opacity = opacity_byte as f32 / 255.0;
                                writer.write_le_arr([
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

                    writer.write_le(path.points.len())?;
                    writer.write_all(
                        &path.points.make_contiguous().chunks(2)
                            .map(|pair| {
                                let mut byte = 0u8;
                                debug_assert!(pair.len() <= 2);
                                for i in 0..pair.len() {
                                    // written out explicitly for clarity
                                    #[allow(unreachable_patterns)]
                                    let nibble = match pair[i].c.as_ref() {
                                        None                                                                => 0b0000,
                                        Some(Ctrl1 { c1: (Ctrl::In,  _), c2: None                        }) => 0b0001,
                                        Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(Ctrl2::Reflect       ) }) => 0b0010,
                                        Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(Ctrl2::Mirror     (_)) }) => 0b0011,
                                        Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(Ctrl2::Transformed(_)) }) => 0b0100,
                                      //Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(                     ) }) => 0b0101, // reserved
                                      //Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(                     ) }) => 0b0110, // reserved
                                        Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(Ctrl2::Exact      (_)) }) => 0b0111,
                                      //None              Ctrl::Out                                         => 0b1000, // invalid
                                        Some(Ctrl1 { c1: (Ctrl::Out, _), c2: None                        }) => 0b1001,
                                        Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(Ctrl2::Reflect       ) }) => 0b1010,
                                        Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(Ctrl2::Mirror     (_)) }) => 0b1011,
                                        Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(Ctrl2::Transformed(_)) }) => 0b1100,
                                      //Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(                     ) }) => 0b1101, // reserved
                                      //Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(                     ) }) => 0b1110, // reserved
                                        Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(Ctrl2::Exact      (_)) }) => 0b1111,
                                    };
                                    byte |= nibble << (4 * i);
                                }
                                byte
                            })
                            .collect::<Box<[u8]>>()
                    )?;

                    for pp in path.points.iter_mut() {
                        write_vector2(&mut writer, &pp.p)?;
                        if let Some(Ctrl1 { c1: (_, c1), c2 }) = pp.c.as_ref() {
                            write_vector2(&mut writer, c1)?;
                            if let Some(c2) = c2.as_ref() {
                                match c2 {
                                    Ctrl2::Reflect => (),
                                    Ctrl2::Mirror(s2) => writer.write_le(*s2)?,
                                    Ctrl2::Transformed(m2) => writer.write_le_arr([m2.m00, m2.m01, m2.m10, m2.m11])?,
                                    Ctrl2::Exact(c2) => write_vector2(&mut writer, c2)?,
                                }
                            }
                        }
                    }
                }

                Layer::Raster(_) => {
                    writer.write_le(b'r')?;
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
        if !matches!(&reader.read_le_arr()?, b"amyvec") { Err(io::Error::other("incompatible file extension"))? }
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

                document.layer_color_acc   = reader.read_le()?;
                document.layer_name_acc    = reader.read_le()?;
                document.artboard_name_acc = reader.read_le()?;

                document.camera.zoom = reader.read_le()?;
                // camera.target * camera.zoom - camera.offset;
                let camera_peculiar = read_vector2(&mut reader)?;
                document.camera.target = (camera_peculiar + mouse_screen_pos) / document.camera.zoom;
                document.camera.offset = mouse_screen_pos;

                // artboards
                {
                    let num_artboards = reader.read_le()?;
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
                    let num_layers = reader.read_le()?;
                    tree.reserve(num_layers);

                    for _ in 0..num_layers {
                        let name = read_str(reader)?;
                        let flags: u8 = reader.read_le()?;
                        let is_expanded = ((flags >> 6) & 1) != 0;
                        let is_hidden   = ((flags >> 5) & 1) != 0;
                        let is_locked   = ((flags >> 4) & 1) != 0;
                        let is_group    = ((flags >> 3) & 1) != 0;
                        let blend_mode = extract_blend_mode(flags);
                        let color = read_color_rgb(reader)?;
                        let opacity_byte: u8 = reader.read_le()?;
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

                        let layer_type = reader.read_le()?;
                        tree.push(match layer_type {
                            b'g' => {
                                Layer::Group(Group {
                                    settings,
                                    is_expanded,
                                    items: read_layer_tree(reader)?,
                                })
                            },

                            b'p' => {
                                let is_closed = reader.read_le()?;
                                let num_style_items = reader.read_le()?;
                                let mut style_items = Vec::with_capacity(num_style_items);
                                for _ in 0..num_style_items {
                                    let style_item_type = reader.read_le()?;
                                    style_items.push(match style_item_type {
                                        b's' => {
                                            let [flags, opacity_byte]: [u8; 2] = reader.read_le_arr()?;
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
                                                stroke::WidthProfile::Constant(reader.read_le()?)
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
                                            let [flags, opacity_byte]: [u8; 2] = reader.read_le_arr()?;
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

                                let num_points = reader.read_le()?;
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
                                for nibble in flags {
                                    #[allow(unreachable_patterns)]
                                    points.push_back(PathPoint {
                                        p: read_vector2(reader)?,
                                        c:
                                        // written out explicitly for clarity
                                        match nibble & 0b1111 {
                                            0b0000 => None,
                                            0b0001 => Some(Ctrl1 { c1: (Ctrl::In, read_vector2(reader)?), c2: None }),
                                            0b0010 => Some(Ctrl1 { c1: (Ctrl::In, read_vector2(reader)?), c2: Some(Ctrl2::Reflect) }),
                                            0b0011 => Some(Ctrl1 { c1: (Ctrl::In, read_vector2(reader)?), c2: Some(Ctrl2::Mirror(reader.read_le()?)) }),
                                            0b0100 => Some(Ctrl1 { c1: (Ctrl::In, read_vector2(reader)?), c2: Some(Ctrl2::Transformed(reader.read_le_arr().map(|[m00, m01, m10, m11]| Matrix2x2 { m00, m01, m10, m11 })?)) }),
                                            0b0101 => Err(io::Error::other("reserved"))?,
                                            0b0110 => Err(io::Error::other("reserved"))?,
                                            0b0111 => Some(Ctrl1 { c1: (Ctrl::In, read_vector2(reader)?), c2: Some(Ctrl2::Exact(read_vector2(reader)?)) }),
                                            0b1000 => Err(io::Error::other("invalid"))?,
                                            0b1001 => Some(Ctrl1 { c1: (Ctrl::Out, read_vector2(reader)?), c2: None }),
                                            0b1010 => Some(Ctrl1 { c1: (Ctrl::Out, read_vector2(reader)?), c2: Some(Ctrl2::Reflect) }),
                                            0b1011 => Some(Ctrl1 { c1: (Ctrl::Out, read_vector2(reader)?), c2: Some(Ctrl2::Mirror(reader.read_le()?)) }),
                                            0b1100 => Some(Ctrl1 { c1: (Ctrl::Out, read_vector2(reader)?), c2: Some(Ctrl2::Transformed(reader.read_le_arr().map(|[m00, m01, m10, m11]| Matrix2x2 { m00, m01, m10, m11 })?)) }),
                                            0b1101 => Err(io::Error::other("reserved"))?,
                                            0b1110 => Err(io::Error::other("reserved"))?,
                                            0b1111 => Some(Ctrl1 { c1: (Ctrl::Out, read_vector2(reader)?), c2: Some(Ctrl2::Exact(read_vector2(reader)?)) }),
                                            0b1111..=u8::MAX => unreachable!("bitmask restricts cases"),
                                        },
                                    });
                                }

                                Layer::Path(StrongMut::new(VectorPath {
                                    settings,
                                    points,
                                    appearance: Appearance { items: style_items },
                                    is_closed,
                                }))
                            },

                            b'r' => {
                                unimplemented!("not doing until supported")
                            },

                            x => Err(io::Error::other(format!("unknown layer type: {x:?}")))?,
                        });
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
