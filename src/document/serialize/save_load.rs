use std::{collections::VecDeque, fs::File, io::{self, BufRead, BufReader, BufWriter, Read, Write}, path::Path};
use amymath::prelude::{IRect2, Matrix2x2};
use amyvec::curve::Curve;
use raylib::prelude::*;
use amylib::{collections::Tree, io::{ReadArr, ReadBytes, WriteArr, WriteBytes}, rc::prelude::*};
use crate::{
    appearance::{Appearance, Blending, StyleItem}, document::{
        artboard::ArtBoard,
        layer::{Layer, LayerSettings, LayerTree},
        Document,
    }, layer::group::Group, raster::Raster, vector_path::{
        fill,
        path_point::{Ctrl, Ctrl1, Ctrl2, PathPoint},
        stroke,
        VectorPath,
    }
};

/// Previous formats are structurally incompatible and cannot be converted \
/// ex: previous versions lack the necessary data to accurately reconstruct them, or rely on removed features
static VERSION_MAJOR: u8 = 0;

/// Previous formats are are distinctly different from the current format, but can be converted \
/// ex: previous versions are more verbose and store data that is more compressed in the current version
static VERSION_MINOR: u8 = 0;

/// Previous formats are fully compatible with the current format \
/// ex: previous versions are indistinguishable from a newer file that just doesn't use any of the new features
static VERSION_PATCH: u8 = 1;

const fn is_sterile(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == ' ' || c.is_ascii_punctuation()
}

fn write_color_rgb(writer: &mut impl Write, c: Color) -> io::Result<()> {
    writer.write_le_arr([c.r, c.g, c.b])
}
fn read_color_rgb(reader: &mut impl Read) -> io::Result<Color> {
    reader.read_le_arr().map(|[r, g, b]| Color { r, g, b, a: 255 })
}

fn write_color_rgba(writer: &mut impl Write, c: Color) -> io::Result<()> {
    writer.write_le_arr([c.r, c.g, c.b, c.a])
}
fn read_color_rgba(reader: &mut impl Read) -> io::Result<Color> {
    reader.read_le_arr().map(|[r, g, b, a]| Color { r, g, b, a })
}

fn write_irect2(writer: &mut impl Write, r: &IRect2) -> io::Result<()> {
    writer.write_le_arr([r.xmin, r.ymin, r.xmax, r.ymax])
}
fn read_irect2(reader: &mut impl Read) -> io::Result<IRect2> {
    reader.read_le_arr().map(|[xmin, ymin, xmax, ymax]| IRect2 { xmin, ymin, xmax, ymax })
}

fn write_vector2(writer: &mut impl Write, v: Vector2) -> io::Result<()> {
    writer.write_le_arr([v.x, v.y])
}
fn read_vector2(reader: &mut impl Read) -> io::Result<Vector2> {
    reader.read_le_arr().map(|[x, y]| Vector2 { x, y  })
}

fn read_vec(reader: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; reader.read_le()?];
    reader.read_exact(buf.as_mut_slice())?;
    Ok(buf)
}

fn write_str(writer: &mut impl Write, s: &str) -> io::Result<()> {
    let bytes = s.as_bytes();
    writer.write_le(bytes.len())?;
    writer.write_all(bytes)
}
fn read_str(reader: &mut impl Read) -> io::Result<String> {
    String::from_utf8(read_vec(reader)?)
        .map_err(io::Error::other)
}

fn extract_blend_mode(bits: u8) -> BlendMode {
    unsafe { std::mem::transmute(i32::from(bits & 0b111)) }
}

fn opacity_from_byte(byte: u8) -> f32 {
    f32::from(byte) / 255.0
}

