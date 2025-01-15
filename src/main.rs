use std::{cell::RefCell, collections::BTreeMap, rc::Rc, sync::OnceLock};
use raylib::prelude::*;
use rand::{distributions::Uniform, prelude::*};

fn mix(c0: &Color, c1: &Color, amount: f32) -> Color {
    Color {
        r: lerp(c0.r as f32, c1.r as f32, amount) as u8,
        g: lerp(c0.g as f32, c1.g as f32, amount) as u8,
        b: lerp(c0.b as f32, c1.b as f32, amount) as u8,
        a: lerp(c0.a as f32, c1.a as f32, amount) as u8,
    }
}

pub struct Blending {
    pub opacity: f32,
    pub mode: BlendMode,
}

impl Default for Blending {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            mode: BlendMode::BLEND_ALPHA,
        }
    }
}

pub mod gradient {
    use super::*;

    pub struct Control {
        pos: u8,
        color: Color,
    }

    pub struct Ramp {
        /// Should be sorted and unique by `pos`
        colors: Vec<Control>,
    }

    impl Ramp {
        pub fn clean(&mut self) {
            self.colors.sort_by_key(|x| x.pos);
            self.colors.dedup_by_key(|x| x.pos);
        }

        pub fn color_at(&self, t: f32) -> Color {
            let pos = u8::try_from((t * 255.0).floor() as isize).expect("t must be between 0 and 1");
            let mut lower: &Control = &self.colors[0];
            let mut upper: &Control = &self.colors[0];
            // todo: use binary search
            for ctrl in &self.colors {
                if ctrl.pos <= pos && ctrl.pos > lower.pos {
                    lower = ctrl;
                }
                if ctrl.pos > pos && ctrl.pos < upper.pos {
                    upper = ctrl;
                }
            }
            let t = (t - lower.pos as f32) / (upper.pos as f32 - lower.pos as f32); // normalize
            mix(&lower.color, &upper.color, t)
        }
    }
}
use gradient::Ramp as GradientRamp;

pub mod stroke {
    use super::*;

    pub enum GradientStyle {
        /// Parallel to the path
        Along,
        /// Perpendicular to the path
        Across,
        /// Gradient ignores the path and acts like a linear fill
        Within,
    }

    pub struct Gradient {
        pub ramp: GradientRamp,
        pub style: GradientStyle,
    }

    pub enum Pattern {
        Solid(Color),
        Gradient {
            ramp: GradientRamp,
            style: GradientStyle,
        },
    }

    pub enum Align {
        Inside,
        Middle,
        Outside,
    }

    pub enum WidthProfile {
        Constant(f32),
        Variable(BTreeMap<f32, f32>),
    }

    pub struct Stroke {
        pub blend: Blending,
        pub pattern: Pattern,
        pub thick: WidthProfile,
        pub align: Align,
    }

    impl Default for Stroke {
        fn default() -> Self {
            Self {
                blend: Blending::default(),
                pattern: Pattern::Solid(Color::BLACK),
                thick: WidthProfile::Constant(1.0),
                align: Align::Middle,
            }
        }
    }
}
use raylib::prelude::RaylibDraw;
use stroke::Stroke;

pub mod fill {
    use super::*;

    pub enum GradientStyle {
        Linear,
        Radial,
    }

    pub enum Pattern {
        Solid(Color),
        Gradient {
            ramp: GradientRamp,
            style: GradientStyle,
        },
    }

    pub struct Fill {
        pub blend: Blending,
        pub pattern: Pattern,
    }

    impl Default for Fill {
        fn default() -> Self {
            Self {
                blend: Blending::default(),
                pattern: Pattern::Solid(Color::WHITE),
            }
        }
    }
}
use fill::Fill;

pub enum StyleItem {
    Stroke(Stroke),
    Fill(Fill),
}