#[allow(clippy::cast_sign_loss, reason = "minimum value is 0 before casting")]
#[allow(clippy::cast_possible_truncation, reason = "value is rounded before casting")]
fn opacity_byte(opacity: &mut f32) -> u8 {
    let byte = (opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
    *opacity = opacity_from_byte(byte);
    byte
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, reason = "BlendMode is repr(i32) only goes up to 7")]
fn blend_mode_to_byte(blend_mode: BlendMode) -> u8 {
    unsafe {
        std::mem::transmute::<BlendMode, i32>(blend_mode) as u8
    }
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
            artboards,
            active_artboard: _,
            history: _,
            future: _,
            layer_color_acc,
            layer_name_acc,
            artboard_name_acc,
        } = self;

        title.retain(is_sterile);
        write_str(&mut writer, title)?;

        write_color_rgb(&mut writer, *paper_color)?;

        writer.write_le(*layer_color_acc)?;
        writer.write_le(*layer_name_acc)?;
        writer.write_le(*artboard_name_acc)?;

        let camera_peculiar = camera.target * camera.zoom - camera.offset;
        writer.write_le(camera.zoom)?;
        write_vector2(&mut writer, camera_peculiar)?;

        // artboards
        writer.write_le(artboards.len())?;
        for ArtBoard { name, rect } in artboards {
            name.retain(is_sterile);
            write_str(&mut writer, name)?;
            write_irect2(&mut writer, rect)?;
        }

        save_layer_tree(&mut writer, layers)?;

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

                document.layers = read_layer_tree(&mut reader)?;

                Ok(document)
            }
        } else {
            Err(io::Error::other("version so new i dont know how to read the number"))
        }
    }
}

fn save_layer_settings(writer: &mut impl Write, layer: &mut Layer) -> io::Result<()> {
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
    } = match layer {
        Layer::Group(group) => &mut group.settings,
        Layer::Path(path) => &mut path.write().settings,
        Layer::Raster(raster) => &mut raster.write().settings,
    };

    name.retain(is_sterile);
    write_str(writer, name)?;

    writer.write_all(&[
        (u8::from(is_expanded) << 6)
        | (u8::from(*is_hidden) << 5)
        | (u8::from(*is_locked) << 4)
        | (u8::from(*is_group) << 3)
        | blend_mode_to_byte(*blend_mode)
    ])?;

    write_color_rgb(writer, *color)?;

    let opacity_byte = opacity_byte(opacity);
    writer.write_all(&[opacity_byte])?;
    Ok(())
}

fn read_layer_settings(reader: &mut impl Read) -> io::Result<(LayerSettings, bool)> {
    let name = read_str(reader)?;
    let flags: u8 = reader.read_le()?;
    let is_expanded = ((flags >> 6) & 1) != 0;
    let is_hidden   = ((flags >> 5) & 1) != 0;
    let is_locked   = ((flags >> 4) & 1) != 0;
    let is_group    = ((flags >> 3) & 1) != 0;
    let blend_mode = extract_blend_mode(flags);
    let color = read_color_rgb(reader)?;
    let opacity_byte: u8 = reader.read_le()?;
    let opacity = opacity_from_byte(opacity_byte);

    Ok((LayerSettings {
        name,
        color,
        is_hidden,
        is_locked,
        is_group,
        blend: Blending {
            opacity,
            mode: blend_mode,
        },
    }, is_expanded))
}

fn save_group(
    writer: &mut impl Write,
    Group {
        settings: _, // already handled
        items,
        is_expanded: _, // already handled
    }: &mut Group
) -> io::Result<()> {
    writer.write_le(b'g')?;
    writer.write_le(items.len())?;
    // actual items handled by containing loop
    Ok(())
}

fn read_group(reader: &mut impl Read, settings: LayerSettings, is_expanded: bool) -> io::Result<Group> {
    Ok(Group {
        settings,
        is_expanded,
        items: read_layer_tree(reader)?,
    })
}