pub struct Appearance {
    pub items: Vec<StyleItem>,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            items: vec![
                StyleItem::Fill(Fill::default()),
                StyleItem::Stroke(Stroke::default()),
            ],
        }
    }
}

impl Appearance {
    pub fn new(items: Vec<StyleItem>) -> Self {
        Self {
            items,
        }
    }
}

pub struct VectorPath {
    /// p1, c2, c3, p4, c5, c6...
    pub points: Vec<(Vector2, Vector2, Vector2)>,
    pub appearance: Appearance,
}

impl VectorPath {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            appearance: Appearance::default(),
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, color: Color) {
        for window in self.points.windows(2) {
            if let [(_c1_in, p1, c1_out), (c2_in, p2, _c2_out)] = window {
                d.draw_spline_segment_bezier_cubic(*p1, *c1_out, *c2_in, *p2, 1.0, color);
            }
        }
        for (c_in, p, c_out) in &self.points {
            d.draw_line_v(c_in, p, color);
            d.draw_line_v(p, c_out, color);
            d.draw_circle_v(c_in, 3.0, color);
            d.draw_circle_v(c_out, 3.0, color);
            d.draw_circle_v(p, 4.0, color);
        }
    }
}

pub struct Bitmap {
    texture: Texture2D,
    source_rec: Rectangle,
    dest_rec: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
}

impl Bitmap {
    pub fn new(texture: Texture2D, position: Vector2) -> Self {
        let (width, height) = (texture.width as f32, texture.height as f32);
        Self {
            texture,
            source_rec: Rectangle::new(0.0, 0.0, width, height),
            dest_rec: Rectangle::new(position.x, position.y, width, height),
            origin: Vector2::new(width * 0.5, height * 0.5),
            rotation: 0.0,
            tint: Color::WHITE,
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_texture_pro(&self.texture, self.source_rec, self.dest_rec, self.origin, self.rotation, self.tint);
    }
}

pub enum LayerItem {
    Group(Rc<RefCell<Layer>>),
    Path(Rc<RefCell<VectorPath>>),
    Bitmap(Bitmap),
}

pub struct Layer {
    /// Name of the layer in the layers panel
    pub name: String,
    /// Color of paths
    pub color: Color,
    /// Skip in rendering
    pub is_hidden: bool,
    /// Disallow selection and editing
    pub is_locked: bool,
    /// Show items in layer tree
    pub is_expanded: bool,
    /// Items move with layer
    pub is_group: bool,
    pub blend: Blending,
    pub bounds: Rectangle,
    pub items: Vec<Rc<RefCell<LayerItem>>>,
}

pub enum LayerPanelTreeItemData<'a> {
    Layer {
        slot: Rectangle,
        color_rec: Rectangle,
        thumbnail_rec: Rectangle,
        name_rec: Rectangle,
        expand_collapse_rec: Rectangle,
        layer: &'a mut Layer,
    },
    Path {
        slot: Rectangle,
        thumbnail_rec: Rectangle,
        path: &'a mut Rc<RefCell<VectorPath>>,
    },
    Bitmap {
        slot: Rectangle,
        thumbnail_rec: Rectangle,
        bitmap: &'a mut Bitmap,
    },
}