fn save_stroke_pattern(writer: &mut impl Write, pattern: &mut stroke::Pattern) -> io::Result<()> {
    match pattern {
        stroke::Pattern::Solid(color) => write_color_rgba(writer, *color)?,
        stroke::Pattern::Gradient { .. } => unimplemented!("not doing until supported"),
    }
    Ok(())
}

fn read_stroke_pattern(reader: &mut impl Read, is_gradient: bool) -> io::Result<stroke::Pattern> {
    Ok(if is_gradient {
        unimplemented!("not doing until supported")
    } else {
        stroke::Pattern::Solid(read_color_rgba(reader)?)
    })
}

fn save_stroke_thickness(writer: &mut impl Write, thick: &mut stroke::WidthProfile) -> io::Result<()> {
    match thick {
        stroke::WidthProfile::Constant(thick) => writer.write_le(*thick)?,
        stroke::WidthProfile::Variable(_) => unimplemented!("not doing until supported"),
    }
    Ok(())
}

fn read_stroke_thickness(reader: &mut impl Read, is_variable_thickness: bool) -> io::Result<stroke::WidthProfile> {
    Ok(if is_variable_thickness {
        unimplemented!("not doing until supported")
    } else {
        stroke::WidthProfile::Constant(reader.read_le()?)
    })
}

fn save_stroke_style(
    writer: &mut impl Write,
    stroke::Stroke {
        blend: Blending {
            opacity,
            mode: blend_mode,
        },
        pattern,
        thick,
        align,
    }: &mut stroke::Stroke,
) -> io::Result<()> {
    writer.write_le(b's')?;

    let opacity_byte = opacity_byte(opacity);
    writer.write_all(&[
        (u8::from(matches!(pattern, stroke::Pattern::Gradient { .. })) << 6)
        | (u8::from(matches!(thick, stroke::WidthProfile::Variable(_))) << 5)
        | (match align {
            stroke::Align::Inside  => 0,
            stroke::Align::Middle  => 1,
            stroke::Align::Outside => 2,
        } << 3)
        | blend_mode_to_byte(*blend_mode),
        opacity_byte])?;

    save_stroke_pattern(writer, pattern)?;
    save_stroke_thickness(writer, thick)?;

    Ok(())
}

fn read_stroke_style(reader: &mut impl Read) -> io::Result<stroke::Stroke> {
    let [flags, opacity_byte]: [u8; 2] = reader.read_le_arr()?;
    let opacity = opacity_from_byte(opacity_byte);
    let is_gradient = (flags >> 6) & 1 != 0;
    let is_variable_thickness = (flags >> 5) & 1 != 0;
    let align = match (flags >> 3) & 0b11 {
        0 => stroke::Align::Inside,
        1 => stroke::Align::Middle,
        2 => stroke::Align::Outside,
        3.. => unreachable!("bitmask restricts possible cases"),
    };
    let blend_mode = extract_blend_mode(flags);
    let pattern = read_stroke_pattern(reader, is_gradient)?;
    let thick = read_stroke_thickness(reader, is_variable_thickness)?;
    Ok(stroke::Stroke {
        blend: Blending {
            opacity,
            mode: blend_mode,
        },
        pattern,
        thick,
        align,
    })
}

fn save_fill_pattern(writer: &mut impl Write, pattern: &mut fill::Pattern) -> io::Result<()> {
    match pattern {
        fill::Pattern::Solid(color) => write_color_rgba(writer, *color)?,
        fill::Pattern::Gradient { .. } => unimplemented!("not doing until supported"),
    }
    Ok(())
}

fn read_fill_pattern(reader: &mut impl Read, is_gradient: bool) -> io::Result<fill::Pattern> {
    Ok(if is_gradient {
        unimplemented!("not doing until supported")
    } else {
        fill::Pattern::Solid(read_color_rgba(reader)?)
    })
}