impl Layer {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            name,
            color,
            is_hidden: false,
            is_locked: false,
            is_expanded: false,
            is_group: false,
            blend: Blending::default(),
            bounds: Rectangle::default(),
            items: Vec::new(),
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        if !self.is_hidden {
            for item in &self.items {
                match &*item.borrow() {
                    LayerItem::Group(group) => group.borrow().draw(d),
                    LayerItem::Path(path) => path.borrow().draw(d, self.color),
                    LayerItem::Bitmap(bitmap) => bitmap.draw(d),
                }
            }
        }
    }

    pub const GAP: f32 = 2.0;
    pub const INDENT: f32 = 6.0;
    pub const THUMBNAIL_SIZE: f32 = 32.0;
    pub const THUMBNAIL_INSET: f32 = 6.0;
    pub const LAYER_HEIGHT: f32 = 2.0 * Self::THUMBNAIL_INSET + Self::THUMBNAIL_SIZE;
    pub const LAYER_COLOR_WIDTH: f32 = 4.0;

    /// Calls the closure on each visible item in the layer panel tree until `f` returns [`Some`], then returns the output of `f`.
    /// Returns [`None`] if `f` never returns [`Some`].
    pub fn for_each_layer_tree_item_internal<T>(
        &mut self,
        panel: &Rectangle,
        y: &mut f32,
        x: f32,
        width: f32,
        f: &mut impl FnMut(LayerPanelTreeItemData<'_>) -> Option<T>
    ) -> Option<T> {
        {
            let mut rec = Rectangle::new(x, *y, width, Self::LAYER_HEIGHT);
            if rec.check_collision_recs(panel) {
                let data = LayerPanelTreeItemData::Layer {
                    slot: rec,
                    color_rec: {
                        rec.width = Self::LAYER_COLOR_WIDTH;
                        rec
                    },
                    thumbnail_rec: {
                        rec.x += Self::LAYER_COLOR_WIDTH + Self::THUMBNAIL_INSET;
                        rec.y += Self::THUMBNAIL_INSET;
                        (rec.width, rec.height) = (Self::THUMBNAIL_SIZE, Self::THUMBNAIL_SIZE);
                        rec
                    },
                    name_rec: {
                        rec.x += Self::THUMBNAIL_SIZE + Self::THUMBNAIL_INSET;
                        rec.height = 10.0;
                        rec.width = width - Self::LAYER_COLOR_WIDTH + Self::THUMBNAIL_INSET - Self::THUMBNAIL_SIZE;
                        rec
                    },
                    expand_collapse_rec: {
                        rec.y += 12.0;
                        rec.width = 10.0;
                        rec.height = 10.0;
                        rec
                    },
                    layer: self,
                };

                let result = f(data);
                if result.is_some() {
                    return result;
                }
            }

            *y += Self::LAYER_HEIGHT + Self::GAP;
        }

        if self.is_expanded {
            for item in self.items.iter().rev() {
                let item = &mut *item.borrow_mut();
                match item {
                    LayerItem::Group(group) => {
                        let result = group.borrow_mut().for_each_layer_tree_item_internal(panel, y, x + Self::INDENT, width - Self::INDENT, f);
                        if result.is_some() {
                            return result;
                        }
                    }

                    LayerItem::Path(path) => {
                        let mut rec = Rectangle::new(x, *y, width, Self::LAYER_HEIGHT);
                        if rec.check_collision_recs(panel) {
                            f(LayerPanelTreeItemData::Path {
                                slot: rec,
                                thumbnail_rec: {
                                    rec.x += Self::LAYER_COLOR_WIDTH + Self::THUMBNAIL_INSET;
                                    rec.y += Self::THUMBNAIL_INSET;
                                    (rec.width, rec.height) = (Self::THUMBNAIL_SIZE, Self::THUMBNAIL_SIZE);
                                    rec
                                },
                                path,
                            });
                        }
                        *y += Self::LAYER_HEIGHT + Self::GAP;
                    }

                    LayerItem::Bitmap(bitmap) => {
                        let mut rec = Rectangle::new(x, *y, width, Self::LAYER_HEIGHT);
                        if rec.check_collision_recs(panel) {
                            f(LayerPanelTreeItemData::Bitmap {
                                slot: rec,
                                thumbnail_rec: {
                                    rec.x += Self::LAYER_COLOR_WIDTH + Self::THUMBNAIL_INSET;
                                    rec.y += Self::THUMBNAIL_INSET;
                                    (rec.width, rec.height) = (Self::THUMBNAIL_SIZE, Self::THUMBNAIL_SIZE);
                                    rec
                                },
                                bitmap,
                            });
                        }
                        *y += Self::LAYER_HEIGHT + Self::GAP;
                    }
                }
            }
        }

        None
    }
}