fn save_fill_style(
    writer: &mut impl Write,
    fill::Fill {
        blend: Blending {
            opacity,
            mode: blend_mode,
        },
        pattern,
    }: &mut fill::Fill,
) -> io::Result<()> {
    writer.write_le(b'f')?;

    let opacity_byte = opacity_byte(opacity);
    writer.write_le_arr([
        (u8::from(matches!(pattern, fill::Pattern::Gradient { .. })) << 3)
        | blend_mode_to_byte(*blend_mode),
        opacity_byte])?;

    save_fill_pattern(writer, pattern)?;

    Ok(())
}

fn read_fill_style(reader: &mut impl Read) -> io::Result<fill::Fill> {
    let [flags, opacity_byte]: [u8; 2] = reader.read_le_arr()?;
    let opacity = opacity_from_byte(opacity_byte);
    let is_gradient = (flags >> 3) & 1 != 0;
    let blend_mode = extract_blend_mode(flags);
    let pattern = read_fill_pattern(reader, is_gradient)?;
    Ok(fill::Fill {
        blend: Blending {
            opacity,
            mode: blend_mode,
        },
        pattern,
    })
}

fn save_appearance(writer: &mut impl Write, appearance: &mut Appearance) -> io::Result<()> {
    writer.write_le(appearance.items.len())?;
    for style_item in &mut appearance.items {
        match style_item {
            StyleItem::Stroke(stroke) => save_stroke_style(writer, stroke)?,
            StyleItem::Fill(fill) => save_fill_style(writer, fill)?,
        }
    }
    Ok(())
}

fn read_appearance(reader: &mut impl Read) -> io::Result<Appearance> {
    let num_style_items = reader.read_le()?;
    let mut style_items = Vec::with_capacity(num_style_items);
    for _ in 0..num_style_items {
        let style_item_type = reader.read_le()?;
        style_items.push(match style_item_type {
            b's' => StyleItem::Stroke(read_stroke_style(reader)?),
            b'f' => StyleItem::Fill(read_fill_style(reader)?),
            x => Err(io::Error::other(format!("unknown style type: {x:?}")))?,
        });
    }
    Ok(Appearance { items: style_items })
}

fn save_path_points(writer: &mut impl Write, curve: &mut Curve) -> io::Result<()> {
    fn encode_pp(pair: &[PathPoint]) -> u8 {
        let mut byte = 0u8;
        debug_assert!(pair.len() <= 2);
        for (i, pp) in pair.iter().enumerate() {
            #[allow(unreachable_patterns, reason = "written out explicitly for clarity")]
            let nibble = match pp.c.as_ref() {
                None                                                                => 0b0000,
                Some(Ctrl1 { c1: (Ctrl::In,  _), c2: None                        }) => 0b0001,
                Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(Ctrl2::Reflect       ) }) => 0b0010,
                Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(Ctrl2::Mirror     (_)) }) => 0b0011,
                Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(Ctrl2::Transformed(_)) }) => 0b0100,
            //  Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(                     ) }) => 0b0101, // reserved
            //  Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(                     ) }) => 0b0110, // reserved
                Some(Ctrl1 { c1: (Ctrl::In,  _), c2: Some(Ctrl2::Exact      (_)) }) => 0b0111,
            //  None              Ctrl::Out                                         => 0b1000, // invalid
                Some(Ctrl1 { c1: (Ctrl::Out, _), c2: None                        }) => 0b1001,
                Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(Ctrl2::Reflect       ) }) => 0b1010,
                Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(Ctrl2::Mirror     (_)) }) => 0b1011,
                Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(Ctrl2::Transformed(_)) }) => 0b1100,
            //  Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(                     ) }) => 0b1101, // reserved
            //  Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(                     ) }) => 0b1110, // reserved
                Some(Ctrl1 { c1: (Ctrl::Out, _), c2: Some(Ctrl2::Exact      (_)) }) => 0b1111,
            };
            byte |= nibble << (4 * i);
        }
        byte
    }

    writer.write_le(curve.points.len())?;
    writer.write_all(
        &curve.points.make_contiguous()
            .chunks(2) // explicitly NOT `chunks_exact`, there may be an odd number of points.
            .map(encode_pp)
            .collect::<Box<[u8]>>()
    )?;

    for pp in &mut curve.points {
        write_vector2(writer, pp.p)?;
        if let Some(Ctrl1 { c1: (_, c1), c2 }) = pp.c.as_ref() {
            write_vector2(writer, *c1)?;
            if let Some(c2) = c2.as_ref() {
                match c2 {
                    Ctrl2::Reflect => (),
                    Ctrl2::Mirror(s2) => writer.write_le(*s2)?,
                    Ctrl2::Transformed(m2) => writer.write_le_arr([m2.m00, m2.m01, m2.m10, m2.m11])?,
                    Ctrl2::Exact(c2) => write_vector2(writer, *c2)?,
                }
            }
        }
    }

    Ok(())
}

fn read_path_points(reader: &mut impl Read, is_closed: bool) -> io::Result<Curve> {
    let num_points = reader.read_le()?;
    let flags = {
        let mut mashed_flags = vec![0u8; num_points / 2 + num_points % 2];
        reader.read_exact(mashed_flags.as_mut_slice())?;
        let mut flags = Vec::with_capacity(num_points);
        for byte in &mashed_flags {
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
               0b10000.. => unreachable!("bitmask restricts cases"),
            },
        });
    }

    Ok(Curve { points, is_closed })
}

fn save_vector_path(writer: &mut impl Write, path: &mut StrongMut<VectorPath>) -> io::Result<()> {
    let path = &mut *path.write();
    writer.write_all(&[b'p', u8::from(path.curve.is_closed)])?; // todo: an entire byte for one bit? :c

    save_appearance(writer, &mut path.appearance)?;
    save_path_points(writer, &mut path.curve)?;

    Ok(())
}

fn read_vector_path(reader: &mut impl Read, settings: LayerSettings) -> io::Result<StrongMut<VectorPath>> {
    let is_closed = reader.read_le()?;
    let appearance = read_appearance(reader)?;
    let curve = read_path_points(reader, is_closed)?;
    Ok(StrongMut::new(VectorPath {
        settings,
        curve,
        appearance,
    }))
}

fn save_raster(writer: &mut impl Write, _raster: &mut StrongMut<Raster>) -> io::Result<()> {
    writer.write_le(b'r')?;
    unimplemented!("not doing until supported")
}

fn read_raster(_reader: &mut impl Read, _settings: LayerSettings) -> io::Result<StrongMut<Raster>> {
    unimplemented!("not doing until supported")
}

fn save_layer_tree(writer: &mut impl Write, layers: &mut Tree<Layer>) -> io::Result<()> {
    writer.write_le(layers.len())?;
    for layer in layers.dfs_iter_mut(|_| true) {
        save_layer_settings(writer, layer)?;

        // specializations
        match layer {
            Layer::Group(group) => save_group(writer, group)?,
            Layer::Path(path) => save_vector_path(writer, path)?,
            Layer::Raster(raster) => save_raster(writer, raster)?,
        }
    }

    Ok(())
}

fn read_layer_tree(reader: &mut impl Read) -> io::Result<LayerTree> {
    let mut tree = LayerTree::new();
    let num_layers = reader.read_le()?;
    tree.reserve(num_layers);

    for _ in 0..num_layers {
        let (settings, is_expanded) = read_layer_settings(reader)?;

        let layer_type = reader.read_le()?;
        tree.push(match layer_type {
            b'g' => Layer::Group(read_group(reader, settings, is_expanded)?),
            b'p' => Layer::Path(read_vector_path(reader, settings)?),
            b'r' => Layer::Raster(read_raster(reader, settings)?),
            x => Err(io::Error::other(format!("unknown layer type: {x:?}")))?,
        });
    }

    Ok(tree)
}