pub struct ArtBoard {
    pub name: String,
    pub rect: Rectangle,
}

impl ArtBoard {
    pub fn new(name: String, rect: Rectangle) -> Self {
        Self { name, rect }
    }
}

pub struct Document {
    pub title: String,
    pub camera: Camera2D,
    pub paper_color: Color,
    pub layers: Vec<Layer>,
    pub art_boards: Vec<ArtBoard>,
}

impl Document {
    pub fn new(title: String, width: f32, height: f32) -> Self {
        Self {
            title,
            camera: Camera2D {
                offset: Vector2::zero(),
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            },
            paper_color: Color::GRAY,
            layers: vec![Layer::new("layer 0".to_string(), Color::BLUEVIOLET)],
            art_boards: vec![ArtBoard::new("artboard 0".to_string(), rrect(0.0, 0.0, width, height))],
        }
    }

    pub fn for_each_layer_tree_item<T>(&mut self, panel: &Rectangle, mut f: impl FnMut(LayerPanelTreeItemData) -> Option<T>) -> Option<T> {
        const INSET: f32 = 2.0;

        let mut y = panel.y + INSET;
        let x = panel.x + INSET;
        let width = panel.width - INSET * 2.0;

        for layer in self.layers.iter_mut().rev() {
            let result = layer.for_each_layer_tree_item_internal(panel, &mut y, x, width, &mut f);
            if result.is_some() { return result; }
        }

        None
    }

    pub fn draw_layer_tree(&mut self, d: &mut impl RaylibDraw, panel: &Rectangle) {
        let mut d = d.begin_scissor_mode(panel.x as i32, panel.y as i32, panel.width as i32, panel.height as i32);
        d.draw_rectangle_rec(*panel, Color::new(24,24,24,255));
        self.for_each_layer_tree_item(panel, |data| -> Option<()> {
            match data {
                LayerPanelTreeItemData::Layer {
                    slot,
                    color_rec,
                    thumbnail_rec,
                    name_rec,
                    expand_collapse_rec,
                    layer,
                } => {
                    d.draw_rectangle_rec(slot, Color::new(32,32,32,255));
                    d.draw_rectangle_rec(color_rec, layer.color);
                    d.draw_rectangle_rec(thumbnail_rec, Color::GRAY);
                    d.draw_text(&layer.name, name_rec.x as i32, name_rec.y as i32, 10, Color::new(200,200,200,255));

                    // expand icon
                    {
                        let [p0, p1, p2] = if layer.is_expanded {
                            [
                                Vector2::new(expand_collapse_rec.x, expand_collapse_rec.y),
                                Vector2::new(expand_collapse_rec.x + 5.0, expand_collapse_rec.y + 6.0),
                                Vector2::new(expand_collapse_rec.x + expand_collapse_rec.height, expand_collapse_rec.y),
                            ]
                        } else {
                            [
                                Vector2::new(expand_collapse_rec.x, expand_collapse_rec.y),
                                Vector2::new(expand_collapse_rec.x, expand_collapse_rec.y + expand_collapse_rec.height),
                                Vector2::new(expand_collapse_rec.x + 6.0, expand_collapse_rec.y + 5.0),
                            ]
                        };
                        d.draw_triangle(p0, p1, p2, Color::new(200,200,200,255));
                    }
                }

                LayerPanelTreeItemData::Path {
                    slot,
                    thumbnail_rec,
                    path: _,
                } => {
                    d.draw_rectangle_rec(slot, Color::new(32,32,32,255));
                    d.draw_rectangle_rec(thumbnail_rec, Color::GRAY);
                }

                LayerPanelTreeItemData::Bitmap {
                    slot,
                    thumbnail_rec,
                    bitmap: _,
                } => {
                    d.draw_rectangle_rec(slot, Color::new(32,32,32,255));
                    d.draw_rectangle_rec(thumbnail_rec, Color::GRAY);
                }
            }

            None
        });
    }
}

pub enum Tool {
    DirectSelection {
        selection: Vec<Rc<RefCell<LayerItem>>>,
    },
    Pen {
        /// If [`Some`], continue seleted.
        /// If [`None`], find a hovered path or create a new path upon clicking.
        current_path: Option<Rc<RefCell<VectorPath>>>,

        /// [`Some`] while dragging, [`None`] otherwise.
        current_anchor: Option<Vector2>,
    },
}

fn main() {
    let background_color: Color = Color::new(32,32,32,255);

    let mut rng = thread_rng();
    let uniform_u8 = Uniform::<u8>::new_inclusive(0, 255);

    let (mut rl, thread) = init()
        .title("Amity Vector Art")
        .size(1280, 720)
        .resizable()
        .build();

    rl.set_target_fps(60);

    let mut document = Document::new("untitled".to_string(), 256.0, 256.0);
    document.camera.target = Vector2::new(
        0.5 * (document.art_boards[0].rect.width  - rl.get_screen_width()  as f32),
        0.5 * (document.art_boards[0].rect.height - rl.get_screen_height() as f32),
    );

    let mut mouse_screen_pos_prev = rl.get_mouse_position();
    let mut current_tool = Tool::DirectSelection { selection: Vec::new(), };
    let mut layers_panel = Rectangle::new(0.0, 0.0, 256.0, 0.0);
    layers_panel.x = rl.get_screen_width() as f32 - layers_panel.width;
    layers_panel.height = rl.get_screen_height() as f32;

    while !rl.window_should_close() {
        let mouse_screen_pos = rl.get_mouse_position();
        let mouse_screen_delta = mouse_screen_pos - mouse_screen_pos_prev;
        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, document.camera);

        if rl.is_window_resized() {
            layers_panel.x = rl.get_screen_width() as f32 - layers_panel.width;
            layers_panel.height = rl.get_screen_height() as f32;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            document.camera.target -= mouse_screen_delta / document.camera.zoom;
        }

        document.camera.target += mouse_screen_delta / document.camera.zoom;
        document.camera.offset += mouse_screen_delta;

        {
            let scroll = rl.get_mouse_wheel_move();
            if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
                if scroll > 0.0 && document.camera.zoom < 8.0 {
                    document.camera.zoom *= 2.0;
                } else if scroll < 0.0 && document.camera.zoom > 0.125 {
                    document.camera.zoom *= 0.5;
                }
            } else if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                document.camera.target.x -= scroll * 20.0 / document.camera.zoom;
            } else {
                document.camera.target.y -= scroll * 20.0 / document.camera.zoom;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_V) {
            current_tool = Tool::DirectSelection {
                selection: Vec::new(),
            };
        } else if rl.is_key_pressed(KeyboardKey::KEY_P) {
            current_tool = Tool::Pen {
                current_path: match current_tool {
                    Tool::DirectSelection { selection } if selection.len() == 1 => {
                        match &*selection[0].borrow_mut() {
                            LayerItem::Path(path) => Some(path.clone()),
                            _ => None,
                        }
                    }
                    _ => None,
                },
                current_anchor: None,
            };
        }

        if layers_panel.check_collision_point_rec(mouse_screen_pos) {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                document.for_each_layer_tree_item(&layers_panel, |data| -> Option<()> {
                    match data {
                        LayerPanelTreeItemData::Layer {
                            slot,
                            color_rec: _,
                            thumbnail_rec: _,
                            name_rec,
                            expand_collapse_rec,
                            layer,
                        } if slot.check_collision_point_rec(mouse_screen_pos) => {
                            if expand_collapse_rec.check_collision_point_rec(mouse_screen_pos) {
                                layer.is_expanded = !layer.is_expanded;
                            } else if name_rec.check_collision_point_rec(mouse_screen_pos) {
                                println!("clicked name");
                            } else {
                                println!("clicked slot");
                            }
                            Some(())
                        }
                        _ => None,
                    }
                });
            }
        } else {
            match &mut current_tool {
                Tool::DirectSelection { selection: _ } => {
                    // todo
                }

                Tool::Pen { current_path, current_anchor } => {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        if current_path.is_none() {
                            // create a new path
                            let path = VectorPath::new();
                            let path = Rc::new(RefCell::new(path));
                            document.layers.push({
                                let mut layer = Layer::new(
                                    format!("layer {}", document.layers.len()),
                                    Color::new(
                                        uniform_u8.sample(&mut rng),
                                        uniform_u8.sample(&mut rng),
                                        uniform_u8.sample(&mut rng),
                                        255,
                                    ),
                                );
                                layer.items.push(Rc::new(RefCell::new(LayerItem::Path(path.clone()))));
                                layer
                            });
                            *current_path = Some(path);
                        }
                        *current_anchor = Some(mouse_world_pos);
                    } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                        if let Some(current_path) = current_path.as_mut() {
                            let mut path = current_path.borrow_mut();
                            if let Some(anchor) = current_anchor.take() {
                                path.points.push((
                                    anchor * 2.0 - mouse_world_pos, // x - (a - x) = 2x - a
                                    anchor,
                                    mouse_world_pos,
                                ));
                            }
                        } else {
                            println!("warning: `current_path` should have been set when mouse was pressed");
                        }
                    }
                }
            }
        }

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(background_color);

            {
                let mut d = d.begin_mode2D(document.camera);

                // Artboards background
                for board in &document.art_boards {
                    d.draw_rectangle_rec(board.rect, document.paper_color);
                }

                for layer in &document.layers {
                    layer.draw(&mut d);
                }

                if let Tool::Pen { current_path: Some(current_path), current_anchor } = &current_tool {
                    let c_out = mouse_world_pos;
                    let path = current_path.borrow();
                    match current_anchor {
                        &Some(p) => {
                            let c_in = p * 2.0 - c_out;
                            if let Some((_, p_last, c_out_last)) = path.points.last() {
                                d.draw_spline_segment_bezier_cubic(
                                    *p_last,
                                    *c_out_last,
                                    c_in,
                                    p,
                                    1.0, Color::BLUEVIOLET);
                            }
                            d.draw_line_v(p, c_out, Color::BLUEVIOLET);
                            d.draw_line_v(p, p * 2.0 - c_out, Color::BLUEVIOLET);
                            d.draw_circle_v(c_in, 3.0, Color::BLUEVIOLET);
                            d.draw_circle_v(p, 5.0, Color::BLUEVIOLET);
                            d.draw_circle_v(c_out, 3.0, Color::BLUEVIOLET);
                        }
                        None => {
                            if let Some((_, p_last, c_out_last)) = path.points.last() {
                                d.draw_spline_segment_bezier_cubic(
                                    *p_last,
                                    *c_out_last,
                                    c_out,
                                    c_out,
                                    1.0,
                                    Color::BLUEVIOLET,
                                );
                            }
                            d.draw_circle_v(c_out, 3.0, Color::BLUEVIOLET);
                        }
                    }
                }

                // Artboards foreground
                for board in &document.art_boards {
                    d.draw_text(&board.name, board.rect.x as i32, board.rect.y as i32 - 10, 10, Color::WHITE);
                    d.draw_rectangle_lines(board.rect.x as i32, board.rect.y as i32, board.rect.width as i32, board.rect.height as i32, Color::BLACK);
                }
            }

            // Draw layers panel
            document.draw_layer_tree(&mut d, &layers_panel);
        }

        mouse_screen_pos_prev = mouse_screen_pos;
    }
}
